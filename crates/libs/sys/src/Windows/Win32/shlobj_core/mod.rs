#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn AssocGetDetailsOfPropKey(psf : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, pkey : *const super::PROPERTYKEY, pv : *mut super::VARIANT, pffoundpropkey : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn CDefFolderMenu_Create2(pidlfolder : super::LPCITEMIDLIST, hwnd : super::HWND, cidl : super::UINT, apidl : *const super::LPCITEMIDLIST, psf : *mut core::ffi::c_void, pfn : LPFNDFMCALLBACK, nkeys : super::UINT, ahkeys : *const super::HKEY, ppcm : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn CIDLData_CreateFromIDArray(pidlfolder : super::LPCITEMIDLIST, cidl : super::UINT, apidl : *const super::LPCITEMIDLIST, ppdtobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn DAD_AutoScroll(hwnd : super::HWND, pad : *mut AUTO_SCROLL_DATA, pptnow : *const super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn DAD_DragEnterEx(hwndtarget : super::HWND, ptstart : super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidl", feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn DAD_DragEnterEx2(hwndtarget : super::HWND, ptstart : super::POINT, pdtobject : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn DAD_DragLeave() -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn DAD_DragMove(pt : super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commctrl", feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn DAD_SetDragImage(him : super::HIMAGELIST, pptoffset : *mut super::POINT) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn DAD_ShowDragImage(fshow : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILAppendID(pidl : super::LPITEMIDLIST, pmkid : super::LPCSHITEMID, fappend : windows_sys::core::BOOL) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILClone(pidl : super::LPCITEMIDLIST) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILCloneFirst(pidl : super::LPCITEMIDLIST) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILCombine(pidl1 : super::LPCITEMIDLIST, pidl2 : super::LPCITEMIDLIST) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILCreateFromPathA(pszpath : windows_sys::core::PCSTR) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILCreateFromPathW(pszpath : windows_sys::core::PCWSTR) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILFindChild(pidlparent : super::LPITEMIDLIST, pidlchild : super::LPCITEMIDLIST) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILFindLastID(pidl : super::LPCITEMIDLIST) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILFree(pidl : super::LPITEMIDLIST));
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILGetNext(pidl : super::LPCITEMIDLIST) -> super::LPITEMIDLIST);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILGetSize(pidl : super::LPCITEMIDLIST) -> super::UINT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILIsEqual(pidl1 : super::LPCITEMIDLIST, pidl2 : super::LPCITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILIsParent(pidl1 : super::LPCITEMIDLIST, pidl2 : super::LPCITEMIDLIST, fimmediate : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILLoadFromStreamEx(pstm : *mut core::ffi::c_void, pidl : *mut super::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILRemoveLastID(pidl : super::LPITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILSaveToStream(pstm : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn IsNetDrive(idrive : i32) -> i32);
windows_link::link!("shell32.dll" "system" fn IsUserAnAdmin() -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn OpenRegStream(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, grfmode : super::DWORD) -> *mut core::ffi::c_void);
windows_link::link!("shell32.dll" "system" fn PathCleanupSpec(pszdir : windows_sys::core::PCWSTR, pszspec : windows_sys::core::PWSTR) -> i32);
windows_link::link!("shell32.dll" "system" fn PathGetShortPath(pszlongpath : windows_sys::core::PWSTR));
windows_link::link!("shell32.dll" "system" fn PathIsExe(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn PathMakeUniqueName(pszuniquename : windows_sys::core::PWSTR, cchmax : super::UINT, psztemplate : windows_sys::core::PCWSTR, pszlongplate : windows_sys::core::PCWSTR, pszdir : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn PathResolve(pszpath : windows_sys::core::PWSTR, dirs : super::PZPCWSTR, fflags : super::UINT) -> i32);
windows_link::link!("shell32.dll" "system" fn PathYetAnotherMakeUniqueName(pszuniquename : windows_sys::core::PWSTR, pszpath : windows_sys::core::PCWSTR, pszshort : windows_sys::core::PCWSTR, pszfilespec : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn PickIconDlg(hwnd : super::HWND, psziconpath : windows_sys::core::PWSTR, cchiconpath : super::UINT, piiconindex : *mut i32) -> i32);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn PifMgr_CloseProperties(hprops : super::HANDLE, flopt : super::UINT) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn PifMgr_GetProperties(hprops : super::HANDLE, pszgroup : windows_sys::core::PCSTR, lpprops : *mut core::ffi::c_void, cbprops : i32, flopt : super::UINT) -> i32);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn PifMgr_OpenProperties(pszapp : windows_sys::core::PCWSTR, pszpif : windows_sys::core::PCWSTR, hinf : super::UINT, flopt : super::UINT) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn PifMgr_SetProperties(hprops : super::HANDLE, pszgroup : windows_sys::core::PCSTR, lpprops : *const core::ffi::c_void, cbprops : i32, flopt : super::UINT) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn ReadCabinetState(pcs : *mut CABINETSTATE, clength : i32) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn RealDriveType(idrive : i32, foktohitnet : windows_sys::core::BOOL) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn RestartDialog(hwnd : super::HWND, pszprompt : windows_sys::core::PCWSTR, dwreturn : super::DWORD) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn RestartDialogEx(hwnd : super::HWND, pszprompt : windows_sys::core::PCWSTR, dwreturn : super::DWORD, dwreasoncode : super::DWORD) -> i32);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn SHAddFromPropSheetExtArray(hpsxa : HPSXA, lpfnaddpage : super::LPFNADDPROPSHEETPAGE, lparam : super::LPARAM) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHAddToRecentDocs(uflags : super::UINT, pv : super::LPCVOID));
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHAlloc(cb : super::SIZE_T) -> *mut core::ffi::c_void);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHBindToFolderIDListParent(psfroot : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHBindToFolderIDListParentEx(psfroot : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, ppbc : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHBindToObject(psf : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, pbc : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHBindToParent(pidl : super::LPCITEMIDLIST, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHBrowseForFolderA(lpbi : LPBROWSEINFOA) -> super::LPITEMIDLIST);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHBrowseForFolderW(lpbi : LPBROWSEINFOW) -> super::LPITEMIDLIST);
#[cfg(feature = "guiddef")]
windows_link::link!("shell32.dll" "system" fn SHCLSIDFromString(psz : windows_sys::core::PCWSTR, pclsid : *mut super::CLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHChangeNotification_Lock(hchange : super::HANDLE, dwprocid : super::DWORD, pppidl : *mut *mut super::LPITEMIDLIST, plevent : *mut super::LONG) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn SHChangeNotification_Unlock(hlock : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHChangeNotify(weventid : super::LONG, uflags : super::UINT, dwitem1 : super::LPCVOID, dwitem2 : super::LPCVOID));
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHChangeNotifyDeregister(ulid : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHChangeNotifyRegister(hwnd : super::HWND, fsources : i32, fevents : super::LONG, wmsg : super::UINT, centries : i32, pshcne : *const SHChangeNotifyEntry) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCloneSpecialIDList(hwnd : super::HWND, csidl : i32, fcreate : windows_sys::core::BOOL) -> super::LPITEMIDLIST);
#[cfg(feature = "guiddef")]
windows_link::link!("shell32.dll" "system" fn SHCoCreateInstance(pszclsid : windows_sys::core::PCWSTR, pclsid : *const super::CLSID, punkouter : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDataObject(pidlfolder : super::LPCITEMIDLIST, cidl : super::UINT, apidl : *const super::LPCITEMIDLIST, pdtinner : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDefaultContextMenu(pdcm : *const DEFCONTEXTMENU, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHCreateDirectory(hwnd : super::HWND, pszpath : windows_sys::core::PCWSTR) -> i32);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDirectoryExA(hwnd : super::HWND, pszpath : windows_sys::core::PCSTR, psa : *const super::SECURITY_ATTRIBUTES) -> i32);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDirectoryExW(hwnd : super::HWND, pszpath : windows_sys::core::PCWSTR, psa : *const super::SECURITY_ATTRIBUTES) -> i32);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn SHCreateFileExtractIconW(pszfile : windows_sys::core::PCWSTR, dwfileattributes : super::DWORD, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oleidl", feature = "shared", feature = "shobjidl_core"))]
windows_link::link!("shell32.dll" "system" fn SHCreateShellFolderView(pcsfv : *const SFV_CREATE, ppsv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateShellFolderViewEx(pcsfv : *const CSFV, ppsv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHCreateShellItem(pidlparent : super::LPCITEMIDLIST, psfparent : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, ppsi : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("shell32.dll" "system" fn SHCreateStdEnumFmtEtc(cfmt : super::UINT, afmt : *const super::FORMATETC, ppenumformatetc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHDefExtractIconA(psziconfile : windows_sys::core::PCSTR, iindex : i32, uflags : super::UINT, phiconlarge : *mut super::HICON, phiconsmall : *mut super::HICON, niconsize : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHDefExtractIconW(psziconfile : windows_sys::core::PCWSTR, iindex : i32, uflags : super::UINT, phiconlarge : *mut super::HICON, phiconsmall : *mut super::HICON, niconsize : super::UINT) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHDestroyPropSheetExtArray(hpsxa : HPSXA));
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHDoDragDrop(hwnd : super::HWND, pdata : *mut core::ffi::c_void, pdsrc : *mut core::ffi::c_void, dweffect : super::DWORD, pdweffect : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHFindFiles(pidlfolder : super::LPCITEMIDLIST, pidlsavefile : super::LPCITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHFind_InitMenuPopup(hmenu : super::HMENU, hwndowner : super::HWND, idcmdfirst : super::UINT, idcmdlast : super::UINT) -> *mut core::ffi::c_void);
windows_link::link!("shell32.dll" "system" fn SHFlushSFCache());
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHFormatDrive(hwnd : super::HWND, drive : super::UINT, fmtid : super::UINT, options : super::UINT) -> super::DWORD);
windows_link::link!("shell32.dll" "system" fn SHFree(pv : *const core::ffi::c_void));
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn SHGetAttributesFromDataObject(pdo : *mut core::ffi::c_void, dwattributemask : super::DWORD, pdwattributes : *mut super::DWORD, pcitems : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetDataFromIDListA(psf : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, nformat : i32, pv : *mut core::ffi::c_void, cb : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetDataFromIDListW(psf : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, nformat : i32, pv : *mut core::ffi::c_void, cb : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "shobjidl_core")]
windows_link::link!("shell32.dll" "system" fn SHGetDesktopFolder(ppshf : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderLocation(hwnd : super::HWND, csidl : i32, htoken : super::HANDLE, dwflags : super::DWORD, ppidl : *mut super::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathA(hwnd : super::HWND, csidl : i32, htoken : super::HANDLE, dwflags : super::DWORD, pszpath : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathAndSubDirA(hwnd : super::HWND, csidl : i32, htoken : super::HANDLE, dwflags : super::DWORD, pszsubdir : windows_sys::core::PCSTR, pszpath : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathAndSubDirW(hwnd : super::HWND, csidl : i32, htoken : super::HANDLE, dwflags : super::DWORD, pszsubdir : windows_sys::core::PCWSTR, pszpath : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathW(hwnd : super::HWND, csidl : i32, htoken : super::HANDLE, dwflags : super::DWORD, pszpath : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHGetIconOverlayIndexA(psziconpath : windows_sys::core::PCSTR, iiconindex : i32) -> i32);
windows_link::link!("shell32.dll" "system" fn SHGetIconOverlayIndexW(psziconpath : windows_sys::core::PCWSTR, iiconindex : i32) -> i32);
windows_link::link!("shell32.dll" "system" fn SHGetInstanceExplorer(ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetKnownFolderIDList(rfid : *const super::KNOWNFOLDERID, dwflags : super::DWORD, htoken : super::HANDLE, ppidl : *mut super::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetKnownFolderItem(rfid : *const super::KNOWNFOLDERID, flags : KNOWN_FOLDER_FLAG, htoken : super::HANDLE, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetKnownFolderPath(rfid : *const super::KNOWNFOLDERID, dwflags : super::DWORD, htoken : super::HANDLE, ppszpath : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("shell32.dll" "system" fn SHGetMalloc(ppmalloc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetPathFromIDListA(pidl : super::LPCITEMIDLIST, pszpath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetPathFromIDListEx(pidl : super::LPCITEMIDLIST, pszpath : windows_sys::core::PWSTR, cchpath : super::DWORD, uopts : GPFIDL_FLAGS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetPathFromIDListW(pidl : super::LPCITEMIDLIST, pszpath : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetRealIDL(psf : *mut core::ffi::c_void, pidlsimple : super::LPCITEMIDLIST, ppidlreal : *mut super::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "shobjidl_core"))]
windows_link::link!("shell32.dll" "system" fn SHGetSetFolderCustomSettings(pfcs : LPSHFOLDERCUSTOMSETTINGS, pszpath : windows_sys::core::PCWSTR, dwreadwrite : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHGetSetSettings(lpss : LPSHELLSTATEA, dwmask : super::DWORD, bset : windows_sys::core::BOOL));
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHGetSettings(psfs : *mut SHELLFLAGSTATE, dwmask : super::DWORD));
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHGetSpecialFolderLocation(hwnd : super::HWND, csidl : i32, ppidl : *mut super::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHGetSpecialFolderPathA(hwnd : super::HWND, pszpath : windows_sys::core::PSTR, csidl : i32, fcreate : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHGetSpecialFolderPathW(hwnd : super::HWND, pszpath : windows_sys::core::PWSTR, csidl : i32, fcreate : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHHandleUpdateImage(pidlextra : super::LPCITEMIDLIST) -> i32);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHILCreateFromPath(pszpath : windows_sys::core::PCWSTR, ppidl : *mut super::LPITEMIDLIST, rgfinout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHLimitInputEdit(hwndedit : super::HWND, psf : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("shell32.dll" "system" fn SHLoadInProc(rclsid : *const super::IID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHMapPIDLToSystemImageListIndex(pshf : *mut core::ffi::c_void, pidl : super::LPCITEMIDLIST, piindexsel : *mut i32) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHObjectProperties(hwnd : super::HWND, shopobjecttype : super::DWORD, pszobjectname : windows_sys::core::PCWSTR, pszpropertypage : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHOpenFolderAndSelectItems(pidlfolder : super::LPCITEMIDLIST, cidl : super::UINT, apidl : *const super::LPCITEMIDLIST, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHOpenWithDialog(hwndparent : super::HWND, poainfo : *const OPENASINFO) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHParseDisplayName(pszname : windows_sys::core::PCWSTR, pbc : *mut core::ffi::c_void, ppidl : *mut super::LPITEMIDLIST, sfgaoin : super::SFGAOF, psfgaoout : *mut super::SFGAOF) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHPathPrepareForWriteA(hwnd : super::HWND, punkenablemodless : *mut core::ffi::c_void, pszpath : windows_sys::core::PCSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHPathPrepareForWriteW(hwnd : super::HWND, punkenablemodless : *mut core::ffi::c_void, pszpath : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "propidlbase", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn SHPropStgCreate(psstg : *mut core::ffi::c_void, fmtid : *const super::IID, pclsid : *const super::CLSID, grfflags : super::DWORD, grfmode : super::DWORD, dwdisposition : super::DWORD, ppstg : *mut *mut core::ffi::c_void, pucodepage : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn SHPropStgReadMultiple(pps : *mut core::ffi::c_void, ucodepage : super::UINT, cpspec : super::ULONG, rgpspec : *const super::PROPSPEC, rgvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn SHPropStgWriteMultiple(pps : *mut core::ffi::c_void, pucodepage : *mut super::UINT, cpspec : super::ULONG, rgpspec : *const super::PROPSPEC, rgvar : *mut super::PROPVARIANT, propidnamefirst : super::PROPID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn SHReplaceFromPropSheetExtArray(hpsxa : HPSXA, upageid : super::UINT, lpfnreplacewith : super::LPFNADDPROPSHEETPAGE, lparam : super::LPARAM) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHRestricted(rest : RESTRICTIONS) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHSetFolderPathA(csidl : i32, htoken : super::HANDLE, dwflags : super::DWORD, pszpath : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHSetFolderPathW(csidl : i32, htoken : super::HANDLE, dwflags : super::DWORD, pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHSetInstanceExplorer(punk : *mut core::ffi::c_void));
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHSetKnownFolderPath(rfid : *const super::KNOWNFOLDERID, dwflags : super::DWORD, htoken : super::HANDLE, pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHShellFolderView_Message(hwndmain : super::HWND, umsg : super::UINT, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHStartNetConnectionDialogW(hwnd : super::HWND, pszremotename : windows_sys::core::PCWSTR, dwtype : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHUpdateImageA(pszhashitem : windows_sys::core::PCSTR, iindex : i32, uflags : super::UINT, iimageindex : i32));
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn SHUpdateImageW(pszhashitem : windows_sys::core::PCWSTR, iindex : i32, uflags : super::UINT, iimageindex : i32));
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHValidateUNC(hwndowner : super::HWND, pszfile : windows_sys::core::PWSTR, fconnect : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn Shell_GetCachedImageIndex(pwsziconpath : windows_sys::core::PCWSTR, iiconindex : i32, uiconflags : super::UINT) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn Shell_GetCachedImageIndexA(psziconpath : windows_sys::core::PCSTR, iiconindex : i32, uiconflags : super::UINT) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn Shell_GetCachedImageIndexW(psziconpath : windows_sys::core::PCWSTR, iiconindex : i32, uiconflags : super::UINT) -> i32);
#[cfg(feature = "commctrl")]
windows_link::link!("shell32.dll" "system" fn Shell_GetImageLists(phiml : *mut super::HIMAGELIST, phimlsmall : *mut super::HIMAGELIST) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn Shell_MergeMenus(hmdst : super::HMENU, hmsrc : super::HMENU, uinsert : super::UINT, uidadjust : super::UINT, uidadjustmax : super::UINT, uflags : super::ULONG) -> super::UINT);
#[cfg(all(feature = "shared", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SignalFileOpen(pidl : super::LPCITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn StgMakeUniqueName(pstgparent : *mut core::ffi::c_void, pszfilespec : windows_sys::core::PCWSTR, grfmode : super::DWORD, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn Win32DeleteFile(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn WriteCabinetState(pcs : *const CABINETSTATE) -> windows_sys::core::BOOL);
pub const ACLO_CURRENTDIR: AUTOCOMPLETELISTOPTIONS = 1;
pub const ACLO_DESKTOP: AUTOCOMPLETELISTOPTIONS = 4;
pub const ACLO_FAVORITES: AUTOCOMPLETELISTOPTIONS = 8;
pub const ACLO_FILESYSDIRS: AUTOCOMPLETELISTOPTIONS = 32;
pub const ACLO_FILESYSONLY: AUTOCOMPLETELISTOPTIONS = 16;
pub const ACLO_MYCOMPUTER: AUTOCOMPLETELISTOPTIONS = 2;
pub const ACLO_NONE: AUTOCOMPLETELISTOPTIONS = 0;
pub const ACLO_VIRTUALNAMESPACE: AUTOCOMPLETELISTOPTIONS = 64;
pub const ADDURL_SILENT: i32 = 1;
pub const AD_APPLY_ALL: i32 = 7;
pub const AD_APPLY_BUFFERED_REFRESH: i32 = 16;
pub const AD_APPLY_DYNAMICREFRESH: i32 = 32;
pub const AD_APPLY_FORCE: i32 = 8;
pub const AD_APPLY_HTMLGEN: i32 = 2;
pub const AD_APPLY_REFRESH: i32 = 4;
pub const AD_APPLY_SAVE: i32 = 1;
pub const AD_GETWP_BMP: i32 = 0;
pub const AD_GETWP_IMAGE: i32 = 1;
pub const AD_GETWP_LAST_APPLIED: i32 = 2;
pub type AUTOCOMPLETELISTOPTIONS = i32;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct AUTO_SCROLL_DATA {
    pub iNextSample: i32,
    pub dwLastScroll: super::DWORD,
    pub bFull: windows_sys::core::BOOL,
    pub pts: [super::POINT; 3],
    pub dwTimes: [super::DWORD; 3],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for AUTO_SCROLL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type BFFCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: super::UINT, lparam: super::LPARAM, lpdata: super::LPARAM) -> i32>;
pub const BFFM_ENABLEOK: i32 = 1125;
pub const BFFM_INITIALIZED: i32 = 1;
pub const BFFM_IUNKNOWN: i32 = 5;
pub const BFFM_SELCHANGED: i32 = 2;
pub const BFFM_SETEXPANDED: i32 = 1130;
pub const BFFM_SETOKTEXT: i32 = 1129;
pub const BFFM_SETSELECTION: i32 = 1126;
pub const BFFM_SETSELECTIONA: i32 = 1126;
pub const BFFM_SETSELECTIONW: i32 = 1127;
pub const BFFM_SETSTATUSTEXT: i32 = 1124;
pub const BFFM_SETSTATUSTEXTA: i32 = 1124;
pub const BFFM_SETSTATUSTEXTW: i32 = 1128;
pub const BFFM_VALIDATEFAILED: i32 = 3;
pub const BFFM_VALIDATEFAILEDA: i32 = 3;
pub const BFFM_VALIDATEFAILEDW: i32 = 4;
pub const BIF_BROWSEFILEJUNCTIONS: i32 = 65536;
pub const BIF_BROWSEFORCOMPUTER: i32 = 4096;
pub const BIF_BROWSEFORPRINTER: i32 = 8192;
pub const BIF_BROWSEINCLUDEFILES: i32 = 16384;
pub const BIF_BROWSEINCLUDEURLS: i32 = 128;
pub const BIF_DONTGOBELOWDOMAIN: i32 = 2;
pub const BIF_EDITBOX: i32 = 16;
pub const BIF_NEWDIALOGSTYLE: i32 = 64;
pub const BIF_NONEWFOLDERBUTTON: i32 = 512;
pub const BIF_NOTRANSLATETARGETS: i32 = 1024;
pub const BIF_RETURNFSANCESTORS: i32 = 8;
pub const BIF_RETURNONLYFSDIRS: i32 = 1;
pub const BIF_SHAREABLE: i32 = 32768;
pub const BIF_STATUSTEXT: i32 = 4;
pub const BIF_UAHINT: i32 = 256;
pub const BIF_USENEWUI: i32 = 80;
pub const BIF_VALIDATE: i32 = 32;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct BROWSEINFOA {
    pub hwndOwner: super::HWND,
    pub pidlRoot: super::LPCITEMIDLIST,
    pub pszDisplayName: windows_sys::core::PSTR,
    pub lpszTitle: windows_sys::core::PCSTR,
    pub ulFlags: super::UINT,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::LPARAM,
    pub iImage: i32,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct BROWSEINFOW {
    pub hwndOwner: super::HWND,
    pub pidlRoot: super::LPCITEMIDLIST,
    pub pszDisplayName: windows_sys::core::PWSTR,
    pub lpszTitle: windows_sys::core::PCWSTR,
    pub ulFlags: super::UINT,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::LPARAM,
    pub iImage: i32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CABINETSTATE {
    pub cLength: super::WORD,
    pub nVersion: super::WORD,
    pub _bitfield: i32,
    pub fMenuEnumFilter: super::UINT,
}
pub const CABINETSTATE_VERSION: i32 = 2;
pub const CFSTR_AUTOPLAY_SHELLIDLISTS: windows_sys::core::PCSTR = windows_sys::core::s!("Autoplay Enumerated IDList Array");
pub const CFSTR_DROPDESCRIPTION: windows_sys::core::PCSTR = windows_sys::core::s!("DropDescription");
pub const CFSTR_FILECONTENTS: windows_sys::core::PCSTR = windows_sys::core::s!("FileContents");
pub const CFSTR_FILEDESCRIPTOR: windows_sys::core::PCSTR = windows_sys::core::s!("FileGroupDescriptor");
pub const CFSTR_FILEDESCRIPTORA: windows_sys::core::PCSTR = windows_sys::core::s!("FileGroupDescriptor");
pub const CFSTR_FILEDESCRIPTORW: windows_sys::core::PCSTR = windows_sys::core::s!("FileGroupDescriptorW");
pub const CFSTR_FILENAME: windows_sys::core::PCSTR = windows_sys::core::s!("FileName");
pub const CFSTR_FILENAMEA: windows_sys::core::PCSTR = windows_sys::core::s!("FileName");
pub const CFSTR_FILENAMEMAP: windows_sys::core::PCSTR = windows_sys::core::s!("FileNameMap");
pub const CFSTR_FILENAMEMAPA: windows_sys::core::PCSTR = windows_sys::core::s!("FileNameMap");
pub const CFSTR_FILENAMEMAPW: windows_sys::core::PCSTR = windows_sys::core::s!("FileNameMapW");
pub const CFSTR_FILENAMEW: windows_sys::core::PCSTR = windows_sys::core::s!("FileNameW");
pub const CFSTR_FILE_ATTRIBUTES_ARRAY: windows_sys::core::PCSTR = windows_sys::core::s!("File Attributes Array");
pub const CFSTR_INDRAGLOOP: windows_sys::core::PCSTR = windows_sys::core::s!("InShellDragLoop");
pub const CFSTR_INETURL: windows_sys::core::PCSTR = windows_sys::core::s!("UniformResourceLocator");
pub const CFSTR_INETURLA: windows_sys::core::PCSTR = windows_sys::core::s!("UniformResourceLocator");
pub const CFSTR_INETURLW: windows_sys::core::PCSTR = windows_sys::core::s!("UniformResourceLocatorW");
pub const CFSTR_INVOKECOMMAND_DROPPARAM: windows_sys::core::PCSTR = windows_sys::core::s!("InvokeCommand DropParam");
pub const CFSTR_LOGICALPERFORMEDDROPEFFECT: windows_sys::core::PCSTR = windows_sys::core::s!("Logical Performed DropEffect");
pub const CFSTR_MOUNTEDVOLUME: windows_sys::core::PCSTR = windows_sys::core::s!("MountedVolume");
pub const CFSTR_NETRESOURCES: windows_sys::core::PCSTR = windows_sys::core::s!("Net Resource");
pub const CFSTR_PASTESUCCEEDED: windows_sys::core::PCSTR = windows_sys::core::s!("Paste Succeeded");
pub const CFSTR_PERFORMEDDROPEFFECT: windows_sys::core::PCSTR = windows_sys::core::s!("Performed DropEffect");
pub const CFSTR_PERSISTEDDATAOBJECT: windows_sys::core::PCSTR = windows_sys::core::s!("PersistedDataObject");
pub const CFSTR_PREFERREDDROPEFFECT: windows_sys::core::PCSTR = windows_sys::core::s!("Preferred DropEffect");
pub const CFSTR_PRINTERGROUP: windows_sys::core::PCSTR = windows_sys::core::s!("PrinterFriendlyName");
pub const CFSTR_SHELLDROPHANDLER: windows_sys::core::PCSTR = windows_sys::core::s!("DropHandlerCLSID");
pub const CFSTR_SHELLIDLIST: windows_sys::core::PCSTR = windows_sys::core::s!("Shell IDList Array");
pub const CFSTR_SHELLIDLISTOFFSET: windows_sys::core::PCSTR = windows_sys::core::s!("Shell Object Offsets");
pub const CFSTR_SHELLURL: windows_sys::core::PCSTR = windows_sys::core::s!("UniformResourceLocator");
pub const CFSTR_TARGETCLSID: windows_sys::core::PCSTR = windows_sys::core::s!("TargetCLSID");
pub const CFSTR_UNTRUSTEDDRAGDROP: windows_sys::core::PCSTR = windows_sys::core::s!("UntrustedDragDrop");
pub const CFSTR_ZONEIDENTIFIER: windows_sys::core::PCSTR = windows_sys::core::s!("ZoneIdentifier");
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CIDA {
    pub cidl: super::UINT,
    pub aoffset: [super::UINT; 1],
}
#[cfg(feature = "shared")]
impl Default for CIDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMDID_INTSHORTCUTCREATE: i32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COMPONENT {
    pub dwSize: super::DWORD,
    pub dwID: super::DWORD,
    pub iComponentType: i32,
    pub fChecked: windows_sys::core::BOOL,
    pub fDirty: windows_sys::core::BOOL,
    pub fNoScroll: windows_sys::core::BOOL,
    pub cpPos: COMPPOS,
    pub wszFriendlyName: [super::WCHAR; 260],
    pub wszSource: [super::WCHAR; 2084],
    pub wszSubscribedURL: [super::WCHAR; 2084],
    pub dwCurItemState: super::DWORD,
    pub csiOriginal: COMPSTATEINFO,
    pub csiRestored: COMPSTATEINFO,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COMPONENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct COMPONENTSOPT {
    pub dwSize: super::DWORD,
    pub fEnableComponents: windows_sys::core::BOOL,
    pub fActiveDesktop: windows_sys::core::BOOL,
}
pub const COMPONENT_DEFAULT_LEFT: i32 = 65535;
pub const COMPONENT_DEFAULT_TOP: i32 = 65535;
pub const COMPONENT_TOP: i32 = 1073741823;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct COMPPOS {
    pub dwSize: super::DWORD,
    pub iLeft: i32,
    pub iTop: i32,
    pub dwWidth: super::DWORD,
    pub dwHeight: super::DWORD,
    pub izIndex: i32,
    pub fCanResize: windows_sys::core::BOOL,
    pub fCanResizeX: windows_sys::core::BOOL,
    pub fCanResizeY: windows_sys::core::BOOL,
    pub iPreferredLeftPercent: i32,
    pub iPreferredTopPercent: i32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct COMPSTATEINFO {
    pub dwSize: super::DWORD,
    pub iLeft: i32,
    pub iTop: i32,
    pub dwWidth: super::DWORD,
    pub dwHeight: super::DWORD,
    pub dwItemState: super::DWORD,
}
pub const COMP_ELEM_ALL: i32 = 32767;
pub const COMP_ELEM_CHECKED: i32 = 2;
pub const COMP_ELEM_CURITEMSTATE: i32 = 16384;
pub const COMP_ELEM_DIRTY: i32 = 4;
pub const COMP_ELEM_FRIENDLYNAME: i32 = 1024;
pub const COMP_ELEM_NOSCROLL: i32 = 8;
pub const COMP_ELEM_ORIGINAL_CSI: i32 = 4096;
pub const COMP_ELEM_POS_LEFT: i32 = 16;
pub const COMP_ELEM_POS_TOP: i32 = 32;
pub const COMP_ELEM_POS_ZINDEX: i32 = 256;
pub const COMP_ELEM_RESTORED_CSI: i32 = 8192;
pub const COMP_ELEM_SIZE_HEIGHT: i32 = 128;
pub const COMP_ELEM_SIZE_WIDTH: i32 = 64;
pub const COMP_ELEM_SOURCE: i32 = 512;
pub const COMP_ELEM_SUBSCRIBEDURL: i32 = 2048;
pub const COMP_ELEM_TYPE: i32 = 1;
pub const COMP_TYPE_CFHTML: i32 = 4;
pub const COMP_TYPE_CONTROL: i32 = 3;
pub const COMP_TYPE_HTMLDOC: i32 = 0;
pub const COMP_TYPE_MAX: i32 = 4;
pub const COMP_TYPE_PICTURE: i32 = 1;
pub const COMP_TYPE_WEBSITE: i32 = 2;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct CSFV {
    pub cbSize: super::UINT,
    pub pshf: *mut core::ffi::c_void,
    pub psvOuter: *mut core::ffi::c_void,
    pub pidl: super::LPCITEMIDLIST,
    pub lEvents: super::LONG,
    pub pfnCallback: LPFNVIEWCALLBACK,
    pub fvm: super::FOLDERVIEWMODE,
}
pub const CSIDL_ADMINTOOLS: i32 = 48;
pub const CSIDL_ALTSTARTUP: i32 = 29;
pub const CSIDL_APPDATA: i32 = 26;
pub const CSIDL_BITBUCKET: i32 = 10;
pub const CSIDL_CDBURN_AREA: i32 = 59;
pub const CSIDL_COMMON_ADMINTOOLS: i32 = 47;
pub const CSIDL_COMMON_ALTSTARTUP: i32 = 30;
pub const CSIDL_COMMON_APPDATA: i32 = 35;
pub const CSIDL_COMMON_DESKTOPDIRECTORY: i32 = 25;
pub const CSIDL_COMMON_DOCUMENTS: i32 = 46;
pub const CSIDL_COMMON_FAVORITES: i32 = 31;
pub const CSIDL_COMMON_MUSIC: i32 = 53;
pub const CSIDL_COMMON_OEM_LINKS: i32 = 58;
pub const CSIDL_COMMON_PICTURES: i32 = 54;
pub const CSIDL_COMMON_PROGRAMS: i32 = 23;
pub const CSIDL_COMMON_STARTMENU: i32 = 22;
pub const CSIDL_COMMON_STARTUP: i32 = 24;
pub const CSIDL_COMMON_TEMPLATES: i32 = 45;
pub const CSIDL_COMMON_VIDEO: i32 = 55;
pub const CSIDL_COMPUTERSNEARME: i32 = 61;
pub const CSIDL_CONNECTIONS: i32 = 49;
pub const CSIDL_CONTROLS: i32 = 3;
pub const CSIDL_COOKIES: i32 = 33;
pub const CSIDL_DESKTOP: i32 = 0;
pub const CSIDL_DESKTOPDIRECTORY: i32 = 16;
pub const CSIDL_DRIVES: i32 = 17;
pub const CSIDL_FAVORITES: i32 = 6;
pub const CSIDL_FLAG_CREATE: i32 = 32768;
pub const CSIDL_FLAG_DONT_UNEXPAND: i32 = 8192;
pub const CSIDL_FLAG_DONT_VERIFY: i32 = 16384;
pub const CSIDL_FLAG_MASK: i32 = 65280;
pub const CSIDL_FLAG_NO_ALIAS: i32 = 4096;
pub const CSIDL_FLAG_PER_USER_INIT: i32 = 2048;
pub const CSIDL_FONTS: i32 = 20;
pub const CSIDL_HISTORY: i32 = 34;
pub const CSIDL_INTERNET: i32 = 1;
pub const CSIDL_INTERNET_CACHE: i32 = 32;
pub const CSIDL_LOCAL_APPDATA: i32 = 28;
pub const CSIDL_MYDOCUMENTS: i32 = 5;
pub const CSIDL_MYMUSIC: i32 = 13;
pub const CSIDL_MYPICTURES: i32 = 39;
pub const CSIDL_MYVIDEO: i32 = 14;
pub const CSIDL_NETHOOD: i32 = 19;
pub const CSIDL_NETWORK: i32 = 18;
pub const CSIDL_PERSONAL: i32 = 5;
pub const CSIDL_PRINTERS: i32 = 4;
pub const CSIDL_PRINTHOOD: i32 = 27;
pub const CSIDL_PROFILE: i32 = 40;
pub const CSIDL_PROGRAMS: i32 = 2;
pub const CSIDL_PROGRAM_FILES: i32 = 38;
pub const CSIDL_PROGRAM_FILESX86: i32 = 42;
pub const CSIDL_PROGRAM_FILES_COMMON: i32 = 43;
pub const CSIDL_PROGRAM_FILES_COMMONX86: i32 = 44;
pub const CSIDL_RECENT: i32 = 8;
pub const CSIDL_RESOURCES: i32 = 56;
pub const CSIDL_RESOURCES_LOCALIZED: i32 = 57;
pub const CSIDL_SENDTO: i32 = 9;
pub const CSIDL_STARTMENU: i32 = 11;
pub const CSIDL_STARTUP: i32 = 7;
pub const CSIDL_SYSTEM: i32 = 37;
pub const CSIDL_SYSTEMX86: i32 = 41;
pub const CSIDL_TEMPLATES: i32 = 21;
pub const CSIDL_WINDOWS: i32 = 36;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DATABLOCK_HEADER {
    pub cbSize: super::DWORD,
    pub dwSignature: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DEFCONTEXTMENU {
    pub hwnd: super::HWND,
    pub pcmcb: *mut core::ffi::c_void,
    pub pidlFolder: super::LPCITEMIDLIST,
    pub psf: *mut core::ffi::c_void,
    pub cidl: super::UINT,
    pub apidl: *mut super::LPCITEMIDLIST,
    pub punkAssociationInfo: *mut core::ffi::c_void,
    pub cKeys: super::UINT,
    pub aKeys: *const super::HKEY,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "shtypes"))]
#[derive(Clone, Copy)]
pub struct DETAILSINFO {
    pub pidl: super::LPCITEMIDLIST,
    pub fmt: i32,
    pub cxChar: i32,
    pub str: super::STRRET,
    pub iImage: i32,
}
#[cfg(all(feature = "shared", feature = "shtypes"))]
impl Default for DETAILSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DFMICS {
    pub cbSize: super::DWORD,
    pub fMask: super::DWORD,
    pub lParam: super::LPARAM,
    pub idCmdFirst: super::UINT,
    pub idDefMax: super::UINT,
    pub pici: super::LPCMINVOKECOMMANDINFO,
    pub punkSite: *mut core::ffi::c_void,
}
#[cfg(feature = "shared")]
pub const DFM_CMD_COPY: super::UINT = 4294967293;
#[cfg(feature = "shared")]
pub const DFM_CMD_DELETE: super::UINT = 4294967295;
#[cfg(feature = "shared")]
pub const DFM_CMD_LINK: super::UINT = 4294967292;
#[cfg(feature = "shared")]
pub const DFM_CMD_MODALPROP: super::UINT = 4294967284;
#[cfg(feature = "shared")]
pub const DFM_CMD_MOVE: super::UINT = 4294967294;
#[cfg(feature = "shared")]
pub const DFM_CMD_NEWFOLDER: super::UINT = 4294967290;
#[cfg(feature = "shared")]
pub const DFM_CMD_PASTE: super::UINT = 4294967289;
#[cfg(feature = "shared")]
pub const DFM_CMD_PASTELINK: super::UINT = 4294967286;
#[cfg(feature = "shared")]
pub const DFM_CMD_PASTESPECIAL: super::UINT = 4294967285;
#[cfg(feature = "shared")]
pub const DFM_CMD_PROPERTIES: super::UINT = 4294967291;
#[cfg(feature = "shared")]
pub const DFM_CMD_RENAME: super::UINT = 4294967283;
#[cfg(feature = "shared")]
pub const DFM_CMD_VIEWDETAILS: super::UINT = 4294967287;
#[cfg(feature = "shared")]
pub const DFM_CMD_VIEWLIST: super::UINT = 4294967288;
pub const DFM_GETDEFSTATICID: i32 = 14;
pub const DFM_GETHELPTEXT: i32 = 5;
pub const DFM_GETHELPTEXTW: i32 = 11;
pub const DFM_GETVERBA: i32 = 16;
pub const DFM_GETVERBW: i32 = 15;
pub const DFM_INVOKECOMMAND: i32 = 2;
pub const DFM_INVOKECOMMANDEX: i32 = 12;
pub const DFM_MAPCOMMANDNAME: i32 = 13;
pub const DFM_MERGECONTEXTMENU: i32 = 1;
pub const DFM_MERGECONTEXTMENU_BOTTOM: i32 = 17;
pub const DFM_MERGECONTEXTMENU_TOP: i32 = 10;
pub const DFM_MODIFYQCMFLAGS: i32 = 18;
pub const DFM_VALIDATECMD: i32 = 9;
pub const DFM_WM_DRAWITEM: i32 = 7;
pub const DFM_WM_INITMENUPOPUP: i32 = 8;
pub const DFM_WM_MEASUREITEM: i32 = 6;
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DROPDESCRIPTION {
    pub r#type: DROPIMAGETYPE,
    pub szMessage: [super::WCHAR; 260],
    pub szInsert: [super::WCHAR; 260],
}
#[cfg(feature = "winnt")]
impl Default for DROPDESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DROPFILES {
    pub pFiles: super::DWORD,
    pub pt: super::POINT,
    pub fNC: windows_sys::core::BOOL,
    pub fWide: windows_sys::core::BOOL,
}
pub type DROPIMAGETYPE = i32;
pub const DROPIMAGE_COPY: DROPIMAGETYPE = 1;
pub const DROPIMAGE_INVALID: DROPIMAGETYPE = -1;
pub const DROPIMAGE_LABEL: DROPIMAGETYPE = 6;
pub const DROPIMAGE_LINK: DROPIMAGETYPE = 4;
pub const DROPIMAGE_MOVE: DROPIMAGETYPE = 2;
pub const DROPIMAGE_NOIMAGE: DROPIMAGETYPE = 8;
pub const DROPIMAGE_NONE: DROPIMAGETYPE = 0;
pub const DROPIMAGE_WARNING: DROPIMAGETYPE = 7;
pub const DTI_ADDUI_DEFAULT: tagDTI_ADTIWUI = 0;
pub const DTI_ADDUI_DISPSUBWIZARD: tagDTI_ADTIWUI = 1;
pub const DTI_ADDUI_POSITIONITEM: tagDTI_ADTIWUI = 2;
pub const DVASPECT_COPY: i32 = 3;
pub const DVASPECT_LINK: i32 = 4;
pub const DVASPECT_SHORTNAME: i32 = 2;
pub const EXP_DARWIN_ID_SIG: u32 = 2684354566;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EXP_DARWIN_LINK {
    pub dbh: DATABLOCK_HEADER,
    pub szDarwinID: [super::CHAR; 260],
    pub szwDarwinID: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EXP_DARWIN_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EXP_PROPERTYSTORAGE {
    pub cbSize: super::DWORD,
    pub dwSignature: super::DWORD,
    pub abPropertyStorage: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for EXP_PROPERTYSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXP_PROPERTYSTORAGE_SIG: u32 = 2684354569;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EXP_SPECIAL_FOLDER {
    pub cbSize: super::DWORD,
    pub dwSignature: super::DWORD,
    pub idSpecialFolder: super::DWORD,
    pub cbOffset: super::DWORD,
}
pub const EXP_SPECIAL_FOLDER_SIG: u32 = 2684354565;
pub const EXP_SZ_ICON_SIG: u32 = 2684354567;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EXP_SZ_LINK {
    pub cbSize: super::DWORD,
    pub dwSignature: super::DWORD,
    pub szTarget: [super::CHAR; 260],
    pub swzTarget: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EXP_SZ_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXP_SZ_LINK_SIG: u32 = 2684354561;
pub const FCIDM_BROWSERFIRST: i32 = 40960;
pub const FCIDM_BROWSERLAST: i32 = 48896;
pub const FCIDM_GLOBALFIRST: i32 = 32768;
pub const FCIDM_GLOBALLAST: i32 = 40959;
pub const FCIDM_MENU_EDIT: i32 = 32832;
pub const FCIDM_MENU_EXPLORE: i32 = 33104;
pub const FCIDM_MENU_FAVORITES: i32 = 33136;
pub const FCIDM_MENU_FILE: i32 = 32768;
pub const FCIDM_MENU_FIND: i32 = 33088;
pub const FCIDM_MENU_HELP: i32 = 33024;
pub const FCIDM_MENU_TOOLS: i32 = 32960;
pub const FCIDM_MENU_TOOLS_SEP_GOTO: i32 = 32961;
pub const FCIDM_MENU_VIEW: i32 = 32896;
pub const FCIDM_MENU_VIEW_SEP_OPTIONS: i32 = 32897;
pub const FCIDM_SHVIEWFIRST: i32 = 0;
pub const FCIDM_SHVIEWLAST: i32 = 32767;
pub const FCSM_CLSID: i32 = 8;
pub const FCSM_FLAGS: i32 = 64;
pub const FCSM_ICONFILE: i32 = 16;
pub const FCSM_INFOTIP: i32 = 4;
pub const FCSM_LOGO: i32 = 32;
pub const FCSM_VIEWID: i32 = 1;
pub const FCSM_WEBVIEWTEMPLATE: i32 = 2;
pub const FCS_FLAG_DRAGDROP: i32 = 2;
pub const FCS_FORCEWRITE: i32 = 2;
pub const FCS_READ: i32 = 1;
pub const FCS_WRITE: i32 = 3;
pub const FD_ACCESSTIME: FD_FLAGS = 16;
pub const FD_ATTRIBUTES: FD_FLAGS = 4;
pub const FD_CLSID: FD_FLAGS = 1;
pub const FD_CREATETIME: FD_FLAGS = 8;
pub const FD_FILESIZE: FD_FLAGS = 64;
pub type FD_FLAGS = i32;
pub const FD_LINKUI: FD_FLAGS = 32768;
pub const FD_PROGRESSUI: FD_FLAGS = 16384;
pub const FD_SIZEPOINT: FD_FLAGS = 2;
pub const FD_UNICODE: FD_FLAGS = -2147483648;
pub const FD_WRITESTIME: FD_FLAGS = 32;
#[repr(C, packed(1))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct FILEDESCRIPTORA {
    pub dwFlags: super::DWORD,
    pub clsid: super::CLSID,
    pub sizel: super::SIZEL,
    pub pointl: super::POINTL,
    pub dwFileAttributes: super::DWORD,
    pub ftCreationTime: super::FILETIME,
    pub ftLastAccessTime: super::FILETIME,
    pub ftLastWriteTime: super::FILETIME,
    pub nFileSizeHigh: super::DWORD,
    pub nFileSizeLow: super::DWORD,
    pub cFileName: [super::CHAR; 260],
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef"))]
impl Default for FILEDESCRIPTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILEDESCRIPTORW {
    pub dwFlags: super::DWORD,
    pub clsid: super::CLSID,
    pub sizel: super::SIZEL,
    pub pointl: super::POINTL,
    pub dwFileAttributes: super::DWORD,
    pub ftCreationTime: super::FILETIME,
    pub ftLastAccessTime: super::FILETIME,
    pub ftLastWriteTime: super::FILETIME,
    pub nFileSizeHigh: super::DWORD,
    pub nFileSizeLow: super::DWORD,
    pub cFileName: [super::WCHAR; 260],
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for FILEDESCRIPTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct FILEGROUPDESCRIPTORA {
    pub cItems: super::UINT,
    pub fgd: [FILEDESCRIPTORA; 1],
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef"))]
impl Default for FILEGROUPDESCRIPTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILEGROUPDESCRIPTORW {
    pub cItems: super::UINT,
    pub fgd: [FILEDESCRIPTORW; 1],
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for FILEGROUPDESCRIPTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FILE_ATTRIBUTES_ARRAY {
    pub cItems: super::UINT,
    pub dwSumFileAttributes: super::DWORD,
    pub dwProductFileAttributes: super::DWORD,
    pub rgdwFileAttributes: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for FILE_ATTRIBUTES_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GIL_ASYNC: i32 = 32;
pub const GIL_CHECKSHIELD: i32 = 512;
pub const GIL_DEFAULTICON: i32 = 64;
pub const GIL_DONTCACHE: i32 = 16;
pub const GIL_FORCENOSHIELD: i32 = 1024;
pub const GIL_FORSHELL: i32 = 2;
pub const GIL_FORSHORTCUT: i32 = 128;
pub const GIL_NOTFILENAME: i32 = 8;
pub const GIL_OPENICON: i32 = 1;
pub const GIL_PERCLASS: i32 = 4;
pub const GIL_PERINSTANCE: i32 = 2;
pub const GIL_SHIELD: i32 = 512;
pub const GIL_SIMULATEDOC: i32 = 1;
pub const GPFIDL_ALTNAME: i32 = 1;
pub const GPFIDL_DEFAULT: i32 = 0;
pub type GPFIDL_FLAGS = i32;
pub const GPFIDL_UNCPRINTER: i32 = 2;
pub type HPSXA = *mut HPSXA__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HPSXA__ {
    pub unused: i32,
}
pub const IDO_SHGIOI_DEFAULT: u32 = 4294967292;
pub const IDO_SHGIOI_LINK: i32 = 268435454;
pub const IDO_SHGIOI_SHARE: i32 = 268435455;
pub const IDO_SHGIOI_SLOWFILE: u32 = 4294967293;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IE4COMPONENT {
    pub dwSize: super::DWORD,
    pub dwID: super::DWORD,
    pub iComponentType: i32,
    pub fChecked: windows_sys::core::BOOL,
    pub fDirty: windows_sys::core::BOOL,
    pub fNoScroll: windows_sys::core::BOOL,
    pub cpPos: COMPPOS,
    pub wszFriendlyName: [super::WCHAR; 260],
    pub wszSource: [super::WCHAR; 2084],
    pub wszSubscribedURL: [super::WCHAR; 2084],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IE4COMPONENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IESHORTCUTFLAGS = i32;
pub const IESHORTCUT_BACKGROUNDTAB: IESHORTCUTFLAGS = 8;
pub const IESHORTCUT_FORCENAVIGATE: IESHORTCUTFLAGS = 4;
pub const IESHORTCUT_NEWBROWSER: IESHORTCUTFLAGS = 1;
pub const IESHORTCUT_OPENNEWTAB: IESHORTCUTFLAGS = 2;
pub const ISHCUTCMDID_COMMITHISTORY: i32 = 2;
pub const ISHCUTCMDID_DOWNLOADICON: i32 = 0;
pub const ISHCUTCMDID_INTSHORTCUTCREATE: i32 = 1;
pub const ISHCUTCMDID_SETUSERAWURL: i32 = 3;
pub const IS_FULLSCREEN: i32 = 2;
pub const IS_NORMAL: i32 = 1;
pub const IS_SPLIT: i32 = 4;
pub const IS_VALIDSIZESTATEBITS: i32 = 7;
pub const IS_VALIDSTATEBITS: u32 = 3221225479;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct ITEMSPACING {
    pub cxSmall: i32,
    pub cySmall: i32,
    pub cxLarge: i32,
    pub cyLarge: i32,
}
pub const KF_FLAG_ALIAS_ONLY: KNOWN_FOLDER_FLAG = -2147483648;
pub const KF_FLAG_CREATE: KNOWN_FOLDER_FLAG = 32768;
pub const KF_FLAG_DEFAULT: KNOWN_FOLDER_FLAG = 0;
pub const KF_FLAG_DEFAULT_PATH: KNOWN_FOLDER_FLAG = 1024;
pub const KF_FLAG_DONT_UNEXPAND: KNOWN_FOLDER_FLAG = 8192;
pub const KF_FLAG_DONT_VERIFY: KNOWN_FOLDER_FLAG = 16384;
pub const KF_FLAG_FORCE_APPCONTAINER_REDIRECTION: KNOWN_FOLDER_FLAG = 131072;
pub const KF_FLAG_FORCE_APP_DATA_REDIRECTION: KNOWN_FOLDER_FLAG = 524288;
pub const KF_FLAG_FORCE_PACKAGE_REDIRECTION: KNOWN_FOLDER_FLAG = 131072;
pub const KF_FLAG_INIT: KNOWN_FOLDER_FLAG = 2048;
pub const KF_FLAG_NOT_PARENT_RELATIVE: KNOWN_FOLDER_FLAG = 512;
pub const KF_FLAG_NO_ALIAS: KNOWN_FOLDER_FLAG = 4096;
pub const KF_FLAG_NO_APPCONTAINER_REDIRECTION: KNOWN_FOLDER_FLAG = 65536;
pub const KF_FLAG_NO_PACKAGE_REDIRECTION: KNOWN_FOLDER_FLAG = 65536;
pub const KF_FLAG_RETURN_FILTER_REDIRECTION_TARGET: KNOWN_FOLDER_FLAG = 262144;
pub const KF_FLAG_SIMPLE_IDLIST: KNOWN_FOLDER_FLAG = 256;
pub type KNOWN_FOLDER_FLAG = i32;
pub type LPACTIVEDESKTOP = *mut core::ffi::c_void;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
pub type LPBROWSEINFOA = *mut BROWSEINFOA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
pub type LPBROWSEINFOW = *mut BROWSEINFOW;
#[cfg(feature = "shared")]
pub type LPCABINETSTATE = *mut CABINETSTATE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCCOMPONENT = *const COMPONENT;
#[cfg(feature = "shared")]
pub type LPCCOMPONENTSOPT = *const COMPONENTSOPT;
#[cfg(feature = "shared")]
pub type LPCCOMPPOS = *const COMPPOS;
#[cfg(feature = "shared")]
pub type LPCCOMPSTATEINFO = *const COMPSTATEINFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCIE4COMPONENT = *const IE4COMPONENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCOMPONENT = *mut COMPONENT;
#[cfg(feature = "shared")]
pub type LPCOMPONENTSOPT = *mut COMPONENTSOPT;
#[cfg(feature = "shared")]
pub type LPCOMPPOS = *mut COMPPOS;
#[cfg(feature = "shared")]
pub type LPCOMPSTATEINFO = *mut COMPSTATEINFO;
#[cfg(feature = "shared")]
pub type LPCPROPPRG = *const PROPPRG;
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPCSFV = *mut CSFV;
#[cfg(feature = "shared")]
pub type LPCWALLPAPEROPT = *const WALLPAPEROPT;
#[cfg(feature = "shared")]
pub type LPDATABLOCK_HEADER = *mut DATABLOCK_HEADER;
#[cfg(feature = "shared")]
pub type LPDBLIST = *mut DATABLOCK_HEADER;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPDROPFILES = *mut DROPFILES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPEXP_DARWIN_LINK = *mut EXP_DARWIN_LINK;
#[cfg(feature = "shared")]
pub type LPEXP_SPECIAL_FOLDER = *mut EXP_SPECIAL_FOLDER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPEXP_SZ_LINK = *mut EXP_SZ_LINK;
pub type LPEXTRACTICONA = *mut core::ffi::c_void;
pub type LPEXTRACTICONW = *mut core::ffi::c_void;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPFILEDESCRIPTORA = *mut FILEDESCRIPTORA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPFILEDESCRIPTORW = *mut FILEDESCRIPTORW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPFILEGROUPDESCRIPTORA = *mut FILEGROUPDESCRIPTORA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPFILEGROUPDESCRIPTORW = *mut FILEGROUPDESCRIPTORW;
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "shared", feature = "shobjidl_core", feature = "windef"))]
pub type LPFNDFMCALLBACK = Option<unsafe extern "system" fn(psf: *mut core::ffi::c_void, hwnd: super::HWND, pdtobj: *mut core::ffi::c_void, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "shobjidl_core", feature = "windef"))]
pub type LPFNVIEWCALLBACK = Option<unsafe extern "system" fn(psvouter: *mut core::ffi::c_void, psf: *mut core::ffi::c_void, hwndmain: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_sys::core::HRESULT>;
#[cfg(feature = "shared")]
pub type LPIDA = *mut CIDA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPIE4COMPONENT = *mut IE4COMPONENT;
#[cfg(all(feature = "shared", feature = "winnetwk"))]
pub type LPNRESARRAY = *mut NRESARRAY;
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "windef", feature = "winnt"))]
pub type LPNT_CONSOLE_PROPS = *mut NT_CONSOLE_PROPS;
#[cfg(feature = "shared")]
pub type LPNT_FE_CONSOLE_PROPS = *mut NT_FE_CONSOLE_PROPS;
#[cfg(feature = "shared")]
pub type LPPROPPRG = *mut PROPPRG;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPQCMINFO = *mut QCMINFO;
#[cfg(feature = "shared")]
pub type LPSHChangeDWORDAsIDList = *mut SHChangeDWORDAsIDList;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHChangeUpdateImageIDList = *mut SHChangeUpdateImageIDList;
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub type LPSHDESCRIPTIONID = *mut SHDESCRIPTIONID;
#[cfg(feature = "shared")]
pub type LPSHELLFLAGSTATE = *mut SHELLFLAGSTATE;
#[cfg(feature = "shared")]
pub type LPSHELLSTATEA = *mut SHELLSTATEA;
#[cfg(feature = "shared")]
pub type LPSHELLSTATEW = *mut SHELLSTATEW;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "shobjidl_core"))]
pub type LPSHFOLDERCUSTOMSETTINGS = *mut SHFOLDERCUSTOMSETTINGS;
#[cfg(feature = "shared")]
pub type LPWALLPAPEROPT = *mut WALLPAPEROPT;
pub const MAX_COLUMN_DESC_LEN: i32 = 128;
pub const MAX_COLUMN_NAME_LEN: i32 = 80;
pub const MM_ADDSEPARATOR: i32 = 1;
pub const MM_DONTREMOVESEPS: i32 = 4;
pub const MM_SUBMENUSHAVEIDS: i32 = 2;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnetwk"))]
#[derive(Clone, Copy)]
pub struct NRESARRAY {
    pub cItems: super::UINT,
    pub nr: [super::NETRESOURCE; 1],
}
#[cfg(all(feature = "shared", feature = "winnetwk"))]
impl Default for NRESARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct NT_CONSOLE_PROPS {
    pub dbh: DATABLOCK_HEADER,
    pub wFillAttribute: super::WORD,
    pub wPopupFillAttribute: super::WORD,
    pub dwScreenBufferSize: super::COORD,
    pub dwWindowSize: super::COORD,
    pub dwWindowOrigin: super::COORD,
    pub nFont: super::DWORD,
    pub nInputBufferSize: super::DWORD,
    pub dwFontSize: super::COORD,
    pub uFontFamily: super::UINT,
    pub uFontWeight: super::UINT,
    pub FaceName: [super::WCHAR; 32],
    pub uCursorSize: super::UINT,
    pub bFullScreen: windows_sys::core::BOOL,
    pub bQuickEdit: windows_sys::core::BOOL,
    pub bInsertMode: windows_sys::core::BOOL,
    pub bAutoPosition: windows_sys::core::BOOL,
    pub uHistoryBufferSize: super::UINT,
    pub uNumberOfHistoryBuffers: super::UINT,
    pub bHistoryNoDup: windows_sys::core::BOOL,
    pub ColorTable: [super::COLORREF; 16],
}
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "windef", feature = "winnt"))]
impl Default for NT_CONSOLE_PROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NT_CONSOLE_PROPS_SIG: u32 = 2684354562;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NT_FE_CONSOLE_PROPS {
    pub dbh: DATABLOCK_HEADER,
    pub uCodePage: super::UINT,
}
pub const NT_FE_CONSOLE_PROPS_SIG: u32 = 2684354564;
pub const NUM_POINTS: i32 = 3;
pub const OAIF_ALLOW_REGISTRATION: tagOPEN_AS_INFO_FLAGS = 1;
pub const OAIF_EXEC: tagOPEN_AS_INFO_FLAGS = 4;
pub const OAIF_FILE_IS_URI: tagOPEN_AS_INFO_FLAGS = 128;
pub const OAIF_FORCE_REGISTRATION: tagOPEN_AS_INFO_FLAGS = 8;
pub const OAIF_HIDE_REGISTRATION: tagOPEN_AS_INFO_FLAGS = 32;
pub const OAIF_REGISTER_EXT: tagOPEN_AS_INFO_FLAGS = 2;
pub const OAIF_URL_PROTOCOL: tagOPEN_AS_INFO_FLAGS = 64;
pub const OFASI_EDIT: i32 = 1;
pub const OFASI_OPENDESKTOP: i32 = 2;
pub const OI_ASYNC: u32 = 4294962926;
pub const OI_DEFAULT: i32 = 0;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct OPENASINFO {
    pub pcszFile: windows_sys::core::PCWSTR,
    pub pcszClass: windows_sys::core::PCWSTR,
    pub oaifInFlags: OPEN_AS_INFO_FLAGS,
}
pub type OPEN_AS_INFO_FLAGS = i32;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
pub type PBROWSEINFOA = *mut BROWSEINFOA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shtypes", feature = "windef"))]
pub type PBROWSEINFOW = *mut BROWSEINFOW;
pub const PCS_FATAL: u32 = 2147483648;
pub const PCS_PATHTOOLONG: i32 = 8;
pub const PCS_REMOVEDCHAR: i32 = 2;
pub const PCS_REPLACEDCHAR: i32 = 1;
pub const PCS_TRUNCATED: i32 = 4;
#[cfg(all(feature = "shared", feature = "shtypes"))]
pub type PDETAILSINFO = *mut DETAILSINFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
pub type PDFMICS = *mut DFMICS;
pub const PDTIMER_PAUSE: i32 = 2;
pub const PDTIMER_RESET: i32 = 1;
pub const PDTIMER_RESUME: i32 = 3;
pub const PIDISF_CACHEDSTICKY: i32 = 2;
pub const PIDISF_CACHEIMAGES: i32 = 16;
pub const PIDISF_FOLLOWALLLINKS: i32 = 32;
pub const PIDISF_RECENTLYCHANGED: i32 = 1;
pub const PIDISM_DONTWATCH: i32 = 2;
pub const PIDISM_GLOBAL: i32 = 0;
pub const PIDISM_WATCH: i32 = 1;
pub const PIDISR_NEEDS_ADD: i32 = 1;
pub const PIDISR_NEEDS_DELETE: i32 = 3;
pub const PIDISR_NEEDS_UPDATE: i32 = 2;
pub const PIDISR_UP_TO_DATE: i32 = 0;
pub const PID_INTSITE_AUTHOR: i32 = 3;
pub const PID_INTSITE_CODEPAGE: i32 = 18;
pub const PID_INTSITE_COMMENT: i32 = 8;
pub const PID_INTSITE_CONTENTCODE: i32 = 11;
pub const PID_INTSITE_CONTENTLEN: i32 = 10;
pub const PID_INTSITE_DESCRIPTION: i32 = 7;
pub const PID_INTSITE_FLAGS: i32 = 9;
pub const PID_INTSITE_ICONFILE: i32 = 21;
pub const PID_INTSITE_ICONINDEX: i32 = 20;
pub const PID_INTSITE_LASTMOD: i32 = 5;
pub const PID_INTSITE_LASTVISIT: i32 = 4;
pub const PID_INTSITE_RECURSE: i32 = 12;
pub const PID_INTSITE_ROAMED: i32 = 34;
pub const PID_INTSITE_SUBSCRIPTION: i32 = 14;
pub const PID_INTSITE_TITLE: i32 = 16;
pub const PID_INTSITE_TRACKING: i32 = 19;
pub const PID_INTSITE_URL: i32 = 15;
pub const PID_INTSITE_VISITCOUNT: i32 = 6;
pub const PID_INTSITE_WATCH: i32 = 13;
pub const PID_INTSITE_WHATSNEW: i32 = 2;
pub const PID_IS_AUTHOR: i32 = 11;
pub const PID_IS_COMMENT: i32 = 13;
pub const PID_IS_DESCRIPTION: i32 = 12;
pub const PID_IS_HOTKEY: i32 = 6;
pub const PID_IS_ICONFILE: i32 = 9;
pub const PID_IS_ICONINDEX: i32 = 8;
pub const PID_IS_NAME: i32 = 4;
pub const PID_IS_ROAMED: i32 = 15;
pub const PID_IS_SHOWCMD: i32 = 7;
pub const PID_IS_URL: i32 = 2;
pub const PID_IS_WHATSNEW: i32 = 10;
pub const PID_IS_WORKINGDIR: i32 = 5;
pub const PIFDEFFILESIZE: i32 = 80;
pub const PIFDEFPATHSIZE: i32 = 64;
pub const PIFMAXFILEPATH: i32 = 260;
pub const PIFNAMESIZE: i32 = 30;
pub const PIFPARAMSSIZE: i32 = 64;
pub const PIFSHDATASIZE: i32 = 64;
pub const PIFSHPROGSIZE: i32 = 64;
pub const PIFSTARTLOCSIZE: i32 = 63;
pub type POPENASINFO = *mut OPENASINFO;
#[cfg(feature = "shared")]
pub type PPROPPRG = *mut PROPPRG;
pub const PRF_DONTFINDLNK: i32 = 8;
pub const PRF_FIRSTDIRDEF: i32 = 4;
pub const PRF_REQUIREABSOLUTE: i32 = 16;
pub const PRF_TRYPROGRAMEXTENSIONS: i32 = 3;
pub const PRF_VERIFYEXISTS: i32 = 1;
pub const PROGDLG_AUTOTIME: i32 = 2;
pub const PROGDLG_MARQUEEPROGRESS: i32 = 32;
pub const PROGDLG_MODAL: i32 = 1;
pub const PROGDLG_NOCANCEL: i32 = 64;
pub const PROGDLG_NOMINIMIZE: i32 = 8;
pub const PROGDLG_NOPROGRESSBAR: i32 = 16;
pub const PROGDLG_NORMAL: i32 = 0;
pub const PROGDLG_NOTIME: i32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PROPPRG {
    pub flPrg: super::WORD,
    pub flPrgInit: super::WORD,
    pub achTitle: [super::CHAR; 30],
    pub achCmdLine: [super::CHAR; 128],
    pub achWorkDir: [super::CHAR; 64],
    pub wHotKey: super::WORD,
    pub achIconFile: [super::CHAR; 80],
    pub wIconIndex: super::WORD,
    pub dwEnhModeFlags: super::DWORD,
    pub dwRealModeFlags: super::DWORD,
    pub achOtherFile: [super::CHAR; 80],
    pub achPIFFile: [super::CHAR; 260],
}
#[cfg(feature = "shared")]
impl Default for PROPPRG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct QCMINFO {
    pub hmenu: super::HMENU,
    pub indexMenu: super::UINT,
    pub idCmdFirst: super::UINT,
    pub idCmdLast: super::UINT,
    pub pIdMap: *const QCMINFO_IDMAP,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct QCMINFO_IDMAP {
    pub nMaxIds: super::UINT,
    pub pIdList: [QCMINFO_IDMAP_PLACEMENT; 1],
}
#[cfg(feature = "shared")]
impl Default for QCMINFO_IDMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QCMINFO_IDMAP_PLACEMENT {
    pub id: super::UINT,
    pub fFlags: super::UINT,
}
pub const QCMINFO_PLACE_AFTER: i32 = 1;
pub const QCMINFO_PLACE_BEFORE: i32 = 0;
pub const QIF_CACHED: i32 = 1;
pub const QIF_DONTEXPANDFOLDER: i32 = 2;
pub const QITIPF_DEFAULT: i32 = 0;
pub const QITIPF_LINKNOTARGET: i32 = 2;
pub const QITIPF_LINKUSETARGET: i32 = 4;
pub const QITIPF_SINGLELINE: i32 = 16;
pub const QITIPF_USENAME: i32 = 1;
pub const QITIPF_USESLOWTIP: i32 = 8;
pub const REGSTR_PATH_SPECIAL_FOLDERS: windows_sys::core::PCSTR = windows_sys::core::s!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders");
pub type RESTRICTIONS = i32;
pub const REST_ALLOWBITBUCKDRIVES: RESTRICTIONS = 1073741905;
pub const REST_ALLOWCOMMENTTOGGLE: RESTRICTIONS = 1090519044;
pub const REST_ALLOWFILECLSIDJUNCTIONS: RESTRICTIONS = 1073741980;
pub const REST_ALLOWLEGACYWEBVIEW: RESTRICTIONS = 1073741955;
pub const REST_ALLOWUNHASHEDWEBVIEW: RESTRICTIONS = 1073741954;
pub const REST_ARP_DONTGROUPPATCHES: RESTRICTIONS = 1073741996;
pub const REST_ARP_NOADDPAGE: RESTRICTIONS = 1073741867;
pub const REST_ARP_NOARP: RESTRICTIONS = 1073741865;
pub const REST_ARP_NOCHOOSEPROGRAMSPAGE: RESTRICTIONS = 1073741997;
pub const REST_ARP_NOREMOVEPAGE: RESTRICTIONS = 1073741866;
pub const REST_ARP_NOWINSETUPPAGE: RESTRICTIONS = 1073741868;
pub const REST_ARP_ShowPostSetup: RESTRICTIONS = 1073741861;
pub const REST_BITBUCKCONFIRMDELETE: RESTRICTIONS = 1073741941;
pub const REST_BITBUCKNOPROP: RESTRICTIONS = 1073741942;
pub const REST_BITBUCKNUKEONDELETE: RESTRICTIONS = 1073741940;
pub const REST_CLASSICSHELL: RESTRICTIONS = 1073741832;
pub const REST_CLEARRECENTDOCSONEXIT: RESTRICTIONS = 1073741831;
pub const REST_DISALLOWCPL: RESTRICTIONS = 1073741889;
pub const REST_DISALLOWRUN: RESTRICTIONS = 1073741886;
pub const REST_DONTRETRYBADNETNAME: RESTRICTIONS = 1073741979;
pub const REST_DONTSHOWSUPERHIDDEN: RESTRICTIONS = 1073741879;
pub const REST_ENFORCESHELLEXTSECURITY: RESTRICTIONS = 1048576;
pub const REST_ENUMWORKGROUP: RESTRICTIONS = 1073741864;
pub const REST_FORCEACTIVEDESKTOPON: RESTRICTIONS = 1073741898;
pub const REST_FORCECOPYACLWITHFILE: RESTRICTIONS = 1073741851;
pub const REST_FORCESTARTMENULOGOFF: RESTRICTIONS = 1073741874;
pub const REST_GREYMSIADS: RESTRICTIONS = 1073741869;
pub const REST_HASFINDCOMPUTERS: RESTRICTIONS = 1073741858;
pub const REST_HIDECLOCK: RESTRICTIONS = 1073741936;
pub const REST_HIDERUNASVERB: RESTRICTIONS = 1073741948;
pub const REST_INHERITCONSOLEHANDLES: RESTRICTIONS = 1073741958;
pub const REST_INTELLIMENUS: RESTRICTIONS = 1073741859;
pub const REST_LINKRESOLVEIGNORELINKINFO: RESTRICTIONS = 2097152;
pub const REST_MYCOMPNOPROP: RESTRICTIONS = 1073741912;
pub const REST_MYDOCSNOPROP: RESTRICTIONS = 1073741913;
pub const REST_MYDOCSONNET: RESTRICTIONS = 262144;
pub const REST_MaxRecentDocs: RESTRICTIONS = 1073741872;
pub const REST_NOACTIVEDESKTOP: RESTRICTIONS = 1073741828;
pub const REST_NOACTIVEDESKTOPCHANGES: RESTRICTIONS = 1073741829;
pub const REST_NOADDDESKCOMP: RESTRICTIONS = 1073741843;
pub const REST_NOAUTOTRAYNOTIFY: RESTRICTIONS = 1073741909;
pub const REST_NOCDBURNING: RESTRICTIONS = 1073741911;
pub const REST_NOCHANGEMAPPEDDRIVECOMMENT: RESTRICTIONS = 1073741871;
pub const REST_NOCHANGEMAPPEDDRIVELABEL: RESTRICTIONS = 1073741870;
pub const REST_NOCHANGESTARMENU: RESTRICTIONS = 1073741856;
pub const REST_NOCHANGINGWALLPAPER: RESTRICTIONS = 1073741841;
pub const REST_NOCLOSE: RESTRICTIONS = 2;
pub const REST_NOCLOSEDESKCOMP: RESTRICTIONS = 1073741845;
pub const REST_NOCLOSE_DRAGDROPBAND: RESTRICTIONS = 1073741846;
pub const REST_NOCOLORCHOICE: RESTRICTIONS = 1073741919;
pub const REST_NOCOMMONGROUPS: RESTRICTIONS = 4194304;
pub const REST_NOCONTROLPANEL: RESTRICTIONS = 1073741863;
pub const REST_NOCONTROLPANELBARRICADE: RESTRICTIONS = 1073741907;
pub const REST_NOCSC: RESTRICTIONS = 1073741862;
pub const REST_NOCURRENTUSERRUN: RESTRICTIONS = 1073741895;
pub const REST_NOCURRENTUSERRUNONCE: RESTRICTIONS = 1073741897;
pub const REST_NOCUSTOMIZETHISFOLDER: RESTRICTIONS = 1073741876;
pub const REST_NOCUSTOMIZEWEBVIEW: RESTRICTIONS = 1073741833;
pub const REST_NODELDESKCOMP: RESTRICTIONS = 1073741844;
pub const REST_NODESKCOMP: RESTRICTIONS = 1073741842;
pub const REST_NODESKTOP: RESTRICTIONS = 64;
pub const REST_NODESKTOPCLEANUP: RESTRICTIONS = 1073741939;
pub const REST_NODISCONNECT: RESTRICTIONS = 1090519041;
pub const REST_NODISPBACKGROUND: RESTRICTIONS = 1073741943;
pub const REST_NODISPLAYAPPEARANCEPAGE: RESTRICTIONS = 1073741915;
pub const REST_NODISPLAYCPL: RESTRICTIONS = 1073741947;
pub const REST_NODISPSCREENSAVEPG: RESTRICTIONS = 1073741944;
pub const REST_NODISPSCREENSAVEPREVIEW: RESTRICTIONS = 1073741946;
pub const REST_NODISPSETTINGSPG: RESTRICTIONS = 1073741945;
pub const REST_NODRIVEAUTORUN: RESTRICTIONS = 512;
pub const REST_NODRIVES: RESTRICTIONS = 256;
pub const REST_NODRIVETYPEAUTORUN: RESTRICTIONS = 1024;
pub const REST_NOEDITDESKCOMP: RESTRICTIONS = 1073741848;
pub const REST_NOENCRYPTION: RESTRICTIONS = 1073741877;
pub const REST_NOENCRYPTONMOVE: RESTRICTIONS = 1073741893;
pub const REST_NOENTIRENETWORK: RESTRICTIONS = 1073741938;
pub const REST_NOENUMENTIRENETWORK: RESTRICTIONS = 1073741971;
pub const REST_NOEXITTODOS: RESTRICTIONS = 524288;
pub const REST_NOFAVORITESMENU: RESTRICTIONS = 1073741830;
pub const REST_NOFILEASSOCIATE: RESTRICTIONS = 1090519043;
pub const REST_NOFILEMENU: RESTRICTIONS = 8;
pub const REST_NOFIND: RESTRICTIONS = 128;
pub const REST_NOFOLDEROPTIONS: RESTRICTIONS = 1073741857;
pub const REST_NOFORGETSOFTWAREUPDATE: RESTRICTIONS = 1073741853;
pub const REST_NOHARDWARETAB: RESTRICTIONS = 1073741881;
pub const REST_NOHTMLWALLPAPER: RESTRICTIONS = 1073741840;
pub const REST_NOINTERNETICON: RESTRICTIONS = 1073741825;
pub const REST_NOINTERNETOPENWITH: RESTRICTIONS = 1073741973;
pub const REST_NOLOCALMACHINERUN: RESTRICTIONS = 1073741894;
pub const REST_NOLOCALMACHINERUNONCE: RESTRICTIONS = 1073741896;
pub const REST_NOLOWDISKSPACECHECKS: RESTRICTIONS = 1073741937;
pub const REST_NOMANAGEMYCOMPUTERVERB: RESTRICTIONS = 1073741884;
pub const REST_NOMOVINGBAND: RESTRICTIONS = 1073741847;
pub const REST_NOMYCOMPUTERICON: RESTRICTIONS = 1073741923;
pub const REST_NONE: RESTRICTIONS = 0;
pub const REST_NONETCONNECTDISCONNECT: RESTRICTIONS = 134217728;
pub const REST_NONETCRAWL: RESTRICTIONS = 1073741901;
pub const REST_NONETHOOD: RESTRICTIONS = 2048;
pub const REST_NONETWORKCONNECTIONS: RESTRICTIONS = 1073741873;
pub const REST_NONLEGACYSHELLMODE: RESTRICTIONS = 1073741906;
pub const REST_NOONLINEPRINTSWIZARD: RESTRICTIONS = 1073741952;
pub const REST_NOPRINTERADD: RESTRICTIONS = 65536;
pub const REST_NOPRINTERDELETE: RESTRICTIONS = 32768;
pub const REST_NOPRINTERTABS: RESTRICTIONS = 16384;
pub const REST_NOPUBLISHWIZARD: RESTRICTIONS = 1073741951;
pub const REST_NORECENTDOCSHISTORY: RESTRICTIONS = 1073741826;
pub const REST_NORECENTDOCSMENU: RESTRICTIONS = 1073741827;
pub const REST_NOREMOTECHANGENOTIFY: RESTRICTIONS = 1073741969;
pub const REST_NOREMOTERECURSIVEEVENTS: RESTRICTIONS = 1073741961;
pub const REST_NORESOLVESEARCH: RESTRICTIONS = 1073741849;
pub const REST_NORESOLVETRACK: RESTRICTIONS = 1073741850;
pub const REST_NORUN: RESTRICTIONS = 1;
pub const REST_NORUNASINSTALLPROMPT: RESTRICTIONS = 1073741882;
pub const REST_NOSAVESET: RESTRICTIONS = 4;
pub const REST_NOSECURITY: RESTRICTIONS = 1090519042;
pub const REST_NOSETACTIVEDESKTOP: RESTRICTIONS = 1073741854;
pub const REST_NOSETFOLDERS: RESTRICTIONS = 16;
pub const REST_NOSETTASKBAR: RESTRICTIONS = 32;
pub const REST_NOSETTINGSASSIST: RESTRICTIONS = 536870912;
pub const REST_NOSHAREDDOCUMENTS: RESTRICTIONS = 1073741902;
pub const REST_NOSHELLSEARCHBUTTON: RESTRICTIONS = 1073741880;
pub const REST_NOSIZECHOICE: RESTRICTIONS = 1073741918;
pub const REST_NOSMBALLOONTIP: RESTRICTIONS = 1073741890;
pub const REST_NOSMCONFIGUREPROGRAMS: RESTRICTIONS = 1073741935;
pub const REST_NOSMEJECTPC: RESTRICTIONS = 1073741927;
pub const REST_NOSMHELP: RESTRICTIONS = 1073741891;
pub const REST_NOSMMFUPROGRAMS: RESTRICTIONS = 1073741929;
pub const REST_NOSMMOREPROGRAMS: RESTRICTIONS = 1073741928;
pub const REST_NOSMMYDOCS: RESTRICTIONS = 1073741903;
pub const REST_NOSMMYMUSIC: RESTRICTIONS = 1073741926;
pub const REST_NOSMMYPICS: RESTRICTIONS = 1073741904;
pub const REST_NOSMNETWORKPLACES: RESTRICTIONS = 1073741924;
pub const REST_NOSMPINNEDLIST: RESTRICTIONS = 1073741925;
pub const REST_NOSTARTMENUSUBFOLDERS: RESTRICTIONS = 131072;
pub const REST_NOSTARTPAGE: RESTRICTIONS = 1073741908;
pub const REST_NOSTARTPANEL: RESTRICTIONS = 1073741914;
pub const REST_NOSTRCMPLOGICAL: RESTRICTIONS = 1073741950;
pub const REST_NOTASKGROUPING: RESTRICTIONS = 1073741910;
pub const REST_NOTHEMESTAB: RESTRICTIONS = 1073741916;
pub const REST_NOTHUMBNAILCACHE: RESTRICTIONS = 1073741949;
pub const REST_NOTOOLBARSONTASKBAR: RESTRICTIONS = 1073741931;
pub const REST_NOTRAYCONTEXTMENU: RESTRICTIONS = 33554432;
pub const REST_NOTRAYITEMSDISPLAY: RESTRICTIONS = 1073741930;
pub const REST_NOUPDATEWINDOWS: RESTRICTIONS = 1073741855;
pub const REST_NOUPNPINSTALL: RESTRICTIONS = 1073741981;
pub const REST_NOUSERNAMEINSTARTPANEL: RESTRICTIONS = 1073741922;
pub const REST_NOVIEWCONTEXTMENU: RESTRICTIONS = 67108864;
pub const REST_NOVIEWONDRIVE: RESTRICTIONS = 1073741900;
pub const REST_NOVISUALSTYLECHOICE: RESTRICTIONS = 1073741917;
pub const REST_NOWEB: RESTRICTIONS = 16777216;
pub const REST_NOWEBSERVICES: RESTRICTIONS = 1073741953;
pub const REST_NOWEBVIEW: RESTRICTIONS = 1073741875;
pub const REST_NOWELCOMESCREEN: RESTRICTIONS = 1073741887;
pub const REST_NOWINKEYS: RESTRICTIONS = 1073741892;
pub const REST_PROMPTRUNASINSTALLNETPATH: RESTRICTIONS = 1073741883;
pub const REST_RESTRICTCPL: RESTRICTIONS = 1073741888;
pub const REST_RESTRICTRUN: RESTRICTIONS = 8192;
pub const REST_REVERTWEBVIEWSECURITY: RESTRICTIONS = 1073741956;
pub const REST_RUNDLGMEMCHECKBOX: RESTRICTIONS = 1073741860;
pub const REST_SEPARATEDESKTOPPROCESS: RESTRICTIONS = 8388608;
pub const REST_SETVISUALSTYLE: RESTRICTIONS = 1073741920;
pub const REST_STARTBANNER: RESTRICTIONS = 4096;
pub const REST_STARTMENULOGOFF: RESTRICTIONS = 268435456;
pub const REST_STARTRUNNOHOMEPATH: RESTRICTIONS = 1073741921;
pub const SCNRT_DISABLE: SCNRT_STATUS = 1;
pub const SCNRT_ENABLE: SCNRT_STATUS = 0;
pub type SCNRT_STATUS = i32;
pub const SFVM_ADDPROPERTYPAGES: i32 = 47;
pub const SFVM_BACKGROUNDENUM: i32 = 32;
pub const SFVM_BACKGROUNDENUMDONE: i32 = 48;
pub const SFVM_COLUMNCLICK: i32 = 24;
pub const SFVM_DEFITEMCOUNT: i32 = 26;
pub const SFVM_DEFVIEWMODE: i32 = 27;
pub const SFVM_DIDDRAGDROP: i32 = 36;
pub const SFVM_FSNOTIFY: i32 = 14;
pub const SFVM_GETANIMATION: i32 = 68;
pub const SFVM_GETBUTTONINFO: i32 = 5;
pub const SFVM_GETBUTTONS: i32 = 6;
pub const SFVM_GETDETAILSOF: i32 = 23;
pub const SFVM_GETHELPTEXT: i32 = 3;
pub const SFVM_GETHELPTOPIC: i32 = 63;
pub const SFVM_GETNOTIFY: i32 = 49;
pub const SFVM_GETPANE: i32 = 59;
pub const SFVM_GETSORTDEFAULTS: i32 = 53;
pub const SFVM_GETTOOLTIPTEXT: i32 = 4;
pub const SFVM_GETZONE: i32 = 58;
#[repr(C, packed(8))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SFVM_HELPTOPIC_DATA {
    pub wszHelpFile: [super::WCHAR; 260],
    pub wszHelpTopic: [super::WCHAR; 260],
}
#[cfg(feature = "winnt")]
impl Default for SFVM_HELPTOPIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SFVM_INITMENUPOPUP: i32 = 7;
pub const SFVM_INVOKECOMMAND: i32 = 2;
pub const SFVM_MERGEMENU: i32 = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SFVM_PROPPAGE_DATA {
    pub dwReserved: super::DWORD,
    pub pfn: super::LPFNADDPROPSHEETPAGE,
    pub lParam: super::LPARAM,
}
pub const SFVM_QUERYFSNOTIFY: i32 = 25;
pub const SFVM_SETISFV: i32 = 39;
pub const SFVM_SIZE: i32 = 57;
pub const SFVM_THISIDLIST: i32 = 41;
pub const SFVM_UNMERGEMENU: i32 = 28;
pub const SFVM_UPDATESTATUSBAR: i32 = 31;
pub const SFVM_WINDOWCREATED: i32 = 15;
pub const SFVSOC_INVALIDATE_ALL: i32 = 1;
pub const SFVSOC_NOSCROLL: i32 = 2;
pub const SFVS_SELECT_ALLITEMS: i32 = 1;
pub const SFVS_SELECT_INVERT: i32 = 2;
pub const SFVS_SELECT_NONE: i32 = 0;
#[repr(C, packed(8))]
#[cfg(all(feature = "oleidl", feature = "shared", feature = "shobjidl_core"))]
#[derive(Clone, Copy, Default)]
pub struct SFV_CREATE {
    pub cbSize: super::UINT,
    pub pshf: *mut core::ffi::c_void,
    pub psvOuter: *mut core::ffi::c_void,
    pub psfvcb: *mut core::ffi::c_void,
}
pub type SHARD = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shobjidl_core")]
#[derive(Clone, Copy, Default)]
pub struct SHARDAPPIDINFO {
    pub psi: *mut core::ffi::c_void,
    pub pszAppID: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "shtypes"))]
#[derive(Clone, Copy, Default)]
pub struct SHARDAPPIDINFOIDLIST {
    pub pidl: super::LPCITEMIDLIST,
    pub pszAppID: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shobjidl_core")]
#[derive(Clone, Copy, Default)]
pub struct SHARDAPPIDINFOLINK {
    pub psl: *mut core::ffi::c_void,
    pub pszAppID: windows_sys::core::PCWSTR,
}
pub const SHARD_APPIDINFO: SHARD = 4;
pub const SHARD_APPIDINFOIDLIST: SHARD = 5;
pub const SHARD_APPIDINFOLINK: SHARD = 7;
pub const SHARD_LINK: SHARD = 6;
pub const SHARD_PATH: SHARD = 2;
pub const SHARD_PATHA: SHARD = 2;
pub const SHARD_PATHW: SHARD = 3;
pub const SHARD_PIDL: SHARD = 1;
pub const SHARD_SHELLITEM: SHARD = 8;
pub const SHCNEE_MSI_CHANGE: i32 = 4;
pub const SHCNEE_MSI_UNINSTALL: i32 = 5;
pub const SHCNEE_ORDERCHANGED: i32 = 2;
pub const SHCNE_ALLEVENTS: i32 = 2147483647;
pub const SHCNE_ASSOCCHANGED: i32 = 134217728;
pub const SHCNE_ATTRIBUTES: i32 = 2048;
pub const SHCNE_CREATE: i32 = 2;
pub const SHCNE_DELETE: i32 = 4;
pub const SHCNE_DISKEVENTS: i32 = 145439;
pub const SHCNE_DRIVEADD: i32 = 256;
pub const SHCNE_DRIVEADDGUI: i32 = 65536;
pub const SHCNE_DRIVEREMOVED: i32 = 128;
pub const SHCNE_EXTENDED_EVENT: i32 = 67108864;
pub const SHCNE_FREESPACE: i32 = 262144;
pub const SHCNE_GLOBALEVENTS: i32 = 201687520;
pub const SHCNE_INTERRUPT: u32 = 2147483648;
pub const SHCNE_MEDIAINSERTED: i32 = 32;
pub const SHCNE_MEDIAREMOVED: i32 = 64;
pub const SHCNE_MKDIR: i32 = 8;
pub const SHCNE_NETSHARE: i32 = 512;
pub const SHCNE_NETUNSHARE: i32 = 1024;
pub const SHCNE_RENAMEFOLDER: i32 = 131072;
pub const SHCNE_RENAMEITEM: i32 = 1;
pub const SHCNE_RMDIR: i32 = 16;
pub const SHCNE_SERVERDISCONNECT: i32 = 16384;
pub const SHCNE_UPDATEDIR: i32 = 4096;
pub const SHCNE_UPDATEIMAGE: i32 = 32768;
pub const SHCNE_UPDATEITEM: i32 = 8192;
pub const SHCNF_DWORD: i32 = 3;
pub const SHCNF_FLUSH: i32 = 4096;
pub const SHCNF_FLUSHNOWAIT: i32 = 12288;
pub const SHCNF_IDLIST: i32 = 0;
pub const SHCNF_NOTIFYRECURSIVE: i32 = 65536;
pub const SHCNF_PATH: i32 = 1;
pub const SHCNF_PATHA: i32 = 1;
pub const SHCNF_PATHW: i32 = 5;
pub const SHCNF_PRINTER: i32 = 2;
pub const SHCNF_PRINTERA: i32 = 2;
pub const SHCNF_PRINTERW: i32 = 6;
pub const SHCNF_TYPE: i32 = 255;
pub const SHCNRF_InterruptLevel: i32 = 1;
pub const SHCNRF_NewDelivery: i32 = 32768;
pub const SHCNRF_RecursiveInterrupt: i32 = 4096;
pub const SHCNRF_ShellLevel: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SHChangeDWORDAsIDList {
    pub cb: super::USHORT,
    pub dwItem1: super::DWORD,
    pub dwItem2: super::DWORD,
    pub cbZero: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "shtypes"))]
#[derive(Clone, Copy, Default)]
pub struct SHChangeNotifyEntry {
    pub pidl: super::LPCITEMIDLIST,
    pub fRecursive: windows_sys::core::BOOL,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SHChangeUpdateImageIDList {
    pub cb: super::USHORT,
    pub iIconIndex: i32,
    pub iCurIndex: i32,
    pub uFlags: super::UINT,
    pub dwProcessID: super::DWORD,
    pub szName: [super::WCHAR; 260],
    pub cbZero: super::USHORT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SHChangeUpdateImageIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SHDESCRIPTIONID {
    pub dwDescriptionId: super::DWORD,
    pub clsid: super::CLSID,
}
pub const SHDID_COMPUTER_AUDIO: i32 = 19;
pub const SHDID_COMPUTER_CDROM: i32 = 10;
pub const SHDID_COMPUTER_DRIVE35: i32 = 5;
pub const SHDID_COMPUTER_DRIVE525: i32 = 6;
pub const SHDID_COMPUTER_FIXED: i32 = 8;
pub const SHDID_COMPUTER_IMAGING: i32 = 18;
pub const SHDID_COMPUTER_NETDRIVE: i32 = 9;
pub const SHDID_COMPUTER_OTHER: i32 = 12;
pub const SHDID_COMPUTER_RAMDISK: i32 = 11;
pub const SHDID_COMPUTER_REMOVABLE: i32 = 7;
pub const SHDID_COMPUTER_SHAREDDOCS: i32 = 20;
pub const SHDID_FS_DIRECTORY: i32 = 3;
pub const SHDID_FS_FILE: i32 = 2;
pub const SHDID_FS_OTHER: i32 = 4;
pub const SHDID_MOBILE_DEVICE: i32 = 21;
pub const SHDID_NET_DOMAIN: i32 = 13;
pub const SHDID_NET_OTHER: i32 = 17;
pub const SHDID_NET_RESTOFNET: i32 = 16;
pub const SHDID_NET_SERVER: i32 = 14;
pub const SHDID_NET_SHARE: i32 = 15;
pub const SHDID_REMOTE_DESKTOP_DRIVE: i32 = 22;
pub const SHDID_ROOT_REGITEM: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SHELLFLAGSTATE {
    pub _bitfield: i32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SHELLSTATEA {
    pub _bitfield1: i32,
    pub dwWin95Unused: super::DWORD,
    pub uWin95Unused: super::UINT,
    pub lParamSort: super::LONG,
    pub iSortDirection: i32,
    pub version: super::UINT,
    pub uNotUsed: super::UINT,
    pub _bitfield2: i32,
}
pub const SHELLSTATEVERSION_IE4: i32 = 9;
pub const SHELLSTATEVERSION_WIN2K: i32 = 10;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SHELLSTATEW {
    pub _bitfield1: i32,
    pub dwWin95Unused: super::DWORD,
    pub uWin95Unused: super::UINT,
    pub lParamSort: super::LONG,
    pub iSortDirection: i32,
    pub version: super::UINT,
    pub uNotUsed: super::UINT,
    pub _bitfield2: i32,
}
#[cfg(feature = "shared")]
pub const SHELLSTATE_SIZE_IE4: super::LONG = 0x18_u32 as _;
#[cfg(feature = "shared")]
pub const SHELLSTATE_SIZE_NT4: super::LONG = 0x14_u32 as _;
#[cfg(target_arch = "x86")]
pub const SHELLSTATE_SIZE_WIN2K: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SHELLSTATE_SIZE_WIN2K: u64 = 32;
#[cfg(feature = "shared")]
pub const SHELLSTATE_SIZE_WIN95: super::LONG = 0xC_u32 as _;
pub type SHELL_LINK_DATA_FLAGS = i32;
pub const SHFMT_CANCEL: u32 = 4294967294;
pub const SHFMT_ERROR: u32 = 4294967295;
pub const SHFMT_ID_DEFAULT: i32 = 65535;
pub const SHFMT_NOFORMAT: u32 = 4294967293;
pub const SHFMT_OPT_FULL: i32 = 1;
pub const SHFMT_OPT_SYSONLY: i32 = 2;
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "shobjidl_core"))]
#[derive(Clone, Copy, Default)]
pub struct SHFOLDERCUSTOMSETTINGS {
    pub dwSize: super::DWORD,
    pub dwMask: super::DWORD,
    pub pvid: *mut super::SHELLVIEWID,
    pub pszWebViewTemplate: windows_sys::core::PWSTR,
    pub cchWebViewTemplate: super::DWORD,
    pub pszWebViewTemplateVersion: windows_sys::core::PWSTR,
    pub pszInfoTip: windows_sys::core::PWSTR,
    pub cchInfoTip: super::DWORD,
    pub pclsid: *mut super::CLSID,
    pub dwFlags: super::DWORD,
    pub pszIconFile: windows_sys::core::PWSTR,
    pub cchIconFile: super::DWORD,
    pub iIconIndex: i32,
    pub pszLogo: windows_sys::core::PWSTR,
    pub cchLogo: super::DWORD,
}
pub const SHGDFIL_DESCRIPTIONID: i32 = 3;
pub const SHGDFIL_FINDDATA: i32 = 1;
pub const SHGDFIL_NETRESOURCE: i32 = 2;
pub type SHGFP_TYPE = i32;
pub const SHGFP_TYPE_CURRENT: SHGFP_TYPE = 0;
pub const SHGFP_TYPE_DEFAULT: SHGFP_TYPE = 1;
pub const SHOP_FILEPATH: i32 = 2;
pub const SHOP_PRINTERNAME: i32 = 1;
pub const SHOP_VOLUMEGUID: i32 = 4;
pub const SHPPFW_ASKDIRCREATE: i32 = 2;
pub const SHPPFW_DEFAULT: i32 = 1;
pub const SHPPFW_DIRCREATE: i32 = 1;
pub const SHPPFW_IGNOREFILENAME: i32 = 4;
pub const SHPPFW_MEDIACHECKONLY: i32 = 16;
pub const SHPPFW_NONE: i32 = 0;
pub const SHPPFW_NOWRITECHECK: i32 = 8;
pub const SIOM_ICONINDEX: i32 = 2;
pub const SIOM_OVERLAYINDEX: i32 = 1;
pub const SIOM_RESERVED_DEFAULT: i32 = 3;
pub const SIOM_RESERVED_LINK: i32 = 1;
pub const SIOM_RESERVED_SHARED: i32 = 0;
pub const SIOM_RESERVED_SLOWFILE: i32 = 2;
pub const SLDF_ALLOW_LINK_TO_LINK: SHELL_LINK_DATA_FLAGS = 8388608;
pub const SLDF_DEFAULT: SHELL_LINK_DATA_FLAGS = 0;
pub const SLDF_DISABLE_KNOWNFOLDER_RELATIVE_TRACKING: SHELL_LINK_DATA_FLAGS = 2097152;
pub const SLDF_DISABLE_LINK_PATH_TRACKING: SHELL_LINK_DATA_FLAGS = 1048576;
pub const SLDF_ENABLE_TARGET_METADATA: SHELL_LINK_DATA_FLAGS = 524288;
pub const SLDF_FORCE_NO_LINKINFO: SHELL_LINK_DATA_FLAGS = 256;
pub const SLDF_FORCE_NO_LINKTRACK: SHELL_LINK_DATA_FLAGS = 262144;
pub const SLDF_FORCE_UNCNAME: SHELL_LINK_DATA_FLAGS = 65536;
pub const SLDF_HAS_ARGS: SHELL_LINK_DATA_FLAGS = 32;
pub const SLDF_HAS_DARWINID: SHELL_LINK_DATA_FLAGS = 4096;
pub const SLDF_HAS_EXP_ICON_SZ: SHELL_LINK_DATA_FLAGS = 16384;
pub const SLDF_HAS_EXP_SZ: SHELL_LINK_DATA_FLAGS = 512;
pub const SLDF_HAS_ICONLOCATION: SHELL_LINK_DATA_FLAGS = 64;
pub const SLDF_HAS_ID_LIST: SHELL_LINK_DATA_FLAGS = 1;
pub const SLDF_HAS_LINK_INFO: SHELL_LINK_DATA_FLAGS = 2;
pub const SLDF_HAS_NAME: SHELL_LINK_DATA_FLAGS = 4;
pub const SLDF_HAS_RELPATH: SHELL_LINK_DATA_FLAGS = 8;
pub const SLDF_HAS_WORKINGDIR: SHELL_LINK_DATA_FLAGS = 16;
pub const SLDF_KEEP_LOCAL_IDLIST_FOR_UNC_TARGET: SHELL_LINK_DATA_FLAGS = 67108864;
pub const SLDF_NO_KF_ALIAS: SHELL_LINK_DATA_FLAGS = 4194304;
pub const SLDF_NO_PIDL_ALIAS: SHELL_LINK_DATA_FLAGS = 32768;
pub const SLDF_PERSIST_VOLUME_ID_RELATIVE: SHELL_LINK_DATA_FLAGS = 134217728;
pub const SLDF_PREFER_ENVIRONMENT_PATH: SHELL_LINK_DATA_FLAGS = 33554432;
pub const SLDF_RESERVED: SHELL_LINK_DATA_FLAGS = -2147483648;
pub const SLDF_RUNAS_USER: SHELL_LINK_DATA_FLAGS = 8192;
pub const SLDF_RUN_IN_SEPARATE: SHELL_LINK_DATA_FLAGS = 1024;
pub const SLDF_RUN_WITH_SHIMLAYER: SHELL_LINK_DATA_FLAGS = 131072;
pub const SLDF_UNALIAS_ON_SAVE: SHELL_LINK_DATA_FLAGS = 16777216;
pub const SLDF_UNICODE: SHELL_LINK_DATA_FLAGS = 128;
pub const SLDF_VALID: SHELL_LINK_DATA_FLAGS = 268433407;
pub const SSF_AUTOCHECKSELECT: i32 = 8388608;
pub const SSF_DESKTOPHTML: i32 = 512;
pub const SSF_DONTPRETTYPATH: i32 = 2048;
pub const SSF_DOUBLECLICKINWEBVIEW: i32 = 128;
pub const SSF_FILTER: i32 = 65536;
pub const SSF_HIDDENFILEEXTS: i32 = 4;
pub const SSF_HIDEICONS: i32 = 16384;
pub const SSF_ICONSONLY: i32 = 16777216;
pub const SSF_MAPNETDRVBUTTON: i32 = 4096;
pub const SSF_NOCONFIRMRECYCLE: i32 = 32768;
pub const SSF_NONETCRAWLING: i32 = 1048576;
pub const SSF_SEPPROCESS: i32 = 524288;
pub const SSF_SERVERADMINUI: i32 = 4;
pub const SSF_SHOWALLOBJECTS: i32 = 1;
pub const SSF_SHOWATTRIBCOL: i32 = 256;
pub const SSF_SHOWCOMPCOLOR: i32 = 8;
pub const SSF_SHOWEXTENSIONS: i32 = 2;
pub const SSF_SHOWINFOTIP: i32 = 8192;
pub const SSF_SHOWSTARTPAGE: i32 = 4194304;
pub const SSF_SHOWSTATUSBAR: i32 = 67108864;
pub const SSF_SHOWSUPERHIDDEN: i32 = 262144;
pub const SSF_SHOWSYSFILES: i32 = 32;
pub const SSF_SHOWTYPEOVERLAY: i32 = 33554432;
pub const SSF_SORTCOLUMNS: i32 = 16;
pub const SSF_STARTPANELON: i32 = 2097152;
pub const SSF_WEBVIEW: i32 = 131072;
pub const SSF_WIN95CLASSIC: i32 = 1024;
pub const STR_PARSE_PARTIAL_IDLIST: windows_sys::core::PCWSTR = windows_sys::core::w!("ParseOriginalItem");
pub const STR_PARSE_WITH_PROPERTIES: windows_sys::core::PCWSTR = windows_sys::core::w!("ParseWithProperties");
pub const VALIDATEUNC_CONNECT: i32 = 1;
pub const VALIDATEUNC_NOUI: i32 = 2;
pub const VALIDATEUNC_PERSIST: i32 = 8;
pub const VALIDATEUNC_PRINT: i32 = 4;
pub const VALIDATEUNC_VALID: i32 = 15;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WALLPAPEROPT {
    pub dwSize: super::DWORD,
    pub dwStyle: super::DWORD,
}
pub const WPSTYLE_CENTER: i32 = 0;
pub const WPSTYLE_CROPTOFIT: i32 = 4;
pub const WPSTYLE_KEEPASPECT: i32 = 3;
pub const WPSTYLE_MAX: i32 = 6;
pub const WPSTYLE_SPAN: i32 = 5;
pub const WPSTYLE_STRETCH: i32 = 2;
pub const WPSTYLE_TILE: i32 = 1;
pub type tagDTI_ADTIWUI = i32;
pub type tagOPEN_AS_INFO_FLAGS = i32;
