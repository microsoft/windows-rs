#[inline]
pub unsafe fn AddDllDirectory<P0>(newdirectory: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddDllDirectory(newdirectory : windows_core::PCWSTR) -> *mut core::ffi::c_void);
    unsafe { AddDllDirectory(newdirectory.param().abi()) }
}
#[inline]
pub unsafe fn BeginUpdateResourceA<P0>(pfilename: P0, bdeleteexistingresources: bool) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BeginUpdateResourceA(pfilename : windows_core::PCSTR, bdeleteexistingresources : windows_core::BOOL) -> super::super::Foundation:: HANDLE);
    unsafe { BeginUpdateResourceA(pfilename.param().abi(), bdeleteexistingresources.into()) }
}
#[inline]
pub unsafe fn BeginUpdateResourceW<P0>(pfilename: P0, bdeleteexistingresources: bool) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BeginUpdateResourceW(pfilename : windows_core::PCWSTR, bdeleteexistingresources : windows_core::BOOL) -> super::super::Foundation:: HANDLE);
    unsafe { BeginUpdateResourceW(pfilename.param().abi(), bdeleteexistingresources.into()) }
}
#[inline]
pub unsafe fn DisableThreadLibraryCalls(hlibmodule: super::super::Foundation::HMODULE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DisableThreadLibraryCalls(hlibmodule : super::super::Foundation:: HMODULE) -> windows_core::BOOL);
    unsafe { DisableThreadLibraryCalls(hlibmodule) }
}
#[inline]
pub unsafe fn EndUpdateResourceA(hupdate: super::super::Foundation::HANDLE, fdiscard: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EndUpdateResourceA(hupdate : super::super::Foundation:: HANDLE, fdiscard : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EndUpdateResourceA(hupdate, fdiscard.into()) }
}
#[inline]
pub unsafe fn EndUpdateResourceW(hupdate: super::super::Foundation::HANDLE, fdiscard: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EndUpdateResourceW(hupdate : super::super::Foundation:: HANDLE, fdiscard : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EndUpdateResourceW(hupdate, fdiscard.into()) }
}
#[inline]
pub unsafe fn EnumResourceLanguagesA<P1, P2>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesA(hmodule, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn EnumResourceLanguagesExA<P1, P2>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize, dwflags : u32, langid : u16) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesExA(hmodule, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam, dwflags, langid) }
}
#[inline]
pub unsafe fn EnumResourceLanguagesExW<P1, P2>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize, dwflags : u32, langid : u16) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesExW(hmodule, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam, dwflags, langid) }
}
#[inline]
pub unsafe fn EnumResourceLanguagesW<P1, P2>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesW(hmodule, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn EnumResourceNamesA<P1>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceNamesA(hmodule, lptype.param().abi(), lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn EnumResourceNamesExA<P1>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize, dwflags : u32, langid : u16) -> windows_core::BOOL);
    unsafe { EnumResourceNamesExA(hmodule, lptype.param().abi(), lpenumfunc, lparam, dwflags, langid) }
}
#[inline]
pub unsafe fn EnumResourceNamesExW<P1>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize, dwflags : u32, langid : u16) -> windows_core::BOOL);
    unsafe { EnumResourceNamesExW(hmodule, lptype.param().abi(), lpenumfunc, lparam, dwflags, langid) }
}
#[inline]
pub unsafe fn EnumResourceNamesW<P1>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceNamesW(hmodule, lptype.param().abi(), lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn EnumResourceTypesA(hmodule: super::super::Foundation::HMODULE, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesA(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceTypesA(hmodule, lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn EnumResourceTypesExA(hmodule: super::super::Foundation::HMODULE, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesExA(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize, dwflags : u32, langid : u16) -> windows_core::BOOL);
    unsafe { EnumResourceTypesExA(hmodule, lpenumfunc, lparam, dwflags, langid) }
}
#[inline]
pub unsafe fn EnumResourceTypesExW(hmodule: super::super::Foundation::HMODULE, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesExW(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize, dwflags : u32, langid : u16) -> windows_core::BOOL);
    unsafe { EnumResourceTypesExW(hmodule, lpenumfunc, lparam, dwflags, langid) }
}
#[inline]
pub unsafe fn EnumResourceTypesW(hmodule: super::super::Foundation::HMODULE, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesW(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceTypesW(hmodule, lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn FindResourceA<P1, P2>(hmodule: super::super::Foundation::HMODULE, lpname: P1, lptype: P2) -> super::super::Foundation::HRSRC
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceA(hmodule : super::super::Foundation:: HMODULE, lpname : windows_core::PCSTR, lptype : windows_core::PCSTR) -> super::super::Foundation:: HRSRC);
    unsafe { FindResourceA(hmodule, lpname.param().abi(), lptype.param().abi()) }
}
#[inline]
pub unsafe fn FindResourceExA<P1, P2>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpname: P2, wlanguage: u16) -> super::super::Foundation::HRSRC
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, wlanguage : u16) -> super::super::Foundation:: HRSRC);
    unsafe { FindResourceExA(hmodule, lptype.param().abi(), lpname.param().abi(), wlanguage) }
}
#[inline]
pub unsafe fn FindResourceExW<P1, P2>(hmodule: super::super::Foundation::HMODULE, lptype: P1, lpname: P2, wlanguage: u16) -> super::super::Foundation::HRSRC
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, wlanguage : u16) -> super::super::Foundation:: HRSRC);
    unsafe { FindResourceExW(hmodule, lptype.param().abi(), lpname.param().abi(), wlanguage) }
}
#[inline]
pub unsafe fn FindResourceW<P1, P2>(hmodule: super::super::Foundation::HMODULE, lpname: P1, lptype: P2) -> super::super::Foundation::HRSRC
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceW(hmodule : super::super::Foundation:: HMODULE, lpname : windows_core::PCWSTR, lptype : windows_core::PCWSTR) -> super::super::Foundation:: HRSRC);
    unsafe { FindResourceW(hmodule, lpname.param().abi(), lptype.param().abi()) }
}
#[inline]
pub unsafe fn FreeLibraryAndExitThread(hlibmodule: super::super::Foundation::HMODULE, dwexitcode: u32) {
    windows_core::link!("kernel32.dll" "system" fn FreeLibraryAndExitThread(hlibmodule : super::super::Foundation:: HMODULE, dwexitcode : u32));
    unsafe { FreeLibraryAndExitThread(hlibmodule, dwexitcode) }
}
#[inline]
pub unsafe fn FreeResource(hresdata: super::super::Foundation::HGLOBAL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeResource(hresdata : super::super::Foundation:: HGLOBAL) -> windows_core::BOOL);
    unsafe { FreeResource(hresdata) }
}
#[inline]
pub unsafe fn GetDllDirectoryA<P1>(nbufferlength: u32, lpbuffer: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDllDirectoryA(nbufferlength : u32, lpbuffer : windows_core::PCSTR) -> u32);
    unsafe { GetDllDirectoryA(nbufferlength, lpbuffer.param().abi()) }
}
#[inline]
pub unsafe fn GetDllDirectoryW<P1>(nbufferlength: u32, lpbuffer: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDllDirectoryW(nbufferlength : u32, lpbuffer : windows_core::PCWSTR) -> u32);
    unsafe { GetDllDirectoryW(nbufferlength, lpbuffer.param().abi()) }
}
#[inline]
pub unsafe fn GetModuleFileNameA<P1>(hmodule: super::super::Foundation::HMODULE, lpfilename: P1, nsize: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleFileNameA(hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PCSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameA(hmodule, lpfilename.param().abi(), nsize) }
}
#[inline]
pub unsafe fn GetModuleFileNameW(hmodule: super::super::Foundation::HMODULE, lpfilename: windows_core::PWSTR, nsize: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetModuleFileNameW(hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameW(hmodule, core::mem::transmute(lpfilename), nsize) }
}
#[inline]
pub unsafe fn GetModuleHandleA<P0>(lpmodulename: P0) -> super::super::Foundation::HMODULE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleA(lpmodulename : windows_core::PCSTR) -> super::super::Foundation:: HMODULE);
    unsafe { GetModuleHandleA(lpmodulename.param().abi()) }
}
#[inline]
pub unsafe fn GetModuleHandleExA<P1>(dwflags: u32, lpmodulename: P1, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleExA(dwflags : u32, lpmodulename : windows_core::PCSTR, phmodule : *mut super::super::Foundation:: HMODULE) -> windows_core::BOOL);
    unsafe { GetModuleHandleExA(dwflags, lpmodulename.param().abi(), phmodule as _) }
}
#[inline]
pub unsafe fn GetModuleHandleExW<P1>(dwflags: u32, lpmodulename: P1, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleExW(dwflags : u32, lpmodulename : windows_core::PCWSTR, phmodule : *mut super::super::Foundation:: HMODULE) -> windows_core::BOOL);
    unsafe { GetModuleHandleExW(dwflags, lpmodulename.param().abi(), phmodule as _) }
}
#[inline]
pub unsafe fn GetModuleHandleW<P0>(lpmodulename: P0) -> super::super::Foundation::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_core::PCWSTR) -> super::super::Foundation:: HMODULE);
    unsafe { GetModuleHandleW(lpmodulename.param().abi()) }
}
#[inline]
pub unsafe fn GetProcAddress<P1>(hmodule: super::super::Foundation::HMODULE, lpprocname: P1) -> super::super::Foundation::FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::super::Foundation:: HMODULE, lpprocname : windows_core::PCSTR) -> super::super::Foundation:: FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[inline]
pub unsafe fn LoadLibraryA<P0>(lplibfilename: P0) -> super::super::Foundation::HMODULE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryA(lplibfilename : windows_core::PCSTR) -> super::super::Foundation:: HMODULE);
    unsafe { LoadLibraryA(lplibfilename.param().abi()) }
}
#[inline]
pub unsafe fn LoadLibraryExA<P0>(lplibfilename: P0, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HMODULE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_core::PCSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
    unsafe { LoadLibraryExA(lplibfilename.param().abi(), hfile, dwflags) }
}
#[inline]
pub unsafe fn LoadLibraryExW<P0>(lplibfilename: P0, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryExW(lplibfilename : windows_core::PCWSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
    unsafe { LoadLibraryExW(lplibfilename.param().abi(), hfile, dwflags) }
}
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> super::super::Foundation::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> super::super::Foundation:: HMODULE);
    unsafe { LoadLibraryW(lplibfilename.param().abi()) }
}
#[inline]
pub unsafe fn LoadModule<P0>(lpmodulename: P0, lpparameterblock: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadModule(lpmodulename : windows_core::PCSTR, lpparameterblock : *mut core::ffi::c_void) -> u32);
    unsafe { LoadModule(lpmodulename.param().abi(), lpparameterblock as _) }
}
#[inline]
pub unsafe fn LoadPackagedLibrary<P0>(lpwlibfilename: P0, reserved: u32) -> super::super::Foundation::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadPackagedLibrary(lpwlibfilename : windows_core::PCWSTR, reserved : u32) -> super::super::Foundation:: HMODULE);
    unsafe { LoadPackagedLibrary(lpwlibfilename.param().abi(), reserved) }
}
#[inline]
pub unsafe fn LoadResource(hmodule: super::super::Foundation::HMODULE, hresinfo: super::super::Foundation::HRSRC) -> super::super::Foundation::HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn LoadResource(hmodule : super::super::Foundation:: HMODULE, hresinfo : super::super::Foundation:: HRSRC) -> super::super::Foundation:: HGLOBAL);
    unsafe { LoadResource(hmodule, hresinfo) }
}
#[inline]
pub unsafe fn LockResource(hresdata: super::super::Foundation::HGLOBAL) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn LockResource(hresdata : super::super::Foundation:: HGLOBAL) -> *mut core::ffi::c_void);
    unsafe { LockResource(hresdata) }
}
#[inline]
pub unsafe fn QueryOptionalDelayLoadedAPI<P1, P2>(hparentmodule: super::super::Foundation::HMODULE, lpdllname: P1, lpprocname: P2, reserved: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("api-ms-win-core-libraryloader-l2-1-0.dll" "system" fn QueryOptionalDelayLoadedAPI(hparentmodule : super::super::Foundation:: HMODULE, lpdllname : windows_core::PCSTR, lpprocname : windows_core::PCSTR, reserved : u32) -> windows_core::BOOL);
    unsafe { QueryOptionalDelayLoadedAPI(hparentmodule, lpdllname.param().abi(), lpprocname.param().abi(), reserved) }
}
#[inline]
pub unsafe fn RemoveDllDirectory(cookie: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RemoveDllDirectory(cookie : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RemoveDllDirectory(cookie as _) }
}
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetDefaultDllDirectories(directoryflags : LOAD_LIBRARY_FLAGS) -> windows_core::BOOL);
    unsafe { SetDefaultDllDirectories(directoryflags) }
}
#[inline]
pub unsafe fn SetDllDirectoryA<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDllDirectoryA(lppathname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetDllDirectoryA(lppathname.param().abi()) }
}
#[inline]
pub unsafe fn SetDllDirectoryW<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDllDirectoryW(lppathname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetDllDirectoryW(lppathname.param().abi()) }
}
#[inline]
pub unsafe fn SizeofResource(hmodule: super::super::Foundation::HMODULE, hresinfo: super::super::Foundation::HRSRC) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SizeofResource(hmodule : super::super::Foundation:: HMODULE, hresinfo : super::super::Foundation:: HRSRC) -> u32);
    unsafe { SizeofResource(hmodule, hresinfo) }
}
#[inline]
pub unsafe fn UpdateResourceA<P1, P2>(hupdate: super::super::Foundation::HANDLE, lptype: P1, lpname: P2, wlanguage: u16, lpdata: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn UpdateResourceA(hupdate : super::super::Foundation:: HANDLE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, wlanguage : u16, lpdata : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { UpdateResourceA(hupdate, lptype.param().abi(), lpname.param().abi(), wlanguage, lpdata as _, cb) }
}
#[inline]
pub unsafe fn UpdateResourceW<P1, P2>(hupdate: super::super::Foundation::HANDLE, lptype: P1, lpname: P2, wlanguage: u16, lpdata: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn UpdateResourceW(hupdate : super::super::Foundation:: HANDLE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, wlanguage : u16, lpdata : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { UpdateResourceW(hupdate, lptype.param().abi(), lpname.param().abi(), wlanguage, lpdata as _, cb) }
}
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1u32);
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, wlanguage: u16, lparam: isize) -> windows_core::BOOL>;
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, wlanguage: u16, lparam: isize) -> windows_core::BOOL>;
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, lparam: isize) -> windows_core::BOOL>;
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, lparam: isize) -> windows_core::BOOL>;
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lparam: isize) -> windows_core::BOOL>;
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lparam: isize) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16u32);
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2u32);
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(64u32);
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(32u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOAD_LIBRARY_FLAGS(pub u32);
impl LOAD_LIBRARY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LOAD_LIBRARY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LOAD_LIBRARY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(128u32);
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8192u32);
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(512u32);
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(4096u32);
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(256u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2048u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16384u32);
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1024u32);
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8u32);
pub type PGET_MODULE_HANDLE_EXA = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCSTR, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::BOOL>;
pub type PGET_MODULE_HANDLE_EXW = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCWSTR, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
impl Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: windows_core::PCSTR,
    pub FunctionName: windows_core::PCSTR,
    pub RedirectionTarget: *mut core::ffi::c_void,
}
impl Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
