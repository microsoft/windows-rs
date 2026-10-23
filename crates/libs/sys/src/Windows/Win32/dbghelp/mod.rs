#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDump(filename : windows_sys::core::PCSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn DbgHelpCreateUserDumpW(filename : windows_sys::core::PCWSTR, callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumDirTree(hprocess : super::HANDLE, rootpath : windows_sys::core::PCSTR, inputpathname : windows_sys::core::PCSTR, outputpathbuffer : windows_sys::core::PSTR, cb : PENUMDIRTREE_CALLBACK, data : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn EnumDirTreeW(hprocess : super::HANDLE, rootpath : windows_sys::core::PCWSTR, inputpathname : windows_sys::core::PCWSTR, outputpathbuffer : windows_sys::core::PWSTR, cb : PENUMDIRTREE_CALLBACKW, data : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModules(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModules64(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesEx(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesExW(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesW64(hprocess : super::HANDLE, enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindDebugInfoFile(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, debugfilepath : windows_sys::core::PSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindDebugInfoFileEx(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, debugfilepath : windows_sys::core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindDebugInfoFileExW(filename : windows_sys::core::PCWSTR, symbolpath : windows_sys::core::PCWSTR, debugfilepath : windows_sys::core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindExecutableImage(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, imagefilepath : windows_sys::core::PSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindExecutableImageEx(filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, imagefilepath : windows_sys::core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn FindExecutableImageExW(filename : windows_sys::core::PCWSTR, symbolpath : windows_sys::core::PCWSTR, imagefilepath : windows_sys::core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn FindFileInPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, id : super::PVOID, two : super::DWORD, three : super::DWORD, flags : super::DWORD, filepath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn FindFileInSearchPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, one : super::DWORD, two : super::DWORD, three : super::DWORD, filepath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn GetSymLoadError() -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dbghelp.dll" "system" fn GetTimestampForLoadedLibrary(module : super::HMODULE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToData(base : super::PVOID, mappedasimage : super::BOOLEAN, directoryentry : super::USHORT, size : super::PULONG) -> super::PVOID);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn ImageDirectoryEntryToDataEx(base : super::PVOID, mappedasimage : super::BOOLEAN, directoryentry : super::USHORT, size : super::PULONG, foundheader : *mut super::PIMAGE_SECTION_HEADER) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn ImageNtHeader(base : super::PVOID) -> super::PIMAGE_NT_HEADERS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn ImageRvaToSection(ntheaders : super::PIMAGE_NT_HEADERS, base : super::PVOID, rva : super::ULONG) -> super::PIMAGE_SECTION_HEADER);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn ImageRvaToVa(ntheaders : super::PIMAGE_NT_HEADERS, base : super::PVOID, rva : super::ULONG, lastrvasection : *const super::PIMAGE_SECTION_HEADER) -> super::PVOID);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn ImagehlpApiVersion() -> LPAPI_VERSION);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn ImagehlpApiVersionEx(appversion : LPAPI_VERSION) -> LPAPI_VERSION);
windows_link::link!("dbghelp.dll" "system" fn MakeSureDirectoryPathExists(dirpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn MapDebugInformation(filehandle : super::HANDLE, filename : windows_sys::core::PCSTR, symbolpath : windows_sys::core::PCSTR, imagebase : super::ULONG) -> PIMAGE_DEBUG_INFORMATION);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn RangeMapAddPeImageSections(rmaphandle : super::PVOID, imagename : windows_sys::core::PCWSTR, mappedimage : super::PVOID, mappingbytes : super::DWORD, imagebase : super::DWORD64, usertag : super::DWORD64, mappingflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn RangeMapCreate() -> super::PVOID);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn RangeMapFree(rmaphandle : super::PVOID));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn RangeMapRead(rmaphandle : super::PVOID, offset : super::DWORD64, buffer : super::PVOID, requestbytes : super::DWORD, flags : super::DWORD, donebytes : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn RangeMapRemove(rmaphandle : super::PVOID, usertag : super::DWORD64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn RangeMapWrite(rmaphandle : super::PVOID, offset : super::DWORD64, buffer : super::PVOID, requestbytes : super::DWORD, flags : super::DWORD, donebytes : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn RemoveInvalidModuleList(hprocess : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn ReportSymbolLoadSummary(hprocess : super::HANDLE, ploadmodule : windows_sys::core::PCWSTR, psymboldata : PDBGHELP_DATA_REPORT_STRUCT) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SearchTreeForFile(rootpath : windows_sys::core::PCSTR, inputpathname : windows_sys::core::PCSTR, outputpathbuffer : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SearchTreeForFileW(rootpath : windows_sys::core::PCWSTR, inputpathname : windows_sys::core::PCWSTR, outputpathbuffer : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SetCheckUserInterruptShared(lpstartaddress : LPCALL_BACK_USER_INTERRUPT_ROUTINE));
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SetSymLoadError(error : super::DWORD));
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn StackWalk(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE, translateaddress : PTRANSLATE_ADDRESS_ROUTINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn StackWalk2(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME_EX, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, gettargetattributevalue : PGET_TARGET_ATTRIBUTE_VALUE64, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn StackWalk64(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME64, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn StackWalkEx(machinetype : super::DWORD, hprocess : super::HANDLE, hthread : super::HANDLE, stackframe : LPSTACKFRAME_EX, contextrecord : super::PVOID, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64, translateaddress : PTRANSLATE_ADDRESS_ROUTINE64, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymAddSourceStream(hprocess : super::HANDLE, base : super::ULONG64, streamfile : windows_sys::core::PCSTR, buffer : super::PBYTE, size : super::size_t) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymAddSourceStreamA(hprocess : super::HANDLE, base : super::ULONG64, streamfile : windows_sys::core::PCSTR, buffer : super::PBYTE, size : super::size_t) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymAddSourceStreamW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_sys::core::PCWSTR, buffer : super::PBYTE, size : super::size_t) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymAddSymbol(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_sys::core::PCSTR, address : super::DWORD64, size : super::DWORD, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymAddSymbolW(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_sys::core::PCWSTR, address : super::DWORD64, size : super::DWORD, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymAddrIncludeInlineTrace(hprocess : super::HANDLE, address : super::DWORD64) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymCleanup(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymCompareInlineTrace(hprocess : super::HANDLE, address1 : super::DWORD64, inlinecontext1 : super::DWORD, retaddress1 : super::DWORD64, address2 : super::DWORD64, retaddress2 : super::DWORD64) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymDeleteSymbol(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_sys::core::PCSTR, address : super::DWORD64, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymDeleteSymbolW(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_sys::core::PCWSTR, address : super::DWORD64, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumLines(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumLinesW(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymEnumProcesses(enumprocessescallback : PSYM_ENUMPROCESSES_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceFileTokens(hprocess : super::HANDLE, base : super::ULONG64, callback : PENUMSOURCEFILETOKENSCALLBACK) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceFiles(hprocess : super::HANDLE, modbase : super::ULONG64, mask : windows_sys::core::PCSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceFilesW(hprocess : super::HANDLE, modbase : super::ULONG64, mask : windows_sys::core::PCWSTR, cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceLines(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, line : super::DWORD, flags : super::DWORD, enumlinescallback : PSYM_ENUMLINES_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSourceLinesW(hprocess : super::HANDLE, base : super::ULONG64, obj : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, line : super::DWORD, flags : super::DWORD, enumlinescallback : PSYM_ENUMLINES_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSym(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbols(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_sys::core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsEx(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_sys::core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID, options : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsExW(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_sys::core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID, options : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddr(hprocess : super::HANDLE, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsForAddrW(hprocess : super::HANDLE, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumSymbolsW(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_sys::core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypes(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypesByName(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_sys::core::PCSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypesByNameW(hprocess : super::HANDLE, baseofdll : super::ULONG64, mask : windows_sys::core::PCWSTR, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumTypesW(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateModules(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateModules64(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACK64, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateModulesW64(hprocess : super::HANDLE, enummodulescallback : PSYM_ENUMMODULES_CALLBACKW64, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbols(hprocess : super::HANDLE, baseofdll : super::ULONG, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbols64(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW(hprocess : super::HANDLE, baseofdll : super::ULONG, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACKW, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymEnumerateSymbolsW64(hprocess : super::HANDLE, baseofdll : super::ULONG64, enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindDebugInfoFile(hprocess : super::HANDLE, filename : windows_sys::core::PCSTR, debugfilepath : windows_sys::core::PSTR, callback : PFIND_DEBUG_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindDebugInfoFileW(hprocess : super::HANDLE, filename : windows_sys::core::PCWSTR, debugfilepath : windows_sys::core::PWSTR, callback : PFIND_DEBUG_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindExecutableImage(hprocess : super::HANDLE, filename : windows_sys::core::PCSTR, imagefilepath : windows_sys::core::PSTR, callback : PFIND_EXE_FILE_CALLBACK, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymFindExecutableImageW(hprocess : super::HANDLE, filename : windows_sys::core::PCWSTR, imagefilepath : windows_sys::core::PWSTR, callback : PFIND_EXE_FILE_CALLBACKW, callerdata : super::PVOID) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFindFileInPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, id : super::PVOID, two : super::DWORD, three : super::DWORD, flags : super::DWORD, foundfile : windows_sys::core::PSTR, callback : PFINDFILEINPATHCALLBACK, context : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFindFileInPathW(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCWSTR, filename : windows_sys::core::PCWSTR, id : super::PVOID, two : super::DWORD, three : super::DWORD, flags : super::DWORD, foundfile : windows_sys::core::PWSTR, callback : PFINDFILEINPATHCALLBACKW, context : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromAddr(hprocess : super::HANDLE, address : super::DWORD64, displacement : super::PDWORD64, symbol : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromAddrW(hprocess : super::HANDLE, address : super::DWORD64, displacement : super::PDWORD64, symbol : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromIndex(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromIndexW(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromInlineContext(hprocess : super::HANDLE, address : super::DWORD64, inlinecontext : super::ULONG, displacement : super::PDWORD64, symbol : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromInlineContextW(hprocess : super::HANDLE, address : super::DWORD64, inlinecontext : super::ULONG, displacement : super::PDWORD64, symbol : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromName(hprocess : super::HANDLE, name : windows_sys::core::PCSTR, symbol : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromNameW(hprocess : super::HANDLE, name : windows_sys::core::PCWSTR, symbol : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromToken(hprocess : super::HANDLE, base : super::DWORD64, token : super::DWORD, symbol : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFromTokenW(hprocess : super::HANDLE, base : super::DWORD64, token : super::DWORD, symbol : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFunctionTableAccess(hprocess : super::HANDLE, addrbase : super::DWORD) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64(hprocess : super::HANDLE, addrbase : super::DWORD64) -> super::PVOID);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymFunctionTableAccess64AccessRoutines(hprocess : super::HANDLE, addrbase : super::DWORD64, readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64) -> super::PVOID);
windows_link::link!("dbghelp.dll" "system" fn SymGetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetFileLineOffsets64(hprocess : super::HANDLE, modulename : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, buffer : super::PDWORD64, bufferlines : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetHomeDirectory(r#type : super::DWORD, dir : windows_sys::core::PSTR, size : super::size_t) -> super::PCHAR);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SymGetHomeDirectoryW(r#type : super::DWORD, dir : windows_sys::core::PWSTR, size : super::size_t) -> windows_sys::core::PWSTR);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromAddr(hprocess : super::HANDLE, dwaddr : super::DWORD, pdwdisplacement : super::PDWORD, line : PIMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromAddr64(hprocess : super::HANDLE, qwaddr : super::DWORD64, pdwdisplacement : super::PDWORD, line64 : PIMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromAddrW64(hprocess : super::HANDLE, dwaddr : super::DWORD64, pdwdisplacement : super::PDWORD, line : PIMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContext(hprocess : super::HANDLE, qwaddr : super::DWORD64, inlinecontext : super::ULONG, qwmodulebaseaddress : super::DWORD64, pdwdisplacement : super::PDWORD, line64 : PIMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromInlineContextW(hprocess : super::HANDLE, dwaddr : super::DWORD64, inlinecontext : super::ULONG, qwmodulebaseaddress : super::DWORD64, pdwdisplacement : super::PDWORD, line : PIMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromName(hprocess : super::HANDLE, modulename : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, dwlinenumber : super::DWORD, pldisplacement : super::PLONG, line : PIMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromName64(hprocess : super::HANDLE, modulename : windows_sys::core::PCSTR, filename : windows_sys::core::PCSTR, dwlinenumber : super::DWORD, pldisplacement : super::PLONG, line : PIMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineFromNameW64(hprocess : super::HANDLE, modulename : windows_sys::core::PCWSTR, filename : windows_sys::core::PCWSTR, dwlinenumber : super::DWORD, pldisplacement : super::PLONG, line : PIMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineNext(hprocess : super::HANDLE, line : PIMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineNext64(hprocess : super::HANDLE, line : PIMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLineNextW64(hprocess : super::HANDLE, line : PIMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLinePrev(hprocess : super::HANDLE, line : PIMAGEHLP_LINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLinePrev64(hprocess : super::HANDLE, line : PIMAGEHLP_LINE64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetLinePrevW64(hprocess : super::HANDLE, line : PIMAGEHLP_LINEW64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleBase(hprocess : super::HANDLE, dwaddr : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleBase64(hprocess : super::HANDLE, qwaddr : super::DWORD64) -> super::DWORD64);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfo(hprocess : super::HANDLE, dwaddr : super::DWORD, moduleinfo : PIMAGEHLP_MODULE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfo64(hprocess : super::HANDLE, qwaddr : super::DWORD64, moduleinfo : PIMAGEHLP_MODULE64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfoW(hprocess : super::HANDLE, dwaddr : super::DWORD, moduleinfo : PIMAGEHLP_MODULEW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetModuleInfoW64(hprocess : super::HANDLE, qwaddr : super::DWORD64, moduleinfo : PIMAGEHLP_MODULEW64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetOmaps(hprocess : super::HANDLE, baseofdll : super::DWORD64, omapto : *mut POMAP, comapto : super::PDWORD64, omapfrom : *mut POMAP, comapfrom : super::PDWORD64) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SymGetOptions() -> super::DWORD);
#[cfg(feature = "windef")]
windows_link::link!("dbghelp.dll" "system" fn SymGetParentWindow(phwnd : *mut super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetScope(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetScopeW(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symbol : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSearchPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PSTR, searchpathlength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSearchPathW(hprocess : super::HANDLE, searchpatha : windows_sys::core::PWSTR, searchpathlength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFile(hprocess : super::HANDLE, base : super::ULONG64, params : windows_sys::core::PCSTR, filespec : windows_sys::core::PCSTR, filepath : windows_sys::core::PSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksum(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_sys::core::PCSTR, pchecksumtype : *mut super::DWORD, pchecksum : *mut super::BYTE, checksumsize : super::DWORD, pactualbyteswritten : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileChecksumW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_sys::core::PCWSTR, pchecksumtype : *mut super::DWORD, pchecksum : *mut super::BYTE, checksumsize : super::DWORD, pactualbyteswritten : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromToken(hprocess : super::HANDLE, token : super::PVOID, params : windows_sys::core::PCSTR, filepath : windows_sys::core::PSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenName(hprocess : super::HANDLE, token : super::PVOID, tokenname : windows_sys::core::PCSTR, params : windows_sys::core::PCSTR, filepath : windows_sys::core::PSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenByTokenNameW(hprocess : super::HANDLE, token : super::PVOID, tokenname : windows_sys::core::PCWSTR, params : windows_sys::core::PCWSTR, filepath : windows_sys::core::PWSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileFromTokenW(hprocess : super::HANDLE, token : super::PVOID, params : windows_sys::core::PCWSTR, filepath : windows_sys::core::PWSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileToken(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_sys::core::PCSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenName(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_sys::core::PCSTR, tokenname : windows_sys::core::PCSTR, tokenparameters : windows_sys::core::PCSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenByTokenNameW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_sys::core::PCWSTR, tokenname : windows_sys::core::PCWSTR, tokenparameters : windows_sys::core::PCWSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileTokenW(hprocess : super::HANDLE, base : super::ULONG64, filespec : windows_sys::core::PCWSTR, token : *mut super::PVOID, size : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceFileW(hprocess : super::HANDLE, base : super::ULONG64, params : windows_sys::core::PCWSTR, filespec : windows_sys::core::PCWSTR, filepath : windows_sys::core::PWSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceVarFromToken(hprocess : super::HANDLE, token : super::PVOID, params : windows_sys::core::PCSTR, varname : windows_sys::core::PCSTR, value : windows_sys::core::PSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSourceVarFromTokenW(hprocess : super::HANDLE, token : super::PVOID, params : windows_sys::core::PCWSTR, varname : windows_sys::core::PCWSTR, value : windows_sys::core::PWSTR, size : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromAddr(hprocess : super::HANDLE, dwaddr : super::DWORD, pdwdisplacement : super::PDWORD, symbol : PIMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromAddr64(hprocess : super::HANDLE, qwaddr : super::DWORD64, pdwdisplacement : super::PDWORD64, symbol : PIMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromName(hprocess : super::HANDLE, name : windows_sys::core::PCSTR, symbol : PIMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymFromName64(hprocess : super::HANDLE, name : windows_sys::core::PCSTR, symbol : PIMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymNext(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymNext64(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymPrev(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymPrev64(hprocess : super::HANDLE, symbol : PIMAGEHLP_SYMBOL64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymbolFile(hprocess : super::HANDLE, sympath : windows_sys::core::PCSTR, imagefile : windows_sys::core::PCSTR, r#type : super::DWORD, symbolfile : windows_sys::core::PSTR, csymbolfile : super::size_t, dbgfile : windows_sys::core::PSTR, cdbgfile : super::size_t) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetSymbolFileW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, imagefile : windows_sys::core::PCWSTR, r#type : super::DWORD, symbolfile : windows_sys::core::PWSTR, csymbolfile : super::size_t, dbgfile : windows_sys::core::PWSTR, cdbgfile : super::size_t) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeFromName(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_sys::core::PCSTR, symbol : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeFromNameW(hprocess : super::HANDLE, baseofdll : super::ULONG64, name : windows_sys::core::PCWSTR, symbol : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeInfo(hprocess : super::HANDLE, modbase : super::DWORD64, typeid : super::ULONG, gettype : IMAGEHLP_SYMBOL_TYPE_INFO, pinfo : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetTypeInfoEx(hprocess : super::HANDLE, modbase : super::DWORD64, params : PIMAGEHLP_GET_TYPE_INFO_PARAMS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymGetUnwindInfo(hprocess : super::HANDLE, address : super::DWORD64, buffer : super::PVOID, size : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymInitialize(hprocess : super::HANDLE, usersearchpath : windows_sys::core::PCSTR, finvadeprocess : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymInitializeW(hprocess : super::HANDLE, usersearchpath : windows_sys::core::PCWSTR, finvadeprocess : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModule(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCSTR, modulename : windows_sys::core::PCSTR, baseofdll : super::DWORD, sizeofdll : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModule64(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCSTR, modulename : windows_sys::core::PCSTR, baseofdll : super::DWORD64, sizeofdll : super::DWORD) -> super::DWORD64);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModuleEx(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCSTR, modulename : windows_sys::core::PCSTR, baseofdll : super::DWORD64, dllsize : super::DWORD, data : PMODLOAD_DATA, flags : super::DWORD) -> super::DWORD64);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymLoadModuleExW(hprocess : super::HANDLE, hfile : super::HANDLE, imagename : windows_sys::core::PCWSTR, modulename : windows_sys::core::PCWSTR, baseofdll : super::DWORD64, dllsize : super::DWORD, data : PMODLOAD_DATA, flags : super::DWORD) -> super::DWORD64);
windows_link::link!("dbghelp.dll" "system" fn SymMatchFileName(filename : windows_sys::core::PCSTR, r#match : windows_sys::core::PCSTR, filenamestop : *mut windows_sys::core::PSTR, matchstop : *mut windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchFileNameW(filename : windows_sys::core::PCWSTR, r#match : windows_sys::core::PCWSTR, filenamestop : *mut windows_sys::core::PWSTR, matchstop : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchString(string : windows_sys::core::PCSTR, expression : windows_sys::core::PCSTR, fcase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchStringA(string : windows_sys::core::PCSTR, expression : windows_sys::core::PCSTR, fcase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymMatchStringW(string : windows_sys::core::PCWSTR, expression : windows_sys::core::PCWSTR, fcase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymNext(hprocess : super::HANDLE, si : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymNextW(hprocess : super::HANDLE, siw : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymPrev(hprocess : super::HANDLE, si : PSYMBOL_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymPrevW(hprocess : super::HANDLE, siw : PSYMBOL_INFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymQueryInlineTrace(hprocess : super::HANDLE, startaddress : super::DWORD64, startcontext : super::DWORD, startretaddress : super::DWORD64, curaddress : super::DWORD64, curcontext : super::LPDWORD, curframeindex : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymRefreshModuleList(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterCallback64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : super::ULONG64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterCallbackW64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_REGISTERED_CALLBACK64, usercontext : super::ULONG64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback(hprocess : super::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK, usercontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymRegisterFunctionEntryCallback64(hprocess : super::HANDLE, callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK64, usercontext : super::ULONG64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSearch(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symtag : super::DWORD, mask : windows_sys::core::PCSTR, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext : super::PVOID, options : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSearchW(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD, symtag : super::DWORD, mask : windows_sys::core::PCWSTR, address : super::DWORD64, enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext : super::PVOID, options : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSetContext(hprocess : super::HANDLE, stackframe : PIMAGEHLP_STACK_FRAME, context : PIMAGEHLP_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("dbghelp.dll" "system" fn SymSetExtendedOption(option : IMAGEHLP_EXTENDED_OPTIONS, value : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetHomeDirectory(hprocess : super::HANDLE, dir : windows_sys::core::PCSTR) -> super::PCHAR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetHomeDirectoryW(hprocess : super::HANDLE, dir : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SymSetOptions(symoptions : super::DWORD) -> super::DWORD);
#[cfg(feature = "windef")]
windows_link::link!("dbghelp.dll" "system" fn SymSetParentWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSetScopeFromAddr(hprocess : super::HANDLE, address : super::ULONG64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSetScopeFromIndex(hprocess : super::HANDLE, baseofdll : super::ULONG64, index : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSetScopeFromInlineContext(hprocess : super::HANDLE, address : super::ULONG64, inlinecontext : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetSearchPath(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSetSearchPathW(hprocess : super::HANDLE, searchpatha : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvDeltaName(hprocess : super::HANDLE, sympath : windows_sys::core::PCSTR, r#type : windows_sys::core::PCSTR, file1 : windows_sys::core::PCSTR, file2 : windows_sys::core::PCSTR) -> windows_sys::core::PCSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvDeltaNameW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, r#type : windows_sys::core::PCWSTR, file1 : windows_sys::core::PCWSTR, file2 : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfo(file : windows_sys::core::PCSTR, info : PSYMSRV_INDEX_INFO, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexInfoW(file : windows_sys::core::PCWSTR, info : PSYMSRV_INDEX_INFOW, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexString(hprocess : super::HANDLE, srvpath : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, index : windows_sys::core::PSTR, size : super::size_t, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexStringW(hprocess : super::HANDLE, srvpath : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, index : windows_sys::core::PWSTR, size : super::size_t, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexes(file : windows_sys::core::PCSTR, id : *mut windows_sys::core::GUID, val1 : super::PDWORD, val2 : super::PDWORD, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetFileIndexesW(file : windows_sys::core::PCWSTR, id : *mut windows_sys::core::GUID, val1 : super::PDWORD, val2 : super::PDWORD, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetSupplement(hprocess : super::HANDLE, sympath : windows_sys::core::PCSTR, node : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR) -> windows_sys::core::PCSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvGetSupplementW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, node : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvIsStore(hprocess : super::HANDLE, path : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("dbghelp.dll" "system" fn SymSrvIsStoreW(hprocess : super::HANDLE, path : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreFile(hprocess : super::HANDLE, srvpath : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, flags : super::DWORD) -> windows_sys::core::PCSTR);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreFileW(hprocess : super::HANDLE, srvpath : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, flags : super::DWORD) -> windows_sys::core::PCWSTR);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreSupplement(hprocess : super::HANDLE, srvpath : windows_sys::core::PCSTR, node : windows_sys::core::PCSTR, file : windows_sys::core::PCSTR, flags : super::DWORD) -> windows_sys::core::PCSTR);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymSrvStoreSupplementW(hprocess : super::HANDLE, sympath : windows_sys::core::PCWSTR, node : windows_sys::core::PCWSTR, file : windows_sys::core::PCWSTR, flags : super::DWORD) -> windows_sys::core::PCWSTR);
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SymUnDName(sym : PIMAGEHLP_SYMBOL, undecname : windows_sys::core::PSTR, undecnamelength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn SymUnDName64(sym : PIMAGEHLP_SYMBOL64, undecname : windows_sys::core::PSTR, undecnamelength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymUnloadModule(hprocess : super::HANDLE, baseofdll : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn SymUnloadModule64(hprocess : super::HANDLE, baseofdll : super::DWORD64) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn UnDecorateSymbolName(name : windows_sys::core::PCSTR, outputstring : windows_sys::core::PSTR, maxstringlength : super::DWORD, flags : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dbghelp.dll" "system" fn UnDecorateSymbolNameW(name : windows_sys::core::PCWSTR, outputstring : windows_sys::core::PWSTR, maxstringlength : super::DWORD, flags : super::DWORD) -> super::DWORD);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dbghelp.dll" "system" fn UnmapDebugInformation(debuginfo : PIMAGE_DEBUG_INFORMATION) -> windows_sys::core::BOOL);
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
    pub pBinPathNonExist: windows_sys::core::PCWSTR,
    pub pSymbolPathNonExist: windows_sys::core::PCWSTR,
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
    pub desc: windows_sys::core::PCWSTR,
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
    pub FileName: windows_sys::core::PWSTR,
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
    pub PdbSig70: windows_sys::core::GUID,
    pub PdbAge: super::DWORD,
    pub PdbUnmatched: windows_sys::core::BOOL,
    pub DbgUnmatched: windows_sys::core::BOOL,
    pub LineNumbers: windows_sys::core::BOOL,
    pub GlobalSymbols: windows_sys::core::BOOL,
    pub TypeInfo: windows_sys::core::BOOL,
    pub SourceIndexed: windows_sys::core::BOOL,
    pub Publics: windows_sys::core::BOOL,
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
    pub PdbSig70: windows_sys::core::GUID,
    pub PdbAge: super::DWORD,
    pub PdbUnmatched: windows_sys::core::BOOL,
    pub DbgUnmatched: windows_sys::core::BOOL,
    pub LineNumbers: windows_sys::core::BOOL,
    pub GlobalSymbols: windows_sys::core::BOOL,
    pub TypeInfo: windows_sys::core::BOOL,
    pub SourceIndexed: windows_sys::core::BOOL,
    pub Publics: windows_sys::core::BOOL,
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
    pub Virtual: windows_sys::core::BOOL,
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
    pub ReservedExportedNames: windows_sys::core::PSTR,
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
    pub ImageFilePath: windows_sys::core::PSTR,
    pub ImageFileName: windows_sys::core::PSTR,
    pub ReservedDebugFilePath: windows_sys::core::PSTR,
    pub ReservedTimeDateStamp: super::DWORD,
    pub ReservedRomImage: windows_sys::core::BOOL,
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
    pub ModuleName: windows_sys::core::PSTR,
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
    pub ModuleName: windows_sys::core::PSTR,
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
    pub PdbGuid: windows_sys::core::GUID,
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
pub type PDBGHELP_CREATE_USER_DUMP_CALLBACK = Option<unsafe extern "system" fn(datatype: super::DWORD, data: *const super::PVOID, datalength: super::LPDWORD, userdata: super::PVOID) -> windows_sys::core::BOOL>;
pub type PDBGHELP_DATA_REPORT_STRUCT = *mut DBGHELP_DATA_REPORT_STRUCT;
#[cfg(feature = "winnt")]
pub type PENUMDIRTREE_CALLBACK = Option<unsafe extern "system" fn(filepath: windows_sys::core::PCSTR, callerdata: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PENUMDIRTREE_CALLBACKW = Option<unsafe extern "system" fn(filepath: windows_sys::core::PCWSTR, callerdata: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMLOADED_MODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, modulebase: super::ULONG, modulesize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMLOADED_MODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, modulebase: super::DWORD64, modulesize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMLOADED_MODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCWSTR, modulebase: super::DWORD64, modulesize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMSOURCEFILETOKENSCALLBACK = Option<unsafe extern "system" fn(token: super::PVOID, size: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFINDFILEINPATHCALLBACK = Option<unsafe extern "system" fn(filename: windows_sys::core::PCSTR, context: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFINDFILEINPATHCALLBACKW = Option<unsafe extern "system" fn(filename: windows_sys::core::PCWSTR, context: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCSTR, callerdata: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_DEBUG_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCWSTR, callerdata: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACK = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCSTR, callerdata: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFIND_EXE_FILE_CALLBACKW = Option<unsafe extern "system" fn(filehandle: super::HANDLE, filename: windows_sys::core::PCWSTR, callerdata: super::PVOID) -> windows_sys::core::BOOL>;
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
pub type PGET_TARGET_ATTRIBUTE_VALUE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, attribute: super::DWORD, attributedata: super::DWORD64, attributevalue: *mut super::DWORD64) -> windows_sys::core::BOOL>;
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
pub type PREAD_PROCESS_MEMORY_ROUTINE = Option<unsafe extern "system" fn(hprocess: super::HANDLE, lpbaseaddress: super::DWORD, lpbuffer: super::PVOID, nsize: super::DWORD, lpnumberofbytesread: super::PDWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, qwbaseaddress: super::DWORD64, lpbuffer: super::PVOID, nsize: super::DWORD, lpnumberofbytesread: super::LPDWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSOURCEFILE = *mut SOURCEFILE;
#[cfg(feature = "shared")]
pub type PSOURCEFILEW = *mut SOURCEFILEW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSRCCODEINFO = *mut SRCCODEINFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSRCCODEINFOW = *mut SRCCODEINFOW;
pub type PSYMBOLSERVERBYINDEXPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERBYINDEXPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PWSTR) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERCALLBACKPROC = Option<unsafe extern "system" fn(action: super::UINT_PTR, data: super::ULONG64, context: super::ULONG64) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERCLOSEPROC = Option<unsafe extern "system" fn() -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERDELTANAME = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: super::PVOID, param2: super::DWORD, param3: super::DWORD, param4: super::PVOID, param5: super::DWORD, param6: super::DWORD, param7: windows_sys::core::PSTR, param8: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERDELTANAMEW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::PVOID, param2: super::DWORD, param3: super::DWORD, param4: super::PVOID, param5: super::DWORD, param6: super::DWORD, param7: windows_sys::core::PWSTR, param8: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERGETINDEXSTRING = Option<unsafe extern "system" fn(param0: super::PVOID, param1: super::DWORD, param2: super::DWORD, param3: windows_sys::core::PSTR, param4: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERGETINDEXSTRINGW = Option<unsafe extern "system" fn(param0: super::PVOID, param1: super::DWORD, param2: super::DWORD, param3: windows_sys::core::PWSTR, param4: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PSYMBOLSERVERGETOPTIONDATAPROC = Option<unsafe extern "system" fn(param0: super::UINT_PTR, param1: super::PULONG64) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETOPTIONSPROC = Option<unsafe extern "system" fn() -> super::UINT_PTR>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETSUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PSTR, param4: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETSUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PWSTR, param4: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERGETVERSION = Option<unsafe extern "system" fn(param0: LPAPI_VERSION) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERISSTORE = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERISSTOREW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERMESSAGEPROC = Option<unsafe extern "system" fn(action: super::UINT_PTR, data: super::ULONG64, context: super::ULONG64) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVEROPENPROC = Option<unsafe extern "system" fn() -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERPINGPROCWEX = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_sys::core::PSTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_sys::core::PSTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_sys::core::PWSTR) -> windows_sys::core::BOOL>;
pub type PSYMBOLSERVERSETHTTPAUTHHEADER = Option<unsafe extern "system" fn(pszauthheader: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSETOPTIONSPROC = Option<unsafe extern "system" fn(param0: super::UINT_PTR, param1: super::ULONG64) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSETOPTIONSWPROC = Option<unsafe extern "system" fn(param0: super::UINT_PTR, param1: super::ULONG64) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERSTOREFILE = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_sys::core::PSTR, param6: super::size_t, param7: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERSTOREFILEW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_sys::core::PWSTR, param6: super::size_t, param7: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSTORESUPPLEMENT = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: windows_sys::core::PSTR, param4: super::size_t, param5: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERSTORESUPPLEMENTW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: windows_sys::core::PWSTR, param4: super::size_t, param5: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PSYMBOLSERVERVERSION = Option<unsafe extern "system" fn() -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOLSERVERWEXPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: super::PVOID, param3: super::DWORD, param4: super::DWORD, param5: windows_sys::core::PWSTR, param6: PSYMSRV_EXTENDED_OUTPUT_DATA) -> windows_sys::core::BOOL>;
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
pub type PSYMBOL_REGISTERED_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, actioncode: super::ULONG, callbackdata: super::PVOID, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMBOL_REGISTERED_CALLBACK64 = Option<unsafe extern "system" fn(hprocess: super::HANDLE, actioncode: super::ULONG, callbackdata: super::ULONG64, usercontext: super::ULONG64) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMSRV_EXTENDED_OUTPUT_DATA = *mut SYMSRV_EXTENDED_OUTPUT_DATA;
#[cfg(feature = "shared")]
pub type PSYMSRV_INDEX_INFO = *mut SYMSRV_INDEX_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYMSRV_INDEX_INFOW = *mut SYMSRV_INDEX_INFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMERATESYMBOLS_CALLBACK = Option<unsafe extern "system" fn(psyminfo: PSYMBOL_INFO, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMERATESYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(psyminfo: PSYMBOL_INFOW, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMLINES_CALLBACK = Option<unsafe extern "system" fn(lineinfo: PSRCCODEINFO, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMLINES_CALLBACKW = Option<unsafe extern "system" fn(lineinfo: PSRCCODEINFOW, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMMODULES_CALLBACK = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, baseofdll: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMMODULES_CALLBACK64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCSTR, baseofdll: super::DWORD64, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMMODULES_CALLBACKW64 = Option<unsafe extern "system" fn(modulename: windows_sys::core::PCWSTR, baseofdll: super::DWORD64, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSYM_ENUMPROCESSES_CALLBACK = Option<unsafe extern "system" fn(hprocess: super::HANDLE, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSOURCEFILES_CALLBACK = Option<unsafe extern "system" fn(psourcefile: PSOURCEFILE, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSOURCEFILES_CALLBACKW = Option<unsafe extern "system" fn(psourcefile: PSOURCEFILEW, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACK = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCSTR, symboladdress: super::ULONG, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACK64 = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCSTR, symboladdress: super::DWORD64, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACK64W = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCWSTR, symboladdress: super::DWORD64, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYM_ENUMSYMBOLS_CALLBACKW = Option<unsafe extern "system" fn(symbolname: windows_sys::core::PCWSTR, symboladdress: super::ULONG, symbolsize: super::ULONG, usercontext: super::PVOID) -> windows_sys::core::BOOL>;
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
    pub FileName: windows_sys::core::PWSTR,
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
pub const SSRVOPT_RESET: super::ULONG_PTR = 4294967295u64 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const SSRVOPT_RESET: super::ULONG_PTR = 18446744073709551615u64 as _;
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
    pub Far: windows_sys::core::BOOL,
    pub Virtual: windows_sys::core::BOOL,
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
    pub Far: windows_sys::core::BOOL,
    pub Virtual: windows_sys::core::BOOL,
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
    pub Far: windows_sys::core::BOOL,
    pub Virtual: windows_sys::core::BOOL,
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
pub type SYMADDSOURCESTREAM = Option<unsafe extern "system" fn(param0: super::HANDLE, param1: super::ULONG64, param2: windows_sys::core::PCSTR, param3: super::PBYTE, param4: super::size_t) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SYMADDSOURCESTREAMA = Option<unsafe extern "system" fn(param0: super::HANDLE, param1: super::ULONG64, param2: windows_sys::core::PCSTR, param3: super::PBYTE, param4: super::size_t) -> windows_sys::core::BOOL>;
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
    pub stripped: windows_sys::core::BOOL,
    pub timestamp: super::DWORD,
    pub size: super::DWORD,
    pub dbgfile: [i8; 261],
    pub pdbfile: [i8; 261],
    pub guid: windows_sys::core::GUID,
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
    pub stripped: windows_sys::core::BOOL,
    pub timestamp: super::DWORD,
    pub size: super::DWORD,
    pub dbgfile: [super::WCHAR; 261],
    pub pdbfile: [super::WCHAR; 261],
    pub guid: windows_sys::core::GUID,
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
