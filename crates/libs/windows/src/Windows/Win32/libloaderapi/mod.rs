#[inline]
pub unsafe fn AddDllDirectory<P0>(newdirectory: P0) -> DLL_DIRECTORY_COOKIE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddDllDirectory(newdirectory : windows_core::PCWSTR) -> DLL_DIRECTORY_COOKIE);
    unsafe { AddDllDirectory(newdirectory.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DisableThreadLibraryCalls(hlibmodule: super::HMODULE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DisableThreadLibraryCalls(hlibmodule : super::HMODULE) -> windows_core::BOOL);
    unsafe { DisableThreadLibraryCalls(hlibmodule) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumResourceLanguagesExA<P1, P2>(hmodule: Option<super::HMODULE>, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: Option<isize>, dwflags: u32, langid: super::LANGID) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesExA(hmodule : super::HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize, dwflags : u32, langid : super::LANGID) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesExA(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam.unwrap_or(core::mem::zeroed()) as _, dwflags, langid) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumResourceLanguagesExW<P1, P2>(hmodule: Option<super::HMODULE>, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: Option<isize>, dwflags: u32, langid: super::LANGID) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesExW(hmodule : super::HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize, dwflags : u32, langid : super::LANGID) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesExW(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam.unwrap_or(core::mem::zeroed()) as _, dwflags, langid) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn EnumResourceNamesA<P1>(hmodule: Option<super::HMODULE>, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesA(hmodule : super::HMODULE, lptype : windows_core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceNamesA(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpenumfunc, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumResourceNamesExA<P1>(hmodule: Option<super::HMODULE>, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: super::LANGID) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesExA(hmodule : super::HMODULE, lptype : windows_core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize, dwflags : u32, langid : super::LANGID) -> windows_core::BOOL);
    unsafe { EnumResourceNamesExA(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpenumfunc, lparam, dwflags, langid) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumResourceNamesExW<P1>(hmodule: Option<super::HMODULE>, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: super::LANGID) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesExW(hmodule : super::HMODULE, lptype : windows_core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize, dwflags : u32, langid : super::LANGID) -> windows_core::BOOL);
    unsafe { EnumResourceNamesExW(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpenumfunc, lparam, dwflags, langid) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn EnumResourceNamesW<P1>(hmodule: Option<super::HMODULE>, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceNamesW(hmodule : super::HMODULE, lptype : windows_core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize) -> windows_core::BOOL);
    unsafe { EnumResourceNamesW(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpenumfunc, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumResourceTypesExA(hmodule: Option<super::HMODULE>, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: super::LANGID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesExA(hmodule : super::HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize, dwflags : u32, langid : super::LANGID) -> windows_core::BOOL);
    unsafe { EnumResourceTypesExA(hmodule.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam, dwflags, langid) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumResourceTypesExW(hmodule: Option<super::HMODULE>, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: super::LANGID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesExW(hmodule : super::HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize, dwflags : u32, langid : super::LANGID) -> windows_core::BOOL);
    unsafe { EnumResourceTypesExW(hmodule.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam, dwflags, langid) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FindResourceExW<P1, P2>(hmodule: Option<super::HMODULE>, lptype: P1, lpname: P2, wlanguage: u16) -> super::HRSRC
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceExW(hmodule : super::HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, wlanguage : u16) -> super::HRSRC);
    unsafe { FindResourceExW(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpname.param().abi(), wlanguage) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FindResourceW<P1, P2>(hmodule: Option<super::HMODULE>, lpname: P1, lptype: P2) -> super::HRSRC
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceW(hmodule : super::HMODULE, lpname : windows_core::PCWSTR, lptype : windows_core::PCWSTR) -> super::HRSRC);
    unsafe { FindResourceW(hmodule.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lptype.param().abi()) }
}
#[inline]
pub unsafe fn FindStringOrdinal(dwfindstringordinalflags: u32, lpstringsource: &[u16], lpstringvalue: &[u16], bignorecase: bool) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn FindStringOrdinal(dwfindstringordinalflags : u32, lpstringsource : windows_core::PCWSTR, cchsource : i32, lpstringvalue : windows_core::PCWSTR, cchvalue : i32, bignorecase : windows_core::BOOL) -> i32);
    unsafe { FindStringOrdinal(dwfindstringordinalflags, core::mem::transmute(lpstringsource.as_ptr()), lpstringsource.len().try_into().unwrap(), core::mem::transmute(lpstringvalue.as_ptr()), lpstringvalue.len().try_into().unwrap(), bignorecase.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FreeLibrary(hlibmodule: super::HMODULE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : super::HMODULE) -> windows_core::BOOL);
    unsafe { FreeLibrary(hlibmodule) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FreeLibraryAndExitThread(hlibmodule: super::HMODULE, dwexitcode: u32) -> ! {
    windows_core::link!("kernel32.dll" "system" fn FreeLibraryAndExitThread(hlibmodule : super::HMODULE, dwexitcode : u32) -> !);
    unsafe { FreeLibraryAndExitThread(hlibmodule, dwexitcode) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FreeResource(hresdata: super::HGLOBAL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeResource(hresdata : super::HGLOBAL) -> windows_core::BOOL);
    unsafe { FreeResource(hresdata) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetModuleFileNameA(hmodule: Option<super::HMODULE>, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetModuleFileNameA(hmodule : super::HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameA(hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpfilename.as_mut_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetModuleFileNameW(hmodule: Option<super::HMODULE>, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetModuleFileNameW(hmodule : super::HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameW(hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpfilename.as_mut_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetModuleHandleA<P0>(lpmodulename: P0) -> super::HMODULE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleA(lpmodulename : windows_core::PCSTR) -> super::HMODULE);
    unsafe { GetModuleHandleA(lpmodulename.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetModuleHandleExA<P1>(dwflags: u32, lpmodulename: P1, phmodule: *mut super::HMODULE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleExA(dwflags : u32, lpmodulename : windows_core::PCSTR, phmodule : *mut super::HMODULE) -> windows_core::BOOL);
    unsafe { GetModuleHandleExA(dwflags, lpmodulename.param().abi(), phmodule as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetModuleHandleExW<P1>(dwflags: u32, lpmodulename: P1, phmodule: *mut super::HMODULE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleExW(dwflags : u32, lpmodulename : windows_core::PCWSTR, phmodule : *mut super::HMODULE) -> windows_core::BOOL);
    unsafe { GetModuleHandleExW(dwflags, lpmodulename.param().abi(), phmodule as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetModuleHandleW<P0>(lpmodulename: P0) -> super::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_core::PCWSTR) -> super::HMODULE);
    unsafe { GetModuleHandleW(lpmodulename.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetProcAddress<P1>(hmodule: super::HMODULE, lpprocname: P1) -> super::FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::HMODULE, lpprocname : windows_core::PCSTR) -> super::FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadLibraryA<P0>(lplibfilename: P0) -> super::HMODULE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryA(lplibfilename : windows_core::PCSTR) -> super::HMODULE);
    unsafe { LoadLibraryA(lplibfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LoadLibraryExA<P0>(lplibfilename: P0, hfile: Option<super::HANDLE>, dwflags: u32) -> super::HMODULE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_core::PCSTR, hfile : super::HANDLE, dwflags : u32) -> super::HMODULE);
    unsafe { LoadLibraryExA(lplibfilename.param().abi(), hfile.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LoadLibraryExW<P0>(lplibfilename: P0, hfile: Option<super::HANDLE>, dwflags: u32) -> super::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryExW(lplibfilename : windows_core::PCWSTR, hfile : super::HANDLE, dwflags : u32) -> super::HMODULE);
    unsafe { LoadLibraryExW(lplibfilename.param().abi(), hfile.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> super::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> super::HMODULE);
    unsafe { LoadLibraryW(lplibfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LoadResource(hmodule: Option<super::HMODULE>, hresinfo: super::HRSRC) -> super::HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn LoadResource(hmodule : super::HMODULE, hresinfo : super::HRSRC) -> super::HGLOBAL);
    unsafe { LoadResource(hmodule.unwrap_or(core::mem::zeroed()) as _, hresinfo) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadStringA(hinstance: Option<super::HINSTANCE>, uid: u32, lpbuffer: &mut [u8]) -> i32 {
    windows_core::link!("user32.dll" "system" fn LoadStringA(hinstance : super::HINSTANCE, uid : u32, lpbuffer : windows_core::PSTR, cchbuffermax : i32) -> i32);
    unsafe { LoadStringA(hinstance.unwrap_or(core::mem::zeroed()) as _, uid, core::mem::transmute(lpbuffer.as_mut_ptr()), lpbuffer.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadStringW(hinstance: Option<super::HINSTANCE>, uid: u32, lpbuffer: &mut [u16]) -> i32 {
    windows_core::link!("user32.dll" "system" fn LoadStringW(hinstance : super::HINSTANCE, uid : u32, lpbuffer : windows_core::PWSTR, cchbuffermax : i32) -> i32);
    unsafe { LoadStringW(hinstance.unwrap_or(core::mem::zeroed()) as _, uid, core::mem::transmute(lpbuffer.as_mut_ptr()), lpbuffer.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LockResource(hresdata: super::HGLOBAL) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn LockResource(hresdata : super::HGLOBAL) -> *mut core::ffi::c_void);
    unsafe { LockResource(hresdata) }
}
#[inline]
pub unsafe fn RemoveDllDirectory(cookie: DLL_DIRECTORY_COOKIE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RemoveDllDirectory(cookie : DLL_DIRECTORY_COOKIE) -> windows_core::BOOL);
    unsafe { RemoveDllDirectory(cookie) }
}
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetDefaultDllDirectories(directoryflags : u32) -> windows_core::BOOL);
    unsafe { SetDefaultDllDirectories(directoryflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SizeofResource(hmodule: Option<super::HMODULE>, hresinfo: super::HRSRC) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SizeofResource(hmodule : super::HMODULE, hresinfo : super::HRSRC) -> u32);
    unsafe { SizeofResource(hmodule.unwrap_or(core::mem::zeroed()) as _, hresinfo) }
}
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DLL_DIRECTORY_COOKIE(pub *mut core::ffi::c_void);
impl Default for DLL_DIRECTORY_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DONT_RESOLVE_DLL_REFERENCES: u32 = 1;
#[cfg(feature = "minwindef")]
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, wlanguage: u16, lparam: isize) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, wlanguage: u16, lparam: isize) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, lparam: isize) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, lparam: isize) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_core::PCSTR, lparam: isize) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_core::PCWSTR, lparam: isize) -> windows_core::BOOL>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut super::LANGID,
}
#[cfg(feature = "winnt")]
impl Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: u32 = 16;
pub const LOAD_LIBRARY_AS_DATAFILE: u32 = 2;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: u32 = 64;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: u32 = 32;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: u32 = 128;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: u32 = 8192;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: u32 = 512;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 4096;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 256;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 2048;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: u32 = 16384;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: u32 = 1024;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: u32 = 8;
pub type PCREDIRECTION_DESCRIPTOR = *const REDIRECTION_DESCRIPTOR;
pub type PCREDIRECTION_FUNCTION_DESCRIPTOR = *const REDIRECTION_FUNCTION_DESCRIPTOR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDLL_DIRECTORY_COOKIE(pub *mut *mut core::ffi::c_void);
impl Default for PDLL_DIRECTORY_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type PENUMUILANG = *mut ENUMUILANG;
#[cfg(feature = "minwindef")]
pub type PGET_MODULE_HANDLE_EXA = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCSTR, phmodule: *mut super::HMODULE) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PGET_MODULE_HANDLE_EXW = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCWSTR, phmodule: *mut super::HMODULE) -> windows_core::BOOL>;
pub type PREDIRECTION_DESCRIPTOR = *mut REDIRECTION_DESCRIPTOR;
pub type PREDIRECTION_FUNCTION_DESCRIPTOR = *mut REDIRECTION_FUNCTION_DESCRIPTOR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: PCREDIRECTION_FUNCTION_DESCRIPTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const RESOURCE_ENUM_LN: u32 = 1;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16;
pub const RESOURCE_ENUM_MUI: u32 = 2;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8;
pub const SUPPORT_LANG_NUMBER: u32 = 32;
