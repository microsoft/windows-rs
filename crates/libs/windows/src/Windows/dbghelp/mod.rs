#[inline]
pub unsafe fn DbgHelpCreateUserDump<P0>(filename: P0, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDump(filename : windows_core::PCSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DbgHelpCreateUserDump(filename.param().abi(), callback, userdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DbgHelpCreateUserDumpW<P0>(filename: P0, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDumpW(filename : windows_core::PCWSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DbgHelpCreateUserDumpW(filename.param().abi(), callback, userdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumDirTree<P1, P2>(hprocess: Option<super::winnt::HANDLE>, rootpath: P1, inputpathname: P2, outputpathbuffer: Option<windows_core::PSTR>, cb: PENUMDIRTREE_CALLBACK, data: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn EnumDirTree(hprocess : super::winnt::HANDLE, rootpath : windows_core::PCSTR, inputpathname : windows_core::PCSTR, outputpathbuffer : windows_core::PSTR, cb : PENUMDIRTREE_CALLBACK, data : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumDirTree(hprocess.unwrap_or(core::mem::zeroed()) as _, rootpath.param().abi(), inputpathname.param().abi(), outputpathbuffer.unwrap_or(core::mem::zeroed()) as _, cb, data.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumDirTreeW<P1, P2>(hprocess: Option<super::winnt::HANDLE>, rootpath: P1, inputpathname: P2, outputpathbuffer: Option<windows_core::PWSTR>, cb: PENUMDIRTREE_CALLBACKW, data: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn EnumDirTreeW(hprocess : super::winnt::HANDLE, rootpath : windows_core::PCWSTR, inputpathname : windows_core::PCWSTR, outputpathbuffer : windows_core::PWSTR, cb : PENUMDIRTREE_CALLBACKW, data : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumDirTreeW(hprocess.unwrap_or(core::mem::zeroed()) as _, rootpath.param().abi(), inputpathname.param().abi(), outputpathbuffer.unwrap_or(core::mem::zeroed()) as _, cb, data.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumerateLoadedModules(hprocess: super::winnt::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModules(hprocess : super::winnt::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModules(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumerateLoadedModules64(hprocess: super::winnt::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModules64(hprocess : super::winnt::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModules64(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumerateLoadedModulesEx(hprocess: super::winnt::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesEx(hprocess : super::winnt::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModulesEx(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumerateLoadedModulesExW(hprocess: super::winnt::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesExW(hprocess : super::winnt::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModulesExW(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumerateLoadedModulesW64(hprocess: super::winnt::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesW64(hprocess : super::winnt::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModulesW64(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindDebugInfoFile<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: windows_core::PSTR) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindDebugInfoFile(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, debugfilepath : windows_core::PSTR) -> super::winnt::HANDLE);
    unsafe { FindDebugInfoFile(filename.param().abi(), symbolpath.param().abi(), core::mem::transmute(debugfilepath)) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindDebugInfoFileEx<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: windows_core::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindDebugInfoFileEx(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, debugfilepath : windows_core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { FindDebugInfoFileEx(filename.param().abi(), symbolpath.param().abi(), core::mem::transmute(debugfilepath), callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindDebugInfoFileExW<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: windows_core::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindDebugInfoFileExW(filename : windows_core::PCWSTR, symbolpath : windows_core::PCWSTR, debugfilepath : windows_core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { FindDebugInfoFileExW(filename.param().abi(), symbolpath.param().abi(), core::mem::transmute(debugfilepath), callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindExecutableImage<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: windows_core::PSTR) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindExecutableImage(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, imagefilepath : windows_core::PSTR) -> super::winnt::HANDLE);
    unsafe { FindExecutableImage(filename.param().abi(), symbolpath.param().abi(), core::mem::transmute(imagefilepath)) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindExecutableImageEx<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: windows_core::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindExecutableImageEx(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, imagefilepath : windows_core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { FindExecutableImageEx(filename.param().abi(), symbolpath.param().abi(), core::mem::transmute(imagefilepath), callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindExecutableImageExW<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: windows_core::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const core::ffi::c_void) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindExecutableImageExW(filename : windows_core::PCWSTR, symbolpath : windows_core::PCWSTR, imagefilepath : windows_core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { FindExecutableImageExW(filename.param().abi(), symbolpath.param().abi(), core::mem::transmute(imagefilepath), callback, callerdata) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFileInPath<P1, P2>(hprocess: super::winnt::HANDLE, searchpatha: P1, filename: P2, id: *const core::ffi::c_void, two: u32, three: u32, flags: u32, filepath: windows_core::PSTR) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindFileInPath(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PCSTR, filename : windows_core::PCSTR, id : *const core::ffi::c_void, two : u32, three : u32, flags : u32, filepath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { FindFileInPath(hprocess, searchpatha.param().abi(), filename.param().abi(), id, two, three, flags, core::mem::transmute(filepath)) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFileInSearchPath<P1, P2>(hprocess: super::winnt::HANDLE, searchpatha: P1, filename: P2, one: u32, two: u32, three: u32, filepath: windows_core::PSTR) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindFileInSearchPath(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PCSTR, filename : windows_core::PCSTR, one : u32, two : u32, three : u32, filepath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { FindFileInSearchPath(hprocess, searchpatha.param().abi(), filename.param().abi(), one, two, three, core::mem::transmute(filepath)) }
}
#[inline]
pub unsafe fn GetSymLoadError() -> u32 {
    windows_core::link!("dbghelp.dll" "system" fn GetSymLoadError() -> u32);
    unsafe { GetSymLoadError() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetTimestampForLoadedLibrary(module: super::minwindef::HMODULE) -> u32 {
    windows_core::link!("dbghelp.dll" "system" fn GetTimestampForLoadedLibrary(module : super::minwindef::HMODULE) -> u32);
    unsafe { GetTimestampForLoadedLibrary(module) }
}
#[inline]
pub unsafe fn ImageDirectoryEntryToData(base: *const core::ffi::c_void, mappedasimage: bool, directoryentry: u16, size: *mut u32) -> *mut core::ffi::c_void {
    windows_core::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToData(base : *const core::ffi::c_void, mappedasimage : bool, directoryentry : u16, size : *mut u32) -> *mut core::ffi::c_void);
    unsafe { ImageDirectoryEntryToData(base, mappedasimage, directoryentry, size as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImageDirectoryEntryToDataEx(base: *const core::ffi::c_void, mappedasimage: bool, directoryentry: u16, size: *mut u32, foundheader: Option<*mut super::winnt::PIMAGE_SECTION_HEADER>) -> *mut core::ffi::c_void {
    windows_core::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToDataEx(base : *const core::ffi::c_void, mappedasimage : bool, directoryentry : u16, size : *mut u32, foundheader : *mut super::winnt::PIMAGE_SECTION_HEADER) -> *mut core::ffi::c_void);
    unsafe { ImageDirectoryEntryToDataEx(base, mappedasimage, directoryentry, size as _, foundheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImageNtHeader(base: *const core::ffi::c_void) -> super::winnt::PIMAGE_NT_HEADERS {
    windows_core::link!("dbghelp.dll" "system" fn ImageNtHeader(base : *const core::ffi::c_void) -> super::winnt::PIMAGE_NT_HEADERS);
    unsafe { ImageNtHeader(base) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImageRvaToSection(ntheaders: super::winnt::PIMAGE_NT_HEADERS, base: *const core::ffi::c_void, rva: u32) -> super::winnt::PIMAGE_SECTION_HEADER {
    windows_core::link!("dbghelp.dll" "system" fn ImageRvaToSection(ntheaders : super::winnt::PIMAGE_NT_HEADERS, base : *const core::ffi::c_void, rva : u32) -> super::winnt::PIMAGE_SECTION_HEADER);
    unsafe { ImageRvaToSection(ntheaders, base, rva) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImageRvaToVa(ntheaders: super::winnt::PIMAGE_NT_HEADERS, base: *const core::ffi::c_void, rva: u32, lastrvasection: Option<*const super::winnt::PIMAGE_SECTION_HEADER>) -> *mut core::ffi::c_void {
    windows_core::link!("dbghelp.dll" "system" fn ImageRvaToVa(ntheaders : super::winnt::PIMAGE_NT_HEADERS, base : *const core::ffi::c_void, rva : u32, lastrvasection : *const super::winnt::PIMAGE_SECTION_HEADER) -> *mut core::ffi::c_void);
    unsafe { ImageRvaToVa(ntheaders, base, rva, lastrvasection.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ImagehlpApiVersion() -> LPAPI_VERSION {
    windows_core::link!("dbghelp.dll" "system" fn ImagehlpApiVersion() -> LPAPI_VERSION);
    unsafe { ImagehlpApiVersion() }
}
#[inline]
pub unsafe fn ImagehlpApiVersionEx(appversion: *const API_VERSION) -> LPAPI_VERSION {
    windows_core::link!("dbghelp.dll" "system" fn ImagehlpApiVersionEx(appversion : *const API_VERSION) -> LPAPI_VERSION);
    unsafe { ImagehlpApiVersionEx(appversion) }
}
#[inline]
pub unsafe fn MakeSureDirectoryPathExists<P0>(dirpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn MakeSureDirectoryPathExists(dirpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { MakeSureDirectoryPathExists(dirpath.param().abi()) }
}
#[inline]
pub unsafe fn RangeMapAddPeImageSections<P1>(rmaphandle: *const core::ffi::c_void, imagename: P1, mappedimage: *const core::ffi::c_void, mappingbytes: u32, imagebase: u64, usertag: u64, mappingflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn RangeMapAddPeImageSections(rmaphandle : *const core::ffi::c_void, imagename : windows_core::PCWSTR, mappedimage : *const core::ffi::c_void, mappingbytes : u32, imagebase : u64, usertag : u64, mappingflags : u32) -> windows_core::BOOL);
    unsafe { RangeMapAddPeImageSections(rmaphandle, imagename.param().abi(), mappedimage, mappingbytes, imagebase, usertag, mappingflags) }
}
#[inline]
pub unsafe fn RangeMapCreate() -> *mut core::ffi::c_void {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapCreate() -> *mut core::ffi::c_void);
    unsafe { RangeMapCreate() }
}
#[inline]
pub unsafe fn RangeMapFree(rmaphandle: Option<*const core::ffi::c_void>) {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapFree(rmaphandle : *const core::ffi::c_void));
    unsafe { RangeMapFree(rmaphandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RangeMapRead(rmaphandle: *const core::ffi::c_void, offset: u64, buffer: *mut core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapRead(rmaphandle : *const core::ffi::c_void, offset : u64, buffer : *mut core::ffi::c_void, requestbytes : u32, flags : u32, donebytes : *mut u32) -> windows_core::BOOL);
    unsafe { RangeMapRead(rmaphandle, offset, buffer as _, requestbytes, flags, donebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RangeMapRemove(rmaphandle: *const core::ffi::c_void, usertag: u64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapRemove(rmaphandle : *const core::ffi::c_void, usertag : u64) -> windows_core::BOOL);
    unsafe { RangeMapRemove(rmaphandle, usertag) }
}
#[inline]
pub unsafe fn RangeMapWrite(rmaphandle: *const core::ffi::c_void, offset: u64, buffer: *const core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapWrite(rmaphandle : *const core::ffi::c_void, offset : u64, buffer : *const core::ffi::c_void, requestbytes : u32, flags : u32, donebytes : *mut u32) -> windows_core::BOOL);
    unsafe { RangeMapWrite(rmaphandle, offset, buffer, requestbytes, flags, donebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RemoveInvalidModuleList(hprocess: super::winnt::HANDLE) {
    windows_core::link!("dbghelp.dll" "system" fn RemoveInvalidModuleList(hprocess : super::winnt::HANDLE));
    unsafe { RemoveInvalidModuleList(hprocess) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReportSymbolLoadSummary<P1>(hprocess: super::winnt::HANDLE, ploadmodule: P1, psymboldata: *const DBGHELP_DATA_REPORT_STRUCT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn ReportSymbolLoadSummary(hprocess : super::winnt::HANDLE, ploadmodule : windows_core::PCWSTR, psymboldata : *const DBGHELP_DATA_REPORT_STRUCT) -> windows_core::BOOL);
    unsafe { ReportSymbolLoadSummary(hprocess, ploadmodule.param().abi(), psymboldata) }
}
#[inline]
pub unsafe fn SearchTreeForFile<P0, P1>(rootpath: P0, inputpathname: P1, outputpathbuffer: windows_core::PSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SearchTreeForFile(rootpath : windows_core::PCSTR, inputpathname : windows_core::PCSTR, outputpathbuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SearchTreeForFile(rootpath.param().abi(), inputpathname.param().abi(), core::mem::transmute(outputpathbuffer)) }
}
#[inline]
pub unsafe fn SearchTreeForFileW<P0, P1>(rootpath: P0, inputpathname: P1, outputpathbuffer: windows_core::PWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SearchTreeForFileW(rootpath : windows_core::PCWSTR, inputpathname : windows_core::PCWSTR, outputpathbuffer : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SearchTreeForFileW(rootpath.param().abi(), inputpathname.param().abi(), core::mem::transmute(outputpathbuffer)) }
}
#[inline]
pub unsafe fn SetCheckUserInterruptShared(lpstartaddress: LPCALL_BACK_USER_INTERRUPT_ROUTINE) {
    windows_core::link!("dbghelp.dll" "system" fn SetCheckUserInterruptShared(lpstartaddress : LPCALL_BACK_USER_INTERRUPT_ROUTINE));
    unsafe { SetCheckUserInterruptShared(lpstartaddress) }
}
#[inline]
pub unsafe fn SetSymLoadError(error: u32) {
    windows_core::link!("dbghelp.dll" "system" fn SetSymLoadError(error : u32));
    unsafe { SetSymLoadError(error) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StackWalk(machinetype: u32, hprocess: super::winnt::HANDLE, hthread: super::winnt::HANDLE, stackframe: *mut STACKFRAME, contextrecord: *mut core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE, translateaddress: PTRANSLATE_ADDRESS_ROUTINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalk(machinetype : u32, hprocess : super::winnt::HANDLE, hthread : super::winnt::HANDLE, stackframe : *mut STACKFRAME, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE, translateaddress : PTRANSLATE_ADDRESS_ROUTINE) -> windows_core::BOOL);
    unsafe { StackWalk(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StackWalk2(machinetype: u32, hprocess: super::winnt::HANDLE, hthread: super::winnt::HANDLE, stackframe: *mut STACKFRAME_EX, contextrecord: *mut core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64, gettargetattributevalue: PGET_TARGET_ATTRIBUTE_VALUE64, flags: u32) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalk2(machinetype : u32, hprocess : super::winnt::HANDLE, hthread : super::winnt::HANDLE, stackframe : *mut STACKFRAME_EX, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, gettargetattributevalue : PGET_TARGET_ATTRIBUTE_VALUE64, flags : u32) -> windows_core::BOOL);
    unsafe { StackWalk2(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress, gettargetattributevalue, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StackWalk64(machinetype: u32, hprocess: super::winnt::HANDLE, hthread: super::winnt::HANDLE, stackframe: *mut STACKFRAME64, contextrecord: *mut core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalk64(machinetype : u32, hprocess : super::winnt::HANDLE, hthread : super::winnt::HANDLE, stackframe : *mut STACKFRAME64, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64) -> windows_core::BOOL);
    unsafe { StackWalk64(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StackWalkEx(machinetype: u32, hprocess: super::winnt::HANDLE, hthread: super::winnt::HANDLE, stackframe: *mut STACKFRAME_EX, contextrecord: *mut core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64, flags: u32) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalkEx(machinetype : u32, hprocess : super::winnt::HANDLE, hthread : super::winnt::HANDLE, stackframe : *mut STACKFRAME_EX, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, flags : u32) -> windows_core::BOOL);
    unsafe { StackWalkEx(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymAddSourceStream<P2>(hprocess: super::winnt::HANDLE, base: u64, streamfile: P2, buffer: Option<&[u8]>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSourceStream(hprocess : super::winnt::HANDLE, base : u64, streamfile : windows_core::PCSTR, buffer : *const u8, size : usize) -> windows_core::BOOL);
    unsafe { SymAddSourceStream(hprocess, base, streamfile.param().abi(), core::mem::transmute(buffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymAddSourceStreamA<P2>(hprocess: super::winnt::HANDLE, base: u64, streamfile: P2, buffer: Option<&[u8]>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSourceStreamA(hprocess : super::winnt::HANDLE, base : u64, streamfile : windows_core::PCSTR, buffer : *const u8, size : usize) -> windows_core::BOOL);
    unsafe { SymAddSourceStreamA(hprocess, base, streamfile.param().abi(), core::mem::transmute(buffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymAddSourceStreamW<P2>(hprocess: super::winnt::HANDLE, base: u64, filespec: P2, buffer: Option<&[u8]>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSourceStreamW(hprocess : super::winnt::HANDLE, base : u64, filespec : windows_core::PCWSTR, buffer : *const u8, size : usize) -> windows_core::BOOL);
    unsafe { SymAddSourceStreamW(hprocess, base, filespec.param().abi(), core::mem::transmute(buffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymAddSymbol<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, name: P2, address: u64, size: u32, flags: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSymbol(hprocess : super::winnt::HANDLE, baseofdll : u64, name : windows_core::PCSTR, address : u64, size : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SymAddSymbol(hprocess, baseofdll, name.param().abi(), address, size, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymAddSymbolW<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, name: P2, address: u64, size: u32, flags: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSymbolW(hprocess : super::winnt::HANDLE, baseofdll : u64, name : windows_core::PCWSTR, address : u64, size : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SymAddSymbolW(hprocess, baseofdll, name.param().abi(), address, size, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymAddrIncludeInlineTrace(hprocess: super::winnt::HANDLE, address: u64) -> u32 {
    windows_core::link!("dbghelp.dll" "system" fn SymAddrIncludeInlineTrace(hprocess : super::winnt::HANDLE, address : u64) -> u32);
    unsafe { SymAddrIncludeInlineTrace(hprocess, address) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymCleanup(hprocess: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymCleanup(hprocess : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { SymCleanup(hprocess) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymCompareInlineTrace(hprocess: super::winnt::HANDLE, address1: u64, inlinecontext1: u32, retaddress1: u64, address2: u64, retaddress2: u64) -> u32 {
    windows_core::link!("dbghelp.dll" "system" fn SymCompareInlineTrace(hprocess : super::winnt::HANDLE, address1 : u64, inlinecontext1 : u32, retaddress1 : u64, address2 : u64, retaddress2 : u64) -> u32);
    unsafe { SymCompareInlineTrace(hprocess, address1, inlinecontext1, retaddress1, address2, retaddress2) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymDeleteSymbol<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, name: P2, address: u64, flags: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymDeleteSymbol(hprocess : super::winnt::HANDLE, baseofdll : u64, name : windows_core::PCSTR, address : u64, flags : u32) -> windows_core::BOOL);
    unsafe { SymDeleteSymbol(hprocess, baseofdll, name.param().abi(), address, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymDeleteSymbolW<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, name: P2, address: u64, flags: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymDeleteSymbolW(hprocess : super::winnt::HANDLE, baseofdll : u64, name : windows_core::PCWSTR, address : u64, flags : u32) -> windows_core::BOOL);
    unsafe { SymDeleteSymbolW(hprocess, baseofdll, name.param().abi(), address, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumLines<P2, P3>(hprocess: super::winnt::HANDLE, base: u64, obj: P2, file: P3, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumLines(hprocess : super::winnt::HANDLE, base : u64, obj : windows_core::PCSTR, file : windows_core::PCSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumLines(hprocess, base, obj.param().abi(), file.param().abi(), enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumLinesW<P2, P3>(hprocess: super::winnt::HANDLE, base: u64, obj: P2, file: P3, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumLinesW(hprocess : super::winnt::HANDLE, base : u64, obj : windows_core::PCWSTR, file : windows_core::PCWSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumLinesW(hprocess, base, obj.param().abi(), file.param().abi(), enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumProcesses(enumprocessescallback: PSYM_ENUMPROCESSES_CALLBACK, usercontext: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumProcesses(enumprocessescallback : PSYM_ENUMPROCESSES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumProcesses(enumprocessescallback, usercontext) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSourceFileTokens(hprocess: super::winnt::HANDLE, base: u64, callback: PENUMSOURCEFILETOKENSCALLBACK) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceFileTokens(hprocess : super::winnt::HANDLE, base : u64, callback : PENUMSOURCEFILETOKENSCALLBACK) -> windows_core::BOOL);
    unsafe { SymEnumSourceFileTokens(hprocess, base, callback) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSourceFiles<P2>(hprocess: super::winnt::HANDLE, modbase: u64, mask: P2, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceFiles(hprocess : super::winnt::HANDLE, modbase : u64, mask : windows_core::PCSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSourceFiles(hprocess, modbase, mask.param().abi(), cbsrcfiles, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSourceFilesW<P2>(hprocess: super::winnt::HANDLE, modbase: u64, mask: P2, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceFilesW(hprocess : super::winnt::HANDLE, modbase : u64, mask : windows_core::PCWSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSourceFilesW(hprocess, modbase, mask.param().abi(), cbsrcfiles, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSourceLines<P2, P3>(hprocess: super::winnt::HANDLE, base: u64, obj: P2, file: P3, line: Option<u32>, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceLines(hprocess : super::winnt::HANDLE, base : u64, obj : windows_core::PCSTR, file : windows_core::PCSTR, line : u32, flags : u32, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSourceLines(hprocess, base, obj.param().abi(), file.param().abi(), line.unwrap_or(core::mem::zeroed()) as _, flags, enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSourceLinesW<P2, P3>(hprocess: super::winnt::HANDLE, base: u64, obj: P2, file: P3, line: Option<u32>, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceLinesW(hprocess : super::winnt::HANDLE, base : u64, obj : windows_core::PCWSTR, file : windows_core::PCWSTR, line : u32, flags : u32, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSourceLinesW(hprocess, base, obj.param().abi(), file.param().abi(), line.unwrap_or(core::mem::zeroed()) as _, flags, enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSym(hprocess: super::winnt::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSym(hprocess : super::winnt::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSym(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSymbols<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbols(hprocess : super::winnt::HANDLE, baseofdll : u64, mask : windows_core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSymbols(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSymbolsEx<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>, options: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsEx(hprocess : super::winnt::HANDLE, baseofdll : u64, mask : windows_core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void, options : u32) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsEx(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSymbolsExW<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<*const core::ffi::c_void>, options: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsExW(hprocess : super::winnt::HANDLE, baseofdll : u64, mask : windows_core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void, options : u32) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsExW(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSymbolsForAddr(hprocess: super::winnt::HANDLE, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddr(hprocess : super::winnt::HANDLE, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsForAddr(hprocess, address, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSymbolsForAddrW(hprocess: super::winnt::HANDLE, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddrW(hprocess : super::winnt::HANDLE, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsForAddrW(hprocess, address, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumSymbolsW<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsW(hprocess : super::winnt::HANDLE, baseofdll : u64, mask : windows_core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsW(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumTypes(hprocess: super::winnt::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypes(hprocess : super::winnt::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumTypes(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumTypesByName<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypesByName(hprocess : super::winnt::HANDLE, baseofdll : u64, mask : windows_core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumTypesByName(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumTypesByNameW<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypesByNameW(hprocess : super::winnt::HANDLE, baseofdll : u64, mask : windows_core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumTypesByNameW(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumTypesW(hprocess: super::winnt::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypesW(hprocess : super::winnt::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumTypesW(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumerateModules(hprocess: super::winnt::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateModules(hprocess : super::winnt::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumerateModules(hprocess, enummodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumerateModules64(hprocess: super::winnt::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK64, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateModules64(hprocess : super::winnt::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumerateModules64(hprocess, enummodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumerateModulesW64(hprocess: super::winnt::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACKW64, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateModulesW64(hprocess : super::winnt::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACKW64, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumerateModulesW64(hprocess, enummodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumerateSymbols(hprocess: super::winnt::HANDLE, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbols(hprocess : super::winnt::HANDLE, baseofdll : u32, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbols(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumerateSymbols64(hprocess: super::winnt::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbols64(hprocess : super::winnt::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbols64(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumerateSymbolsW(hprocess: super::winnt::HANDLE, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACKW, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW(hprocess : super::winnt::HANDLE, baseofdll : u32, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbolsW(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumerateSymbolsW64(hprocess: super::winnt::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW64(hprocess : super::winnt::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbolsW64(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindDebugInfoFile<P1>(hprocess: super::winnt::HANDLE, filename: P1, debugfilepath: windows_core::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindDebugInfoFile(hprocess : super::winnt::HANDLE, filename : windows_core::PCSTR, debugfilepath : windows_core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { SymFindDebugInfoFile(hprocess, filename.param().abi(), core::mem::transmute(debugfilepath), callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindDebugInfoFileW<P1>(hprocess: super::winnt::HANDLE, filename: P1, debugfilepath: windows_core::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindDebugInfoFileW(hprocess : super::winnt::HANDLE, filename : windows_core::PCWSTR, debugfilepath : windows_core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { SymFindDebugInfoFileW(hprocess, filename.param().abi(), core::mem::transmute(debugfilepath), callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindExecutableImage<P1>(hprocess: super::winnt::HANDLE, filename: P1, imagefilepath: windows_core::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: *const core::ffi::c_void) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindExecutableImage(hprocess : super::winnt::HANDLE, filename : windows_core::PCSTR, imagefilepath : windows_core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { SymFindExecutableImage(hprocess, filename.param().abi(), core::mem::transmute(imagefilepath), callback, callerdata) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindExecutableImageW<P1>(hprocess: super::winnt::HANDLE, filename: P1, imagefilepath: windows_core::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const core::ffi::c_void) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindExecutableImageW(hprocess : super::winnt::HANDLE, filename : windows_core::PCWSTR, imagefilepath : windows_core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { SymFindExecutableImageW(hprocess, filename.param().abi(), core::mem::transmute(imagefilepath), callback, callerdata) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindFileInPath<P1, P2>(hprocess: super::winnt::HANDLE, searchpatha: P1, filename: P2, id: Option<*const core::ffi::c_void>, two: u32, three: u32, flags: u32, foundfile: windows_core::PSTR, callback: PFINDFILEINPATHCALLBACK, context: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindFileInPath(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PCSTR, filename : windows_core::PCSTR, id : *const core::ffi::c_void, two : u32, three : u32, flags : u32, foundfile : windows_core::PSTR, callback : PFINDFILEINPATHCALLBACK, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymFindFileInPath(hprocess, searchpatha.param().abi(), filename.param().abi(), id.unwrap_or(core::mem::zeroed()) as _, two, three, flags, core::mem::transmute(foundfile), callback, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindFileInPathW<P1, P2>(hprocess: super::winnt::HANDLE, searchpatha: P1, filename: P2, id: Option<*const core::ffi::c_void>, two: u32, three: u32, flags: u32, foundfile: windows_core::PWSTR, callback: PFINDFILEINPATHCALLBACKW, context: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindFileInPathW(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PCWSTR, filename : windows_core::PCWSTR, id : *const core::ffi::c_void, two : u32, three : u32, flags : u32, foundfile : windows_core::PWSTR, callback : PFINDFILEINPATHCALLBACKW, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymFindFileInPathW(hprocess, searchpatha.param().abi(), filename.param().abi(), id.unwrap_or(core::mem::zeroed()) as _, two, three, flags, core::mem::transmute(foundfile), callback, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromAddr(hprocess: super::winnt::HANDLE, address: u64, displacement: Option<*mut u64>, symbol: *mut SYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromAddr(hprocess : super::winnt::HANDLE, address : u64, displacement : *mut u64, symbol : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromAddr(hprocess, address, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromAddrW(hprocess: super::winnt::HANDLE, address: u64, displacement: Option<*mut u64>, symbol: *mut SYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromAddrW(hprocess : super::winnt::HANDLE, address : u64, displacement : *mut u64, symbol : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromAddrW(hprocess, address, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromIndex(hprocess: super::winnt::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromIndex(hprocess : super::winnt::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromIndex(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromIndexW(hprocess: super::winnt::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromIndexW(hprocess : super::winnt::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromIndexW(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromInlineContext(hprocess: super::winnt::HANDLE, address: u64, inlinecontext: u32, displacement: Option<*mut u64>, symbol: *mut SYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromInlineContext(hprocess : super::winnt::HANDLE, address : u64, inlinecontext : u32, displacement : *mut u64, symbol : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromInlineContext(hprocess, address, inlinecontext, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromInlineContextW(hprocess: super::winnt::HANDLE, address: u64, inlinecontext: u32, displacement: Option<*mut u64>, symbol: *mut SYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromInlineContextW(hprocess : super::winnt::HANDLE, address : u64, inlinecontext : u32, displacement : *mut u64, symbol : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromInlineContextW(hprocess, address, inlinecontext, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromName<P1>(hprocess: super::winnt::HANDLE, name: P1, symbol: *mut SYMBOL_INFO) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFromName(hprocess : super::winnt::HANDLE, name : windows_core::PCSTR, symbol : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromName(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromNameW<P1>(hprocess: super::winnt::HANDLE, name: P1, symbol: *mut SYMBOL_INFOW) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFromNameW(hprocess : super::winnt::HANDLE, name : windows_core::PCWSTR, symbol : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromNameW(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromToken(hprocess: super::winnt::HANDLE, base: u64, token: u32, symbol: *mut SYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromToken(hprocess : super::winnt::HANDLE, base : u64, token : u32, symbol : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromToken(hprocess, base, token, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFromTokenW(hprocess: super::winnt::HANDLE, base: u64, token: u32, symbol: *mut SYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromTokenW(hprocess : super::winnt::HANDLE, base : u64, token : u32, symbol : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromTokenW(hprocess, base, token, symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFunctionTableAccess(hprocess: super::winnt::HANDLE, addrbase: u32) -> *mut core::ffi::c_void {
    windows_core::link!("dbghelp.dll" "system" fn SymFunctionTableAccess(hprocess : super::winnt::HANDLE, addrbase : u32) -> *mut core::ffi::c_void);
    unsafe { SymFunctionTableAccess(hprocess, addrbase) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFunctionTableAccess64(hprocess: super::winnt::HANDLE, addrbase: u64) -> *mut core::ffi::c_void {
    windows_core::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64(hprocess : super::winnt::HANDLE, addrbase : u64) -> *mut core::ffi::c_void);
    unsafe { SymFunctionTableAccess64(hprocess, addrbase) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFunctionTableAccess64AccessRoutines(hprocess: super::winnt::HANDLE, addrbase: u64, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64) -> *mut core::ffi::c_void {
    windows_core::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64AccessRoutines(hprocess : super::winnt::HANDLE, addrbase : u64, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64) -> *mut core::ffi::c_void);
    unsafe { SymFunctionTableAccess64AccessRoutines(hprocess, addrbase, readmemoryroutine, getmodulebaseroutine) }
}
#[inline]
pub unsafe fn SymGetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS) -> windows_core::BOOL);
    unsafe { SymGetExtendedOption(option) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetFileLineOffsets64<P1, P2>(hprocess: super::winnt::HANDLE, modulename: P1, filename: P2, buffer: &mut [u64]) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetFileLineOffsets64(hprocess : super::winnt::HANDLE, modulename : windows_core::PCSTR, filename : windows_core::PCSTR, buffer : *mut u64, bufferlines : u32) -> u32);
    unsafe { SymGetFileLineOffsets64(hprocess, modulename.param().abi(), filename.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetHomeDirectory(r#type: u32, dir: &mut [u8]) -> super::winnt::PCHAR {
    windows_core::link!("dbghelp.dll" "system" fn SymGetHomeDirectory(r#type : u32, dir : windows_core::PSTR, size : usize) -> super::winnt::PCHAR);
    unsafe { SymGetHomeDirectory(r#type, core::mem::transmute(dir.as_ptr()), dir.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SymGetHomeDirectoryW(r#type: u32, dir: &mut [u16]) -> windows_core::PWSTR {
    windows_core::link!("dbghelp.dll" "system" fn SymGetHomeDirectoryW(r#type : u32, dir : windows_core::PWSTR, size : usize) -> windows_core::PWSTR);
    unsafe { SymGetHomeDirectoryW(r#type, core::mem::transmute(dir.as_ptr()), dir.len().try_into().unwrap()) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromAddr(hprocess: super::winnt::HANDLE, dwaddr: u32, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromAddr(hprocess : super::winnt::HANDLE, dwaddr : u32, pdwdisplacement : *mut u32, line : *mut IMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddr(hprocess, dwaddr, pdwdisplacement as _, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromAddr64(hprocess: super::winnt::HANDLE, qwaddr: u64, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromAddr64(hprocess : super::winnt::HANDLE, qwaddr : u64, pdwdisplacement : *mut u32, line64 : *mut IMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddr64(hprocess, qwaddr, pdwdisplacement as _, line64 as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromAddrW64(hprocess: super::winnt::HANDLE, dwaddr: u64, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromAddrW64(hprocess : super::winnt::HANDLE, dwaddr : u64, pdwdisplacement : *mut u32, line : *mut IMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddrW64(hprocess, dwaddr, pdwdisplacement as _, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromInlineContext(hprocess: super::winnt::HANDLE, qwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: Option<u64>, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContext(hprocess : super::winnt::HANDLE, qwaddr : u64, inlinecontext : u32, qwmodulebaseaddress : u64, pdwdisplacement : *mut u32, line64 : *mut IMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromInlineContext(hprocess, qwaddr, inlinecontext, qwmodulebaseaddress.unwrap_or(core::mem::zeroed()) as _, pdwdisplacement as _, line64 as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromInlineContextW(hprocess: super::winnt::HANDLE, dwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: Option<u64>, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContextW(hprocess : super::winnt::HANDLE, dwaddr : u64, inlinecontext : u32, qwmodulebaseaddress : u64, pdwdisplacement : *mut u32, line : *mut IMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineFromInlineContextW(hprocess, dwaddr, inlinecontext, qwmodulebaseaddress.unwrap_or(core::mem::zeroed()) as _, pdwdisplacement as _, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromName<P1, P2>(hprocess: super::winnt::HANDLE, modulename: P1, filename: P2, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromName(hprocess : super::winnt::HANDLE, modulename : windows_core::PCSTR, filename : windows_core::PCSTR, dwlinenumber : u32, pldisplacement : *mut i32, line : *mut IMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLineFromName(hprocess, modulename.param().abi(), filename.param().abi(), dwlinenumber, pldisplacement as _, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromName64<P1, P2>(hprocess: super::winnt::HANDLE, modulename: P1, filename: P2, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromName64(hprocess : super::winnt::HANDLE, modulename : windows_core::PCSTR, filename : windows_core::PCSTR, dwlinenumber : u32, pldisplacement : *mut i32, line : *mut IMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromName64(hprocess, modulename.param().abi(), filename.param().abi(), dwlinenumber, pldisplacement as _, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineFromNameW64<P1, P2>(hprocess: super::winnt::HANDLE, modulename: P1, filename: P2, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINEW64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromNameW64(hprocess : super::winnt::HANDLE, modulename : windows_core::PCWSTR, filename : windows_core::PCWSTR, dwlinenumber : u32, pldisplacement : *mut i32, line : *mut IMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineFromNameW64(hprocess, modulename.param().abi(), filename.param().abi(), dwlinenumber, pldisplacement as _, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineNext(hprocess: super::winnt::HANDLE, line: *mut IMAGEHLP_LINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineNext(hprocess : super::winnt::HANDLE, line : *mut IMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLineNext(hprocess, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineNext64(hprocess: super::winnt::HANDLE, line: *mut IMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineNext64(hprocess : super::winnt::HANDLE, line : *mut IMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineNext64(hprocess, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLineNextW64(hprocess: super::winnt::HANDLE, line: *mut IMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineNextW64(hprocess : super::winnt::HANDLE, line : *mut IMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineNextW64(hprocess, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLinePrev(hprocess: super::winnt::HANDLE, line: *mut IMAGEHLP_LINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLinePrev(hprocess : super::winnt::HANDLE, line : *mut IMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLinePrev(hprocess, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLinePrev64(hprocess: super::winnt::HANDLE, line: *mut IMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLinePrev64(hprocess : super::winnt::HANDLE, line : *mut IMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLinePrev64(hprocess, line as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetLinePrevW64(hprocess: super::winnt::HANDLE, line: *mut IMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLinePrevW64(hprocess : super::winnt::HANDLE, line : *mut IMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLinePrevW64(hprocess, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetModuleBase(hprocess: super::winnt::HANDLE, dwaddr: u32) -> u32 {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleBase(hprocess : super::winnt::HANDLE, dwaddr : u32) -> u32);
    unsafe { SymGetModuleBase(hprocess, dwaddr) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetModuleBase64(hprocess: super::winnt::HANDLE, qwaddr: u64) -> u64 {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleBase64(hprocess : super::winnt::HANDLE, qwaddr : u64) -> u64);
    unsafe { SymGetModuleBase64(hprocess, qwaddr) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetModuleInfo(hprocess: super::winnt::HANDLE, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfo(hprocess : super::winnt::HANDLE, dwaddr : u32, moduleinfo : *mut IMAGEHLP_MODULE) -> windows_core::BOOL);
    unsafe { SymGetModuleInfo(hprocess, dwaddr, moduleinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetModuleInfo64(hprocess: super::winnt::HANDLE, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfo64(hprocess : super::winnt::HANDLE, qwaddr : u64, moduleinfo : *mut IMAGEHLP_MODULE64) -> windows_core::BOOL);
    unsafe { SymGetModuleInfo64(hprocess, qwaddr, moduleinfo as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetModuleInfoW(hprocess: super::winnt::HANDLE, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULEW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfoW(hprocess : super::winnt::HANDLE, dwaddr : u32, moduleinfo : *mut IMAGEHLP_MODULEW) -> windows_core::BOOL);
    unsafe { SymGetModuleInfoW(hprocess, dwaddr, moduleinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetModuleInfoW64(hprocess: super::winnt::HANDLE, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfoW64(hprocess : super::winnt::HANDLE, qwaddr : u64, moduleinfo : *mut IMAGEHLP_MODULEW64) -> windows_core::BOOL);
    unsafe { SymGetModuleInfoW64(hprocess, qwaddr, moduleinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetOmaps(hprocess: super::winnt::HANDLE, baseofdll: u64, omapto: *mut POMAP, comapto: *mut u64, omapfrom: *mut POMAP, comapfrom: *mut u64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetOmaps(hprocess : super::winnt::HANDLE, baseofdll : u64, omapto : *mut POMAP, comapto : *mut u64, omapfrom : *mut POMAP, comapfrom : *mut u64) -> windows_core::BOOL);
    unsafe { SymGetOmaps(hprocess, baseofdll, omapto as _, comapto as _, omapfrom as _, comapfrom as _) }
}
#[inline]
pub unsafe fn SymGetOptions() -> u32 {
    windows_core::link!("dbghelp.dll" "system" fn SymGetOptions() -> u32);
    unsafe { SymGetOptions() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SymGetParentWindow(phwnd: *mut super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetParentWindow(phwnd : *mut super::windef::HWND) -> windows_core::BOOL);
    unsafe { SymGetParentWindow(phwnd as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetScope(hprocess: super::winnt::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetScope(hprocess : super::winnt::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymGetScope(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetScopeW(hprocess: super::winnt::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetScopeW(hprocess : super::winnt::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymGetScopeW(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSearchPath(hprocess: super::winnt::HANDLE, searchpatha: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSearchPath(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PSTR, searchpathlength : u32) -> windows_core::BOOL);
    unsafe { SymGetSearchPath(hprocess, core::mem::transmute(searchpatha.as_ptr()), searchpatha.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSearchPathW(hprocess: super::winnt::HANDLE, searchpatha: &mut [u16]) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSearchPathW(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PWSTR, searchpathlength : u32) -> windows_core::BOOL);
    unsafe { SymGetSearchPathW(hprocess, core::mem::transmute(searchpatha.as_ptr()), searchpatha.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFile<P2, P3>(hprocess: super::winnt::HANDLE, base: u64, params: P2, filespec: P3, filepath: &mut [u8]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFile(hprocess : super::winnt::HANDLE, base : u64, params : windows_core::PCSTR, filespec : windows_core::PCSTR, filepath : windows_core::PSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFile(hprocess, base, params.param().abi(), filespec.param().abi(), core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileChecksum<P2>(hprocess: super::winnt::HANDLE, base: u64, filespec: P2, pchecksumtype: *mut u32, pchecksum: &mut [u8], pactualbyteswritten: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksum(hprocess : super::winnt::HANDLE, base : u64, filespec : windows_core::PCSTR, pchecksumtype : *mut u32, pchecksum : *mut u8, checksumsize : u32, pactualbyteswritten : *mut u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileChecksum(hprocess, base, filespec.param().abi(), pchecksumtype as _, core::mem::transmute(pchecksum.as_ptr()), pchecksum.len().try_into().unwrap(), pactualbyteswritten as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileChecksumW<P2>(hprocess: super::winnt::HANDLE, base: u64, filespec: P2, pchecksumtype: *mut u32, pchecksum: &mut [u8], pactualbyteswritten: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksumW(hprocess : super::winnt::HANDLE, base : u64, filespec : windows_core::PCWSTR, pchecksumtype : *mut u32, pchecksum : *mut u8, checksumsize : u32, pactualbyteswritten : *mut u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileChecksumW(hprocess, base, filespec.param().abi(), pchecksumtype as _, core::mem::transmute(pchecksum.as_ptr()), pchecksum.len().try_into().unwrap(), pactualbyteswritten as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileFromToken<P2>(hprocess: super::winnt::HANDLE, token: *const core::ffi::c_void, params: P2, filepath: &mut [u8]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromToken(hprocess : super::winnt::HANDLE, token : *const core::ffi::c_void, params : windows_core::PCSTR, filepath : windows_core::PSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromToken(hprocess, token, params.param().abi(), core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenByTokenName<P2, P3>(hprocess: super::winnt::HANDLE, token: *const core::ffi::c_void, tokenname: P2, params: P3, filepath: &mut [u8]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenName(hprocess : super::winnt::HANDLE, token : *const core::ffi::c_void, tokenname : windows_core::PCSTR, params : windows_core::PCSTR, filepath : windows_core::PSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromTokenByTokenName(hprocess, token, tokenname.param().abi(), params.param().abi(), core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenByTokenNameW<P2, P3>(hprocess: super::winnt::HANDLE, token: *const core::ffi::c_void, tokenname: P2, params: P3, filepath: &mut [u16]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenNameW(hprocess : super::winnt::HANDLE, token : *const core::ffi::c_void, tokenname : windows_core::PCWSTR, params : windows_core::PCWSTR, filepath : windows_core::PWSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromTokenByTokenNameW(hprocess, token, tokenname.param().abi(), params.param().abi(), core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenW<P2>(hprocess: super::winnt::HANDLE, token: *const core::ffi::c_void, params: P2, filepath: &mut [u16]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenW(hprocess : super::winnt::HANDLE, token : *const core::ffi::c_void, params : windows_core::PCWSTR, filepath : windows_core::PWSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromTokenW(hprocess, token, params.param().abi(), core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileToken<P2>(hprocess: super::winnt::HANDLE, base: u64, filespec: P2, token: *mut *mut core::ffi::c_void, size: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileToken(hprocess : super::winnt::HANDLE, base : u64, filespec : windows_core::PCSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileToken(hprocess, base, filespec.param().abi(), token as _, size as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileTokenByTokenName<P2, P3, P4>(hprocess: super::winnt::HANDLE, base: u64, filespec: P2, tokenname: P3, tokenparameters: P4, token: *mut *mut core::ffi::c_void, size: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenName(hprocess : super::winnt::HANDLE, base : u64, filespec : windows_core::PCSTR, tokenname : windows_core::PCSTR, tokenparameters : windows_core::PCSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileTokenByTokenName(hprocess, base, filespec.param().abi(), tokenname.param().abi(), tokenparameters.param().abi(), token as _, size as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileTokenByTokenNameW<P2, P3, P4>(hprocess: super::winnt::HANDLE, base: u64, filespec: P2, tokenname: P3, tokenparameters: P4, token: *mut *mut core::ffi::c_void, size: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenNameW(hprocess : super::winnt::HANDLE, base : u64, filespec : windows_core::PCWSTR, tokenname : windows_core::PCWSTR, tokenparameters : windows_core::PCWSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileTokenByTokenNameW(hprocess, base, filespec.param().abi(), tokenname.param().abi(), tokenparameters.param().abi(), token as _, size as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileTokenW<P2>(hprocess: super::winnt::HANDLE, base: u64, filespec: P2, token: *mut *mut core::ffi::c_void, size: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenW(hprocess : super::winnt::HANDLE, base : u64, filespec : windows_core::PCWSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileTokenW(hprocess, base, filespec.param().abi(), token as _, size as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceFileW<P2, P3>(hprocess: super::winnt::HANDLE, base: u64, params: P2, filespec: P3, filepath: &mut [u16]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileW(hprocess : super::winnt::HANDLE, base : u64, params : windows_core::PCWSTR, filespec : windows_core::PCWSTR, filepath : windows_core::PWSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceFileW(hprocess, base, params.param().abi(), filespec.param().abi(), core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceVarFromToken<P2, P3>(hprocess: super::winnt::HANDLE, token: *const core::ffi::c_void, params: P2, varname: P3, value: &mut [u8]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceVarFromToken(hprocess : super::winnt::HANDLE, token : *const core::ffi::c_void, params : windows_core::PCSTR, varname : windows_core::PCSTR, value : windows_core::PSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceVarFromToken(hprocess, token, params.param().abi(), varname.param().abi(), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSourceVarFromTokenW<P2, P3>(hprocess: super::winnt::HANDLE, token: *const core::ffi::c_void, params: P2, varname: P3, value: &mut [u16]) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceVarFromTokenW(hprocess : super::winnt::HANDLE, token : *const core::ffi::c_void, params : windows_core::PCWSTR, varname : windows_core::PCWSTR, value : windows_core::PWSTR, size : u32) -> windows_core::BOOL);
    unsafe { SymGetSourceVarFromTokenW(hprocess, token, params.param().abi(), varname.param().abi(), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymFromAddr(hprocess: super::winnt::HANDLE, dwaddr: u32, pdwdisplacement: Option<*mut u32>, symbol: *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromAddr(hprocess : super::winnt::HANDLE, dwaddr : u32, pdwdisplacement : *mut u32, symbol : *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymFromAddr(hprocess, dwaddr, pdwdisplacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymFromAddr64(hprocess: super::winnt::HANDLE, qwaddr: u64, pdwdisplacement: Option<*mut u64>, symbol: *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromAddr64(hprocess : super::winnt::HANDLE, qwaddr : u64, pdwdisplacement : *mut u64, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymFromAddr64(hprocess, qwaddr, pdwdisplacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymFromName<P1>(hprocess: super::winnt::HANDLE, name: P1, symbol: *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromName(hprocess : super::winnt::HANDLE, name : windows_core::PCSTR, symbol : *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymFromName(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymFromName64<P1>(hprocess: super::winnt::HANDLE, name: P1, symbol: *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromName64(hprocess : super::winnt::HANDLE, name : windows_core::PCSTR, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymFromName64(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymNext(hprocess: super::winnt::HANDLE, symbol: *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymNext(hprocess : super::winnt::HANDLE, symbol : *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymNext(hprocess, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymNext64(hprocess: super::winnt::HANDLE, symbol: *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymNext64(hprocess : super::winnt::HANDLE, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymNext64(hprocess, symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymPrev(hprocess: super::winnt::HANDLE, symbol: *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymPrev(hprocess : super::winnt::HANDLE, symbol : *mut IMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymPrev(hprocess, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymPrev64(hprocess: super::winnt::HANDLE, symbol: *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymPrev64(hprocess : super::winnt::HANDLE, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymPrev64(hprocess, symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymbolFile<P1, P2>(hprocess: Option<super::winnt::HANDLE>, sympath: P1, imagefile: P2, r#type: u32, symbolfile: &mut [u8], dbgfile: &mut [u8]) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymbolFile(hprocess : super::winnt::HANDLE, sympath : windows_core::PCSTR, imagefile : windows_core::PCSTR, r#type : u32, symbolfile : windows_core::PSTR, csymbolfile : usize, dbgfile : windows_core::PSTR, cdbgfile : usize) -> windows_core::BOOL);
    unsafe { SymGetSymbolFile(hprocess.unwrap_or(core::mem::zeroed()) as _, sympath.param().abi(), imagefile.param().abi(), r#type, core::mem::transmute(symbolfile.as_ptr()), symbolfile.len().try_into().unwrap(), core::mem::transmute(dbgfile.as_ptr()), dbgfile.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetSymbolFileW<P1, P2>(hprocess: Option<super::winnt::HANDLE>, sympath: P1, imagefile: P2, r#type: u32, symbolfile: &mut [u16], dbgfile: &mut [u16]) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymbolFileW(hprocess : super::winnt::HANDLE, sympath : windows_core::PCWSTR, imagefile : windows_core::PCWSTR, r#type : u32, symbolfile : windows_core::PWSTR, csymbolfile : usize, dbgfile : windows_core::PWSTR, cdbgfile : usize) -> windows_core::BOOL);
    unsafe { SymGetSymbolFileW(hprocess.unwrap_or(core::mem::zeroed()) as _, sympath.param().abi(), imagefile.param().abi(), r#type, core::mem::transmute(symbolfile.as_ptr()), symbolfile.len().try_into().unwrap(), core::mem::transmute(dbgfile.as_ptr()), dbgfile.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetTypeFromName<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, name: P2, symbol: *mut SYMBOL_INFO) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeFromName(hprocess : super::winnt::HANDLE, baseofdll : u64, name : windows_core::PCSTR, symbol : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymGetTypeFromName(hprocess, baseofdll, name.param().abi(), symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetTypeFromNameW<P2>(hprocess: super::winnt::HANDLE, baseofdll: u64, name: P2, symbol: *mut SYMBOL_INFOW) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeFromNameW(hprocess : super::winnt::HANDLE, baseofdll : u64, name : windows_core::PCWSTR, symbol : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymGetTypeFromNameW(hprocess, baseofdll, name.param().abi(), symbol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetTypeInfo(hprocess: super::winnt::HANDLE, modbase: u64, typeid: u32, gettype: IMAGEHLP_SYMBOL_TYPE_INFO, pinfo: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeInfo(hprocess : super::winnt::HANDLE, modbase : u64, typeid : u32, gettype : IMAGEHLP_SYMBOL_TYPE_INFO, pinfo : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymGetTypeInfo(hprocess, modbase, typeid, gettype, pinfo as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetTypeInfoEx(hprocess: super::winnt::HANDLE, modbase: u64, params: *mut IMAGEHLP_GET_TYPE_INFO_PARAMS) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeInfoEx(hprocess : super::winnt::HANDLE, modbase : u64, params : *mut IMAGEHLP_GET_TYPE_INFO_PARAMS) -> windows_core::BOOL);
    unsafe { SymGetTypeInfoEx(hprocess, modbase, params as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymGetUnwindInfo(hprocess: super::winnt::HANDLE, address: u64, buffer: Option<*mut core::ffi::c_void>, size: *mut u32) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetUnwindInfo(hprocess : super::winnt::HANDLE, address : u64, buffer : *mut core::ffi::c_void, size : *mut u32) -> windows_core::BOOL);
    unsafe { SymGetUnwindInfo(hprocess, address, buffer.unwrap_or(core::mem::zeroed()) as _, size as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymInitialize<P1>(hprocess: super::winnt::HANDLE, usersearchpath: P1, finvadeprocess: bool) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymInitialize(hprocess : super::winnt::HANDLE, usersearchpath : windows_core::PCSTR, finvadeprocess : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymInitialize(hprocess, usersearchpath.param().abi(), finvadeprocess.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymInitializeW<P1>(hprocess: super::winnt::HANDLE, usersearchpath: P1, finvadeprocess: bool) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymInitializeW(hprocess : super::winnt::HANDLE, usersearchpath : windows_core::PCWSTR, finvadeprocess : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymInitializeW(hprocess, usersearchpath.param().abi(), finvadeprocess.into()) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymLoadModule<P2, P3>(hprocess: super::winnt::HANDLE, hfile: Option<super::winnt::HANDLE>, imagename: P2, modulename: P3, baseofdll: u32, sizeofdll: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModule(hprocess : super::winnt::HANDLE, hfile : super::winnt::HANDLE, imagename : windows_core::PCSTR, modulename : windows_core::PCSTR, baseofdll : u32, sizeofdll : u32) -> u32);
    unsafe { SymLoadModule(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, sizeofdll) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymLoadModule64<P2, P3>(hprocess: super::winnt::HANDLE, hfile: Option<super::winnt::HANDLE>, imagename: P2, modulename: P3, baseofdll: u64, sizeofdll: u32) -> u64
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModule64(hprocess : super::winnt::HANDLE, hfile : super::winnt::HANDLE, imagename : windows_core::PCSTR, modulename : windows_core::PCSTR, baseofdll : u64, sizeofdll : u32) -> u64);
    unsafe { SymLoadModule64(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, sizeofdll) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymLoadModuleEx<P2, P3>(hprocess: super::winnt::HANDLE, hfile: Option<super::winnt::HANDLE>, imagename: P2, modulename: P3, baseofdll: u64, dllsize: u32, data: Option<*const MODLOAD_DATA>, flags: Option<u32>) -> u64
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModuleEx(hprocess : super::winnt::HANDLE, hfile : super::winnt::HANDLE, imagename : windows_core::PCSTR, modulename : windows_core::PCSTR, baseofdll : u64, dllsize : u32, data : *const MODLOAD_DATA, flags : u32) -> u64);
    unsafe { SymLoadModuleEx(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, dllsize, data.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymLoadModuleExW<P2, P3>(hprocess: super::winnt::HANDLE, hfile: Option<super::winnt::HANDLE>, imagename: P2, modulename: P3, baseofdll: u64, dllsize: u32, data: Option<*const MODLOAD_DATA>, flags: Option<u32>) -> u64
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModuleExW(hprocess : super::winnt::HANDLE, hfile : super::winnt::HANDLE, imagename : windows_core::PCWSTR, modulename : windows_core::PCWSTR, baseofdll : u64, dllsize : u32, data : *const MODLOAD_DATA, flags : u32) -> u64);
    unsafe { SymLoadModuleExW(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, dllsize, data.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SymMatchFileName<P0, P1>(filename: P0, r#match: P1, filenamestop: *mut windows_core::PSTR, matchstop: *mut windows_core::PSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymMatchFileName(filename : windows_core::PCSTR, r#match : windows_core::PCSTR, filenamestop : *mut windows_core::PSTR, matchstop : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SymMatchFileName(filename.param().abi(), r#match.param().abi(), filenamestop as _, matchstop as _) }
}
#[inline]
pub unsafe fn SymMatchFileNameW<P0, P1>(filename: P0, r#match: P1, filenamestop: *mut windows_core::PWSTR, matchstop: *mut windows_core::PWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymMatchFileNameW(filename : windows_core::PCWSTR, r#match : windows_core::PCWSTR, filenamestop : *mut windows_core::PWSTR, matchstop : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SymMatchFileNameW(filename.param().abi(), r#match.param().abi(), filenamestop as _, matchstop as _) }
}
#[inline]
pub unsafe fn SymMatchString<P0, P1>(string: P0, expression: P1, fcase: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymMatchString(string : windows_core::PCSTR, expression : windows_core::PCSTR, fcase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymMatchString(string.param().abi(), expression.param().abi(), fcase.into()) }
}
#[inline]
pub unsafe fn SymMatchStringA<P0, P1>(string: P0, expression: P1, fcase: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymMatchStringA(string : windows_core::PCSTR, expression : windows_core::PCSTR, fcase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymMatchStringA(string.param().abi(), expression.param().abi(), fcase.into()) }
}
#[inline]
pub unsafe fn SymMatchStringW<P0, P1>(string: P0, expression: P1, fcase: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymMatchStringW(string : windows_core::PCWSTR, expression : windows_core::PCWSTR, fcase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymMatchStringW(string.param().abi(), expression.param().abi(), fcase.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymNext(hprocess: super::winnt::HANDLE, si: *mut SYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymNext(hprocess : super::winnt::HANDLE, si : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymNext(hprocess, si as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymNextW(hprocess: super::winnt::HANDLE, siw: *mut SYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymNextW(hprocess : super::winnt::HANDLE, siw : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymNextW(hprocess, siw as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymPrev(hprocess: super::winnt::HANDLE, si: *mut SYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymPrev(hprocess : super::winnt::HANDLE, si : *mut SYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymPrev(hprocess, si as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymPrevW(hprocess: super::winnt::HANDLE, siw: *mut SYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymPrevW(hprocess : super::winnt::HANDLE, siw : *mut SYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymPrevW(hprocess, siw as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymQueryInlineTrace(hprocess: super::winnt::HANDLE, startaddress: u64, startcontext: u32, startretaddress: u64, curaddress: u64, curcontext: *mut u32, curframeindex: *mut u32) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymQueryInlineTrace(hprocess : super::winnt::HANDLE, startaddress : u64, startcontext : u32, startretaddress : u64, curaddress : u64, curcontext : *mut u32, curframeindex : *mut u32) -> windows_core::BOOL);
    unsafe { SymQueryInlineTrace(hprocess, startaddress, startcontext, startretaddress, curaddress, curcontext as _, curframeindex as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymRefreshModuleList(hprocess: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRefreshModuleList(hprocess : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { SymRefreshModuleList(hprocess) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymRegisterCallback(hprocess: super::winnt::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterCallback(hprocess : super::winnt::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymRegisterCallback(hprocess, callbackfunction, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymRegisterCallback64(hprocess: super::winnt::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterCallback64(hprocess : super::winnt::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : u64) -> windows_core::BOOL);
    unsafe { SymRegisterCallback64(hprocess, callbackfunction, usercontext) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymRegisterCallbackW64(hprocess: super::winnt::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterCallbackW64(hprocess : super::winnt::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : u64) -> windows_core::BOOL);
    unsafe { SymRegisterCallbackW64(hprocess, callbackfunction, usercontext) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymRegisterFunctionEntryCallback(hprocess: super::winnt::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK, usercontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback(hprocess : super::winnt::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SymRegisterFunctionEntryCallback(hprocess, callbackfunction, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymRegisterFunctionEntryCallback64(hprocess: super::winnt::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK64, usercontext: u64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback64(hprocess : super::winnt::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK64, usercontext : u64) -> windows_core::BOOL);
    unsafe { SymRegisterFunctionEntryCallback64(hprocess, callbackfunction, usercontext) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSearch<P4>(hprocess: super::winnt::HANDLE, baseofdll: u64, index: Option<u32>, symtag: Option<u32>, mask: P4, address: Option<u64>, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<*const core::ffi::c_void>, options: u32) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSearch(hprocess : super::winnt::HANDLE, baseofdll : u64, index : u32, symtag : u32, mask : windows_core::PCSTR, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void, options : u32) -> windows_core::BOOL);
    unsafe { SymSearch(hprocess, baseofdll, index.unwrap_or(core::mem::zeroed()) as _, symtag.unwrap_or(core::mem::zeroed()) as _, mask.param().abi(), address.unwrap_or(core::mem::zeroed()) as _, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSearchW<P4>(hprocess: super::winnt::HANDLE, baseofdll: u64, index: Option<u32>, symtag: Option<u32>, mask: P4, address: Option<u64>, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<*const core::ffi::c_void>, options: u32) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSearchW(hprocess : super::winnt::HANDLE, baseofdll : u64, index : u32, symtag : u32, mask : windows_core::PCWSTR, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void, options : u32) -> windows_core::BOOL);
    unsafe { SymSearchW(hprocess, baseofdll, index.unwrap_or(core::mem::zeroed()) as _, symtag.unwrap_or(core::mem::zeroed()) as _, mask.param().abi(), address.unwrap_or(core::mem::zeroed()) as _, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetContext(hprocess: super::winnt::HANDLE, stackframe: *const IMAGEHLP_STACK_FRAME, context: Option<PIMAGEHLP_CONTEXT>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetContext(hprocess : super::winnt::HANDLE, stackframe : *const IMAGEHLP_STACK_FRAME, context : PIMAGEHLP_CONTEXT) -> windows_core::BOOL);
    unsafe { SymSetContext(hprocess, stackframe, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SymSetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS, value: bool) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS, value : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymSetExtendedOption(option, value.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetHomeDirectory<P1>(hprocess: Option<super::winnt::HANDLE>, dir: P1) -> super::winnt::PCHAR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetHomeDirectory(hprocess : super::winnt::HANDLE, dir : windows_core::PCSTR) -> super::winnt::PCHAR);
    unsafe { SymSetHomeDirectory(hprocess.unwrap_or(core::mem::zeroed()) as _, dir.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetHomeDirectoryW<P1>(hprocess: Option<super::winnt::HANDLE>, dir: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetHomeDirectoryW(hprocess : super::winnt::HANDLE, dir : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { SymSetHomeDirectoryW(hprocess.unwrap_or(core::mem::zeroed()) as _, dir.param().abi()) }
}
#[inline]
pub unsafe fn SymSetOptions(symoptions: u32) -> u32 {
    windows_core::link!("dbghelp.dll" "system" fn SymSetOptions(symoptions : u32) -> u32);
    unsafe { SymSetOptions(symoptions) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SymSetParentWindow(hwnd: super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetParentWindow(hwnd : super::windef::HWND) -> windows_core::BOOL);
    unsafe { SymSetParentWindow(hwnd) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetScopeFromAddr(hprocess: super::winnt::HANDLE, address: u64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetScopeFromAddr(hprocess : super::winnt::HANDLE, address : u64) -> windows_core::BOOL);
    unsafe { SymSetScopeFromAddr(hprocess, address) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetScopeFromIndex(hprocess: super::winnt::HANDLE, baseofdll: u64, index: u32) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetScopeFromIndex(hprocess : super::winnt::HANDLE, baseofdll : u64, index : u32) -> windows_core::BOOL);
    unsafe { SymSetScopeFromIndex(hprocess, baseofdll, index) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetScopeFromInlineContext(hprocess: super::winnt::HANDLE, address: u64, inlinecontext: u32) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetScopeFromInlineContext(hprocess : super::winnt::HANDLE, address : u64, inlinecontext : u32) -> windows_core::BOOL);
    unsafe { SymSetScopeFromInlineContext(hprocess, address, inlinecontext) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetSearchPath<P1>(hprocess: super::winnt::HANDLE, searchpatha: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetSearchPath(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SymSetSearchPath(hprocess, searchpatha.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetSearchPathW<P1>(hprocess: super::winnt::HANDLE, searchpatha: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetSearchPathW(hprocess : super::winnt::HANDLE, searchpatha : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SymSetSearchPathW(hprocess, searchpatha.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvDeltaName<P1, P2, P3, P4>(hprocess: super::winnt::HANDLE, sympath: P1, r#type: P2, file1: P3, file2: P4) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvDeltaName(hprocess : super::winnt::HANDLE, sympath : windows_core::PCSTR, r#type : windows_core::PCSTR, file1 : windows_core::PCSTR, file2 : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { SymSrvDeltaName(hprocess, sympath.param().abi(), r#type.param().abi(), file1.param().abi(), file2.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvDeltaNameW<P1, P2, P3, P4>(hprocess: super::winnt::HANDLE, sympath: P1, r#type: P2, file1: P3, file2: P4) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvDeltaNameW(hprocess : super::winnt::HANDLE, sympath : windows_core::PCWSTR, r#type : windows_core::PCWSTR, file1 : windows_core::PCWSTR, file2 : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { SymSrvDeltaNameW(hprocess, sympath.param().abi(), r#type.param().abi(), file1.param().abi(), file2.param().abi()) }
}
#[inline]
pub unsafe fn SymSrvGetFileIndexInfo<P0>(file: P0, info: *mut SYMSRV_INDEX_INFO, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfo(file : windows_core::PCSTR, info : *mut SYMSRV_INDEX_INFO, flags : u32) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexInfo(file.param().abi(), info as _, flags) }
}
#[inline]
pub unsafe fn SymSrvGetFileIndexInfoW<P0>(file: P0, info: *mut SYMSRV_INDEX_INFOW, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfoW(file : windows_core::PCWSTR, info : *mut SYMSRV_INDEX_INFOW, flags : u32) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexInfoW(file.param().abi(), info as _, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvGetFileIndexString<P1, P2>(hprocess: super::winnt::HANDLE, srvpath: P1, file: P2, index: &mut [u8], flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexString(hprocess : super::winnt::HANDLE, srvpath : windows_core::PCSTR, file : windows_core::PCSTR, index : windows_core::PSTR, size : usize, flags : u32) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexString(hprocess, srvpath.param().abi(), file.param().abi(), core::mem::transmute(index.as_ptr()), index.len().try_into().unwrap(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvGetFileIndexStringW<P1, P2>(hprocess: super::winnt::HANDLE, srvpath: P1, file: P2, index: &mut [u16], flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexStringW(hprocess : super::winnt::HANDLE, srvpath : windows_core::PCWSTR, file : windows_core::PCWSTR, index : windows_core::PWSTR, size : usize, flags : u32) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexStringW(hprocess, srvpath.param().abi(), file.param().abi(), core::mem::transmute(index.as_ptr()), index.len().try_into().unwrap(), flags) }
}
#[inline]
pub unsafe fn SymSrvGetFileIndexes<P0>(file: P0, id: *mut windows_core::GUID, val1: *mut u32, val2: Option<*mut u32>, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexes(file : windows_core::PCSTR, id : *mut windows_core::GUID, val1 : *mut u32, val2 : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexes(file.param().abi(), id as _, val1 as _, val2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SymSrvGetFileIndexesW<P0>(file: P0, id: *mut windows_core::GUID, val1: *mut u32, val2: Option<*mut u32>, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexesW(file : windows_core::PCWSTR, id : *mut windows_core::GUID, val1 : *mut u32, val2 : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexesW(file.param().abi(), id as _, val1 as _, val2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvGetSupplement<P1, P2, P3>(hprocess: super::winnt::HANDLE, sympath: P1, node: P2, file: P3) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetSupplement(hprocess : super::winnt::HANDLE, sympath : windows_core::PCSTR, node : windows_core::PCSTR, file : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { SymSrvGetSupplement(hprocess, sympath.param().abi(), node.param().abi(), file.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvGetSupplementW<P1, P2, P3>(hprocess: super::winnt::HANDLE, sympath: P1, node: P2, file: P3) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetSupplementW(hprocess : super::winnt::HANDLE, sympath : windows_core::PCWSTR, node : windows_core::PCWSTR, file : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { SymSrvGetSupplementW(hprocess, sympath.param().abi(), node.param().abi(), file.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvIsStore<P1>(hprocess: Option<super::winnt::HANDLE>, path: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvIsStore(hprocess : super::winnt::HANDLE, path : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SymSrvIsStore(hprocess.unwrap_or(core::mem::zeroed()) as _, path.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvIsStoreW<P1>(hprocess: Option<super::winnt::HANDLE>, path: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvIsStoreW(hprocess : super::winnt::HANDLE, path : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SymSrvIsStoreW(hprocess.unwrap_or(core::mem::zeroed()) as _, path.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvStoreFile<P1, P2>(hprocess: super::winnt::HANDLE, srvpath: P1, file: P2, flags: u32) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreFile(hprocess : super::winnt::HANDLE, srvpath : windows_core::PCSTR, file : windows_core::PCSTR, flags : u32) -> windows_core::PCSTR);
    unsafe { SymSrvStoreFile(hprocess, srvpath.param().abi(), file.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvStoreFileW<P1, P2>(hprocess: super::winnt::HANDLE, srvpath: P1, file: P2, flags: u32) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreFileW(hprocess : super::winnt::HANDLE, srvpath : windows_core::PCWSTR, file : windows_core::PCWSTR, flags : u32) -> windows_core::PCWSTR);
    unsafe { SymSrvStoreFileW(hprocess, srvpath.param().abi(), file.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvStoreSupplement<P1, P2, P3>(hprocess: super::winnt::HANDLE, srvpath: P1, node: P2, file: P3, flags: u32) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreSupplement(hprocess : super::winnt::HANDLE, srvpath : windows_core::PCSTR, node : windows_core::PCSTR, file : windows_core::PCSTR, flags : u32) -> windows_core::PCSTR);
    unsafe { SymSrvStoreSupplement(hprocess, srvpath.param().abi(), node.param().abi(), file.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvStoreSupplementW<P1, P2, P3>(hprocess: super::winnt::HANDLE, sympath: P1, node: P2, file: P3, flags: u32) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreSupplementW(hprocess : super::winnt::HANDLE, sympath : windows_core::PCWSTR, node : windows_core::PCWSTR, file : windows_core::PCWSTR, flags : u32) -> windows_core::PCWSTR);
    unsafe { SymSrvStoreSupplementW(hprocess, sympath.param().abi(), node.param().abi(), file.param().abi(), flags) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SymUnDName(sym: *const IMAGEHLP_SYMBOL, undecname: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnDName(sym : *const IMAGEHLP_SYMBOL, undecname : windows_core::PSTR, undecnamelength : u32) -> windows_core::BOOL);
    unsafe { SymUnDName(sym, core::mem::transmute(undecname.as_ptr()), undecname.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SymUnDName64(sym: *const IMAGEHLP_SYMBOL64, undecname: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnDName64(sym : *const IMAGEHLP_SYMBOL64, undecname : windows_core::PSTR, undecnamelength : u32) -> windows_core::BOOL);
    unsafe { SymUnDName64(sym, core::mem::transmute(undecname.as_ptr()), undecname.len().try_into().unwrap()) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymUnloadModule(hprocess: super::winnt::HANDLE, baseofdll: u32) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnloadModule(hprocess : super::winnt::HANDLE, baseofdll : u32) -> windows_core::BOOL);
    unsafe { SymUnloadModule(hprocess, baseofdll) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymUnloadModule64(hprocess: super::winnt::HANDLE, baseofdll: u64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnloadModule64(hprocess : super::winnt::HANDLE, baseofdll : u64) -> windows_core::BOOL);
    unsafe { SymUnloadModule64(hprocess, baseofdll) }
}
#[inline]
pub unsafe fn UnDecorateSymbolName<P0>(name: P0, outputstring: &mut [u8], flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn UnDecorateSymbolName(name : windows_core::PCSTR, outputstring : windows_core::PSTR, maxstringlength : u32, flags : u32) -> u32);
    unsafe { UnDecorateSymbolName(name.param().abi(), core::mem::transmute(outputstring.as_ptr()), outputstring.len().try_into().unwrap(), flags) }
}
#[inline]
pub unsafe fn UnDecorateSymbolNameW<P0>(name: P0, outputstring: &mut [u16], flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn UnDecorateSymbolNameW(name : windows_core::PCWSTR, outputstring : windows_core::PWSTR, maxstringlength : u32, flags : u32) -> u32);
    unsafe { UnDecorateSymbolNameW(name.param().abi(), core::mem::transmute(outputstring.as_ptr()), outputstring.len().try_into().unwrap(), flags) }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADDRESS {
    pub Offset: u32,
    pub Segment: u16,
    pub Mode: ADDRESS_MODE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADDRESS64 {
    pub Offset: u64,
    pub Segment: u16,
    pub Mode: ADDRESS_MODE,
}
pub type ADDRESS_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct API_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Revision: u16,
    pub Reserved: u16,
}
pub const API_VERSION_NUMBER: u32 = 12;
pub const AddrMode1616: ADDRESS_MODE = 0;
pub const AddrMode1632: ADDRESS_MODE = 1;
pub const AddrModeFlat: ADDRESS_MODE = 3;
pub const AddrModeReal: ADDRESS_MODE = 2;
pub const CBA_CHECK_ARM_MACHINE_THUMB_TYPE_OVERRIDE: u32 = 2147483648;
pub const CBA_CHECK_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 1879048192;
pub const CBA_DEBUG_INFO: u32 = 268435456;
pub const CBA_DEFERRED_SYMBOL_LOAD_CANCEL: u32 = 7;
pub const CBA_DEFERRED_SYMBOL_LOAD_COMPLETE: u32 = 2;
pub const CBA_DEFERRED_SYMBOL_LOAD_FAILURE: u32 = 3;
pub const CBA_DEFERRED_SYMBOL_LOAD_PARTIAL: u32 = 32;
pub const CBA_DEFERRED_SYMBOL_LOAD_START: u32 = 1;
pub const CBA_DUPLICATE_SYMBOL: u32 = 5;
pub const CBA_ENGINE_PRESENT: u32 = 1610612736;
pub const CBA_EVENT: u32 = 16;
pub const CBA_MAP_JIT_SYMBOL: u32 = 2684354560;
pub const CBA_QUERY_IF_SYMBOL_DOWNLOAD_DISALLOWED: u32 = 64;
pub const CBA_READ_MEMORY: u32 = 6;
pub const CBA_SET_OPTIONS: u32 = 8;
pub const CBA_SRCSRV_EVENT: u32 = 1073741824;
pub const CBA_SRCSRV_INFO: u32 = 536870912;
pub const CBA_SYMBOLS_UNLOADED: u32 = 4;
pub const CBA_UPDATE_STATUS_BAR: u32 = 1342177280;
pub const CBA_XML_LOG: u32 = 2415919104;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DBGHELP_DATA_REPORT_STRUCT {
    pub pBinPathNonExist: windows_core::PCWSTR,
    pub pSymbolPathNonExist: windows_core::PCWSTR,
}
pub const DBHHEADER_CVMISC: u32 = 2;
pub const DBHHEADER_DEBUGDIRS: u32 = 1;
pub const DBHHEADER_PDBGUID: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISCRIMINATEDUNION_TAG_VALUE {
    pub value: [u8; 16],
    pub valueSizeBytes: u8,
}
impl Default for DISCRIMINATEDUNION_TAG_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSLFLAG_MISMATCHED_DBG: u32 = 2;
pub const DSLFLAG_MISMATCHED_PDB: u32 = 1;
pub const ERROR_IMAGE_NOT_STRIPPED: u32 = 34816;
pub const ERROR_NO_DBG_POINTER: u32 = 34817;
pub const ERROR_NO_PDB_POINTER: u32 = 34818;
pub const ESLFLAG_FULLPATH: u32 = 1;
pub const ESLFLAG_INLINE_SITE: u32 = 16;
pub const ESLFLAG_NEAREST: u32 = 2;
pub const ESLFLAG_NEXT: u32 = 8;
pub const ESLFLAG_PREV: u32 = 4;
pub const EVENT_SRCSPEW: u32 = 100;
pub const EVENT_SRCSPEW_END: u32 = 199;
pub const EVENT_SRCSPEW_START: u32 = 100;
pub const EXT_OUTPUT_VER: u32 = 1;
pub const FLAG_ENGINE_PRESENT: u32 = 4;
pub const FLAG_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 8;
pub const FLAG_OVERRIDE_ARM_MACHINE_TYPE: u32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_CBA_EVENT {
    pub severity: u32,
    pub code: u32,
    pub desc: super::winnt::PCHAR,
    pub object: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_CBA_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_CBA_EVENTW {
    pub severity: u32,
    pub code: u32,
    pub desc: windows_core::PCWSTR,
    pub object: *mut core::ffi::c_void,
}
impl Default for IMAGEHLP_CBA_EVENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_CBA_READ_MEMORY {
    pub addr: u64,
    pub buf: *mut core::ffi::c_void,
    pub bytes: u32,
    pub bytesread: *mut u32,
}
impl Default for IMAGEHLP_CBA_READ_MEMORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IMAGEHLP_CONTEXT = core::ffi::c_void;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [i8; 260],
    pub Reparse: bool,
    pub hFile: super::winnt::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [i8; 260],
    pub Reparse: bool,
    pub hFile: super::winnt::HANDLE,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [u16; 261],
    pub Reparse: bool,
    pub hFile: super::winnt::HANDLE,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL {
    pub SizeOfStruct: u32,
    pub NumberOfDups: u32,
    pub Symbol: PIMAGEHLP_SYMBOL,
    pub SelectedSymbol: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL64 {
    pub SizeOfStruct: u32,
    pub NumberOfDups: u32,
    pub Symbol: PIMAGEHLP_SYMBOL64,
    pub SelectedSymbol: u32,
}
pub type IMAGEHLP_EXTENDED_OPTIONS = i32;
pub const IMAGEHLP_GET_TYPE_INFO_CHILDREN: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_GET_TYPE_INFO_PARAMS {
    pub SizeOfStruct: u32,
    pub Flags: u32,
    pub NumIds: u32,
    pub TypeIds: super::minwindef::PULONG,
    pub TagFilter: u64,
    pub NumReqs: u32,
    pub ReqKinds: *mut IMAGEHLP_SYMBOL_TYPE_INFO,
    pub ReqOffsets: super::basetsd::PULONG_PTR,
    pub ReqSizes: super::minwindef::PULONG,
    pub ReqStride: usize,
    pub BufferSize: usize,
    pub Buffer: *mut core::ffi::c_void,
    pub EntriesMatched: u32,
    pub EntriesFilled: u32,
    pub TagsFound: u64,
    pub AllReqsValid: u64,
    pub NumReqsValid: u32,
    pub ReqsValid: super::basetsd::PULONG64,
}
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
impl Default for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGEHLP_GET_TYPE_INFO_UNCACHED: u32 = 1;
pub type IMAGEHLP_HD_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGEHLP_JIT_SYMBOLMAP {
    pub SizeOfStruct: u32,
    pub Address: u64,
    pub BaseOfImage: u64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_LINE {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: super::winnt::PCHAR,
    pub Address: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_LINE64 {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: super::winnt::PCHAR,
    pub Address: u64,
}
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_LINE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_LINEW {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: super::winnt::PCHAR,
    pub Address: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_LINEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_LINEW64 {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: windows_core::PWSTR,
    pub Address: u64,
}
impl Default for IMAGEHLP_LINEW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_MODULE {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u32,
    pub ImageSize: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub NumSyms: u32,
    pub SymType: SYM_TYPE,
    pub ModuleName: [i8; 32],
    pub ImageName: [i8; 256],
    pub LoadedImageName: [i8; 256],
}
#[cfg(target_arch = "x86")]
impl Default for IMAGEHLP_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_MODULE64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub ImageSize: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub NumSyms: u32,
    pub SymType: SYM_TYPE,
    pub ModuleName: [i8; 32],
    pub ImageName: [i8; 256],
    pub LoadedImageName: [i8; 256],
    pub LoadedPdbName: [i8; 256],
    pub CVSig: u32,
    pub CVData: [i8; 780],
    pub PdbSig: u32,
    pub PdbSig70: windows_core::GUID,
    pub PdbAge: u32,
    pub PdbUnmatched: windows_core::BOOL,
    pub DbgUnmatched: windows_core::BOOL,
    pub LineNumbers: windows_core::BOOL,
    pub GlobalSymbols: windows_core::BOOL,
    pub TypeInfo: windows_core::BOOL,
    pub SourceIndexed: windows_core::BOOL,
    pub Publics: windows_core::BOOL,
    pub MachineType: u32,
    pub Reserved: u32,
}
impl Default for IMAGEHLP_MODULE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGEHLP_MODULE64_EX {
    pub Module: IMAGEHLP_MODULE64,
    pub RegionFlags: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for IMAGEHLP_MODULEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub PdbSig70: windows_core::GUID,
    pub PdbAge: u32,
    pub PdbUnmatched: windows_core::BOOL,
    pub DbgUnmatched: windows_core::BOOL,
    pub LineNumbers: windows_core::BOOL,
    pub GlobalSymbols: windows_core::BOOL,
    pub TypeInfo: windows_core::BOOL,
    pub SourceIndexed: windows_core::BOOL,
    pub Publics: windows_core::BOOL,
    pub MachineType: u32,
    pub Reserved: u32,
}
impl Default for IMAGEHLP_MODULEW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGEHLP_MODULEW64_EX {
    pub Module: IMAGEHLP_MODULEW64,
    pub RegionFlags: u32,
}
pub const IMAGEHLP_MODULE_REGION_ADDITIONAL: u32 = 4;
pub const IMAGEHLP_MODULE_REGION_ALL: u32 = 255;
pub const IMAGEHLP_MODULE_REGION_DLLBASE: u32 = 1;
pub const IMAGEHLP_MODULE_REGION_DLLRANGE: u32 = 2;
pub const IMAGEHLP_MODULE_REGION_JIT: u32 = 8;
pub const IMAGEHLP_RMAP_BIG_ENDIAN: u32 = 2;
pub const IMAGEHLP_RMAP_FIXUP_ARM64X: u32 = 268435456;
pub const IMAGEHLP_RMAP_FIXUP_IMAGEBASE: u32 = 2147483648;
pub const IMAGEHLP_RMAP_IGNORE_MISCOMPARE: u32 = 4;
pub const IMAGEHLP_RMAP_LOAD_RW_DATA_SECTIONS: u32 = 536870912;
pub const IMAGEHLP_RMAP_MAPPED_FLAT: u32 = 1;
pub const IMAGEHLP_RMAP_NOBASERELOCATIONS: u32 = 134217728;
pub const IMAGEHLP_RMAP_OMIT_SHARED_RW_DATA_SECTIONS: u32 = 1073741824;
pub type IMAGEHLP_SF_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_STACK_FRAME {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
    pub BackingStoreOffset: u64,
    pub FuncTableEntry: u64,
    pub Params: [u64; 4],
    pub Reserved: [u64; 5],
    pub Virtual: windows_core::BOOL,
    pub Reserved2: u32,
}
impl Default for IMAGEHLP_STACK_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOL {
    pub SizeOfStruct: u32,
    pub Address: u32,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [i8; 1],
}
#[cfg(target_arch = "x86")]
impl Default for IMAGEHLP_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOL64 {
    pub SizeOfStruct: u32,
    pub Address: u64,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [i8; 1],
}
impl Default for IMAGEHLP_SYMBOL64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOL64_PACKAGE {
    pub sym: IMAGEHLP_SYMBOL64,
    pub name: [i8; 2001],
}
impl Default for IMAGEHLP_SYMBOL64_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOLW {
    pub SizeOfStruct: u32,
    pub Address: u32,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [u16; 1],
}
#[cfg(target_arch = "x86")]
impl Default for IMAGEHLP_SYMBOLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOLW64 {
    pub SizeOfStruct: u32,
    pub Address: u64,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [u16; 1],
}
impl Default for IMAGEHLP_SYMBOLW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOLW64_PACKAGE {
    pub sym: IMAGEHLP_SYMBOLW64,
    pub name: [u16; 2001],
}
impl Default for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOLW_PACKAGE {
    pub sym: IMAGEHLP_SYMBOLW,
    pub name: [u16; 2001],
}
#[cfg(target_arch = "x86")]
impl Default for IMAGEHLP_SYMBOLW_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGEHLP_SYMBOL_FUNCTION: u32 = 2048;
pub const IMAGEHLP_SYMBOL_INFO_CONSTANT: u32 = 256;
pub const IMAGEHLP_SYMBOL_INFO_FRAMERELATIVE: u32 = 32;
pub const IMAGEHLP_SYMBOL_INFO_LOCAL: u32 = 128;
pub const IMAGEHLP_SYMBOL_INFO_PARAMETER: u32 = 64;
pub const IMAGEHLP_SYMBOL_INFO_REGISTER: u32 = 8;
pub const IMAGEHLP_SYMBOL_INFO_REGRELATIVE: u32 = 16;
pub const IMAGEHLP_SYMBOL_INFO_TLSRELATIVE: u32 = 16384;
pub const IMAGEHLP_SYMBOL_INFO_VALUEPRESENT: u32 = 1;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOL_PACKAGE {
    pub sym: IMAGEHLP_SYMBOL,
    pub name: [i8; 2001],
}
#[cfg(target_arch = "x86")]
impl Default for IMAGEHLP_SYMBOL_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGEHLP_SYMBOL_SRC {
    pub sizeofstruct: u32,
    pub r#type: u32,
    pub file: [i8; 260],
}
impl Default for IMAGEHLP_SYMBOL_SRC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGEHLP_SYMBOL_THUNK: u32 = 8192;
pub type IMAGEHLP_SYMBOL_TYPE_INFO = i32;
pub const IMAGEHLP_SYMBOL_TYPE_INFO_MAX: IMAGEHLP_SYMBOL_TYPE_INFO = 39;
pub const IMAGEHLP_SYMBOL_VIRTUAL: u32 = 4096;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_DEBUG_INFORMATION {
    pub List: super::winnt::LIST_ENTRY,
    pub ReservedSize: u32,
    pub ReservedMappedBase: *mut core::ffi::c_void,
    pub ReservedMachine: u16,
    pub ReservedCharacteristics: u16,
    pub ReservedCheckSum: u32,
    pub ImageBase: u32,
    pub SizeOfImage: u32,
    pub ReservedNumberOfSections: u32,
    pub ReservedSections: super::winnt::PIMAGE_SECTION_HEADER,
    pub ReservedExportedNamesSize: u32,
    pub ReservedExportedNames: windows_core::PSTR,
    pub ReservedNumberOfFunctionTableEntries: u32,
    pub ReservedFunctionTableEntries: super::winnt::PIMAGE_FUNCTION_ENTRY,
    pub ReservedLowestFunctionStartingAddress: u32,
    pub ReservedHighestFunctionEndingAddress: u32,
    pub ReservedNumberOfFpoTableEntries: u32,
    pub ReservedFpoTableEntries: super::winnt::PFPO_DATA,
    pub SizeOfCoffSymbols: u32,
    pub CoffSymbols: super::winnt::PIMAGE_COFF_SYMBOLS_HEADER,
    pub ReservedSizeOfCodeViewSymbols: u32,
    pub ReservedCodeViewSymbols: *mut core::ffi::c_void,
    pub ImageFilePath: windows_core::PSTR,
    pub ImageFileName: windows_core::PSTR,
    pub ReservedDebugFilePath: windows_core::PSTR,
    pub ReservedTimeDateStamp: u32,
    pub ReservedRomImage: windows_core::BOOL,
    pub ReservedDebugDirectory: super::winnt::PIMAGE_DEBUG_DIRECTORY,
    pub ReservedNumberOfDebugDirectories: u32,
    pub ReservedOriginalFunctionTableBaseAddress: u32,
    pub Reserved: [u32; 2],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for IMAGE_DEBUG_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_SEPARATION: u32 = 65536;
pub const INLINE_FRAME_CONTEXT_IGNORE: u32 = 4294967295;
pub const INLINE_FRAME_CONTEXT_INIT: u32 = 0;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for KDHELP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for KDHELP64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LOADED_IMAGE {
    pub ModuleName: windows_core::PSTR,
    pub hFile: super::winnt::HANDLE,
    pub MappedAddress: super::minwindef::PUCHAR,
    pub FileHeader: super::winnt::PIMAGE_NT_HEADERS32,
    pub LastRvaSection: super::winnt::PIMAGE_SECTION_HEADER,
    pub NumberOfSections: u32,
    pub Sections: super::winnt::PIMAGE_SECTION_HEADER,
    pub Characteristics: u32,
    pub fSystemImage: bool,
    pub fDOSImage: bool,
    pub fReadOnly: bool,
    pub Version: u8,
    pub Links: super::winnt::LIST_ENTRY,
    pub SizeOfImage: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LOADED_IMAGE {
    pub ModuleName: windows_core::PSTR,
    pub hFile: super::winnt::HANDLE,
    pub MappedAddress: super::minwindef::PUCHAR,
    pub FileHeader: super::winnt::PIMAGE_NT_HEADERS64,
    pub LastRvaSection: super::winnt::PIMAGE_SECTION_HEADER,
    pub NumberOfSections: u32,
    pub Sections: super::winnt::PIMAGE_SECTION_HEADER,
    pub Characteristics: u32,
    pub fSystemImage: bool,
    pub fDOSImage: bool,
    pub fReadOnly: bool,
    pub Version: u8,
    pub Links: super::winnt::LIST_ENTRY,
    pub SizeOfImage: u32,
}
#[cfg(target_arch = "x86")]
pub type LPADDRESS = *mut ADDRESS;
pub type LPADDRESS64 = *mut ADDRESS64;
pub type LPAPI_VERSION = *mut API_VERSION;
pub type LPCALL_BACK_USER_INTERRUPT_ROUTINE = Option<unsafe extern "system" fn() -> u32>;
#[cfg(target_arch = "x86")]
pub type LPSTACKFRAME = *mut STACKFRAME;
pub type LPSTACKFRAME64 = *mut STACKFRAME64;
pub type LPSTACKFRAME_EX = *mut STACKFRAME_EX;
pub const MAX_SYM_NAME: u32 = 2000;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MODLOAD_CVMISC {
    pub oCV: u32,
    pub cCV: usize,
    pub oMisc: u32,
    pub cMisc: usize,
    pub dtImage: u32,
    pub cImage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODLOAD_DATA {
    pub ssize: u32,
    pub ssig: u32,
    pub data: *mut core::ffi::c_void,
    pub size: u32,
    pub flags: u32,
}
impl Default for MODLOAD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MODLOAD_PDBGUID_PDBAGE {
    pub PdbGuid: windows_core::GUID,
    pub PdbAge: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODULE_TYPE_INFO {
    pub dataLength: u16,
    pub leaf: u16,
    pub data: [u8; 1],
}
impl Default for MODULE_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NONGAMESPARTITIONS: u32 = 1;
pub const NUM_SSRVOPTS: u32 = 32;
pub const NumSymTypes: SYM_TYPE = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OMAP {
    pub rva: u32,
    pub rvaTo: u32,
}
pub type PDBGHELP_CREATE_USER_DUMP_CALLBACK = Option<unsafe extern "system" fn(datatype: u32, data: *const *const core::ffi::c_void, datalength: *mut u32, userdata: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PDBGHELP_DATA_REPORT_STRUCT = *mut DBGHELP_DATA_REPORT_STRUCT;
pub type PENUMDIRTREE_CALLBACK = Option<unsafe extern "system" fn(filepath: windows_core::PCSTR, callerdata: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PENUMDIRTREE_CALLBACKW = Option<unsafe extern "system" fn(filepath: windows_core::PCWSTR, callerdata: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PENUMLOADED_MODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, modulebase: u32, modulesize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PENUMLOADED_MODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, modulebase: u64, modulesize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PENUMLOADED_MODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_core::PCWSTR, modulebase: u64, modulesize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PENUMSOURCEFILETOKENSCALLBACK = Option<unsafe extern "system" fn(token: *const core::ffi::c_void, size: usize) -> windows_core::BOOL>;
pub type PFINDFILEINPATHCALLBACK = Option<unsafe extern "system" fn(filename: windows_core::PCSTR, context: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFINDFILEINPATHCALLBACKW = Option<unsafe extern "system" fn(filename: windows_core::PCWSTR, context: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::winnt::HANDLE, filename: windows_core::PCSTR, callerdata: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::winnt::HANDLE, filename: windows_core::PCWSTR, callerdata: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::winnt::HANDLE, filename: windows_core::PCSTR, callerdata: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::winnt::HANDLE, filename: windows_core::PCWSTR, callerdata: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, addrbase: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = Option<unsafe extern "system" fn(ahprocess: super::winnt::HANDLE, addrbase: u64) -> *mut core::ffi::c_void>;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PGET_MODULE_BASE_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, address: u32) -> u32>;
#[cfg(feature = "winnt")]
pub type PGET_MODULE_BASE_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, address: u64) -> u64>;
#[cfg(feature = "winnt")]
pub type PGET_TARGET_ATTRIBUTE_VALUE64 = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, attribute: u32, attributedata: u64, attributevalue: *mut u64) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_CBA_EVENT = *mut IMAGEHLP_CBA_EVENT;
pub type PIMAGEHLP_CBA_EVENTW = *mut IMAGEHLP_CBA_EVENTW;
pub type PIMAGEHLP_CBA_READ_MEMORY = *mut IMAGEHLP_CBA_READ_MEMORY;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGEHLP_CONTEXT(pub *mut core::ffi::c_void);
impl Default for PIMAGEHLP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_DEFERRED_SYMBOL_LOAD = *mut IMAGEHLP_DEFERRED_SYMBOL_LOAD;
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_DEFERRED_SYMBOL_LOAD64 = *mut IMAGEHLP_DEFERRED_SYMBOL_LOAD64;
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_DEFERRED_SYMBOL_LOADW64 = *mut IMAGEHLP_DEFERRED_SYMBOL_LOADW64;
#[cfg(target_arch = "x86")]
pub type PIMAGEHLP_DUPLICATE_SYMBOL = *mut IMAGEHLP_DUPLICATE_SYMBOL;
pub type PIMAGEHLP_DUPLICATE_SYMBOL64 = *mut IMAGEHLP_DUPLICATE_SYMBOL64;
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
pub type PIMAGEHLP_GET_TYPE_INFO_PARAMS = *mut IMAGEHLP_GET_TYPE_INFO_PARAMS;
pub type PIMAGEHLP_JIT_SYMBOLMAP = *mut IMAGEHLP_JIT_SYMBOLMAP;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_LINE = *mut IMAGEHLP_LINE;
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_LINE64 = *mut IMAGEHLP_LINE64;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_LINEW = *mut IMAGEHLP_LINEW;
pub type PIMAGEHLP_LINEW64 = *mut IMAGEHLP_LINEW64;
#[cfg(target_arch = "x86")]
pub type PIMAGEHLP_MODULE = *mut IMAGEHLP_MODULE;
pub type PIMAGEHLP_MODULE64 = *mut IMAGEHLP_MODULE64;
pub type PIMAGEHLP_MODULE64_EX = *mut IMAGEHLP_MODULE64_EX;
#[cfg(target_arch = "x86")]
pub type PIMAGEHLP_MODULEW = *mut IMAGEHLP_MODULEW;
pub type PIMAGEHLP_MODULEW64 = *mut IMAGEHLP_MODULEW64;
pub type PIMAGEHLP_MODULEW64_EX = *mut IMAGEHLP_MODULEW64_EX;
pub type PIMAGEHLP_STACK_FRAME = *mut IMAGEHLP_STACK_FRAME;
#[cfg(target_arch = "x86")]
pub type PIMAGEHLP_SYMBOL = *mut IMAGEHLP_SYMBOL;
pub type PIMAGEHLP_SYMBOL64 = *mut IMAGEHLP_SYMBOL64;
pub type PIMAGEHLP_SYMBOL64_PACKAGE = *mut IMAGEHLP_SYMBOL64_PACKAGE;
#[cfg(target_arch = "x86")]
pub type PIMAGEHLP_SYMBOLW = *mut IMAGEHLP_SYMBOLW;
pub type PIMAGEHLP_SYMBOLW64 = *mut IMAGEHLP_SYMBOLW64;
pub type PIMAGEHLP_SYMBOLW64_PACKAGE = *mut IMAGEHLP_SYMBOLW64_PACKAGE;
#[cfg(target_arch = "x86")]
pub type PIMAGEHLP_SYMBOLW_PACKAGE = *mut IMAGEHLP_SYMBOLW_PACKAGE;
#[cfg(target_arch = "x86")]
pub type PIMAGEHLP_SYMBOL_PACKAGE = *mut IMAGEHLP_SYMBOL_PACKAGE;
pub type PIMAGEHLP_SYMBOL_SRC = *mut IMAGEHLP_SYMBOL_SRC;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PIMAGE_DEBUG_INFORMATION = *mut IMAGE_DEBUG_INFORMATION;
#[cfg(target_arch = "x86")]
pub type PKDHELP = *mut KDHELP;
pub type PKDHELP64 = *mut KDHELP64;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PLOADED_IMAGE = *mut LOADED_IMAGE;
pub type PMODLOAD_CVMISC = *mut MODLOAD_CVMISC;
pub type PMODLOAD_DATA = *mut MODLOAD_DATA;
pub type PMODLOAD_PDBGUID_PDBAGE = *mut MODLOAD_PDBGUID_PDBAGE;
pub type PMODULE_TYPE_INFO = *mut MODULE_TYPE_INFO;
pub type POMAP = *mut OMAP;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PREAD_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, lpbaseaddress: u32, lpbuffer: *mut core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, qwbaseaddress: u64, lpbuffer: *mut core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSOURCEFILE = *mut SOURCEFILE;
pub type PSOURCEFILEW = *mut SOURCEFILEW;
pub type PSRCCODEINFO = *mut SRCCODEINFO;
pub type PSRCCODEINFOW = *mut SRCCODEINFOW;
pub type PSYMBOLSERVERBYINDEXPROC = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERCALLBACKPROC = Option<unsafe extern "system" fn(action: usize, data: u64, context: u64) -> windows_core::BOOL>;
pub type PSYMBOLSERVERCLOSEPROC = Option<unsafe extern "system" fn() -> windows_core::BOOL>;
pub type PSYMBOLSERVERDELTANAME = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: *mut core::ffi::c_void, param2: u32, param3: u32, param4: *mut core::ffi::c_void, param5: u32, param6: u32, param7: windows_core::PCSTR, param8: usize) -> windows_core::BOOL>;
pub type PSYMBOLSERVERDELTANAMEW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: *mut core::ffi::c_void, param2: u32, param3: u32, param4: *mut core::ffi::c_void, param5: u32, param6: u32, param7: windows_core::PCWSTR, param8: usize) -> windows_core::BOOL>;
pub type PSYMBOLSERVERGETINDEXSTRING = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32, param2: u32, param3: windows_core::PCSTR, param4: usize) -> windows_core::BOOL>;
pub type PSYMBOLSERVERGETINDEXSTRINGW = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32, param2: u32, param3: windows_core::PCWSTR, param4: usize) -> windows_core::BOOL>;
pub type PSYMBOLSERVERGETOPTIONDATAPROC = Option<unsafe extern "system" fn(param0: usize, param1: *mut u64) -> windows_core::BOOL>;
pub type PSYMBOLSERVERGETOPTIONSPROC = Option<unsafe extern "system" fn() -> usize>;
pub type PSYMBOLSERVERGETSUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PCSTR, param4: usize) -> windows_core::BOOL>;
pub type PSYMBOLSERVERGETSUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: windows_core::PCWSTR, param4: usize) -> windows_core::BOOL>;
pub type PSYMBOLSERVERGETVERSION = Option<unsafe extern "system" fn(param0: *mut API_VERSION) -> windows_core::BOOL>;
pub type PSYMBOLSERVERISSTORE = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERISSTOREW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERMESSAGEPROC = Option<unsafe extern "system" fn(action: usize, data: u64, context: u64) -> windows_core::BOOL>;
pub type PSYMBOLSERVEROPENPROC = Option<unsafe extern "system" fn() -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROC = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROCWEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPROC = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSETHTTPAUTHHEADER = Option<unsafe extern "system" fn(pszauthheader: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSETOPTIONSPROC = Option<unsafe extern "system" fn(param0: usize, param1: u64) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSETOPTIONSWPROC = Option<unsafe extern "system" fn(param0: usize, param1: u64) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSTOREFILE = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_core::PCSTR, param6: usize, param7: u32) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSTOREFILEW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_core::PCWSTR, param6: usize, param7: u32) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSTORESUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PCSTR, param4: usize, param5: u32) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSTORESUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: windows_core::PCWSTR, param4: usize, param5: u32) -> windows_core::BOOL>;
pub type PSYMBOLSERVERVERSION = Option<unsafe extern "system" fn() -> u32>;
pub type PSYMBOLSERVERWEXPROC = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_core::PCWSTR, param6: *mut SYMSRV_EXTENDED_OUTPUT_DATA) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYMBOL_FUNCENTRY_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, addrbase: u32, usercontext: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type PSYMBOL_FUNCENTRY_CALLBACK64 = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, addrbase: u64, usercontext: u64) -> *mut core::ffi::c_void>;
pub type PSYMBOL_INFO = *mut SYMBOL_INFO;
pub type PSYMBOL_INFOW = *mut SYMBOL_INFOW;
pub type PSYMBOL_INFO_PACKAGE = *mut SYMBOL_INFO_PACKAGE;
pub type PSYMBOL_INFO_PACKAGEW = *mut SYMBOL_INFO_PACKAGEW;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PSYMBOL_REGISTERED_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, actioncode: u32, callbackdata: *const core::ffi::c_void, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYMBOL_REGISTERED_CALLBACK64 = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, actioncode: u32, callbackdata: u64, usercontext: u64) -> windows_core::BOOL>;
pub type PSYMSRV_EXTENDED_OUTPUT_DATA = *mut SYMSRV_EXTENDED_OUTPUT_DATA;
pub type PSYMSRV_INDEX_INFO = *mut SYMSRV_INDEX_INFO;
pub type PSYMSRV_INDEX_INFOW = *mut SYMSRV_INDEX_INFOW;
pub type PSYM_ENUMERATESYMBOLS_CALLBACK = Option<unsafe extern "system" fn(psyminfo: *const SYMBOL_INFO, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMERATESYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(psyminfo: *const SYMBOL_INFOW, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMLINES_CALLBACK = Option<unsafe extern "system" fn(lineinfo: *const SRCCODEINFO, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMLINES_CALLBACKW = Option<unsafe extern "system" fn(lineinfo: *const SRCCODEINFOW, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PSYM_ENUMMODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, baseofdll: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMMODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, baseofdll: u64, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMMODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_core::PCWSTR, baseofdll: u64, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYM_ENUMPROCESSES_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYM_ENUMSOURCEFILES_CALLBACK = Option<unsafe extern "system" fn(psourcefile: *const SOURCEFILE, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMSOURCEFILES_CALLBACKW = Option<unsafe extern "system" fn(psourcefile: *const SOURCEFILEW, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PSYM_ENUMSYMBOLS_CALLBACK = Option<unsafe extern "system" fn(symbolname: windows_core::PCSTR, symboladdress: u32, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMSYMBOLS_CALLBACK64 = Option<unsafe extern "system" fn(symbolname: windows_core::PCSTR, symboladdress: u64, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PSYM_ENUMSYMBOLS_CALLBACK64W = Option<unsafe extern "system" fn(symbolname: windows_core::PCWSTR, symboladdress: u64, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PSYM_ENUMSYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(symbolname: windows_core::PCWSTR, symboladdress: u32, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PTRANSLATE_ADDRESS_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, hthread: super::winnt::HANDLE, lpaddr: *mut ADDRESS) -> u32>;
#[cfg(feature = "winnt")]
pub type PTRANSLATE_ADDRESS_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::winnt::HANDLE, hthread: super::winnt::HANDLE, lpaddr: *const ADDRESS64) -> u64>;
pub const SLMFLAG_ALT_INDEX: u32 = 2;
pub const SLMFLAG_NO_SYMBOLS: u32 = 4;
pub const SLMFLAG_VIRTUAL: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOURCEFILE {
    pub ModBase: u64,
    pub FileName: super::winnt::PCHAR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOURCEFILEW {
    pub ModBase: u64,
    pub FileName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SRCCODEINFO {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub ModBase: u64,
    pub Obj: [i8; 261],
    pub FileName: [i8; 261],
    pub LineNumber: u32,
    pub Address: u64,
}
impl Default for SRCCODEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SRCCODEINFOW {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub ModBase: u64,
    pub Obj: [u16; 261],
    pub FileName: [u16; 261],
    pub LineNumber: u32,
    pub Address: u64,
}
impl Default for SRCCODEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSRVACTION_CHECKSUMSTATUS: u32 = 8;
pub const SSRVACTION_EVENT: u32 = 3;
pub const SSRVACTION_EVENTW: u32 = 4;
pub const SSRVACTION_HTTPSTATUS: u32 = 6;
pub const SSRVACTION_QUERYCANCEL: u32 = 2;
pub const SSRVACTION_QUERY_IF_DOWNLOAD_DISALLOWED: u32 = 9;
pub const SSRVACTION_SIZE: u32 = 5;
pub const SSRVACTION_TRACE: u32 = 1;
pub const SSRVACTION_XMLOUTPUT: u32 = 7;
pub const SSRVOPT_CALLBACK: u32 = 1;
pub const SSRVOPT_CALLBACKW: u32 = 65536;
pub const SSRVOPT_DISABLE_PING_HOST: u32 = 67108864;
pub const SSRVOPT_DISABLE_TIMEOUT: u32 = 134217728;
pub const SSRVOPT_DONT_UNCOMPRESS: u32 = 33554432;
pub const SSRVOPT_DOWNSTREAM_STORE: u32 = 8192;
pub const SSRVOPT_DWORD: u32 = 2;
pub const SSRVOPT_DWORDPTR: u32 = 4;
pub const SSRVOPT_ENABLE_COMM_MSG: u32 = 268435456;
pub const SSRVOPT_FAVOR_COMPRESSED: u32 = 2097152;
pub const SSRVOPT_FLAT_DEFAULT_STORE: u32 = 131072;
pub const SSRVOPT_GETPATH: u32 = 64;
pub const SSRVOPT_GUIDPTR: u32 = 8;
pub const SSRVOPT_MAX: u32 = 2147483648;
pub const SSRVOPT_MESSAGE: u32 = 524288;
pub const SSRVOPT_NOCOPY: u32 = 64;
pub const SSRVOPT_OLDGUIDPTR: u32 = 16;
pub const SSRVOPT_OVERWRITE: u32 = 16384;
pub const SSRVOPT_PARAMTYPE: u32 = 256;
pub const SSRVOPT_PARENTWIN: u32 = 128;
pub const SSRVOPT_PROXY: u32 = 4096;
pub const SSRVOPT_PROXYW: u32 = 262144;
pub const SSRVOPT_RESET: usize = 18446744073709551615u64 as usize;
pub const SSRVOPT_RESETTOU: u32 = 32768;
pub const SSRVOPT_RETRY_APP_HANG: u32 = 2147483648;
pub const SSRVOPT_SECURE: u32 = 512;
pub const SSRVOPT_SERVICE: u32 = 1048576;
pub const SSRVOPT_SETCONTEXT: u32 = 2048;
pub const SSRVOPT_STRING: u32 = 4194304;
pub const SSRVOPT_TRACE: u32 = 1024;
pub const SSRVOPT_UNATTENDED: u32 = 32;
pub const SSRVOPT_URI_FILTER: u32 = 536870912;
pub const SSRVOPT_URI_TIERS: u32 = 1073741824;
pub const SSRVOPT_WINHTTP: u32 = 8388608;
pub const SSRVOPT_WININET: u32 = 16777216;
pub const SSRVURI_ALL: u32 = 255;
pub const SSRVURI_COMPRESSED: u32 = 2;
pub const SSRVURI_FILEPTR: u32 = 4;
pub const SSRVURI_HTTP_COMPRESSED: u32 = 2;
pub const SSRVURI_HTTP_FILEPTR: u32 = 4;
pub const SSRVURI_HTTP_MASK: u32 = 15;
pub const SSRVURI_HTTP_MSFZ0: u32 = 8;
pub const SSRVURI_HTTP_NORMAL: u32 = 1;
pub const SSRVURI_NORMAL: u32 = 1;
pub const SSRVURI_UNC_COMPRESSED: u32 = 32;
pub const SSRVURI_UNC_FILEPTR: u32 = 64;
pub const SSRVURI_UNC_MASK: u32 = 240;
pub const SSRVURI_UNC_MSFZ0: u32 = 128;
pub const SSRVURI_UNC_NORMAL: u32 = 16;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STACKFRAME {
    pub AddrPC: ADDRESS,
    pub AddrReturn: ADDRESS,
    pub AddrFrame: ADDRESS,
    pub AddrStack: ADDRESS,
    pub FuncTableEntry: *mut core::ffi::c_void,
    pub Params: [u32; 4],
    pub Far: windows_core::BOOL,
    pub Virtual: windows_core::BOOL,
    pub Reserved: [u32; 3],
    pub KdHelp: KDHELP,
    pub AddrBStore: ADDRESS,
}
#[cfg(target_arch = "x86")]
impl Default for STACKFRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STACKFRAME64 {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: *mut core::ffi::c_void,
    pub Params: [u64; 4],
    pub Far: windows_core::BOOL,
    pub Virtual: windows_core::BOOL,
    pub Reserved: [u64; 3],
    pub KdHelp: KDHELP64,
}
impl Default for STACKFRAME64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STACKFRAME_EX {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: *mut core::ffi::c_void,
    pub Params: [u64; 4],
    pub Far: windows_core::BOOL,
    pub Virtual: windows_core::BOOL,
    pub Reserved: [u64; 3],
    pub KdHelp: KDHELP64,
    pub StackFrameSize: u32,
    pub InlineFrameContext: u32,
}
impl Default for STACKFRAME_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type SYMADDSOURCESTREAM = Option<unsafe extern "system" fn(param0: super::winnt::HANDLE, param1: u64, param2: windows_core::PCSTR, param3: *mut u8, param4: usize) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type SYMADDSOURCESTREAMA = Option<unsafe extern "system" fn(param0: super::winnt::HANDLE, param1: u64, param2: windows_core::PCSTR, param3: *mut u8, param4: usize) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYMBOL_INFO {
    pub SizeOfStruct: u32,
    pub TypeIndex: u32,
    pub Reserved: [u64; 2],
    pub Index: u32,
    pub Size: u32,
    pub ModBase: u64,
    pub Flags: u32,
    pub Value: u64,
    pub Address: u64,
    pub Register: u32,
    pub Scope: u32,
    pub Tag: u32,
    pub NameLen: u32,
    pub MaxNameLen: u32,
    pub Name: [i8; 1],
}
impl Default for SYMBOL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYMBOL_INFOW {
    pub SizeOfStruct: u32,
    pub TypeIndex: u32,
    pub Reserved: [u64; 2],
    pub Index: u32,
    pub Size: u32,
    pub ModBase: u64,
    pub Flags: u32,
    pub Value: u64,
    pub Address: u64,
    pub Register: u32,
    pub Scope: u32,
    pub Tag: u32,
    pub NameLen: u32,
    pub MaxNameLen: u32,
    pub Name: [u16; 1],
}
impl Default for SYMBOL_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYMBOL_INFO_PACKAGE {
    pub si: SYMBOL_INFO,
    pub name: [i8; 2001],
}
impl Default for SYMBOL_INFO_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYMBOL_INFO_PACKAGEW {
    pub si: SYMBOL_INFOW,
    pub name: [u16; 2001],
}
impl Default for SYMBOL_INFO_PACKAGEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYMENUM_OPTIONS_DEFAULT: u32 = 1;
pub const SYMENUM_OPTIONS_INLINE: u32 = 2;
pub const SYMFLAG_CLR_TOKEN: u32 = 262144;
pub const SYMFLAG_COMPLEX: u32 = 67108864;
pub const SYMFLAG_CONSTANT: u32 = 256;
pub const SYMFLAG_EXPORT: u32 = 512;
pub const SYMFLAG_FIXUP_ARM64X: u32 = 16777216;
pub const SYMFLAG_FORWARDER: u32 = 1024;
pub const SYMFLAG_FRAMEREL: u32 = 32;
pub const SYMFLAG_FUNCTION: u32 = 2048;
pub const SYMFLAG_FUNC_NO_RETURN: u32 = 1048576;
pub const SYMFLAG_GLOBAL: u32 = 33554432;
pub const SYMFLAG_ILREL: u32 = 65536;
pub const SYMFLAG_LOCAL: u32 = 128;
pub const SYMFLAG_METADATA: u32 = 131072;
pub const SYMFLAG_NULL: u32 = 524288;
pub const SYMFLAG_PARAMETER: u32 = 64;
pub const SYMFLAG_PUBLIC_CODE: u32 = 4194304;
pub const SYMFLAG_REGISTER: u32 = 8;
pub const SYMFLAG_REGREL: u32 = 16;
pub const SYMFLAG_REGREL_ALIASINDIR: u32 = 8388608;
pub const SYMFLAG_RESET: u32 = 2147483648;
pub const SYMFLAG_SLOT: u32 = 32768;
pub const SYMFLAG_SYNTHETIC_ZEROBASE: u32 = 2097152;
pub const SYMFLAG_THUNK: u32 = 8192;
pub const SYMFLAG_TLSREL: u32 = 16384;
pub const SYMFLAG_VALUEPRESENT: u32 = 1;
pub const SYMFLAG_VIRTUAL: u32 = 4096;
pub const SYMF_CONSTANT: u32 = 256;
pub const SYMF_EXPORT: u32 = 512;
pub const SYMF_FORWARDER: u32 = 1024;
pub const SYMF_FRAMEREL: u32 = 32;
pub const SYMF_FUNCTION: u32 = 2048;
pub const SYMF_LOCAL: u32 = 128;
pub const SYMF_OMAP_GENERATED: u32 = 1;
pub const SYMF_OMAP_MODIFIED: u32 = 2;
pub const SYMF_PARAMETER: u32 = 64;
pub const SYMF_REGISTER: u32 = 8;
pub const SYMF_REGREL: u32 = 16;
pub const SYMF_THUNK: u32 = 8192;
pub const SYMF_TLSREL: u32 = 16384;
pub const SYMF_VIRTUAL: u32 = 4096;
pub const SYMOPT_ALLOW_ABSOLUTE_SYMBOLS: u32 = 2048;
pub const SYMOPT_ALLOW_ZERO_ADDRESS: u32 = 16777216;
pub const SYMOPT_AUTO_PUBLICS: u32 = 65536;
pub const SYMOPT_CASE_INSENSITIVE: u32 = 1;
pub const SYMOPT_DEBUG: u32 = 2147483648;
pub const SYMOPT_DEFERRED_LOADS: u32 = 4;
pub const SYMOPT_DISABLE_FAST_SYMBOLS: u32 = 268435456;
pub const SYMOPT_DISABLE_SRVSTAR_ON_STARTUP: u32 = 1073741824;
pub const SYMOPT_DISABLE_SYMSRV_AUTODETECT: u32 = 33554432;
pub const SYMOPT_DISABLE_SYMSRV_TIMEOUT: u32 = 536870912;
pub const SYMOPT_EXACT_SYMBOLS: u32 = 1024;
pub const SYMOPT_EX_DISABLEACCESSTIMEUPDATE: IMAGEHLP_EXTENDED_OPTIONS = 0;
pub const SYMOPT_EX_DISALLOW_NETWORK_PATHS: IMAGEHLP_EXTENDED_OPTIONS = 4;
pub const SYMOPT_EX_LASTVALIDDEBUGDIRECTORY: IMAGEHLP_EXTENDED_OPTIONS = 1;
pub const SYMOPT_EX_MAX: IMAGEHLP_EXTENDED_OPTIONS = 5;
pub const SYMOPT_EX_NEVERLOADSYMBOLS: IMAGEHLP_EXTENDED_OPTIONS = 3;
pub const SYMOPT_EX_NOIMPLICITPATTERNSEARCH: IMAGEHLP_EXTENDED_OPTIONS = 2;
pub const SYMOPT_FAIL_CRITICAL_ERRORS: u32 = 512;
pub const SYMOPT_FAVOR_COMPRESSED: u32 = 8388608;
pub const SYMOPT_FLAT_DIRECTORY: u32 = 4194304;
pub const SYMOPT_IGNORE_CVREC: u32 = 128;
pub const SYMOPT_IGNORE_IMAGEDIR: u32 = 2097152;
pub const SYMOPT_IGNORE_NT_SYMPATH: u32 = 4096;
pub const SYMOPT_INCLUDE_32BIT_MODULES: u32 = 8192;
pub const SYMOPT_LOAD_ANYTHING: u32 = 64;
pub const SYMOPT_LOAD_LINES: u32 = 16;
pub const SYMOPT_NO_CPP: u32 = 8;
pub const SYMOPT_NO_IMAGE_SEARCH: u32 = 131072;
pub const SYMOPT_NO_PROMPTS: u32 = 524288;
pub const SYMOPT_NO_PUBLICS: u32 = 32768;
pub const SYMOPT_NO_UNQUALIFIED_LOADS: u32 = 256;
pub const SYMOPT_OMAP_FIND_NEAREST: u32 = 32;
pub const SYMOPT_OVERWRITE: u32 = 1048576;
pub const SYMOPT_PUBLICS_ONLY: u32 = 16384;
pub const SYMOPT_READONLY_CACHE: u32 = 67108864;
pub const SYMOPT_SECURE: u32 = 262144;
pub const SYMOPT_SYMPATH_LAST: u32 = 134217728;
pub const SYMOPT_UNDNAME: u32 = 2;
pub const SYMSEARCH_ALLITEMS: u32 = 8;
pub const SYMSEARCH_GLOBALSONLY: u32 = 4;
pub const SYMSEARCH_MASKOBJS: u32 = 1;
pub const SYMSEARCH_RECURSE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYMSRV_EXTENDED_OUTPUT_DATA {
    pub sizeOfStruct: u32,
    pub version: u32,
    pub filePtrMsg: [u16; 261],
}
impl Default for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYMSRV_INDEX_INFO {
    pub sizeofstruct: u32,
    pub file: [i8; 261],
    pub stripped: windows_core::BOOL,
    pub timestamp: u32,
    pub size: u32,
    pub dbgfile: [i8; 261],
    pub pdbfile: [i8; 261],
    pub guid: windows_core::GUID,
    pub sig: u32,
    pub age: u32,
}
impl Default for SYMSRV_INDEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYMSRV_INDEX_INFOW {
    pub sizeofstruct: u32,
    pub file: [u16; 261],
    pub stripped: windows_core::BOOL,
    pub timestamp: u32,
    pub size: u32,
    pub dbgfile: [u16; 261],
    pub pdbfile: [u16; 261],
    pub guid: windows_core::GUID,
    pub sig: u32,
    pub age: u32,
}
impl Default for SYMSRV_INDEX_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYMSRV_VERSION: u32 = 2;
pub const SYMSTOREOPT_ALT_INDEX: u32 = 16;
pub const SYMSTOREOPT_COMPRESS: u32 = 1;
pub const SYMSTOREOPT_OVERWRITE: u32 = 2;
pub const SYMSTOREOPT_PASS_IF_EXISTS: u32 = 64;
pub const SYMSTOREOPT_POINTER: u32 = 8;
pub const SYMSTOREOPT_RETURNINDEX: u32 = 4;
pub const SYMSTOREOPT_UNICODE: u32 = 32;
pub const SYM_INLINE_COMP_DIFFERENT: u32 = 5;
pub const SYM_INLINE_COMP_ERROR: u32 = 0;
pub const SYM_INLINE_COMP_IDENTICAL: u32 = 1;
pub const SYM_INLINE_COMP_STEPIN: u32 = 2;
pub const SYM_INLINE_COMP_STEPOUT: u32 = 3;
pub const SYM_INLINE_COMP_STEPOVER: u32 = 4;
pub const SYM_STKWALK_DEFAULT: u32 = 0;
pub const SYM_STKWALK_FORCE_FRAMEPTR: u32 = 1;
pub const SYM_STKWALK_ZEROEXTEND_PTRS: u32 = 2;
pub type SYM_TYPE = i32;
pub const SymCoff: SYM_TYPE = 1;
pub const SymCv: SYM_TYPE = 2;
pub const SymDeferred: SYM_TYPE = 5;
pub const SymDia: SYM_TYPE = 7;
pub const SymExport: SYM_TYPE = 4;
pub const SymNone: SYM_TYPE = 0;
pub const SymPdb: SYM_TYPE = 3;
pub const SymSym: SYM_TYPE = 6;
pub const SymVirtual: SYM_TYPE = 8;
pub const TARGET_ATTRIBUTE_PACMASK: u32 = 1;
pub const TARGET_ATTRIBUTE_PACMASK_LIVETARGET: u64 = 18446744073709551615;
pub const TI_FINDCHILDREN: IMAGEHLP_SYMBOL_TYPE_INFO = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TI_FINDCHILDREN_PARAMS {
    pub Count: u32,
    pub Start: u32,
    pub ChildId: [u32; 1],
}
impl Default for TI_FINDCHILDREN_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TI_GET_ADDRESS: IMAGEHLP_SYMBOL_TYPE_INFO = 22;
pub const TI_GET_ADDRESSOFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = 9;
pub const TI_GET_ARRAYINDEXTYPEID: IMAGEHLP_SYMBOL_TYPE_INFO = 6;
pub const TI_GET_BASETYPE: IMAGEHLP_SYMBOL_TYPE_INFO = 5;
pub const TI_GET_BITPOSITION: IMAGEHLP_SYMBOL_TYPE_INFO = 14;
pub const TI_GET_CALLING_CONVENTION: IMAGEHLP_SYMBOL_TYPE_INFO = 26;
pub const TI_GET_CHILDRENCOUNT: IMAGEHLP_SYMBOL_TYPE_INFO = 13;
pub const TI_GET_CLASSPARENTID: IMAGEHLP_SYMBOL_TYPE_INFO = 18;
pub const TI_GET_COUNT: IMAGEHLP_SYMBOL_TYPE_INFO = 12;
pub const TI_GET_DATAKIND: IMAGEHLP_SYMBOL_TYPE_INFO = 8;
pub const TI_GET_DISCRIMINATEDUNION_TAG_OFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = 36;
pub const TI_GET_DISCRIMINATEDUNION_TAG_RANGES: IMAGEHLP_SYMBOL_TYPE_INFO = 38;
pub const TI_GET_DISCRIMINATEDUNION_TAG_RANGESCOUNT: IMAGEHLP_SYMBOL_TYPE_INFO = 37;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TI_GET_DISCRIMINATEDUNION_TAG_RANGES_PARAMS {
    pub Count: u32,
    pub Start: u32,
    pub Range: [DISCRIMINATEDUNION_TAG_VALUE; 1],
}
impl Default for TI_GET_DISCRIMINATEDUNION_TAG_RANGES_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TI_GET_DISCRIMINATEDUNION_TAG_TYPEID: IMAGEHLP_SYMBOL_TYPE_INFO = 35;
pub const TI_GET_INDIRECTVIRTUALBASECLASS: IMAGEHLP_SYMBOL_TYPE_INFO = 32;
pub const TI_GET_IS_REFERENCE: IMAGEHLP_SYMBOL_TYPE_INFO = 31;
pub const TI_GET_LENGTH: IMAGEHLP_SYMBOL_TYPE_INFO = 2;
pub const TI_GET_LEXICALPARENT: IMAGEHLP_SYMBOL_TYPE_INFO = 21;
pub const TI_GET_NESTED: IMAGEHLP_SYMBOL_TYPE_INFO = 19;
pub const TI_GET_OBJECTPOINTERTYPE: IMAGEHLP_SYMBOL_TYPE_INFO = 34;
pub const TI_GET_OFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = 10;
pub const TI_GET_SYMINDEX: IMAGEHLP_SYMBOL_TYPE_INFO = 20;
pub const TI_GET_SYMNAME: IMAGEHLP_SYMBOL_TYPE_INFO = 1;
pub const TI_GET_SYMTAG: IMAGEHLP_SYMBOL_TYPE_INFO = 0;
pub const TI_GET_THISADJUST: IMAGEHLP_SYMBOL_TYPE_INFO = 23;
pub const TI_GET_TYPE: IMAGEHLP_SYMBOL_TYPE_INFO = 3;
pub const TI_GET_TYPEID: IMAGEHLP_SYMBOL_TYPE_INFO = 4;
pub const TI_GET_UDTKIND: IMAGEHLP_SYMBOL_TYPE_INFO = 24;
pub const TI_GET_VALUE: IMAGEHLP_SYMBOL_TYPE_INFO = 11;
pub const TI_GET_VIRTUALBASECLASS: IMAGEHLP_SYMBOL_TYPE_INFO = 15;
pub const TI_GET_VIRTUALBASEDISPINDEX: IMAGEHLP_SYMBOL_TYPE_INFO = 30;
pub const TI_GET_VIRTUALBASEOFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = 29;
pub const TI_GET_VIRTUALBASEPOINTEROFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = 17;
pub const TI_GET_VIRTUALBASETABLETYPE: IMAGEHLP_SYMBOL_TYPE_INFO = 33;
pub const TI_GET_VIRTUALTABLESHAPEID: IMAGEHLP_SYMBOL_TYPE_INFO = 16;
pub const TI_GTIEX_REQS_VALID: IMAGEHLP_SYMBOL_TYPE_INFO = 28;
pub const TI_IS_CLOSE_EQUIV_TO: IMAGEHLP_SYMBOL_TYPE_INFO = 27;
pub const TI_IS_EQUIV_TO: IMAGEHLP_SYMBOL_TYPE_INFO = 25;
pub const UNDNAME_32_BIT_DECODE: u32 = 2048;
pub const UNDNAME_COMPLETE: u32 = 0;
pub const UNDNAME_NAME_ONLY: u32 = 4096;
pub const UNDNAME_NO_ACCESS_SPECIFIERS: u32 = 128;
pub const UNDNAME_NO_ALLOCATION_LANGUAGE: u32 = 16;
pub const UNDNAME_NO_ALLOCATION_MODEL: u32 = 8;
pub const UNDNAME_NO_ARGUMENTS: u32 = 8192;
pub const UNDNAME_NO_CV_THISTYPE: u32 = 64;
pub const UNDNAME_NO_FUNCTION_RETURNS: u32 = 4;
pub const UNDNAME_NO_LEADING_UNDERSCORES: u32 = 1;
pub const UNDNAME_NO_MEMBER_TYPE: u32 = 512;
pub const UNDNAME_NO_MS_KEYWORDS: u32 = 2;
pub const UNDNAME_NO_MS_THISTYPE: u32 = 32;
pub const UNDNAME_NO_RETURN_UDT_MODEL: u32 = 1024;
pub const UNDNAME_NO_SPECIAL_SYMS: u32 = 16384;
pub const UNDNAME_NO_THISTYPE: u32 = 96;
pub const UNDNAME_NO_THROW_SIGNATURES: u32 = 256;
pub const hdBase: IMAGEHLP_HD_TYPE = 0;
pub const hdMax: IMAGEHLP_HD_TYPE = 3;
pub const hdSrc: IMAGEHLP_HD_TYPE = 2;
pub const hdSym: IMAGEHLP_HD_TYPE = 1;
pub const sevAttn: i32 = 2;
pub const sevFatal: i32 = 3;
pub const sevInfo: i32 = 0;
pub const sevMax: i32 = 4;
pub const sevProblem: i32 = 1;
pub const sfDbg: IMAGEHLP_SF_TYPE = 1;
pub const sfImage: IMAGEHLP_SF_TYPE = 0;
pub const sfMax: IMAGEHLP_SF_TYPE = 4;
pub const sfMpd: IMAGEHLP_SF_TYPE = 3;
pub const sfPdb: IMAGEHLP_SF_TYPE = 2;
