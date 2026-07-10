#[inline]
pub unsafe fn PathAllocCanonicalize<P0>(pszpathin: P0, dwflags: u32) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathAllocCanonicalize(pszpathin : windows_core::PCWSTR, dwflags : u32, ppszpathout : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PathAllocCanonicalize(pszpathin.param().abi(), dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PathAllocCombine<P0, P1>(pszpathin: P0, pszmore: P1, dwflags: u32) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathAllocCombine(pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR, dwflags : u32, ppszpathout : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PathAllocCombine(pszpathin.param().abi(), pszmore.param().abi(), dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PathCchAddBackslash(pszpath: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAddBackslash(pszpath : windows_core::PWSTR, cchpath : usize) -> windows_core::HRESULT);
    unsafe { PathCchAddBackslash(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathCchAddBackslashEx(pszpath: &mut [u16], ppszend: *mut windows_core::PWSTR, pcchremaining: Option<*mut usize>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAddBackslashEx(pszpath : windows_core::PWSTR, cchpath : usize, ppszend : *mut windows_core::PWSTR, pcchremaining : *mut usize) -> windows_core::HRESULT);
    unsafe { PathCchAddBackslashEx(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), ppszend as _, pcchremaining.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PathCchAddExtension<P2>(pszpath: &mut [u16], pszext: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAddExtension(pszpath : windows_core::PWSTR, cchpath : usize, pszext : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchAddExtension(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), pszext.param().abi()) }
}
#[inline]
pub unsafe fn PathCchAppend<P2>(pszpath: &mut [u16], pszmore: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAppend(pszpath : windows_core::PWSTR, cchpath : usize, pszmore : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchAppend(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), pszmore.param().abi()) }
}
#[inline]
pub unsafe fn PathCchAppendEx<P2>(pszpath: &mut [u16], pszmore: P2, dwflags: u32) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAppendEx(pszpath : windows_core::PWSTR, cchpath : usize, pszmore : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathCchAppendEx(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), pszmore.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn PathCchCanonicalize<P2>(pszpathout: &mut [u16], pszpathin: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCanonicalize(pszpathout : windows_core::PWSTR, cchpathout : usize, pszpathin : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchCanonicalize(core::mem::transmute(pszpathout.as_ptr()), pszpathout.len().try_into().unwrap(), pszpathin.param().abi()) }
}
#[inline]
pub unsafe fn PathCchCanonicalizeEx<P2>(pszpathout: &mut [u16], pszpathin: P2, dwflags: u32) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCanonicalizeEx(pszpathout : windows_core::PWSTR, cchpathout : usize, pszpathin : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathCchCanonicalizeEx(core::mem::transmute(pszpathout.as_ptr()), pszpathout.len().try_into().unwrap(), pszpathin.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn PathCchCombine<P2, P3>(pszpathout: &mut [u16], pszpathin: P2, pszmore: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCombine(pszpathout : windows_core::PWSTR, cchpathout : usize, pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchCombine(core::mem::transmute(pszpathout.as_ptr()), pszpathout.len().try_into().unwrap(), pszpathin.param().abi(), pszmore.param().abi()) }
}
#[inline]
pub unsafe fn PathCchCombineEx<P2, P3>(pszpathout: &mut [u16], pszpathin: P2, pszmore: P3, dwflags: u32) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCombineEx(pszpathout : windows_core::PWSTR, cchpathout : usize, pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathCchCombineEx(core::mem::transmute(pszpathout.as_ptr()), pszpathout.len().try_into().unwrap(), pszpathin.param().abi(), pszmore.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn PathCchFindExtension<P0>(pszpath: P0, cchpath: usize) -> windows_core::Result<windows_core::PCWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchFindExtension(pszpath : windows_core::PCWSTR, cchpath : usize, ppszext : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PathCchFindExtension(pszpath.param().abi(), cchpath, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PathCchIsRoot<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchIsRoot(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathCchIsRoot(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathCchRemoveBackslash(pszpath: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveBackslash(pszpath : windows_core::PWSTR, cchpath : usize) -> windows_core::HRESULT);
    unsafe { PathCchRemoveBackslash(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathCchRemoveBackslashEx(pszpath: windows_core::PWSTR, cchpath: usize, ppszend: *mut windows_core::PWSTR, pcchremaining: Option<*mut usize>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveBackslashEx(pszpath : windows_core::PWSTR, cchpath : usize, ppszend : *mut windows_core::PWSTR, pcchremaining : *mut usize) -> windows_core::HRESULT);
    unsafe { PathCchRemoveBackslashEx(core::mem::transmute(pszpath), cchpath, ppszend as _, pcchremaining.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PathCchRemoveExtension(pszpath: windows_core::PWSTR, cchpath: usize) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveExtension(pszpath : windows_core::PWSTR, cchpath : usize) -> windows_core::HRESULT);
    unsafe { PathCchRemoveExtension(core::mem::transmute(pszpath), cchpath) }
}
#[inline]
pub unsafe fn PathCchRemoveFileSpec(pszpath: windows_core::PWSTR, cchpath: usize) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveFileSpec(pszpath : windows_core::PWSTR, cchpath : usize) -> windows_core::HRESULT);
    unsafe { PathCchRemoveFileSpec(core::mem::transmute(pszpath), cchpath) }
}
#[inline]
pub unsafe fn PathCchRenameExtension<P2>(pszpath: &mut [u16], pszext: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRenameExtension(pszpath : windows_core::PWSTR, cchpath : usize, pszext : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchRenameExtension(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), pszext.param().abi()) }
}
#[inline]
pub unsafe fn PathCchSkipRoot<P0>(pszpath: P0) -> windows_core::Result<windows_core::PCWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchSkipRoot(pszpath : windows_core::PCWSTR, ppszrootend : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PathCchSkipRoot(pszpath.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PathCchStripPrefix(pszpath: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchStripPrefix(pszpath : windows_core::PWSTR, cchpath : usize) -> windows_core::HRESULT);
    unsafe { PathCchStripPrefix(core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathCchStripToRoot(pszpath: windows_core::PWSTR, cchpath: usize) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchStripToRoot(pszpath : windows_core::PWSTR, cchpath : usize) -> windows_core::HRESULT);
    unsafe { PathCchStripToRoot(core::mem::transmute(pszpath), cchpath) }
}
#[inline]
pub unsafe fn PathIsUNCEx<P0>(pszpath: P0, ppszserver: *mut windows_core::PCWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathIsUNCEx(pszpath : windows_core::PCWSTR, ppszserver : *mut windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCEx(pszpath.param().abi(), ppszserver as _) }
}
pub const PATHCCH_ALLOW_LONG_PATHS: PATHCCH_OPTIONS = 1;
pub const PATHCCH_CANONICALIZE_SLASHES: PATHCCH_OPTIONS = 64;
pub const PATHCCH_DO_NOT_NORMALIZE_SEGMENTS: PATHCCH_OPTIONS = 8;
pub const PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH: PATHCCH_OPTIONS = 16;
pub const PATHCCH_ENSURE_TRAILING_SLASH: PATHCCH_OPTIONS = 32;
pub const PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS: PATHCCH_OPTIONS = 4;
pub const PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS: PATHCCH_OPTIONS = 2;
pub const PATHCCH_MAX_CCH: u32 = 32768;
pub const PATHCCH_NONE: PATHCCH_OPTIONS = 0;
pub type PATHCCH_OPTIONS = u32;
pub const VOLUME_PREFIX: windows_core::PCWSTR = windows_core::w!("\\\\?\\Volume");
pub const VOLUME_PREFIX_LEN: u32 = 10;
