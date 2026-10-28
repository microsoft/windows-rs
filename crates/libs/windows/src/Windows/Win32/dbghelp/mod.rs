#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DbgHelpCreateUserDump<P0>(filename: P0, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: Option<super::PVOID>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDump(filename : windows_core::PCSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : super::PVOID) -> windows_core::BOOL);
    unsafe { DbgHelpCreateUserDump(filename.param().abi(), callback, userdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DbgHelpCreateUserDumpW<P0>(filename: P0, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: Option<super::PVOID>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDumpW(filename : windows_core::PCWSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : super::PVOID) -> windows_core::BOOL);
    unsafe { DbgHelpCreateUserDumpW(filename.param().abi(), callback, userdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumDirTree<P1, P2>(hprocess: Option<super::HANDLE>, rootpath: P1, inputpathname: P2, outputpathbuffer: Option<windows_core::PSTR>, cb: PENUMDIRTREE_CALLBACK, data: Option<super::PVOID>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn EnumDirTree(hprocess : super::HANDLE, rootpath : windows_core::PCSTR, inputpathname : windows_core::PCSTR, outputpathbuffer : windows_core::PSTR, cb : PENUMDIRTREE_CALLBACK, data : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumDirTree(hprocess.unwrap_or(core::mem::zeroed()) as _, rootpath.param().abi(), inputpathname.param().abi(), outputpathbuffer.unwrap_or(core::mem::zeroed()) as _, cb, data.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnumDirTreeW<P1, P2>(hprocess: Option<super::HANDLE>, rootpath: P1, inputpathname: P2, outputpathbuffer: Option<windows_core::PWSTR>, cb: PENUMDIRTREE_CALLBACKW, data: Option<super::PVOID>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn EnumDirTreeW(hprocess : super::HANDLE, rootpath : windows_core::PCWSTR, inputpathname : windows_core::PCWSTR, outputpathbuffer : windows_core::PWSTR, cb : PENUMDIRTREE_CALLBACKW, data : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumDirTreeW(hprocess.unwrap_or(core::mem::zeroed()) as _, rootpath.param().abi(), inputpathname.param().abi(), outputpathbuffer.unwrap_or(core::mem::zeroed()) as _, cb, data.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumerateLoadedModules(hprocess: super::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModules(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModules(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumerateLoadedModules(hprocess: super::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "EnumerateLoadedModules64" fn EnumerateLoadedModules(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModules(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumerateLoadedModules64(hprocess: super::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModules64(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModules64(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumerateLoadedModulesEx(hprocess: super::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesEx(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModulesEx(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumerateLoadedModulesExW(hprocess: super::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesExW(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModulesExW(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumerateLoadedModulesW64(hprocess: super::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesW64(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { EnumerateLoadedModulesW64(hprocess, enumloadedmodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindDebugInfoFile<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: windows_core::PSTR) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindDebugInfoFile(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, debugfilepath : windows_core::PSTR) -> super::HANDLE);
    unsafe { FindDebugInfoFile(filename.param().abi(), symbolpath.param().abi(), debugfilepath) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindDebugInfoFileEx<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: windows_core::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: Option<super::PVOID>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindDebugInfoFileEx(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, debugfilepath : windows_core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { FindDebugInfoFileEx(filename.param().abi(), symbolpath.param().abi(), debugfilepath, callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindDebugInfoFileExW<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: windows_core::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: Option<super::PVOID>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindDebugInfoFileExW(filename : windows_core::PCWSTR, symbolpath : windows_core::PCWSTR, debugfilepath : windows_core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { FindDebugInfoFileExW(filename.param().abi(), symbolpath.param().abi(), debugfilepath, callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindExecutableImage<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: windows_core::PSTR) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindExecutableImage(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, imagefilepath : windows_core::PSTR) -> super::HANDLE);
    unsafe { FindExecutableImage(filename.param().abi(), symbolpath.param().abi(), imagefilepath) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindExecutableImageEx<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: windows_core::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: Option<super::PVOID>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindExecutableImageEx(filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, imagefilepath : windows_core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { FindExecutableImageEx(filename.param().abi(), symbolpath.param().abi(), imagefilepath, callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindExecutableImageExW<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: windows_core::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: super::PVOID) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindExecutableImageExW(filename : windows_core::PCWSTR, symbolpath : windows_core::PCWSTR, imagefilepath : windows_core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { FindExecutableImageExW(filename.param().abi(), symbolpath.param().abi(), imagefilepath, callback, callerdata) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFileInPath<P1, P2>(hprocess: super::HANDLE, searchpath: P1, filename: P2, id: super::PVOID, two: super::DWORD, three: super::DWORD, flags: super::DWORD, filepath: windows_core::PSTR) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindFileInPath(hprocess : super::HANDLE, searchpath : windows_core::PCSTR, filename : windows_core::PCSTR, id : super::PVOID, two : super::DWORD, three : super::DWORD, flags : super::DWORD, filepath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { FindFileInPath(hprocess, searchpath.param().abi(), filename.param().abi(), id, two, three, flags, filepath) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFileInSearchPath<P1, P2>(hprocess: super::HANDLE, searchpath: P1, filename: P2, one: super::DWORD, two: super::DWORD, three: super::DWORD, filepath: windows_core::PSTR) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn FindFileInSearchPath(hprocess : super::HANDLE, searchpath : windows_core::PCSTR, filename : windows_core::PCSTR, one : super::DWORD, two : super::DWORD, three : super::DWORD, filepath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { FindFileInSearchPath(hprocess, searchpath.param().abi(), filename.param().abi(), one, two, three, filepath) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetSymLoadError() -> super::DWORD {
    windows_core::link!("dbghelp.dll" "system" fn GetSymLoadError() -> super::DWORD);
    unsafe { GetSymLoadError() }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetTimestampForLoadedLibrary(module: super::HMODULE) -> super::DWORD {
    windows_core::link!("dbghelp.dll" "system" fn GetTimestampForLoadedLibrary(module : super::HMODULE) -> super::DWORD);
    unsafe { GetTimestampForLoadedLibrary(module) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ImageDirectoryEntryToData(base: super::PVOID, mappedasimage: super::BOOLEAN, directoryentry: super::USHORT, size: super::PULONG) -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToData(base : super::PVOID, mappedasimage : super::BOOLEAN, directoryentry : super::USHORT, size : super::PULONG) -> super::PVOID);
    unsafe { ImageDirectoryEntryToData(base, mappedasimage, directoryentry, size as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ImageDirectoryEntryToDataEx(base: super::PVOID, mappedasimage: super::BOOLEAN, directoryentry: super::USHORT, size: super::PULONG, foundheader: Option<*mut super::PIMAGE_SECTION_HEADER>) -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToDataEx(base : super::PVOID, mappedasimage : super::BOOLEAN, directoryentry : super::USHORT, size : super::PULONG, foundheader : *mut super::PIMAGE_SECTION_HEADER) -> super::PVOID);
    unsafe { ImageDirectoryEntryToDataEx(base, mappedasimage, directoryentry, size as _, foundheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ImageNtHeader(base: super::PVOID) -> super::PIMAGE_NT_HEADERS {
    windows_core::link!("dbghelp.dll" "system" fn ImageNtHeader(base : super::PVOID) -> super::PIMAGE_NT_HEADERS);
    unsafe { ImageNtHeader(base) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ImageRvaToSection(ntheaders: super::PIMAGE_NT_HEADERS, base: super::PVOID, rva: super::ULONG) -> super::PIMAGE_SECTION_HEADER {
    windows_core::link!("dbghelp.dll" "system" fn ImageRvaToSection(ntheaders : super::PIMAGE_NT_HEADERS, base : super::PVOID, rva : super::ULONG) -> super::PIMAGE_SECTION_HEADER);
    unsafe { ImageRvaToSection(ntheaders, base, rva) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ImageRvaToVa(ntheaders: super::PIMAGE_NT_HEADERS, base: super::PVOID, rva: super::ULONG, lastrvasection: Option<*const super::PIMAGE_SECTION_HEADER>) -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" fn ImageRvaToVa(ntheaders : super::PIMAGE_NT_HEADERS, base : super::PVOID, rva : super::ULONG, lastrvasection : *const super::PIMAGE_SECTION_HEADER) -> super::PVOID);
    unsafe { ImageRvaToVa(ntheaders, base, rva, lastrvasection.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ImagehlpApiVersion() -> LPAPI_VERSION {
    windows_core::link!("dbghelp.dll" "system" fn ImagehlpApiVersion() -> LPAPI_VERSION);
    unsafe { ImagehlpApiVersion() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ImagehlpApiVersionEx(appversion: LPAPI_VERSION) -> LPAPI_VERSION {
    windows_core::link!("dbghelp.dll" "system" fn ImagehlpApiVersionEx(appversion : LPAPI_VERSION) -> LPAPI_VERSION);
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
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapDebugInformation<P1, P2>(filehandle: Option<super::HANDLE>, filename: P1, symbolpath: P2, imagebase: super::ULONG) -> PIMAGE_DEBUG_INFORMATION
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn MapDebugInformation(filehandle : super::HANDLE, filename : windows_core::PCSTR, symbolpath : windows_core::PCSTR, imagebase : super::ULONG) -> PIMAGE_DEBUG_INFORMATION);
    unsafe { MapDebugInformation(filehandle.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), symbolpath.param().abi(), imagebase) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RangeMapAddPeImageSections<P1>(rmaphandle: super::PVOID, imagename: P1, mappedimage: super::PVOID, mappingbytes: super::DWORD, imagebase: super::DWORD64, usertag: super::DWORD64, mappingflags: super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn RangeMapAddPeImageSections(rmaphandle : super::PVOID, imagename : windows_core::PCWSTR, mappedimage : super::PVOID, mappingbytes : super::DWORD, imagebase : super::DWORD64, usertag : super::DWORD64, mappingflags : super::DWORD) -> windows_core::BOOL);
    unsafe { RangeMapAddPeImageSections(rmaphandle, imagename.param().abi(), mappedimage, mappingbytes, imagebase, usertag, mappingflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RangeMapCreate() -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapCreate() -> super::PVOID);
    unsafe { RangeMapCreate() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RangeMapFree(rmaphandle: Option<super::PVOID>) {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapFree(rmaphandle : super::PVOID));
    unsafe { RangeMapFree(rmaphandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RangeMapRead(rmaphandle: super::PVOID, offset: super::DWORD64, buffer: super::PVOID, requestbytes: super::DWORD, flags: super::DWORD, donebytes: Option<super::PDWORD>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapRead(rmaphandle : super::PVOID, offset : super::DWORD64, buffer : super::PVOID, requestbytes : super::DWORD, flags : super::DWORD, donebytes : super::PDWORD) -> windows_core::BOOL);
    unsafe { RangeMapRead(rmaphandle, offset, buffer as _, requestbytes, flags, donebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RangeMapRemove(rmaphandle: super::PVOID, usertag: super::DWORD64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapRemove(rmaphandle : super::PVOID, usertag : super::DWORD64) -> windows_core::BOOL);
    unsafe { RangeMapRemove(rmaphandle, usertag) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RangeMapWrite(rmaphandle: super::PVOID, offset: super::DWORD64, buffer: super::PVOID, requestbytes: super::DWORD, flags: super::DWORD, donebytes: Option<super::PDWORD>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn RangeMapWrite(rmaphandle : super::PVOID, offset : super::DWORD64, buffer : super::PVOID, requestbytes : super::DWORD, flags : super::DWORD, donebytes : super::PDWORD) -> windows_core::BOOL);
    unsafe { RangeMapWrite(rmaphandle, offset, buffer, requestbytes, flags, donebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RemoveInvalidModuleList(hprocess: super::HANDLE) {
    windows_core::link!("dbghelp.dll" "system" fn RemoveInvalidModuleList(hprocess : super::HANDLE));
    unsafe { RemoveInvalidModuleList(hprocess) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReportSymbolLoadSummary<P1>(hprocess: super::HANDLE, ploadmodule: P1, psymboldata: PDBGHELP_DATA_REPORT_STRUCT) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn ReportSymbolLoadSummary(hprocess : super::HANDLE, ploadmodule : windows_core::PCWSTR, psymboldata : PDBGHELP_DATA_REPORT_STRUCT) -> windows_core::BOOL);
    unsafe { ReportSymbolLoadSummary(hprocess, ploadmodule.param().abi(), psymboldata) }
}
#[inline]
pub unsafe fn SearchTreeForFile<P0, P1>(rootpath: P0, inputpathname: P1, outputpathbuffer: windows_core::PSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SearchTreeForFile(rootpath : windows_core::PCSTR, inputpathname : windows_core::PCSTR, outputpathbuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SearchTreeForFile(rootpath.param().abi(), inputpathname.param().abi(), outputpathbuffer) }
}
#[inline]
pub unsafe fn SearchTreeForFileW<P0, P1>(rootpath: P0, inputpathname: P1, outputpathbuffer: windows_core::PWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SearchTreeForFileW(rootpath : windows_core::PCWSTR, inputpathname : windows_core::PCWSTR, outputpathbuffer : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SearchTreeForFileW(rootpath.param().abi(), inputpathname.param().abi(), outputpathbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetCheckUserInterruptShared(lpstartaddress: LPCALL_BACK_USER_INTERRUPT_ROUTINE) {
    windows_core::link!("dbghelp.dll" "system" fn SetCheckUserInterruptShared(lpstartaddress : LPCALL_BACK_USER_INTERRUPT_ROUTINE));
    unsafe { SetCheckUserInterruptShared(lpstartaddress) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetSymLoadError(error: super::DWORD) {
    windows_core::link!("dbghelp.dll" "system" fn SetSymLoadError(error : super::DWORD));
    unsafe { SetSymLoadError(error) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StackWalk(machinetype: super::DWORD, hprocess: super::HANDLE, hthread: super::HANDLE, stackframe: LPSTACKFRAME, contextrecord: super::PVOID, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE, translateaddress: PTRANSLATE_ADDRESS_ROUTINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalk(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE, translateaddress : PTRANSLATE_ADDRESS_ROUTINE) -> windows_core::BOOL);
    unsafe { StackWalk(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StackWalk(machinetype: super::DWORD, hprocess: super::HANDLE, hthread: super::HANDLE, stackframe: LPSTACKFRAME64, contextrecord: super::PVOID, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "StackWalk64" fn StackWalk(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME64, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64) -> windows_core::BOOL);
    unsafe { StackWalk(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StackWalk2(machinetype: super::DWORD, hprocess: super::HANDLE, hthread: super::HANDLE, stackframe: LPSTACKFRAME_EX, contextrecord: super::PVOID, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64, gettargetattributevalue: PGET_TARGET_ATTRIBUTE_VALUE64, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalk2(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME_EX, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, gettargetattributevalue : PGET_TARGET_ATTRIBUTE_VALUE64, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { StackWalk2(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress, gettargetattributevalue, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StackWalk64(machinetype: super::DWORD, hprocess: super::HANDLE, hthread: super::HANDLE, stackframe: LPSTACKFRAME64, contextrecord: super::PVOID, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalk64(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME64, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64) -> windows_core::BOOL);
    unsafe { StackWalk64(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StackWalkEx(machinetype: super::DWORD, hprocess: super::HANDLE, hthread: super::HANDLE, stackframe: LPSTACKFRAME_EX, contextrecord: super::PVOID, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn StackWalkEx(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME_EX, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { StackWalkEx(machinetype, hprocess, hthread, stackframe as _, contextrecord as _, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymAddSourceStream<P2>(hprocess: super::HANDLE, base: super::ULONG64, streamfile: P2, buffer: Option<&[u8]>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSourceStream(hprocess : super::HANDLE, base : super::ULONG64, streamfile : windows_core::PCSTR, buffer : super::PBYTE, size : super::size_t) -> windows_core::BOOL);
    unsafe { SymAddSourceStream(hprocess, base, streamfile.param().abi(), core::mem::transmute(buffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymAddSourceStreamA<P2>(hprocess: super::HANDLE, base: super::ULONG64, streamfile: P2, buffer: Option<&[u8]>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSourceStreamA(hprocess : super::HANDLE, base : super::ULONG64, streamfile : windows_core::PCSTR, buffer : super::PBYTE, size : super::size_t) -> windows_core::BOOL);
    unsafe { SymAddSourceStreamA(hprocess, base, streamfile.param().abi(), core::mem::transmute(buffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymAddSourceStreamW<P2>(hprocess: super::HANDLE, base: super::ULONG64, filespec: P2, buffer: Option<&[u8]>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSourceStreamW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_core::PCWSTR, buffer : super::PBYTE, size : super::size_t) -> windows_core::BOOL);
    unsafe { SymAddSourceStreamW(hprocess, base, filespec.param().abi(), core::mem::transmute(buffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymAddSymbol<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, name: P2, address: super::DWORD64, size: super::DWORD, flags: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSymbol(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_core::PCSTR, address : super::DWORD64, size : super::DWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymAddSymbol(hprocess, baseofdll, name.param().abi(), address, size, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymAddSymbolW<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, name: P2, address: super::DWORD64, size: super::DWORD, flags: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymAddSymbolW(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_core::PCWSTR, address : super::DWORD64, size : super::DWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymAddSymbolW(hprocess, baseofdll, name.param().abi(), address, size, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymAddrIncludeInlineTrace(hprocess: super::HANDLE, address: super::DWORD64) -> super::DWORD {
    windows_core::link!("dbghelp.dll" "system" fn SymAddrIncludeInlineTrace(hprocess : super::HANDLE, address : super::DWORD64) -> super::DWORD);
    unsafe { SymAddrIncludeInlineTrace(hprocess, address) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymCleanup(hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymCleanup(hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { SymCleanup(hprocess) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymCompareInlineTrace(hprocess: super::HANDLE, address1: super::DWORD64, inlinecontext1: super::DWORD, retaddress1: super::DWORD64, address2: super::DWORD64, retaddress2: super::DWORD64) -> super::DWORD {
    windows_core::link!("dbghelp.dll" "system" fn SymCompareInlineTrace(hprocess : super::HANDLE, address1 : super::DWORD64, inlinecontext1 : super::DWORD, retaddress1 : super::DWORD64, address2 : super::DWORD64, retaddress2 : super::DWORD64) -> super::DWORD);
    unsafe { SymCompareInlineTrace(hprocess, address1, inlinecontext1, retaddress1, address2, retaddress2) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymDeleteSymbol<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, name: P2, address: super::DWORD64, flags: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymDeleteSymbol(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_core::PCSTR, address : super::DWORD64, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymDeleteSymbol(hprocess, baseofdll, name.param().abi(), address, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymDeleteSymbolW<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, name: P2, address: super::DWORD64, flags: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymDeleteSymbolW(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_core::PCWSTR, address : super::DWORD64, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymDeleteSymbolW(hprocess, baseofdll, name.param().abi(), address, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumLines<P2, P3>(hprocess: super::HANDLE, base: super::ULONG64, obj: P2, file: P3, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumLines(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_core::PCSTR, file : windows_core::PCSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumLines(hprocess, base, obj.param().abi(), file.param().abi(), enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumLinesW<P2, P3>(hprocess: super::HANDLE, base: super::ULONG64, obj: P2, file: P3, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumLinesW(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_core::PCWSTR, file : windows_core::PCWSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumLinesW(hprocess, base, obj.param().abi(), file.param().abi(), enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymEnumProcesses(enumprocessescallback: PSYM_ENUMPROCESSES_CALLBACK, usercontext: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumProcesses(enumprocessescallback : PSYM_ENUMPROCESSES_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumProcesses(enumprocessescallback, usercontext) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSourceFileTokens(hprocess: super::HANDLE, base: super::ULONG64, callback: PENUMSOURCEFILETOKENSCALLBACK) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceFileTokens(hprocess : super::HANDLE, base : super::ULONG64, callback : PENUMSOURCEFILETOKENSCALLBACK) -> windows_core::BOOL);
    unsafe { SymEnumSourceFileTokens(hprocess, base, callback) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSourceFiles<P2>(hprocess: super::HANDLE, modbase: super::ULONG64, mask: P2, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceFiles(hprocess : super::HANDLE, modbase : super::ULONG64, mask : windows_core::PCSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSourceFiles(hprocess, modbase, mask.param().abi(), cbsrcfiles, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSourceFilesW<P2>(hprocess: super::HANDLE, modbase: super::ULONG64, mask: P2, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceFilesW(hprocess : super::HANDLE, modbase : super::ULONG64, mask : windows_core::PCWSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSourceFilesW(hprocess, modbase, mask.param().abi(), cbsrcfiles, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSourceLines<P2, P3>(hprocess: super::HANDLE, base: super::ULONG64, obj: P2, file: P3, line: Option<super::DWORD>, flags: super::DWORD, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceLines(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_core::PCSTR, file : windows_core::PCSTR, line : super::DWORD, flags : super::DWORD, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSourceLines(hprocess, base, obj.param().abi(), file.param().abi(), line.unwrap_or(core::mem::zeroed()) as _, flags, enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSourceLinesW<P2, P3>(hprocess: super::HANDLE, base: super::ULONG64, obj: P2, file: P3, line: Option<super::DWORD>, flags: super::DWORD, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSourceLinesW(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_core::PCWSTR, file : windows_core::PCWSTR, line : super::DWORD, flags : super::DWORD, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSourceLinesW(hprocess, base, obj.param().abi(), file.param().abi(), line.unwrap_or(core::mem::zeroed()) as _, flags, enumlinescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSym(hprocess: super::HANDLE, baseofdll: super::ULONG64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSym(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSym(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSymbols<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbols(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSymbols(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSymbolsEx<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<super::PVOID>, options: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsEx(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID, options : super::DWORD) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsEx(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSymbolsExW<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<super::PVOID>, options: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsExW(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID, options : super::DWORD) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsExW(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSymbolsForAddr(hprocess: super::HANDLE, address: super::DWORD64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddr(hprocess : super::HANDLE, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsForAddr(hprocess, address, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSymbolsForAddrW(hprocess: super::HANDLE, address: super::DWORD64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddrW(hprocess : super::HANDLE, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsForAddrW(hprocess, address, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumSymbolsW<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumSymbolsW(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumSymbolsW(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumTypes(hprocess: super::HANDLE, baseofdll: super::ULONG64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypes(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumTypes(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumTypesByName<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypesByName(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumTypesByName(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumTypesByNameW<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, mask: P2, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypesByNameW(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumTypesByNameW(hprocess, baseofdll, mask.param().abi(), enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumTypesW(hprocess: super::HANDLE, baseofdll: super::ULONG64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumTypesW(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumTypesW(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateModules(hprocess: super::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateModules(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateModules(hprocess, enummodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateModules(hprocess: super::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymEnumerateModules64" fn SymEnumerateModules(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateModules(hprocess, enummodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateModules64(hprocess: super::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateModules64(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateModules64(hprocess, enummodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateModulesW64(hprocess: super::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACKW64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateModulesW64(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACKW64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateModulesW64(hprocess, enummodulescallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateSymbols(hprocess: super::HANDLE, baseofdll: super::ULONG, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbols(hprocess : super::HANDLE, baseofdll : super::ULONG, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbols(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateSymbols(hprocess: super::HANDLE, baseofdll: super::ULONG64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymEnumerateSymbols64" fn SymEnumerateSymbols(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbols(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateSymbols64(hprocess: super::HANDLE, baseofdll: super::ULONG64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbols64(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbols64(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateSymbolsW(hprocess: super::HANDLE, baseofdll: super::ULONG, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACKW, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW(hprocess : super::HANDLE, baseofdll : super::ULONG, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbolsW(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateSymbolsW(hprocess: super::HANDLE, baseofdll: super::ULONG64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymEnumerateSymbolsW64" fn SymEnumerateSymbolsW(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbolsW(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymEnumerateSymbolsW64(hprocess: super::HANDLE, baseofdll: super::ULONG64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW64(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymEnumerateSymbolsW64(hprocess, baseofdll, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindDebugInfoFile<P1>(hprocess: super::HANDLE, filename: P1, debugfilepath: windows_core::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: Option<super::PVOID>) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindDebugInfoFile(hprocess : super::HANDLE, filename : windows_core::PCSTR, debugfilepath : windows_core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { SymFindDebugInfoFile(hprocess, filename.param().abi(), debugfilepath, callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindDebugInfoFileW<P1>(hprocess: super::HANDLE, filename: P1, debugfilepath: windows_core::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: Option<super::PVOID>) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindDebugInfoFileW(hprocess : super::HANDLE, filename : windows_core::PCWSTR, debugfilepath : windows_core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { SymFindDebugInfoFileW(hprocess, filename.param().abi(), debugfilepath, callback, callerdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindExecutableImage<P1>(hprocess: super::HANDLE, filename: P1, imagefilepath: windows_core::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: super::PVOID) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindExecutableImage(hprocess : super::HANDLE, filename : windows_core::PCSTR, imagefilepath : windows_core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { SymFindExecutableImage(hprocess, filename.param().abi(), imagefilepath, callback, callerdata) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymFindExecutableImageW<P1>(hprocess: super::HANDLE, filename: P1, imagefilepath: windows_core::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: super::PVOID) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindExecutableImageW(hprocess : super::HANDLE, filename : windows_core::PCWSTR, imagefilepath : windows_core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
    unsafe { SymFindExecutableImageW(hprocess, filename.param().abi(), imagefilepath, callback, callerdata) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFindFileInPath<P1, P2>(hprocess: super::HANDLE, searchpath: P1, filename: P2, id: Option<super::PVOID>, two: super::DWORD, three: super::DWORD, flags: super::DWORD, foundfile: windows_core::PSTR, callback: PFINDFILEINPATHCALLBACK, context: Option<super::PVOID>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindFileInPath(hprocess : super::HANDLE, searchpath : windows_core::PCSTR, filename : windows_core::PCSTR, id : super::PVOID, two : super::DWORD, three : super::DWORD, flags : super::DWORD, foundfile : windows_core::PSTR, callback : PFINDFILEINPATHCALLBACK, context : super::PVOID) -> windows_core::BOOL);
    unsafe { SymFindFileInPath(hprocess, searchpath.param().abi(), filename.param().abi(), id.unwrap_or(core::mem::zeroed()) as _, two, three, flags, foundfile, callback, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFindFileInPathW<P1, P2>(hprocess: super::HANDLE, searchpath: P1, filename: P2, id: Option<super::PVOID>, two: super::DWORD, three: super::DWORD, flags: super::DWORD, foundfile: windows_core::PWSTR, callback: PFINDFILEINPATHCALLBACKW, context: Option<super::PVOID>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFindFileInPathW(hprocess : super::HANDLE, searchpath : windows_core::PCWSTR, filename : windows_core::PCWSTR, id : super::PVOID, two : super::DWORD, three : super::DWORD, flags : super::DWORD, foundfile : windows_core::PWSTR, callback : PFINDFILEINPATHCALLBACKW, context : super::PVOID) -> windows_core::BOOL);
    unsafe { SymFindFileInPathW(hprocess, searchpath.param().abi(), filename.param().abi(), id.unwrap_or(core::mem::zeroed()) as _, two, three, flags, foundfile, callback, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromAddr(hprocess: super::HANDLE, address: super::DWORD64, displacement: Option<super::PDWORD64>, symbol: PSYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromAddr(hprocess : super::HANDLE, address : super::DWORD64, displacement : super::PDWORD64, symbol : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromAddr(hprocess, address, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromAddrW(hprocess: super::HANDLE, address: super::DWORD64, displacement: Option<super::PDWORD64>, symbol: PSYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromAddrW(hprocess : super::HANDLE, address : super::DWORD64, displacement : super::PDWORD64, symbol : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromAddrW(hprocess, address, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromIndex(hprocess: super::HANDLE, baseofdll: super::ULONG64, index: super::DWORD, symbol: PSYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromIndex(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromIndex(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromIndexW(hprocess: super::HANDLE, baseofdll: super::ULONG64, index: super::DWORD, symbol: PSYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromIndexW(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromIndexW(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromInlineContext(hprocess: super::HANDLE, address: super::DWORD64, inlinecontext: super::ULONG, displacement: Option<super::PDWORD64>, symbol: PSYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromInlineContext(hprocess : super::HANDLE, address : super::DWORD64, inlinecontext : super::ULONG, displacement : super::PDWORD64, symbol : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromInlineContext(hprocess, address, inlinecontext, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromInlineContextW(hprocess: super::HANDLE, address: super::DWORD64, inlinecontext: super::ULONG, displacement: Option<super::PDWORD64>, symbol: PSYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromInlineContextW(hprocess : super::HANDLE, address : super::DWORD64, inlinecontext : super::ULONG, displacement : super::PDWORD64, symbol : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromInlineContextW(hprocess, address, inlinecontext, displacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromName<P1>(hprocess: super::HANDLE, name: P1, symbol: PSYMBOL_INFO) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFromName(hprocess : super::HANDLE, name : windows_core::PCSTR, symbol : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromName(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromNameW<P1>(hprocess: super::HANDLE, name: P1, symbol: PSYMBOL_INFOW) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymFromNameW(hprocess : super::HANDLE, name : windows_core::PCWSTR, symbol : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromNameW(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromToken(hprocess: super::HANDLE, base: super::DWORD64, token: super::DWORD, symbol: PSYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromToken(hprocess : super::HANDLE, base : super::DWORD64, token : super::DWORD, symbol : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymFromToken(hprocess, base, token, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFromTokenW(hprocess: super::HANDLE, base: super::DWORD64, token: super::DWORD, symbol: PSYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymFromTokenW(hprocess : super::HANDLE, base : super::DWORD64, token : super::DWORD, symbol : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymFromTokenW(hprocess, base, token, symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFunctionTableAccess(hprocess: super::HANDLE, addrbase: super::DWORD) -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" fn SymFunctionTableAccess(hprocess : super::HANDLE, addrbase : super::DWORD) -> super::PVOID);
    unsafe { SymFunctionTableAccess(hprocess, addrbase) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFunctionTableAccess(hprocess: super::HANDLE, addrbase: super::DWORD64) -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" "SymFunctionTableAccess64" fn SymFunctionTableAccess(hprocess : super::HANDLE, addrbase : super::DWORD64) -> super::PVOID);
    unsafe { SymFunctionTableAccess(hprocess, addrbase) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFunctionTableAccess64(hprocess: super::HANDLE, addrbase: super::DWORD64) -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64(hprocess : super::HANDLE, addrbase : super::DWORD64) -> super::PVOID);
    unsafe { SymFunctionTableAccess64(hprocess, addrbase) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymFunctionTableAccess64AccessRoutines(hprocess: super::HANDLE, addrbase: super::DWORD64, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64) -> super::PVOID {
    windows_core::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64AccessRoutines(hprocess : super::HANDLE, addrbase : super::DWORD64, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64) -> super::PVOID);
    unsafe { SymFunctionTableAccess64AccessRoutines(hprocess, addrbase, readmemoryroutine, getmodulebaseroutine) }
}
#[inline]
pub unsafe fn SymGetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS) -> windows_core::BOOL);
    unsafe { SymGetExtendedOption(option) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetFileLineOffsets64<P1, P2>(hprocess: super::HANDLE, modulename: P1, filename: P2, buffer: super::PDWORD64, bufferlines: super::ULONG) -> super::ULONG
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetFileLineOffsets64(hprocess : super::HANDLE, modulename : windows_core::PCSTR, filename : windows_core::PCSTR, buffer : super::PDWORD64, bufferlines : super::ULONG) -> super::ULONG);
    unsafe { SymGetFileLineOffsets64(hprocess, modulename.param().abi(), filename.param().abi(), buffer as _, bufferlines) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetHomeDirectory(r#type: super::DWORD, dir: windows_core::PSTR, size: super::size_t) -> super::PCHAR {
    windows_core::link!("dbghelp.dll" "system" fn SymGetHomeDirectory(r#type : super::DWORD, dir : windows_core::PSTR, size : super::size_t) -> super::PCHAR);
    unsafe { SymGetHomeDirectory(r#type, dir, size) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SymGetHomeDirectoryW(r#type: super::DWORD, dir: windows_core::PWSTR, size: super::size_t) -> windows_core::PWSTR {
    windows_core::link!("dbghelp.dll" "system" fn SymGetHomeDirectoryW(r#type : super::DWORD, dir : windows_core::PWSTR, size : super::size_t) -> windows_core::PWSTR);
    unsafe { SymGetHomeDirectoryW(r#type, dir, size) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromAddr(hprocess: super::HANDLE, dwaddr: super::DWORD, pdwdisplacement: super::PDWORD, line: PIMAGEHLP_LINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromAddr(hprocess : super::HANDLE, dwaddr : super::DWORD, pdwdisplacement : super::PDWORD, line : PIMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddr(hprocess, dwaddr, pdwdisplacement as _, line as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromAddr(hprocess: super::HANDLE, qwaddr: super::DWORD64, pdwdisplacement: super::PDWORD, line64: PIMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetLineFromAddr64" fn SymGetLineFromAddr(hprocess : super::HANDLE, qwaddr : super::DWORD64, pdwdisplacement : super::PDWORD, line64 : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddr(hprocess, qwaddr, pdwdisplacement as _, line64 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromAddr64(hprocess: super::HANDLE, qwaddr: super::DWORD64, pdwdisplacement: super::PDWORD, line64: PIMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromAddr64(hprocess : super::HANDLE, qwaddr : super::DWORD64, pdwdisplacement : super::PDWORD, line64 : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddr64(hprocess, qwaddr, pdwdisplacement as _, line64 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromAddrW(hprocess: super::HANDLE, dwaddr: super::DWORD64, pdwdisplacement: super::PDWORD, line: PIMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetLineFromAddrW64" fn SymGetLineFromAddrW(hprocess : super::HANDLE, dwaddr : super::DWORD64, pdwdisplacement : super::PDWORD, line : PIMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddrW(hprocess, dwaddr, pdwdisplacement as _, line as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromAddrW64(hprocess: super::HANDLE, dwaddr: super::DWORD64, pdwdisplacement: super::PDWORD, line: PIMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromAddrW64(hprocess : super::HANDLE, dwaddr : super::DWORD64, pdwdisplacement : super::PDWORD, line : PIMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineFromAddrW64(hprocess, dwaddr, pdwdisplacement as _, line as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromInlineContext(hprocess: super::HANDLE, qwaddr: super::DWORD64, inlinecontext: super::ULONG, qwmodulebaseaddress: Option<super::DWORD64>, pdwdisplacement: super::PDWORD, line64: PIMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContext(hprocess : super::HANDLE, qwaddr : super::DWORD64, inlinecontext : super::ULONG, qwmodulebaseaddress : super::DWORD64, pdwdisplacement : super::PDWORD, line64 : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromInlineContext(hprocess, qwaddr, inlinecontext, qwmodulebaseaddress.unwrap_or(core::mem::zeroed()) as _, pdwdisplacement as _, line64 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromInlineContextW(hprocess: super::HANDLE, dwaddr: super::DWORD64, inlinecontext: super::ULONG, qwmodulebaseaddress: Option<super::DWORD64>, pdwdisplacement: super::PDWORD, line: PIMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContextW(hprocess : super::HANDLE, dwaddr : super::DWORD64, inlinecontext : super::ULONG, qwmodulebaseaddress : super::DWORD64, pdwdisplacement : super::PDWORD, line : PIMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineFromInlineContextW(hprocess, dwaddr, inlinecontext, qwmodulebaseaddress.unwrap_or(core::mem::zeroed()) as _, pdwdisplacement as _, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromName<P1, P2>(hprocess: super::HANDLE, modulename: P1, filename: P2, dwlinenumber: super::DWORD, pldisplacement: super::PLONG, line: PIMAGEHLP_LINE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromName(hprocess : super::HANDLE, modulename : windows_core::PCSTR, filename : windows_core::PCSTR, dwlinenumber : super::DWORD, pldisplacement : super::PLONG, line : PIMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLineFromName(hprocess, modulename.param().abi(), filename.param().abi(), dwlinenumber, pldisplacement as _, line as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromName<P1, P2>(hprocess: super::HANDLE, modulename: P1, filename: P2, dwlinenumber: super::DWORD, pldisplacement: super::PLONG, line: PIMAGEHLP_LINE64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" "SymGetLineFromName64" fn SymGetLineFromName(hprocess : super::HANDLE, modulename : windows_core::PCSTR, filename : windows_core::PCSTR, dwlinenumber : super::DWORD, pldisplacement : super::PLONG, line : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromName(hprocess, modulename.param().abi(), filename.param().abi(), dwlinenumber, pldisplacement as _, line as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromName64<P1, P2>(hprocess: super::HANDLE, modulename: P1, filename: P2, dwlinenumber: super::DWORD, pldisplacement: super::PLONG, line: PIMAGEHLP_LINE64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromName64(hprocess : super::HANDLE, modulename : windows_core::PCSTR, filename : windows_core::PCSTR, dwlinenumber : super::DWORD, pldisplacement : super::PLONG, line : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineFromName64(hprocess, modulename.param().abi(), filename.param().abi(), dwlinenumber, pldisplacement as _, line as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineFromNameW64<P1, P2>(hprocess: super::HANDLE, modulename: P1, filename: P2, dwlinenumber: super::DWORD, pldisplacement: super::PLONG, line: PIMAGEHLP_LINEW64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineFromNameW64(hprocess : super::HANDLE, modulename : windows_core::PCWSTR, filename : windows_core::PCWSTR, dwlinenumber : super::DWORD, pldisplacement : super::PLONG, line : PIMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineFromNameW64(hprocess, modulename.param().abi(), filename.param().abi(), dwlinenumber, pldisplacement as _, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineNext(hprocess: super::HANDLE, line: PIMAGEHLP_LINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineNext(hprocess : super::HANDLE, line : PIMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLineNext(hprocess, line as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineNext(hprocess: super::HANDLE, line: PIMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetLineNext64" fn SymGetLineNext(hprocess : super::HANDLE, line : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineNext(hprocess, line as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineNext64(hprocess: super::HANDLE, line: PIMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineNext64(hprocess : super::HANDLE, line : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLineNext64(hprocess, line as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLineNextW64(hprocess: super::HANDLE, line: PIMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLineNextW64(hprocess : super::HANDLE, line : PIMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLineNextW64(hprocess, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLinePrev(hprocess: super::HANDLE, line: PIMAGEHLP_LINE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLinePrev(hprocess : super::HANDLE, line : PIMAGEHLP_LINE) -> windows_core::BOOL);
    unsafe { SymGetLinePrev(hprocess, line as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLinePrev(hprocess: super::HANDLE, line: PIMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetLinePrev64" fn SymGetLinePrev(hprocess : super::HANDLE, line : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLinePrev(hprocess, line as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLinePrev64(hprocess: super::HANDLE, line: PIMAGEHLP_LINE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLinePrev64(hprocess : super::HANDLE, line : PIMAGEHLP_LINE64) -> windows_core::BOOL);
    unsafe { SymGetLinePrev64(hprocess, line as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetLinePrevW64(hprocess: super::HANDLE, line: PIMAGEHLP_LINEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetLinePrevW64(hprocess : super::HANDLE, line : PIMAGEHLP_LINEW64) -> windows_core::BOOL);
    unsafe { SymGetLinePrevW64(hprocess, line as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleBase(hprocess: super::HANDLE, dwaddr: super::DWORD) -> super::DWORD {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleBase(hprocess : super::HANDLE, dwaddr : super::DWORD) -> super::DWORD);
    unsafe { SymGetModuleBase(hprocess, dwaddr) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleBase(hprocess: super::HANDLE, qwaddr: super::DWORD64) -> super::DWORD64 {
    windows_core::link!("dbghelp.dll" "system" "SymGetModuleBase64" fn SymGetModuleBase(hprocess : super::HANDLE, qwaddr : super::DWORD64) -> super::DWORD64);
    unsafe { SymGetModuleBase(hprocess, qwaddr) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleBase64(hprocess: super::HANDLE, qwaddr: super::DWORD64) -> super::DWORD64 {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleBase64(hprocess : super::HANDLE, qwaddr : super::DWORD64) -> super::DWORD64);
    unsafe { SymGetModuleBase64(hprocess, qwaddr) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleInfo(hprocess: super::HANDLE, dwaddr: super::DWORD, moduleinfo: PIMAGEHLP_MODULE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfo(hprocess : super::HANDLE, dwaddr : super::DWORD, moduleinfo : PIMAGEHLP_MODULE) -> windows_core::BOOL);
    unsafe { SymGetModuleInfo(hprocess, dwaddr, moduleinfo as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleInfo(hprocess: super::HANDLE, qwaddr: super::DWORD64, moduleinfo: PIMAGEHLP_MODULE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetModuleInfo64" fn SymGetModuleInfo(hprocess : super::HANDLE, qwaddr : super::DWORD64, moduleinfo : PIMAGEHLP_MODULE64) -> windows_core::BOOL);
    unsafe { SymGetModuleInfo(hprocess, qwaddr, moduleinfo as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleInfo64(hprocess: super::HANDLE, qwaddr: super::DWORD64, moduleinfo: PIMAGEHLP_MODULE64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfo64(hprocess : super::HANDLE, qwaddr : super::DWORD64, moduleinfo : PIMAGEHLP_MODULE64) -> windows_core::BOOL);
    unsafe { SymGetModuleInfo64(hprocess, qwaddr, moduleinfo as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleInfoW(hprocess: super::HANDLE, dwaddr: super::DWORD, moduleinfo: PIMAGEHLP_MODULEW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfoW(hprocess : super::HANDLE, dwaddr : super::DWORD, moduleinfo : PIMAGEHLP_MODULEW) -> windows_core::BOOL);
    unsafe { SymGetModuleInfoW(hprocess, dwaddr, moduleinfo as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleInfoW(hprocess: super::HANDLE, qwaddr: super::DWORD64, moduleinfo: PIMAGEHLP_MODULEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetModuleInfoW64" fn SymGetModuleInfoW(hprocess : super::HANDLE, qwaddr : super::DWORD64, moduleinfo : PIMAGEHLP_MODULEW64) -> windows_core::BOOL);
    unsafe { SymGetModuleInfoW(hprocess, qwaddr, moduleinfo as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetModuleInfoW64(hprocess: super::HANDLE, qwaddr: super::DWORD64, moduleinfo: PIMAGEHLP_MODULEW64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetModuleInfoW64(hprocess : super::HANDLE, qwaddr : super::DWORD64, moduleinfo : PIMAGEHLP_MODULEW64) -> windows_core::BOOL);
    unsafe { SymGetModuleInfoW64(hprocess, qwaddr, moduleinfo as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetOmaps(hprocess: super::HANDLE, baseofdll: super::DWORD64, omapto: *mut POMAP, comapto: super::PDWORD64, omapfrom: *mut POMAP, comapfrom: super::PDWORD64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetOmaps(hprocess : super::HANDLE, baseofdll : super::DWORD64, omapto : *mut POMAP, comapto : super::PDWORD64, omapfrom : *mut POMAP, comapfrom : super::PDWORD64) -> windows_core::BOOL);
    unsafe { SymGetOmaps(hprocess, baseofdll, omapto as _, comapto as _, omapfrom as _, comapfrom as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SymGetOptions() -> super::DWORD {
    windows_core::link!("dbghelp.dll" "system" fn SymGetOptions() -> super::DWORD);
    unsafe { SymGetOptions() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SymGetParentWindow(phwnd: *mut super::HWND) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetParentWindow(phwnd : *mut super::HWND) -> windows_core::BOOL);
    unsafe { SymGetParentWindow(phwnd as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetScope(hprocess: super::HANDLE, baseofdll: super::ULONG64, index: super::DWORD, symbol: PSYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetScope(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymGetScope(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetScopeW(hprocess: super::HANDLE, baseofdll: super::ULONG64, index: super::DWORD, symbol: PSYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetScopeW(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymGetScopeW(hprocess, baseofdll, index, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSearchPath(hprocess: super::HANDLE, searchpath: windows_core::PSTR, searchpathlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSearchPath(hprocess : super::HANDLE, searchpath : windows_core::PSTR, searchpathlength : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSearchPath(hprocess, searchpath, searchpathlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSearchPathW(hprocess: super::HANDLE, searchpath: windows_core::PWSTR, searchpathlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSearchPathW(hprocess : super::HANDLE, searchpath : windows_core::PWSTR, searchpathlength : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSearchPathW(hprocess, searchpath, searchpathlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFile<P2, P3>(hprocess: super::HANDLE, base: super::ULONG64, params: P2, filespec: P3, filepath: windows_core::PSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFile(hprocess : super::HANDLE, base : super::ULONG64, params : windows_core::PCSTR, filespec : windows_core::PCSTR, filepath : windows_core::PSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFile(hprocess, base, params.param().abi(), filespec.param().abi(), filepath, size) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileChecksum<P2>(hprocess: super::HANDLE, base: super::ULONG64, filespec: P2, pchecksumtype: *mut super::DWORD, pchecksum: *mut super::BYTE, checksumsize: super::DWORD, pactualbyteswritten: *mut super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksum(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_core::PCSTR, pchecksumtype : *mut super::DWORD, pchecksum : *mut super::BYTE, checksumsize : super::DWORD, pactualbyteswritten : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileChecksum(hprocess, base, filespec.param().abi(), pchecksumtype as _, pchecksum as _, checksumsize, pactualbyteswritten as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileChecksumW<P2>(hprocess: super::HANDLE, base: super::ULONG64, filespec: P2, pchecksumtype: *mut super::DWORD, pchecksum: *mut super::BYTE, checksumsize: super::DWORD, pactualbyteswritten: *mut super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksumW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_core::PCWSTR, pchecksumtype : *mut super::DWORD, pchecksum : *mut super::BYTE, checksumsize : super::DWORD, pactualbyteswritten : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileChecksumW(hprocess, base, filespec.param().abi(), pchecksumtype as _, pchecksum as _, checksumsize, pactualbyteswritten as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileFromToken<P2>(hprocess: super::HANDLE, token: super::PVOID, params: P2, filepath: windows_core::PSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromToken(hprocess : super::HANDLE, token : super::PVOID, params : windows_core::PCSTR, filepath : windows_core::PSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromToken(hprocess, token, params.param().abi(), filepath, size) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenByTokenName<P2, P3>(hprocess: super::HANDLE, token: super::PVOID, tokenname: P2, params: P3, filepath: windows_core::PSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenName(hprocess : super::HANDLE, token : super::PVOID, tokenname : windows_core::PCSTR, params : windows_core::PCSTR, filepath : windows_core::PSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromTokenByTokenName(hprocess, token, tokenname.param().abi(), params.param().abi(), filepath, size) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenByTokenNameW<P2, P3>(hprocess: super::HANDLE, token: super::PVOID, tokenname: P2, params: P3, filepath: windows_core::PWSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenNameW(hprocess : super::HANDLE, token : super::PVOID, tokenname : windows_core::PCWSTR, params : windows_core::PCWSTR, filepath : windows_core::PWSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromTokenByTokenNameW(hprocess, token, tokenname.param().abi(), params.param().abi(), filepath, size) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenW<P2>(hprocess: super::HANDLE, token: super::PVOID, params: P2, filepath: windows_core::PWSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenW(hprocess : super::HANDLE, token : super::PVOID, params : windows_core::PCWSTR, filepath : windows_core::PWSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileFromTokenW(hprocess, token, params.param().abi(), filepath, size) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileToken<P2>(hprocess: super::HANDLE, base: super::ULONG64, filespec: P2, token: *mut super::PVOID, size: *mut super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileToken(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_core::PCSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileToken(hprocess, base, filespec.param().abi(), token as _, size as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileTokenByTokenName<P2, P3, P4>(hprocess: super::HANDLE, base: super::ULONG64, filespec: P2, tokenname: P3, tokenparameters: P4, token: *mut super::PVOID, size: *mut super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenName(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_core::PCSTR, tokenname : windows_core::PCSTR, tokenparameters : windows_core::PCSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileTokenByTokenName(hprocess, base, filespec.param().abi(), tokenname.param().abi(), tokenparameters.param().abi(), token as _, size as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileTokenByTokenNameW<P2, P3, P4>(hprocess: super::HANDLE, base: super::ULONG64, filespec: P2, tokenname: P3, tokenparameters: P4, token: *mut super::PVOID, size: *mut super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenNameW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_core::PCWSTR, tokenname : windows_core::PCWSTR, tokenparameters : windows_core::PCWSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileTokenByTokenNameW(hprocess, base, filespec.param().abi(), tokenname.param().abi(), tokenparameters.param().abi(), token as _, size as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileTokenW<P2>(hprocess: super::HANDLE, base: super::ULONG64, filespec: P2, token: *mut super::PVOID, size: *mut super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_core::PCWSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileTokenW(hprocess, base, filespec.param().abi(), token as _, size as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceFileW<P2, P3>(hprocess: super::HANDLE, base: super::ULONG64, params: P2, filespec: P3, filepath: windows_core::PWSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceFileW(hprocess : super::HANDLE, base : super::ULONG64, params : windows_core::PCWSTR, filespec : windows_core::PCWSTR, filepath : windows_core::PWSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceFileW(hprocess, base, params.param().abi(), filespec.param().abi(), filepath, size) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceVarFromToken<P2, P3>(hprocess: super::HANDLE, token: super::PVOID, params: P2, varname: P3, value: windows_core::PSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceVarFromToken(hprocess : super::HANDLE, token : super::PVOID, params : windows_core::PCSTR, varname : windows_core::PCSTR, value : windows_core::PSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceVarFromToken(hprocess, token, params.param().abi(), varname.param().abi(), value, size) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSourceVarFromTokenW<P2, P3>(hprocess: super::HANDLE, token: super::PVOID, params: P2, varname: P3, value: windows_core::PWSTR, size: super::DWORD) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSourceVarFromTokenW(hprocess : super::HANDLE, token : super::PVOID, params : windows_core::PCWSTR, varname : windows_core::PCWSTR, value : windows_core::PWSTR, size : super::DWORD) -> windows_core::BOOL);
    unsafe { SymGetSourceVarFromTokenW(hprocess, token, params.param().abi(), varname.param().abi(), value, size) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymFromAddr(hprocess: super::HANDLE, dwaddr: super::DWORD, pdwdisplacement: Option<super::PDWORD>, symbol: PIMAGEHLP_SYMBOL) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromAddr(hprocess : super::HANDLE, dwaddr : super::DWORD, pdwdisplacement : super::PDWORD, symbol : PIMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymFromAddr(hprocess, dwaddr, pdwdisplacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymFromAddr(hprocess: super::HANDLE, qwaddr: super::DWORD64, pdwdisplacement: Option<super::PDWORD64>, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetSymFromAddr64" fn SymGetSymFromAddr(hprocess : super::HANDLE, qwaddr : super::DWORD64, pdwdisplacement : super::PDWORD64, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymFromAddr(hprocess, qwaddr, pdwdisplacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymFromAddr64(hprocess: super::HANDLE, qwaddr: super::DWORD64, pdwdisplacement: Option<super::PDWORD64>, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromAddr64(hprocess : super::HANDLE, qwaddr : super::DWORD64, pdwdisplacement : super::PDWORD64, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymFromAddr64(hprocess, qwaddr, pdwdisplacement.unwrap_or(core::mem::zeroed()) as _, symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymFromName<P1>(hprocess: super::HANDLE, name: P1, symbol: PIMAGEHLP_SYMBOL) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromName(hprocess : super::HANDLE, name : windows_core::PCSTR, symbol : PIMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymFromName(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymFromName<P1>(hprocess: super::HANDLE, name: P1, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" "SymGetSymFromName64" fn SymGetSymFromName(hprocess : super::HANDLE, name : windows_core::PCSTR, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymFromName(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymFromName64<P1>(hprocess: super::HANDLE, name: P1, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymFromName64(hprocess : super::HANDLE, name : windows_core::PCSTR, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymFromName64(hprocess, name.param().abi(), symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymNext(hprocess: super::HANDLE, symbol: PIMAGEHLP_SYMBOL) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymNext(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymNext(hprocess, symbol as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymNext(hprocess: super::HANDLE, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetSymNext64" fn SymGetSymNext(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymNext(hprocess, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymNext64(hprocess: super::HANDLE, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymNext64(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymNext64(hprocess, symbol as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymPrev(hprocess: super::HANDLE, symbol: PIMAGEHLP_SYMBOL) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymPrev(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL) -> windows_core::BOOL);
    unsafe { SymGetSymPrev(hprocess, symbol as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymPrev(hprocess: super::HANDLE, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymGetSymPrev64" fn SymGetSymPrev(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymPrev(hprocess, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymPrev64(hprocess: super::HANDLE, symbol: PIMAGEHLP_SYMBOL64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymPrev64(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL64) -> windows_core::BOOL);
    unsafe { SymGetSymPrev64(hprocess, symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymbolFile<P1, P2>(hprocess: Option<super::HANDLE>, sympath: P1, imagefile: P2, r#type: super::DWORD, symbolfile: windows_core::PSTR, csymbolfile: super::size_t, dbgfile: windows_core::PSTR, cdbgfile: super::size_t) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymbolFile(hprocess : super::HANDLE, sympath : windows_core::PCSTR, imagefile : windows_core::PCSTR, r#type : super::DWORD, symbolfile : windows_core::PSTR, csymbolfile : super::size_t, dbgfile : windows_core::PSTR, cdbgfile : super::size_t) -> windows_core::BOOL);
    unsafe { SymGetSymbolFile(hprocess.unwrap_or(core::mem::zeroed()) as _, sympath.param().abi(), imagefile.param().abi(), r#type, symbolfile, csymbolfile, dbgfile, cdbgfile) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetSymbolFileW<P1, P2>(hprocess: Option<super::HANDLE>, sympath: P1, imagefile: P2, r#type: super::DWORD, symbolfile: windows_core::PWSTR, csymbolfile: super::size_t, dbgfile: windows_core::PWSTR, cdbgfile: super::size_t) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetSymbolFileW(hprocess : super::HANDLE, sympath : windows_core::PCWSTR, imagefile : windows_core::PCWSTR, r#type : super::DWORD, symbolfile : windows_core::PWSTR, csymbolfile : super::size_t, dbgfile : windows_core::PWSTR, cdbgfile : super::size_t) -> windows_core::BOOL);
    unsafe { SymGetSymbolFileW(hprocess.unwrap_or(core::mem::zeroed()) as _, sympath.param().abi(), imagefile.param().abi(), r#type, symbolfile, csymbolfile, dbgfile, cdbgfile) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetTypeFromName<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, name: P2, symbol: PSYMBOL_INFO) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeFromName(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_core::PCSTR, symbol : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymGetTypeFromName(hprocess, baseofdll, name.param().abi(), symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetTypeFromNameW<P2>(hprocess: super::HANDLE, baseofdll: super::ULONG64, name: P2, symbol: PSYMBOL_INFOW) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeFromNameW(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_core::PCWSTR, symbol : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymGetTypeFromNameW(hprocess, baseofdll, name.param().abi(), symbol as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetTypeInfo(hprocess: super::HANDLE, modbase: super::DWORD64, typeid: super::ULONG, gettype: IMAGEHLP_SYMBOL_TYPE_INFO, pinfo: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeInfo(hprocess : super::HANDLE, modbase : super::DWORD64, typeid : super::ULONG, gettype : IMAGEHLP_SYMBOL_TYPE_INFO, pinfo : super::PVOID) -> windows_core::BOOL);
    unsafe { SymGetTypeInfo(hprocess, modbase, typeid, gettype, pinfo as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetTypeInfoEx(hprocess: super::HANDLE, modbase: super::DWORD64, params: PIMAGEHLP_GET_TYPE_INFO_PARAMS) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetTypeInfoEx(hprocess : super::HANDLE, modbase : super::DWORD64, params : PIMAGEHLP_GET_TYPE_INFO_PARAMS) -> windows_core::BOOL);
    unsafe { SymGetTypeInfoEx(hprocess, modbase, params as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymGetUnwindInfo(hprocess: super::HANDLE, address: super::DWORD64, buffer: Option<super::PVOID>, size: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymGetUnwindInfo(hprocess : super::HANDLE, address : super::DWORD64, buffer : super::PVOID, size : super::PULONG) -> windows_core::BOOL);
    unsafe { SymGetUnwindInfo(hprocess, address, buffer.unwrap_or(core::mem::zeroed()) as _, size as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymInitialize<P1>(hprocess: super::HANDLE, usersearchpath: P1, finvadeprocess: bool) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymInitialize(hprocess : super::HANDLE, usersearchpath : windows_core::PCSTR, finvadeprocess : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymInitialize(hprocess, usersearchpath.param().abi(), finvadeprocess.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymInitializeW<P1>(hprocess: super::HANDLE, usersearchpath: P1, finvadeprocess: bool) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymInitializeW(hprocess : super::HANDLE, usersearchpath : windows_core::PCWSTR, finvadeprocess : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymInitializeW(hprocess, usersearchpath.param().abi(), finvadeprocess.into()) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymLoadModule<P2, P3>(hprocess: super::HANDLE, hfile: Option<super::HANDLE>, imagename: P2, modulename: P3, baseofdll: super::DWORD, sizeofdll: super::DWORD) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModule(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_core::PCSTR, modulename : windows_core::PCSTR, baseofdll : super::DWORD, sizeofdll : super::DWORD) -> super::DWORD);
    unsafe { SymLoadModule(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, sizeofdll) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymLoadModule<P2, P3>(hprocess: super::HANDLE, hfile: Option<super::HANDLE>, imagename: P2, modulename: P3, baseofdll: super::DWORD64, sizeofdll: super::DWORD) -> super::DWORD64
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" "SymLoadModule64" fn SymLoadModule(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_core::PCSTR, modulename : windows_core::PCSTR, baseofdll : super::DWORD64, sizeofdll : super::DWORD) -> super::DWORD64);
    unsafe { SymLoadModule(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, sizeofdll) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymLoadModule64<P2, P3>(hprocess: super::HANDLE, hfile: Option<super::HANDLE>, imagename: P2, modulename: P3, baseofdll: super::DWORD64, sizeofdll: super::DWORD) -> super::DWORD64
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModule64(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_core::PCSTR, modulename : windows_core::PCSTR, baseofdll : super::DWORD64, sizeofdll : super::DWORD) -> super::DWORD64);
    unsafe { SymLoadModule64(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, sizeofdll) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymLoadModuleEx<P2, P3>(hprocess: super::HANDLE, hfile: Option<super::HANDLE>, imagename: P2, modulename: P3, baseofdll: super::DWORD64, dllsize: super::DWORD, data: Option<PMODLOAD_DATA>, flags: Option<super::DWORD>) -> super::DWORD64
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModuleEx(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_core::PCSTR, modulename : windows_core::PCSTR, baseofdll : super::DWORD64, dllsize : super::DWORD, data : PMODLOAD_DATA, flags : super::DWORD) -> super::DWORD64);
    unsafe { SymLoadModuleEx(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, dllsize, data.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymLoadModuleExW<P2, P3>(hprocess: super::HANDLE, hfile: Option<super::HANDLE>, imagename: P2, modulename: P3, baseofdll: super::DWORD64, dllsize: super::DWORD, data: Option<PMODLOAD_DATA>, flags: Option<super::DWORD>) -> super::DWORD64
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymLoadModuleExW(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_core::PCWSTR, modulename : windows_core::PCWSTR, baseofdll : super::DWORD64, dllsize : super::DWORD, data : PMODLOAD_DATA, flags : super::DWORD) -> super::DWORD64);
    unsafe { SymLoadModuleExW(hprocess, hfile.unwrap_or(core::mem::zeroed()) as _, imagename.param().abi(), modulename.param().abi(), baseofdll, dllsize, data.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SymMatchFileName<P0, P1>(filename: P0, r#match: P1, filenamestop: Option<*mut windows_core::PSTR>, matchstop: Option<*mut windows_core::PSTR>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymMatchFileName(filename : windows_core::PCSTR, r#match : windows_core::PCSTR, filenamestop : *mut windows_core::PSTR, matchstop : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SymMatchFileName(filename.param().abi(), r#match.param().abi(), filenamestop.unwrap_or(core::mem::zeroed()) as _, matchstop.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SymMatchFileNameW<P0, P1>(filename: P0, r#match: P1, filenamestop: Option<*mut windows_core::PWSTR>, matchstop: Option<*mut windows_core::PWSTR>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymMatchFileNameW(filename : windows_core::PCWSTR, r#match : windows_core::PCWSTR, filenamestop : *mut windows_core::PWSTR, matchstop : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SymMatchFileNameW(filename.param().abi(), r#match.param().abi(), filenamestop.unwrap_or(core::mem::zeroed()) as _, matchstop.unwrap_or(core::mem::zeroed()) as _) }
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymNext(hprocess: super::HANDLE, si: PSYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymNext(hprocess : super::HANDLE, si : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymNext(hprocess, si as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymNextW(hprocess: super::HANDLE, siw: PSYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymNextW(hprocess : super::HANDLE, siw : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymNextW(hprocess, siw as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymPrev(hprocess: super::HANDLE, si: PSYMBOL_INFO) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymPrev(hprocess : super::HANDLE, si : PSYMBOL_INFO) -> windows_core::BOOL);
    unsafe { SymPrev(hprocess, si as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymPrevW(hprocess: super::HANDLE, siw: PSYMBOL_INFOW) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymPrevW(hprocess : super::HANDLE, siw : PSYMBOL_INFOW) -> windows_core::BOOL);
    unsafe { SymPrevW(hprocess, siw as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymQueryInlineTrace(hprocess: super::HANDLE, startaddress: super::DWORD64, startcontext: super::DWORD, startretaddress: super::DWORD64, curaddress: super::DWORD64, curcontext: super::LPDWORD, curframeindex: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymQueryInlineTrace(hprocess : super::HANDLE, startaddress : super::DWORD64, startcontext : super::DWORD, startretaddress : super::DWORD64, curaddress : super::DWORD64, curcontext : super::LPDWORD, curframeindex : super::LPDWORD) -> windows_core::BOOL);
    unsafe { SymQueryInlineTrace(hprocess, startaddress, startcontext, startretaddress, curaddress, curcontext as _, curframeindex as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymRefreshModuleList(hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRefreshModuleList(hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { SymRefreshModuleList(hprocess) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymRegisterCallback(hprocess: super::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymRegisterCallback(hprocess, callbackfunction, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymRegisterCallback(hprocess: super::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymRegisterCallback64" fn SymRegisterCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : super::ULONG64) -> windows_core::BOOL);
    unsafe { SymRegisterCallback(hprocess, callbackfunction, usercontext) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymRegisterCallback64(hprocess: super::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterCallback64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : super::ULONG64) -> windows_core::BOOL);
    unsafe { SymRegisterCallback64(hprocess, callbackfunction, usercontext) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymRegisterCallbackW64(hprocess: super::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterCallbackW64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : super::ULONG64) -> windows_core::BOOL);
    unsafe { SymRegisterCallbackW64(hprocess, callbackfunction, usercontext) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymRegisterFunctionEntryCallback(hprocess: super::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK, usercontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK, usercontext : super::PVOID) -> windows_core::BOOL);
    unsafe { SymRegisterFunctionEntryCallback(hprocess, callbackfunction, usercontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymRegisterFunctionEntryCallback(hprocess: super::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK64, usercontext: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymRegisterFunctionEntryCallback64" fn SymRegisterFunctionEntryCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK64, usercontext : super::ULONG64) -> windows_core::BOOL);
    unsafe { SymRegisterFunctionEntryCallback(hprocess, callbackfunction, usercontext) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymRegisterFunctionEntryCallback64(hprocess: super::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK64, usercontext: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK64, usercontext : super::ULONG64) -> windows_core::BOOL);
    unsafe { SymRegisterFunctionEntryCallback64(hprocess, callbackfunction, usercontext) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSearch<P4>(hprocess: super::HANDLE, baseofdll: super::ULONG64, index: Option<super::DWORD>, symtag: Option<super::DWORD>, mask: P4, address: Option<super::DWORD64>, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: Option<super::PVOID>, options: super::DWORD) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSearch(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symtag : super::DWORD, mask : windows_core::PCSTR, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID, options : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSearch(hprocess, baseofdll, index.unwrap_or(core::mem::zeroed()) as _, symtag.unwrap_or(core::mem::zeroed()) as _, mask.param().abi(), address.unwrap_or(core::mem::zeroed()) as _, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSearchW<P4>(hprocess: super::HANDLE, baseofdll: super::ULONG64, index: Option<super::DWORD>, symtag: Option<super::DWORD>, mask: P4, address: Option<super::DWORD64>, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: Option<super::PVOID>, options: super::DWORD) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSearchW(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symtag : super::DWORD, mask : windows_core::PCWSTR, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID, options : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSearchW(hprocess, baseofdll, index.unwrap_or(core::mem::zeroed()) as _, symtag.unwrap_or(core::mem::zeroed()) as _, mask.param().abi(), address.unwrap_or(core::mem::zeroed()) as _, enumsymbolscallback, usercontext.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSetContext(hprocess: super::HANDLE, stackframe: PIMAGEHLP_STACK_FRAME, context: Option<PIMAGEHLP_CONTEXT>) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetContext(hprocess : super::HANDLE, stackframe : PIMAGEHLP_STACK_FRAME, context : PIMAGEHLP_CONTEXT) -> windows_core::BOOL);
    unsafe { SymSetContext(hprocess, stackframe, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SymSetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS, value: bool) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS, value : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SymSetExtendedOption(option, value.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetHomeDirectory<P1>(hprocess: Option<super::HANDLE>, dir: P1) -> super::PCHAR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetHomeDirectory(hprocess : super::HANDLE, dir : windows_core::PCSTR) -> super::PCHAR);
    unsafe { SymSetHomeDirectory(hprocess.unwrap_or(core::mem::zeroed()) as _, dir.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetHomeDirectoryW<P1>(hprocess: Option<super::HANDLE>, dir: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetHomeDirectoryW(hprocess : super::HANDLE, dir : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { SymSetHomeDirectoryW(hprocess.unwrap_or(core::mem::zeroed()) as _, dir.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SymSetOptions(symoptions: super::DWORD) -> super::DWORD {
    windows_core::link!("dbghelp.dll" "system" fn SymSetOptions(symoptions : super::DWORD) -> super::DWORD);
    unsafe { SymSetOptions(symoptions) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SymSetParentWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetParentWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { SymSetParentWindow(hwnd) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSetScopeFromAddr(hprocess: super::HANDLE, address: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetScopeFromAddr(hprocess : super::HANDLE, address : super::ULONG64) -> windows_core::BOOL);
    unsafe { SymSetScopeFromAddr(hprocess, address) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSetScopeFromIndex(hprocess: super::HANDLE, baseofdll: super::ULONG64, index: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetScopeFromIndex(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSetScopeFromIndex(hprocess, baseofdll, index) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSetScopeFromInlineContext(hprocess: super::HANDLE, address: super::ULONG64, inlinecontext: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymSetScopeFromInlineContext(hprocess : super::HANDLE, address : super::ULONG64, inlinecontext : super::ULONG) -> windows_core::BOOL);
    unsafe { SymSetScopeFromInlineContext(hprocess, address, inlinecontext) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetSearchPath<P1>(hprocess: super::HANDLE, searchpath: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetSearchPath(hprocess : super::HANDLE, searchpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SymSetSearchPath(hprocess, searchpath.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSetSearchPathW<P1>(hprocess: super::HANDLE, searchpath: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSetSearchPathW(hprocess : super::HANDLE, searchpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SymSetSearchPathW(hprocess, searchpath.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvDeltaName<P1, P2, P3, P4>(hprocess: super::HANDLE, sympath: P1, r#type: P2, file1: P3, file2: P4) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvDeltaName(hprocess : super::HANDLE, sympath : windows_core::PCSTR, r#type : windows_core::PCSTR, file1 : windows_core::PCSTR, file2 : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { SymSrvDeltaName(hprocess, sympath.param().abi(), r#type.param().abi(), file1.param().abi(), file2.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvDeltaNameW<P1, P2, P3, P4>(hprocess: super::HANDLE, sympath: P1, r#type: P2, file1: P3, file2: P4) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvDeltaNameW(hprocess : super::HANDLE, sympath : windows_core::PCWSTR, r#type : windows_core::PCWSTR, file1 : windows_core::PCWSTR, file2 : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { SymSrvDeltaNameW(hprocess, sympath.param().abi(), r#type.param().abi(), file1.param().abi(), file2.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SymSrvGetFileIndexInfo<P0>(file: P0, info: PSYMSRV_INDEX_INFO, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfo(file : windows_core::PCSTR, info : PSYMSRV_INDEX_INFO, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexInfo(file.param().abi(), info as _, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSrvGetFileIndexInfoW<P0>(file: P0, info: PSYMSRV_INDEX_INFOW, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfoW(file : windows_core::PCWSTR, info : PSYMSRV_INDEX_INFOW, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexInfoW(file.param().abi(), info as _, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSrvGetFileIndexString<P1, P2>(hprocess: super::HANDLE, srvpath: P1, file: P2, index: windows_core::PSTR, size: super::size_t, flags: super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexString(hprocess : super::HANDLE, srvpath : windows_core::PCSTR, file : windows_core::PCSTR, index : windows_core::PSTR, size : super::size_t, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexString(hprocess, srvpath.param().abi(), file.param().abi(), index, size, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSrvGetFileIndexStringW<P1, P2>(hprocess: super::HANDLE, srvpath: P1, file: P2, index: windows_core::PWSTR, size: super::size_t, flags: super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexStringW(hprocess : super::HANDLE, srvpath : windows_core::PCWSTR, file : windows_core::PCWSTR, index : windows_core::PWSTR, size : super::size_t, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexStringW(hprocess, srvpath.param().abi(), file.param().abi(), index, size, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SymSrvGetFileIndexes<P0>(file: P0, id: *mut windows_core::GUID, val1: super::PDWORD, val2: Option<super::PDWORD>, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexes(file : windows_core::PCSTR, id : *mut windows_core::GUID, val1 : super::PDWORD, val2 : super::PDWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexes(file.param().abi(), id as _, val1 as _, val2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SymSrvGetFileIndexesW<P0>(file: P0, id: *mut windows_core::GUID, val1: super::PDWORD, val2: Option<super::PDWORD>, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexesW(file : windows_core::PCWSTR, id : *mut windows_core::GUID, val1 : super::PDWORD, val2 : super::PDWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SymSrvGetFileIndexesW(file.param().abi(), id as _, val1 as _, val2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvGetSupplement<P1, P2, P3>(hprocess: super::HANDLE, sympath: P1, node: P2, file: P3) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetSupplement(hprocess : super::HANDLE, sympath : windows_core::PCSTR, node : windows_core::PCSTR, file : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { SymSrvGetSupplement(hprocess, sympath.param().abi(), node.param().abi(), file.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvGetSupplementW<P1, P2, P3>(hprocess: super::HANDLE, sympath: P1, node: P2, file: P3) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvGetSupplementW(hprocess : super::HANDLE, sympath : windows_core::PCWSTR, node : windows_core::PCWSTR, file : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { SymSrvGetSupplementW(hprocess, sympath.param().abi(), node.param().abi(), file.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvIsStore<P1>(hprocess: Option<super::HANDLE>, path: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvIsStore(hprocess : super::HANDLE, path : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SymSrvIsStore(hprocess.unwrap_or(core::mem::zeroed()) as _, path.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SymSrvIsStoreW<P1>(hprocess: Option<super::HANDLE>, path: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvIsStoreW(hprocess : super::HANDLE, path : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SymSrvIsStoreW(hprocess.unwrap_or(core::mem::zeroed()) as _, path.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSrvStoreFile<P1, P2>(hprocess: super::HANDLE, srvpath: P1, file: P2, flags: super::DWORD) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreFile(hprocess : super::HANDLE, srvpath : windows_core::PCSTR, file : windows_core::PCSTR, flags : super::DWORD) -> windows_core::PCSTR);
    unsafe { SymSrvStoreFile(hprocess, srvpath.param().abi(), file.param().abi(), flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSrvStoreFileW<P1, P2>(hprocess: super::HANDLE, srvpath: P1, file: P2, flags: super::DWORD) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreFileW(hprocess : super::HANDLE, srvpath : windows_core::PCWSTR, file : windows_core::PCWSTR, flags : super::DWORD) -> windows_core::PCWSTR);
    unsafe { SymSrvStoreFileW(hprocess, srvpath.param().abi(), file.param().abi(), flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSrvStoreSupplement<P1, P2, P3>(hprocess: super::HANDLE, srvpath: P1, node: P2, file: P3, flags: super::DWORD) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreSupplement(hprocess : super::HANDLE, srvpath : windows_core::PCSTR, node : windows_core::PCSTR, file : windows_core::PCSTR, flags : super::DWORD) -> windows_core::PCSTR);
    unsafe { SymSrvStoreSupplement(hprocess, srvpath.param().abi(), node.param().abi(), file.param().abi(), flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymSrvStoreSupplementW<P1, P2, P3>(hprocess: super::HANDLE, sympath: P1, node: P2, file: P3, flags: super::DWORD) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn SymSrvStoreSupplementW(hprocess : super::HANDLE, sympath : windows_core::PCWSTR, node : windows_core::PCWSTR, file : windows_core::PCWSTR, flags : super::DWORD) -> windows_core::PCWSTR);
    unsafe { SymSrvStoreSupplementW(hprocess, sympath.param().abi(), node.param().abi(), file.param().abi(), flags) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SymUnDName(sym: PIMAGEHLP_SYMBOL, undecname: windows_core::PSTR, undecnamelength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnDName(sym : PIMAGEHLP_SYMBOL, undecname : windows_core::PSTR, undecnamelength : super::DWORD) -> windows_core::BOOL);
    unsafe { SymUnDName(sym, undecname, undecnamelength) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SymUnDName(sym: PIMAGEHLP_SYMBOL64, undecname: windows_core::PSTR, undecnamelength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymUnDName64" fn SymUnDName(sym : PIMAGEHLP_SYMBOL64, undecname : windows_core::PSTR, undecnamelength : super::DWORD) -> windows_core::BOOL);
    unsafe { SymUnDName(sym, undecname, undecnamelength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SymUnDName64(sym: PIMAGEHLP_SYMBOL64, undecname: windows_core::PSTR, undecnamelength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnDName64(sym : PIMAGEHLP_SYMBOL64, undecname : windows_core::PSTR, undecnamelength : super::DWORD) -> windows_core::BOOL);
    unsafe { SymUnDName64(sym, undecname, undecnamelength) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymUnloadModule(hprocess: super::HANDLE, baseofdll: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnloadModule(hprocess : super::HANDLE, baseofdll : super::DWORD) -> windows_core::BOOL);
    unsafe { SymUnloadModule(hprocess, baseofdll) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymUnloadModule(hprocess: super::HANDLE, baseofdll: super::DWORD64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" "SymUnloadModule64" fn SymUnloadModule(hprocess : super::HANDLE, baseofdll : super::DWORD64) -> windows_core::BOOL);
    unsafe { SymUnloadModule(hprocess, baseofdll) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SymUnloadModule64(hprocess: super::HANDLE, baseofdll: super::DWORD64) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn SymUnloadModule64(hprocess : super::HANDLE, baseofdll : super::DWORD64) -> windows_core::BOOL);
    unsafe { SymUnloadModule64(hprocess, baseofdll) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn UnDecorateSymbolName<P0>(name: P0, outputstring: windows_core::PSTR, maxstringlength: super::DWORD, flags: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn UnDecorateSymbolName(name : windows_core::PCSTR, outputstring : windows_core::PSTR, maxstringlength : super::DWORD, flags : super::DWORD) -> super::DWORD);
    unsafe { UnDecorateSymbolName(name.param().abi(), outputstring, maxstringlength, flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn UnDecorateSymbolNameW<P0>(name: P0, outputstring: windows_core::PWSTR, maxstringlength: super::DWORD, flags: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dbghelp.dll" "system" fn UnDecorateSymbolNameW(name : windows_core::PCWSTR, outputstring : windows_core::PWSTR, maxstringlength : super::DWORD, flags : super::DWORD) -> super::DWORD);
    unsafe { UnDecorateSymbolNameW(name.param().abi(), outputstring, maxstringlength, flags) }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnmapDebugInformation(debuginfo: PIMAGE_DEBUG_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("dbghelp.dll" "system" fn UnmapDebugInformation(debuginfo : PIMAGE_DEBUG_INFORMATION) -> windows_core::BOOL);
    unsafe { UnmapDebugInformation(debuginfo as _) }
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ADDRESS {
    pub Offset: super::DWORD,
    pub Segment: super::WORD,
    pub Mode: ADDRESS_MODE,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ADDRESS64 {
    pub Offset: super::DWORD64,
    pub Segment: super::WORD,
    pub Mode: ADDRESS_MODE,
}
pub type ADDRESS_MODE = i32;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct API_VERSION {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
    pub Revision: super::USHORT,
    pub Reserved: super::USHORT,
}
pub const API_VERSION_NUMBER: i32 = 12;
pub const AddrMode1616: ADDRESS_MODE = 0;
pub const AddrMode1632: ADDRESS_MODE = 1;
pub const AddrModeFlat: ADDRESS_MODE = 3;
pub const AddrModeReal: ADDRESS_MODE = 2;
pub const CBA_CHECK_ARM_MACHINE_THUMB_TYPE_OVERRIDE: u32 = 2147483648;
pub const CBA_CHECK_ENGOPT_DISALLOW_NETWORK_PATHS: i32 = 1879048192;
pub const CBA_DEBUG_INFO: i32 = 268435456;
pub const CBA_DEFERRED_SYMBOL_LOAD_CANCEL: i32 = 7;
pub const CBA_DEFERRED_SYMBOL_LOAD_COMPLETE: i32 = 2;
pub const CBA_DEFERRED_SYMBOL_LOAD_FAILURE: i32 = 3;
pub const CBA_DEFERRED_SYMBOL_LOAD_PARTIAL: i32 = 32;
pub const CBA_DEFERRED_SYMBOL_LOAD_START: i32 = 1;
pub const CBA_DUPLICATE_SYMBOL: i32 = 5;
pub const CBA_ENGINE_PRESENT: i32 = 1610612736;
pub const CBA_EVENT: i32 = 16;
pub const CBA_MAP_JIT_SYMBOL: u32 = 2684354560;
pub const CBA_QUERY_IF_SYMBOL_DOWNLOAD_DISALLOWED: i32 = 64;
pub const CBA_READ_MEMORY: i32 = 6;
pub const CBA_SET_OPTIONS: i32 = 8;
pub const CBA_SRCSRV_EVENT: i32 = 1073741824;
pub const CBA_SRCSRV_INFO: i32 = 536870912;
pub const CBA_SYMBOLS_UNLOADED: i32 = 4;
pub const CBA_UPDATE_STATUS_BAR: i32 = 1342177280;
pub const CBA_XML_LOG: u32 = 2415919104;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct DBGHELP_DATA_REPORT_STRUCT {
    pub pBinPathNonExist: windows_core::PCWSTR,
    pub pSymbolPathNonExist: windows_core::PCWSTR,
}
pub const DBHHEADER_CVMISC: i32 = 2;
pub const DBHHEADER_DEBUGDIRS: i32 = 1;
pub const DBHHEADER_PDBGUID: i32 = 3;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISCRIMINATEDUNION_TAG_VALUE {
    pub value: [super::BYTE; 16],
    pub valueSizeBytes: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for DISCRIMINATEDUNION_TAG_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSLFLAG_MISMATCHED_DBG: i32 = 2;
pub const DSLFLAG_MISMATCHED_PDB: i32 = 1;
pub const ERROR_IMAGE_NOT_STRIPPED: i32 = 34816;
pub const ERROR_NO_DBG_POINTER: i32 = 34817;
pub const ERROR_NO_PDB_POINTER: i32 = 34818;
pub const ESLFLAG_FULLPATH: i32 = 1;
pub const ESLFLAG_INLINE_SITE: i32 = 16;
pub const ESLFLAG_NEAREST: i32 = 2;
pub const ESLFLAG_NEXT: i32 = 8;
pub const ESLFLAG_PREV: i32 = 4;
pub const EVENT_SRCSPEW: i32 = 100;
pub const EVENT_SRCSPEW_END: i32 = 199;
pub const EVENT_SRCSPEW_START: i32 = 100;
pub const EXT_OUTPUT_VER: i32 = 1;
pub const FLAG_ENGINE_PRESENT: i32 = 4;
pub const FLAG_ENGOPT_DISALLOW_NETWORK_PATHS: i32 = 8;
pub const FLAG_OVERRIDE_ARM_MACHINE_TYPE: i32 = 16;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_CBA_EVENT {
    pub severity: super::DWORD,
    pub code: super::DWORD,
    pub desc: super::PCHAR,
    pub object: super::PVOID,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_CBA_EVENTW {
    pub severity: super::DWORD,
    pub code: super::DWORD,
    pub desc: windows_core::PCWSTR,
    pub object: super::PVOID,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_CBA_READ_MEMORY {
    pub addr: super::DWORD64,
    pub buf: super::PVOID,
    pub bytes: super::DWORD,
    pub bytesread: *mut super::DWORD,
}
pub type IMAGEHLP_CONTEXT = core::ffi::c_void;
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    pub SizeOfStruct: super::DWORD,
    pub BaseOfImage: super::DWORD,
    pub CheckSum: super::DWORD,
    pub TimeDateStamp: super::DWORD,
    pub FileName: [super::CHAR; 260],
    pub Reparse: super::BOOLEAN,
    pub hFile: super::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    pub SizeOfStruct: super::DWORD,
    pub BaseOfImage: super::DWORD64,
    pub CheckSum: super::DWORD,
    pub TimeDateStamp: super::DWORD,
    pub FileName: [super::CHAR; 260],
    pub Reparse: super::BOOLEAN,
    pub hFile: super::HANDLE,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    pub SizeOfStruct: super::DWORD,
    pub BaseOfImage: super::DWORD64,
    pub CheckSum: super::DWORD,
    pub TimeDateStamp: super::DWORD,
    pub FileName: [super::WCHAR; 261],
    pub Reparse: super::BOOLEAN,
    pub hFile: super::HANDLE,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL {
    pub SizeOfStruct: super::DWORD,
    pub NumberOfDups: super::DWORD,
    pub Symbol: PIMAGEHLP_SYMBOL,
    pub SelectedSymbol: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL64 {
    pub SizeOfStruct: super::DWORD,
    pub NumberOfDups: super::DWORD,
    pub Symbol: PIMAGEHLP_SYMBOL64,
    pub SelectedSymbol: super::DWORD,
}
pub type IMAGEHLP_EXTENDED_OPTIONS = i32;
pub const IMAGEHLP_GET_TYPE_INFO_CHILDREN: i32 = 2;
#[repr(C, packed(8))]
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_GET_TYPE_INFO_PARAMS {
    pub SizeOfStruct: super::ULONG,
    pub Flags: super::ULONG,
    pub NumIds: super::ULONG,
    pub TypeIds: super::PULONG,
    pub TagFilter: super::ULONG64,
    pub NumReqs: super::ULONG,
    pub ReqKinds: *mut IMAGEHLP_SYMBOL_TYPE_INFO,
    pub ReqOffsets: super::PULONG_PTR,
    pub ReqSizes: super::PULONG,
    pub ReqStride: super::ULONG_PTR,
    pub BufferSize: super::ULONG_PTR,
    pub Buffer: super::PVOID,
    pub EntriesMatched: super::ULONG,
    pub EntriesFilled: super::ULONG,
    pub TagsFound: super::ULONG64,
    pub AllReqsValid: super::ULONG64,
    pub NumReqsValid: super::ULONG,
    pub ReqsValid: super::PULONG64,
}
pub const IMAGEHLP_GET_TYPE_INFO_UNCACHED: i32 = 1;
pub type IMAGEHLP_HD_TYPE = i32;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_JIT_SYMBOLMAP {
    pub SizeOfStruct: super::DWORD,
    pub Address: super::DWORD64,
    pub BaseOfImage: super::DWORD64,
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_LINE {
    pub SizeOfStruct: super::DWORD,
    pub Key: super::PVOID,
    pub LineNumber: super::DWORD,
    pub FileName: super::PCHAR,
    pub Address: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_LINE64 {
    pub SizeOfStruct: super::DWORD,
    pub Key: super::PVOID,
    pub LineNumber: super::DWORD,
    pub FileName: super::PCHAR,
    pub Address: super::DWORD64,
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_LINEW {
    pub SizeOfStruct: super::DWORD,
    pub Key: super::PVOID,
    pub LineNumber: super::DWORD,
    pub FileName: super::PCHAR,
    pub Address: super::DWORD64,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_LINEW64 {
    pub SizeOfStruct: super::DWORD,
    pub Key: super::PVOID,
    pub LineNumber: super::DWORD,
    pub FileName: windows_core::PWSTR,
    pub Address: super::DWORD64,
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_MODULE {
    pub SizeOfStruct: super::DWORD,
    pub BaseOfImage: super::DWORD,
    pub ImageSize: super::DWORD,
    pub TimeDateStamp: super::DWORD,
    pub CheckSum: super::DWORD,
    pub NumSyms: super::DWORD,
    pub SymType: SYM_TYPE,
    pub ModuleName: [super::CHAR; 32],
    pub ImageName: [super::CHAR; 256],
    pub LoadedImageName: [super::CHAR; 256],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_MODULE64 {
    pub SizeOfStruct: super::DWORD,
    pub BaseOfImage: super::DWORD64,
    pub ImageSize: super::DWORD,
    pub TimeDateStamp: super::DWORD,
    pub CheckSum: super::DWORD,
    pub NumSyms: super::DWORD,
    pub SymType: SYM_TYPE,
    pub ModuleName: [super::CHAR; 32],
    pub ImageName: [super::CHAR; 256],
    pub LoadedImageName: [super::CHAR; 256],
    pub LoadedPdbName: [super::CHAR; 256],
    pub CVSig: super::DWORD,
    pub CVData: [super::CHAR; 780],
    pub PdbSig: super::DWORD,
    pub PdbSig70: windows_core::GUID,
    pub PdbAge: super::DWORD,
    pub PdbUnmatched: windows_core::BOOL,
    pub DbgUnmatched: windows_core::BOOL,
    pub LineNumbers: windows_core::BOOL,
    pub GlobalSymbols: windows_core::BOOL,
    pub TypeInfo: windows_core::BOOL,
    pub SourceIndexed: windows_core::BOOL,
    pub Publics: windows_core::BOOL,
    pub MachineType: super::DWORD,
    pub Reserved: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_MODULE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_MODULE64_EX {
    pub Module: IMAGEHLP_MODULE64,
    pub RegionFlags: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_MODULEW {
    pub SizeOfStruct: super::DWORD,
    pub BaseOfImage: super::DWORD,
    pub ImageSize: super::DWORD,
    pub TimeDateStamp: super::DWORD,
    pub CheckSum: super::DWORD,
    pub NumSyms: super::DWORD,
    pub SymType: SYM_TYPE,
    pub ModuleName: [super::WCHAR; 32],
    pub ImageName: [super::WCHAR; 256],
    pub LoadedImageName: [super::WCHAR; 256],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_MODULEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_MODULEW64 {
    pub SizeOfStruct: super::DWORD,
    pub BaseOfImage: super::DWORD64,
    pub ImageSize: super::DWORD,
    pub TimeDateStamp: super::DWORD,
    pub CheckSum: super::DWORD,
    pub NumSyms: super::DWORD,
    pub SymType: SYM_TYPE,
    pub ModuleName: [super::WCHAR; 32],
    pub ImageName: [super::WCHAR; 256],
    pub LoadedImageName: [super::WCHAR; 256],
    pub LoadedPdbName: [super::WCHAR; 256],
    pub CVSig: super::DWORD,
    pub CVData: [super::WCHAR; 780],
    pub PdbSig: super::DWORD,
    pub PdbSig70: windows_core::GUID,
    pub PdbAge: super::DWORD,
    pub PdbUnmatched: windows_core::BOOL,
    pub DbgUnmatched: windows_core::BOOL,
    pub LineNumbers: windows_core::BOOL,
    pub GlobalSymbols: windows_core::BOOL,
    pub TypeInfo: windows_core::BOOL,
    pub SourceIndexed: windows_core::BOOL,
    pub Publics: windows_core::BOOL,
    pub MachineType: super::DWORD,
    pub Reserved: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_MODULEW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_MODULEW64_EX {
    pub Module: IMAGEHLP_MODULEW64,
    pub RegionFlags: super::DWORD,
}
pub const IMAGEHLP_MODULE_REGION_ADDITIONAL: i32 = 4;
pub const IMAGEHLP_MODULE_REGION_ALL: i32 = 255;
pub const IMAGEHLP_MODULE_REGION_DLLBASE: i32 = 1;
pub const IMAGEHLP_MODULE_REGION_DLLRANGE: i32 = 2;
pub const IMAGEHLP_MODULE_REGION_JIT: i32 = 8;
pub const IMAGEHLP_RMAP_BIG_ENDIAN: i32 = 2;
pub const IMAGEHLP_RMAP_FIXUP_ARM64X: i32 = 268435456;
pub const IMAGEHLP_RMAP_FIXUP_IMAGEBASE: u32 = 2147483648;
pub const IMAGEHLP_RMAP_IGNORE_MISCOMPARE: i32 = 4;
pub const IMAGEHLP_RMAP_LOAD_RW_DATA_SECTIONS: i32 = 536870912;
pub const IMAGEHLP_RMAP_MAPPED_FLAT: i32 = 1;
pub const IMAGEHLP_RMAP_NOBASERELOCATIONS: i32 = 134217728;
pub const IMAGEHLP_RMAP_OMIT_SHARED_RW_DATA_SECTIONS: i32 = 1073741824;
pub type IMAGEHLP_SF_TYPE = i32;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_STACK_FRAME {
    pub InstructionOffset: super::ULONG64,
    pub ReturnOffset: super::ULONG64,
    pub FrameOffset: super::ULONG64,
    pub StackOffset: super::ULONG64,
    pub BackingStoreOffset: super::ULONG64,
    pub FuncTableEntry: super::ULONG64,
    pub Params: [super::ULONG64; 4],
    pub Reserved: [super::ULONG64; 5],
    pub Virtual: windows_core::BOOL,
    pub Reserved2: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_STACK_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOL {
    pub SizeOfStruct: super::DWORD,
    pub Address: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub MaxNameLength: super::DWORD,
    pub Name: [super::CHAR; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOL64 {
    pub SizeOfStruct: super::DWORD,
    pub Address: super::DWORD64,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub MaxNameLength: super::DWORD,
    pub Name: [super::CHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_SYMBOL64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOL64_PACKAGE {
    pub sym: IMAGEHLP_SYMBOL64,
    pub name: [super::CHAR; 2001],
}
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_SYMBOL64_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOLW {
    pub SizeOfStruct: super::DWORD,
    pub Address: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub MaxNameLength: super::DWORD,
    pub Name: [super::WCHAR; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_SYMBOLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOLW64 {
    pub SizeOfStruct: super::DWORD,
    pub Address: super::DWORD64,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub MaxNameLength: super::DWORD,
    pub Name: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_SYMBOLW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOLW64_PACKAGE {
    pub sym: IMAGEHLP_SYMBOLW64,
    pub name: [super::WCHAR; 2001],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOLW_PACKAGE {
    pub sym: IMAGEHLP_SYMBOLW,
    pub name: [super::WCHAR; 2001],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGEHLP_SYMBOLW_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGEHLP_SYMBOL_FUNCTION: i32 = 2048;
pub const IMAGEHLP_SYMBOL_INFO_CONSTANT: i32 = 256;
pub const IMAGEHLP_SYMBOL_INFO_FRAMERELATIVE: i32 = 32;
pub const IMAGEHLP_SYMBOL_INFO_LOCAL: i32 = 128;
pub const IMAGEHLP_SYMBOL_INFO_PARAMETER: i32 = 64;
pub const IMAGEHLP_SYMBOL_INFO_REGISTER: i32 = 8;
pub const IMAGEHLP_SYMBOL_INFO_REGRELATIVE: i32 = 16;
pub const IMAGEHLP_SYMBOL_INFO_TLSRELATIVE: i32 = 16384;
pub const IMAGEHLP_SYMBOL_INFO_VALUEPRESENT: i32 = 1;
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOL_PACKAGE {
    pub sym: IMAGEHLP_SYMBOL,
    pub name: [super::CHAR; 2001],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_SYMBOL_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_SYMBOL_SRC {
    pub sizeofstruct: super::DWORD,
    pub r#type: super::DWORD,
    pub file: [i8; 260],
}
#[cfg(feature = "shared")]
impl Default for IMAGEHLP_SYMBOL_SRC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGEHLP_SYMBOL_THUNK: i32 = 8192;
pub type IMAGEHLP_SYMBOL_TYPE_INFO = i32;
pub const IMAGEHLP_SYMBOL_TYPE_INFO_MAX: IMAGEHLP_SYMBOL_TYPE_INFO = 39;
pub const IMAGEHLP_SYMBOL_VIRTUAL: i32 = 4096;
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMAGE_DEBUG_INFORMATION {
    pub List: super::LIST_ENTRY,
    pub ReservedSize: super::DWORD,
    pub ReservedMappedBase: super::PVOID,
    pub ReservedMachine: super::USHORT,
    pub ReservedCharacteristics: super::USHORT,
    pub ReservedCheckSum: super::DWORD,
    pub ImageBase: super::DWORD,
    pub SizeOfImage: super::DWORD,
    pub ReservedNumberOfSections: super::DWORD,
    pub ReservedSections: super::PIMAGE_SECTION_HEADER,
    pub ReservedExportedNamesSize: super::DWORD,
    pub ReservedExportedNames: windows_core::PSTR,
    pub ReservedNumberOfFunctionTableEntries: super::DWORD,
    pub ReservedFunctionTableEntries: super::PIMAGE_FUNCTION_ENTRY,
    pub ReservedLowestFunctionStartingAddress: super::DWORD,
    pub ReservedHighestFunctionEndingAddress: super::DWORD,
    pub ReservedNumberOfFpoTableEntries: super::DWORD,
    pub ReservedFpoTableEntries: super::PFPO_DATA,
    pub SizeOfCoffSymbols: super::DWORD,
    pub CoffSymbols: super::PIMAGE_COFF_SYMBOLS_HEADER,
    pub ReservedSizeOfCodeViewSymbols: super::DWORD,
    pub ReservedCodeViewSymbols: super::PVOID,
    pub ImageFilePath: windows_core::PSTR,
    pub ImageFileName: windows_core::PSTR,
    pub ReservedDebugFilePath: windows_core::PSTR,
    pub ReservedTimeDateStamp: super::DWORD,
    pub ReservedRomImage: windows_core::BOOL,
    pub ReservedDebugDirectory: super::PIMAGE_DEBUG_DIRECTORY,
    pub ReservedNumberOfDebugDirectories: super::DWORD,
    pub ReservedOriginalFunctionTableBaseAddress: super::DWORD,
    pub Reserved: [super::DWORD; 2],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IMAGE_DEBUG_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_SEPARATION: i32 = 65536;
pub const INLINE_FRAME_CONTEXT_IGNORE: u32 = 4294967295;
pub const INLINE_FRAME_CONTEXT_INIT: i32 = 0;
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct KDHELP {
    pub Thread: super::DWORD,
    pub ThCallbackStack: super::DWORD,
    pub NextCallback: super::DWORD,
    pub FramePointer: super::DWORD,
    pub KiCallUserMode: super::DWORD,
    pub KeUserCallbackDispatcher: super::DWORD,
    pub SystemRangeStart: super::DWORD,
    pub ThCallbackBStore: super::DWORD,
    pub KiUserExceptionDispatcher: super::DWORD,
    pub StackBase: super::DWORD,
    pub StackLimit: super::DWORD,
    pub Reserved: [super::DWORD; 5],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for KDHELP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct KDHELP64 {
    pub Thread: super::DWORD64,
    pub ThCallbackStack: super::DWORD,
    pub ThCallbackBStore: super::DWORD,
    pub NextCallback: super::DWORD,
    pub FramePointer: super::DWORD,
    pub KiCallUserMode: super::DWORD64,
    pub KeUserCallbackDispatcher: super::DWORD64,
    pub SystemRangeStart: super::DWORD64,
    pub KiUserExceptionDispatcher: super::DWORD64,
    pub StackBase: super::DWORD64,
    pub StackLimit: super::DWORD64,
    pub BuildVersion: super::DWORD,
    pub RetpolineStubFunctionTableSize: super::DWORD,
    pub RetpolineStubFunctionTable: super::DWORD64,
    pub RetpolineStubOffset: super::DWORD,
    pub RetpolineStubSize: super::DWORD,
    pub Reserved0: [super::DWORD64; 2],
}
#[cfg(feature = "shared")]
impl Default for KDHELP64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct LOADED_IMAGE {
    pub ModuleName: windows_core::PSTR,
    pub hFile: super::HANDLE,
    pub MappedAddress: super::PUCHAR,
    pub FileHeader: super::PIMAGE_NT_HEADERS32,
    pub LastRvaSection: super::PIMAGE_SECTION_HEADER,
    pub NumberOfSections: super::ULONG,
    pub Sections: super::PIMAGE_SECTION_HEADER,
    pub Characteristics: super::ULONG,
    pub fSystemImage: super::BOOLEAN,
    pub fDOSImage: super::BOOLEAN,
    pub fReadOnly: super::BOOLEAN,
    pub Version: super::UCHAR,
    pub Links: super::LIST_ENTRY,
    pub SizeOfImage: super::ULONG,
}
#[repr(C, packed(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct LOADED_IMAGE {
    pub ModuleName: windows_core::PSTR,
    pub hFile: super::HANDLE,
    pub MappedAddress: super::PUCHAR,
    pub FileHeader: super::PIMAGE_NT_HEADERS64,
    pub LastRvaSection: super::PIMAGE_SECTION_HEADER,
    pub NumberOfSections: super::ULONG,
    pub Sections: super::PIMAGE_SECTION_HEADER,
    pub Characteristics: super::ULONG,
    pub fSystemImage: super::BOOLEAN,
    pub fDOSImage: super::BOOLEAN,
    pub fReadOnly: super::BOOLEAN,
    pub Version: super::UCHAR,
    pub Links: super::LIST_ENTRY,
    pub SizeOfImage: super::ULONG,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type LPADDRESS = *mut ADDRESS;
#[cfg(feature = "shared")]
pub type LPADDRESS64 = *mut ADDRESS64;
#[cfg(feature = "shared")]
pub type LPAPI_VERSION = *mut API_VERSION;
#[cfg(feature = "shared")]
pub type LPCALL_BACK_USER_INTERRUPT_ROUTINE = Option<unsafe extern "system" fn() -> super::ULONG>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTACKFRAME = *mut STACKFRAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTACKFRAME64 = *mut STACKFRAME64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTACKFRAME_EX = *mut STACKFRAME_EX;
pub const MAX_SYM_NAME: i32 = 2000;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MODLOAD_CVMISC {
    pub oCV: super::DWORD,
    pub cCV: super::size_t,
    pub oMisc: super::DWORD,
    pub cMisc: super::size_t,
    pub dtImage: super::DWORD,
    pub cImage: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MODLOAD_DATA {
    pub ssize: super::DWORD,
    pub ssig: super::DWORD,
    pub data: super::PVOID,
    pub size: super::DWORD,
    pub flags: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MODLOAD_PDBGUID_PDBAGE {
    pub PdbGuid: windows_core::GUID,
    pub PdbAge: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MODULE_TYPE_INFO {
    pub dataLength: super::USHORT,
    pub leaf: super::USHORT,
    pub data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for MODULE_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NONGAMESPARTITIONS: i32 = 1;
pub const NUM_SSRVOPTS: i32 = 32;
pub const NumSymTypes: SYM_TYPE = 9;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct OMAP {
    pub rva: super::ULONG,
    pub rvaTo: super::ULONG,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PDBGHELP_CREATE_USER_DUMP_CALLBACK = Option<unsafe extern "system" fn(datatype: super::DWORD, data: *const super::PVOID, datalength: super::LPDWORD, userdata: super::PVOID) -> windows_core::BOOL>;
pub type PDBGHELP_DATA_REPORT_STRUCT = *mut DBGHELP_DATA_REPORT_STRUCT;
#[cfg(feature = "winnt")]
pub type PENUMDIRTREE_CALLBACK = Option<unsafe extern "system" fn(filepath: windows_core::PCSTR, callerdata: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PENUMDIRTREE_CALLBACKW = Option<unsafe extern "system" fn(filepath: windows_core::PCWSTR, callerdata: super::PVOID) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMLOADED_MODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, modulebase: super::ULONG, modulesize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMLOADED_MODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, modulebase: super::DWORD64, modulesize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMLOADED_MODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_core::PCWSTR, modulebase: super::DWORD64, modulesize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMSOURCEFILETOKENSCALLBACK = Option<unsafe extern "system" fn(token: super::PVOID, size: super::size_t) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFINDFILEINPATHCALLBACK = Option<unsafe extern "system" fn(filename: windows_core::PCSTR, context: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFINDFILEINPATHCALLBACKW = Option<unsafe extern "system" fn(filename: windows_core::PCWSTR, context: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_core::PCSTR, callerdata: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_core::PCWSTR, callerdata: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_core::PCSTR, callerdata: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_core::PCWSTR, callerdata: super::PVOID) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, addrbase: super::DWORD) -> super::PVOID>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = Option<unsafe extern "system" fn(ahprocess: super::HANDLE, addrbase: super::DWORD64) -> super::PVOID>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_MODULE_BASE_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, address: super::DWORD) -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_MODULE_BASE_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, address: super::DWORD64) -> super::DWORD64>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_TARGET_ATTRIBUTE_VALUE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, attribute: super::DWORD, attributedata: super::DWORD64, attributevalue: *mut super::DWORD64) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_CBA_EVENT = *mut IMAGEHLP_CBA_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_CBA_EVENTW = *mut IMAGEHLP_CBA_EVENTW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_CBA_READ_MEMORY = *mut IMAGEHLP_CBA_READ_MEMORY;
pub type PIMAGEHLP_CONTEXT = *mut core::ffi::c_void;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_DEFERRED_SYMBOL_LOAD = *mut IMAGEHLP_DEFERRED_SYMBOL_LOAD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_DEFERRED_SYMBOL_LOAD64 = *mut IMAGEHLP_DEFERRED_SYMBOL_LOAD64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_DEFERRED_SYMBOL_LOADW64 = *mut IMAGEHLP_DEFERRED_SYMBOL_LOADW64;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type PIMAGEHLP_DUPLICATE_SYMBOL = *mut IMAGEHLP_DUPLICATE_SYMBOL;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_DUPLICATE_SYMBOL64 = *mut IMAGEHLP_DUPLICATE_SYMBOL64;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_GET_TYPE_INFO_PARAMS = *mut IMAGEHLP_GET_TYPE_INFO_PARAMS;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_JIT_SYMBOLMAP = *mut IMAGEHLP_JIT_SYMBOLMAP;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_LINE = *mut IMAGEHLP_LINE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_LINE64 = *mut IMAGEHLP_LINE64;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_LINEW = *mut IMAGEHLP_LINEW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_LINEW64 = *mut IMAGEHLP_LINEW64;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type PIMAGEHLP_MODULE = *mut IMAGEHLP_MODULE;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_MODULE64 = *mut IMAGEHLP_MODULE64;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_MODULE64_EX = *mut IMAGEHLP_MODULE64_EX;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_MODULEW = *mut IMAGEHLP_MODULEW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_MODULEW64 = *mut IMAGEHLP_MODULEW64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_MODULEW64_EX = *mut IMAGEHLP_MODULEW64_EX;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_STACK_FRAME = *mut IMAGEHLP_STACK_FRAME;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type PIMAGEHLP_SYMBOL = *mut IMAGEHLP_SYMBOL;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_SYMBOL64 = *mut IMAGEHLP_SYMBOL64;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_SYMBOL64_PACKAGE = *mut IMAGEHLP_SYMBOL64_PACKAGE;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_SYMBOLW = *mut IMAGEHLP_SYMBOLW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_SYMBOLW64 = *mut IMAGEHLP_SYMBOLW64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_SYMBOLW64_PACKAGE = *mut IMAGEHLP_SYMBOLW64_PACKAGE;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGEHLP_SYMBOLW_PACKAGE = *mut IMAGEHLP_SYMBOLW_PACKAGE;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type PIMAGEHLP_SYMBOL_PACKAGE = *mut IMAGEHLP_SYMBOL_PACKAGE;
#[cfg(feature = "shared")]
pub type PIMAGEHLP_SYMBOL_SRC = *mut IMAGEHLP_SYMBOL_SRC;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIMAGE_DEBUG_INFORMATION = *mut IMAGE_DEBUG_INFORMATION;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type PKDHELP = *mut KDHELP;
#[cfg(feature = "shared")]
pub type PKDHELP64 = *mut KDHELP64;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PLOADED_IMAGE = *mut LOADED_IMAGE;
#[cfg(feature = "shared")]
pub type PMODLOAD_CVMISC = *mut MODLOAD_CVMISC;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMODLOAD_DATA = *mut MODLOAD_DATA;
#[cfg(feature = "shared")]
pub type PMODLOAD_PDBGUID_PDBAGE = *mut MODLOAD_PDBGUID_PDBAGE;
#[cfg(feature = "shared")]
pub type PMODULE_TYPE_INFO = *mut MODULE_TYPE_INFO;
#[cfg(feature = "shared")]
pub type POMAP = *mut OMAP;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PREAD_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, lpbaseaddress: super::DWORD, lpbuffer: super::PVOID, nsize: super::DWORD, lpnumberofbytesread: super::PDWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, qwbaseaddress: super::DWORD64, lpbuffer: super::PVOID, nsize: super::DWORD, lpnumberofbytesread: super::LPDWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSOURCEFILE = *mut SOURCEFILE;
#[cfg(feature = "shared")]
pub type PSOURCEFILEW = *mut SOURCEFILEW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSRCCODEINFO = *mut SRCCODEINFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSRCCODEINFOW = *mut SRCCODEINFOW;
pub type PSYMBOLSERVERBYINDEXPROC = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: windows_core::PWSTR) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERCALLBACKPROC = Option<unsafe extern "system" fn(action: super::UINT_PTR, data: super::ULONG64, context: super::ULONG64) -> windows_core::BOOL>;
pub type PSYMBOLSERVERCLOSEPROC = Option<unsafe extern "system" fn() -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERDELTANAME = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::PVOID, param2: super::DWORD, param3: super::DWORD, param4: super::PVOID, param5: super::DWORD, param6: super::DWORD, param7: windows_core::PSTR, param8: super::size_t) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERDELTANAMEW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::PVOID, param2: super::DWORD, param3: super::DWORD, param4: super::PVOID, param5: super::DWORD, param6: super::DWORD, param7: windows_core::PWSTR, param8: super::size_t) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERGETINDEXSTRING = Option<unsafe extern "system" fn(param0: super::PVOID, param1: super::DWORD, param2: super::DWORD, param3: windows_core::PSTR, param4: super::size_t) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERGETINDEXSTRINGW = Option<unsafe extern "system" fn(param0: super::PVOID, param1: super::DWORD, param2: super::DWORD, param3: windows_core::PWSTR, param4: super::size_t) -> windows_core::BOOL>;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PSYMBOLSERVERGETOPTIONDATAPROC = Option<unsafe extern "system" fn(param0: super::UINT_PTR, param1: super::PULONG64) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETOPTIONSPROC = Option<unsafe extern "system" fn() -> super::UINT_PTR>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETSUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PSTR, param4: super::size_t) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETSUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: windows_core::PWSTR, param4: super::size_t) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETVERSION = Option<unsafe extern "system" fn(param0: LPAPI_VERSION) -> windows_core::BOOL>;
pub type PSYMBOLSERVERISSTORE = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERISSTOREW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERMESSAGEPROC = Option<unsafe extern "system" fn(action: super::UINT_PTR, data: super::ULONG64, context: super::ULONG64) -> windows_core::BOOL>;
pub type PSYMBOLSERVEROPENPROC = Option<unsafe extern "system" fn() -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROC = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERPINGPROCWEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERPROC = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_core::PSTR) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_core::PSTR) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_core::PWSTR) -> windows_core::BOOL>;
pub type PSYMBOLSERVERSETHTTPAUTHHEADER = Option<unsafe extern "system" fn(pszauthheader: windows_core::PCWSTR) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSETOPTIONSPROC = Option<unsafe extern "system" fn(param0: super::UINT_PTR, param1: super::ULONG64) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSETOPTIONSWPROC = Option<unsafe extern "system" fn(param0: super::UINT_PTR, param1: super::ULONG64) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERSTOREFILE = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_core::PSTR, param6: super::size_t, param7: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERSTOREFILEW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_core::PWSTR, param6: super::size_t, param7: super::DWORD) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSTORESUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: windows_core::PSTR, param4: super::size_t, param5: super::DWORD) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSTORESUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: windows_core::PWSTR, param4: super::size_t, param5: super::DWORD) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERVERSION = Option<unsafe extern "system" fn() -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERWEXPROC = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_core::PWSTR, param6: PSYMSRV_EXTENDED_OUTPUT_DATA) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOL_FUNCENTRY_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, addrbase: super::DWORD, usercontext: super::PVOID) -> super::PVOID>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOL_FUNCENTRY_CALLBACK64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, addrbase: super::ULONG64, usercontext: super::ULONG64) -> super::PVOID>;
#[cfg(feature = "shared")]
pub type PSYMBOL_INFO = *mut SYMBOL_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOL_INFOW = *mut SYMBOL_INFOW;
#[cfg(feature = "shared")]
pub type PSYMBOL_INFO_PACKAGE = *mut SYMBOL_INFO_PACKAGE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOL_INFO_PACKAGEW = *mut SYMBOL_INFO_PACKAGEW;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOL_REGISTERED_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, actioncode: super::ULONG, callbackdata: super::PVOID, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOL_REGISTERED_CALLBACK64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, actioncode: super::ULONG, callbackdata: super::ULONG64, usercontext: super::ULONG64) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMSRV_EXTENDED_OUTPUT_DATA = *mut SYMSRV_EXTENDED_OUTPUT_DATA;
#[cfg(feature = "shared")]
pub type PSYMSRV_INDEX_INFO = *mut SYMSRV_INDEX_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMSRV_INDEX_INFOW = *mut SYMSRV_INDEX_INFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMERATESYMBOLS_CALLBACK = Option<unsafe extern "system" fn(psyminfo: PSYMBOL_INFO, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMERATESYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(psyminfo: PSYMBOL_INFOW, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMLINES_CALLBACK = Option<unsafe extern "system" fn(lineinfo: PSRCCODEINFO, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMLINES_CALLBACKW = Option<unsafe extern "system" fn(lineinfo: PSRCCODEINFOW, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMMODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, baseofdll: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMMODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_core::PCSTR, baseofdll: super::DWORD64, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMMODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_core::PCWSTR, baseofdll: super::DWORD64, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYM_ENUMPROCESSES_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSOURCEFILES_CALLBACK = Option<unsafe extern "system" fn(psourcefile: PSOURCEFILE, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSOURCEFILES_CALLBACKW = Option<unsafe extern "system" fn(psourcefile: PSOURCEFILEW, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACK = Option<unsafe extern "system" fn(symbolname: windows_core::PCSTR, symboladdress: super::ULONG, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACK64 = Option<unsafe extern "system" fn(symbolname: windows_core::PCSTR, symboladdress: super::DWORD64, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACK64W = Option<unsafe extern "system" fn(symbolname: windows_core::PCWSTR, symboladdress: super::DWORD64, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(symbolname: windows_core::PCWSTR, symboladdress: super::ULONG, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTRANSLATE_ADDRESS_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, hthread: super::HANDLE, lpaddr: LPADDRESS) -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTRANSLATE_ADDRESS_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, hthread: super::HANDLE, lpaddr: LPADDRESS64) -> super::DWORD64>;
pub const SLMFLAG_ALT_INDEX: i32 = 2;
pub const SLMFLAG_NO_SYMBOLS: i32 = 4;
pub const SLMFLAG_VIRTUAL: i32 = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SOURCEFILE {
    pub ModBase: super::DWORD64,
    pub FileName: super::PCHAR,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SOURCEFILEW {
    pub ModBase: super::DWORD64,
    pub FileName: windows_core::PWSTR,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SRCCODEINFO {
    pub SizeOfStruct: super::DWORD,
    pub Key: super::PVOID,
    pub ModBase: super::DWORD64,
    pub Obj: [super::CHAR; 261],
    pub FileName: [super::CHAR; 261],
    pub LineNumber: super::DWORD,
    pub Address: super::DWORD64,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SRCCODEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SRCCODEINFOW {
    pub SizeOfStruct: super::DWORD,
    pub Key: super::PVOID,
    pub ModBase: super::DWORD64,
    pub Obj: [super::WCHAR; 261],
    pub FileName: [super::WCHAR; 261],
    pub LineNumber: super::DWORD,
    pub Address: super::DWORD64,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SRCCODEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSRVACTION_CHECKSUMSTATUS: i32 = 8;
pub const SSRVACTION_EVENT: i32 = 3;
pub const SSRVACTION_EVENTW: i32 = 4;
pub const SSRVACTION_HTTPSTATUS: i32 = 6;
pub const SSRVACTION_QUERYCANCEL: i32 = 2;
pub const SSRVACTION_QUERY_IF_DOWNLOAD_DISALLOWED: i32 = 9;
pub const SSRVACTION_SIZE: i32 = 5;
pub const SSRVACTION_TRACE: i32 = 1;
pub const SSRVACTION_XMLOUTPUT: i32 = 7;
pub const SSRVOPT_CALLBACK: i32 = 1;
pub const SSRVOPT_CALLBACKW: i32 = 65536;
pub const SSRVOPT_DISABLE_PING_HOST: i32 = 67108864;
pub const SSRVOPT_DISABLE_TIMEOUT: i32 = 134217728;
pub const SSRVOPT_DONT_UNCOMPRESS: i32 = 33554432;
pub const SSRVOPT_DOWNSTREAM_STORE: i32 = 8192;
pub const SSRVOPT_DWORD: i32 = 2;
pub const SSRVOPT_DWORDPTR: i32 = 4;
pub const SSRVOPT_ENABLE_COMM_MSG: i32 = 268435456;
pub const SSRVOPT_FAVOR_COMPRESSED: i32 = 2097152;
pub const SSRVOPT_FLAT_DEFAULT_STORE: i32 = 131072;
pub const SSRVOPT_GETPATH: i32 = 64;
pub const SSRVOPT_GUIDPTR: i32 = 8;
pub const SSRVOPT_MAX: u32 = 2147483648;
pub const SSRVOPT_MESSAGE: i32 = 524288;
pub const SSRVOPT_NOCOPY: i32 = 64;
pub const SSRVOPT_OLDGUIDPTR: i32 = 16;
pub const SSRVOPT_OVERWRITE: i32 = 16384;
pub const SSRVOPT_PARAMTYPE: i32 = 256;
pub const SSRVOPT_PARENTWIN: i32 = 128;
pub const SSRVOPT_PROXY: i32 = 4096;
pub const SSRVOPT_PROXYW: i32 = 262144;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const SSRVOPT_RESET: super::ULONG_PTR = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const SSRVOPT_RESET: super::ULONG_PTR = 18446744073709551615u64 as usize;
pub const SSRVOPT_RESETTOU: i32 = 32768;
pub const SSRVOPT_RETRY_APP_HANG: u32 = 2147483648;
pub const SSRVOPT_SECURE: i32 = 512;
pub const SSRVOPT_SERVICE: i32 = 1048576;
pub const SSRVOPT_SETCONTEXT: i32 = 2048;
pub const SSRVOPT_STRING: i32 = 4194304;
pub const SSRVOPT_TRACE: i32 = 1024;
pub const SSRVOPT_UNATTENDED: i32 = 32;
pub const SSRVOPT_URI_FILTER: i32 = 536870912;
pub const SSRVOPT_URI_TIERS: i32 = 1073741824;
pub const SSRVOPT_WINHTTP: i32 = 8388608;
pub const SSRVOPT_WININET: i32 = 16777216;
pub const SSRVURI_ALL: i32 = 255;
pub const SSRVURI_COMPRESSED: i32 = 2;
pub const SSRVURI_FILEPTR: i32 = 4;
pub const SSRVURI_HTTP_COMPRESSED: i32 = 2;
pub const SSRVURI_HTTP_FILEPTR: i32 = 4;
pub const SSRVURI_HTTP_MASK: i32 = 15;
pub const SSRVURI_HTTP_MSFZ0: i32 = 8;
pub const SSRVURI_HTTP_NORMAL: i32 = 1;
pub const SSRVURI_NORMAL: i32 = 1;
pub const SSRVURI_UNC_COMPRESSED: i32 = 32;
pub const SSRVURI_UNC_FILEPTR: i32 = 64;
pub const SSRVURI_UNC_MASK: i32 = 240;
pub const SSRVURI_UNC_MSFZ0: i32 = 128;
pub const SSRVURI_UNC_NORMAL: i32 = 16;
#[repr(C, packed(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STACKFRAME {
    pub AddrPC: ADDRESS,
    pub AddrReturn: ADDRESS,
    pub AddrFrame: ADDRESS,
    pub AddrStack: ADDRESS,
    pub FuncTableEntry: super::PVOID,
    pub Params: [super::DWORD; 4],
    pub Far: windows_core::BOOL,
    pub Virtual: windows_core::BOOL,
    pub Reserved: [super::DWORD; 3],
    pub KdHelp: KDHELP,
    pub AddrBStore: ADDRESS,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STACKFRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STACKFRAME64 {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: super::PVOID,
    pub Params: [super::DWORD64; 4],
    pub Far: windows_core::BOOL,
    pub Virtual: windows_core::BOOL,
    pub Reserved: [super::DWORD64; 3],
    pub KdHelp: KDHELP64,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STACKFRAME64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STACKFRAME_EX {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: super::PVOID,
    pub Params: [super::DWORD64; 4],
    pub Far: windows_core::BOOL,
    pub Virtual: windows_core::BOOL,
    pub Reserved: [super::DWORD64; 3],
    pub KdHelp: KDHELP64,
    pub StackFrameSize: super::DWORD,
    pub InlineFrameContext: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STACKFRAME_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SYMADDSOURCESTREAM = Option<unsafe extern "system" fn(param0: super::HANDLE, param1: super::ULONG64, param2: windows_core::PCSTR, param3: super::PBYTE, param4: super::size_t) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SYMADDSOURCESTREAMA = Option<unsafe extern "system" fn(param0: super::HANDLE, param1: super::ULONG64, param2: windows_core::PCSTR, param3: super::PBYTE, param4: super::size_t) -> windows_core::BOOL>;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SYMBOL_INFO {
    pub SizeOfStruct: super::ULONG,
    pub TypeIndex: super::ULONG,
    pub Reserved: [super::ULONG64; 2],
    pub Index: super::ULONG,
    pub Size: super::ULONG,
    pub ModBase: super::ULONG64,
    pub Flags: super::ULONG,
    pub Value: super::ULONG64,
    pub Address: super::ULONG64,
    pub Register: super::ULONG,
    pub Scope: super::ULONG,
    pub Tag: super::ULONG,
    pub NameLen: super::ULONG,
    pub MaxNameLen: super::ULONG,
    pub Name: [super::CHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for SYMBOL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYMBOL_INFOW {
    pub SizeOfStruct: super::ULONG,
    pub TypeIndex: super::ULONG,
    pub Reserved: [super::ULONG64; 2],
    pub Index: super::ULONG,
    pub Size: super::ULONG,
    pub ModBase: super::ULONG64,
    pub Flags: super::ULONG,
    pub Value: super::ULONG64,
    pub Address: super::ULONG64,
    pub Register: super::ULONG,
    pub Scope: super::ULONG,
    pub Tag: super::ULONG,
    pub NameLen: super::ULONG,
    pub MaxNameLen: super::ULONG,
    pub Name: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYMBOL_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SYMBOL_INFO_PACKAGE {
    pub si: SYMBOL_INFO,
    pub name: [super::CHAR; 2001],
}
#[cfg(feature = "shared")]
impl Default for SYMBOL_INFO_PACKAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYMBOL_INFO_PACKAGEW {
    pub si: SYMBOL_INFOW,
    pub name: [super::WCHAR; 2001],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYMBOL_INFO_PACKAGEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYMENUM_OPTIONS_DEFAULT: i32 = 1;
pub const SYMENUM_OPTIONS_INLINE: i32 = 2;
pub const SYMFLAG_CLR_TOKEN: i32 = 262144;
pub const SYMFLAG_COMPLEX: i32 = 67108864;
pub const SYMFLAG_CONSTANT: i32 = 256;
pub const SYMFLAG_EXPORT: i32 = 512;
pub const SYMFLAG_FIXUP_ARM64X: i32 = 16777216;
pub const SYMFLAG_FORWARDER: i32 = 1024;
pub const SYMFLAG_FRAMEREL: i32 = 32;
pub const SYMFLAG_FUNCTION: i32 = 2048;
pub const SYMFLAG_FUNC_NO_RETURN: i32 = 1048576;
pub const SYMFLAG_GLOBAL: i32 = 33554432;
pub const SYMFLAG_ILREL: i32 = 65536;
pub const SYMFLAG_LOCAL: i32 = 128;
pub const SYMFLAG_METADATA: i32 = 131072;
pub const SYMFLAG_NULL: i32 = 524288;
pub const SYMFLAG_PARAMETER: i32 = 64;
pub const SYMFLAG_PUBLIC_CODE: i32 = 4194304;
pub const SYMFLAG_REGISTER: i32 = 8;
pub const SYMFLAG_REGREL: i32 = 16;
pub const SYMFLAG_REGREL_ALIASINDIR: i32 = 8388608;
pub const SYMFLAG_RESET: u32 = 2147483648;
pub const SYMFLAG_SLOT: i32 = 32768;
pub const SYMFLAG_SYNTHETIC_ZEROBASE: i32 = 2097152;
pub const SYMFLAG_THUNK: i32 = 8192;
pub const SYMFLAG_TLSREL: i32 = 16384;
pub const SYMFLAG_VALUEPRESENT: i32 = 1;
pub const SYMFLAG_VIRTUAL: i32 = 4096;
pub const SYMF_CONSTANT: i32 = 256;
pub const SYMF_EXPORT: i32 = 512;
pub const SYMF_FORWARDER: i32 = 1024;
pub const SYMF_FRAMEREL: i32 = 32;
pub const SYMF_FUNCTION: i32 = 2048;
pub const SYMF_LOCAL: i32 = 128;
pub const SYMF_OMAP_GENERATED: i32 = 1;
pub const SYMF_OMAP_MODIFIED: i32 = 2;
pub const SYMF_PARAMETER: i32 = 64;
pub const SYMF_REGISTER: i32 = 8;
pub const SYMF_REGREL: i32 = 16;
pub const SYMF_THUNK: i32 = 8192;
pub const SYMF_TLSREL: i32 = 16384;
pub const SYMF_VIRTUAL: i32 = 4096;
pub const SYMOPT_ALLOW_ABSOLUTE_SYMBOLS: i32 = 2048;
pub const SYMOPT_ALLOW_ZERO_ADDRESS: i32 = 16777216;
pub const SYMOPT_AUTO_PUBLICS: i32 = 65536;
pub const SYMOPT_CASE_INSENSITIVE: i32 = 1;
pub const SYMOPT_DEBUG: u32 = 2147483648;
pub const SYMOPT_DEFERRED_LOADS: i32 = 4;
pub const SYMOPT_DISABLE_FAST_SYMBOLS: i32 = 268435456;
pub const SYMOPT_DISABLE_SRVSTAR_ON_STARTUP: i32 = 1073741824;
pub const SYMOPT_DISABLE_SYMSRV_AUTODETECT: i32 = 33554432;
pub const SYMOPT_DISABLE_SYMSRV_TIMEOUT: i32 = 536870912;
pub const SYMOPT_EXACT_SYMBOLS: i32 = 1024;
pub const SYMOPT_EX_DISABLEACCESSTIMEUPDATE: IMAGEHLP_EXTENDED_OPTIONS = 0;
pub const SYMOPT_EX_DISALLOW_NETWORK_PATHS: IMAGEHLP_EXTENDED_OPTIONS = 4;
pub const SYMOPT_EX_LASTVALIDDEBUGDIRECTORY: IMAGEHLP_EXTENDED_OPTIONS = 1;
pub const SYMOPT_EX_MAX: IMAGEHLP_EXTENDED_OPTIONS = 5;
pub const SYMOPT_EX_NEVERLOADSYMBOLS: IMAGEHLP_EXTENDED_OPTIONS = 3;
pub const SYMOPT_EX_NOIMPLICITPATTERNSEARCH: IMAGEHLP_EXTENDED_OPTIONS = 2;
pub const SYMOPT_FAIL_CRITICAL_ERRORS: i32 = 512;
pub const SYMOPT_FAVOR_COMPRESSED: i32 = 8388608;
pub const SYMOPT_FLAT_DIRECTORY: i32 = 4194304;
pub const SYMOPT_IGNORE_CVREC: i32 = 128;
pub const SYMOPT_IGNORE_IMAGEDIR: i32 = 2097152;
pub const SYMOPT_IGNORE_NT_SYMPATH: i32 = 4096;
pub const SYMOPT_INCLUDE_32BIT_MODULES: i32 = 8192;
pub const SYMOPT_LOAD_ANYTHING: i32 = 64;
pub const SYMOPT_LOAD_LINES: i32 = 16;
pub const SYMOPT_NO_CPP: i32 = 8;
pub const SYMOPT_NO_IMAGE_SEARCH: i32 = 131072;
pub const SYMOPT_NO_PROMPTS: i32 = 524288;
pub const SYMOPT_NO_PUBLICS: i32 = 32768;
pub const SYMOPT_NO_UNQUALIFIED_LOADS: i32 = 256;
pub const SYMOPT_OMAP_FIND_NEAREST: i32 = 32;
pub const SYMOPT_OVERWRITE: i32 = 1048576;
pub const SYMOPT_PUBLICS_ONLY: i32 = 16384;
pub const SYMOPT_READONLY_CACHE: i32 = 67108864;
pub const SYMOPT_SECURE: i32 = 262144;
pub const SYMOPT_SYMPATH_LAST: i32 = 134217728;
pub const SYMOPT_UNDNAME: i32 = 2;
pub const SYMSEARCH_ALLITEMS: i32 = 8;
pub const SYMSEARCH_GLOBALSONLY: i32 = 4;
pub const SYMSEARCH_MASKOBJS: i32 = 1;
pub const SYMSEARCH_RECURSE: i32 = 2;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYMSRV_EXTENDED_OUTPUT_DATA {
    pub sizeOfStruct: super::DWORD,
    pub version: super::DWORD,
    pub filePtrMsg: [super::WCHAR; 261],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SYMSRV_INDEX_INFO {
    pub sizeofstruct: super::DWORD,
    pub file: [i8; 261],
    pub stripped: windows_core::BOOL,
    pub timestamp: super::DWORD,
    pub size: super::DWORD,
    pub dbgfile: [i8; 261],
    pub pdbfile: [i8; 261],
    pub guid: windows_core::GUID,
    pub sig: super::DWORD,
    pub age: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for SYMSRV_INDEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYMSRV_INDEX_INFOW {
    pub sizeofstruct: super::DWORD,
    pub file: [super::WCHAR; 261],
    pub stripped: windows_core::BOOL,
    pub timestamp: super::DWORD,
    pub size: super::DWORD,
    pub dbgfile: [super::WCHAR; 261],
    pub pdbfile: [super::WCHAR; 261],
    pub guid: windows_core::GUID,
    pub sig: super::DWORD,
    pub age: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYMSRV_INDEX_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYMSRV_VERSION: i32 = 2;
pub const SYMSTOREOPT_ALT_INDEX: i32 = 16;
pub const SYMSTOREOPT_COMPRESS: i32 = 1;
pub const SYMSTOREOPT_OVERWRITE: i32 = 2;
pub const SYMSTOREOPT_PASS_IF_EXISTS: i32 = 64;
pub const SYMSTOREOPT_POINTER: i32 = 8;
pub const SYMSTOREOPT_RETURNINDEX: i32 = 4;
pub const SYMSTOREOPT_UNICODE: i32 = 32;
pub const SYM_INLINE_COMP_DIFFERENT: i32 = 5;
pub const SYM_INLINE_COMP_ERROR: i32 = 0;
pub const SYM_INLINE_COMP_IDENTICAL: i32 = 1;
pub const SYM_INLINE_COMP_STEPIN: i32 = 2;
pub const SYM_INLINE_COMP_STEPOUT: i32 = 3;
pub const SYM_INLINE_COMP_STEPOVER: i32 = 4;
pub const SYM_STKWALK_DEFAULT: i32 = 0;
pub const SYM_STKWALK_FORCE_FRAMEPTR: i32 = 1;
pub const SYM_STKWALK_ZEROEXTEND_PTRS: i32 = 2;
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
pub const TARGET_ATTRIBUTE_PACMASK: i32 = 1;
pub const TARGET_ATTRIBUTE_PACMASK_LIVETARGET: u64 = 18446744073709551615;
pub const TI_FINDCHILDREN: IMAGEHLP_SYMBOL_TYPE_INFO = 7;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct TI_FINDCHILDREN_PARAMS {
    pub Count: super::ULONG,
    pub Start: super::ULONG,
    pub ChildId: [super::ULONG; 1],
}
#[cfg(feature = "shared")]
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
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct TI_GET_DISCRIMINATEDUNION_TAG_RANGES_PARAMS {
    pub Count: super::ULONG,
    pub Start: super::ULONG,
    pub Range: [DISCRIMINATEDUNION_TAG_VALUE; 1],
}
#[cfg(feature = "shared")]
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
pub const UNDNAME_32_BIT_DECODE: i32 = 2048;
pub const UNDNAME_COMPLETE: i32 = 0;
pub const UNDNAME_NAME_ONLY: i32 = 4096;
pub const UNDNAME_NO_ACCESS_SPECIFIERS: i32 = 128;
pub const UNDNAME_NO_ALLOCATION_LANGUAGE: i32 = 16;
pub const UNDNAME_NO_ALLOCATION_MODEL: i32 = 8;
pub const UNDNAME_NO_ARGUMENTS: i32 = 8192;
pub const UNDNAME_NO_CV_THISTYPE: i32 = 64;
pub const UNDNAME_NO_FUNCTION_RETURNS: i32 = 4;
pub const UNDNAME_NO_LEADING_UNDERSCORES: i32 = 1;
pub const UNDNAME_NO_MEMBER_TYPE: i32 = 512;
pub const UNDNAME_NO_MS_KEYWORDS: i32 = 2;
pub const UNDNAME_NO_MS_THISTYPE: i32 = 32;
pub const UNDNAME_NO_RETURN_UDT_MODEL: i32 = 1024;
pub const UNDNAME_NO_SPECIAL_SYMS: i32 = 16384;
pub const UNDNAME_NO_THISTYPE: i32 = 96;
pub const UNDNAME_NO_THROW_SIGNATURES: i32 = 256;
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
