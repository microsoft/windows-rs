#[cfg(all(feature = "oaidl", feature = "shobjidl_core", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn AssocGetDetailsOfPropKey(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, pkey : *const super::wtypes::PROPERTYKEY, pv : *mut super::oaidl::VARIANT, pffoundpropkey : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn CDefFolderMenu_Create2(pidlfolder : *const super::shtypes::ITEMIDLIST, hwnd : super::windef::HWND, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, psf : *mut core::ffi::c_void, pfn : LPFNDFMCALLBACK, nkeys : u32, ahkeys : *const super::minwindef::HKEY, ppcm : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn CIDLData_CreateFromIDArray(pidlfolder : *const super::shtypes::ITEMIDLIST, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, ppdtobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn DAD_AutoScroll(hwnd : super::windef::HWND, pad : *mut AUTO_SCROLL_DATA, pptnow : *const super::windef::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn DAD_DragEnterEx(hwndtarget : super::windef::HWND, ptstart : super::windef::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidl", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn DAD_DragEnterEx2(hwndtarget : super::windef::HWND, ptstart : super::windef::POINT, pdtobject : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn DAD_DragLeave() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn DAD_DragMove(pt : super::windef::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commctrl", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn DAD_SetDragImage(him : *mut super::commctrl::_IMAGELIST, pptoffset : *mut super::windef::POINT) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn DAD_ShowDragImage(fshow : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn DriveType(idrive : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn GetFileNameFromBrowse(hwnd : super::windef::HWND, pszfilepath : windows_sys::core::PWSTR, cchfilepath : u32, pszworkingdir : windows_sys::core::PCWSTR, pszdefext : windows_sys::core::PCWSTR, pszfilters : windows_sys::core::PCWSTR, psztitle : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILAppendID(pidl : *const super::shtypes::ITEMIDLIST, pmkid : *const super::shtypes::SHITEMID, fappend : windows_sys::core::BOOL) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILClone(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILCloneFirst(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILCombine(pidl1 : *const super::shtypes::ITEMIDLIST, pidl2 : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILCreateFromPathA(pszpath : windows_sys::core::PCSTR) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILCreateFromPathW(pszpath : windows_sys::core::PCWSTR) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILFindChild(pidlparent : *const super::shtypes::ITEMIDLIST, pidlchild : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILFindLastID(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILFree(pidl : *const super::shtypes::ITEMIDLIST));
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILGetNext(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILGetSize(pidl : *const super::shtypes::ITEMIDLIST) -> u32);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILIsEqual(pidl1 : *const super::shtypes::ITEMIDLIST, pidl2 : *const super::shtypes::ITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILIsParent(pidl1 : *const super::shtypes::ITEMIDLIST, pidl2 : *const super::shtypes::ITEMIDLIST, fimmediate : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidlbase", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILLoadFromStreamEx(pstm : *mut core::ffi::c_void, pidl : *mut super::shtypes::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn ILRemoveLastID(pidl : *mut super::shtypes::ITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidlbase", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn ILSaveToStream(pstm : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn IsNetDrive(idrive : i32) -> i32);
windows_link::link!("shell32.dll" "system" fn IsUserAnAdmin() -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
windows_link::link!("shell32.dll" "system" fn OpenRegStream(hkey : super::minwindef::HKEY, pszsubkey : windows_sys::core::PCWSTR, pszvalue : windows_sys::core::PCWSTR, grfmode : u32) -> *mut core::ffi::c_void);
windows_link::link!("shell32.dll" "system" fn PathCleanupSpec(pszdir : windows_sys::core::PCWSTR, pszspec : windows_sys::core::PWSTR) -> i32);
windows_link::link!("shell32.dll" "system" fn PathGetShortPath(pszlongpath : windows_sys::core::PWSTR));
windows_link::link!("shell32.dll" "system" fn PathIsExe(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn PathMakeUniqueName(pszuniquename : windows_sys::core::PWSTR, cchmax : u32, psztemplate : windows_sys::core::PCWSTR, pszlongplate : windows_sys::core::PCWSTR, pszdir : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn PathResolve(pszpath : windows_sys::core::PWSTR, dirs : *const windows_sys::core::PCWSTR, fflags : u32) -> i32);
windows_link::link!("shell32.dll" "system" fn PathYetAnotherMakeUniqueName(pszuniquename : windows_sys::core::PWSTR, pszpath : windows_sys::core::PCWSTR, pszshort : windows_sys::core::PCWSTR, pszfilespec : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn PickIconDlg(hwnd : super::windef::HWND, psziconpath : windows_sys::core::PWSTR, cchiconpath : u32, piiconindex : *mut i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn PifMgr_CloseProperties(hprops : super::winnt::HANDLE, flopt : u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn PifMgr_GetProperties(hprops : super::winnt::HANDLE, pszgroup : windows_sys::core::PCSTR, lpprops : *mut core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn PifMgr_OpenProperties(pszapp : windows_sys::core::PCWSTR, pszpif : windows_sys::core::PCWSTR, hinf : u32, flopt : u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn PifMgr_SetProperties(hprops : super::winnt::HANDLE, pszgroup : windows_sys::core::PCSTR, lpprops : *const core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
windows_link::link!("shell32.dll" "system" fn ReadCabinetState(pcs : *mut CABINETSTATE, clength : i32) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn RealDriveType(idrive : i32, foktohitnet : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn RestartDialog(hwnd : super::windef::HWND, pszprompt : windows_sys::core::PCWSTR, dwreturn : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn RestartDialogEx(hwnd : super::windef::HWND, pszprompt : windows_sys::core::PCWSTR, dwreturn : u32, dwreasoncode : u32) -> i32);
#[cfg(all(feature = "minwindef", feature = "prsht"))]
windows_link::link!("shell32.dll" "system" fn SHAddFromPropSheetExtArray(hpsxa : HPSXA, lpfnaddpage : super::prsht::LPFNADDPROPSHEETPAGE, lparam : super::minwindef::LPARAM) -> u32);
windows_link::link!("shell32.dll" "system" fn SHAddToRecentDocs(uflags : u32, pv : *const core::ffi::c_void));
windows_link::link!("shell32.dll" "system" fn SHAlloc(cb : usize) -> *mut core::ffi::c_void);
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHBindToFolderIDListParent(psfroot : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::shtypes::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHBindToFolderIDListParentEx(psfroot : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, ppbc : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::shtypes::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHBindToObject(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, pbc : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHBindToParent(pidl : *const super::shtypes::ITEMIDLIST, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::shtypes::LPCITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHBrowseForFolderA(lpbi : *const BROWSEINFOA) -> super::shtypes::LPITEMIDLIST);
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHBrowseForFolderW(lpbi : *const BROWSEINFOW) -> super::shtypes::LPITEMIDLIST);
windows_link::link!("shell32.dll" "system" fn SHCLSIDFromString(psz : windows_sys::core::PCWSTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHChangeNotification_Lock(hchange : super::winnt::HANDLE, dwprocid : u32, pppidl : *mut *mut super::shtypes::LPITEMIDLIST, plevent : *mut i32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn SHChangeNotification_Unlock(hlock : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn SHChangeNotify(weventid : i32, uflags : u32, dwitem1 : *const core::ffi::c_void, dwitem2 : *const core::ffi::c_void));
windows_link::link!("shell32.dll" "system" fn SHChangeNotifyDeregister(ulid : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHChangeNotifyRegister(hwnd : super::windef::HWND, fsources : i32, fevents : i32, wmsg : u32, centries : i32, pshcne : *const SHChangeNotifyEntry) -> u32);
#[cfg(all(feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCloneSpecialIDList(hwnd : super::windef::HWND, csidl : i32, fcreate : windows_sys::core::BOOL) -> super::shtypes::LPITEMIDLIST);
windows_link::link!("shell32.dll" "system" fn SHCoCreateInstance(pszclsid : windows_sys::core::PCWSTR, pclsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDataObject(pidlfolder : *const super::shtypes::ITEMIDLIST, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, pdtinner : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDefaultContextMenu(pdcm : *const DEFCONTEXTMENU, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHCreateDirectory(hwnd : super::windef::HWND, pszpath : windows_sys::core::PCWSTR) -> i32);
#[cfg(all(feature = "minwinbase", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDirectoryExA(hwnd : super::windef::HWND, pszpath : windows_sys::core::PCSTR, psa : *const super::minwinbase::SECURITY_ATTRIBUTES) -> i32);
#[cfg(all(feature = "minwinbase", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateDirectoryExW(hwnd : super::windef::HWND, pszpath : windows_sys::core::PCWSTR, psa : *const super::minwinbase::SECURITY_ATTRIBUTES) -> i32);
windows_link::link!("shell32.dll" "system" fn SHCreateFileExtractIconW(pszfile : windows_sys::core::PCWSTR, dwfileattributes : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
windows_link::link!("shell32.dll" "system" fn SHCreateShellFolderView(pcsfv : *const SFV_CREATE, ppsv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHCreateShellFolderViewEx(pcsfv : *const CSFV, ppsv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHCreateShellItem(pidlparent : *const super::shtypes::ITEMIDLIST, psfparent : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, ppsi : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypes"))]
windows_link::link!("shell32.dll" "system" fn SHCreateStdEnumFmtEtc(cfmt : u32, afmt : *const super::objidl::FORMATETC, ppenumformatetc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHDefExtractIconA(psziconfile : windows_sys::core::PCSTR, iindex : i32, uflags : u32, phiconlarge : *mut super::windef::HICON, phiconsmall : *mut super::windef::HICON, niconsize : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHDefExtractIconW(psziconfile : windows_sys::core::PCWSTR, iindex : i32, uflags : u32, phiconlarge : *mut super::windef::HICON, phiconsmall : *mut super::windef::HICON, niconsize : u32) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHDestroyPropSheetExtArray(hpsxa : HPSXA));
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHDoDragDrop(hwnd : super::windef::HWND, pdata : *mut core::ffi::c_void, pdsrc : *mut core::ffi::c_void, dweffect : u32, pdweffect : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHFindFiles(pidlfolder : *const super::shtypes::ITEMIDLIST, pidlsavefile : *const super::shtypes::ITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHFind_InitMenuPopup(hmenu : super::windef::HMENU, hwndowner : super::windef::HWND, idcmdfirst : u32, idcmdlast : u32) -> *mut core::ffi::c_void);
windows_link::link!("shell32.dll" "system" fn SHFlushSFCache());
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHFormatDrive(hwnd : super::windef::HWND, drive : u32, fmtid : u32, options : u32) -> u32);
windows_link::link!("shell32.dll" "system" fn SHFree(pv : *const core::ffi::c_void));
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHGetAttributesFromDataObject(pdo : *mut core::ffi::c_void, dwattributemask : u32, pdwattributes : *mut u32, pcitems : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetDataFromIDListA(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, nformat : i32, pv : *mut core::ffi::c_void, cb : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetDataFromIDListW(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, nformat : i32, pv : *mut core::ffi::c_void, cb : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "shobjidl_core")]
windows_link::link!("shell32.dll" "system" fn SHGetDesktopFolder(ppshf : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shtypes", feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderLocation(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, ppidl : *mut super::shtypes::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathA(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathAndSubDirA(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszsubdir : windows_sys::core::PCSTR, pszpath : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathAndSubDirW(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszsubdir : windows_sys::core::PCWSTR, pszpath : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetFolderPathW(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHGetIconOverlayIndexA(psziconpath : windows_sys::core::PCSTR, iiconindex : i32) -> i32);
windows_link::link!("shell32.dll" "system" fn SHGetIconOverlayIndexW(psziconpath : windows_sys::core::PCWSTR, iiconindex : i32) -> i32);
windows_link::link!("shell32.dll" "system" fn SHGetInstanceExplorer(ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetKnownFolderIDList(rfid : *const super::shtypes::KNOWNFOLDERID, dwflags : u32, htoken : super::winnt::HANDLE, ppidl : *mut super::shtypes::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetKnownFolderItem(rfid : *const super::shtypes::KNOWNFOLDERID, flags : KNOWN_FOLDER_FLAG, htoken : super::winnt::HANDLE, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHGetKnownFolderPath(rfid : *const super::shtypes::KNOWNFOLDERID, dwflags : u32, htoken : super::winnt::HANDLE, ppszpath : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("shell32.dll" "system" fn SHGetMalloc(ppmalloc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHGetPathFromIDListA(pidl : *const super::shtypes::ITEMIDLIST, pszpath : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHGetPathFromIDListEx(pidl : *const super::shtypes::ITEMIDLIST, pszpath : windows_sys::core::PWSTR, cchpath : u32, uopts : GPFIDL_FLAGS) -> windows_sys::core::BOOL);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHGetPathFromIDListW(pidl : *const super::shtypes::ITEMIDLIST, pszpath : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetRealIDL(psf : *mut core::ffi::c_void, pidlsimple : *const super::shtypes::ITEMIDLIST, ppidlreal : *mut super::shtypes::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(feature = "shobjidl_core")]
windows_link::link!("shell32.dll" "system" fn SHGetSetFolderCustomSettings(pfcs : *mut SHFOLDERCUSTOMSETTINGS, pszpath : windows_sys::core::PCWSTR, dwreadwrite : u32) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHGetSetSettings(lpss : *mut SHELLSTATEA, dwmask : u32, bset : windows_sys::core::BOOL));
windows_link::link!("shell32.dll" "system" fn SHGetSettings(psfs : *mut SHELLFLAGSTATE, dwmask : u32));
#[cfg(all(feature = "shtypes", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHGetSpecialFolderLocation(hwnd : super::windef::HWND, csidl : i32, ppidl : *mut super::shtypes::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHGetSpecialFolderPathA(hwnd : super::windef::HWND, pszpath : windows_sys::core::PSTR, csidl : i32, fcreate : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHGetSpecialFolderPathW(hwnd : super::windef::HWND, pszpath : windows_sys::core::PWSTR, csidl : i32, fcreate : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHHandleUpdateImage(pidlextra : *const super::shtypes::ITEMIDLIST) -> i32);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHILCreateFromPath(pszpath : windows_sys::core::PCWSTR, ppidl : *mut super::shtypes::LPITEMIDLIST, rgfinout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHLimitInputEdit(hwndedit : super::windef::HWND, psf : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHLoadInProc(rclsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHMapPIDLToSystemImageListIndex(pshf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, piindexsel : *mut i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHObjectProperties(hwnd : super::windef::HWND, shopobjecttype : u32, pszobjectname : windows_sys::core::PCWSTR, pszpropertypage : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHOpenFolderAndSelectItems(pidlfolder : *const super::shtypes::ITEMIDLIST, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHOpenWithDialog(hwndparent : super::windef::HWND, poainfo : *const OPENASINFO) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHParseDisplayName(pszname : windows_sys::core::PCWSTR, pbc : *mut core::ffi::c_void, ppidl : *mut super::shtypes::LPITEMIDLIST, sfgaoin : super::shobjidl_core::SFGAOF, psfgaoout : *mut super::shobjidl_core::SFGAOF) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHPathPrepareForWriteA(hwnd : super::windef::HWND, punkenablemodless : *mut core::ffi::c_void, pszpath : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHPathPrepareForWriteW(hwnd : super::windef::HWND, punkenablemodless : *mut core::ffi::c_void, pszpath : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "propidlbase")]
windows_link::link!("shell32.dll" "system" fn SHPropStgCreate(psstg : *mut core::ffi::c_void, fmtid : *const windows_sys::core::GUID, pclsid : *const windows_sys::core::GUID, grfflags : u32, grfmode : u32, dwdisposition : u32, ppstg : *mut *mut core::ffi::c_void, pucodepage : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn SHPropStgReadMultiple(pps : *mut core::ffi::c_void, ucodepage : u32, cpspec : u32, rgpspec : *const super::propidlbase::PROPSPEC, rgvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn SHPropStgWriteMultiple(pps : *mut core::ffi::c_void, pucodepage : *mut u32, cpspec : u32, rgpspec : *const super::propidlbase::PROPSPEC, rgvar : *mut super::propidlbase::PROPVARIANT, propidnamefirst : super::wtypes::PROPID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "prsht"))]
windows_link::link!("shell32.dll" "system" fn SHReplaceFromPropSheetExtArray(hpsxa : HPSXA, upageid : u32, lpfnreplacewith : super::prsht::LPFNADDPROPSHEETPAGE, lparam : super::minwindef::LPARAM) -> u32);
windows_link::link!("shell32.dll" "system" fn SHRestricted(rest : RESTRICTIONS) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn SHSetFolderPathA(csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("shell32.dll" "system" fn SHSetFolderPathW(csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHSetInstanceExplorer(punk : *mut core::ffi::c_void));
#[cfg(all(feature = "shtypes", feature = "winnt"))]
windows_link::link!("shell32.dll" "system" fn SHSetKnownFolderPath(rfid : *const super::shtypes::KNOWNFOLDERID, dwflags : u32, htoken : super::winnt::HANDLE, pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("shell32.dll" "system" fn SHShellFolderView_Message(hwndmain : super::windef::HWND, umsg : u32, lparam : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHStartNetConnectionDialogW(hwnd : super::windef::HWND, pszremotename : windows_sys::core::PCWSTR, dwtype : u32) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHUpdateImageA(pszhashitem : windows_sys::core::PCSTR, iindex : i32, uflags : u32, iimageindex : i32));
windows_link::link!("shell32.dll" "system" fn SHUpdateImageW(pszhashitem : windows_sys::core::PCWSTR, iindex : i32, uflags : u32, iimageindex : i32));
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHValidateUNC(hwndowner : super::windef::HWND, pszfile : windows_sys::core::PWSTR, fconnect : u32) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn Shell_GetCachedImageIndex(pwsziconpath : windows_sys::core::PCWSTR, iiconindex : i32, uiconflags : u32) -> i32);
windows_link::link!("shell32.dll" "system" fn Shell_GetCachedImageIndexA(psziconpath : windows_sys::core::PCSTR, iiconindex : i32, uiconflags : u32) -> i32);
windows_link::link!("shell32.dll" "system" fn Shell_GetCachedImageIndexW(psziconpath : windows_sys::core::PCWSTR, iiconindex : i32, uiconflags : u32) -> i32);
#[cfg(feature = "commctrl")]
windows_link::link!("shell32.dll" "system" fn Shell_GetImageLists(phiml : *mut super::commctrl::HIMAGELIST, phimlsmall : *mut super::commctrl::HIMAGELIST) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn Shell_MergeMenus(hmdst : super::windef::HMENU, hmsrc : super::windef::HMENU, uinsert : u32, uidadjust : u32, uidadjustmax : u32, uflags : u32) -> u32);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SignalFileOpen(pidl : *const super::shtypes::ITEMIDLIST) -> windows_sys::core::BOOL);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn StgMakeUniqueName(pstgparent : *mut core::ffi::c_void, pszfilespec : windows_sys::core::PCWSTR, grfmode : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn Win32DeleteFile(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn WriteCabinetState(pcs : *const CABINETSTATE) -> windows_sys::core::BOOL);
pub const ACLO_CURRENTDIR: AUTOCOMPLETELISTOPTIONS = 1;
pub const ACLO_DESKTOP: AUTOCOMPLETELISTOPTIONS = 4;
pub const ACLO_FAVORITES: AUTOCOMPLETELISTOPTIONS = 8;
pub const ACLO_FILESYSDIRS: AUTOCOMPLETELISTOPTIONS = 32;
pub const ACLO_FILESYSONLY: AUTOCOMPLETELISTOPTIONS = 16;
pub const ACLO_MYCOMPUTER: AUTOCOMPLETELISTOPTIONS = 2;
pub const ACLO_NONE: AUTOCOMPLETELISTOPTIONS = 0;
pub const ACLO_VIRTUALNAMESPACE: AUTOCOMPLETELISTOPTIONS = 64;
pub const ADDURL_SILENT: u32 = 1;
pub const AD_APPLY_ALL: u32 = 7;
pub const AD_APPLY_BUFFERED_REFRESH: u32 = 16;
pub const AD_APPLY_DYNAMICREFRESH: u32 = 32;
pub const AD_APPLY_FORCE: u32 = 8;
pub const AD_APPLY_HTMLGEN: u32 = 2;
pub const AD_APPLY_REFRESH: u32 = 4;
pub const AD_APPLY_SAVE: u32 = 1;
pub const AD_GETWP_BMP: u32 = 0;
pub const AD_GETWP_IMAGE: u32 = 1;
pub const AD_GETWP_LAST_APPLIED: u32 = 2;
pub type AUTOCOMPLETELISTOPTIONS = i32;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct AUTO_SCROLL_DATA {
    pub iNextSample: i32,
    pub dwLastScroll: u32,
    pub bFull: windows_sys::core::BOOL,
    pub pts: [super::windef::POINT; 3],
    pub dwTimes: [u32; 3],
}
#[cfg(feature = "windef")]
impl Default for AUTO_SCROLL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type BFFCALLBACK = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, umsg: u32, lparam: super::minwindef::LPARAM, lpdata: super::minwindef::LPARAM) -> i32>;
pub const BFFM_ENABLEOK: u32 = 1125;
pub const BFFM_INITIALIZED: u32 = 1;
pub const BFFM_IUNKNOWN: u32 = 5;
pub const BFFM_SELCHANGED: u32 = 2;
pub const BFFM_SETEXPANDED: u32 = 1130;
pub const BFFM_SETOKTEXT: u32 = 1129;
pub const BFFM_SETSELECTION: u32 = 1126;
pub const BFFM_SETSELECTIONA: u32 = 1126;
pub const BFFM_SETSELECTIONW: u32 = 1127;
pub const BFFM_SETSTATUSTEXT: u32 = 1124;
pub const BFFM_SETSTATUSTEXTA: u32 = 1124;
pub const BFFM_SETSTATUSTEXTW: u32 = 1128;
pub const BFFM_VALIDATEFAILED: u32 = 3;
pub const BFFM_VALIDATEFAILEDA: u32 = 3;
pub const BFFM_VALIDATEFAILEDW: u32 = 4;
pub const BIF_BROWSEFILEJUNCTIONS: u32 = 65536;
pub const BIF_BROWSEFORCOMPUTER: u32 = 4096;
pub const BIF_BROWSEFORPRINTER: u32 = 8192;
pub const BIF_BROWSEINCLUDEFILES: u32 = 16384;
pub const BIF_BROWSEINCLUDEURLS: u32 = 128;
pub const BIF_DONTGOBELOWDOMAIN: u32 = 2;
pub const BIF_EDITBOX: u32 = 16;
pub const BIF_NEWDIALOGSTYLE: u32 = 64;
pub const BIF_NONEWFOLDERBUTTON: u32 = 512;
pub const BIF_NOTRANSLATETARGETS: u32 = 1024;
pub const BIF_RETURNFSANCESTORS: u32 = 8;
pub const BIF_RETURNONLYFSDIRS: u32 = 1;
pub const BIF_SHAREABLE: u32 = 32768;
pub const BIF_STATUSTEXT: u32 = 4;
pub const BIF_UAHINT: u32 = 256;
pub const BIF_USENEWUI: u32 = 80;
pub const BIF_VALIDATE: u32 = 32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct BROWSEINFOA {
    pub hwndOwner: super::windef::HWND,
    pub pidlRoot: super::shtypes::LPCITEMIDLIST,
    pub pszDisplayName: windows_sys::core::PSTR,
    pub lpszTitle: windows_sys::core::PCSTR,
    pub ulFlags: u32,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::minwindef::LPARAM,
    pub iImage: i32,
}
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
impl Default for BROWSEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct BROWSEINFOW {
    pub hwndOwner: super::windef::HWND,
    pub pidlRoot: super::shtypes::LPCITEMIDLIST,
    pub pszDisplayName: windows_sys::core::PWSTR,
    pub lpszTitle: windows_sys::core::PCWSTR,
    pub ulFlags: u32,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::minwindef::LPARAM,
    pub iImage: i32,
}
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
impl Default for BROWSEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CABINETSTATE {
    pub cLength: u16,
    pub nVersion: u16,
    pub _bitfield: windows_sys::core::BOOL,
    pub fMenuEnumFilter: u32,
}
pub const CABINETSTATE_VERSION: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CIDA {
    pub cidl: u32,
    pub aoffset: [u32; 1],
}
impl Default for CIDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMDID_INTSHORTCUTCREATE: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct COMPONENT {
    pub dwSize: u32,
    pub dwID: u32,
    pub iComponentType: i32,
    pub fChecked: windows_sys::core::BOOL,
    pub fDirty: windows_sys::core::BOOL,
    pub fNoScroll: windows_sys::core::BOOL,
    pub cpPos: COMPPOS,
    pub wszFriendlyName: [u16; 260],
    pub wszSource: [u16; 2084],
    pub wszSubscribedURL: [u16; 2084],
    pub dwCurItemState: u32,
    pub csiOriginal: COMPSTATEINFO,
    pub csiRestored: COMPSTATEINFO,
}
impl Default for COMPONENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct COMPONENTSOPT {
    pub dwSize: u32,
    pub fEnableComponents: windows_sys::core::BOOL,
    pub fActiveDesktop: windows_sys::core::BOOL,
}
pub const COMPONENT_DEFAULT_LEFT: u32 = 65535;
pub const COMPONENT_DEFAULT_TOP: u32 = 65535;
pub const COMPONENT_TOP: u32 = 1073741823;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct COMPPOS {
    pub dwSize: u32,
    pub iLeft: i32,
    pub iTop: i32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub izIndex: i32,
    pub fCanResize: windows_sys::core::BOOL,
    pub fCanResizeX: windows_sys::core::BOOL,
    pub fCanResizeY: windows_sys::core::BOOL,
    pub iPreferredLeftPercent: i32,
    pub iPreferredTopPercent: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct COMPSTATEINFO {
    pub dwSize: u32,
    pub iLeft: i32,
    pub iTop: i32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwItemState: u32,
}
pub const COMP_ELEM_ALL: u32 = 32767;
pub const COMP_ELEM_CHECKED: u32 = 2;
pub const COMP_ELEM_CURITEMSTATE: u32 = 16384;
pub const COMP_ELEM_DIRTY: u32 = 4;
pub const COMP_ELEM_FRIENDLYNAME: u32 = 1024;
pub const COMP_ELEM_NOSCROLL: u32 = 8;
pub const COMP_ELEM_ORIGINAL_CSI: u32 = 4096;
pub const COMP_ELEM_POS_LEFT: u32 = 16;
pub const COMP_ELEM_POS_TOP: u32 = 32;
pub const COMP_ELEM_POS_ZINDEX: u32 = 256;
pub const COMP_ELEM_RESTORED_CSI: u32 = 8192;
pub const COMP_ELEM_SIZE_HEIGHT: u32 = 128;
pub const COMP_ELEM_SIZE_WIDTH: u32 = 64;
pub const COMP_ELEM_SOURCE: u32 = 512;
pub const COMP_ELEM_SUBSCRIBEDURL: u32 = 2048;
pub const COMP_ELEM_TYPE: u32 = 1;
pub const COMP_TYPE_CFHTML: u32 = 4;
pub const COMP_TYPE_CONTROL: u32 = 3;
pub const COMP_TYPE_HTMLDOC: u32 = 0;
pub const COMP_TYPE_MAX: u32 = 4;
pub const COMP_TYPE_PICTURE: u32 = 1;
pub const COMP_TYPE_WEBSITE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CSFV {
    pub cbSize: u32,
    pub pshf: *mut core::ffi::c_void,
    pub psvOuter: *mut core::ffi::c_void,
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub lEvents: i32,
    pub pfnCallback: LPFNVIEWCALLBACK,
    pub fvm: super::shobjidl_core::FOLDERVIEWMODE,
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl Default for CSFV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSIDL_ADMINTOOLS: u32 = 48;
pub const CSIDL_ALTSTARTUP: u32 = 29;
pub const CSIDL_APPDATA: u32 = 26;
pub const CSIDL_BITBUCKET: u32 = 10;
pub const CSIDL_CDBURN_AREA: u32 = 59;
pub const CSIDL_COMMON_ADMINTOOLS: u32 = 47;
pub const CSIDL_COMMON_ALTSTARTUP: u32 = 30;
pub const CSIDL_COMMON_APPDATA: u32 = 35;
pub const CSIDL_COMMON_DESKTOPDIRECTORY: u32 = 25;
pub const CSIDL_COMMON_DOCUMENTS: u32 = 46;
pub const CSIDL_COMMON_FAVORITES: u32 = 31;
pub const CSIDL_COMMON_MUSIC: u32 = 53;
pub const CSIDL_COMMON_OEM_LINKS: u32 = 58;
pub const CSIDL_COMMON_PICTURES: u32 = 54;
pub const CSIDL_COMMON_PROGRAMS: u32 = 23;
pub const CSIDL_COMMON_STARTMENU: u32 = 22;
pub const CSIDL_COMMON_STARTUP: u32 = 24;
pub const CSIDL_COMMON_TEMPLATES: u32 = 45;
pub const CSIDL_COMMON_VIDEO: u32 = 55;
pub const CSIDL_COMPUTERSNEARME: u32 = 61;
pub const CSIDL_CONNECTIONS: u32 = 49;
pub const CSIDL_CONTROLS: u32 = 3;
pub const CSIDL_COOKIES: u32 = 33;
pub const CSIDL_DESKTOP: u32 = 0;
pub const CSIDL_DESKTOPDIRECTORY: u32 = 16;
pub const CSIDL_DRIVES: u32 = 17;
pub const CSIDL_FAVORITES: u32 = 6;
pub const CSIDL_FLAG_CREATE: u32 = 32768;
pub const CSIDL_FLAG_DONT_UNEXPAND: u32 = 8192;
pub const CSIDL_FLAG_DONT_VERIFY: u32 = 16384;
pub const CSIDL_FLAG_MASK: u32 = 65280;
pub const CSIDL_FLAG_NO_ALIAS: u32 = 4096;
pub const CSIDL_FLAG_PER_USER_INIT: u32 = 2048;
pub const CSIDL_FONTS: u32 = 20;
pub const CSIDL_HISTORY: u32 = 34;
pub const CSIDL_INTERNET: u32 = 1;
pub const CSIDL_INTERNET_CACHE: u32 = 32;
pub const CSIDL_LOCAL_APPDATA: u32 = 28;
pub const CSIDL_MYDOCUMENTS: u32 = 5;
pub const CSIDL_MYMUSIC: u32 = 13;
pub const CSIDL_MYPICTURES: u32 = 39;
pub const CSIDL_MYVIDEO: u32 = 14;
pub const CSIDL_NETHOOD: u32 = 19;
pub const CSIDL_NETWORK: u32 = 18;
pub const CSIDL_PERSONAL: u32 = 5;
pub const CSIDL_PRINTERS: u32 = 4;
pub const CSIDL_PRINTHOOD: u32 = 27;
pub const CSIDL_PROFILE: u32 = 40;
pub const CSIDL_PROGRAMS: u32 = 2;
pub const CSIDL_PROGRAM_FILES: u32 = 38;
pub const CSIDL_PROGRAM_FILESX86: u32 = 42;
pub const CSIDL_PROGRAM_FILES_COMMON: u32 = 43;
pub const CSIDL_PROGRAM_FILES_COMMONX86: u32 = 44;
pub const CSIDL_RECENT: u32 = 8;
pub const CSIDL_RESOURCES: u32 = 56;
pub const CSIDL_RESOURCES_LOCALIZED: u32 = 57;
pub const CSIDL_SENDTO: u32 = 9;
pub const CSIDL_STARTMENU: u32 = 11;
pub const CSIDL_STARTUP: u32 = 7;
pub const CSIDL_SYSTEM: u32 = 37;
pub const CSIDL_SYSTEMX86: u32 = 41;
pub const CSIDL_TEMPLATES: u32 = 21;
pub const CSIDL_WINDOWS: u32 = 36;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DATABLOCK_HEADER {
    pub cbSize: u32,
    pub dwSignature: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DEFCONTEXTMENU {
    pub hwnd: super::windef::HWND,
    pub pcmcb: *mut core::ffi::c_void,
    pub pidlFolder: super::shtypes::LPCITEMIDLIST,
    pub psf: *mut core::ffi::c_void,
    pub cidl: u32,
    pub apidl: *mut super::shtypes::LPCITEMIDLIST,
    pub punkAssociationInfo: *mut core::ffi::c_void,
    pub cKeys: u32,
    pub aKeys: *const super::minwindef::HKEY,
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl Default for DEFCONTEXTMENU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy)]
pub struct DETAILSINFO {
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub fmt: i32,
    pub cxChar: i32,
    pub str: super::shtypes::STRRET,
    pub iImage: i32,
}
#[cfg(feature = "shtypes")]
impl Default for DETAILSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DFMICS {
    pub cbSize: u32,
    pub fMask: u32,
    pub lParam: super::minwindef::LPARAM,
    pub idCmdFirst: u32,
    pub idDefMax: u32,
    pub pici: super::shobjidl_core::LPCMINVOKECOMMANDINFO,
    pub punkSite: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
impl Default for DFMICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DFM_CMD_COPY: u32 = 4294967293;
pub const DFM_CMD_DELETE: u32 = 4294967295;
pub const DFM_CMD_LINK: u32 = 4294967292;
pub const DFM_CMD_MODALPROP: u32 = 4294967284;
pub const DFM_CMD_MOVE: u32 = 4294967294;
pub const DFM_CMD_NEWFOLDER: u32 = 4294967290;
pub const DFM_CMD_PASTE: u32 = 4294967289;
pub const DFM_CMD_PASTELINK: u32 = 4294967286;
pub const DFM_CMD_PASTESPECIAL: u32 = 4294967285;
pub const DFM_CMD_PROPERTIES: u32 = 4294967291;
pub const DFM_CMD_RENAME: u32 = 4294967283;
pub const DFM_CMD_VIEWDETAILS: u32 = 4294967287;
pub const DFM_CMD_VIEWLIST: u32 = 4294967288;
pub const DFM_GETDEFSTATICID: u32 = 14;
pub const DFM_GETHELPTEXT: u32 = 5;
pub const DFM_GETHELPTEXTW: u32 = 11;
pub const DFM_GETVERBA: u32 = 16;
pub const DFM_GETVERBW: u32 = 15;
pub const DFM_INVOKECOMMAND: u32 = 2;
pub const DFM_INVOKECOMMANDEX: u32 = 12;
pub const DFM_MAPCOMMANDNAME: u32 = 13;
pub const DFM_MERGECONTEXTMENU: u32 = 1;
pub const DFM_MERGECONTEXTMENU_BOTTOM: u32 = 17;
pub const DFM_MERGECONTEXTMENU_TOP: u32 = 10;
pub const DFM_MODIFYQCMFLAGS: u32 = 18;
pub const DFM_VALIDATECMD: u32 = 9;
pub const DFM_WM_DRAWITEM: u32 = 7;
pub const DFM_WM_INITMENUPOPUP: u32 = 8;
pub const DFM_WM_MEASUREITEM: u32 = 6;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DROPDESCRIPTION {
    pub r#type: DROPIMAGETYPE,
    pub szMessage: [u16; 260],
    pub szInsert: [u16; 260],
}
impl Default for DROPDESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DROPFILES {
    pub pFiles: u32,
    pub pt: super::windef::POINT,
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
pub const DVASPECT_COPY: u32 = 3;
pub const DVASPECT_LINK: u32 = 4;
pub const DVASPECT_SHORTNAME: u32 = 2;
pub const EXP_DARWIN_ID_SIG: u32 = 2684354566;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EXP_DARWIN_LINK {
    pub dbh: DATABLOCK_HEADER,
    pub szDarwinID: [i8; 260],
    pub szwDarwinID: [u16; 260],
}
impl Default for EXP_DARWIN_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EXP_PROPERTYSTORAGE {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub abPropertyStorage: [u8; 1],
}
impl Default for EXP_PROPERTYSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXP_PROPERTYSTORAGE_SIG: u32 = 2684354569;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct EXP_SPECIAL_FOLDER {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub idSpecialFolder: u32,
    pub cbOffset: u32,
}
pub const EXP_SPECIAL_FOLDER_SIG: u32 = 2684354565;
pub const EXP_SZ_ICON_SIG: u32 = 2684354567;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EXP_SZ_LINK {
    pub cbSize: u32,
    pub dwSignature: u32,
    pub szTarget: [i8; 260],
    pub swzTarget: [u16; 260],
}
impl Default for EXP_SZ_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXP_SZ_LINK_SIG: u32 = 2684354561;
pub const FCIDM_BROWSERFIRST: u32 = 40960;
pub const FCIDM_BROWSERLAST: u32 = 48896;
pub const FCIDM_GLOBALFIRST: u32 = 32768;
pub const FCIDM_GLOBALLAST: u32 = 40959;
pub const FCIDM_MENU_EDIT: u32 = 32832;
pub const FCIDM_MENU_EXPLORE: u32 = 33104;
pub const FCIDM_MENU_FAVORITES: u32 = 33136;
pub const FCIDM_MENU_FILE: u32 = 32768;
pub const FCIDM_MENU_FIND: u32 = 33088;
pub const FCIDM_MENU_HELP: u32 = 33024;
pub const FCIDM_MENU_TOOLS: u32 = 32960;
pub const FCIDM_MENU_TOOLS_SEP_GOTO: u32 = 32961;
pub const FCIDM_MENU_VIEW: u32 = 32896;
pub const FCIDM_MENU_VIEW_SEP_OPTIONS: u32 = 32897;
pub const FCIDM_SHVIEWFIRST: u32 = 0;
pub const FCIDM_SHVIEWLAST: u32 = 32767;
pub const FCSM_CLSID: u32 = 8;
pub const FCSM_FLAGS: u32 = 64;
pub const FCSM_ICONFILE: u32 = 16;
pub const FCSM_INFOTIP: u32 = 4;
pub const FCSM_LOGO: u32 = 32;
pub const FCSM_VIEWID: u32 = 1;
pub const FCSM_WEBVIEWTEMPLATE: u32 = 2;
pub const FCS_FLAG_DRAGDROP: u32 = 2;
pub const FCS_FORCEWRITE: u32 = 2;
pub const FCS_READ: u32 = 1;
pub const FCS_WRITE: u32 = 3;
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
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct FILEDESCRIPTORA {
    pub dwFlags: u32,
    pub clsid: windows_sys::core::GUID,
    pub sizel: super::windef::SIZEL,
    pub pointl: super::windef::POINTL,
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::minwindef::FILETIME,
    pub ftLastAccessTime: super::minwindef::FILETIME,
    pub ftLastWriteTime: super::minwindef::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub cFileName: [i8; 260],
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for FILEDESCRIPTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct FILEDESCRIPTORW {
    pub dwFlags: u32,
    pub clsid: windows_sys::core::GUID,
    pub sizel: super::windef::SIZEL,
    pub pointl: super::windef::POINTL,
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::minwindef::FILETIME,
    pub ftLastAccessTime: super::minwindef::FILETIME,
    pub ftLastWriteTime: super::minwindef::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub cFileName: [u16; 260],
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for FILEDESCRIPTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct FILEGROUPDESCRIPTORA {
    pub cItems: u32,
    pub fgd: [FILEDESCRIPTORA; 1],
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for FILEGROUPDESCRIPTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct FILEGROUPDESCRIPTORW {
    pub cItems: u32,
    pub fgd: [FILEDESCRIPTORW; 1],
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for FILEGROUPDESCRIPTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct FILE_ATTRIBUTES_ARRAY {
    pub cItems: u32,
    pub dwSumFileAttributes: u32,
    pub dwProductFileAttributes: u32,
    pub rgdwFileAttributes: [u32; 1],
}
impl Default for FILE_ATTRIBUTES_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GIL_ASYNC: u32 = 32;
pub const GIL_CHECKSHIELD: u32 = 512;
pub const GIL_DEFAULTICON: u32 = 64;
pub const GIL_DONTCACHE: u32 = 16;
pub const GIL_FORCENOSHIELD: u32 = 1024;
pub const GIL_FORSHELL: u32 = 2;
pub const GIL_FORSHORTCUT: u32 = 128;
pub const GIL_NOTFILENAME: u32 = 8;
pub const GIL_OPENICON: u32 = 1;
pub const GIL_PERCLASS: u32 = 4;
pub const GIL_PERINSTANCE: u32 = 2;
pub const GIL_SHIELD: u32 = 512;
pub const GIL_SIMULATEDOC: u32 = 1;
pub const GPFIDL_ALTNAME: i32 = 1;
pub const GPFIDL_DEFAULT: i32 = 0;
pub type GPFIDL_FLAGS = i32;
pub const GPFIDL_UNCPRINTER: i32 = 2;
pub type HPSXA = *mut core::ffi::c_void;
pub const IDO_SHGIOI_DEFAULT: u32 = 4294967292;
pub const IDO_SHGIOI_LINK: u32 = 268435454;
pub const IDO_SHGIOI_SHARE: u32 = 268435455;
pub const IDO_SHGIOI_SLOWFILE: u32 = 4294967293;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IE4COMPONENT {
    pub dwSize: u32,
    pub dwID: u32,
    pub iComponentType: i32,
    pub fChecked: windows_sys::core::BOOL,
    pub fDirty: windows_sys::core::BOOL,
    pub fNoScroll: windows_sys::core::BOOL,
    pub cpPos: COMPPOS,
    pub wszFriendlyName: [u16; 260],
    pub wszSource: [u16; 2084],
    pub wszSubscribedURL: [u16; 2084],
}
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
pub const IS_FULLSCREEN: u32 = 2;
pub const IS_NORMAL: u32 = 1;
pub const IS_SPLIT: u32 = 4;
pub const IS_VALIDSIZESTATEBITS: u32 = 7;
pub const IS_VALIDSTATEBITS: i32 = -1073741817;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ITEMSPACING {
    pub cxSmall: i32,
    pub cySmall: i32,
    pub cxLarge: i32,
    pub cyLarge: i32,
}
pub const KF_FLAG_ALIAS_ONLY: KNOWN_FOLDER_FLAG = 2147483648;
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
pub type KNOWN_FOLDER_FLAG = u32;
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
pub type LPBROWSEINFOA = *mut BROWSEINFOA;
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
pub type LPBROWSEINFOW = *mut BROWSEINFOW;
pub type LPCABINETSTATE = *mut CABINETSTATE;
pub type LPCCOMPONENT = *const COMPONENT;
pub type LPCCOMPONENTSOPT = *const COMPONENTSOPT;
pub type LPCCOMPPOS = *const COMPPOS;
pub type LPCCOMPSTATEINFO = *const COMPSTATEINFO;
pub type LPCIE4COMPONENT = *const IE4COMPONENT;
pub type LPCOMPONENT = *mut COMPONENT;
pub type LPCOMPONENTSOPT = *mut COMPONENTSOPT;
pub type LPCOMPPOS = *mut COMPPOS;
pub type LPCOMPSTATEINFO = *mut COMPSTATEINFO;
pub type LPCPROPPRG = *const PROPPRG;
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPCSFV = *mut CSFV;
pub type LPCWALLPAPEROPT = *const WALLPAPEROPT;
pub type LPDATABLOCK_HEADER = *mut DATABLOCK_HEADER;
pub type LPDBLIST = *mut DATABLOCK_HEADER;
#[cfg(feature = "windef")]
pub type LPDROPFILES = *mut DROPFILES;
pub type LPEXP_DARWIN_LINK = *mut EXP_DARWIN_LINK;
pub type LPEXP_SPECIAL_FOLDER = *mut EXP_SPECIAL_FOLDER;
pub type LPEXP_SZ_LINK = *mut EXP_SZ_LINK;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFILEDESCRIPTORA = *mut FILEDESCRIPTORA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFILEDESCRIPTORW = *mut FILEDESCRIPTORW;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFILEGROUPDESCRIPTORA = *mut FILEGROUPDESCRIPTORA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFILEGROUPDESCRIPTORW = *mut FILEGROUPDESCRIPTORW;
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "shobjidl_core", feature = "windef"))]
pub type LPFNDFMCALLBACK = Option<unsafe extern "system" fn(psf: *mut core::ffi::c_void, hwnd: super::windef::HWND, pdtobj: *mut core::ffi::c_void, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
pub type LPFNVIEWCALLBACK = Option<unsafe extern "system" fn(psvouter: *mut core::ffi::c_void, psf: *mut core::ffi::c_void, hwndmain: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_sys::core::HRESULT>;
pub type LPIDA = *mut CIDA;
pub type LPIE4COMPONENT = *mut IE4COMPONENT;
#[cfg(feature = "winnetwk")]
pub type LPNRESARRAY = *mut NRESARRAY;
#[cfg(all(feature = "wincontypes", feature = "windef"))]
pub type LPNT_CONSOLE_PROPS = *mut NT_CONSOLE_PROPS;
pub type LPNT_FE_CONSOLE_PROPS = *mut NT_FE_CONSOLE_PROPS;
pub type LPPROPPRG = *mut PROPPRG;
#[cfg(feature = "windef")]
pub type LPQCMINFO = *mut QCMINFO;
pub type LPSHChangeDWORDAsIDList = *mut SHChangeDWORDAsIDList;
pub type LPSHChangeUpdateImageIDList = *mut SHChangeUpdateImageIDList;
pub type LPSHDESCRIPTIONID = *mut SHDESCRIPTIONID;
pub type LPSHELLFLAGSTATE = *mut SHELLFLAGSTATE;
pub type LPSHELLSTATEA = *mut SHELLSTATEA;
pub type LPSHELLSTATEW = *mut SHELLSTATEW;
#[cfg(feature = "shobjidl_core")]
pub type LPSHFOLDERCUSTOMSETTINGS = *mut SHFOLDERCUSTOMSETTINGS;
pub type LPWALLPAPEROPT = *mut WALLPAPEROPT;
pub const MAX_COLUMN_DESC_LEN: u32 = 128;
pub const MAX_COLUMN_NAME_LEN: u32 = 80;
pub const MM_ADDSEPARATOR: u32 = 1;
pub const MM_DONTREMOVESEPS: u32 = 4;
pub const MM_SUBMENUSHAVEIDS: u32 = 2;
#[repr(C)]
#[cfg(feature = "winnetwk")]
#[derive(Clone, Copy)]
pub struct NRESARRAY {
    pub cItems: u32,
    pub nr: [super::winnetwk::NETRESOURCE; 1],
}
#[cfg(feature = "winnetwk")]
impl Default for NRESARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "wincontypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct NT_CONSOLE_PROPS {
    pub dbh: DATABLOCK_HEADER,
    pub wFillAttribute: u16,
    pub wPopupFillAttribute: u16,
    pub dwScreenBufferSize: super::wincontypes::COORD,
    pub dwWindowSize: super::wincontypes::COORD,
    pub dwWindowOrigin: super::wincontypes::COORD,
    pub nFont: u32,
    pub nInputBufferSize: u32,
    pub dwFontSize: super::wincontypes::COORD,
    pub uFontFamily: u32,
    pub uFontWeight: u32,
    pub FaceName: [u16; 32],
    pub uCursorSize: u32,
    pub bFullScreen: windows_sys::core::BOOL,
    pub bQuickEdit: windows_sys::core::BOOL,
    pub bInsertMode: windows_sys::core::BOOL,
    pub bAutoPosition: windows_sys::core::BOOL,
    pub uHistoryBufferSize: u32,
    pub uNumberOfHistoryBuffers: u32,
    pub bHistoryNoDup: windows_sys::core::BOOL,
    pub ColorTable: [super::windef::COLORREF; 16],
}
#[cfg(all(feature = "wincontypes", feature = "windef"))]
impl Default for NT_CONSOLE_PROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NT_CONSOLE_PROPS_SIG: u32 = 2684354562;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NT_FE_CONSOLE_PROPS {
    pub dbh: DATABLOCK_HEADER,
    pub uCodePage: u32,
}
pub const NT_FE_CONSOLE_PROPS_SIG: u32 = 2684354564;
pub const NUM_POINTS: u32 = 3;
pub const OAIF_ALLOW_REGISTRATION: tagOPEN_AS_INFO_FLAGS = 1;
pub const OAIF_EXEC: tagOPEN_AS_INFO_FLAGS = 4;
pub const OAIF_FILE_IS_URI: tagOPEN_AS_INFO_FLAGS = 128;
pub const OAIF_FORCE_REGISTRATION: tagOPEN_AS_INFO_FLAGS = 8;
pub const OAIF_HIDE_REGISTRATION: tagOPEN_AS_INFO_FLAGS = 32;
pub const OAIF_REGISTER_EXT: tagOPEN_AS_INFO_FLAGS = 2;
pub const OAIF_URL_PROTOCOL: tagOPEN_AS_INFO_FLAGS = 64;
pub const OFASI_EDIT: u32 = 1;
pub const OFASI_OPENDESKTOP: u32 = 2;
pub const OI_ASYNC: u32 = 4294962926;
pub const OI_DEFAULT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OPENASINFO {
    pub pcszFile: windows_sys::core::PCWSTR,
    pub pcszClass: windows_sys::core::PCWSTR,
    pub oaifInFlags: OPEN_AS_INFO_FLAGS,
}
impl Default for OPENASINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OPEN_AS_INFO_FLAGS = i32;
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
pub type PBROWSEINFOA = *mut BROWSEINFOA;
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
pub type PBROWSEINFOW = *mut BROWSEINFOW;
pub const PCS_FATAL: u32 = 2147483648;
pub const PCS_PATHTOOLONG: u32 = 8;
pub const PCS_REMOVEDCHAR: u32 = 2;
pub const PCS_REPLACEDCHAR: u32 = 1;
pub const PCS_TRUNCATED: u32 = 4;
#[cfg(feature = "shtypes")]
pub type PDETAILSINFO = *mut DETAILSINFO;
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
pub type PDFMICS = *mut DFMICS;
pub const PDTIMER_PAUSE: u32 = 2;
pub const PDTIMER_RESET: u32 = 1;
pub const PDTIMER_RESUME: u32 = 3;
pub const PIDISF_CACHEDSTICKY: u32 = 2;
pub const PIDISF_CACHEIMAGES: u32 = 16;
pub const PIDISF_FOLLOWALLLINKS: u32 = 32;
pub const PIDISF_RECENTLYCHANGED: u32 = 1;
pub const PIDISM_DONTWATCH: u32 = 2;
pub const PIDISM_GLOBAL: u32 = 0;
pub const PIDISM_WATCH: u32 = 1;
pub const PIDISR_NEEDS_ADD: u32 = 1;
pub const PIDISR_NEEDS_DELETE: u32 = 3;
pub const PIDISR_NEEDS_UPDATE: u32 = 2;
pub const PIDISR_UP_TO_DATE: u32 = 0;
pub const PID_INTSITE_AUTHOR: u32 = 3;
pub const PID_INTSITE_CODEPAGE: u32 = 18;
pub const PID_INTSITE_COMMENT: u32 = 8;
pub const PID_INTSITE_CONTENTCODE: u32 = 11;
pub const PID_INTSITE_CONTENTLEN: u32 = 10;
pub const PID_INTSITE_DESCRIPTION: u32 = 7;
pub const PID_INTSITE_FLAGS: u32 = 9;
pub const PID_INTSITE_ICONFILE: u32 = 21;
pub const PID_INTSITE_ICONINDEX: u32 = 20;
pub const PID_INTSITE_LASTMOD: u32 = 5;
pub const PID_INTSITE_LASTVISIT: u32 = 4;
pub const PID_INTSITE_RECURSE: u32 = 12;
pub const PID_INTSITE_ROAMED: u32 = 34;
pub const PID_INTSITE_SUBSCRIPTION: u32 = 14;
pub const PID_INTSITE_TITLE: u32 = 16;
pub const PID_INTSITE_TRACKING: u32 = 19;
pub const PID_INTSITE_URL: u32 = 15;
pub const PID_INTSITE_VISITCOUNT: u32 = 6;
pub const PID_INTSITE_WATCH: u32 = 13;
pub const PID_INTSITE_WHATSNEW: u32 = 2;
pub const PID_IS_AUTHOR: u32 = 11;
pub const PID_IS_COMMENT: u32 = 13;
pub const PID_IS_DESCRIPTION: u32 = 12;
pub const PID_IS_HOTKEY: u32 = 6;
pub const PID_IS_ICONFILE: u32 = 9;
pub const PID_IS_ICONINDEX: u32 = 8;
pub const PID_IS_NAME: u32 = 4;
pub const PID_IS_ROAMED: u32 = 15;
pub const PID_IS_SHOWCMD: u32 = 7;
pub const PID_IS_URL: u32 = 2;
pub const PID_IS_WHATSNEW: u32 = 10;
pub const PID_IS_WORKINGDIR: u32 = 5;
pub const PIFDEFFILESIZE: u32 = 80;
pub const PIFDEFPATHSIZE: u32 = 64;
pub const PIFMAXFILEPATH: u32 = 260;
pub const PIFNAMESIZE: u32 = 30;
pub const PIFPARAMSSIZE: u32 = 64;
pub const PIFSHDATASIZE: u32 = 64;
pub const PIFSHPROGSIZE: u32 = 64;
pub const PIFSTARTLOCSIZE: u32 = 63;
pub type POPENASINFO = *mut OPENASINFO;
pub type PPROPPRG = *mut PROPPRG;
pub const PRF_DONTFINDLNK: u32 = 8;
pub const PRF_FIRSTDIRDEF: u32 = 4;
pub const PRF_REQUIREABSOLUTE: u32 = 16;
pub const PRF_TRYPROGRAMEXTENSIONS: u32 = 3;
pub const PRF_VERIFYEXISTS: u32 = 1;
pub const PROGDLG_AUTOTIME: u32 = 2;
pub const PROGDLG_MARQUEEPROGRESS: u32 = 32;
pub const PROGDLG_MODAL: u32 = 1;
pub const PROGDLG_NOCANCEL: u32 = 64;
pub const PROGDLG_NOMINIMIZE: u32 = 8;
pub const PROGDLG_NOPROGRESSBAR: u32 = 16;
pub const PROGDLG_NORMAL: u32 = 0;
pub const PROGDLG_NOTIME: u32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [i8; 30],
    pub achCmdLine: [i8; 128],
    pub achWorkDir: [i8; 64],
    pub wHotKey: u16,
    pub achIconFile: [i8; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [i8; 80],
    pub achPIFFile: [i8; 260],
}
impl Default for PROPPRG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct QCMINFO {
    pub hmenu: super::windef::HMENU,
    pub indexMenu: u32,
    pub idCmdFirst: u32,
    pub idCmdLast: u32,
    pub pIdMap: *const QCMINFO_IDMAP,
}
#[cfg(feature = "windef")]
impl Default for QCMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QCMINFO_IDMAP {
    pub nMaxIds: u32,
    pub pIdList: [QCMINFO_IDMAP_PLACEMENT; 1],
}
impl Default for QCMINFO_IDMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QCMINFO_IDMAP_PLACEMENT {
    pub id: u32,
    pub fFlags: u32,
}
pub const QCMINFO_PLACE_AFTER: u32 = 1;
pub const QCMINFO_PLACE_BEFORE: u32 = 0;
pub const QIF_CACHED: u32 = 1;
pub const QIF_DONTEXPANDFOLDER: u32 = 2;
pub const QITIPF_DEFAULT: u32 = 0;
pub const QITIPF_LINKNOTARGET: u32 = 2;
pub const QITIPF_LINKUSETARGET: u32 = 4;
pub const QITIPF_SINGLELINE: u32 = 16;
pub const QITIPF_USENAME: u32 = 1;
pub const QITIPF_USESLOWTIP: u32 = 8;
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
pub const SFVM_ADDPROPERTYPAGES: u32 = 47;
pub const SFVM_BACKGROUNDENUM: u32 = 32;
pub const SFVM_BACKGROUNDENUMDONE: u32 = 48;
pub const SFVM_COLUMNCLICK: u32 = 24;
pub const SFVM_DEFITEMCOUNT: u32 = 26;
pub const SFVM_DEFVIEWMODE: u32 = 27;
pub const SFVM_DIDDRAGDROP: u32 = 36;
pub const SFVM_FSNOTIFY: u32 = 14;
pub const SFVM_GETANIMATION: u32 = 68;
pub const SFVM_GETBUTTONINFO: u32 = 5;
pub const SFVM_GETBUTTONS: u32 = 6;
pub const SFVM_GETDETAILSOF: u32 = 23;
pub const SFVM_GETHELPTEXT: u32 = 3;
pub const SFVM_GETHELPTOPIC: u32 = 63;
pub const SFVM_GETNOTIFY: u32 = 49;
pub const SFVM_GETPANE: u32 = 59;
pub const SFVM_GETSORTDEFAULTS: u32 = 53;
pub const SFVM_GETTOOLTIPTEXT: u32 = 4;
pub const SFVM_GETZONE: u32 = 58;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SFVM_HELPTOPIC_DATA {
    pub wszHelpFile: [u16; 260],
    pub wszHelpTopic: [u16; 260],
}
impl Default for SFVM_HELPTOPIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SFVM_INITMENUPOPUP: u32 = 7;
pub const SFVM_INVOKECOMMAND: u32 = 2;
pub const SFVM_MERGEMENU: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "prsht"))]
#[derive(Clone, Copy, Default)]
pub struct SFVM_PROPPAGE_DATA {
    pub dwReserved: u32,
    pub pfn: super::prsht::LPFNADDPROPSHEETPAGE,
    pub lParam: super::minwindef::LPARAM,
}
pub const SFVM_QUERYFSNOTIFY: u32 = 25;
pub const SFVM_SETISFV: u32 = 39;
pub const SFVM_SIZE: u32 = 57;
pub const SFVM_THISIDLIST: u32 = 41;
pub const SFVM_UNMERGEMENU: u32 = 28;
pub const SFVM_UPDATESTATUSBAR: u32 = 31;
pub const SFVM_WINDOWCREATED: u32 = 15;
pub const SFVSOC_INVALIDATE_ALL: u32 = 1;
pub const SFVSOC_NOSCROLL: u32 = 2;
pub const SFVS_SELECT_ALLITEMS: u32 = 1;
pub const SFVS_SELECT_INVERT: u32 = 2;
pub const SFVS_SELECT_NONE: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
#[derive(Clone, Copy)]
pub struct SFV_CREATE {
    pub cbSize: u32,
    pub pshf: *mut core::ffi::c_void,
    pub psvOuter: *mut core::ffi::c_void,
    pub psfvcb: *mut core::ffi::c_void,
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
impl Default for SFV_CREATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SHARD = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shobjidl_core")]
#[derive(Clone, Copy)]
pub struct SHARDAPPIDINFO {
    pub psi: *mut core::ffi::c_void,
    pub pszAppID: windows_sys::core::PCWSTR,
}
#[cfg(feature = "shobjidl_core")]
impl Default for SHARDAPPIDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy)]
pub struct SHARDAPPIDINFOIDLIST {
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub pszAppID: windows_sys::core::PCWSTR,
}
#[cfg(feature = "shtypes")]
impl Default for SHARDAPPIDINFOIDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shobjidl_core")]
#[derive(Clone, Copy)]
pub struct SHARDAPPIDINFOLINK {
    pub psl: *mut core::ffi::c_void,
    pub pszAppID: windows_sys::core::PCWSTR,
}
#[cfg(feature = "shobjidl_core")]
impl Default for SHARDAPPIDINFOLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHARD_APPIDINFO: SHARD = 4;
pub const SHARD_APPIDINFOIDLIST: SHARD = 5;
pub const SHARD_APPIDINFOLINK: SHARD = 7;
pub const SHARD_LINK: SHARD = 6;
pub const SHARD_PATH: u32 = 2;
pub const SHARD_PATHA: SHARD = 2;
pub const SHARD_PATHW: SHARD = 3;
pub const SHARD_PIDL: SHARD = 1;
pub const SHARD_SHELLITEM: SHARD = 8;
pub const SHCNEE_MSI_CHANGE: u32 = 4;
pub const SHCNEE_MSI_UNINSTALL: u32 = 5;
pub const SHCNEE_ORDERCHANGED: u32 = 2;
pub const SHCNE_ALLEVENTS: u32 = 2147483647;
pub const SHCNE_ASSOCCHANGED: u32 = 134217728;
pub const SHCNE_ATTRIBUTES: u32 = 2048;
pub const SHCNE_CREATE: u32 = 2;
pub const SHCNE_DELETE: u32 = 4;
pub const SHCNE_DISKEVENTS: u32 = 145439;
pub const SHCNE_DRIVEADD: u32 = 256;
pub const SHCNE_DRIVEADDGUI: u32 = 65536;
pub const SHCNE_DRIVEREMOVED: u32 = 128;
pub const SHCNE_EXTENDED_EVENT: u32 = 67108864;
pub const SHCNE_FREESPACE: u32 = 262144;
pub const SHCNE_GLOBALEVENTS: u32 = 201687520;
pub const SHCNE_INTERRUPT: u32 = 2147483648;
pub const SHCNE_MEDIAINSERTED: u32 = 32;
pub const SHCNE_MEDIAREMOVED: u32 = 64;
pub const SHCNE_MKDIR: u32 = 8;
pub const SHCNE_NETSHARE: u32 = 512;
pub const SHCNE_NETUNSHARE: u32 = 1024;
pub const SHCNE_RENAMEFOLDER: u32 = 131072;
pub const SHCNE_RENAMEITEM: u32 = 1;
pub const SHCNE_RMDIR: u32 = 16;
pub const SHCNE_SERVERDISCONNECT: u32 = 16384;
pub const SHCNE_UPDATEDIR: u32 = 4096;
pub const SHCNE_UPDATEIMAGE: u32 = 32768;
pub const SHCNE_UPDATEITEM: u32 = 8192;
pub const SHCNF_DWORD: u32 = 3;
pub const SHCNF_FLUSH: u32 = 4096;
pub const SHCNF_FLUSHNOWAIT: u32 = 12288;
pub const SHCNF_IDLIST: u32 = 0;
pub const SHCNF_NOTIFYRECURSIVE: u32 = 65536;
pub const SHCNF_PATH: u32 = 1;
pub const SHCNF_PATHA: u32 = 1;
pub const SHCNF_PATHW: u32 = 5;
pub const SHCNF_PRINTER: u32 = 2;
pub const SHCNF_PRINTERA: u32 = 2;
pub const SHCNF_PRINTERW: u32 = 6;
pub const SHCNF_TYPE: u32 = 255;
pub const SHCNRF_InterruptLevel: u32 = 1;
pub const SHCNRF_NewDelivery: u32 = 32768;
pub const SHCNRF_RecursiveInterrupt: u32 = 4096;
pub const SHCNRF_ShellLevel: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SHChangeDWORDAsIDList {
    pub cb: u16,
    pub dwItem1: u32,
    pub dwItem2: u32,
    pub cbZero: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy)]
pub struct SHChangeNotifyEntry {
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub fRecursive: windows_sys::core::BOOL,
}
#[cfg(feature = "shtypes")]
impl Default for SHChangeNotifyEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SHChangeUpdateImageIDList {
    pub cb: u16,
    pub iIconIndex: i32,
    pub iCurIndex: i32,
    pub uFlags: u32,
    pub dwProcessID: u32,
    pub szName: [u16; 260],
    pub cbZero: u16,
}
impl Default for SHChangeUpdateImageIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SHDESCRIPTIONID {
    pub dwDescriptionId: u32,
    pub clsid: windows_sys::core::GUID,
}
pub const SHDID_COMPUTER_AUDIO: u32 = 19;
pub const SHDID_COMPUTER_CDROM: u32 = 10;
pub const SHDID_COMPUTER_DRIVE35: u32 = 5;
pub const SHDID_COMPUTER_DRIVE525: u32 = 6;
pub const SHDID_COMPUTER_FIXED: u32 = 8;
pub const SHDID_COMPUTER_IMAGING: u32 = 18;
pub const SHDID_COMPUTER_NETDRIVE: u32 = 9;
pub const SHDID_COMPUTER_OTHER: u32 = 12;
pub const SHDID_COMPUTER_RAMDISK: u32 = 11;
pub const SHDID_COMPUTER_REMOVABLE: u32 = 7;
pub const SHDID_COMPUTER_SHAREDDOCS: u32 = 20;
pub const SHDID_FS_DIRECTORY: u32 = 3;
pub const SHDID_FS_FILE: u32 = 2;
pub const SHDID_FS_OTHER: u32 = 4;
pub const SHDID_MOBILE_DEVICE: u32 = 21;
pub const SHDID_NET_DOMAIN: u32 = 13;
pub const SHDID_NET_OTHER: u32 = 17;
pub const SHDID_NET_RESTOFNET: u32 = 16;
pub const SHDID_NET_SERVER: u32 = 14;
pub const SHDID_NET_SHARE: u32 = 15;
pub const SHDID_REMOTE_DESKTOP_DRIVE: u32 = 22;
pub const SHDID_ROOT_REGITEM: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SHELLFLAGSTATE {
    pub _bitfield: windows_sys::core::BOOL,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SHELLSTATEA {
    pub _bitfield1: windows_sys::core::BOOL,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: windows_sys::core::BOOL,
}
pub const SHELLSTATEVERSION_IE4: u32 = 9;
pub const SHELLSTATEVERSION_WIN2K: u32 = 10;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SHELLSTATEW {
    pub _bitfield1: windows_sys::core::BOOL,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: windows_sys::core::BOOL,
}
pub const SHELLSTATE_SIZE_IE4: u32 = 24;
pub const SHELLSTATE_SIZE_NT4: u32 = 20;
pub const SHELLSTATE_SIZE_WIN95: u32 = 12;
pub type SHELL_LINK_DATA_FLAGS = u32;
pub const SHFMT_CANCEL: u32 = 4294967294;
pub const SHFMT_ERROR: u32 = 4294967295;
pub const SHFMT_ID_DEFAULT: u32 = 65535;
pub const SHFMT_NOFORMAT: u32 = 4294967293;
pub const SHFMT_OPT_FULL: u32 = 1;
pub const SHFMT_OPT_SYSONLY: u32 = 2;
#[repr(C)]
#[cfg(feature = "shobjidl_core")]
#[derive(Clone, Copy)]
pub struct SHFOLDERCUSTOMSETTINGS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub pvid: *mut super::shobjidl_core::SHELLVIEWID,
    pub pszWebViewTemplate: windows_sys::core::PWSTR,
    pub cchWebViewTemplate: u32,
    pub pszWebViewTemplateVersion: windows_sys::core::PWSTR,
    pub pszInfoTip: windows_sys::core::PWSTR,
    pub cchInfoTip: u32,
    pub pclsid: *mut windows_sys::core::GUID,
    pub dwFlags: u32,
    pub pszIconFile: windows_sys::core::PWSTR,
    pub cchIconFile: u32,
    pub iIconIndex: i32,
    pub pszLogo: windows_sys::core::PWSTR,
    pub cchLogo: u32,
}
#[cfg(feature = "shobjidl_core")]
impl Default for SHFOLDERCUSTOMSETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHGDFIL_DESCRIPTIONID: u32 = 3;
pub const SHGDFIL_FINDDATA: u32 = 1;
pub const SHGDFIL_NETRESOURCE: u32 = 2;
pub type SHGFP_TYPE = i32;
pub const SHGFP_TYPE_CURRENT: SHGFP_TYPE = 0;
pub const SHGFP_TYPE_DEFAULT: SHGFP_TYPE = 1;
pub const SHOP_FILEPATH: u32 = 2;
pub const SHOP_PRINTERNAME: u32 = 1;
pub const SHOP_VOLUMEGUID: u32 = 4;
pub const SHPPFW_ASKDIRCREATE: u32 = 2;
pub const SHPPFW_DEFAULT: u32 = 1;
pub const SHPPFW_DIRCREATE: u32 = 1;
pub const SHPPFW_IGNOREFILENAME: u32 = 4;
pub const SHPPFW_MEDIACHECKONLY: u32 = 16;
pub const SHPPFW_NONE: u32 = 0;
pub const SHPPFW_NOWRITECHECK: u32 = 8;
pub const SIOM_ICONINDEX: u32 = 2;
pub const SIOM_OVERLAYINDEX: u32 = 1;
pub const SIOM_RESERVED_DEFAULT: u32 = 3;
pub const SIOM_RESERVED_LINK: u32 = 1;
pub const SIOM_RESERVED_SHARED: u32 = 0;
pub const SIOM_RESERVED_SLOWFILE: u32 = 2;
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
pub const SLDF_RESERVED: SHELL_LINK_DATA_FLAGS = 2147483648;
pub const SLDF_RUNAS_USER: SHELL_LINK_DATA_FLAGS = 8192;
pub const SLDF_RUN_IN_SEPARATE: SHELL_LINK_DATA_FLAGS = 1024;
pub const SLDF_RUN_WITH_SHIMLAYER: SHELL_LINK_DATA_FLAGS = 131072;
pub const SLDF_UNALIAS_ON_SAVE: SHELL_LINK_DATA_FLAGS = 16777216;
pub const SLDF_UNICODE: SHELL_LINK_DATA_FLAGS = 128;
pub const SLDF_VALID: SHELL_LINK_DATA_FLAGS = 268433407;
pub const SSF_AUTOCHECKSELECT: u32 = 8388608;
pub const SSF_DESKTOPHTML: u32 = 512;
pub const SSF_DONTPRETTYPATH: u32 = 2048;
pub const SSF_DOUBLECLICKINWEBVIEW: u32 = 128;
pub const SSF_FILTER: u32 = 65536;
pub const SSF_HIDDENFILEEXTS: u32 = 4;
pub const SSF_HIDEICONS: u32 = 16384;
pub const SSF_ICONSONLY: u32 = 16777216;
pub const SSF_MAPNETDRVBUTTON: u32 = 4096;
pub const SSF_NOCONFIRMRECYCLE: u32 = 32768;
pub const SSF_NONETCRAWLING: u32 = 1048576;
pub const SSF_SEPPROCESS: u32 = 524288;
pub const SSF_SERVERADMINUI: u32 = 4;
pub const SSF_SHOWALLOBJECTS: u32 = 1;
pub const SSF_SHOWATTRIBCOL: u32 = 256;
pub const SSF_SHOWCOMPCOLOR: u32 = 8;
pub const SSF_SHOWEXTENSIONS: u32 = 2;
pub const SSF_SHOWINFOTIP: u32 = 8192;
pub const SSF_SHOWSTARTPAGE: u32 = 4194304;
pub const SSF_SHOWSTATUSBAR: u32 = 67108864;
pub const SSF_SHOWSUPERHIDDEN: u32 = 262144;
pub const SSF_SHOWSYSFILES: u32 = 32;
pub const SSF_SHOWTYPEOVERLAY: u32 = 33554432;
pub const SSF_SORTCOLUMNS: u32 = 16;
pub const SSF_STARTPANELON: u32 = 2097152;
pub const SSF_WEBVIEW: u32 = 131072;
pub const SSF_WIN95CLASSIC: u32 = 1024;
pub const STR_PARSE_PARTIAL_IDLIST: windows_sys::core::PCWSTR = windows_sys::core::w!("ParseOriginalItem");
pub const STR_PARSE_WITH_PROPERTIES: windows_sys::core::PCWSTR = windows_sys::core::w!("ParseWithProperties");
pub const VALIDATEUNC_CONNECT: u32 = 1;
pub const VALIDATEUNC_NOUI: u32 = 2;
pub const VALIDATEUNC_PERSIST: u32 = 8;
pub const VALIDATEUNC_PRINT: u32 = 4;
pub const VALIDATEUNC_VALID: u32 = 15;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WALLPAPEROPT {
    pub dwSize: u32,
    pub dwStyle: u32,
}
pub const WPSTYLE_CENTER: u32 = 0;
pub const WPSTYLE_CROPTOFIT: u32 = 4;
pub const WPSTYLE_KEEPASPECT: u32 = 3;
pub const WPSTYLE_MAX: u32 = 6;
pub const WPSTYLE_SPAN: u32 = 5;
pub const WPSTYLE_STRETCH: u32 = 2;
pub const WPSTYLE_TILE: u32 = 1;
pub type tagDTI_ADTIWUI = i32;
pub type tagOPEN_AS_INFO_FLAGS = i32;
