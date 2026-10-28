#[cfg(feature = "guiddef")]
windows_link::link!("shlwapi.dll" "system" fn AssocCreate(clsid : super::CLSID, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shlwapi.dll" "system" fn AssocGetPerceivedType(pszext : windows_sys::core::PCWSTR, ptype : *mut super::PERCEIVED, pflag : *mut super::PERCEIVEDFLAG, ppsztype : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn AssocIsDangerous(pszassoc : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("shlwapi.dll" "system" fn AssocQueryKeyA(flags : ASSOCF, key : ASSOCKEY, pszassoc : windows_sys::core::PCSTR, pszextra : windows_sys::core::PCSTR, phkeyout : *mut super::HKEY) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("shlwapi.dll" "system" fn AssocQueryKeyW(flags : ASSOCF, key : ASSOCKEY, pszassoc : windows_sys::core::PCWSTR, pszextra : windows_sys::core::PCWSTR, phkeyout : *mut super::HKEY) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn AssocQueryStringA(flags : ASSOCF, str : ASSOCSTR, pszassoc : windows_sys::core::PCSTR, pszextra : windows_sys::core::PCSTR, pszout : windows_sys::core::PSTR, pcchout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn AssocQueryStringByKeyA(flags : ASSOCF, str : ASSOCSTR, hkassoc : super::HKEY, pszextra : windows_sys::core::PCSTR, pszout : windows_sys::core::PSTR, pcchout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn AssocQueryStringByKeyW(flags : ASSOCF, str : ASSOCSTR, hkassoc : super::HKEY, pszextra : windows_sys::core::PCWSTR, pszout : windows_sys::core::PWSTR, pcchout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn AssocQueryStringW(flags : ASSOCF, str : ASSOCSTR, pszassoc : windows_sys::core::PCWSTR, pszextra : windows_sys::core::PCWSTR, pszout : windows_sys::core::PWSTR, pcchout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn ChrCmpIA(w1 : super::WORD, w2 : super::WORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn ChrCmpIW(w1 : super::WCHAR, w2 : super::WCHAR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shlwapi.dll" "system" fn ColorAdjustLuma(clrrgb : super::COLORREF, n : i32, fscale : windows_sys::core::BOOL) -> super::COLORREF);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn ColorHLSToRGB(whue : super::WORD, wluminance : super::WORD, wsaturation : super::WORD) -> super::COLORREF);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn ColorRGBToHLS(clrrgb : super::COLORREF, pwhue : *mut super::WORD, pwluminance : *mut super::WORD, pwsaturation : *mut super::WORD));
#[cfg(all(feature = "guiddef", feature = "ocidl", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn ConnectToConnectionPoint(punk : *mut core::ffi::c_void, riidevent : *const super::IID, fconnect : windows_sys::core::BOOL, punktarget : *mut core::ffi::c_void, pdwcookie : *mut super::DWORD, ppcpout : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn GetAcceptLanguagesA(pszlanguages : windows_sys::core::PSTR, pcchlanguages : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn GetAcceptLanguagesW(pszlanguages : windows_sys::core::PWSTR, pcchlanguages : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn GetMenuPosFromID(hmenu : super::HMENU, id : super::UINT) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn HashData(pbdata : *const super::BYTE, cbdata : super::DWORD, pbhash : *mut super::BYTE, cbhash : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn IStream_Copy(pstmfrom : *mut core::ffi::c_void, pstmto : *mut core::ffi::c_void, cb : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn IStream_Read(pstm : *mut core::ffi::c_void, pv : *mut core::ffi::c_void, cb : super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn IStream_ReadPidl(pstm : *mut core::ffi::c_void, ppidlout : *mut super::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("shlwapi.dll" "system" fn IStream_ReadStr(pstm : *mut core::ffi::c_void, ppsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("shlwapi.dll" "system" fn IStream_Reset(pstm : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn IStream_Size(pstm : *mut core::ffi::c_void, pui : *mut super::ULARGE_INTEGER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn IStream_Write(pstm : *mut core::ffi::c_void, pv : *const core::ffi::c_void, cb : super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn IStream_WritePidl(pstm : *mut core::ffi::c_void, pidlwrite : super::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("shlwapi.dll" "system" fn IStream_WriteStr(pstm : *mut core::ffi::c_void, psz : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn IUnknown_AtomicRelease(ppunk : *mut *mut core::ffi::c_void));
#[cfg(feature = "guiddef")]
windows_link::link!("shlwapi.dll" "system" fn IUnknown_GetSite(punk : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shlwapi.dll" "system" fn IUnknown_GetWindow(punk : *mut core::ffi::c_void, phwnd : *mut super::HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("shlwapi.dll" "system" fn IUnknown_QueryService(punk : *mut core::ffi::c_void, guidservice : *const windows_sys::core::GUID, riid : *const super::IID, ppvout : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn IUnknown_Set(ppunk : *mut *mut core::ffi::c_void, punk : *mut core::ffi::c_void));
windows_link::link!("shlwapi.dll" "system" fn IUnknown_SetSite(punk : *mut core::ffi::c_void, punksite : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn IntlStrEqWorkerA(fcasesens : windows_sys::core::BOOL, lpstring1 : windows_sys::core::PCSTR, lpstring2 : windows_sys::core::PCSTR, nchar : i32) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn IntlStrEqWorkerW(fcasesens : windows_sys::core::BOOL, lpstring1 : windows_sys::core::PCWSTR, lpstring2 : windows_sys::core::PCWSTR, nchar : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn IsCharSpaceA(wch : super::CHAR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn IsCharSpaceW(wch : super::WCHAR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn IsInternetESCEnabled() -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn IsOS(dwos : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn ParseURLA(pcszurl : windows_sys::core::PCSTR, ppu : *mut PARSEDURLA) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn ParseURLW(pcszurl : windows_sys::core::PCWSTR, ppu : *mut PARSEDURLW) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn PathAddBackslashA(pszpath : windows_sys::core::PSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathAddBackslashW(pszpath : windows_sys::core::PWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathAddExtensionA(pszpath : windows_sys::core::PSTR, pszext : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathAddExtensionW(pszpath : windows_sys::core::PWSTR, pszext : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathAppendA(pszpath : windows_sys::core::PSTR, pszmore : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathAppendW(pszpath : windows_sys::core::PWSTR, pszmore : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathBuildRootA(pszroot : windows_sys::core::PSTR, idrive : i32) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathBuildRootW(pszroot : windows_sys::core::PWSTR, idrive : i32) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathCanonicalizeA(pszbuf : windows_sys::core::PSTR, pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathCanonicalizeW(pszbuf : windows_sys::core::PWSTR, pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathCombineA(pszdest : windows_sys::core::PSTR, pszdir : windows_sys::core::PCSTR, pszfile : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathCombineW(pszdest : windows_sys::core::PWSTR, pszdir : windows_sys::core::PCWSTR, pszfile : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathCommonPrefixA(pszfile1 : windows_sys::core::PCSTR, pszfile2 : windows_sys::core::PCSTR, achpath : windows_sys::core::PSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn PathCommonPrefixW(pszfile1 : windows_sys::core::PCWSTR, pszfile2 : windows_sys::core::PCWSTR, achpath : windows_sys::core::PWSTR) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn PathCompactPathA(hdc : super::HDC, pszpath : windows_sys::core::PSTR, dx : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathCompactPathExA(pszout : windows_sys::core::PSTR, pszsrc : windows_sys::core::PCSTR, cchmax : super::UINT, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathCompactPathExW(pszout : windows_sys::core::PWSTR, pszsrc : windows_sys::core::PCWSTR, cchmax : super::UINT, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn PathCompactPathW(hdc : super::HDC, pszpath : windows_sys::core::PWSTR, dx : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathCreateFromUrlA(pszurl : windows_sys::core::PCSTR, pszpath : windows_sys::core::PSTR, pcchpath : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathCreateFromUrlAlloc(pszin : windows_sys::core::PCWSTR, ppszout : *mut windows_sys::core::PWSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathCreateFromUrlW(pszurl : windows_sys::core::PCWSTR, pszpath : windows_sys::core::PWSTR, pcchpath : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn PathFileExistsA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathFileExistsW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathFindExtensionA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathFindExtensionW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathFindFileNameA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathFindFileNameW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathFindNextComponentA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathFindNextComponentW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn PathFindOnPathA(pszpath : windows_sys::core::PSTR, ppszotherdirs : super::PZPCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn PathFindOnPathW(pszpath : windows_sys::core::PWSTR, ppszotherdirs : super::PZPCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathFindSuffixArrayA(pszpath : windows_sys::core::PCSTR, apszsuffix : *const windows_sys::core::PCSTR, iarraysize : i32) -> windows_sys::core::PCSTR);
windows_link::link!("shlwapi.dll" "system" fn PathFindSuffixArrayW(pszpath : windows_sys::core::PCWSTR, apszsuffix : *const windows_sys::core::PCWSTR, iarraysize : i32) -> windows_sys::core::PCWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathGetArgsA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathGetArgsW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathGetCharTypeA(ch : super::UCHAR) -> super::UINT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn PathGetCharTypeW(ch : super::WCHAR) -> super::UINT);
windows_link::link!("shlwapi.dll" "system" fn PathGetDriveNumberA(pszpath : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn PathGetDriveNumberW(pszpath : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn PathIsContentTypeA(pszpath : windows_sys::core::PCSTR, pszcontenttype : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsContentTypeW(pszpath : windows_sys::core::PCWSTR, pszcontenttype : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsDirectoryA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsDirectoryEmptyA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsDirectoryEmptyW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsDirectoryW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsFileSpecA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsFileSpecW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsLFNFileSpecA(pszname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsLFNFileSpecW(pszname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsNetworkPathA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsNetworkPathW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsPrefixA(pszprefix : windows_sys::core::PCSTR, pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsPrefixW(pszprefix : windows_sys::core::PCWSTR, pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsRelativeA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsRelativeW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsRootA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsRootW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsSameRootA(pszpath1 : windows_sys::core::PCSTR, pszpath2 : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsSameRootW(pszpath1 : windows_sys::core::PCWSTR, pszpath2 : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathIsSystemFolderA(pszpath : windows_sys::core::PCSTR, dwattrb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathIsSystemFolderW(pszpath : windows_sys::core::PCWSTR, dwattrb : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsUNCA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsUNCServerA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsUNCServerShareA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsUNCServerShareW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsUNCServerW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsUNCW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsURLA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathIsURLW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathMakePrettyA(pszpath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathMakePrettyW(pszpath : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathMakeSystemFolderA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathMakeSystemFolderW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathMatchSpecA(pszfile : windows_sys::core::PCSTR, pszspec : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathMatchSpecExA(pszfile : windows_sys::core::PCSTR, pszspec : windows_sys::core::PCSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathMatchSpecExW(pszfile : windows_sys::core::PCWSTR, pszspec : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn PathMatchSpecW(pszfile : windows_sys::core::PCWSTR, pszspec : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathParseIconLocationA(psziconfile : windows_sys::core::PSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn PathParseIconLocationW(psziconfile : windows_sys::core::PWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn PathQuoteSpacesA(lpsz : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathQuoteSpacesW(lpsz : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathRelativePathToA(pszpath : windows_sys::core::PSTR, pszfrom : windows_sys::core::PCSTR, dwattrfrom : super::DWORD, pszto : windows_sys::core::PCSTR, dwattrto : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathRelativePathToW(pszpath : windows_sys::core::PWSTR, pszfrom : windows_sys::core::PCWSTR, dwattrfrom : super::DWORD, pszto : windows_sys::core::PCWSTR, dwattrto : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathRemoveArgsA(pszpath : windows_sys::core::PSTR));
windows_link::link!("shlwapi.dll" "system" fn PathRemoveArgsW(pszpath : windows_sys::core::PWSTR));
windows_link::link!("shlwapi.dll" "system" fn PathRemoveBackslashA(pszpath : windows_sys::core::PSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathRemoveBackslashW(pszpath : windows_sys::core::PWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathRemoveBlanksA(pszpath : windows_sys::core::PSTR));
windows_link::link!("shlwapi.dll" "system" fn PathRemoveBlanksW(pszpath : windows_sys::core::PWSTR));
windows_link::link!("shlwapi.dll" "system" fn PathRemoveExtensionA(pszpath : windows_sys::core::PSTR));
windows_link::link!("shlwapi.dll" "system" fn PathRemoveExtensionW(pszpath : windows_sys::core::PWSTR));
windows_link::link!("shlwapi.dll" "system" fn PathRemoveFileSpecA(pszpath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathRemoveFileSpecW(pszpath : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathRenameExtensionA(pszpath : windows_sys::core::PSTR, pszext : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathRenameExtensionW(pszpath : windows_sys::core::PWSTR, pszext : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathSearchAndQualifyA(pszpath : windows_sys::core::PCSTR, pszbuf : windows_sys::core::PSTR, cchbuf : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathSearchAndQualifyW(pszpath : windows_sys::core::PCWSTR, pszbuf : windows_sys::core::PWSTR, cchbuf : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shlwapi.dll" "system" fn PathSetDlgItemPathA(hdlg : super::HWND, id : i32, pszpath : windows_sys::core::PCSTR));
#[cfg(feature = "windef")]
windows_link::link!("shlwapi.dll" "system" fn PathSetDlgItemPathW(hdlg : super::HWND, id : i32, pszpath : windows_sys::core::PCWSTR));
windows_link::link!("shlwapi.dll" "system" fn PathSkipRootA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn PathSkipRootW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn PathStripPathA(pszpath : windows_sys::core::PSTR));
windows_link::link!("shlwapi.dll" "system" fn PathStripPathW(pszpath : windows_sys::core::PWSTR));
windows_link::link!("shlwapi.dll" "system" fn PathStripToRootA(pszpath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathStripToRootW(pszpath : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathUnExpandEnvStringsA(pszpath : windows_sys::core::PCSTR, pszbuf : windows_sys::core::PSTR, cchbuf : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn PathUnExpandEnvStringsW(pszpath : windows_sys::core::PCWSTR, pszbuf : windows_sys::core::PWSTR, cchbuf : super::UINT) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathUndecorateA(pszpath : windows_sys::core::PSTR));
windows_link::link!("shlwapi.dll" "system" fn PathUndecorateW(pszpath : windows_sys::core::PWSTR));
windows_link::link!("shlwapi.dll" "system" fn PathUnmakeSystemFolderA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathUnmakeSystemFolderW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathUnquoteSpacesA(lpsz : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn PathUnquoteSpacesW(lpsz : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn QISearch(that : *mut core::ffi::c_void, pqit : LPCQITAB, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn SHAllocShared(pvdata : *const core::ffi::c_void, dwsize : super::DWORD, dwprocessid : super::DWORD) -> super::HANDLE);
windows_link::link!("shlwapi.dll" "system" fn SHAnsiToAnsi(pszsrc : windows_sys::core::PCSTR, pszdst : windows_sys::core::PSTR, cchbuf : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn SHAnsiToUnicode(pszsrc : windows_sys::core::PCSTR, pwszdst : windows_sys::core::PWSTR, cwchbuf : i32) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn SHAutoComplete(hwndedit : super::HWND, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHCopyKeyA(hkeysrc : super::HKEY, pszsrcsubkey : windows_sys::core::PCSTR, hkeydest : super::HKEY, freserved : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHCopyKeyW(hkeysrc : super::HKEY, pszsrcsubkey : windows_sys::core::PCWSTR, hkeydest : super::HKEY, freserved : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHCreateMemStream(pinit : *const super::BYTE, cbinit : super::UINT) -> *mut core::ffi::c_void);
#[cfg(feature = "windef")]
windows_link::link!("shlwapi.dll" "system" fn SHCreateShellPalette(hdc : super::HDC) -> super::HPALETTE);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHCreateStreamOnFileA(pszfile : windows_sys::core::PCSTR, grfmode : super::DWORD, ppstm : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHCreateStreamOnFileEx(pszfile : windows_sys::core::PCWSTR, grfmode : super::DWORD, dwattributes : super::DWORD, fcreate : windows_sys::core::BOOL, pstmtemplate : *mut core::ffi::c_void, ppstm : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHCreateStreamOnFileW(pszfile : windows_sys::core::PCWSTR, grfmode : super::DWORD, ppstm : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHCreateThread(pfnthreadproc : super::LPTHREAD_START_ROUTINE, pdata : *const core::ffi::c_void, flags : SHCT_FLAGS, pfncallback : super::LPTHREAD_START_ROUTINE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn SHCreateThreadRef(pcref : *mut super::LONG, ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn SHCreateThreadWithHandle(pfnthreadproc : super::LPTHREAD_START_ROUTINE, pdata : *const core::ffi::c_void, flags : SHCT_FLAGS, pfncallback : super::LPTHREAD_START_ROUTINE, phandle : *mut super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHDeleteEmptyKeyA(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHDeleteEmptyKeyW(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHDeleteKeyA(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHDeleteKeyW(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHDeleteValueA(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHDeleteValueW(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHEnumKeyExA(hkey : super::HKEY, dwindex : super::DWORD, pszname : windows_sys::core::PSTR, pcchname : super::LPDWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHEnumKeyExW(hkey : super::HKEY, dwindex : super::DWORD, pszname : windows_sys::core::PWSTR, pcchname : super::LPDWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHEnumValueA(hkey : super::HKEY, dwindex : super::DWORD, pszvaluename : windows_sys::core::PSTR, pcchvaluename : super::LPDWORD, pdwtype : super::LPDWORD, pvdata : *mut core::ffi::c_void, pcbdata : super::LPDWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHEnumValueW(hkey : super::HKEY, dwindex : super::DWORD, pszvaluename : windows_sys::core::PWSTR, pcchvaluename : super::LPDWORD, pdwtype : super::LPDWORD, pvdata : *mut core::ffi::c_void, pcbdata : super::LPDWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn SHFreeShared(hdata : super::HANDLE, dwprocessid : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn SHGetInverseCMAP(pbmap : *mut super::BYTE, cbmap : super::ULONG) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn SHGetThreadRef(ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHGetValueA(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHGetValueW(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn SHGetViewStatePropertyBag(pidl : super::LPCITEMIDLIST, pszbagname : windows_sys::core::PCWSTR, dwflags : super::DWORD, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn SHGlobalCounterDecrement(id : SHGLOBALCOUNTER) -> i32);
windows_link::link!("shlwapi.dll" "system" fn SHGlobalCounterGetValue(id : SHGLOBALCOUNTER) -> i32);
windows_link::link!("shlwapi.dll" "system" fn SHGlobalCounterIncrement(id : SHGLOBALCOUNTER) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn SHIsLowMemoryMachine(dwtype : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn SHLoadIndirectString(pszsource : windows_sys::core::PCWSTR, pszoutbuf : windows_sys::core::PWSTR, cchoutbuf : super::UINT, ppvreserved : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn SHLockShared(hdata : super::HANDLE, dwprocessid : super::DWORD) -> *mut core::ffi::c_void);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn SHMessageBoxCheckA(hwnd : super::HWND, psztext : windows_sys::core::PCSTR, pszcaption : windows_sys::core::PCSTR, utype : super::UINT, idefault : i32, pszregval : windows_sys::core::PCSTR) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shlwapi.dll" "system" fn SHMessageBoxCheckW(hwnd : super::HWND, psztext : windows_sys::core::PCWSTR, pszcaption : windows_sys::core::PCWSTR, utype : super::UINT, idefault : i32, pszregval : windows_sys::core::PCWSTR) -> i32);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHOpenRegStream2A(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, grfmode : super::DWORD) -> *mut core::ffi::c_void);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHOpenRegStream2W(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, grfmode : super::DWORD) -> *mut core::ffi::c_void);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHOpenRegStreamA(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, grfmode : super::DWORD) -> *mut core::ffi::c_void);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHOpenRegStreamW(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, grfmode : super::DWORD) -> *mut core::ffi::c_void);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHQueryInfoKeyA(hkey : super::HKEY, pcsubkeys : super::LPDWORD, pcchmaxsubkeylen : super::LPDWORD, pcvalues : super::LPDWORD, pcchmaxvaluenamelen : super::LPDWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHQueryInfoKeyW(hkey : super::HKEY, pcsubkeys : super::LPDWORD, pcchmaxsubkeylen : super::LPDWORD, pcvalues : super::LPDWORD, pcchmaxvaluenamelen : super::LPDWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHQueryValueExA(hkey : super::HKEY, pszvalue : windows_sys::core::PCSTR, pdwreserved : *mut super::DWORD, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHQueryValueExW(hkey : super::HKEY, pszvalue : windows_sys::core::PCWSTR, pdwreserved : *mut super::DWORD, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD) -> super::LSTATUS);
#[cfg(feature = "winreg")]
windows_link::link!("shlwapi.dll" "system" fn SHRegCloseUSKey(huskey : HUSKEY) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegCreateUSKeyA(pszpath : windows_sys::core::PCSTR, samdesired : super::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : PHUSKEY, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegCreateUSKeyW(pwzpath : windows_sys::core::PCWSTR, samdesired : super::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : PHUSKEY, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(feature = "winreg")]
windows_link::link!("shlwapi.dll" "system" fn SHRegDeleteEmptyUSKeyA(huskey : HUSKEY, pszsubkey : windows_sys::core::PCSTR, delregflags : SHREGDEL_FLAGS) -> super::LSTATUS);
#[cfg(feature = "winreg")]
windows_link::link!("shlwapi.dll" "system" fn SHRegDeleteEmptyUSKeyW(huskey : HUSKEY, pwzsubkey : windows_sys::core::PCWSTR, delregflags : SHREGDEL_FLAGS) -> super::LSTATUS);
#[cfg(feature = "winreg")]
windows_link::link!("shlwapi.dll" "system" fn SHRegDeleteUSValueA(huskey : HUSKEY, pszvalue : windows_sys::core::PCSTR, delregflags : SHREGDEL_FLAGS) -> super::LSTATUS);
#[cfg(feature = "winreg")]
windows_link::link!("shlwapi.dll" "system" fn SHRegDeleteUSValueW(huskey : HUSKEY, pwzvalue : windows_sys::core::PCWSTR, delregflags : SHREGDEL_FLAGS) -> super::LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("shlwapi.dll" "system" fn SHRegDuplicateHKey(hkey : super::HKEY) -> super::HKEY);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegEnumUSKeyA(huskey : HUSKEY, dwindex : super::DWORD, pszname : windows_sys::core::PSTR, pcchname : super::LPDWORD, enumregflags : SHREGENUM_FLAGS) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegEnumUSKeyW(huskey : HUSKEY, dwindex : super::DWORD, pwzname : windows_sys::core::PWSTR, pcchname : super::LPDWORD, enumregflags : SHREGENUM_FLAGS) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegEnumUSValueA(huskey : HUSKEY, dwindex : super::DWORD, pszvaluename : windows_sys::core::PSTR, pcchvaluename : super::LPDWORD, pdwtype : super::LPDWORD, pvdata : *mut core::ffi::c_void, pcbdata : super::LPDWORD, enumregflags : SHREGENUM_FLAGS) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegEnumUSValueW(huskey : HUSKEY, dwindex : super::DWORD, pszvaluename : windows_sys::core::PWSTR, pcchvaluename : super::LPDWORD, pdwtype : super::LPDWORD, pvdata : *mut core::ffi::c_void, pcbdata : super::LPDWORD, enumregflags : SHREGENUM_FLAGS) -> super::LSTATUS);
windows_link::link!("shlwapi.dll" "system" fn SHRegGetBoolUSValueA(pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, fignorehkcu : windows_sys::core::BOOL, fdefault : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn SHRegGetBoolUSValueW(pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, fignorehkcu : windows_sys::core::BOOL, fdefault : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetIntW(hk : super::HKEY, pwzkey : windows_sys::core::PCWSTR, idefault : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetPathA(hkey : super::HKEY, pcszsubkey : windows_sys::core::PCSTR, pcszvalue : windows_sys::core::PCSTR, pszpath : windows_sys::core::PSTR, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetPathW(hkey : super::HKEY, pcszsubkey : windows_sys::core::PCWSTR, pcszvalue : windows_sys::core::PCWSTR, pszpath : windows_sys::core::PWSTR, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetUSValueA(pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD, fignorehkcu : windows_sys::core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetUSValueW(pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD, fignorehkcu : windows_sys::core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetValueA(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, srrfflags : SRRF, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetValueFromHKCUHKLM(pwszkey : windows_sys::core::PCWSTR, pwszvalue : windows_sys::core::PCWSTR, srrfflags : SRRF, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegGetValueW(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, srrfflags : SRRF, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD) -> super::LSTATUS);
#[cfg(feature = "winreg")]
windows_link::link!("shlwapi.dll" "system" fn SHRegOpenUSKeyA(pszpath : windows_sys::core::PCSTR, samdesired : super::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : PHUSKEY, fignorehkcu : windows_sys::core::BOOL) -> super::LSTATUS);
#[cfg(feature = "winreg")]
windows_link::link!("shlwapi.dll" "system" fn SHRegOpenUSKeyW(pwzpath : windows_sys::core::PCWSTR, samdesired : super::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : PHUSKEY, fignorehkcu : windows_sys::core::BOOL) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegQueryInfoUSKeyA(huskey : HUSKEY, pcsubkeys : super::LPDWORD, pcchmaxsubkeylen : super::LPDWORD, pcvalues : super::LPDWORD, pcchmaxvaluenamelen : super::LPDWORD, enumregflags : SHREGENUM_FLAGS) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegQueryInfoUSKeyW(huskey : HUSKEY, pcsubkeys : super::LPDWORD, pcchmaxsubkeylen : super::LPDWORD, pcvalues : super::LPDWORD, pcchmaxvaluenamelen : super::LPDWORD, enumregflags : SHREGENUM_FLAGS) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegQueryUSValueA(huskey : HUSKEY, pszvalue : windows_sys::core::PCSTR, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD, fignorehkcu : windows_sys::core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegQueryUSValueW(huskey : HUSKEY, pszvalue : windows_sys::core::PCWSTR, pdwtype : *mut super::DWORD, pvdata : *mut core::ffi::c_void, pcbdata : *mut super::DWORD, fignorehkcu : windows_sys::core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegSetPathA(hkey : super::HKEY, pcszsubkey : windows_sys::core::PCSTR, pcszvalue : windows_sys::core::PCSTR, pcszpath : windows_sys::core::PCSTR, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegSetPathW(hkey : super::HKEY, pcszsubkey : windows_sys::core::PCWSTR, pcszvalue : windows_sys::core::PCWSTR, pcszpath : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegSetUSValueA(pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, dwtype : super::DWORD, pvdata : *const core::ffi::c_void, cbdata : super::DWORD, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegSetUSValueW(pwzsubkey : windows_sys::core::PCWSTR, pwzvalue : windows_sys::core::PCWSTR, dwtype : super::DWORD, pvdata : *const core::ffi::c_void, cbdata : super::DWORD, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegWriteUSValueA(huskey : HUSKEY, pszvalue : windows_sys::core::PCSTR, dwtype : super::DWORD, pvdata : *const core::ffi::c_void, cbdata : super::DWORD, dwflags : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHRegWriteUSValueW(huskey : HUSKEY, pwzvalue : windows_sys::core::PCWSTR, dwtype : super::DWORD, pvdata : *const core::ffi::c_void, cbdata : super::DWORD, dwflags : super::DWORD) -> super::LSTATUS);
windows_link::link!("shlwapi.dll" "system" fn SHReleaseThreadRef() -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHSendMessageBroadcastA(umsg : super::UINT, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("shlwapi.dll" "system" fn SHSendMessageBroadcastW(umsg : super::UINT, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
windows_link::link!("shlwapi.dll" "system" fn SHSetThreadRef(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHSetValueA(hkey : super::HKEY, pszsubkey : windows_sys::core::PCSTR, pszvalue : windows_sys::core::PCSTR, dwtype : super::DWORD, pvdata : super::LPCVOID, cbdata : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("shlwapi.dll" "system" fn SHSetValueW(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, dwtype : super::DWORD, pvdata : super::LPCVOID, cbdata : super::DWORD) -> super::LSTATUS);
#[cfg(all(feature = "guiddef", feature = "objidl"))]
windows_link::link!("shlwapi.dll" "system" fn SHSkipJunction(pbc : *mut core::ffi::c_void, pclsid : *const super::CLSID) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn SHStrDupA(psz : windows_sys::core::PCSTR, ppwsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn SHStrDupW(psz : windows_sys::core::PCWSTR, ppwsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn SHStripMneumonicA(pszmenu : windows_sys::core::PSTR) -> super::CHAR);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn SHStripMneumonicW(pszmenu : windows_sys::core::PWSTR) -> super::WCHAR);
windows_link::link!("shlwapi.dll" "system" fn SHUnicodeToAnsi(pwszsrc : windows_sys::core::PCWSTR, pszdst : windows_sys::core::PSTR, cchbuf : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn SHUnicodeToUnicode(pwzsrc : windows_sys::core::PCWSTR, pwzdst : windows_sys::core::PWSTR, cwchbuf : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn SHUnlockShared(pvdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn StrCSpnA(pszstr : windows_sys::core::PCSTR, pszset : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCSpnIA(pszstr : windows_sys::core::PCSTR, pszset : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCSpnIW(pszstr : windows_sys::core::PCWSTR, pszset : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCSpnW(pszstr : windows_sys::core::PCWSTR, pszset : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("kernel32.dll" "system" "lstrcatA" fn StrCatA(lpstring1 : windows_sys::core::PSTR, lpstring2 : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrCatBuffA(pszdest : windows_sys::core::PSTR, pszsrc : windows_sys::core::PCSTR, cchdestbuffsize : i32) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrCatBuffW(pszdest : windows_sys::core::PWSTR, pszsrc : windows_sys::core::PCWSTR, cchdestbuffsize : i32) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrCatChainW(pszdst : windows_sys::core::PWSTR, cchdst : super::DWORD, ichat : super::DWORD, pszsrc : windows_sys::core::PCWSTR) -> super::DWORD);
windows_link::link!("shlwapi.dll" "system" fn StrCatW(psz1 : windows_sys::core::PWSTR, psz2 : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrChrA(pszstart : windows_sys::core::PCSTR, wmatch : super::WORD) -> windows_sys::core::PSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrChrIA(pszstart : windows_sys::core::PCSTR, wmatch : super::WORD) -> windows_sys::core::PSTR);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn StrChrIW(pszstart : windows_sys::core::PCWSTR, wmatch : super::WCHAR) -> windows_sys::core::PWSTR);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn StrChrNIW(pszstart : windows_sys::core::PCWSTR, wmatch : super::WCHAR, cchmax : super::UINT) -> windows_sys::core::PWSTR);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shlwapi.dll" "system" fn StrChrNW(pszstart : windows_sys::core::PCWSTR, wmatch : super::WCHAR, cchmax : super::UINT) -> windows_sys::core::PWSTR);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn StrChrW(pszstart : windows_sys::core::PCWSTR, wmatch : super::WCHAR) -> windows_sys::core::PWSTR);
windows_link::link!("kernel32.dll" "system" "lstrcmpA" fn StrCmpA(lpstring1 : windows_sys::core::PCSTR, lpstring2 : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpCA(pszstr1 : windows_sys::core::PCSTR, pszstr2 : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpCW(pszstr1 : windows_sys::core::PCWSTR, pszstr2 : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("kernel32.dll" "system" "lstrcmpiA" fn StrCmpIA(lpstring1 : windows_sys::core::PCSTR, lpstring2 : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpICA(pszstr1 : windows_sys::core::PCSTR, pszstr2 : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpICW(pszstr1 : windows_sys::core::PCWSTR, pszstr2 : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpIW(psz1 : windows_sys::core::PCWSTR, psz2 : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpLogicalW(psz1 : windows_sys::core::PCWSTR, psz2 : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNA(psz1 : windows_sys::core::PCSTR, psz2 : windows_sys::core::PCSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNCA(pszstr1 : windows_sys::core::PCSTR, pszstr2 : windows_sys::core::PCSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNCW(pszstr1 : windows_sys::core::PCWSTR, pszstr2 : windows_sys::core::PCWSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNIA(psz1 : windows_sys::core::PCSTR, psz2 : windows_sys::core::PCSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNICA(pszstr1 : windows_sys::core::PCSTR, pszstr2 : windows_sys::core::PCSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNICW(pszstr1 : windows_sys::core::PCWSTR, pszstr2 : windows_sys::core::PCWSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNIW(psz1 : windows_sys::core::PCWSTR, psz2 : windows_sys::core::PCWSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpNW(psz1 : windows_sys::core::PCWSTR, psz2 : windows_sys::core::PCWSTR, nchar : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrCmpW(psz1 : windows_sys::core::PCWSTR, psz2 : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("kernel32.dll" "system" "lstrcpyA" fn StrCpyA(lpstring1 : windows_sys::core::PSTR, lpstring2 : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("kernel32.dll" "system" "lstrcpynA" fn StrCpyNA(lpstring1 : windows_sys::core::PSTR, lpstring2 : windows_sys::core::PCSTR, imaxlength : i32) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrCpyNW(pszdst : windows_sys::core::PWSTR, pszsrc : windows_sys::core::PCWSTR, cchmax : i32) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn StrCpyW(psz1 : windows_sys::core::PWSTR, psz2 : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn StrDupA(pszsrch : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrDupW(pszsrch : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFormatByteSize64A(qdw : super::LONGLONG, pszbuf : windows_sys::core::PSTR, cchbuf : super::UINT) -> windows_sys::core::PSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFormatByteSizeA(dw : super::DWORD, pszbuf : windows_sys::core::PSTR, cchbuf : super::UINT) -> windows_sys::core::PSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFormatByteSizeEx(ull : super::ULONGLONG, flags : SFBS_FLAGS, pszbuf : windows_sys::core::PWSTR, cchbuf : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFormatByteSizeW(qdw : super::LONGLONG, pszbuf : windows_sys::core::PWSTR, cchbuf : super::UINT) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFormatKBSizeA(qdw : super::LONGLONG, pszbuf : windows_sys::core::PSTR, cchbuf : super::UINT) -> windows_sys::core::PSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFormatKBSizeW(qdw : super::LONGLONG, pszbuf : windows_sys::core::PWSTR, cchbuf : super::UINT) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFromTimeIntervalA(pszout : windows_sys::core::PSTR, cchmax : super::UINT, dwtimems : super::DWORD, digits : i32) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrFromTimeIntervalW(pszout : windows_sys::core::PWSTR, cchmax : super::UINT, dwtimems : super::DWORD, digits : i32) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrIsIntlEqualA(fcasesens : windows_sys::core::BOOL, pszstring1 : windows_sys::core::PCSTR, pszstring2 : windows_sys::core::PCSTR, nchar : i32) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn StrIsIntlEqualW(fcasesens : windows_sys::core::BOOL, pszstring1 : windows_sys::core::PCWSTR, pszstring2 : windows_sys::core::PCWSTR, nchar : i32) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn StrNCatA(psz1 : windows_sys::core::PSTR, psz2 : windows_sys::core::PCSTR, cchmax : i32) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrNCatW(psz1 : windows_sys::core::PWSTR, psz2 : windows_sys::core::PCWSTR, cchmax : i32) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn StrPBrkA(psz : windows_sys::core::PCSTR, pszset : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrPBrkW(psz : windows_sys::core::PCWSTR, pszset : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrRChrA(pszstart : windows_sys::core::PCSTR, pszend : windows_sys::core::PCSTR, wmatch : super::WORD) -> windows_sys::core::PSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrRChrIA(pszstart : windows_sys::core::PCSTR, pszend : windows_sys::core::PCSTR, wmatch : super::WORD) -> windows_sys::core::PSTR);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn StrRChrIW(pszstart : windows_sys::core::PCWSTR, pszend : windows_sys::core::PCWSTR, wmatch : super::WCHAR) -> windows_sys::core::PWSTR);
#[cfg(feature = "winnt")]
windows_link::link!("shlwapi.dll" "system" fn StrRChrW(pszstart : windows_sys::core::PCWSTR, pszend : windows_sys::core::PCWSTR, wmatch : super::WCHAR) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn StrRStrIA(pszsource : windows_sys::core::PCSTR, pszlast : windows_sys::core::PCSTR, pszsrch : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrRStrIW(pszsource : windows_sys::core::PCWSTR, pszlast : windows_sys::core::PCWSTR, pszsrch : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn StrRetToBSTR(pstr : *mut super::STRRET, pidl : super::LPCITEMIDLIST, pbstr : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn StrRetToBufA(pstr : *mut super::STRRET, pidl : super::LPCITEMIDLIST, pszbuf : windows_sys::core::PSTR, cchbuf : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn StrRetToBufW(pstr : *mut super::STRRET, pidl : super::LPCITEMIDLIST, pszbuf : windows_sys::core::PWSTR, cchbuf : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn StrRetToStrA(pstr : *mut super::STRRET, pidl : super::LPCITEMIDLIST, ppsz : *mut windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shlwapi.dll" "system" fn StrRetToStrW(pstr : *mut super::STRRET, pidl : super::LPCITEMIDLIST, ppsz : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn StrSpnA(psz : windows_sys::core::PCSTR, pszset : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrSpnW(psz : windows_sys::core::PCWSTR, pszset : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrStrA(pszfirst : windows_sys::core::PCSTR, pszsrch : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrStrIA(pszfirst : windows_sys::core::PCSTR, pszsrch : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("shlwapi.dll" "system" fn StrStrIW(pszfirst : windows_sys::core::PCWSTR, pszsrch : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrStrNIW(pszfirst : windows_sys::core::PCWSTR, pszsrch : windows_sys::core::PCWSTR, cchmax : super::UINT) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrStrNW(pszfirst : windows_sys::core::PCWSTR, pszsrch : windows_sys::core::PCWSTR, cchmax : super::UINT) -> windows_sys::core::PWSTR);
windows_link::link!("shlwapi.dll" "system" fn StrStrW(pszfirst : windows_sys::core::PCWSTR, pszsrch : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrToInt64ExA(pszstring : windows_sys::core::PCSTR, dwflags : STIF_FLAGS, pllret : *mut super::LONGLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn StrToInt64ExW(pszstring : windows_sys::core::PCWSTR, dwflags : STIF_FLAGS, pllret : *mut super::LONGLONG) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn StrToIntA(pszsrc : windows_sys::core::PCSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrToIntExA(pszstring : windows_sys::core::PCSTR, dwflags : STIF_FLAGS, piret : *mut i32) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn StrToIntExW(pszstring : windows_sys::core::PCWSTR, dwflags : STIF_FLAGS, piret : *mut i32) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn StrToIntW(pszsrc : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("shlwapi.dll" "system" fn StrTrimA(psz : windows_sys::core::PSTR, psztrimchars : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn StrTrimW(psz : windows_sys::core::PWSTR, psztrimchars : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlApplySchemeA(pszin : windows_sys::core::PCSTR, pszout : windows_sys::core::PSTR, pcchout : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlApplySchemeW(pszin : windows_sys::core::PCWSTR, pszout : windows_sys::core::PWSTR, pcchout : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlCanonicalizeA(pszurl : windows_sys::core::PCSTR, pszcanonicalized : windows_sys::core::PSTR, pcchcanonicalized : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlCanonicalizeW(pszurl : windows_sys::core::PCWSTR, pszcanonicalized : windows_sys::core::PWSTR, pcchcanonicalized : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlCombineA(pszbase : windows_sys::core::PCSTR, pszrelative : windows_sys::core::PCSTR, pszcombined : windows_sys::core::PSTR, pcchcombined : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlCombineW(pszbase : windows_sys::core::PCWSTR, pszrelative : windows_sys::core::PCWSTR, pszcombined : windows_sys::core::PWSTR, pcchcombined : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn UrlCompareA(psz1 : windows_sys::core::PCSTR, psz2 : windows_sys::core::PCSTR, fignoreslash : windows_sys::core::BOOL) -> i32);
windows_link::link!("shlwapi.dll" "system" fn UrlCompareW(psz1 : windows_sys::core::PCWSTR, psz2 : windows_sys::core::PCWSTR, fignoreslash : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlCreateFromPathA(pszpath : windows_sys::core::PCSTR, pszurl : windows_sys::core::PSTR, pcchurl : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlCreateFromPathW(pszpath : windows_sys::core::PCWSTR, pszurl : windows_sys::core::PWSTR, pcchurl : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlEscapeA(pszurl : windows_sys::core::PCSTR, pszescaped : windows_sys::core::PSTR, pcchescaped : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlEscapeW(pszurl : windows_sys::core::PCWSTR, pszescaped : windows_sys::core::PWSTR, pcchescaped : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlFixupW(pcszurl : windows_sys::core::PCWSTR, psztranslatedurl : windows_sys::core::PWSTR, cchmax : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn UrlGetLocationA(pszurl : windows_sys::core::PCSTR) -> windows_sys::core::PCSTR);
windows_link::link!("shlwapi.dll" "system" fn UrlGetLocationW(pszurl : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlGetPartA(pszin : windows_sys::core::PCSTR, pszout : windows_sys::core::PSTR, pcchout : *mut super::DWORD, dwpart : super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlGetPartW(pszin : windows_sys::core::PCWSTR, pszout : windows_sys::core::PWSTR, pcchout : *mut super::DWORD, dwpart : super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlHashA(pszurl : windows_sys::core::PCSTR, pbhash : *mut super::BYTE, cbhash : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlHashW(pszurl : windows_sys::core::PCWSTR, pbhash : *mut super::BYTE, cbhash : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("shlwapi.dll" "system" fn UrlIsA(pszurl : windows_sys::core::PCSTR, urlis : URLIS) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn UrlIsNoHistoryA(pszurl : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn UrlIsNoHistoryW(pszurl : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn UrlIsOpaqueA(pszurl : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn UrlIsOpaqueW(pszurl : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shlwapi.dll" "system" fn UrlIsW(pszurl : windows_sys::core::PCWSTR, urlis : URLIS) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlUnescapeA(pszurl : windows_sys::core::PSTR, pszunescaped : windows_sys::core::PSTR, pcchunescaped : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn UrlUnescapeW(pszurl : windows_sys::core::PWSTR, pszunescaped : windows_sys::core::PWSTR, pcchunescaped : *mut super::DWORD, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn WhichPlatform() -> super::UINT);
windows_link::link!("shlwapi.dll" "C" fn wnsprintfA(pszdest : windows_sys::core::PSTR, cchdest : i32, pszfmt : windows_sys::core::PCSTR, ...) -> i32);
windows_link::link!("shlwapi.dll" "C" fn wnsprintfW(pszdest : windows_sys::core::PWSTR, cchdest : i32, pszfmt : windows_sys::core::PCWSTR, ...) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn wvnsprintfA(pszdest : windows_sys::core::PSTR, cchdest : i32, pszfmt : windows_sys::core::PCSTR, arglist : super::va_list) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shlwapi.dll" "system" fn wvnsprintfW(pszdest : windows_sys::core::PWSTR, cchdest : i32, pszfmt : windows_sys::core::PCWSTR, arglist : super::va_list) -> i32);
pub type ASSOCDATA = i32;
pub const ASSOCDATA_EDITFLAGS: ASSOCDATA = 5;
pub const ASSOCDATA_HASPERUSERASSOC: ASSOCDATA = 4;
pub const ASSOCDATA_MAX: ASSOCDATA = 7;
pub const ASSOCDATA_MSIDESCRIPTOR: ASSOCDATA = 1;
pub const ASSOCDATA_NOACTIVATEHANDLER: ASSOCDATA = 2;
pub const ASSOCDATA_UNUSED1: ASSOCDATA = 3;
pub const ASSOCDATA_VALUE: ASSOCDATA = 6;
pub type ASSOCENUM = i32;
pub const ASSOCENUM_NONE: ASSOCENUM = 0;
pub type ASSOCF = u32;
pub const ASSOCF_APP_TO_APP: i32 = 65536;
pub const ASSOCF_IGNOREBASECLASS: i32 = 512;
pub const ASSOCF_INIT_BYEXENAME: i32 = 2;
pub const ASSOCF_INIT_DEFAULTTOFOLDER: i32 = 8;
pub const ASSOCF_INIT_DEFAULTTOSTAR: i32 = 4;
pub const ASSOCF_INIT_FIXED_PROGID: i32 = 2048;
pub const ASSOCF_INIT_FOR_FILE: i32 = 8192;
pub const ASSOCF_INIT_IGNOREUNKNOWN: i32 = 1024;
pub const ASSOCF_INIT_NOREMAPCLSID: i32 = 1;
pub const ASSOCF_IS_FULL_URI: i32 = 16384;
pub const ASSOCF_IS_PROTOCOL: i32 = 4096;
pub const ASSOCF_NOFIXUPS: i32 = 256;
pub const ASSOCF_NONE: i32 = 0;
pub const ASSOCF_NOTRUNCATE: i32 = 32;
pub const ASSOCF_NOUSERSETTINGS: i32 = 16;
pub const ASSOCF_OPEN_BYEXENAME: i32 = 2;
pub const ASSOCF_PER_MACHINE_ONLY: i32 = 32768;
pub const ASSOCF_REMAPRUNDLL: i32 = 128;
pub const ASSOCF_VERIFY: i32 = 64;
pub type ASSOCKEY = i32;
pub const ASSOCKEY_APP: ASSOCKEY = 2;
pub const ASSOCKEY_BASECLASS: ASSOCKEY = 4;
pub const ASSOCKEY_CLASS: ASSOCKEY = 3;
pub const ASSOCKEY_MAX: ASSOCKEY = 5;
pub const ASSOCKEY_SHELLEXECCLASS: ASSOCKEY = 1;
pub type ASSOCSTR = i32;
pub const ASSOCSTR_APPICONREFERENCE: ASSOCSTR = 23;
pub const ASSOCSTR_APPID: ASSOCSTR = 21;
pub const ASSOCSTR_APPPUBLISHER: ASSOCSTR = 22;
pub const ASSOCSTR_COMMAND: ASSOCSTR = 1;
pub const ASSOCSTR_CONTENTTYPE: ASSOCSTR = 14;
pub const ASSOCSTR_DDEAPPLICATION: ASSOCSTR = 9;
pub const ASSOCSTR_DDECOMMAND: ASSOCSTR = 7;
pub const ASSOCSTR_DDEIFEXEC: ASSOCSTR = 8;
pub const ASSOCSTR_DDETOPIC: ASSOCSTR = 10;
pub const ASSOCSTR_DEFAULTICON: ASSOCSTR = 15;
pub const ASSOCSTR_DELEGATEEXECUTE: ASSOCSTR = 18;
pub const ASSOCSTR_DROPTARGET: ASSOCSTR = 17;
pub const ASSOCSTR_EXECUTABLE: ASSOCSTR = 2;
pub const ASSOCSTR_FRIENDLYAPPNAME: ASSOCSTR = 4;
pub const ASSOCSTR_FRIENDLYDOCNAME: ASSOCSTR = 3;
pub const ASSOCSTR_INFOTIP: ASSOCSTR = 11;
pub const ASSOCSTR_MAX: ASSOCSTR = 24;
pub const ASSOCSTR_NOOPEN: ASSOCSTR = 5;
pub const ASSOCSTR_PROGID: ASSOCSTR = 20;
pub const ASSOCSTR_QUICKTIP: ASSOCSTR = 12;
pub const ASSOCSTR_SHELLEXTENSION: ASSOCSTR = 16;
pub const ASSOCSTR_SHELLNEWVALUE: ASSOCSTR = 6;
pub const ASSOCSTR_SUPPORTED_URI_PROTOCOLS: ASSOCSTR = 19;
pub const ASSOCSTR_TILEINFO: ASSOCSTR = 13;
pub const CTF_COINIT: i32 = 8;
pub const CTF_COINIT_MTA: i32 = 4096;
pub const CTF_COINIT_STA: i32 = 8;
pub const CTF_FREELIBANDEXIT: i32 = 16;
pub const CTF_INHERITWOW64: i32 = 256;
pub const CTF_INSIST: i32 = 1;
pub const CTF_KEYBOARD_LOCALE: i32 = 1024;
pub const CTF_NOADDREFLIB: i32 = 8192;
pub const CTF_OLEINITIALIZE: i32 = 2048;
pub const CTF_PROCESS_REF: i32 = 4;
pub const CTF_REF_COUNTED: i32 = 32;
pub const CTF_THREAD_REF: i32 = 2;
pub const CTF_UNUSED: i32 = 128;
pub const CTF_WAIT_ALLOWCOM: i32 = 64;
pub const CTF_WAIT_NO_REENTRANCY: i32 = 512;
#[cfg(feature = "shared")]
pub type DLLGETVERSIONPROC = Option<unsafe extern "system" fn(param0: *mut DLLVERSIONINFO) -> windows_sys::core::HRESULT>;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DLLVERSIONINFO {
    pub cbSize: super::DWORD,
    pub dwMajorVersion: super::DWORD,
    pub dwMinorVersion: super::DWORD,
    pub dwBuildNumber: super::DWORD,
    pub dwPlatformID: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DLLVERSIONINFO2 {
    pub info1: DLLVERSIONINFO,
    pub dwFlags: super::DWORD,
    pub ullVersion: super::ULONGLONG,
}
pub const DLLVER_BUILD_MASK: u32 = 4294901760;
pub const DLLVER_MAJOR_MASK: u64 = 18446462598732840960;
pub const DLLVER_MINOR_MASK: i64 = 281470681743360;
pub const DLLVER_PLATFORM_NT: i32 = 2;
pub const DLLVER_PLATFORM_WINDOWS: i32 = 1;
pub const DLLVER_QFE_MASK: i32 = 65535;
pub const FDTF_DEFAULT: i32 = 3;
pub const FDTF_LONGDATE: i32 = 4;
pub const FDTF_LONGTIME: i32 = 8;
pub const FDTF_LTRDATE: i32 = 256;
pub const FDTF_NOAUTOREADINGORDER: i32 = 1024;
pub const FDTF_RELATIVE: i32 = 16;
pub const FDTF_RTLDATE: i32 = 512;
pub const FDTF_SHORTDATE: i32 = 2;
pub const FDTF_SHORTTIME: i32 = 1;
pub type FILETYPEATTRIBUTEFLAGS = i32;
pub const FTA_AlwaysUnsafe: FILETYPEATTRIBUTEFLAGS = 131072;
pub const FTA_AlwaysUseDirectInvoke: FILETYPEATTRIBUTEFLAGS = 4194304;
pub const FTA_Exclude: FILETYPEATTRIBUTEFLAGS = 1;
pub const FTA_HasExtension: FILETYPEATTRIBUTEFLAGS = 4;
pub const FTA_NoDDE: FILETYPEATTRIBUTEFLAGS = 8192;
pub const FTA_NoEdit: FILETYPEATTRIBUTEFLAGS = 8;
pub const FTA_NoEditDesc: FILETYPEATTRIBUTEFLAGS = 256;
pub const FTA_NoEditDflt: FILETYPEATTRIBUTEFLAGS = 1024;
pub const FTA_NoEditIcon: FILETYPEATTRIBUTEFLAGS = 512;
pub const FTA_NoEditMIME: FILETYPEATTRIBUTEFLAGS = 32768;
pub const FTA_NoEditVerb: FILETYPEATTRIBUTEFLAGS = 64;
pub const FTA_NoEditVerbCmd: FILETYPEATTRIBUTEFLAGS = 2048;
pub const FTA_NoEditVerbExe: FILETYPEATTRIBUTEFLAGS = 4096;
pub const FTA_NoNewVerb: FILETYPEATTRIBUTEFLAGS = 32;
pub const FTA_NoRecentDocs: FILETYPEATTRIBUTEFLAGS = 1048576;
pub const FTA_NoRemove: FILETYPEATTRIBUTEFLAGS = 16;
pub const FTA_NoRemoveVerb: FILETYPEATTRIBUTEFLAGS = 128;
pub const FTA_None: FILETYPEATTRIBUTEFLAGS = 0;
pub const FTA_OpenIsSafe: FILETYPEATTRIBUTEFLAGS = 65536;
pub const FTA_SafeForElevation: FILETYPEATTRIBUTEFLAGS = 2097152;
pub const FTA_Show: FILETYPEATTRIBUTEFLAGS = 2;
pub const GCT_INVALID: i32 = 0;
pub const GCT_LFNCHAR: i32 = 1;
pub const GCT_SEPARATOR: i32 = 8;
pub const GCT_SHORTCHAR: i32 = 2;
pub const GCT_WILD: i32 = 4;
pub const GLOBALCOUNTER_APPLICATION_DESTINATIONS: SHGLOBALCOUNTER = 12;
pub const GLOBALCOUNTER_APPROVEDSITES: SHGLOBALCOUNTER = 4;
pub const GLOBALCOUNTER_APPSFOLDER_FILETYPEASSOCIATION_COUNTER: SHGLOBALCOUNTER = 55;
pub const GLOBALCOUNTER_APP_ITEMS_STATE_STORE_CACHE: SHGLOBALCOUNTER = 53;
pub const GLOBALCOUNTER_ASSOCCHANGED: SHGLOBALCOUNTER = 52;
pub const GLOBALCOUNTER_BANNERS_DATAMODEL_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 58;
pub const GLOBALCOUNTER_BITBUCKETNUMDELETERS: SHGLOBALCOUNTER = 14;
pub const GLOBALCOUNTER_COMMONPLACES_LIST_CACHE: SHGLOBALCOUNTER = 50;
pub const GLOBALCOUNTER_FOLDERDEFINITION_CACHE: SHGLOBALCOUNTER = 49;
pub const GLOBALCOUNTER_FOLDERSETTINGSCHANGE: SHGLOBALCOUNTER = 2;
pub const GLOBALCOUNTER_IEONLY_SESSIONS: SHGLOBALCOUNTER = 11;
pub const GLOBALCOUNTER_IESESSIONS: SHGLOBALCOUNTER = 10;
pub const GLOBALCOUNTER_INTERNETTOOLBAR_LAYOUT: SHGLOBALCOUNTER = 48;
pub const GLOBALCOUNTER_MAXIMUMVALUE: SHGLOBALCOUNTER = 59;
pub const GLOBALCOUNTER_OVERLAYMANAGER: SHGLOBALCOUNTER = 8;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE: SHGLOBALCOUNTER = 47;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 51;
pub const GLOBALCOUNTER_QUERYASSOCIATIONS: SHGLOBALCOUNTER = 9;
pub const GLOBALCOUNTER_RATINGS: SHGLOBALCOUNTER = 3;
pub const GLOBALCOUNTER_RATINGS_STATECOUNTER: SHGLOBALCOUNTER = 46;
pub const GLOBALCOUNTER_RECYCLEBINCORRUPTED: SHGLOBALCOUNTER = 45;
pub const GLOBALCOUNTER_RECYCLEBINENUM: SHGLOBALCOUNTER = 44;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_A: SHGLOBALCOUNTER = 16;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_B: SHGLOBALCOUNTER = 17;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_C: SHGLOBALCOUNTER = 18;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_D: SHGLOBALCOUNTER = 19;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_E: SHGLOBALCOUNTER = 20;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_F: SHGLOBALCOUNTER = 21;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_G: SHGLOBALCOUNTER = 22;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_H: SHGLOBALCOUNTER = 23;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_I: SHGLOBALCOUNTER = 24;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_J: SHGLOBALCOUNTER = 25;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_K: SHGLOBALCOUNTER = 26;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_L: SHGLOBALCOUNTER = 27;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_M: SHGLOBALCOUNTER = 28;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_N: SHGLOBALCOUNTER = 29;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_O: SHGLOBALCOUNTER = 30;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_P: SHGLOBALCOUNTER = 31;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Q: SHGLOBALCOUNTER = 32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_R: SHGLOBALCOUNTER = 33;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_S: SHGLOBALCOUNTER = 34;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_T: SHGLOBALCOUNTER = 35;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_U: SHGLOBALCOUNTER = 36;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_V: SHGLOBALCOUNTER = 37;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_W: SHGLOBALCOUNTER = 38;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_X: SHGLOBALCOUNTER = 39;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Y: SHGLOBALCOUNTER = 40;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Z: SHGLOBALCOUNTER = 41;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SHARES: SHGLOBALCOUNTER = 15;
pub const GLOBALCOUNTER_RESTRICTIONS: SHGLOBALCOUNTER = 5;
pub const GLOBALCOUNTER_SEARCHMANAGER: SHGLOBALCOUNTER = 0;
pub const GLOBALCOUNTER_SEARCHOPTIONS: SHGLOBALCOUNTER = 1;
pub const GLOBALCOUNTER_SETTINGSYNC_ENABLED: SHGLOBALCOUNTER = 54;
pub const GLOBALCOUNTER_SHELLSETTINGSCHANGED: SHGLOBALCOUNTER = 6;
pub const GLOBALCOUNTER_SYNC_ENGINE_INFORMATION_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 57;
pub const GLOBALCOUNTER_SYSTEMPIDLCHANGE: SHGLOBALCOUNTER = 7;
pub const GLOBALCOUNTER_USERINFOCHANGED: SHGLOBALCOUNTER = 56;
pub type HUSKEY = *mut core::ffi::c_void;
pub const ILMM_IE4: i32 = 0;
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub type LPCQITAB = *const QITAB;
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub type LPQITAB = *mut QITAB;
pub const OS_ADVSERVER: i32 = 22;
pub const OS_ANYSERVER: i32 = 29;
pub const OS_APPLIANCE: i32 = 36;
pub const OS_DATACENTER: i32 = 21;
pub const OS_DOMAINMEMBER: i32 = 28;
pub const OS_EMBEDDED: i32 = 13;
pub const OS_FASTUSERSWITCHING: i32 = 26;
pub const OS_HOME: i32 = 19;
pub const OS_MEDIACENTER: i32 = 35;
pub const OS_MEORGREATER: i32 = 17;
pub const OS_NT: i32 = 1;
pub const OS_NT4ORGREATER: i32 = 3;
pub const OS_PERSONALTERMINALSERVER: i32 = 25;
pub const OS_PROFESSIONAL: i32 = 20;
pub const OS_SERVER: i32 = 23;
pub const OS_SERVERADMINUI: i32 = 34;
pub const OS_SMALLBUSINESSSERVER: i32 = 32;
pub const OS_TABLETPC: i32 = 33;
pub const OS_TERMINALCLIENT: i32 = 14;
pub const OS_TERMINALREMOTEADMIN: i32 = 15;
pub const OS_TERMINALSERVER: i32 = 24;
pub const OS_WEBSERVER: i32 = 31;
pub const OS_WELCOMELOGONUI: i32 = 27;
pub const OS_WIN2000ADVSERVER: i32 = 10;
pub const OS_WIN2000DATACENTER: i32 = 11;
pub const OS_WIN2000ORGREATER: i32 = 7;
pub const OS_WIN2000PRO: i32 = 8;
pub const OS_WIN2000SERVER: i32 = 9;
pub const OS_WIN2000TERMINAL: i32 = 12;
pub const OS_WIN95ORGREATER: i32 = 2;
pub const OS_WIN95_GOLD: i32 = 16;
pub const OS_WIN98ORGREATER: i32 = 5;
pub const OS_WIN98_GOLD: i32 = 6;
pub const OS_WINDOWS: i32 = 0;
pub const OS_WOW6432: i32 = 30;
pub const OS_XPORGREATER: i32 = 18;
#[cfg(feature = "shared")]
pub type PARSEDURL = PARSEDURLA;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PARSEDURLA {
    pub cbSize: super::DWORD,
    pub pszProtocol: windows_sys::core::PCSTR,
    pub cchProtocol: super::UINT,
    pub pszSuffix: windows_sys::core::PCSTR,
    pub cchSuffix: super::UINT,
    pub nScheme: super::UINT,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PARSEDURLW {
    pub cbSize: super::DWORD,
    pub pszProtocol: windows_sys::core::PCWSTR,
    pub cchProtocol: super::UINT,
    pub pszSuffix: windows_sys::core::PCWSTR,
    pub cchSuffix: super::UINT,
    pub nScheme: super::UINT,
}
pub type PHUSKEY = *mut *mut core::ffi::c_void;
pub const PLATFORM_BROWSERONLY: i32 = 1;
pub const PLATFORM_IE3: i32 = 1;
pub const PLATFORM_INTEGRATED: i32 = 2;
pub const PLATFORM_UNKNOWN: i32 = 0;
pub const PMSF_DONT_STRIP_SPACES: i32 = 65536;
pub const PMSF_MULTIPLE: i32 = 1;
pub const PMSF_NORMAL: i32 = 0;
#[cfg(feature = "shared")]
pub type PPARSEDURL = *mut PARSEDURLA;
#[cfg(feature = "shared")]
pub type PPARSEDURLA = *mut PARSEDURLA;
#[cfg(feature = "shared")]
pub type PPARSEDURLW = *mut PARSEDURLW;
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct QITAB {
    pub piid: *const super::IID,
    pub dwOffset: super::DWORD,
}
pub type SFBS_FLAGS = i32;
pub const SFBS_FLAGS_ROUND_TO_NEAREST_DISPLAYED_DIGIT: tagSFBS_FLAGS = 1;
pub const SFBS_FLAGS_TRUNCATE_UNDISPLAYED_DECIMAL_DIGITS: tagSFBS_FLAGS = 2;
pub const SHACF_AUTOAPPEND_FORCE_OFF: u32 = 2147483648;
pub const SHACF_AUTOAPPEND_FORCE_ON: i32 = 1073741824;
pub const SHACF_AUTOSUGGEST_FORCE_OFF: i32 = 536870912;
pub const SHACF_AUTOSUGGEST_FORCE_ON: i32 = 268435456;
pub const SHACF_DEFAULT: i32 = 0;
pub const SHACF_FILESYSTEM: i32 = 1;
pub const SHACF_FILESYS_DIRS: i32 = 32;
pub const SHACF_FILESYS_ONLY: i32 = 16;
pub const SHACF_URLALL: i32 = 6;
pub const SHACF_URLHISTORY: i32 = 2;
pub const SHACF_URLMRU: i32 = 4;
pub const SHACF_USETAB: i32 = 8;
pub const SHACF_VIRTUAL_NAMESPACE: i32 = 64;
pub type SHCT_FLAGS = u32;
pub type SHGLOBALCOUNTER = i32;
pub const SHGVSPB_ALLFOLDERS: i32 = 8;
pub const SHGVSPB_ALLUSERS: i32 = 2;
pub const SHGVSPB_FOLDER: i32 = 5;
pub const SHGVSPB_FOLDERNODEFAULTS: u32 = 2147483653;
pub const SHGVSPB_GLOBALDEFAULTS: i32 = 10;
pub const SHGVSPB_INHERIT: i32 = 16;
pub const SHGVSPB_NOAUTODEFAULTS: u32 = 2147483648;
pub const SHGVSPB_PERFOLDER: i32 = 4;
pub const SHGVSPB_PERUSER: i32 = 1;
pub const SHGVSPB_ROAM: i32 = 32;
pub const SHGVSPB_USERDEFAULTS: i32 = 9;
pub const SHREGDEL_BOTH: SHREGDEL_FLAGS = 17;
pub const SHREGDEL_DEFAULT: SHREGDEL_FLAGS = 0;
pub type SHREGDEL_FLAGS = i32;
pub const SHREGDEL_HKCU: SHREGDEL_FLAGS = 1;
pub const SHREGDEL_HKLM: SHREGDEL_FLAGS = 16;
pub const SHREGENUM_BOTH: SHREGENUM_FLAGS = 17;
pub const SHREGENUM_DEFAULT: SHREGENUM_FLAGS = 0;
pub type SHREGENUM_FLAGS = i32;
pub const SHREGENUM_HKCU: SHREGENUM_FLAGS = 1;
pub const SHREGENUM_HKLM: SHREGENUM_FLAGS = 16;
pub const SHREGSET_DEFAULT: i32 = 6;
pub const SHREGSET_FORCE_HKCU: i32 = 2;
pub const SHREGSET_FORCE_HKLM: i32 = 8;
pub const SHREGSET_HKCU: i32 = 1;
pub const SHREGSET_HKLM: i32 = 4;
pub type SRRF = i32;
pub const SRRF_NOEXPAND: i32 = 268435456;
pub const SRRF_NOVIRT: i32 = 1073741824;
pub const SRRF_RM_ANY: i32 = 0;
pub const SRRF_RM_NORMAL: i32 = 65536;
pub const SRRF_RM_SAFE: i32 = 131072;
pub const SRRF_RM_SAFENETWORK: i32 = 262144;
pub const SRRF_RT_ANY: i32 = 65535;
pub const SRRF_RT_DWORD: i32 = 24;
pub const SRRF_RT_QWORD: i32 = 72;
pub const SRRF_RT_REG_BINARY: i32 = 8;
pub const SRRF_RT_REG_DWORD: i32 = 16;
pub const SRRF_RT_REG_EXPAND_SZ: i32 = 4;
pub const SRRF_RT_REG_MULTI_SZ: i32 = 32;
pub const SRRF_RT_REG_NONE: i32 = 1;
pub const SRRF_RT_REG_QWORD: i32 = 64;
pub const SRRF_RT_REG_SZ: i32 = 2;
pub const SRRF_ZEROONFAILURE: i32 = 536870912;
pub const STIF_DEFAULT: i32 = 0;
pub type STIF_FLAGS = i32;
pub const STIF_SUPPORT_HEX: i32 = 1;
pub const SZ_CONTENTTYPE_CDF: windows_sys::core::PCSTR = windows_sys::core::s!("application/x-cdf");
pub const SZ_CONTENTTYPE_CDFA: windows_sys::core::PCSTR = windows_sys::core::s!("application/x-cdf");
pub const SZ_CONTENTTYPE_CDFW: windows_sys::core::PCWSTR = windows_sys::core::w!("application/x-cdf");
pub const SZ_CONTENTTYPE_HTML: windows_sys::core::PCSTR = windows_sys::core::s!("text/html");
pub const SZ_CONTENTTYPE_HTMLA: windows_sys::core::PCSTR = windows_sys::core::s!("text/html");
pub const SZ_CONTENTTYPE_HTMLW: windows_sys::core::PCWSTR = windows_sys::core::w!("text/html");
pub type URLIS = i32;
pub const URLIS_APPLIABLE: URLIS = 4;
pub const URLIS_DIRECTORY: URLIS = 5;
pub const URLIS_FILEURL: URLIS = 3;
pub const URLIS_HASQUERY: URLIS = 6;
pub const URLIS_NOHISTORY: URLIS = 2;
pub const URLIS_OPAQUE: URLIS = 1;
pub const URLIS_URL: URLIS = 0;
pub const URL_APPLY_DEFAULT: i32 = 1;
pub const URL_APPLY_FORCEAPPLY: i32 = 8;
pub const URL_APPLY_GUESSFILE: i32 = 4;
pub const URL_APPLY_GUESSSCHEME: i32 = 2;
pub const URL_BROWSER_MODE: i32 = 33554432;
pub const URL_CONVERT_IF_DOSPATH: i32 = 2097152;
pub const URL_DONT_ESCAPE_EXTRA_INFO: i32 = 33554432;
pub const URL_DONT_SIMPLIFY: i32 = 134217728;
pub const URL_DONT_UNESCAPE: i32 = 131072;
pub const URL_DONT_UNESCAPE_EXTRA_INFO: i32 = 33554432;
pub const URL_ESCAPE_ASCII_URI_COMPONENT: i32 = 524288;
pub const URL_ESCAPE_AS_UTF8: i32 = 262144;
pub const URL_ESCAPE_PERCENT: i32 = 4096;
pub const URL_ESCAPE_SEGMENT_ONLY: i32 = 8192;
pub const URL_ESCAPE_SPACES_ONLY: i32 = 67108864;
pub const URL_ESCAPE_UNSAFE: i32 = 536870912;
pub const URL_ESCAPE_URI_COMPONENT: i32 = 786432;
pub const URL_FILE_USE_PATHURL: i32 = 65536;
pub const URL_INTERNAL_PATH: i32 = 8388608;
pub const URL_NO_META: i32 = 134217728;
pub type URL_PART = i32;
pub const URL_PARTFLAG_KEEPSCHEME: i32 = 1;
pub const URL_PART_HOSTNAME: URL_PART = 2;
pub const URL_PART_NONE: URL_PART = 0;
pub const URL_PART_PASSWORD: URL_PART = 4;
pub const URL_PART_PORT: URL_PART = 5;
pub const URL_PART_QUERY: URL_PART = 6;
pub const URL_PART_SCHEME: URL_PART = 1;
pub const URL_PART_USERNAME: URL_PART = 3;
pub const URL_PLUGGABLE_PROTOCOL: i32 = 1073741824;
pub type URL_SCHEME = i32;
pub const URL_SCHEME_ABOUT: URL_SCHEME = 17;
pub const URL_SCHEME_FILE: URL_SCHEME = 9;
pub const URL_SCHEME_FTP: URL_SCHEME = 1;
pub const URL_SCHEME_GOPHER: URL_SCHEME = 3;
pub const URL_SCHEME_HTTP: URL_SCHEME = 2;
pub const URL_SCHEME_HTTPS: URL_SCHEME = 11;
pub const URL_SCHEME_INVALID: URL_SCHEME = -1;
pub const URL_SCHEME_JAVASCRIPT: URL_SCHEME = 15;
pub const URL_SCHEME_KNOWNFOLDER: URL_SCHEME = 26;
pub const URL_SCHEME_LOCAL: URL_SCHEME = 14;
pub const URL_SCHEME_MAILTO: URL_SCHEME = 4;
pub const URL_SCHEME_MAXVALUE: URL_SCHEME = 27;
pub const URL_SCHEME_MK: URL_SCHEME = 10;
pub const URL_SCHEME_MSHELP: URL_SCHEME = 21;
pub const URL_SCHEME_MSSHELLDEVICE: URL_SCHEME = 22;
pub const URL_SCHEME_MSSHELLIDLIST: URL_SCHEME = 20;
pub const URL_SCHEME_MSSHELLROOTED: URL_SCHEME = 19;
pub const URL_SCHEME_NEWS: URL_SCHEME = 5;
pub const URL_SCHEME_NNTP: URL_SCHEME = 6;
pub const URL_SCHEME_RES: URL_SCHEME = 18;
pub const URL_SCHEME_SEARCH: URL_SCHEME = 25;
pub const URL_SCHEME_SEARCH_MS: URL_SCHEME = 24;
pub const URL_SCHEME_SHELL: URL_SCHEME = 12;
pub const URL_SCHEME_SNEWS: URL_SCHEME = 13;
pub const URL_SCHEME_TELNET: URL_SCHEME = 7;
pub const URL_SCHEME_UNKNOWN: URL_SCHEME = 0;
pub const URL_SCHEME_VBSCRIPT: URL_SCHEME = 16;
pub const URL_SCHEME_WAIS: URL_SCHEME = 8;
pub const URL_SCHEME_WILDCARD: URL_SCHEME = 23;
pub const URL_UNESCAPE: i32 = 268435456;
pub const URL_UNESCAPE_AS_UTF8: i32 = 262144;
pub const URL_UNESCAPE_HIGH_ANSI_ONLY: i32 = 4194304;
pub const URL_UNESCAPE_INPLACE: i32 = 1048576;
pub const URL_UNESCAPE_URI_COMPONENT: i32 = 262144;
pub const URL_WININET_COMPATIBILITY: u32 = 2147483648;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_CSCSYNCINPROGRESS: SHGLOBALCOUNTER = 13;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SERVERDRIVE: SHGLOBALCOUNTER = 42;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEGLOBALDIRTYCOUNT: SHGLOBALCOUNTER = 43;
pub type tagSFBS_FLAGS = i32;
