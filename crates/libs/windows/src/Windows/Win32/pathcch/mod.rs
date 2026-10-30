#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathAllocCanonicalize<P0>(pszpathin: P0, dwflags: super::ULONG, ppszpathout: *mut windows_core::PWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathAllocCanonicalize(pszpathin : windows_core::PCWSTR, dwflags : super::ULONG, ppszpathout : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { PathAllocCanonicalize(pszpathin.param().abi(), dwflags, ppszpathout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathAllocCombine<P0, P1>(pszpathin: P0, pszmore: P1, dwflags: super::ULONG, ppszpathout: *mut windows_core::PWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathAllocCombine(pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR, dwflags : super::ULONG, ppszpathout : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { PathAllocCombine(pszpathin.param().abi(), pszmore.param().abi(), dwflags, ppszpathout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchAddBackslash(pszpath: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAddBackslash(pszpath : windows_core::PWSTR, cchpath : super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchAddBackslash(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchAddBackslashEx(pszpath: &mut [u16], ppszend: Option<*mut windows_core::PWSTR>, pcchremaining: Option<*mut super::size_t>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAddBackslashEx(pszpath : windows_core::PWSTR, cchpath : super::size_t, ppszend : *mut windows_core::PWSTR, pcchremaining : *mut super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchAddBackslashEx(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap(), ppszend.unwrap_or(core::mem::zeroed()) as _, pcchremaining.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchAddExtension<P2>(pszpath: &mut [u16], pszext: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAddExtension(pszpath : windows_core::PWSTR, cchpath : super::size_t, pszext : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchAddExtension(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap(), pszext.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchAppend<P2>(pszpath: &mut [u16], pszmore: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAppend(pszpath : windows_core::PWSTR, cchpath : super::size_t, pszmore : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchAppend(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap(), pszmore.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchAppendEx<P2>(pszpath: &mut [u16], pszmore: P2, dwflags: super::ULONG) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchAppendEx(pszpath : windows_core::PWSTR, cchpath : super::size_t, pszmore : windows_core::PCWSTR, dwflags : super::ULONG) -> windows_core::HRESULT);
    unsafe { PathCchAppendEx(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap(), pszmore.param().abi(), dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchCanonicalize<P2>(pszpathout: windows_core::PWSTR, cchpathout: super::size_t, pszpathin: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCanonicalize(pszpathout : windows_core::PWSTR, cchpathout : super::size_t, pszpathin : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchCanonicalize(pszpathout, cchpathout, pszpathin.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchCanonicalizeEx<P2>(pszpathout: windows_core::PWSTR, cchpathout: super::size_t, pszpathin: P2, dwflags: super::ULONG) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCanonicalizeEx(pszpathout : windows_core::PWSTR, cchpathout : super::size_t, pszpathin : windows_core::PCWSTR, dwflags : super::ULONG) -> windows_core::HRESULT);
    unsafe { PathCchCanonicalizeEx(pszpathout, cchpathout, pszpathin.param().abi(), dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchCombine<P2, P3>(pszpathout: windows_core::PWSTR, cchpathout: super::size_t, pszpathin: P2, pszmore: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCombine(pszpathout : windows_core::PWSTR, cchpathout : super::size_t, pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchCombine(pszpathout, cchpathout, pszpathin.param().abi(), pszmore.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchCombineEx<P2, P3>(pszpathout: windows_core::PWSTR, cchpathout: super::size_t, pszpathin: P2, pszmore: P3, dwflags: super::ULONG) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCombineEx(pszpathout : windows_core::PWSTR, cchpathout : super::size_t, pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR, dwflags : super::ULONG) -> windows_core::HRESULT);
    unsafe { PathCchCombineEx(pszpathout, cchpathout, pszpathin.param().abi(), pszmore.param().abi(), dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchFindExtension(pszpath: windows_core::PCWSTR, cchpath: super::size_t, ppszext: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchFindExtension(pszpath : windows_core::PCWSTR, cchpath : super::size_t, ppszext : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchFindExtension(pszpath, cchpath, ppszext as _) }
}
#[inline]
pub unsafe fn PathCchIsRoot<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchIsRoot(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathCchIsRoot(pszpath.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchRemoveBackslash(pszpath: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveBackslash(pszpath : windows_core::PWSTR, cchpath : super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchRemoveBackslash(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchRemoveBackslashEx(pszpath: windows_core::PWSTR, cchpath: super::size_t, ppszend: Option<*mut windows_core::PWSTR>, pcchremaining: Option<*mut super::size_t>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveBackslashEx(pszpath : windows_core::PWSTR, cchpath : super::size_t, ppszend : *mut windows_core::PWSTR, pcchremaining : *mut super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchRemoveBackslashEx(pszpath, cchpath, ppszend.unwrap_or(core::mem::zeroed()) as _, pcchremaining.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchRemoveExtension(pszpath: windows_core::PWSTR, cchpath: super::size_t) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveExtension(pszpath : windows_core::PWSTR, cchpath : super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchRemoveExtension(pszpath, cchpath) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchRemoveFileSpec(pszpath: windows_core::PWSTR, cchpath: super::size_t) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRemoveFileSpec(pszpath : windows_core::PWSTR, cchpath : super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchRemoveFileSpec(pszpath, cchpath) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchRenameExtension<P2>(pszpath: &mut [u16], pszext: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchRenameExtension(pszpath : windows_core::PWSTR, cchpath : super::size_t, pszext : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchRenameExtension(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap(), pszext.param().abi()) }
}
#[inline]
pub unsafe fn PathCchSkipRoot<P0>(pszpath: P0, ppszrootend: *mut windows_core::PCWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchSkipRoot(pszpath : windows_core::PCWSTR, ppszrootend : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PathCchSkipRoot(pszpath.param().abi(), ppszrootend as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchStripPrefix(pszpath: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchStripPrefix(pszpath : windows_core::PWSTR, cchpath : super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchStripPrefix(core::mem::transmute(pszpath.as_mut_ptr()), pszpath.len().try_into().unwrap()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathCchStripToRoot(pszpath: windows_core::PWSTR, cchpath: super::size_t) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchStripToRoot(pszpath : windows_core::PWSTR, cchpath : super::size_t) -> windows_core::HRESULT);
    unsafe { PathCchStripToRoot(pszpath, cchpath) }
}
#[inline]
pub unsafe fn PathIsUNCEx<P0>(pszpath: P0, ppszserver: Option<*mut windows_core::PCWSTR>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathIsUNCEx(pszpath : windows_core::PCWSTR, ppszserver : *mut windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCEx(pszpath.param().abi(), ppszserver.unwrap_or(core::mem::zeroed()) as _) }
}
pub const PATHCCH_ALLOW_LONG_PATHS: PATHCCH_OPTIONS = 1;
pub const PATHCCH_CANONICALIZE_SLASHES: PATHCCH_OPTIONS = 64;
pub const PATHCCH_DO_NOT_NORMALIZE_SEGMENTS: PATHCCH_OPTIONS = 8;
pub const PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH: PATHCCH_OPTIONS = 16;
pub const PATHCCH_ENSURE_TRAILING_SLASH: PATHCCH_OPTIONS = 32;
pub const PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS: PATHCCH_OPTIONS = 4;
pub const PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS: PATHCCH_OPTIONS = 2;
pub const PATHCCH_MAX_CCH: i32 = 32768;
pub const PATHCCH_NONE: PATHCCH_OPTIONS = 0;
pub type PATHCCH_OPTIONS = i32;
pub const VOLUME_PREFIX: windows_core::PCWSTR = windows_core::w!("\\\\?\\Volume");
#[cfg(target_arch = "x86")]
pub const VOLUME_PREFIX_LEN: u32 = 10;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const VOLUME_PREFIX_LEN: u64 = 10;
