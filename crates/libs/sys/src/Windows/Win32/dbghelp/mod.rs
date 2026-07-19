windows_link::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDump(filename : windows_sys::core::PCSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDumpW(filename : windows_sys::core::PCWSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumDirTree(hprocess : super::HANDLE, rootpath : windows_sys::core::PCSTR, inputpathname : windows_sys::core::PCSTR, outputpathbuffer : windows_sys::core::PSTR, cb : PENUMDIRTREE_CALLBACK, data : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumDirTreeW(hprocess : super::HANDLE, rootpath : windows_sys::core::PCWSTR, inputpathname : windows_sys::core::PCWSTR, outputpathbuffer : windows_sys::core::PWSTR, cb : PENUMDIRTREE_CALLBACKW, data : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModules(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModules64(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesEx(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesExW(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesW64(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindDebugInfoFile(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, debugfilepath : windows_sys::core::PSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindDebugInfoFileEx(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, debugfilepath : windows_sys::core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindDebugInfoFileExW(filename : windows_sys::core::PCWSTR, symbolpath : windows_sys::core::PCWSTR, debugfilepath : windows_sys::core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindExecutableImage(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, imagefilepath : windows_sys::core::PSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindExecutableImageEx(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, imagefilepath : windows_sys::core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindExecutableImageExW(filename : windows_sys::core::PCWSTR, symbolpath : windows_sys::core::PCWSTR, imagefilepath : windows_sys::core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindFileInPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, id : *const core::ffi::c_void, two : u32, three : u32, flags : u32, filepath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindFileInSearchPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, one : u32, two : u32, three : u32, filepath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn GetSymLoadError() -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("dbghelp.dll" "system" fn GetTimestampForLoadedLibrary(module : super::HMODULE) -> u32);
windows_link::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToData(base : *const core::ffi::c_void, mappedasimage : bool, directoryentry : u16, size : *mut u32) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToDataEx(base : *const core::ffi::c_void, mappedasimage : bool, directoryentry : u16, size : *mut u32, foundheader : *mut super::PIMAGE_SECTION_HEADER) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn ImageNtHeader(base : *const core::ffi::c_void) -> super::PIMAGE_NT_HEADERS);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn ImageRvaToSection(ntheaders : super::PIMAGE_NT_HEADERS, base : *const core::ffi::c_void, rva : u32) -> super::PIMAGE_SECTION_HEADER);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn ImageRvaToVa(ntheaders : super::PIMAGE_NT_HEADERS, base : *const core::ffi::c_void, rva : u32, lastrvasection : *const super::PIMAGE_SECTION_HEADER) -> *mut core::ffi::c_void);
windows_link::link!("dbghelp.dll" "system" fn ImagehlpApiVersion() -> LPAPI_VERSION);
windows_link::link!("dbghelp.dll" "system" fn ImagehlpApiVersionEx(appversion : *const API_VERSION) -> LPAPI_VERSION);
windows_link::link!("dbghelp.dll" "system" fn MakeSureDirectoryPathExists(dirpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn RangeMapAddPeImageSections(rmaphandle : *const core::ffi::c_void, imagename : windows_sys::core::PCWSTR, mappedimage : *const core::ffi::c_void, mappingbytes : u32, imagebase : u64, usertag : u64, mappingflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn RangeMapCreate() -> *mut core::ffi::c_void);
windows_link::link!("dbghelp.dll" "system" fn RangeMapFree(rmaphandle : *const core::ffi::c_void));
windows_link::link!("dbghelp.dll" "system" fn RangeMapRead(rmaphandle : *const core::ffi::c_void, offset : u64, buffer : *mut core::ffi::c_void, requestbytes : u32, flags : u32, donebytes : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn RangeMapRemove(rmaphandle : *const core::ffi::c_void, usertag : u64) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn RangeMapWrite(rmaphandle : *const core::ffi::c_void, offset : u64, buffer : *const core::ffi::c_void, requestbytes : u32, flags : u32, donebytes : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn RemoveInvalidModuleList(hprocess : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn ReportSymbolLoadSummary(hprocess : super::HANDLE, ploadmodule : windows_sys::core::PCWSTR, psymboldata : *const DBGHELP_DATA_REPORT_STRUCT) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SearchTreeForFile(rootpath : windows_sys::core::PCSTR, inputpathname : windows_sys::core::PCSTR, outputpathbuffer : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SearchTreeForFileW(rootpath : windows_sys::core::PCWSTR, inputpathname : windows_sys::core::PCWSTR, outputpathbuffer : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SetCheckUserInterruptShared(lpstartaddress : LPCALL_BACK_USER_INTERRUPT_ROUTINE));
windows_link::link!("dbghelp.dll" "system" fn SetSymLoadError(error : u32));
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn StackWalk(machinetype : u32, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : *mut STACKFRAME, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE, translateaddress : PTRANSLATE_ADDRESS_ROUTINE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn StackWalk2(machinetype : u32, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : *mut STACKFRAME_EX, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, gettargetattributevalue : PGET_TARGET_ATTRIBUTE_VALUE64, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn StackWalk64(machinetype : u32, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : *mut STACKFRAME64, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn StackWalkEx(machinetype : u32, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : *mut STACKFRAME_EX, contextrecord : *mut core::ffi::c_void, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymAddSourceStream(hprocess : super::HANDLE, base : u64, streamfile : windows_sys::core::PCSTR, buffer : *const u8, size : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymAddSourceStreamA(hprocess : super::HANDLE, base : u64, streamfile : windows_sys::core::PCSTR, buffer : *const u8, size : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymAddSourceStreamW(hprocess : super::HANDLE, base : u64, filespec : windows_sys::core::PCWSTR, buffer : *const u8, size : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymAddSymbol(hprocess : super::HANDLE, baseofdll : u64, name : windows_sys::core::PCSTR, address : u64, size : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymAddSymbolW(hprocess : super::HANDLE, baseofdll : u64, name : windows_sys::core::PCWSTR, address : u64, size : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymAddrIncludeInlineTrace(hprocess : super::HANDLE, address : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymCleanup(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymCompareInlineTrace(hprocess : super::HANDLE, address1 : u64, inlinecontext1 : u32, retaddress1 : u64, address2 : u64, retaddress2 : u64) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymDeleteSymbol(hprocess : super::HANDLE, baseofdll : u64, name : windows_sys::core::PCSTR, address : u64, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymDeleteSymbolW(hprocess : super::HANDLE, baseofdll : u64, name : windows_sys::core::PCWSTR, address : u64, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumLines(hprocess : super::HANDLE, base : u64, obj : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumLinesW(hprocess : super::HANDLE, base : u64, obj : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumProcesses(enumprocessescallback : PSYM_ENUMPROCESSES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceFileTokens(hprocess : super::HANDLE, base : u64, callback : PENUMSOURCEFILETOKENSCALLBACK) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceFiles(hprocess : super::HANDLE, modbase : u64, mask : windows_sys::core::PCSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceFilesW(hprocess : super::HANDLE, modbase : u64, mask : windows_sys::core::PCWSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceLines(hprocess : super::HANDLE, base : u64, obj : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, line : u32, flags : u32, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceLinesW(hprocess : super::HANDLE, base : u64, obj : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, line : u32, flags : u32, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSym(hprocess : super::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbols(hprocess : super::HANDLE, baseofdll : u64, mask : windows_sys::core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsEx(hprocess : super::HANDLE, baseofdll : u64, mask : windows_sys::core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void, options : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsExW(hprocess : super::HANDLE, baseofdll : u64, mask : windows_sys::core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void, options : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddr(hprocess : super::HANDLE, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddrW(hprocess : super::HANDLE, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsW(hprocess : super::HANDLE, baseofdll : u64, mask : windows_sys::core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypes(hprocess : super::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypesByName(hprocess : super::HANDLE, baseofdll : u64, mask : windows_sys::core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypesByNameW(hprocess : super::HANDLE, baseofdll : u64, mask : windows_sys::core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypesW(hprocess : super::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateModules(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateModules64(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateModulesW64(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACKW64, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbols(hprocess : super::HANDLE, baseofdll : u32, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbols64(hprocess : super::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW(hprocess : super::HANDLE, baseofdll : u32, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW64(hprocess : super::HANDLE, baseofdll : u64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindDebugInfoFile(hprocess : super::HANDLE, filename : windows_sys::core::PCSTR, debugfilepath : windows_sys::core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindDebugInfoFileW(hprocess : super::HANDLE, filename : windows_sys::core::PCWSTR, debugfilepath : windows_sys::core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindExecutableImage(hprocess : super::HANDLE, filename : windows_sys::core::PCSTR, imagefilepath : windows_sys::core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindExecutableImageW(hprocess : super::HANDLE, filename : windows_sys::core::PCWSTR, imagefilepath : windows_sys::core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindFileInPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, id : *const core::ffi::c_void, two : u32, three : u32, flags : u32, foundfile : windows_sys::core::PSTR, callback : PFINDFILEINPATHCALLBACK, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindFileInPathW(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCWSTR, filename : windows_sys::core::PCWSTR, id : *const core::ffi::c_void, two : u32, three : u32, flags : u32, foundfile : windows_sys::core::PWSTR, callback : PFINDFILEINPATHCALLBACKW, context : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromAddr(hprocess : super::HANDLE, address : u64, displacement : *mut u64, symbol : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromAddrW(hprocess : super::HANDLE, address : u64, displacement : *mut u64, symbol : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromIndex(hprocess : super::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromIndexW(hprocess : super::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromInlineContext(hprocess : super::HANDLE, address : u64, inlinecontext : u32, displacement : *mut u64, symbol : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromInlineContextW(hprocess : super::HANDLE, address : u64, inlinecontext : u32, displacement : *mut u64, symbol : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromName(hprocess : super::HANDLE, name : windows_sys::core::PCSTR, symbol : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromNameW(hprocess : super::HANDLE, name : windows_sys::core::PCWSTR, symbol : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromToken(hprocess : super::HANDLE, base : u64, token : u32, symbol : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFromTokenW(hprocess : super::HANDLE, base : u64, token : u32, symbol : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFunctionTableAccess(hprocess : super::HANDLE, addrbase : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64(hprocess : super::HANDLE, addrbase : u64) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64AccessRoutines(hprocess : super::HANDLE, addrbase : u64, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64) -> *mut core::ffi::c_void);
windows_link::link!("dbghelp.dll" "system" fn SymGetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetFileLineOffsets64(hprocess : super::HANDLE, modulename : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, buffer : *mut u64, bufferlines : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetHomeDirectory(r#type : u32, dir : windows_sys::core::PSTR, size : usize) -> super::PCHAR);
windows_link::link!("dbghelp.dll" "system" fn SymGetHomeDirectoryW(r#type : u32, dir : windows_sys::core::PWSTR, size : usize) -> windows_sys::core::PWSTR);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromAddr(hprocess : super::HANDLE, dwaddr : u32, pdwdisplacement : *mut u32, line : *mut IMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromAddr64(hprocess : super::HANDLE, qwaddr : u64, pdwdisplacement : *mut u32, line64 : *mut IMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromAddrW64(hprocess : super::HANDLE, dwaddr : u64, pdwdisplacement : *mut u32, line : *mut IMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContext(hprocess : super::HANDLE, qwaddr : u64, inlinecontext : u32, qwmodulebaseaddress : u64, pdwdisplacement : *mut u32, line64 : *mut IMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContextW(hprocess : super::HANDLE, dwaddr : u64, inlinecontext : u32, qwmodulebaseaddress : u64, pdwdisplacement : *mut u32, line : *mut IMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromName(hprocess : super::HANDLE, modulename : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, dwlinenumber : u32, pldisplacement : *mut i32, line : *mut IMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromName64(hprocess : super::HANDLE, modulename : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, dwlinenumber : u32, pldisplacement : *mut i32, line : *mut IMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromNameW64(hprocess : super::HANDLE, modulename : windows_sys::core::PCWSTR, filename : windows_sys::core::PCWSTR, dwlinenumber : u32, pldisplacement : *mut i32, line : *mut IMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineNext(hprocess : super::HANDLE, line : *mut IMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineNext64(hprocess : super::HANDLE, line : *mut IMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineNextW64(hprocess : super::HANDLE, line : *mut IMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLinePrev(hprocess : super::HANDLE, line : *mut IMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLinePrev64(hprocess : super::HANDLE, line : *mut IMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetLinePrevW64(hprocess : super::HANDLE, line : *mut IMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleBase(hprocess : super::HANDLE, dwaddr : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleBase64(hprocess : super::HANDLE, qwaddr : u64) -> u64);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfo(hprocess : super::HANDLE, dwaddr : u32, moduleinfo : *mut IMAGEHLP_MODULE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfo64(hprocess : super::HANDLE, qwaddr : u64, moduleinfo : *mut IMAGEHLP_MODULE64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfoW(hprocess : super::HANDLE, dwaddr : u32, moduleinfo : *mut IMAGEHLP_MODULEW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfoW64(hprocess : super::HANDLE, qwaddr : u64, moduleinfo : *mut IMAGEHLP_MODULEW64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetOmaps(hprocess : super::HANDLE, baseofdll : u64, omapto : *mut POMAP, comapto : *mut u64, omapfrom : *mut POMAP, comapfrom : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymGetOptions() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("dbghelp.dll" "system" fn SymGetParentWindow(phwnd : *mut super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetScope(hprocess : super::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetScopeW(hprocess : super::HANDLE, baseofdll : u64, index : u32, symbol : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSearchPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PSTR, searchpathlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSearchPathW(hprocess : super::HANDLE, searchpatha : windows_sys::core::PWSTR, searchpathlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFile(hprocess : super::HANDLE, base : u64, params : windows_sys::core::PCSTR, filespec : windows_sys::core::PCSTR, filepath : windows_sys::core::PSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksum(hprocess : super::HANDLE, base : u64, filespec : windows_sys::core::PCSTR, pchecksumtype : *mut u32, pchecksum : *mut u8, checksumsize : u32, pactualbyteswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksumW(hprocess : super::HANDLE, base : u64, filespec : windows_sys::core::PCWSTR, pchecksumtype : *mut u32, pchecksum : *mut u8, checksumsize : u32, pactualbyteswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromToken(hprocess : super::HANDLE, token : *const core::ffi::c_void, params : windows_sys::core::PCSTR, filepath : windows_sys::core::PSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenName(hprocess : super::HANDLE, token : *const core::ffi::c_void, tokenname : windows_sys::core::PCSTR, params : windows_sys::core::PCSTR, filepath : windows_sys::core::PSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenNameW(hprocess : super::HANDLE, token : *const core::ffi::c_void, tokenname : windows_sys::core::PCWSTR, params : windows_sys::core::PCWSTR, filepath : windows_sys::core::PWSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenW(hprocess : super::HANDLE, token : *const core::ffi::c_void, params : windows_sys::core::PCWSTR, filepath : windows_sys::core::PWSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileToken(hprocess : super::HANDLE, base : u64, filespec : windows_sys::core::PCSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenName(hprocess : super::HANDLE, base : u64, filespec : windows_sys::core::PCSTR, tokenname : windows_sys::core::PCSTR, tokenparameters : windows_sys::core::PCSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenNameW(hprocess : super::HANDLE, base : u64, filespec : windows_sys::core::PCWSTR, tokenname : windows_sys::core::PCWSTR, tokenparameters : windows_sys::core::PCWSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenW(hprocess : super::HANDLE, base : u64, filespec : windows_sys::core::PCWSTR, token : *mut *mut core::ffi::c_void, size : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileW(hprocess : super::HANDLE, base : u64, params : windows_sys::core::PCWSTR, filespec : windows_sys::core::PCWSTR, filepath : windows_sys::core::PWSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceVarFromToken(hprocess : super::HANDLE, token : *const core::ffi::c_void, params : windows_sys::core::PCSTR, varname : windows_sys::core::PCSTR, value : windows_sys::core::PSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceVarFromTokenW(hprocess : super::HANDLE, token : *const core::ffi::c_void, params : windows_sys::core::PCWSTR, varname : windows_sys::core::PCWSTR, value : windows_sys::core::PWSTR, size : u32) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromAddr(hprocess : super::HANDLE, dwaddr : u32, pdwdisplacement : *mut u32, symbol : *mut IMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromAddr64(hprocess : super::HANDLE, qwaddr : u64, pdwdisplacement : *mut u64, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromName(hprocess : super::HANDLE, name : windows_sys::core::PCSTR, symbol : *mut IMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromName64(hprocess : super::HANDLE, name : windows_sys::core::PCSTR, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymNext(hprocess : super::HANDLE, symbol : *mut IMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymNext64(hprocess : super::HANDLE, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymPrev(hprocess : super::HANDLE, symbol : *mut IMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymPrev64(hprocess : super::HANDLE, symbol : *mut IMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymbolFile(hprocess : super::HANDLE, sympath : windows_sys::core::PCSTR, imagefile : windows_sys::core::PCSTR, r#type : u32, symbolfile : windows_sys::core::PSTR, csymbolfile : usize, dbgfile : windows_sys::core::PSTR, cdbgfile : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymbolFileW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, imagefile : windows_sys::core::PCWSTR, r#type : u32, symbolfile : windows_sys::core::PWSTR, csymbolfile : usize, dbgfile : windows_sys::core::PWSTR, cdbgfile : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeFromName(hprocess : super::HANDLE, baseofdll : u64, name : windows_sys::core::PCSTR, symbol : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeFromNameW(hprocess : super::HANDLE, baseofdll : u64, name : windows_sys::core::PCWSTR, symbol : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeInfo(hprocess : super::HANDLE, modbase : u64, typeid : u32, gettype : IMAGEHLP_SYMBOL_TYPE_INFO, pinfo : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeInfoEx(hprocess : super::HANDLE, modbase : u64, params : *mut IMAGEHLP_GET_TYPE_INFO_PARAMS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymGetUnwindInfo(hprocess : super::HANDLE, address : u64, buffer : *mut core::ffi::c_void, size : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymInitialize(hprocess : super::HANDLE, usersearchpath : windows_sys::core::PCSTR, finvadeprocess : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymInitializeW(hprocess : super::HANDLE, usersearchpath : windows_sys::core::PCWSTR, finvadeprocess : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModule(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCSTR, modulename : windows_sys::core::PCSTR, baseofdll : u32, sizeofdll : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModule64(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCSTR, modulename : windows_sys::core::PCSTR, baseofdll : u64, sizeofdll : u32) -> u64);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModuleEx(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCSTR, modulename : windows_sys::core::PCSTR, baseofdll : u64, dllsize : u32, data : *const MODLOAD_DATA, flags : u32) -> u64);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModuleExW(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCWSTR, modulename : windows_sys::core::PCWSTR, baseofdll : u64, dllsize : u32, data : *const MODLOAD_DATA, flags : u32) -> u64);
windows_link::link!("dbghelp.dll" "system" fn SymMatchFileName(filename : windows_sys::core::PCSTR, r#match : windows_sys::core::PCSTR, filenamestop : *mut windows_sys::core::PSTR, matchstop : *mut windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchFileNameW(filename : windows_sys::core::PCWSTR, r#match : windows_sys::core::PCWSTR, filenamestop : *mut windows_sys::core::PWSTR, matchstop : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchString(string : windows_sys::core::PCSTR, expression : windows_sys::core::PCSTR, fcase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchStringA(string : windows_sys::core::PCSTR, expression : windows_sys::core::PCSTR, fcase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchStringW(string : windows_sys::core::PCWSTR, expression : windows_sys::core::PCWSTR, fcase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymNext(hprocess : super::HANDLE, si : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymNextW(hprocess : super::HANDLE, siw : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymPrev(hprocess : super::HANDLE, si : *mut SYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymPrevW(hprocess : super::HANDLE, siw : *mut SYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymQueryInlineTrace(hprocess : super::HANDLE, startaddress : u64, startcontext : u32, startretaddress : u64, curaddress : u64, curcontext : *mut u32, curframeindex : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymRefreshModuleList(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterCallback64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterCallbackW64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : u64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK, usercontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK64, usercontext : u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSearch(hprocess : super::HANDLE, baseofdll : u64, index : u32, symtag : u32, mask : windows_sys::core::PCSTR, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : *const core::ffi::c_void, options : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSearchW(hprocess : super::HANDLE, baseofdll : u64, index : u32, symtag : u32, mask : windows_sys::core::PCWSTR, address : u64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : *const core::ffi::c_void, options : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetContext(hprocess : super::HANDLE, stackframe : *const IMAGEHLP_STACK_FRAME, context : PIMAGEHLP_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymSetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS, value : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetHomeDirectory(hprocess : super::HANDLE, dir : windows_sys::core::PCSTR) -> super::PCHAR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetHomeDirectoryW(hprocess : super::HANDLE, dir : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("dbghelp.dll" "system" fn SymSetOptions(symoptions : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("dbghelp.dll" "system" fn SymSetParentWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetScopeFromAddr(hprocess : super::HANDLE, address : u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetScopeFromIndex(hprocess : super::HANDLE, baseofdll : u64, index : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetScopeFromInlineContext(hprocess : super::HANDLE, address : u64, inlinecontext : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetSearchPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetSearchPathW(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvDeltaName(hprocess : super::HANDLE, sympath : windows_sys::core::PCSTR, r#type : windows_sys::core::PCSTR, file1 : windows_sys::core::PCSTR, file2 : windows_sys::core::PCSTR) -> windows_sys::core::PCSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvDeltaNameW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, r#type : windows_sys::core::PCWSTR, file1 : windows_sys::core::PCWSTR, file2 : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfo(file : windows_sys::core::PCSTR, info : *mut SYMSRV_INDEX_INFO, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfoW(file : windows_sys::core::PCWSTR, info : *mut SYMSRV_INDEX_INFOW, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexString(hprocess : super::HANDLE, srvpath : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, index : windows_sys::core::PSTR, size : usize, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexStringW(hprocess : super::HANDLE, srvpath : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, index : windows_sys::core::PWSTR, size : usize, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexes(file : windows_sys::core::PCSTR, id : *mut windows_sys::core::GUID, val1 : *mut u32, val2 : *mut u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexesW(file : windows_sys::core::PCWSTR, id : *mut windows_sys::core::GUID, val1 : *mut u32, val2 : *mut u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetSupplement(hprocess : super::HANDLE, sympath : windows_sys::core::PCSTR, node : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR) -> windows_sys::core::PCSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetSupplementW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, node : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvIsStore(hprocess : super::HANDLE, path : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvIsStoreW(hprocess : super::HANDLE, path : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreFile(hprocess : super::HANDLE, srvpath : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, flags : u32) -> windows_sys::core::PCSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreFileW(hprocess : super::HANDLE, srvpath : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::PCWSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreSupplement(hprocess : super::HANDLE, srvpath : windows_sys::core::PCSTR, node : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, flags : u32) -> windows_sys::core::PCSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreSupplementW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, node : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::PCWSTR);
#[cfg(target_arch = "x86")]
windows_link::link!("dbghelp.dll" "system" fn SymUnDName(sym : *const IMAGEHLP_SYMBOL, undecname : windows_sys::core::PSTR, undecnamelength : u32) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymUnDName64(sym : *const IMAGEHLP_SYMBOL64, undecname : windows_sys::core::PSTR, undecnamelength : u32) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymUnloadModule(hprocess : super::HANDLE, baseofdll : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymUnloadModule64(hprocess : super::HANDLE, baseofdll : u64) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn UnDecorateSymbolName(name : windows_sys::core::PCSTR, outputstring : windows_sys::core::PSTR, maxstringlength : u32, flags : u32) -> u32);
windows_link::link!("dbghelp.dll" "system" fn UnDecorateSymbolNameW(name : windows_sys::core::PCWSTR, outputstring : windows_sys::core::PWSTR, maxstringlength : u32, flags : u32) -> u32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct ADDRESS {
    pub Offset: u32,
    pub Segment: u16,
    pub Mode: ADDRESS_MODE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ADDRESS64 {
    pub Offset: u64,
    pub Segment: u16,
    pub Mode: ADDRESS_MODE,
}
pub type ADDRESS_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct DBGHELP_DATA_REPORT_STRUCT {
    pub pBinPathNonExist: windows_sys::core::PCWSTR,
    pub pSymbolPathNonExist: windows_sys::core::PCWSTR,
}
impl Default for DBGHELP_DATA_REPORT_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DBHHEADER_CVMISC: u32 = 2;
pub const DBHHEADER_DEBUGDIRS: u32 = 1;
pub const DBHHEADER_PDBGUID: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_CBA_EVENT {
    pub severity: u32,
    pub code: u32,
    pub desc: super::PCHAR,
    pub object: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for IMAGEHLP_CBA_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_CBA_EVENTW {
    pub severity: u32,
    pub code: u32,
    pub desc: windows_sys::core::PCWSTR,
    pub object: *mut core::ffi::c_void,
}
impl Default for IMAGEHLP_CBA_EVENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [i8; 260],
    pub Reparse: bool,
    pub hFile: super::HANDLE,
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [i8; 260],
    pub Reparse: bool,
    pub hFile: super::HANDLE,
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [u16; 261],
    pub Reparse: bool,
    pub hFile: super::HANDLE,
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL {
    pub SizeOfStruct: u32,
    pub NumberOfDups: u32,
    pub Symbol: PIMAGEHLP_SYMBOL,
    pub SelectedSymbol: u32,
}
#[cfg(target_arch = "x86")]
impl Default for IMAGEHLP_DUPLICATE_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL64 {
    pub SizeOfStruct: u32,
    pub NumberOfDups: u32,
    pub Symbol: PIMAGEHLP_SYMBOL64,
    pub SelectedSymbol: u32,
}
impl Default for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IMAGEHLP_EXTENDED_OPTIONS = i32;
pub const IMAGEHLP_GET_TYPE_INFO_CHILDREN: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_GET_TYPE_INFO_PARAMS {
    pub SizeOfStruct: u32,
    pub Flags: u32,
    pub NumIds: u32,
    pub TypeIds: super::PULONG,
    pub TagFilter: u64,
    pub NumReqs: u32,
    pub ReqKinds: *mut IMAGEHLP_SYMBOL_TYPE_INFO,
    pub ReqOffsets: super::PULONG_PTR,
    pub ReqSizes: super::PULONG,
    pub ReqStride: usize,
    pub BufferSize: usize,
    pub Buffer: *mut core::ffi::c_void,
    pub EntriesMatched: u32,
    pub EntriesFilled: u32,
    pub TagsFound: u64,
    pub AllReqsValid: u64,
    pub NumReqsValid: u32,
    pub ReqsValid: super::PULONG64,
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
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_JIT_SYMBOLMAP {
    pub SizeOfStruct: u32,
    pub Address: u64,
    pub BaseOfImage: u64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct IMAGEHLP_LINE {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: super::PCHAR,
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_LINE64 {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: super::PCHAR,
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_LINEW {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: super::PCHAR,
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_LINEW64 {
    pub SizeOfStruct: u32,
    pub Key: *mut core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: windows_sys::core::PWSTR,
    pub Address: u64,
}
impl Default for IMAGEHLP_LINEW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
    pub PdbSig70: windows_sys::core::GUID,
    pub PdbAge: u32,
    pub PdbUnmatched: windows_sys::core::BOOL,
    pub DbgUnmatched: windows_sys::core::BOOL,
    pub LineNumbers: windows_sys::core::BOOL,
    pub GlobalSymbols: windows_sys::core::BOOL,
    pub TypeInfo: windows_sys::core::BOOL,
    pub SourceIndexed: windows_sys::core::BOOL,
    pub Publics: windows_sys::core::BOOL,
    pub MachineType: u32,
    pub Reserved: u32,
}
impl Default for IMAGEHLP_MODULE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGEHLP_MODULE64_EX {
    pub Module: IMAGEHLP_MODULE64,
    pub RegionFlags: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
    pub PdbSig70: windows_sys::core::GUID,
    pub PdbAge: u32,
    pub PdbUnmatched: windows_sys::core::BOOL,
    pub DbgUnmatched: windows_sys::core::BOOL,
    pub LineNumbers: windows_sys::core::BOOL,
    pub GlobalSymbols: windows_sys::core::BOOL,
    pub TypeInfo: windows_sys::core::BOOL,
    pub SourceIndexed: windows_sys::core::BOOL,
    pub Publics: windows_sys::core::BOOL,
    pub MachineType: u32,
    pub Reserved: u32,
}
impl Default for IMAGEHLP_MODULEW64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct IMAGEHLP_STACK_FRAME {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
    pub BackingStoreOffset: u64,
    pub FuncTableEntry: u64,
    pub Params: [u64; 4],
    pub Reserved: [u64; 5],
    pub Virtual: windows_sys::core::BOOL,
    pub Reserved2: u32,
}
impl Default for IMAGEHLP_STACK_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct IMAGE_DEBUG_INFORMATION {
    pub List: super::LIST_ENTRY,
    pub ReservedSize: u32,
    pub ReservedMappedBase: *mut core::ffi::c_void,
    pub ReservedMachine: u16,
    pub ReservedCharacteristics: u16,
    pub ReservedCheckSum: u32,
    pub ImageBase: u32,
    pub SizeOfImage: u32,
    pub ReservedNumberOfSections: u32,
    pub ReservedSections: super::PIMAGE_SECTION_HEADER,
    pub ReservedExportedNamesSize: u32,
    pub ReservedExportedNames: windows_sys::core::PSTR,
    pub ReservedNumberOfFunctionTableEntries: u32,
    pub ReservedFunctionTableEntries: super::PIMAGE_FUNCTION_ENTRY,
    pub ReservedLowestFunctionStartingAddress: u32,
    pub ReservedHighestFunctionEndingAddress: u32,
    pub ReservedNumberOfFpoTableEntries: u32,
    pub ReservedFpoTableEntries: super::PFPO_DATA,
    pub SizeOfCoffSymbols: u32,
    pub CoffSymbols: super::PIMAGE_COFF_SYMBOLS_HEADER,
    pub ReservedSizeOfCodeViewSymbols: u32,
    pub ReservedCodeViewSymbols: *mut core::ffi::c_void,
    pub ImageFilePath: windows_sys::core::PSTR,
    pub ImageFileName: windows_sys::core::PSTR,
    pub ReservedDebugFilePath: windows_sys::core::PSTR,
    pub ReservedTimeDateStamp: u32,
    pub ReservedRomImage: windows_sys::core::BOOL,
    pub ReservedDebugDirectory: super::PIMAGE_DEBUG_DIRECTORY,
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct LOADED_IMAGE {
    pub ModuleName: windows_sys::core::PSTR,
    pub hFile: super::HANDLE,
    pub MappedAddress: super::PUCHAR,
    pub FileHeader: super::PIMAGE_NT_HEADERS32,
    pub LastRvaSection: super::PIMAGE_SECTION_HEADER,
    pub NumberOfSections: u32,
    pub Sections: super::PIMAGE_SECTION_HEADER,
    pub Characteristics: u32,
    pub fSystemImage: bool,
    pub fDOSImage: bool,
    pub fReadOnly: bool,
    pub Version: u8,
    pub Links: super::LIST_ENTRY,
    pub SizeOfImage: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for LOADED_IMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LOADED_IMAGE {
    pub ModuleName: windows_sys::core::PSTR,
    pub hFile: super::HANDLE,
    pub MappedAddress: super::PUCHAR,
    pub FileHeader: super::PIMAGE_NT_HEADERS64,
    pub LastRvaSection: super::PIMAGE_SECTION_HEADER,
    pub NumberOfSections: u32,
    pub Sections: super::PIMAGE_SECTION_HEADER,
    pub Characteristics: u32,
    pub fSystemImage: bool,
    pub fDOSImage: bool,
    pub fReadOnly: bool,
    pub Version: u8,
    pub Links: super::LIST_ENTRY,
    pub SizeOfImage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for LOADED_IMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy, Default)]
pub struct MODLOAD_CVMISC {
    pub oCV: u32,
    pub cCV: usize,
    pub oMisc: u32,
    pub cMisc: usize,
    pub dtImage: u32,
    pub cImage: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct MODLOAD_PDBGUID_PDBAGE {
    pub PdbGuid: windows_sys::core::GUID,
    pub PdbAge: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct OMAP {
    pub rva: u32,
    pub rvaTo: u32,
}
pub type PDBGHELP_CREATE_USER_DUMP_CALLBACK = Option<unsafe extern "system" fn(datatype: u32, data: *const *const core::ffi::c_void, datalength: *mut u32, userdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PDBGHELP_DATA_REPORT_STRUCT = *mut DBGHELP_DATA_REPORT_STRUCT;
pub type PENUMDIRTREE_CALLBACK = Option<unsafe extern "system" fn(filepath: windows_sys::core::PCSTR, callerdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PENUMDIRTREE_CALLBACKW = Option<unsafe extern "system" fn(filepath: windows_sys::core::PCWSTR, callerdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PENUMLOADED_MODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, modulebase: u32, modulesize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PENUMLOADED_MODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, modulebase: u64, modulesize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PENUMLOADED_MODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCWSTR, modulebase: u64, modulesize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PENUMSOURCEFILETOKENSCALLBACK = Option<unsafe extern "system" fn(token: *const core::ffi::c_void, size: usize) -> windows_sys::core::BOOL>;
pub type PFINDFILEINPATHCALLBACK = Option<unsafe extern "system" fn(filename: windows_sys::core::PCSTR, context: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFINDFILEINPATHCALLBACKW = Option<unsafe extern "system" fn(filename: windows_sys::core::PCWSTR, context: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCSTR, callerdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCWSTR, callerdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCSTR, callerdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCWSTR, callerdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, addrbase: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = Option<unsafe extern "system" fn(ahprocess: super::HANDLE, addrbase: u64) -> *mut core::ffi::c_void>;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PGET_MODULE_BASE_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, address: u32) -> u32>;
#[cfg(feature = "winnt")]
pub type PGET_MODULE_BASE_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, address: u64) -> u64>;
#[cfg(feature = "winnt")]
pub type PGET_TARGET_ATTRIBUTE_VALUE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, attribute: u32, attributedata: u64, attributevalue: *mut u64) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PIMAGEHLP_CBA_EVENT = *mut IMAGEHLP_CBA_EVENT;
pub type PIMAGEHLP_CBA_EVENTW = *mut IMAGEHLP_CBA_EVENTW;
pub type PIMAGEHLP_CBA_READ_MEMORY = *mut IMAGEHLP_CBA_READ_MEMORY;
pub type PIMAGEHLP_CONTEXT = *mut core::ffi::c_void;
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
pub type PREAD_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, lpbaseaddress: u32, lpbuffer: *mut core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, qwbaseaddress: u64, lpbuffer: *mut core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSOURCEFILE = *mut SOURCEFILE;
pub type PSOURCEFILEW = *mut SOURCEFILEW;
pub type PSRCCODEINFO = *mut SRCCODEINFO;
pub type PSRCCODEINFOW = *mut SRCCODEINFOW;
pub type PSYMBOLSERVERBYINDEXPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERCALLBACKPROC = Option<unsafe extern "system" fn(action: usize, data: u64, context: u64) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERCLOSEPROC = Option<unsafe extern "system" fn() -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERDELTANAME = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: *mut core::ffi::c_void, param2: u32, param3: u32, param4: *mut core::ffi::c_void, param5: u32, param6: u32, param7: windows_sys::core::PCSTR, param8: usize) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERDELTANAMEW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: *mut core::ffi::c_void, param2: u32, param3: u32, param4: *mut core::ffi::c_void, param5: u32, param6: u32, param7: windows_sys::core::PCWSTR, param8: usize) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERGETINDEXSTRING = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32, param2: u32, param3: windows_sys::core::PCSTR, param4: usize) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERGETINDEXSTRINGW = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32, param2: u32, param3: windows_sys::core::PCWSTR, param4: usize) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERGETOPTIONDATAPROC = Option<unsafe extern "system" fn(param0: usize, param1: *mut u64) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERGETOPTIONSPROC = Option<unsafe extern "system" fn() -> usize>;
pub type PSYMBOLSERVERGETSUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PCSTR, param4: usize) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERGETSUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PCWSTR, param4: usize) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERGETVERSION = Option<unsafe extern "system" fn(param0: *mut API_VERSION) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERISSTORE = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERISSTOREW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERMESSAGEPROC = Option<unsafe extern "system" fn(action: usize, data: u64, context: u64) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVEROPENPROC = Option<unsafe extern "system" fn() -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROCWEX = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSETHTTPAUTHHEADER = Option<unsafe extern "system" fn(pszauthheader: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSETOPTIONSPROC = Option<unsafe extern "system" fn(param0: usize, param1: u64) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSETOPTIONSWPROC = Option<unsafe extern "system" fn(param0: usize, param1: u64) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSTOREFILE = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_sys::core::PCSTR, param6: usize, param7: u32) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSTOREFILEW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_sys::core::PCWSTR, param6: usize, param7: u32) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSTORESUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PCSTR, param4: usize, param5: u32) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSTORESUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PCWSTR, param4: usize, param5: u32) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERVERSION = Option<unsafe extern "system" fn() -> u32>;
pub type PSYMBOLSERVERWEXPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: *mut core::ffi::c_void, param3: u32, param4: u32, param5: windows_sys::core::PCWSTR, param6: *mut SYMSRV_EXTENDED_OUTPUT_DATA) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYMBOL_FUNCENTRY_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, addrbase: u32, usercontext: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type PSYMBOL_FUNCENTRY_CALLBACK64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, addrbase: u64, usercontext: u64) -> *mut core::ffi::c_void>;
pub type PSYMBOL_INFO = *mut SYMBOL_INFO;
pub type PSYMBOL_INFOW = *mut SYMBOL_INFOW;
pub type PSYMBOL_INFO_PACKAGE = *mut SYMBOL_INFO_PACKAGE;
pub type PSYMBOL_INFO_PACKAGEW = *mut SYMBOL_INFO_PACKAGEW;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PSYMBOL_REGISTERED_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, actioncode: u32, callbackdata: *const core::ffi::c_void, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYMBOL_REGISTERED_CALLBACK64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, actioncode: u32, callbackdata: u64, usercontext: u64) -> windows_sys::core::BOOL>;
pub type PSYMSRV_EXTENDED_OUTPUT_DATA = *mut SYMSRV_EXTENDED_OUTPUT_DATA;
pub type PSYMSRV_INDEX_INFO = *mut SYMSRV_INDEX_INFO;
pub type PSYMSRV_INDEX_INFOW = *mut SYMSRV_INDEX_INFOW;
pub type PSYM_ENUMERATESYMBOLS_CALLBACK = Option<unsafe extern "system" fn(psyminfo: *const SYMBOL_INFO, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMERATESYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(psyminfo: *const SYMBOL_INFOW, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMLINES_CALLBACK = Option<unsafe extern "system" fn(lineinfo: *const SRCCODEINFO, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMLINES_CALLBACKW = Option<unsafe extern "system" fn(lineinfo: *const SRCCODEINFOW, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PSYM_ENUMMODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, baseofdll: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMMODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, baseofdll: u64, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMMODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCWSTR, baseofdll: u64, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYM_ENUMPROCESSES_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYM_ENUMSOURCEFILES_CALLBACK = Option<unsafe extern "system" fn(psourcefile: *const SOURCEFILE, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMSOURCEFILES_CALLBACKW = Option<unsafe extern "system" fn(psourcefile: *const SOURCEFILEW, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PSYM_ENUMSYMBOLS_CALLBACK = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCSTR, symboladdress: u32, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMSYMBOLS_CALLBACK64 = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCSTR, symboladdress: u64, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PSYM_ENUMSYMBOLS_CALLBACK64W = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCWSTR, symboladdress: u64, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
pub type PSYM_ENUMSYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCWSTR, symboladdress: u32, symbolsize: u32, usercontext: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type PTRANSLATE_ADDRESS_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, hthread: super::HANDLE, lpaddr: *mut ADDRESS) -> u32>;
#[cfg(feature = "winnt")]
pub type PTRANSLATE_ADDRESS_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, hthread: super::HANDLE, lpaddr: *const ADDRESS64) -> u64>;
pub const SLMFLAG_ALT_INDEX: u32 = 2;
pub const SLMFLAG_NO_SYMBOLS: u32 = 4;
pub const SLMFLAG_VIRTUAL: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SOURCEFILE {
    pub ModBase: u64,
    pub FileName: super::PCHAR,
}
#[cfg(feature = "winnt")]
impl Default for SOURCEFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOURCEFILEW {
    pub ModBase: u64,
    pub FileName: windows_sys::core::PWSTR,
}
impl Default for SOURCEFILEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct STACKFRAME {
    pub AddrPC: ADDRESS,
    pub AddrReturn: ADDRESS,
    pub AddrFrame: ADDRESS,
    pub AddrStack: ADDRESS,
    pub FuncTableEntry: *mut core::ffi::c_void,
    pub Params: [u32; 4],
    pub Far: windows_sys::core::BOOL,
    pub Virtual: windows_sys::core::BOOL,
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
#[derive(Clone, Copy)]
pub struct STACKFRAME64 {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: *mut core::ffi::c_void,
    pub Params: [u64; 4],
    pub Far: windows_sys::core::BOOL,
    pub Virtual: windows_sys::core::BOOL,
    pub Reserved: [u64; 3],
    pub KdHelp: KDHELP64,
}
impl Default for STACKFRAME64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STACKFRAME_EX {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: *mut core::ffi::c_void,
    pub Params: [u64; 4],
    pub Far: windows_sys::core::BOOL,
    pub Virtual: windows_sys::core::BOOL,
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
pub type SYMADDSOURCESTREAM = Option<unsafe extern "system" fn(param0: super::HANDLE, param1: u64, param2: windows_sys::core::PCSTR, param3: *mut u8, param4: usize) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type SYMADDSOURCESTREAMA = Option<unsafe extern "system" fn(param0: super::HANDLE, param1: u64, param2: windows_sys::core::PCSTR, param3: *mut u8, param4: usize) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct SYMSRV_INDEX_INFO {
    pub sizeofstruct: u32,
    pub file: [i8; 261],
    pub stripped: windows_sys::core::BOOL,
    pub timestamp: u32,
    pub size: u32,
    pub dbgfile: [i8; 261],
    pub pdbfile: [i8; 261],
    pub guid: windows_sys::core::GUID,
    pub sig: u32,
    pub age: u32,
}
impl Default for SYMSRV_INDEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYMSRV_INDEX_INFOW {
    pub sizeofstruct: u32,
    pub file: [u16; 261],
    pub stripped: windows_sys::core::BOOL,
    pub timestamp: u32,
    pub size: u32,
    pub dbgfile: [u16; 261],
    pub pdbfile: [u16; 261],
    pub guid: windows_sys::core::GUID,
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
