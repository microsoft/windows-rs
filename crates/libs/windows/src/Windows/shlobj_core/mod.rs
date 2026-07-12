#[cfg(all(feature = "oaidl", feature = "shobjidl_core", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn AssocGetDetailsOfPropKey<P0>(psf: P0, pidl: *const super::shtypes::ITEMIDLIST, pkey: *const super::wtypes::PROPERTYKEY, pv: *mut super::oaidl::VARIANT, pffoundpropkey: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn AssocGetDetailsOfPropKey(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, pkey : *const super::wtypes::PROPERTYKEY, pv : *mut super::oaidl::VARIANT, pffoundpropkey : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { AssocGetDetailsOfPropKey(psf.param().abi(), pidl, pkey, core::mem::transmute(pv), pffoundpropkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn CDefFolderMenu_Create2<P4>(pidlfolder: Option<*const super::shtypes::ITEMIDLIST>, hwnd: Option<super::windef::HWND>, apidl: Option<&[super::shtypes::LPCITEMIDLIST]>, psf: P4, pfn: LPFNDFMCALLBACK, ahkeys: Option<&[super::minwindef::HKEY]>) -> windows_core::Result<super::shobjidl_core::IContextMenu>
where
    P4: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn CDefFolderMenu_Create2(pidlfolder : *const super::shtypes::ITEMIDLIST, hwnd : super::windef::HWND, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, psf : *mut core::ffi::c_void, pfn : LPFNDFMCALLBACK, nkeys : u32, ahkeys : *const super::minwindef::HKEY, ppcm : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CDefFolderMenu_Create2(pidlfolder.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, apidl.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(apidl.map_or(core::ptr::null(), |slice| slice.as_ptr())), psf.param().abi(), pfn, ahkeys.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ahkeys.map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "objidl", feature = "shtypes"))]
#[inline]
pub unsafe fn CIDLData_CreateFromIDArray(pidlfolder: *const super::shtypes::ITEMIDLIST, apidl: Option<&[super::shtypes::LPCITEMIDLIST]>) -> windows_core::Result<super::objidl::IDataObject> {
    windows_core::link!("shell32.dll" "system" fn CIDLData_CreateFromIDArray(pidlfolder : *const super::shtypes::ITEMIDLIST, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, ppdtobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CIDLData_CreateFromIDArray(pidlfolder, apidl.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(apidl.map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DAD_AutoScroll(hwnd: super::windef::HWND, pad: *mut AUTO_SCROLL_DATA, pptnow: *const super::windef::POINT) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn DAD_AutoScroll(hwnd : super::windef::HWND, pad : *mut AUTO_SCROLL_DATA, pptnow : *const super::windef::POINT) -> windows_core::BOOL);
    unsafe { DAD_AutoScroll(hwnd, pad as _, pptnow) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DAD_DragEnterEx(hwndtarget: super::windef::HWND, ptstart: super::windef::POINT) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn DAD_DragEnterEx(hwndtarget : super::windef::HWND, ptstart : super::windef::POINT) -> windows_core::BOOL);
    unsafe { DAD_DragEnterEx(hwndtarget, core::mem::transmute(ptstart)) }
}
#[cfg(all(feature = "objidl", feature = "windef"))]
#[inline]
pub unsafe fn DAD_DragEnterEx2<P2>(hwndtarget: super::windef::HWND, ptstart: super::windef::POINT, pdtobject: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<super::objidl::IDataObject>,
{
    windows_core::link!("shell32.dll" "system" fn DAD_DragEnterEx2(hwndtarget : super::windef::HWND, ptstart : super::windef::POINT, pdtobject : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DAD_DragEnterEx2(hwndtarget, core::mem::transmute(ptstart), pdtobject.param().abi()) }
}
#[inline]
pub unsafe fn DAD_DragLeave() -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn DAD_DragLeave() -> windows_core::BOOL);
    unsafe { DAD_DragLeave() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DAD_DragMove(pt: super::windef::POINT) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn DAD_DragMove(pt : super::windef::POINT) -> windows_core::BOOL);
    unsafe { DAD_DragMove(core::mem::transmute(pt)) }
}
#[cfg(all(feature = "commctrl", feature = "windef"))]
#[inline]
pub unsafe fn DAD_SetDragImage(him: *mut super::commctrl::_IMAGELIST, pptoffset: *mut super::windef::POINT) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn DAD_SetDragImage(him : *mut super::commctrl::_IMAGELIST, pptoffset : *mut super::windef::POINT) -> windows_core::BOOL);
    unsafe { DAD_SetDragImage(him as _, pptoffset as _) }
}
#[inline]
pub unsafe fn DAD_ShowDragImage(fshow: bool) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn DAD_ShowDragImage(fshow : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DAD_ShowDragImage(fshow.into()) }
}
#[inline]
pub unsafe fn DriveType(idrive: i32) -> i32 {
    windows_core::link!("shell32.dll" "system" fn DriveType(idrive : i32) -> i32);
    unsafe { DriveType(idrive) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetFileNameFromBrowse<P3, P4, P5, P6>(hwnd: Option<super::windef::HWND>, pszfilepath: &mut [u16], pszworkingdir: P3, pszdefext: P4, pszfilters: P5, psztitle: P6) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn GetFileNameFromBrowse(hwnd : super::windef::HWND, pszfilepath : windows_core::PWSTR, cchfilepath : u32, pszworkingdir : windows_core::PCWSTR, pszdefext : windows_core::PCWSTR, pszfilters : windows_core::PCWSTR, psztitle : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { GetFileNameFromBrowse(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszfilepath.as_ptr()), pszfilepath.len().try_into().unwrap(), pszworkingdir.param().abi(), pszdefext.param().abi(), pszfilters.param().abi(), psztitle.param().abi()) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILAppendID(pidl: Option<*const super::shtypes::ITEMIDLIST>, pmkid: *const super::shtypes::SHITEMID, fappend: bool) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn ILAppendID(pidl : *const super::shtypes::ITEMIDLIST, pmkid : *const super::shtypes::SHITEMID, fappend : windows_core::BOOL) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILAppendID(pidl.unwrap_or(core::mem::zeroed()) as _, pmkid, fappend.into()) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILClone(pidl: *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn ILClone(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILClone(pidl) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILCloneFirst(pidl: *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn ILCloneFirst(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILCloneFirst(pidl) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILCombine(pidl1: Option<*const super::shtypes::ITEMIDLIST>, pidl2: Option<*const super::shtypes::ITEMIDLIST>) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn ILCombine(pidl1 : *const super::shtypes::ITEMIDLIST, pidl2 : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILCombine(pidl1.unwrap_or(core::mem::zeroed()) as _, pidl2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILCreateFromPathA<P0>(pszpath: P0) -> super::shtypes::LPITEMIDLIST
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn ILCreateFromPathA(pszpath : windows_core::PCSTR) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILCreateFromPathA(pszpath.param().abi()) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILCreateFromPathW<P0>(pszpath: P0) -> super::shtypes::LPITEMIDLIST
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn ILCreateFromPathW(pszpath : windows_core::PCWSTR) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILCreateFromPathW(pszpath.param().abi()) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILFindChild(pidlparent: *const super::shtypes::ITEMIDLIST, pidlchild: *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn ILFindChild(pidlparent : *const super::shtypes::ITEMIDLIST, pidlchild : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILFindChild(pidlparent, pidlchild) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILFindLastID(pidl: *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn ILFindLastID(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILFindLastID(pidl) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILFree(pidl: Option<*const super::shtypes::ITEMIDLIST>) {
    windows_core::link!("shell32.dll" "system" fn ILFree(pidl : *const super::shtypes::ITEMIDLIST));
    unsafe { ILFree(pidl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILGetNext(pidl: Option<*const super::shtypes::ITEMIDLIST>) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn ILGetNext(pidl : *const super::shtypes::ITEMIDLIST) -> super::shtypes::LPITEMIDLIST);
    unsafe { ILGetNext(pidl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILGetSize(pidl: Option<*const super::shtypes::ITEMIDLIST>) -> u32 {
    windows_core::link!("shell32.dll" "system" fn ILGetSize(pidl : *const super::shtypes::ITEMIDLIST) -> u32);
    unsafe { ILGetSize(pidl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILIsEqual(pidl1: *const super::shtypes::ITEMIDLIST, pidl2: *const super::shtypes::ITEMIDLIST) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn ILIsEqual(pidl1 : *const super::shtypes::ITEMIDLIST, pidl2 : *const super::shtypes::ITEMIDLIST) -> windows_core::BOOL);
    unsafe { ILIsEqual(pidl1, pidl2) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILIsParent(pidl1: *const super::shtypes::ITEMIDLIST, pidl2: *const super::shtypes::ITEMIDLIST, fimmediate: bool) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn ILIsParent(pidl1 : *const super::shtypes::ITEMIDLIST, pidl2 : *const super::shtypes::ITEMIDLIST, fimmediate : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ILIsParent(pidl1, pidl2, fimmediate.into()) }
}
#[cfg(all(feature = "objidlbase", feature = "shtypes"))]
#[inline]
pub unsafe fn ILLoadFromStreamEx<P0>(pstm: P0) -> windows_core::Result<super::shtypes::LPITEMIDLIST>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shell32.dll" "system" fn ILLoadFromStreamEx(pstm : *mut core::ffi::c_void, pidl : *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ILLoadFromStreamEx(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn ILRemoveLastID(pidl: Option<*mut super::shtypes::ITEMIDLIST>) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn ILRemoveLastID(pidl : *mut super::shtypes::ITEMIDLIST) -> windows_core::BOOL);
    unsafe { ILRemoveLastID(pidl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "objidlbase", feature = "shtypes"))]
#[inline]
pub unsafe fn ILSaveToStream<P0>(pstm: P0, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shell32.dll" "system" fn ILSaveToStream(pstm : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT);
    unsafe { ILSaveToStream(pstm.param().abi(), pidl) }
}
#[inline]
pub unsafe fn IsNetDrive(idrive: i32) -> i32 {
    windows_core::link!("shell32.dll" "system" fn IsNetDrive(idrive : i32) -> i32);
    unsafe { IsNetDrive(idrive) }
}
#[inline]
pub unsafe fn IsUserAnAdmin() -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn IsUserAnAdmin() -> windows_core::BOOL);
    unsafe { IsUserAnAdmin() }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
#[inline]
pub unsafe fn OpenRegStream<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, grfmode: u32) -> Option<super::objidlbase::IStream>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn OpenRegStream(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, grfmode : u32) -> Option < super::objidlbase::IStream >);
    unsafe { OpenRegStream(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), grfmode) }
}
#[inline]
pub unsafe fn PathCleanupSpec<P0>(pszdir: P0, pszspec: windows_core::PWSTR) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathCleanupSpec(pszdir : windows_core::PCWSTR, pszspec : windows_core::PWSTR) -> i32);
    unsafe { PathCleanupSpec(pszdir.param().abi(), core::mem::transmute(pszspec)) }
}
#[inline]
pub unsafe fn PathGetShortPath(pszlongpath: windows_core::PWSTR) {
    windows_core::link!("shell32.dll" "system" fn PathGetShortPath(pszlongpath : windows_core::PWSTR));
    unsafe { PathGetShortPath(core::mem::transmute(pszlongpath)) }
}
#[inline]
pub unsafe fn PathIsExe<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathIsExe(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsExe(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathMakeUniqueName<P2, P3, P4>(pszuniquename: &mut [u16], psztemplate: P2, pszlongplate: P3, pszdir: P4) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathMakeUniqueName(pszuniquename : windows_core::PWSTR, cchmax : u32, psztemplate : windows_core::PCWSTR, pszlongplate : windows_core::PCWSTR, pszdir : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathMakeUniqueName(core::mem::transmute(pszuniquename.as_ptr()), pszuniquename.len().try_into().unwrap(), psztemplate.param().abi(), pszlongplate.param().abi(), pszdir.param().abi()) }
}
#[inline]
pub unsafe fn PathResolve(pszpath: windows_core::PWSTR, dirs: Option<*const windows_core::PCWSTR>, fflags: u32) -> i32 {
    windows_core::link!("shell32.dll" "system" fn PathResolve(pszpath : windows_core::PWSTR, dirs : *const windows_core::PCWSTR, fflags : u32) -> i32);
    unsafe { PathResolve(core::mem::transmute(pszpath), dirs.unwrap_or(core::mem::zeroed()) as _, fflags) }
}
#[inline]
pub unsafe fn PathYetAnotherMakeUniqueName<P1, P2, P3>(pszuniquename: windows_core::PWSTR, pszpath: P1, pszshort: P2, pszfilespec: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathYetAnotherMakeUniqueName(pszuniquename : windows_core::PWSTR, pszpath : windows_core::PCWSTR, pszshort : windows_core::PCWSTR, pszfilespec : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathYetAnotherMakeUniqueName(core::mem::transmute(pszuniquename), pszpath.param().abi(), pszshort.param().abi(), pszfilespec.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PickIconDlg(hwnd: Option<super::windef::HWND>, psziconpath: &mut [u16], piiconindex: Option<*mut i32>) -> i32 {
    windows_core::link!("shell32.dll" "system" fn PickIconDlg(hwnd : super::windef::HWND, psziconpath : windows_core::PWSTR, cchiconpath : u32, piiconindex : *mut i32) -> i32);
    unsafe { PickIconDlg(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(psziconpath.as_ptr()), psziconpath.len().try_into().unwrap(), piiconindex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PifMgr_CloseProperties(hprops: Option<super::winnt::HANDLE>, flopt: u32) -> super::winnt::HANDLE {
    windows_core::link!("shell32.dll" "system" fn PifMgr_CloseProperties(hprops : super::winnt::HANDLE, flopt : u32) -> super::winnt::HANDLE);
    unsafe { PifMgr_CloseProperties(hprops.unwrap_or(core::mem::zeroed()) as _, flopt) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PifMgr_GetProperties<P1>(hprops: Option<super::winnt::HANDLE>, pszgroup: P1, lpprops: Option<*mut core::ffi::c_void>, cbprops: i32, flopt: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PifMgr_GetProperties(hprops : super::winnt::HANDLE, pszgroup : windows_core::PCSTR, lpprops : *mut core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
    unsafe { PifMgr_GetProperties(hprops.unwrap_or(core::mem::zeroed()) as _, pszgroup.param().abi(), lpprops.unwrap_or(core::mem::zeroed()) as _, cbprops, flopt) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PifMgr_OpenProperties<P0, P1>(pszapp: P0, pszpif: P1, hinf: u32, flopt: u32) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PifMgr_OpenProperties(pszapp : windows_core::PCWSTR, pszpif : windows_core::PCWSTR, hinf : u32, flopt : u32) -> super::winnt::HANDLE);
    unsafe { PifMgr_OpenProperties(pszapp.param().abi(), pszpif.param().abi(), hinf, flopt) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PifMgr_SetProperties<P1>(hprops: Option<super::winnt::HANDLE>, pszgroup: P1, lpprops: *const core::ffi::c_void, cbprops: i32, flopt: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PifMgr_SetProperties(hprops : super::winnt::HANDLE, pszgroup : windows_core::PCSTR, lpprops : *const core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
    unsafe { PifMgr_SetProperties(hprops.unwrap_or(core::mem::zeroed()) as _, pszgroup.param().abi(), lpprops, cbprops, flopt) }
}
#[inline]
pub unsafe fn ReadCabinetState(pcs: *mut CABINETSTATE, clength: i32) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn ReadCabinetState(pcs : *mut CABINETSTATE, clength : i32) -> windows_core::BOOL);
    unsafe { ReadCabinetState(pcs as _, clength) }
}
#[inline]
pub unsafe fn RealDriveType(idrive: i32, foktohitnet: bool) -> i32 {
    windows_core::link!("shell32.dll" "system" fn RealDriveType(idrive : i32, foktohitnet : windows_core::BOOL) -> i32);
    unsafe { RealDriveType(idrive, foktohitnet.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RestartDialog<P1>(hwnd: Option<super::windef::HWND>, pszprompt: P1, dwreturn: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn RestartDialog(hwnd : super::windef::HWND, pszprompt : windows_core::PCWSTR, dwreturn : u32) -> i32);
    unsafe { RestartDialog(hwnd.unwrap_or(core::mem::zeroed()) as _, pszprompt.param().abi(), dwreturn) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RestartDialogEx<P1>(hwnd: Option<super::windef::HWND>, pszprompt: P1, dwreturn: u32, dwreasoncode: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn RestartDialogEx(hwnd : super::windef::HWND, pszprompt : windows_core::PCWSTR, dwreturn : u32, dwreasoncode : u32) -> i32);
    unsafe { RestartDialogEx(hwnd.unwrap_or(core::mem::zeroed()) as _, pszprompt.param().abi(), dwreturn, dwreasoncode) }
}
#[cfg(all(feature = "minwindef", feature = "prsht"))]
#[inline]
pub unsafe fn SHAddFromPropSheetExtArray(hpsxa: HPSXA, lpfnaddpage: super::prsht::LPFNADDPROPSHEETPAGE, lparam: super::minwindef::LPARAM) -> u32 {
    windows_core::link!("shell32.dll" "system" fn SHAddFromPropSheetExtArray(hpsxa : HPSXA, lpfnaddpage : super::prsht::LPFNADDPROPSHEETPAGE, lparam : super::minwindef::LPARAM) -> u32);
    unsafe { SHAddFromPropSheetExtArray(hpsxa, lpfnaddpage, lparam) }
}
#[inline]
pub unsafe fn SHAddToRecentDocs(uflags: u32, pv: Option<*const core::ffi::c_void>) {
    windows_core::link!("shell32.dll" "system" fn SHAddToRecentDocs(uflags : u32, pv : *const core::ffi::c_void));
    unsafe { SHAddToRecentDocs(uflags, pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SHAlloc(cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("shell32.dll" "system" fn SHAlloc(cb : usize) -> *mut core::ffi::c_void);
    unsafe { SHAlloc(cb) }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHBindToFolderIDListParent<P0, T>(psfroot: P0, pidl: *const super::shtypes::ITEMIDLIST, ppidllast: *mut super::shtypes::LPCITEMIDLIST) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHBindToFolderIDListParent(psfroot : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::shtypes::LPCITEMIDLIST) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHBindToFolderIDListParent(psfroot.param().abi(), pidl, &T::IID, &mut result__, ppidllast as _).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHBindToFolderIDListParentEx<P0, P2, T>(psfroot: P0, pidl: *const super::shtypes::ITEMIDLIST, ppbc: P2, ppidllast: *mut super::shtypes::LPCITEMIDLIST) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
    P2: windows_core::Param<super::objidl::IBindCtx>,
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHBindToFolderIDListParentEx(psfroot : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, ppbc : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::shtypes::LPCITEMIDLIST) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHBindToFolderIDListParentEx(psfroot.param().abi(), pidl, ppbc.param().abi(), &T::IID, &mut result__, ppidllast as _).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHBindToObject<P0, P2, T>(psf: P0, pidl: *const super::shtypes::ITEMIDLIST, pbc: P2) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
    P2: windows_core::Param<super::objidl::IBindCtx>,
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHBindToObject(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, pbc : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHBindToObject(psf.param().abi(), pidl, pbc.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHBindToParent<T>(pidl: *const super::shtypes::ITEMIDLIST, ppidllast: *mut super::shtypes::LPCITEMIDLIST) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHBindToParent(pidl : *const super::shtypes::ITEMIDLIST, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void, ppidllast : *mut super::shtypes::LPCITEMIDLIST) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHBindToParent(pidl, &T::IID, &mut result__, ppidllast as _).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn SHBrowseForFolderA(lpbi: *const BROWSEINFOA) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn SHBrowseForFolderA(lpbi : *const BROWSEINFOA) -> super::shtypes::LPITEMIDLIST);
    unsafe { SHBrowseForFolderA(lpbi) }
}
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn SHBrowseForFolderW(lpbi: *const BROWSEINFOW) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn SHBrowseForFolderW(lpbi : *const BROWSEINFOW) -> super::shtypes::LPITEMIDLIST);
    unsafe { SHBrowseForFolderW(lpbi) }
}
#[inline]
pub unsafe fn SHCLSIDFromString<P0>(psz: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHCLSIDFromString(psz : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCLSIDFromString(psz.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "shtypes", feature = "winnt"))]
#[inline]
pub unsafe fn SHChangeNotification_Lock(hchange: super::winnt::HANDLE, dwprocid: u32, pppidl: *mut *mut super::shtypes::LPITEMIDLIST, plevent: Option<*mut i32>) -> super::winnt::HANDLE {
    windows_core::link!("shell32.dll" "system" fn SHChangeNotification_Lock(hchange : super::winnt::HANDLE, dwprocid : u32, pppidl : *mut *mut super::shtypes::LPITEMIDLIST, plevent : *mut i32) -> super::winnt::HANDLE);
    unsafe { SHChangeNotification_Lock(hchange, dwprocid, pppidl as _, plevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SHChangeNotification_Unlock(hlock: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHChangeNotification_Unlock(hlock : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { SHChangeNotification_Unlock(hlock) }
}
#[inline]
pub unsafe fn SHChangeNotify(weventid: i32, uflags: u32, dwitem1: Option<*const core::ffi::c_void>, dwitem2: Option<*const core::ffi::c_void>) {
    windows_core::link!("shell32.dll" "system" fn SHChangeNotify(weventid : i32, uflags : u32, dwitem1 : *const core::ffi::c_void, dwitem2 : *const core::ffi::c_void));
    unsafe { SHChangeNotify(weventid, uflags, dwitem1.unwrap_or(core::mem::zeroed()) as _, dwitem2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SHChangeNotifyDeregister(ulid: u32) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHChangeNotifyDeregister(ulid : u32) -> windows_core::BOOL);
    unsafe { SHChangeNotifyDeregister(ulid) }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn SHChangeNotifyRegister(hwnd: super::windef::HWND, fsources: i32, fevents: i32, wmsg: u32, centries: i32, pshcne: *const SHChangeNotifyEntry) -> u32 {
    windows_core::link!("shell32.dll" "system" fn SHChangeNotifyRegister(hwnd : super::windef::HWND, fsources : i32, fevents : i32, wmsg : u32, centries : i32, pshcne : *const SHChangeNotifyEntry) -> u32);
    unsafe { SHChangeNotifyRegister(hwnd, fsources, fevents, wmsg, centries, pshcne) }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn SHCloneSpecialIDList(hwnd: Option<super::windef::HWND>, csidl: i32, fcreate: bool) -> super::shtypes::LPITEMIDLIST {
    windows_core::link!("shell32.dll" "system" fn SHCloneSpecialIDList(hwnd : super::windef::HWND, csidl : i32, fcreate : windows_core::BOOL) -> super::shtypes::LPITEMIDLIST);
    unsafe { SHCloneSpecialIDList(hwnd.unwrap_or(core::mem::zeroed()) as _, csidl, fcreate.into()) }
}
#[inline]
pub unsafe fn SHCoCreateInstance<P0, P2, T>(pszclsid: P0, pclsid: Option<*const windows_core::GUID>, punkouter: P2) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHCoCreateInstance(pszclsid : windows_core::PCWSTR, pclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHCoCreateInstance(pszclsid.param().abi(), pclsid.unwrap_or(core::mem::zeroed()) as _, punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "objidl", feature = "shtypes"))]
#[inline]
pub unsafe fn SHCreateDataObject<P3, T>(pidlfolder: Option<*const super::shtypes::ITEMIDLIST>, apidl: Option<&[super::shtypes::LPCITEMIDLIST]>, pdtinner: P3) -> windows_core::Result<T>
where
    P3: windows_core::Param<super::objidl::IDataObject>,
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateDataObject(pidlfolder : *const super::shtypes::ITEMIDLIST, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, pdtinner : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHCreateDataObject(pidlfolder.unwrap_or(core::mem::zeroed()) as _, apidl.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(apidl.map_or(core::ptr::null(), |slice| slice.as_ptr())), pdtinner.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn SHCreateDefaultContextMenu<T>(pdcm: *const DEFCONTEXTMENU) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateDefaultContextMenu(pdcm : *const DEFCONTEXTMENU, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHCreateDefaultContextMenu(core::mem::transmute(pdcm), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHCreateDirectory<P1>(hwnd: Option<super::windef::HWND>, pszpath: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateDirectory(hwnd : super::windef::HWND, pszpath : windows_core::PCWSTR) -> i32);
    unsafe { SHCreateDirectory(hwnd.unwrap_or(core::mem::zeroed()) as _, pszpath.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[inline]
pub unsafe fn SHCreateDirectoryExA<P1>(hwnd: Option<super::windef::HWND>, pszpath: P1, psa: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateDirectoryExA(hwnd : super::windef::HWND, pszpath : windows_core::PCSTR, psa : *const super::minwinbase::SECURITY_ATTRIBUTES) -> i32);
    unsafe { SHCreateDirectoryExA(hwnd.unwrap_or(core::mem::zeroed()) as _, pszpath.param().abi(), psa.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[inline]
pub unsafe fn SHCreateDirectoryExW<P1>(hwnd: Option<super::windef::HWND>, pszpath: P1, psa: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateDirectoryExW(hwnd : super::windef::HWND, pszpath : windows_core::PCWSTR, psa : *const super::minwinbase::SECURITY_ATTRIBUTES) -> i32);
    unsafe { SHCreateDirectoryExW(hwnd.unwrap_or(core::mem::zeroed()) as _, pszpath.param().abi(), psa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SHCreateFileExtractIconW<P0, T>(pszfile: P0, dwfileattributes: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateFileExtractIconW(pszfile : windows_core::PCWSTR, dwfileattributes : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHCreateFileExtractIconW(pszfile.param().abi(), dwfileattributes, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
#[inline]
pub unsafe fn SHCreateShellFolderView(pcsfv: *const SFV_CREATE) -> windows_core::Result<super::shobjidl_core::IShellView> {
    windows_core::link!("shell32.dll" "system" fn SHCreateShellFolderView(pcsfv : *const SFV_CREATE, ppsv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateShellFolderView(core::mem::transmute(pcsfv), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn SHCreateShellFolderViewEx(pcsfv: *const CSFV) -> windows_core::Result<super::shobjidl_core::IShellView> {
    windows_core::link!("shell32.dll" "system" fn SHCreateShellFolderViewEx(pcsfv : *const CSFV, ppsv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateShellFolderViewEx(core::mem::transmute(pcsfv), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHCreateShellItem<P1>(pidlparent: Option<*const super::shtypes::ITEMIDLIST>, psfparent: P1, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<super::shobjidl_core::IShellItem>
where
    P1: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateShellItem(pidlparent : *const super::shtypes::ITEMIDLIST, psfparent : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, ppsi : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateShellItem(pidlparent.unwrap_or(core::mem::zeroed()) as _, psfparent.param().abi(), pidl, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "objidl", feature = "wtypes"))]
#[inline]
pub unsafe fn SHCreateStdEnumFmtEtc(afmt: &[super::objidl::FORMATETC]) -> windows_core::Result<super::objidl::IEnumFORMATETC> {
    windows_core::link!("shell32.dll" "system" fn SHCreateStdEnumFmtEtc(cfmt : u32, afmt : *const super::objidl::FORMATETC, ppenumformatetc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateStdEnumFmtEtc(afmt.len().try_into().unwrap(), core::mem::transmute(afmt.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHDefExtractIconA<P0>(psziconfile: P0, iindex: i32, uflags: u32, phiconlarge: Option<*mut super::windef::HICON>, phiconsmall: Option<*mut super::windef::HICON>, niconsize: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHDefExtractIconA(psziconfile : windows_core::PCSTR, iindex : i32, uflags : u32, phiconlarge : *mut super::windef::HICON, phiconsmall : *mut super::windef::HICON, niconsize : u32) -> windows_core::HRESULT);
    unsafe { SHDefExtractIconA(psziconfile.param().abi(), iindex, uflags, phiconlarge.unwrap_or(core::mem::zeroed()) as _, phiconsmall.unwrap_or(core::mem::zeroed()) as _, niconsize) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHDefExtractIconW<P0>(psziconfile: P0, iindex: i32, uflags: u32, phiconlarge: Option<*mut super::windef::HICON>, phiconsmall: Option<*mut super::windef::HICON>, niconsize: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHDefExtractIconW(psziconfile : windows_core::PCWSTR, iindex : i32, uflags : u32, phiconlarge : *mut super::windef::HICON, phiconsmall : *mut super::windef::HICON, niconsize : u32) -> windows_core::HRESULT);
    unsafe { SHDefExtractIconW(psziconfile.param().abi(), iindex, uflags, phiconlarge.unwrap_or(core::mem::zeroed()) as _, phiconsmall.unwrap_or(core::mem::zeroed()) as _, niconsize) }
}
#[inline]
pub unsafe fn SHDestroyPropSheetExtArray(hpsxa: HPSXA) {
    windows_core::link!("shell32.dll" "system" fn SHDestroyPropSheetExtArray(hpsxa : HPSXA));
    unsafe { SHDestroyPropSheetExtArray(hpsxa) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
#[inline]
pub unsafe fn SHDoDragDrop<P1, P2>(hwnd: Option<super::windef::HWND>, pdata: P1, pdsrc: P2, dweffect: u32) -> windows_core::Result<u32>
where
    P1: windows_core::Param<super::objidl::IDataObject>,
    P2: windows_core::Param<super::oleidl::IDropSource>,
{
    windows_core::link!("shell32.dll" "system" fn SHDoDragDrop(hwnd : super::windef::HWND, pdata : *mut core::ffi::c_void, pdsrc : *mut core::ffi::c_void, dweffect : u32, pdweffect : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHDoDragDrop(hwnd.unwrap_or(core::mem::zeroed()) as _, pdata.param().abi(), pdsrc.param().abi(), dweffect, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHFindFiles(pidlfolder: Option<*const super::shtypes::ITEMIDLIST>, pidlsavefile: Option<*const super::shtypes::ITEMIDLIST>) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHFindFiles(pidlfolder : *const super::shtypes::ITEMIDLIST, pidlsavefile : *const super::shtypes::ITEMIDLIST) -> windows_core::BOOL);
    unsafe { SHFindFiles(pidlfolder.unwrap_or(core::mem::zeroed()) as _, pidlsavefile.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
#[inline]
pub unsafe fn SHFind_InitMenuPopup(hmenu: super::windef::HMENU, hwndowner: Option<super::windef::HWND>, idcmdfirst: u32, idcmdlast: u32) -> Option<super::shobjidl_core::IContextMenu> {
    windows_core::link!("shell32.dll" "system" fn SHFind_InitMenuPopup(hmenu : super::windef::HMENU, hwndowner : super::windef::HWND, idcmdfirst : u32, idcmdlast : u32) -> Option < super::shobjidl_core::IContextMenu >);
    unsafe { SHFind_InitMenuPopup(hmenu, hwndowner.unwrap_or(core::mem::zeroed()) as _, idcmdfirst, idcmdlast) }
}
#[inline]
pub unsafe fn SHFlushSFCache() {
    windows_core::link!("shell32.dll" "system" fn SHFlushSFCache());
    unsafe { SHFlushSFCache() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHFormatDrive(hwnd: super::windef::HWND, drive: u32, fmtid: u32, options: u32) -> u32 {
    windows_core::link!("shell32.dll" "system" fn SHFormatDrive(hwnd : super::windef::HWND, drive : u32, fmtid : u32, options : u32) -> u32);
    unsafe { SHFormatDrive(hwnd, drive, fmtid, options) }
}
#[inline]
pub unsafe fn SHFree(pv: Option<*const core::ffi::c_void>) {
    windows_core::link!("shell32.dll" "system" fn SHFree(pv : *const core::ffi::c_void));
    unsafe { SHFree(pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn SHGetAttributesFromDataObject<P0>(pdo: P0, dwattributemask: u32, pdwattributes: Option<*mut u32>, pcitems: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IDataObject>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetAttributesFromDataObject(pdo : *mut core::ffi::c_void, dwattributemask : u32, pdwattributes : *mut u32, pcitems : *mut u32) -> windows_core::HRESULT);
    unsafe { SHGetAttributesFromDataObject(pdo.param().abi(), dwattributemask, pdwattributes.unwrap_or(core::mem::zeroed()) as _, pcitems.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHGetDataFromIDListA<P0>(psf: P0, pidl: *const super::shtypes::ITEMIDLIST, nformat: i32, pv: *mut core::ffi::c_void, cb: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetDataFromIDListA(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, nformat : i32, pv : *mut core::ffi::c_void, cb : i32) -> windows_core::HRESULT);
    unsafe { SHGetDataFromIDListA(psf.param().abi(), pidl, nformat, pv as _, cb) }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHGetDataFromIDListW<P0>(psf: P0, pidl: *const super::shtypes::ITEMIDLIST, nformat: i32, pv: *mut core::ffi::c_void, cb: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetDataFromIDListW(psf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, nformat : i32, pv : *mut core::ffi::c_void, cb : i32) -> windows_core::HRESULT);
    unsafe { SHGetDataFromIDListW(psf.param().abi(), pidl, nformat, pv as _, cb) }
}
#[cfg(feature = "shobjidl_core")]
#[inline]
pub unsafe fn SHGetDesktopFolder() -> windows_core::Result<super::shobjidl_core::IShellFolder> {
    windows_core::link!("shell32.dll" "system" fn SHGetDesktopFolder(ppshf : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetDesktopFolder(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shtypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetFolderLocation(hwnd: Option<super::windef::HWND>, csidl: i32, htoken: Option<super::winnt::HANDLE>, dwflags: u32) -> windows_core::Result<super::shtypes::LPITEMIDLIST> {
    windows_core::link!("shell32.dll" "system" fn SHGetFolderLocation(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, ppidl : *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetFolderLocation(hwnd.unwrap_or(core::mem::zeroed()) as _, csidl, htoken.unwrap_or(core::mem::zeroed()) as _, dwflags, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetFolderPathA(hwnd: Option<super::windef::HWND>, csidl: i32, htoken: Option<super::winnt::HANDLE>, dwflags: u32, pszpath: windows_core::PSTR) -> windows_core::HRESULT {
    windows_core::link!("shell32.dll" "system" fn SHGetFolderPathA(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_core::PSTR) -> windows_core::HRESULT);
    unsafe { SHGetFolderPathA(hwnd.unwrap_or(core::mem::zeroed()) as _, csidl, htoken.unwrap_or(core::mem::zeroed()) as _, dwflags, core::mem::transmute(pszpath)) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetFolderPathAndSubDirA<P4>(hwnd: Option<super::windef::HWND>, csidl: i32, htoken: Option<super::winnt::HANDLE>, dwflags: u32, pszsubdir: P4, pszpath: windows_core::PSTR) -> windows_core::HRESULT
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetFolderPathAndSubDirA(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszsubdir : windows_core::PCSTR, pszpath : windows_core::PSTR) -> windows_core::HRESULT);
    unsafe { SHGetFolderPathAndSubDirA(hwnd.unwrap_or(core::mem::zeroed()) as _, csidl, htoken.unwrap_or(core::mem::zeroed()) as _, dwflags, pszsubdir.param().abi(), core::mem::transmute(pszpath)) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetFolderPathAndSubDirW<P4>(hwnd: Option<super::windef::HWND>, csidl: i32, htoken: Option<super::winnt::HANDLE>, dwflags: u32, pszsubdir: P4, pszpath: windows_core::PWSTR) -> windows_core::HRESULT
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetFolderPathAndSubDirW(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszsubdir : windows_core::PCWSTR, pszpath : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { SHGetFolderPathAndSubDirW(hwnd.unwrap_or(core::mem::zeroed()) as _, csidl, htoken.unwrap_or(core::mem::zeroed()) as _, dwflags, pszsubdir.param().abi(), core::mem::transmute(pszpath)) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetFolderPathW(hwnd: Option<super::windef::HWND>, csidl: i32, htoken: Option<super::winnt::HANDLE>, dwflags: u32, pszpath: windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("shell32.dll" "system" fn SHGetFolderPathW(hwnd : super::windef::HWND, csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { SHGetFolderPathW(hwnd.unwrap_or(core::mem::zeroed()) as _, csidl, htoken.unwrap_or(core::mem::zeroed()) as _, dwflags, core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn SHGetIconOverlayIndexA<P0>(psziconpath: P0, iiconindex: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetIconOverlayIndexA(psziconpath : windows_core::PCSTR, iiconindex : i32) -> i32);
    unsafe { SHGetIconOverlayIndexA(psziconpath.param().abi(), iiconindex) }
}
#[inline]
pub unsafe fn SHGetIconOverlayIndexW<P0>(psziconpath: P0, iiconindex: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetIconOverlayIndexW(psziconpath : windows_core::PCWSTR, iiconindex : i32) -> i32);
    unsafe { SHGetIconOverlayIndexW(psziconpath.param().abi(), iiconindex) }
}
#[inline]
pub unsafe fn SHGetInstanceExplorer() -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("shell32.dll" "system" fn SHGetInstanceExplorer(ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetInstanceExplorer(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shtypes", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetKnownFolderIDList(rfid: *const super::shtypes::KNOWNFOLDERID, dwflags: u32, htoken: Option<super::winnt::HANDLE>) -> windows_core::Result<super::shtypes::LPITEMIDLIST> {
    windows_core::link!("shell32.dll" "system" fn SHGetKnownFolderIDList(rfid : *const super::shtypes::KNOWNFOLDERID, dwflags : u32, htoken : super::winnt::HANDLE, ppidl : *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetKnownFolderIDList(rfid, dwflags, htoken.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "shtypes", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetKnownFolderItem<T>(rfid: *const super::shtypes::KNOWNFOLDERID, flags: KNOWN_FOLDER_FLAG, htoken: Option<super::winnt::HANDLE>) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn SHGetKnownFolderItem(rfid : *const super::shtypes::KNOWNFOLDERID, flags : KNOWN_FOLDER_FLAG, htoken : super::winnt::HANDLE, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { SHGetKnownFolderItem(rfid, flags, htoken.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "shtypes", feature = "winnt"))]
#[inline]
pub unsafe fn SHGetKnownFolderPath(rfid: *const super::shtypes::KNOWNFOLDERID, dwflags: u32, htoken: Option<super::winnt::HANDLE>) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("shell32.dll" "system" fn SHGetKnownFolderPath(rfid : *const super::shtypes::KNOWNFOLDERID, dwflags : u32, htoken : super::winnt::HANDLE, ppszpath : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetKnownFolderPath(rfid, dwflags, htoken.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn SHGetMalloc() -> windows_core::Result<super::objidlbase::IMalloc> {
    windows_core::link!("shell32.dll" "system" fn SHGetMalloc(ppmalloc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetMalloc(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHGetPathFromIDListA(pidl: *const super::shtypes::ITEMIDLIST, pszpath: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHGetPathFromIDListA(pidl : *const super::shtypes::ITEMIDLIST, pszpath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SHGetPathFromIDListA(pidl, core::mem::transmute(pszpath)) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHGetPathFromIDListEx(pidl: *const super::shtypes::ITEMIDLIST, pszpath: &mut [u16], uopts: GPFIDL_FLAGS) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHGetPathFromIDListEx(pidl : *const super::shtypes::ITEMIDLIST, pszpath : windows_core::PWSTR, cchpath : u32, uopts : GPFIDL_FLAGS) -> windows_core::BOOL);
    unsafe { SHGetPathFromIDListEx(pidl, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), uopts) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHGetPathFromIDListW(pidl: *const super::shtypes::ITEMIDLIST, pszpath: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHGetPathFromIDListW(pidl : *const super::shtypes::ITEMIDLIST, pszpath : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SHGetPathFromIDListW(pidl, core::mem::transmute(pszpath)) }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHGetRealIDL<P0>(psf: P0, pidlsimple: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<super::shtypes::LPITEMIDLIST>
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetRealIDL(psf : *mut core::ffi::c_void, pidlsimple : *const super::shtypes::ITEMIDLIST, ppidlreal : *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetRealIDL(psf.param().abi(), pidlsimple, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shobjidl_core")]
#[inline]
pub unsafe fn SHGetSetFolderCustomSettings<P1>(pfcs: *mut SHFOLDERCUSTOMSETTINGS, pszpath: P1, dwreadwrite: u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHGetSetFolderCustomSettings(pfcs : *mut SHFOLDERCUSTOMSETTINGS, pszpath : windows_core::PCWSTR, dwreadwrite : u32) -> windows_core::HRESULT);
    unsafe { SHGetSetFolderCustomSettings(pfcs as _, pszpath.param().abi(), dwreadwrite) }
}
#[inline]
pub unsafe fn SHGetSetSettings(lpss: Option<*mut SHELLSTATEA>, dwmask: u32, bset: bool) {
    windows_core::link!("shell32.dll" "system" fn SHGetSetSettings(lpss : *mut SHELLSTATEA, dwmask : u32, bset : windows_core::BOOL));
    unsafe { SHGetSetSettings(lpss.unwrap_or(core::mem::zeroed()) as _, dwmask, bset.into()) }
}
#[inline]
pub unsafe fn SHGetSettings(psfs: *mut SHELLFLAGSTATE, dwmask: u32) {
    windows_core::link!("shell32.dll" "system" fn SHGetSettings(psfs : *mut SHELLFLAGSTATE, dwmask : u32));
    unsafe { SHGetSettings(psfs as _, dwmask) }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[inline]
pub unsafe fn SHGetSpecialFolderLocation(hwnd: Option<super::windef::HWND>, csidl: i32) -> windows_core::Result<super::shtypes::LPITEMIDLIST> {
    windows_core::link!("shell32.dll" "system" fn SHGetSpecialFolderLocation(hwnd : super::windef::HWND, csidl : i32, ppidl : *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetSpecialFolderLocation(hwnd.unwrap_or(core::mem::zeroed()) as _, csidl, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHGetSpecialFolderPathA(hwnd: Option<super::windef::HWND>, pszpath: windows_core::PSTR, csidl: i32, fcreate: bool) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHGetSpecialFolderPathA(hwnd : super::windef::HWND, pszpath : windows_core::PSTR, csidl : i32, fcreate : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SHGetSpecialFolderPathA(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszpath), csidl, fcreate.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHGetSpecialFolderPathW(hwnd: Option<super::windef::HWND>, pszpath: windows_core::PWSTR, csidl: i32, fcreate: bool) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHGetSpecialFolderPathW(hwnd : super::windef::HWND, pszpath : windows_core::PWSTR, csidl : i32, fcreate : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SHGetSpecialFolderPathW(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszpath), csidl, fcreate.into()) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHHandleUpdateImage(pidlextra: *const super::shtypes::ITEMIDLIST) -> i32 {
    windows_core::link!("shell32.dll" "system" fn SHHandleUpdateImage(pidlextra : *const super::shtypes::ITEMIDLIST) -> i32);
    unsafe { SHHandleUpdateImage(pidlextra) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHILCreateFromPath<P0>(pszpath: P0, ppidl: *mut super::shtypes::LPITEMIDLIST, rgfinout: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHILCreateFromPath(pszpath : windows_core::PCWSTR, ppidl : *mut super::shtypes::LPITEMIDLIST, rgfinout : *mut u32) -> windows_core::HRESULT);
    unsafe { SHILCreateFromPath(pszpath.param().abi(), ppidl as _, rgfinout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
#[inline]
pub unsafe fn SHLimitInputEdit<P1>(hwndedit: super::windef::HWND, psf: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn SHLimitInputEdit(hwndedit : super::windef::HWND, psf : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHLimitInputEdit(hwndedit, psf.param().abi()) }
}
#[inline]
pub unsafe fn SHLoadInProc(rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("shell32.dll" "system" fn SHLoadInProc(rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SHLoadInProc(rclsid) }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHMapPIDLToSystemImageListIndex<P0>(pshf: P0, pidl: *const super::shtypes::ITEMIDLIST, piindexsel: Option<*mut i32>) -> i32
where
    P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
{
    windows_core::link!("shell32.dll" "system" fn SHMapPIDLToSystemImageListIndex(pshf : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, piindexsel : *mut i32) -> i32);
    unsafe { SHMapPIDLToSystemImageListIndex(pshf.param().abi(), pidl, piindexsel.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHObjectProperties<P2, P3>(hwnd: Option<super::windef::HWND>, shopobjecttype: u32, pszobjectname: P2, pszpropertypage: P3) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHObjectProperties(hwnd : super::windef::HWND, shopobjecttype : u32, pszobjectname : windows_core::PCWSTR, pszpropertypage : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SHObjectProperties(hwnd.unwrap_or(core::mem::zeroed()) as _, shopobjecttype, pszobjectname.param().abi(), pszpropertypage.param().abi()) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SHOpenFolderAndSelectItems(pidlfolder: *const super::shtypes::ITEMIDLIST, apidl: Option<&[super::shtypes::LPCITEMIDLIST]>, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("shell32.dll" "system" fn SHOpenFolderAndSelectItems(pidlfolder : *const super::shtypes::ITEMIDLIST, cidl : u32, apidl : *const super::shtypes::LPCITEMIDLIST, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SHOpenFolderAndSelectItems(pidlfolder, apidl.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(apidl.map_or(core::ptr::null(), |slice| slice.as_ptr())), dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHOpenWithDialog(hwndparent: Option<super::windef::HWND>, poainfo: *const OPENASINFO) -> windows_core::HRESULT {
    windows_core::link!("shell32.dll" "system" fn SHOpenWithDialog(hwndparent : super::windef::HWND, poainfo : *const OPENASINFO) -> windows_core::HRESULT);
    unsafe { SHOpenWithDialog(hwndparent.unwrap_or(core::mem::zeroed()) as _, poainfo) }
}
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "shtypes"))]
#[inline]
pub unsafe fn SHParseDisplayName<P0, P1>(pszname: P0, pbc: P1, ppidl: *mut super::shtypes::LPITEMIDLIST, sfgaoin: super::shobjidl_core::SFGAOF, psfgaoout: Option<*mut super::shobjidl_core::SFGAOF>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::objidl::IBindCtx>,
{
    windows_core::link!("shell32.dll" "system" fn SHParseDisplayName(pszname : windows_core::PCWSTR, pbc : *mut core::ffi::c_void, ppidl : *mut super::shtypes::LPITEMIDLIST, sfgaoin : super::shobjidl_core::SFGAOF, psfgaoout : *mut super::shobjidl_core::SFGAOF) -> windows_core::HRESULT);
    unsafe { SHParseDisplayName(pszname.param().abi(), pbc.param().abi(), ppidl as _, sfgaoin, psfgaoout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHPathPrepareForWriteA<P1, P2>(hwnd: Option<super::windef::HWND>, punkenablemodless: P1, pszpath: P2, dwflags: u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHPathPrepareForWriteA(hwnd : super::windef::HWND, punkenablemodless : *mut core::ffi::c_void, pszpath : windows_core::PCSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SHPathPrepareForWriteA(hwnd.unwrap_or(core::mem::zeroed()) as _, punkenablemodless.param().abi(), pszpath.param().abi(), dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHPathPrepareForWriteW<P1, P2>(hwnd: Option<super::windef::HWND>, punkenablemodless: P1, pszpath: P2, dwflags: u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHPathPrepareForWriteW(hwnd : super::windef::HWND, punkenablemodless : *mut core::ffi::c_void, pszpath : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SHPathPrepareForWriteW(hwnd.unwrap_or(core::mem::zeroed()) as _, punkenablemodless.param().abi(), pszpath.param().abi(), dwflags) }
}
#[cfg(feature = "propidlbase")]
#[inline]
pub unsafe fn SHPropStgCreate<P0>(psstg: P0, fmtid: *const windows_core::GUID, pclsid: Option<*const windows_core::GUID>, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut Option<super::propidlbase::IPropertyStorage>, pucodepage: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::propidlbase::IPropertySetStorage>,
{
    windows_core::link!("shell32.dll" "system" fn SHPropStgCreate(psstg : *mut core::ffi::c_void, fmtid : *const windows_core::GUID, pclsid : *const windows_core::GUID, grfflags : u32, grfmode : u32, dwdisposition : u32, ppstg : *mut *mut core::ffi::c_void, pucodepage : *mut u32) -> windows_core::HRESULT);
    unsafe { SHPropStgCreate(psstg.param().abi(), fmtid, pclsid.unwrap_or(core::mem::zeroed()) as _, grfflags, grfmode, dwdisposition, core::mem::transmute(ppstg), pucodepage.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SHPropStgReadMultiple<P0>(pps: P0, ucodepage: u32, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::propidlbase::IPropertyStorage>,
{
    windows_core::link!("shell32.dll" "system" fn SHPropStgReadMultiple(pps : *mut core::ffi::c_void, ucodepage : u32, cpspec : u32, rgpspec : *const super::propidlbase::PROPSPEC, rgvar : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { SHPropStgReadMultiple(pps.param().abi(), ucodepage, cpspec, rgpspec, core::mem::transmute(rgvar)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SHPropStgWriteMultiple<P0>(pps: P0, pucodepage: Option<*mut u32>, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgvar: *mut super::propidlbase::PROPVARIANT, propidnamefirst: super::wtypes::PROPID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::propidlbase::IPropertyStorage>,
{
    windows_core::link!("shell32.dll" "system" fn SHPropStgWriteMultiple(pps : *mut core::ffi::c_void, pucodepage : *mut u32, cpspec : u32, rgpspec : *const super::propidlbase::PROPSPEC, rgvar : *mut super::propidlbase::PROPVARIANT, propidnamefirst : super::wtypes::PROPID) -> windows_core::HRESULT);
    unsafe { SHPropStgWriteMultiple(pps.param().abi(), pucodepage.unwrap_or(core::mem::zeroed()) as _, cpspec, rgpspec, core::mem::transmute(rgvar), propidnamefirst) }
}
#[cfg(all(feature = "minwindef", feature = "prsht"))]
#[inline]
pub unsafe fn SHReplaceFromPropSheetExtArray(hpsxa: HPSXA, upageid: u32, lpfnreplacewith: super::prsht::LPFNADDPROPSHEETPAGE, lparam: super::minwindef::LPARAM) -> u32 {
    windows_core::link!("shell32.dll" "system" fn SHReplaceFromPropSheetExtArray(hpsxa : HPSXA, upageid : u32, lpfnreplacewith : super::prsht::LPFNADDPROPSHEETPAGE, lparam : super::minwindef::LPARAM) -> u32);
    unsafe { SHReplaceFromPropSheetExtArray(hpsxa, upageid, lpfnreplacewith, lparam) }
}
#[inline]
pub unsafe fn SHRestricted(rest: RESTRICTIONS) -> u32 {
    windows_core::link!("shell32.dll" "system" fn SHRestricted(rest : RESTRICTIONS) -> u32);
    unsafe { SHRestricted(rest) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SHSetFolderPathA<P3>(csidl: i32, htoken: Option<super::winnt::HANDLE>, dwflags: u32, pszpath: P3) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHSetFolderPathA(csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_core::PCSTR) -> windows_core::HRESULT);
    unsafe { SHSetFolderPathA(csidl, htoken.unwrap_or(core::mem::zeroed()) as _, dwflags, pszpath.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SHSetFolderPathW<P3>(csidl: i32, htoken: Option<super::winnt::HANDLE>, dwflags: u32, pszpath: P3) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHSetFolderPathW(csidl : i32, htoken : super::winnt::HANDLE, dwflags : u32, pszpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SHSetFolderPathW(csidl, htoken.unwrap_or(core::mem::zeroed()) as _, dwflags, pszpath.param().abi()) }
}
#[inline]
pub unsafe fn SHSetInstanceExplorer<P0>(punk: P0)
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("shell32.dll" "system" fn SHSetInstanceExplorer(punk : *mut core::ffi::c_void));
    unsafe { SHSetInstanceExplorer(punk.param().abi()) }
}
#[cfg(all(feature = "shtypes", feature = "winnt"))]
#[inline]
pub unsafe fn SHSetKnownFolderPath<P3>(rfid: *const super::shtypes::KNOWNFOLDERID, dwflags: u32, htoken: Option<super::winnt::HANDLE>, pszpath: P3) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHSetKnownFolderPath(rfid : *const super::shtypes::KNOWNFOLDERID, dwflags : u32, htoken : super::winnt::HANDLE, pszpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SHSetKnownFolderPath(rfid, dwflags, htoken.unwrap_or(core::mem::zeroed()) as _, pszpath.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SHShellFolderView_Message(hwndmain: super::windef::HWND, umsg: u32, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("shell32.dll" "system" fn SHShellFolderView_Message(hwndmain : super::windef::HWND, umsg : u32, lparam : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { SHShellFolderView_Message(hwndmain, umsg, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHStartNetConnectionDialogW<P1>(hwnd: Option<super::windef::HWND>, pszremotename: P1, dwtype: u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHStartNetConnectionDialogW(hwnd : super::windef::HWND, pszremotename : windows_core::PCWSTR, dwtype : u32) -> windows_core::HRESULT);
    unsafe { SHStartNetConnectionDialogW(hwnd.unwrap_or(core::mem::zeroed()) as _, pszremotename.param().abi(), dwtype) }
}
#[inline]
pub unsafe fn SHUpdateImageA<P0>(pszhashitem: P0, iindex: i32, uflags: u32, iimageindex: i32)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHUpdateImageA(pszhashitem : windows_core::PCSTR, iindex : i32, uflags : u32, iimageindex : i32));
    unsafe { SHUpdateImageA(pszhashitem.param().abi(), iindex, uflags, iimageindex) }
}
#[inline]
pub unsafe fn SHUpdateImageW<P0>(pszhashitem: P0, iindex: i32, uflags: u32, iimageindex: i32)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHUpdateImageW(pszhashitem : windows_core::PCWSTR, iindex : i32, uflags : u32, iimageindex : i32));
    unsafe { SHUpdateImageW(pszhashitem.param().abi(), iindex, uflags, iimageindex) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SHValidateUNC(hwndowner: Option<super::windef::HWND>, pszfile: windows_core::PWSTR, fconnect: u32) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHValidateUNC(hwndowner : super::windef::HWND, pszfile : windows_core::PWSTR, fconnect : u32) -> windows_core::BOOL);
    unsafe { SHValidateUNC(hwndowner.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszfile), fconnect) }
}
#[inline]
pub unsafe fn Shell_GetCachedImageIndex<P0>(pwsziconpath: P0, iiconindex: i32, uiconflags: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn Shell_GetCachedImageIndex(pwsziconpath : windows_core::PCWSTR, iiconindex : i32, uiconflags : u32) -> i32);
    unsafe { Shell_GetCachedImageIndex(pwsziconpath.param().abi(), iiconindex, uiconflags) }
}
#[inline]
pub unsafe fn Shell_GetCachedImageIndexA<P0>(psziconpath: P0, iiconindex: i32, uiconflags: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn Shell_GetCachedImageIndexA(psziconpath : windows_core::PCSTR, iiconindex : i32, uiconflags : u32) -> i32);
    unsafe { Shell_GetCachedImageIndexA(psziconpath.param().abi(), iiconindex, uiconflags) }
}
#[inline]
pub unsafe fn Shell_GetCachedImageIndexW<P0>(psziconpath: P0, iiconindex: i32, uiconflags: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn Shell_GetCachedImageIndexW(psziconpath : windows_core::PCWSTR, iiconindex : i32, uiconflags : u32) -> i32);
    unsafe { Shell_GetCachedImageIndexW(psziconpath.param().abi(), iiconindex, uiconflags) }
}
#[cfg(feature = "commctrl")]
#[inline]
pub unsafe fn Shell_GetImageLists(phiml: Option<*mut super::commctrl::HIMAGELIST>, phimlsmall: Option<*mut super::commctrl::HIMAGELIST>) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn Shell_GetImageLists(phiml : *mut super::commctrl::HIMAGELIST, phimlsmall : *mut super::commctrl::HIMAGELIST) -> windows_core::BOOL);
    unsafe { Shell_GetImageLists(phiml.unwrap_or(core::mem::zeroed()) as _, phimlsmall.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn Shell_MergeMenus(hmdst: super::windef::HMENU, hmsrc: super::windef::HMENU, uinsert: u32, uidadjust: u32, uidadjustmax: u32, uflags: u32) -> u32 {
    windows_core::link!("shell32.dll" "system" fn Shell_MergeMenus(hmdst : super::windef::HMENU, hmsrc : super::windef::HMENU, uinsert : u32, uidadjust : u32, uidadjustmax : u32, uflags : u32) -> u32);
    unsafe { Shell_MergeMenus(hmdst, hmsrc, uinsert, uidadjust, uidadjustmax, uflags) }
}
#[cfg(feature = "shtypes")]
#[inline]
pub unsafe fn SignalFileOpen(pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SignalFileOpen(pidl : *const super::shtypes::ITEMIDLIST) -> windows_core::BOOL);
    unsafe { SignalFileOpen(pidl) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgMakeUniqueName<P0, P1, T>(pstgparent: P0, pszfilespec: P1, grfmode: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::objidl::IStorage>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("shell32.dll" "system" fn StgMakeUniqueName(pstgparent : *mut core::ffi::c_void, pszfilespec : windows_core::PCWSTR, grfmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { StgMakeUniqueName(pstgparent.param().abi(), pszfilespec.param().abi(), grfmode, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn Win32DeleteFile<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn Win32DeleteFile(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { Win32DeleteFile(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn WriteCabinetState(pcs: *const CABINETSTATE) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn WriteCabinetState(pcs : *const CABINETSTATE) -> windows_core::BOOL);
    unsafe { WriteCabinetState(pcs) }
}
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
    pub bFull: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default)]
pub struct BROWSEINFOA {
    pub hwndOwner: super::windef::HWND,
    pub pidlRoot: super::shtypes::LPCITEMIDLIST,
    pub pszDisplayName: windows_core::PSTR,
    pub lpszTitle: windows_core::PCSTR,
    pub ulFlags: u32,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::minwindef::LPARAM,
    pub iImage: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct BROWSEINFOW {
    pub hwndOwner: super::windef::HWND,
    pub pidlRoot: super::shtypes::LPCITEMIDLIST,
    pub pszDisplayName: windows_core::PWSTR,
    pub lpszTitle: windows_core::PCWSTR,
    pub ulFlags: u32,
    pub lpfn: BFFCALLBACK,
    pub lParam: super::minwindef::LPARAM,
    pub iImage: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CABINETSTATE {
    pub cLength: u16,
    pub nVersion: u16,
    pub _bitfield: windows_core::BOOL,
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
    pub fChecked: windows_core::BOOL,
    pub fDirty: windows_core::BOOL,
    pub fNoScroll: windows_core::BOOL,
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
    pub fEnableComponents: windows_core::BOOL,
    pub fActiveDesktop: windows_core::BOOL,
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
    pub fCanResize: windows_core::BOOL,
    pub fCanResizeX: windows_core::BOOL,
    pub fCanResizeY: windows_core::BOOL,
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
#[derive(Clone, Debug, Default)]
pub struct CSFV {
    pub cbSize: u32,
    pub pshf: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub psvOuter: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellView>>,
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub lEvents: i32,
    pub pfnCallback: LPFNVIEWCALLBACK,
    pub fvm: super::shobjidl_core::FOLDERVIEWMODE,
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
#[derive(Clone, Debug, PartialEq)]
pub struct DEFCONTEXTMENU {
    pub hwnd: super::windef::HWND,
    pub pcmcb: core::mem::ManuallyDrop<Option<super::shobjidl_core::IContextMenuCB>>,
    pub pidlFolder: super::shtypes::LPCITEMIDLIST,
    pub psf: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub cidl: u32,
    pub apidl: *mut super::shtypes::LPCITEMIDLIST,
    pub punkAssociationInfo: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DFMICS {
    pub cbSize: u32,
    pub fMask: u32,
    pub lParam: super::minwindef::LPARAM,
    pub idCmdFirst: u32,
    pub idDefMax: u32,
    pub pici: super::shobjidl_core::LPCMINVOKECOMMANDINFO,
    pub punkSite: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
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
    pub fNC: windows_core::BOOL,
    pub fWide: windows_core::BOOL,
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
    pub clsid: windows_core::GUID,
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
    pub clsid: windows_core::GUID,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GPFIDL_FLAGS(pub i32);
pub const GPFIDL_UNCPRINTER: i32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPSXA(pub *mut core::ffi::c_void);
impl Default for HPSXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IACList, IACList_Vtbl, 0x77a130b0_94fd_11d0_a544_00c04fd7d062);
windows_core::imp::interface_hierarchy!(IACList, windows_core::IUnknown);
impl IACList {
    pub unsafe fn Expand<P0>(&self, pszexpand: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), pszexpand.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IACList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IACList_Impl: windows_core::IUnknownImpl {
    fn Expand(&self, pszexpand: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IACList_Vtbl {
    pub const fn new<Identity: IACList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Expand<Identity: IACList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszexpand: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IACList_Impl::Expand(this, core::mem::transmute(&pszexpand)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Expand: Expand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IACList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IACList {}
windows_core::imp::define_interface!(IACList2, IACList2_Vtbl, 0x470141a0_5186_11d2_bbb6_0060977b464c);
impl core::ops::Deref for IACList2 {
    type Target = IACList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IACList2, windows_core::IUnknown, IACList);
impl IACList2 {
    pub unsafe fn SetOptions(&self, dwflag: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), dwflag) }
    }
    pub unsafe fn GetOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IACList2_Vtbl {
    pub base__: IACList_Vtbl,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IACList2_Impl: IACList_Impl {
    fn SetOptions(&self, dwflag: u32) -> windows_core::Result<()>;
    fn GetOptions(&self) -> windows_core::Result<u32>;
}
impl IACList2_Vtbl {
    pub const fn new<Identity: IACList2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOptions<Identity: IACList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IACList2_Impl::SetOptions(this, core::mem::transmute_copy(&dwflag)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IACList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflag: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IACList2_Impl::GetOptions(this) {
                    Ok(ok__) => {
                        pdwflag.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IACList_Vtbl::new::<Identity, OFFSET>(), SetOptions: SetOptions::<Identity, OFFSET>, GetOptions: GetOptions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IACList2 as windows_core::Interface>::IID || iid == &<IACList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IACList2 {}
windows_core::imp::define_interface!(IActiveDesktop, IActiveDesktop_Vtbl, 0xf490eb00_1240_11d1_9888_006097deacf9);
windows_core::imp::interface_hierarchy!(IActiveDesktop, windows_core::IUnknown);
impl IActiveDesktop {
    pub unsafe fn ApplyChanges(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ApplyChanges)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn GetWallpaper(&self, pwszwallpaper: &mut [u16], dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWallpaper)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszwallpaper.as_ptr()), pwszwallpaper.len().try_into().unwrap(), dwflags) }
    }
    pub unsafe fn SetWallpaper<P0>(&self, pwszwallpaper: P0, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWallpaper)(windows_core::Interface::as_raw(self), pwszwallpaper.param().abi(), dwreserved) }
    }
    pub unsafe fn GetWallpaperOptions(&self, pwpo: *mut WALLPAPEROPT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWallpaperOptions)(windows_core::Interface::as_raw(self), pwpo as _, dwreserved) }
    }
    pub unsafe fn SetWallpaperOptions(&self, pwpo: *const WALLPAPEROPT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWallpaperOptions)(windows_core::Interface::as_raw(self), pwpo, dwreserved) }
    }
    pub unsafe fn GetPattern(&self, pwszpattern: &mut [u16], dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPattern)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszpattern.as_ptr()), pwszpattern.len().try_into().unwrap(), dwreserved) }
    }
    pub unsafe fn SetPattern<P0>(&self, pwszpattern: P0, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPattern)(windows_core::Interface::as_raw(self), pwszpattern.param().abi(), dwreserved) }
    }
    pub unsafe fn GetDesktopItemOptions(&self, pco: *mut COMPONENTSOPT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesktopItemOptions)(windows_core::Interface::as_raw(self), pco as _, dwreserved) }
    }
    pub unsafe fn SetDesktopItemOptions(&self, pco: *const COMPONENTSOPT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDesktopItemOptions)(windows_core::Interface::as_raw(self), pco, dwreserved) }
    }
    pub unsafe fn AddDesktopItem(&self, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDesktopItem)(windows_core::Interface::as_raw(self), pcomp, dwreserved) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddDesktopItemWithUI(&self, hwnd: Option<super::windef::HWND>, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDesktopItemWithUI)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, pcomp, dwreserved) }
    }
    pub unsafe fn ModifyDesktopItem(&self, pcomp: *mut COMPONENT, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ModifyDesktopItem)(windows_core::Interface::as_raw(self), pcomp as _, dwflags) }
    }
    pub unsafe fn RemoveDesktopItem(&self, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveDesktopItem)(windows_core::Interface::as_raw(self), pcomp, dwreserved) }
    }
    pub unsafe fn GetDesktopItemCount(&self, pcitems: *mut i32, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesktopItemCount)(windows_core::Interface::as_raw(self), pcitems as _, dwreserved) }
    }
    pub unsafe fn GetDesktopItem(&self, ncomponent: i32, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesktopItem)(windows_core::Interface::as_raw(self), ncomponent, pcomp as _, dwreserved) }
    }
    pub unsafe fn GetDesktopItemByID(&self, dwid: usize, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesktopItemByID)(windows_core::Interface::as_raw(self), dwid, pcomp as _, dwreserved) }
    }
    pub unsafe fn GenerateDesktopItemHtml<P0>(&self, pwszfilename: P0, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GenerateDesktopItemHtml)(windows_core::Interface::as_raw(self), pwszfilename.param().abi(), pcomp, dwreserved) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddUrl<P1>(&self, hwnd: Option<super::windef::HWND>, pszsource: P1, pcomp: *const COMPONENT, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddUrl)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, pszsource.param().abi(), pcomp, dwflags) }
    }
    pub unsafe fn GetDesktopItemBySource<P0>(&self, pwszsource: P0, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDesktopItemBySource)(windows_core::Interface::as_raw(self), pwszsource.param().abi(), pcomp as _, dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ApplyChanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetWallpaper: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, u32) -> windows_core::HRESULT,
    pub SetWallpaper: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetWallpaperOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WALLPAPEROPT, u32) -> windows_core::HRESULT,
    pub SetWallpaperOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const WALLPAPEROPT, u32) -> windows_core::HRESULT,
    pub GetPattern: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, u32) -> windows_core::HRESULT,
    pub SetPattern: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetDesktopItemOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COMPONENTSOPT, u32) -> windows_core::HRESULT,
    pub SetDesktopItemOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMPONENTSOPT, u32) -> windows_core::HRESULT,
    pub AddDesktopItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMPONENT, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddDesktopItemWithUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const COMPONENT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddDesktopItemWithUI: usize,
    pub ModifyDesktopItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COMPONENT, u32) -> windows_core::HRESULT,
    pub RemoveDesktopItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMPONENT, u32) -> windows_core::HRESULT,
    pub GetDesktopItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32) -> windows_core::HRESULT,
    pub GetDesktopItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut COMPONENT, u32) -> windows_core::HRESULT,
    pub GetDesktopItemByID: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut COMPONENT, u32) -> windows_core::HRESULT,
    pub GenerateDesktopItemHtml: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const COMPONENT, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddUrl: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, windows_core::PCWSTR, *const COMPONENT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddUrl: usize,
    pub GetDesktopItemBySource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut COMPONENT, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IActiveDesktop_Impl: windows_core::IUnknownImpl {
    fn ApplyChanges(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetWallpaper(&self, pwszwallpaper: windows_core::PWSTR, cchwallpaper: u32, dwflags: u32) -> windows_core::Result<()>;
    fn SetWallpaper(&self, pwszwallpaper: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
    fn GetWallpaperOptions(&self, pwpo: *mut WALLPAPEROPT, dwreserved: u32) -> windows_core::Result<()>;
    fn SetWallpaperOptions(&self, pwpo: *const WALLPAPEROPT, dwreserved: u32) -> windows_core::Result<()>;
    fn GetPattern(&self, pwszpattern: windows_core::PWSTR, cchpattern: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn SetPattern(&self, pwszpattern: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
    fn GetDesktopItemOptions(&self, pco: *mut COMPONENTSOPT, dwreserved: u32) -> windows_core::Result<()>;
    fn SetDesktopItemOptions(&self, pco: *const COMPONENTSOPT, dwreserved: u32) -> windows_core::Result<()>;
    fn AddDesktopItem(&self, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::Result<()>;
    fn AddDesktopItemWithUI(&self, hwnd: super::windef::HWND, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::Result<()>;
    fn ModifyDesktopItem(&self, pcomp: *mut COMPONENT, dwflags: u32) -> windows_core::Result<()>;
    fn RemoveDesktopItem(&self, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::Result<()>;
    fn GetDesktopItemCount(&self, pcitems: *mut i32, dwreserved: u32) -> windows_core::Result<()>;
    fn GetDesktopItem(&self, ncomponent: i32, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::Result<()>;
    fn GetDesktopItemByID(&self, dwid: usize, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::Result<()>;
    fn GenerateDesktopItemHtml(&self, pwszfilename: &windows_core::PCWSTR, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::Result<()>;
    fn AddUrl(&self, hwnd: super::windef::HWND, pszsource: &windows_core::PCWSTR, pcomp: *const COMPONENT, dwflags: u32) -> windows_core::Result<()>;
    fn GetDesktopItemBySource(&self, pwszsource: &windows_core::PCWSTR, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IActiveDesktop_Vtbl {
    pub const fn new<Identity: IActiveDesktop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplyChanges<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::ApplyChanges(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetWallpaper<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszwallpaper: windows_core::PWSTR, cchwallpaper: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetWallpaper(this, core::mem::transmute_copy(&pwszwallpaper), core::mem::transmute_copy(&cchwallpaper), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetWallpaper<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszwallpaper: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::SetWallpaper(this, core::mem::transmute(&pwszwallpaper), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetWallpaperOptions<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwpo: *mut WALLPAPEROPT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetWallpaperOptions(this, core::mem::transmute_copy(&pwpo), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn SetWallpaperOptions<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwpo: *const WALLPAPEROPT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::SetWallpaperOptions(this, core::mem::transmute_copy(&pwpo), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetPattern<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpattern: windows_core::PWSTR, cchpattern: u32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetPattern(this, core::mem::transmute_copy(&pwszpattern), core::mem::transmute_copy(&cchpattern), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn SetPattern<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpattern: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::SetPattern(this, core::mem::transmute(&pwszpattern), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetDesktopItemOptions<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pco: *mut COMPONENTSOPT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetDesktopItemOptions(this, core::mem::transmute_copy(&pco), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn SetDesktopItemOptions<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pco: *const COMPONENTSOPT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::SetDesktopItemOptions(this, core::mem::transmute_copy(&pco), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn AddDesktopItem<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::AddDesktopItem(this, core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn AddDesktopItemWithUI<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::AddDesktopItemWithUI(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn ModifyDesktopItem<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomp: *mut COMPONENT, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::ModifyDesktopItem(this, core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RemoveDesktopItem<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::RemoveDesktopItem(this, core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetDesktopItemCount<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcitems: *mut i32, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetDesktopItemCount(this, core::mem::transmute_copy(&pcitems), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetDesktopItem<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncomponent: i32, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetDesktopItem(this, core::mem::transmute_copy(&ncomponent), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetDesktopItemByID<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: usize, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetDesktopItemByID(this, core::mem::transmute_copy(&dwid), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GenerateDesktopItemHtml<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR, pcomp: *const COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GenerateDesktopItemHtml(this, core::mem::transmute(&pwszfilename), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn AddUrl<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, pszsource: windows_core::PCWSTR, pcomp: *const COMPONENT, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::AddUrl(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&pszsource), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetDesktopItemBySource<Identity: IActiveDesktop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsource: windows_core::PCWSTR, pcomp: *mut COMPONENT, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktop_Impl::GetDesktopItemBySource(this, core::mem::transmute(&pwszsource), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplyChanges: ApplyChanges::<Identity, OFFSET>,
            GetWallpaper: GetWallpaper::<Identity, OFFSET>,
            SetWallpaper: SetWallpaper::<Identity, OFFSET>,
            GetWallpaperOptions: GetWallpaperOptions::<Identity, OFFSET>,
            SetWallpaperOptions: SetWallpaperOptions::<Identity, OFFSET>,
            GetPattern: GetPattern::<Identity, OFFSET>,
            SetPattern: SetPattern::<Identity, OFFSET>,
            GetDesktopItemOptions: GetDesktopItemOptions::<Identity, OFFSET>,
            SetDesktopItemOptions: SetDesktopItemOptions::<Identity, OFFSET>,
            AddDesktopItem: AddDesktopItem::<Identity, OFFSET>,
            AddDesktopItemWithUI: AddDesktopItemWithUI::<Identity, OFFSET>,
            ModifyDesktopItem: ModifyDesktopItem::<Identity, OFFSET>,
            RemoveDesktopItem: RemoveDesktopItem::<Identity, OFFSET>,
            GetDesktopItemCount: GetDesktopItemCount::<Identity, OFFSET>,
            GetDesktopItem: GetDesktopItem::<Identity, OFFSET>,
            GetDesktopItemByID: GetDesktopItemByID::<Identity, OFFSET>,
            GenerateDesktopItemHtml: GenerateDesktopItemHtml::<Identity, OFFSET>,
            AddUrl: AddUrl::<Identity, OFFSET>,
            GetDesktopItemBySource: GetDesktopItemBySource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveDesktop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IActiveDesktop {}
pub const IDO_SHGIOI_DEFAULT: u32 = 4294967292;
pub const IDO_SHGIOI_LINK: u32 = 268435454;
pub const IDO_SHGIOI_SHARE: u32 = 268435455;
pub const IDO_SHGIOI_SLOWFILE: u32 = 4294967293;
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IDockingWindowSite, IDockingWindowSite_Vtbl, 0x2a342fc2_7b26_11d0_8ca9_00a0c92dbfe8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IDockingWindowSite {
    type Target = super::oleidl::IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IDockingWindowSite, windows_core::IUnknown, super::oleidl::IOleWindow);
#[cfg(feature = "oleidl")]
impl IDockingWindowSite {
    #[cfg(feature = "windef")]
    pub unsafe fn GetBorderDW<P0>(&self, punkobj: P0) -> windows_core::Result<super::windef::RECT>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBorderDW)(windows_core::Interface::as_raw(self), punkobj.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn RequestBorderSpaceDW<P0>(&self, punkobj: P0, pbw: super::oleidl::LPCBORDERWIDTHS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestBorderSpaceDW)(windows_core::Interface::as_raw(self), punkobj.param().abi(), pbw) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetBorderSpaceDW<P0>(&self, punkobj: P0, pbw: super::oleidl::LPCBORDERWIDTHS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBorderSpaceDW)(windows_core::Interface::as_raw(self), punkobj.param().abi(), pbw) }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDockingWindowSite_Vtbl {
    pub base__: super::oleidl::IOleWindow_Vtbl,
    #[cfg(feature = "windef")]
    pub GetBorderDW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetBorderDW: usize,
    #[cfg(feature = "windef")]
    pub RequestBorderSpaceDW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oleidl::LPCBORDERWIDTHS) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RequestBorderSpaceDW: usize,
    #[cfg(feature = "windef")]
    pub SetBorderSpaceDW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oleidl::LPCBORDERWIDTHS) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetBorderSpaceDW: usize,
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
pub trait IDockingWindowSite_Impl: super::oleidl::IOleWindow_Impl {
    fn GetBorderDW(&self, punkobj: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::windef::RECT>;
    fn RequestBorderSpaceDW(&self, punkobj: windows_core::Ref<windows_core::IUnknown>, pbw: super::oleidl::LPCBORDERWIDTHS) -> windows_core::Result<()>;
    fn SetBorderSpaceDW(&self, punkobj: windows_core::Ref<windows_core::IUnknown>, pbw: super::oleidl::LPCBORDERWIDTHS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl IDockingWindowSite_Vtbl {
    pub const fn new<Identity: IDockingWindowSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBorderDW<Identity: IDockingWindowSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkobj: *mut core::ffi::c_void, prcborder: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDockingWindowSite_Impl::GetBorderDW(this, core::mem::transmute_copy(&punkobj)) {
                    Ok(ok__) => {
                        prcborder.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestBorderSpaceDW<Identity: IDockingWindowSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkobj: *mut core::ffi::c_void, pbw: super::oleidl::LPCBORDERWIDTHS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowSite_Impl::RequestBorderSpaceDW(this, core::mem::transmute_copy(&punkobj), core::mem::transmute_copy(&pbw)).into()
            }
        }
        unsafe extern "system" fn SetBorderSpaceDW<Identity: IDockingWindowSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkobj: *mut core::ffi::c_void, pbw: super::oleidl::LPCBORDERWIDTHS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowSite_Impl::SetBorderSpaceDW(this, core::mem::transmute_copy(&punkobj), core::mem::transmute_copy(&pbw)).into()
            }
        }
        Self {
            base__: super::oleidl::IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            GetBorderDW: GetBorderDW::<Identity, OFFSET>,
            RequestBorderSpaceDW: RequestBorderSpaceDW::<Identity, OFFSET>,
            SetBorderSpaceDW: SetBorderSpaceDW::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDockingWindowSite as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl windows_core::RuntimeName for IDockingWindowSite {}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IE4COMPONENT {
    pub dwSize: u32,
    pub dwID: u32,
    pub iComponentType: i32,
    pub fChecked: windows_core::BOOL,
    pub fDirty: windows_core::BOOL,
    pub fNoScroll: windows_core::BOOL,
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
windows_core::imp::define_interface!(IExtractIconA, IExtractIconA_Vtbl, 0x000214eb_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IExtractIconA, windows_core::IUnknown);
impl IExtractIconA {
    pub unsafe fn GetIconLocation(&self, uflags: u32, psziconfile: &mut [u8], piindex: *mut i32, pwflags: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIconLocation)(windows_core::Interface::as_raw(self), uflags, core::mem::transmute(psziconfile.as_ptr()), psziconfile.len().try_into().unwrap(), piindex as _, pwflags as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Extract<P0>(&self, pszfile: P0, niconindex: u32, phiconlarge: Option<*mut super::windef::HICON>, phiconsmall: Option<*mut super::windef::HICON>, niconsize: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Extract)(windows_core::Interface::as_raw(self), pszfile.param().abi(), niconindex, phiconlarge.unwrap_or(core::mem::zeroed()) as _, phiconsmall.unwrap_or(core::mem::zeroed()) as _, niconsize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtractIconA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIconLocation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PSTR, u32, *mut i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Extract: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut super::windef::HICON, *mut super::windef::HICON, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Extract: usize,
}
#[cfg(feature = "windef")]
pub trait IExtractIconA_Impl: windows_core::IUnknownImpl {
    fn GetIconLocation(&self, uflags: u32, psziconfile: windows_core::PSTR, cchmax: u32, piindex: *mut i32, pwflags: *mut u32) -> windows_core::Result<()>;
    fn Extract(&self, pszfile: &windows_core::PCSTR, niconindex: u32, phiconlarge: *mut super::windef::HICON, phiconsmall: *mut super::windef::HICON, niconsize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IExtractIconA_Vtbl {
    pub const fn new<Identity: IExtractIconA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIconLocation<Identity: IExtractIconA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, psziconfile: windows_core::PSTR, cchmax: u32, piindex: *mut i32, pwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtractIconA_Impl::GetIconLocation(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&psziconfile), core::mem::transmute_copy(&cchmax), core::mem::transmute_copy(&piindex), core::mem::transmute_copy(&pwflags)).into()
            }
        }
        unsafe extern "system" fn Extract<Identity: IExtractIconA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCSTR, niconindex: u32, phiconlarge: *mut super::windef::HICON, phiconsmall: *mut super::windef::HICON, niconsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtractIconA_Impl::Extract(this, core::mem::transmute(&pszfile), core::mem::transmute_copy(&niconindex), core::mem::transmute_copy(&phiconlarge), core::mem::transmute_copy(&phiconsmall), core::mem::transmute_copy(&niconsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIconLocation: GetIconLocation::<Identity, OFFSET>,
            Extract: Extract::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtractIconA as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IExtractIconA {}
windows_core::imp::define_interface!(IExtractIconW, IExtractIconW_Vtbl, 0x000214fa_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IExtractIconW, windows_core::IUnknown);
impl IExtractIconW {
    pub unsafe fn GetIconLocation(&self, uflags: u32, psziconfile: &mut [u16], piindex: *mut i32, pwflags: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIconLocation)(windows_core::Interface::as_raw(self), uflags, core::mem::transmute(psziconfile.as_ptr()), psziconfile.len().try_into().unwrap(), piindex as _, pwflags as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Extract<P0>(&self, pszfile: P0, niconindex: u32, phiconlarge: Option<*mut super::windef::HICON>, phiconsmall: Option<*mut super::windef::HICON>, niconsize: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Extract)(windows_core::Interface::as_raw(self), pszfile.param().abi(), niconindex, phiconlarge.unwrap_or(core::mem::zeroed()) as _, phiconsmall.unwrap_or(core::mem::zeroed()) as _, niconsize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtractIconW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIconLocation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Extract: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut super::windef::HICON, *mut super::windef::HICON, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Extract: usize,
}
#[cfg(feature = "windef")]
pub trait IExtractIconW_Impl: windows_core::IUnknownImpl {
    fn GetIconLocation(&self, uflags: u32, psziconfile: windows_core::PWSTR, cchmax: u32, piindex: *mut i32, pwflags: *mut u32) -> windows_core::Result<()>;
    fn Extract(&self, pszfile: &windows_core::PCWSTR, niconindex: u32, phiconlarge: *mut super::windef::HICON, phiconsmall: *mut super::windef::HICON, niconsize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IExtractIconW_Vtbl {
    pub const fn new<Identity: IExtractIconW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIconLocation<Identity: IExtractIconW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, psziconfile: windows_core::PWSTR, cchmax: u32, piindex: *mut i32, pwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtractIconW_Impl::GetIconLocation(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&psziconfile), core::mem::transmute_copy(&cchmax), core::mem::transmute_copy(&piindex), core::mem::transmute_copy(&pwflags)).into()
            }
        }
        unsafe extern "system" fn Extract<Identity: IExtractIconW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, niconindex: u32, phiconlarge: *mut super::windef::HICON, phiconsmall: *mut super::windef::HICON, niconsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtractIconW_Impl::Extract(this, core::mem::transmute(&pszfile), core::mem::transmute_copy(&niconindex), core::mem::transmute_copy(&phiconlarge), core::mem::transmute_copy(&phiconsmall), core::mem::transmute_copy(&niconsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIconLocation: GetIconLocation::<Identity, OFFSET>,
            Extract: Extract::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtractIconW as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IExtractIconW {}
windows_core::imp::define_interface!(INamedPropertyBag, INamedPropertyBag_Vtbl, 0xfb700430_952c_11d1_946f_000000000000);
windows_core::imp::interface_hierarchy!(INamedPropertyBag, windows_core::IUnknown);
impl INamedPropertyBag {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReadPropertyNPB<P0, P1>(&self, pszbagname: P0, pszpropname: P1, pvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReadPropertyNPB)(windows_core::Interface::as_raw(self), pszbagname.param().abi(), pszpropname.param().abi(), core::mem::transmute(pvar)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn WritePropertyNPB<P0, P1>(&self, pszbagname: P0, pszpropname: P1, pvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyNPB)(windows_core::Interface::as_raw(self), pszbagname.param().abi(), pszpropname.param().abi(), core::mem::transmute(pvar)) }
    }
    pub unsafe fn RemovePropertyNPB<P0, P1>(&self, pszbagname: P0, pszpropname: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemovePropertyNPB)(windows_core::Interface::as_raw(self), pszbagname.param().abi(), pszpropname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPropertyBag_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub ReadPropertyNPB: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    ReadPropertyNPB: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub WritePropertyNPB: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    WritePropertyNPB: usize,
    pub RemovePropertyNPB: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait INamedPropertyBag_Impl: windows_core::IUnknownImpl {
    fn ReadPropertyNPB(&self, pszbagname: &windows_core::PCWSTR, pszpropname: &windows_core::PCWSTR, pvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn WritePropertyNPB(&self, pszbagname: &windows_core::PCWSTR, pszpropname: &windows_core::PCWSTR, pvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn RemovePropertyNPB(&self, pszbagname: &windows_core::PCWSTR, pszpropname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl INamedPropertyBag_Vtbl {
    pub const fn new<Identity: INamedPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadPropertyNPB<Identity: INamedPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbagname: windows_core::PCWSTR, pszpropname: windows_core::PCWSTR, pvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INamedPropertyBag_Impl::ReadPropertyNPB(this, core::mem::transmute(&pszbagname), core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar)).into()
            }
        }
        unsafe extern "system" fn WritePropertyNPB<Identity: INamedPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbagname: windows_core::PCWSTR, pszpropname: windows_core::PCWSTR, pvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INamedPropertyBag_Impl::WritePropertyNPB(this, core::mem::transmute(&pszbagname), core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar)).into()
            }
        }
        unsafe extern "system" fn RemovePropertyNPB<Identity: INamedPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbagname: windows_core::PCWSTR, pszpropname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INamedPropertyBag_Impl::RemovePropertyNPB(this, core::mem::transmute(&pszbagname), core::mem::transmute(&pszpropname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadPropertyNPB: ReadPropertyNPB::<Identity, OFFSET>,
            WritePropertyNPB: WritePropertyNPB::<Identity, OFFSET>,
            RemovePropertyNPB: RemovePropertyNPB::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INamedPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INamedPropertyBag {}
windows_core::imp::define_interface!(IObjMgr, IObjMgr_Vtbl, 0x00bb2761_6a77_11d0_a535_00c04fd7d062);
windows_core::imp::interface_hierarchy!(IObjMgr, windows_core::IUnknown);
impl IObjMgr {
    pub unsafe fn Append<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn Remove<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IObjMgr_Impl: windows_core::IUnknownImpl {
    fn Append(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Remove(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IObjMgr_Vtbl {
    pub const fn new<Identity: IObjMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Append<Identity: IObjMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjMgr_Impl::Append(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IObjMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjMgr_Impl::Remove(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Append: Append::<Identity, OFFSET>, Remove: Remove::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjMgr {}
windows_core::imp::define_interface!(IProgressDialog, IProgressDialog_Vtbl, 0xebbc7c04_315e_11d2_b62f_006097df5bd4);
windows_core::imp::interface_hierarchy!(IProgressDialog, windows_core::IUnknown);
impl IProgressDialog {
    #[cfg(feature = "windef")]
    pub unsafe fn StartProgressDialog<P1>(&self, hwndparent: Option<super::windef::HWND>, punkenablemodless: P1, dwflags: u32, pvresevered: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartProgressDialog)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, punkenablemodless.param().abi(), dwflags, pvresevered.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn StopProgressDialog(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopProgressDialog)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTitle<P0>(&self, pwztitle: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), pwztitle.param().abi()) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn SetAnimation(&self, hinstanimation: Option<super::minwindef::HINSTANCE>, idanimation: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAnimation)(windows_core::Interface::as_raw(self), hinstanimation.unwrap_or(core::mem::zeroed()) as _, idanimation) }
    }
    pub unsafe fn HasUserCancelled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasUserCancelled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetProgress(&self, dwcompleted: u32, dwtotal: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProgress)(windows_core::Interface::as_raw(self), dwcompleted, dwtotal) }
    }
    pub unsafe fn SetProgress64(&self, ullcompleted: u64, ulltotal: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProgress64)(windows_core::Interface::as_raw(self), ullcompleted, ulltotal) }
    }
    pub unsafe fn SetLine<P1>(&self, dwlinenum: u32, pwzstring: P1, fcompactpath: bool, pvresevered: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLine)(windows_core::Interface::as_raw(self), dwlinenum, pwzstring.param().abi(), fcompactpath.into(), pvresevered.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetCancelMsg<P0>(&self, pwzcancelmsg: P0, pvresevered: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCancelMsg)(windows_core::Interface::as_raw(self), pwzcancelmsg.param().abi(), pvresevered.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Timer(&self, dwtimeraction: u32, pvresevered: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Timer)(windows_core::Interface::as_raw(self), dwtimeraction, pvresevered.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressDialog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub StartProgressDialog: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    StartProgressDialog: usize,
    pub StopProgressDialog: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub SetAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HINSTANCE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    SetAnimation: usize,
    pub HasUserCancelled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetProgress64: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
    pub SetLine: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::BOOL, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCancelMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Timer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub trait IProgressDialog_Impl: windows_core::IUnknownImpl {
    fn StartProgressDialog(&self, hwndparent: super::windef::HWND, punkenablemodless: windows_core::Ref<windows_core::IUnknown>, dwflags: u32, pvresevered: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn StopProgressDialog(&self) -> windows_core::Result<()>;
    fn SetTitle(&self, pwztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetAnimation(&self, hinstanimation: super::minwindef::HINSTANCE, idanimation: u32) -> windows_core::Result<()>;
    fn HasUserCancelled(&self) -> windows_core::BOOL;
    fn SetProgress(&self, dwcompleted: u32, dwtotal: u32) -> windows_core::Result<()>;
    fn SetProgress64(&self, ullcompleted: u64, ulltotal: u64) -> windows_core::Result<()>;
    fn SetLine(&self, dwlinenum: u32, pwzstring: &windows_core::PCWSTR, fcompactpath: windows_core::BOOL, pvresevered: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetCancelMsg(&self, pwzcancelmsg: &windows_core::PCWSTR, pvresevered: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Timer(&self, dwtimeraction: u32, pvresevered: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl IProgressDialog_Vtbl {
    pub const fn new<Identity: IProgressDialog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartProgressDialog<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, punkenablemodless: *mut core::ffi::c_void, dwflags: u32, pvresevered: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::StartProgressDialog(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&punkenablemodless), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pvresevered)).into()
            }
        }
        unsafe extern "system" fn StopProgressDialog<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::StopProgressDialog(this).into()
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::SetTitle(this, core::mem::transmute(&pwztitle)).into()
            }
        }
        unsafe extern "system" fn SetAnimation<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstanimation: super::minwindef::HINSTANCE, idanimation: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::SetAnimation(this, core::mem::transmute_copy(&hinstanimation), core::mem::transmute_copy(&idanimation)).into()
            }
        }
        unsafe extern "system" fn HasUserCancelled<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::HasUserCancelled(this)
            }
        }
        unsafe extern "system" fn SetProgress<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcompleted: u32, dwtotal: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::SetProgress(this, core::mem::transmute_copy(&dwcompleted), core::mem::transmute_copy(&dwtotal)).into()
            }
        }
        unsafe extern "system" fn SetProgress64<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullcompleted: u64, ulltotal: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::SetProgress64(this, core::mem::transmute_copy(&ullcompleted), core::mem::transmute_copy(&ulltotal)).into()
            }
        }
        unsafe extern "system" fn SetLine<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlinenum: u32, pwzstring: windows_core::PCWSTR, fcompactpath: windows_core::BOOL, pvresevered: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::SetLine(this, core::mem::transmute_copy(&dwlinenum), core::mem::transmute(&pwzstring), core::mem::transmute_copy(&fcompactpath), core::mem::transmute_copy(&pvresevered)).into()
            }
        }
        unsafe extern "system" fn SetCancelMsg<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzcancelmsg: windows_core::PCWSTR, pvresevered: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::SetCancelMsg(this, core::mem::transmute(&pwzcancelmsg), core::mem::transmute_copy(&pvresevered)).into()
            }
        }
        unsafe extern "system" fn Timer<Identity: IProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeraction: u32, pvresevered: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressDialog_Impl::Timer(this, core::mem::transmute_copy(&dwtimeraction), core::mem::transmute_copy(&pvresevered)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartProgressDialog: StartProgressDialog::<Identity, OFFSET>,
            StopProgressDialog: StopProgressDialog::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            SetAnimation: SetAnimation::<Identity, OFFSET>,
            HasUserCancelled: HasUserCancelled::<Identity, OFFSET>,
            SetProgress: SetProgress::<Identity, OFFSET>,
            SetProgress64: SetProgress64::<Identity, OFFSET>,
            SetLine: SetLine::<Identity, OFFSET>,
            SetCancelMsg: SetCancelMsg::<Identity, OFFSET>,
            Timer: Timer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressDialog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl windows_core::RuntimeName for IProgressDialog {}
windows_core::imp::define_interface!(IQueryInfo, IQueryInfo_Vtbl, 0x00021500_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IQueryInfo, windows_core::IUnknown);
impl IQueryInfo {
    pub unsafe fn GetInfoTip(&self, dwflags: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfoTip)(windows_core::Interface::as_raw(self), dwflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInfoFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfoFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInfoTip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetInfoFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IQueryInfo_Impl: windows_core::IUnknownImpl {
    fn GetInfoTip(&self, dwflags: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetInfoFlags(&self) -> windows_core::Result<u32>;
}
impl IQueryInfo_Vtbl {
    pub const fn new<Identity: IQueryInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInfoTip<Identity: IQueryInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppwsztip: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryInfo_Impl::GetInfoTip(this, core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppwsztip.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInfoFlags<Identity: IQueryInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryInfo_Impl::GetInfoFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfoTip: GetInfoTip::<Identity, OFFSET>,
            GetInfoFlags: GetInfoFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IQueryInfo {}
pub const ISHCUTCMDID_COMMITHISTORY: i32 = 2;
pub const ISHCUTCMDID_DOWNLOADICON: i32 = 0;
pub const ISHCUTCMDID_INTSHORTCUTCREATE: i32 = 1;
pub const ISHCUTCMDID_SETUSERAWURL: i32 = 3;
pub const IS_FULLSCREEN: u32 = 2;
pub const IS_NORMAL: u32 = 1;
pub const IS_SPLIT: u32 = 4;
pub const IS_VALIDSIZESTATEBITS: u32 = 7;
pub const IS_VALIDSTATEBITS: i32 = -1073741817;
windows_core::imp::define_interface!(ISearchContext, ISearchContext_Vtbl, 0x09f656a2_41af_480c_88f7_16cc0d164615);
windows_core::imp::interface_hierarchy!(ISearchContext, windows_core::IUnknown);
impl ISearchContext {
    pub unsafe fn GetSearchUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSearchUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSearchText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSearchText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSearchStyle(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSearchStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSearchUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSearchText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSearchStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ISearchContext_Impl: windows_core::IUnknownImpl {
    fn GetSearchUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSearchText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSearchStyle(&self) -> windows_core::Result<u32>;
}
impl ISearchContext_Vtbl {
    pub const fn new<Identity: ISearchContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSearchUrl<Identity: ISearchContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsearchurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchContext_Impl::GetSearchUrl(this) {
                    Ok(ok__) => {
                        pbstrsearchurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSearchText<Identity: ISearchContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsearchtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchContext_Impl::GetSearchText(this) {
                    Ok(ok__) => {
                        pbstrsearchtext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSearchStyle<Identity: ISearchContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsearchstyle: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchContext_Impl::GetSearchStyle(this) {
                    Ok(ok__) => {
                        pdwsearchstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSearchUrl: GetSearchUrl::<Identity, OFFSET>,
            GetSearchText: GetSearchText::<Identity, OFFSET>,
            GetSearchStyle: GetSearchStyle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchContext {}
windows_core::imp::define_interface!(IShellChangeNotify, IShellChangeNotify_Vtbl, 0xd82be2b1_5764_11d0_a96e_00c04fd705a2);
windows_core::imp::interface_hierarchy!(IShellChangeNotify, windows_core::IUnknown);
impl IShellChangeNotify {
    #[cfg(feature = "shtypes")]
    pub unsafe fn OnChange(&self, levent: i32, pidl1: Option<*const super::shtypes::ITEMIDLIST>, pidl2: Option<*const super::shtypes::ITEMIDLIST>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), levent, pidl1.unwrap_or(core::mem::zeroed()) as _, pidl2.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shtypes")]
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::shtypes::ITEMIDLIST, *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    OnChange: usize,
}
#[cfg(feature = "shtypes")]
pub trait IShellChangeNotify_Impl: windows_core::IUnknownImpl {
    fn OnChange(&self, levent: i32, pidl1: *const super::shtypes::ITEMIDLIST, pidl2: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<()>;
}
#[cfg(feature = "shtypes")]
impl IShellChangeNotify_Vtbl {
    pub const fn new<Identity: IShellChangeNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChange<Identity: IShellChangeNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, levent: i32, pidl1: *const super::shtypes::ITEMIDLIST, pidl2: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellChangeNotify_Impl::OnChange(this, core::mem::transmute_copy(&levent), core::mem::transmute_copy(&pidl1), core::mem::transmute_copy(&pidl2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellChangeNotify as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shtypes")]
impl windows_core::RuntimeName for IShellChangeNotify {}
windows_core::imp::define_interface!(IShellDetails, IShellDetails_Vtbl, 0x000214ec_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IShellDetails, windows_core::IUnknown);
impl IShellDetails {
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetDetailsOf(&self, pidl: Option<*const super::shtypes::ITEMIDLIST>, icolumn: u32, pdetails: *mut super::shtypes::SHELLDETAILS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDetailsOf)(windows_core::Interface::as_raw(self), pidl.unwrap_or(core::mem::zeroed()) as _, icolumn, pdetails as _) }
    }
    pub unsafe fn ColumnClick(&self, icolumn: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ColumnClick)(windows_core::Interface::as_raw(self), icolumn) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellDetails_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shtypes")]
    pub GetDetailsOf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, u32, *mut super::shtypes::SHELLDETAILS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetDetailsOf: usize,
    pub ColumnClick: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "shtypes")]
pub trait IShellDetails_Impl: windows_core::IUnknownImpl {
    fn GetDetailsOf(&self, pidl: *const super::shtypes::ITEMIDLIST, icolumn: u32, pdetails: *mut super::shtypes::SHELLDETAILS) -> windows_core::Result<()>;
    fn ColumnClick(&self, icolumn: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "shtypes")]
impl IShellDetails_Vtbl {
    pub const fn new<Identity: IShellDetails_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDetailsOf<Identity: IShellDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, icolumn: u32, pdetails: *mut super::shtypes::SHELLDETAILS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDetails_Impl::GetDetailsOf(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&icolumn), core::mem::transmute_copy(&pdetails)).into()
            }
        }
        unsafe extern "system" fn ColumnClick<Identity: IShellDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icolumn: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDetails_Impl::ColumnClick(this, core::mem::transmute_copy(&icolumn)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDetailsOf: GetDetailsOf::<Identity, OFFSET>,
            ColumnClick: ColumnClick::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellDetails as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shtypes")]
impl windows_core::RuntimeName for IShellDetails {}
windows_core::imp::define_interface!(IShellExecuteHookA, IShellExecuteHookA_Vtbl, 0x000214f5_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IShellExecuteHookA, windows_core::IUnknown);
impl IShellExecuteHookA {
    #[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
    pub unsafe fn Execute(&self, pei: *mut super::shellapi::SHELLEXECUTEINFOA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), pei as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellExecuteHookA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::shellapi::SHELLEXECUTEINFOA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt")))]
    Execute: usize,
}
#[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
pub trait IShellExecuteHookA_Impl: windows_core::IUnknownImpl {
    fn Execute(&self, pei: *mut super::shellapi::SHELLEXECUTEINFOA) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
impl IShellExecuteHookA_Vtbl {
    pub const fn new<Identity: IShellExecuteHookA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Execute<Identity: IShellExecuteHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pei: *mut super::shellapi::SHELLEXECUTEINFOA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellExecuteHookA_Impl::Execute(this, core::mem::transmute_copy(&pei)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Execute: Execute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellExecuteHookA as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IShellExecuteHookA {}
windows_core::imp::define_interface!(IShellExecuteHookW, IShellExecuteHookW_Vtbl, 0x000214fb_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IShellExecuteHookW, windows_core::IUnknown);
impl IShellExecuteHookW {
    #[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
    pub unsafe fn Execute(&self, pei: *mut super::shellapi::SHELLEXECUTEINFOW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), pei as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellExecuteHookW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::shellapi::SHELLEXECUTEINFOW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt")))]
    Execute: usize,
}
#[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
pub trait IShellExecuteHookW_Impl: windows_core::IUnknownImpl {
    fn Execute(&self, pei: *mut super::shellapi::SHELLEXECUTEINFOW) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
impl IShellExecuteHookW_Vtbl {
    pub const fn new<Identity: IShellExecuteHookW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Execute<Identity: IShellExecuteHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pei: *mut super::shellapi::SHELLEXECUTEINFOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellExecuteHookW_Impl::Execute(this, core::mem::transmute_copy(&pei)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Execute: Execute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellExecuteHookW as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shellapi", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IShellExecuteHookW {}
windows_core::imp::define_interface!(IShellFolderView, IShellFolderView_Vtbl, 0x37a378c0_f82d_11ce_ae65_08002b2e1262);
windows_core::imp::interface_hierarchy!(IShellFolderView, windows_core::IUnknown);
impl IShellFolderView {
    #[cfg(feature = "minwindef")]
    pub unsafe fn Rearrange(&self, lparamsort: super::minwindef::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Rearrange)(windows_core::Interface::as_raw(self), lparamsort) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetArrangeParam(&self) -> windows_core::Result<super::minwindef::LPARAM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArrangeParam)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ArrangeGrid(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ArrangeGrid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AutoArrange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AutoArrange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAutoArrange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAutoArrange)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn AddObject(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddObject)(windows_core::Interface::as_raw(self), pidl, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetObject(&self, ppidl: *mut super::shtypes::LPITEMIDLIST, uitem: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), ppidl as _, uitem) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn RemoveObject(&self, pidl: Option<*const super::shtypes::ITEMIDLIST>) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoveObject)(windows_core::Interface::as_raw(self), pidl.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObjectCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetObjectCount(&self, ucount: u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectCount)(windows_core::Interface::as_raw(self), ucount, dwflags) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn UpdateObject(&self, pidlold: *const super::shtypes::ITEMIDLIST, pidlnew: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateObject)(windows_core::Interface::as_raw(self), pidlold, pidlnew, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn RefreshObject(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RefreshObject)(windows_core::Interface::as_raw(self), pidl, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRedraw(&self, bredraw: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRedraw)(windows_core::Interface::as_raw(self), bredraw.into()) }
    }
    pub unsafe fn GetSelectedCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectedCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetSelectedObjects(&self, pppidl: *mut *mut super::shtypes::LPCITEMIDLIST, puitems: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelectedObjects)(windows_core::Interface::as_raw(self), pppidl as _, puitems as _) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn IsDropOnSource<P0>(&self, pdroptarget: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oleidl::IDropTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsDropOnSource)(windows_core::Interface::as_raw(self), pdroptarget.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDragPoint(&self) -> windows_core::Result<super::windef::POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDragPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDropPoint(&self) -> windows_core::Result<super::windef::POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDropPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn MoveIcons<P0>(&self, pdataobject: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).MoveIcons)(windows_core::Interface::as_raw(self), pdataobject.param().abi()) }
    }
    #[cfg(all(feature = "shtypes", feature = "windef"))]
    pub unsafe fn SetItemPos(&self, pidl: *const super::shtypes::ITEMIDLIST, ppt: *const super::windef::POINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetItemPos)(windows_core::Interface::as_raw(self), pidl, ppt) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn IsBkDropTarget<P0>(&self, pdroptarget: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oleidl::IDropTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsBkDropTarget)(windows_core::Interface::as_raw(self), pdroptarget.param().abi()) }
    }
    pub unsafe fn SetClipboard(&self, bmove: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipboard)(windows_core::Interface::as_raw(self), bmove.into()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SetPoints<P0>(&self, pdataobject: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPoints)(windows_core::Interface::as_raw(self), pdataobject.param().abi()) }
    }
    pub unsafe fn GetItemSpacing(&self) -> windows_core::Result<ITEMSPACING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemSpacing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCallback<P0>(&self, pnewcb: P0) -> windows_core::Result<IShellFolderViewCB>
    where
        P0: windows_core::Param<IShellFolderViewCB>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetCallback)(windows_core::Interface::as_raw(self), pnewcb.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Select(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn QuerySupport(&self, pdwsupport: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QuerySupport)(windows_core::Interface::as_raw(self), pdwsupport as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn SetAutomationObject<P0>(&self, pdisp: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAutomationObject)(windows_core::Interface::as_raw(self), pdisp.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellFolderView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub Rearrange: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Rearrange: usize,
    #[cfg(feature = "minwindef")]
    pub GetArrangeParam: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetArrangeParam: usize,
    pub ArrangeGrid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutoArrange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAutoArrange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub AddObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    AddObject: usize,
    #[cfg(feature = "shtypes")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::shtypes::LPITEMIDLIST, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetObject: usize,
    #[cfg(feature = "shtypes")]
    pub RemoveObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    RemoveObject: usize,
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub UpdateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *const super::shtypes::ITEMIDLIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    UpdateObject: usize,
    #[cfg(feature = "shtypes")]
    pub RefreshObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    RefreshObject: usize,
    pub SetRedraw: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSelectedCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub GetSelectedObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::shtypes::LPCITEMIDLIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetSelectedObjects: usize,
    #[cfg(feature = "oleidl")]
    pub IsDropOnSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    IsDropOnSource: usize,
    #[cfg(feature = "windef")]
    pub GetDragPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDragPoint: usize,
    #[cfg(feature = "windef")]
    pub GetDropPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDropPoint: usize,
    #[cfg(feature = "objidl")]
    pub MoveIcons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    MoveIcons: usize,
    #[cfg(all(feature = "shtypes", feature = "windef"))]
    pub SetItemPos: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *const super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shtypes", feature = "windef")))]
    SetItemPos: usize,
    #[cfg(feature = "oleidl")]
    pub IsBkDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    IsBkDropTarget: usize,
    pub SetClipboard: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub SetPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SetPoints: usize,
    pub GetItemSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ITEMSPACING) -> windows_core::HRESULT,
    pub SetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QuerySupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub SetAutomationObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    SetAutomationObject: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "oleidl", feature = "shtypes", feature = "windef"))]
pub trait IShellFolderView_Impl: windows_core::IUnknownImpl {
    fn Rearrange(&self, lparamsort: super::minwindef::LPARAM) -> windows_core::Result<()>;
    fn GetArrangeParam(&self) -> windows_core::Result<super::minwindef::LPARAM>;
    fn ArrangeGrid(&self) -> windows_core::Result<()>;
    fn AutoArrange(&self) -> windows_core::Result<()>;
    fn GetAutoArrange(&self) -> windows_core::Result<()>;
    fn AddObject(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<u32>;
    fn GetObject(&self, ppidl: *mut super::shtypes::LPITEMIDLIST, uitem: u32) -> windows_core::Result<()>;
    fn RemoveObject(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<u32>;
    fn GetObjectCount(&self) -> windows_core::Result<u32>;
    fn SetObjectCount(&self, ucount: u32, dwflags: u32) -> windows_core::Result<()>;
    fn UpdateObject(&self, pidlold: *const super::shtypes::ITEMIDLIST, pidlnew: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<u32>;
    fn RefreshObject(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<u32>;
    fn SetRedraw(&self, bredraw: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSelectedCount(&self) -> windows_core::Result<u32>;
    fn GetSelectedObjects(&self, pppidl: *mut *mut super::shtypes::LPCITEMIDLIST, puitems: *mut u32) -> windows_core::Result<()>;
    fn IsDropOnSource(&self, pdroptarget: windows_core::Ref<super::oleidl::IDropTarget>) -> windows_core::Result<()>;
    fn GetDragPoint(&self) -> windows_core::Result<super::windef::POINT>;
    fn GetDropPoint(&self) -> windows_core::Result<super::windef::POINT>;
    fn MoveIcons(&self, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<()>;
    fn SetItemPos(&self, pidl: *const super::shtypes::ITEMIDLIST, ppt: *const super::windef::POINT) -> windows_core::Result<()>;
    fn IsBkDropTarget(&self, pdroptarget: windows_core::Ref<super::oleidl::IDropTarget>) -> windows_core::Result<()>;
    fn SetClipboard(&self, bmove: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetPoints(&self, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<()>;
    fn GetItemSpacing(&self) -> windows_core::Result<ITEMSPACING>;
    fn SetCallback(&self, pnewcb: windows_core::Ref<IShellFolderViewCB>) -> windows_core::Result<IShellFolderViewCB>;
    fn Select(&self, dwflags: u32) -> windows_core::Result<()>;
    fn QuerySupport(&self, pdwsupport: *mut u32) -> windows_core::Result<()>;
    fn SetAutomationObject(&self, pdisp: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "oleidl", feature = "shtypes", feature = "windef"))]
impl IShellFolderView_Vtbl {
    pub const fn new<Identity: IShellFolderView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Rearrange<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lparamsort: super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::Rearrange(this, core::mem::transmute_copy(&lparamsort)).into()
            }
        }
        unsafe extern "system" fn GetArrangeParam<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plparamsort: *mut super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::GetArrangeParam(this) {
                    Ok(ok__) => {
                        plparamsort.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ArrangeGrid<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::ArrangeGrid(this).into()
            }
        }
        unsafe extern "system" fn AutoArrange<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::AutoArrange(this).into()
            }
        }
        unsafe extern "system" fn GetAutoArrange<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::GetAutoArrange(this).into()
            }
        }
        unsafe extern "system" fn AddObject<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, puitem: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::AddObject(this, core::mem::transmute_copy(&pidl)) {
                    Ok(ok__) => {
                        puitem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObject<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidl: *mut super::shtypes::LPITEMIDLIST, uitem: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::GetObject(this, core::mem::transmute_copy(&ppidl), core::mem::transmute_copy(&uitem)).into()
            }
        }
        unsafe extern "system" fn RemoveObject<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, puitem: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::RemoveObject(this, core::mem::transmute_copy(&pidl)) {
                    Ok(ok__) => {
                        puitem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectCount<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::GetObjectCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectCount<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucount: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::SetObjectCount(this, core::mem::transmute_copy(&ucount), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn UpdateObject<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidlold: *const super::shtypes::ITEMIDLIST, pidlnew: *const super::shtypes::ITEMIDLIST, puitem: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::UpdateObject(this, core::mem::transmute_copy(&pidlold), core::mem::transmute_copy(&pidlnew)) {
                    Ok(ok__) => {
                        puitem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RefreshObject<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, puitem: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::RefreshObject(this, core::mem::transmute_copy(&pidl)) {
                    Ok(ok__) => {
                        puitem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRedraw<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bredraw: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::SetRedraw(this, core::mem::transmute_copy(&bredraw)).into()
            }
        }
        unsafe extern "system" fn GetSelectedCount<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puselected: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::GetSelectedCount(this) {
                    Ok(ok__) => {
                        puselected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelectedObjects<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppidl: *mut *mut super::shtypes::LPCITEMIDLIST, puitems: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::GetSelectedObjects(this, core::mem::transmute_copy(&pppidl), core::mem::transmute_copy(&puitems)).into()
            }
        }
        unsafe extern "system" fn IsDropOnSource<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdroptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::IsDropOnSource(this, core::mem::transmute_copy(&pdroptarget)).into()
            }
        }
        unsafe extern "system" fn GetDragPoint<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppt: *mut super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::GetDragPoint(this) {
                    Ok(ok__) => {
                        ppt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDropPoint<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppt: *mut super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::GetDropPoint(this) {
                    Ok(ok__) => {
                        ppt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveIcons<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::MoveIcons(this, core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        unsafe extern "system" fn SetItemPos<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, ppt: *const super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::SetItemPos(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&ppt)).into()
            }
        }
        unsafe extern "system" fn IsBkDropTarget<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdroptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::IsBkDropTarget(this, core::mem::transmute_copy(&pdroptarget)).into()
            }
        }
        unsafe extern "system" fn SetClipboard<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmove: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::SetClipboard(this, core::mem::transmute_copy(&bmove)).into()
            }
        }
        unsafe extern "system" fn SetPoints<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::SetPoints(this, core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        unsafe extern "system" fn GetItemSpacing<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspacing: *mut ITEMSPACING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::GetItemSpacing(this) {
                    Ok(ok__) => {
                        pspacing.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCallback<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewcb: *mut core::ffi::c_void, ppoldcb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderView_Impl::SetCallback(this, core::mem::transmute_copy(&pnewcb)) {
                    Ok(ok__) => {
                        ppoldcb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Select<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::Select(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn QuerySupport<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::QuerySupport(this, core::mem::transmute_copy(&pdwsupport)).into()
            }
        }
        unsafe extern "system" fn SetAutomationObject<Identity: IShellFolderView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisp: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderView_Impl::SetAutomationObject(this, core::mem::transmute_copy(&pdisp)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Rearrange: Rearrange::<Identity, OFFSET>,
            GetArrangeParam: GetArrangeParam::<Identity, OFFSET>,
            ArrangeGrid: ArrangeGrid::<Identity, OFFSET>,
            AutoArrange: AutoArrange::<Identity, OFFSET>,
            GetAutoArrange: GetAutoArrange::<Identity, OFFSET>,
            AddObject: AddObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            RemoveObject: RemoveObject::<Identity, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
            SetObjectCount: SetObjectCount::<Identity, OFFSET>,
            UpdateObject: UpdateObject::<Identity, OFFSET>,
            RefreshObject: RefreshObject::<Identity, OFFSET>,
            SetRedraw: SetRedraw::<Identity, OFFSET>,
            GetSelectedCount: GetSelectedCount::<Identity, OFFSET>,
            GetSelectedObjects: GetSelectedObjects::<Identity, OFFSET>,
            IsDropOnSource: IsDropOnSource::<Identity, OFFSET>,
            GetDragPoint: GetDragPoint::<Identity, OFFSET>,
            GetDropPoint: GetDropPoint::<Identity, OFFSET>,
            MoveIcons: MoveIcons::<Identity, OFFSET>,
            SetItemPos: SetItemPos::<Identity, OFFSET>,
            IsBkDropTarget: IsBkDropTarget::<Identity, OFFSET>,
            SetClipboard: SetClipboard::<Identity, OFFSET>,
            SetPoints: SetPoints::<Identity, OFFSET>,
            GetItemSpacing: GetItemSpacing::<Identity, OFFSET>,
            SetCallback: SetCallback::<Identity, OFFSET>,
            Select: Select::<Identity, OFFSET>,
            QuerySupport: QuerySupport::<Identity, OFFSET>,
            SetAutomationObject: SetAutomationObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellFolderView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "oleidl", feature = "shtypes", feature = "windef"))]
impl windows_core::RuntimeName for IShellFolderView {}
windows_core::imp::define_interface!(IShellFolderViewCB, IShellFolderViewCB_Vtbl, 0x2047e320_f2a9_11ce_ae65_08002b2e1262);
windows_core::imp::interface_hierarchy!(IShellFolderViewCB, windows_core::IUnknown);
impl IShellFolderViewCB {
    #[cfg(feature = "minwindef")]
    pub unsafe fn MessageSFVCB(&self, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MessageSFVCB)(windows_core::Interface::as_raw(self), umsg, wparam, lparam) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellFolderViewCB_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub MessageSFVCB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::minwindef::WPARAM, super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    MessageSFVCB: usize,
}
#[cfg(feature = "minwindef")]
pub trait IShellFolderViewCB_Impl: windows_core::IUnknownImpl {
    fn MessageSFVCB(&self, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<()>;
}
#[cfg(feature = "minwindef")]
impl IShellFolderViewCB_Vtbl {
    pub const fn new<Identity: IShellFolderViewCB_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MessageSFVCB<Identity: IShellFolderViewCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewCB_Impl::MessageSFVCB(this, core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MessageSFVCB: MessageSFVCB::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellFolderViewCB as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IShellFolderViewCB {}
windows_core::imp::define_interface!(IShellIconOverlay, IShellIconOverlay_Vtbl, 0x7d688a70_c613_11d0_999b_00c04fd655e1);
windows_core::imp::interface_hierarchy!(IShellIconOverlay, windows_core::IUnknown);
impl IShellIconOverlay {
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetOverlayIndex(&self, pidl: *const super::shtypes::ITEMIDLIST, pindex: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOverlayIndex)(windows_core::Interface::as_raw(self), pidl, pindex as _) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetOverlayIconIndex(&self, pidl: *const super::shtypes::ITEMIDLIST, piconindex: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOverlayIconIndex)(windows_core::Interface::as_raw(self), pidl, piconindex as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellIconOverlay_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shtypes")]
    pub GetOverlayIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetOverlayIndex: usize,
    #[cfg(feature = "shtypes")]
    pub GetOverlayIconIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetOverlayIconIndex: usize,
}
#[cfg(feature = "shtypes")]
pub trait IShellIconOverlay_Impl: windows_core::IUnknownImpl {
    fn GetOverlayIndex(&self, pidl: *const super::shtypes::ITEMIDLIST, pindex: *mut i32) -> windows_core::Result<()>;
    fn GetOverlayIconIndex(&self, pidl: *const super::shtypes::ITEMIDLIST, piconindex: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "shtypes")]
impl IShellIconOverlay_Vtbl {
    pub const fn new<Identity: IShellIconOverlay_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOverlayIndex<Identity: IShellIconOverlay_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellIconOverlay_Impl::GetOverlayIndex(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&pindex)).into()
            }
        }
        unsafe extern "system" fn GetOverlayIconIndex<Identity: IShellIconOverlay_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, piconindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellIconOverlay_Impl::GetOverlayIconIndex(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&piconindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOverlayIndex: GetOverlayIndex::<Identity, OFFSET>,
            GetOverlayIconIndex: GetOverlayIconIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellIconOverlay as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shtypes")]
impl windows_core::RuntimeName for IShellIconOverlay {}
windows_core::imp::define_interface!(IShellIconOverlayManager, IShellIconOverlayManager_Vtbl, 0xf10b5e34_dd3b_42a7_aa7d_2f4ec54bb09b);
windows_core::imp::interface_hierarchy!(IShellIconOverlayManager, windows_core::IUnknown);
impl IShellIconOverlayManager {
    pub unsafe fn GetFileOverlayInfo<P0>(&self, pwszpath: P0, dwattrib: u32, pindex: *mut i32, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFileOverlayInfo)(windows_core::Interface::as_raw(self), pwszpath.param().abi(), dwattrib, pindex as _, dwflags) }
    }
    pub unsafe fn GetReservedOverlayInfo<P0>(&self, pwszpath: P0, dwattrib: u32, pindex: *mut i32, dwflags: u32, ireservedid: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetReservedOverlayInfo)(windows_core::Interface::as_raw(self), pwszpath.param().abi(), dwattrib, pindex as _, dwflags, ireservedid) }
    }
    pub unsafe fn RefreshOverlayImages(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshOverlayImages)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn LoadNonloadedOverlayIdentifiers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LoadNonloadedOverlayIdentifiers)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OverlayIndexFromImageIndex(&self, iimage: i32, piindex: *mut i32, fadd: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OverlayIndexFromImageIndex)(windows_core::Interface::as_raw(self), iimage, piindex as _, fadd.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellIconOverlayManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFileOverlayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut i32, u32) -> windows_core::HRESULT,
    pub GetReservedOverlayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut i32, u32, i32) -> windows_core::HRESULT,
    pub RefreshOverlayImages: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub LoadNonloadedOverlayIdentifiers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OverlayIndexFromImageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IShellIconOverlayManager_Impl: windows_core::IUnknownImpl {
    fn GetFileOverlayInfo(&self, pwszpath: &windows_core::PCWSTR, dwattrib: u32, pindex: *mut i32, dwflags: u32) -> windows_core::Result<()>;
    fn GetReservedOverlayInfo(&self, pwszpath: &windows_core::PCWSTR, dwattrib: u32, pindex: *mut i32, dwflags: u32, ireservedid: i32) -> windows_core::Result<()>;
    fn RefreshOverlayImages(&self, dwflags: u32) -> windows_core::Result<()>;
    fn LoadNonloadedOverlayIdentifiers(&self) -> windows_core::Result<()>;
    fn OverlayIndexFromImageIndex(&self, iimage: i32, piindex: *mut i32, fadd: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IShellIconOverlayManager_Vtbl {
    pub const fn new<Identity: IShellIconOverlayManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFileOverlayInfo<Identity: IShellIconOverlayManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpath: windows_core::PCWSTR, dwattrib: u32, pindex: *mut i32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellIconOverlayManager_Impl::GetFileOverlayInfo(this, core::mem::transmute(&pwszpath), core::mem::transmute_copy(&dwattrib), core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetReservedOverlayInfo<Identity: IShellIconOverlayManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpath: windows_core::PCWSTR, dwattrib: u32, pindex: *mut i32, dwflags: u32, ireservedid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellIconOverlayManager_Impl::GetReservedOverlayInfo(this, core::mem::transmute(&pwszpath), core::mem::transmute_copy(&dwattrib), core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ireservedid)).into()
            }
        }
        unsafe extern "system" fn RefreshOverlayImages<Identity: IShellIconOverlayManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellIconOverlayManager_Impl::RefreshOverlayImages(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn LoadNonloadedOverlayIdentifiers<Identity: IShellIconOverlayManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellIconOverlayManager_Impl::LoadNonloadedOverlayIdentifiers(this).into()
            }
        }
        unsafe extern "system" fn OverlayIndexFromImageIndex<Identity: IShellIconOverlayManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, piindex: *mut i32, fadd: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellIconOverlayManager_Impl::OverlayIndexFromImageIndex(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&piindex), core::mem::transmute_copy(&fadd)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileOverlayInfo: GetFileOverlayInfo::<Identity, OFFSET>,
            GetReservedOverlayInfo: GetReservedOverlayInfo::<Identity, OFFSET>,
            RefreshOverlayImages: RefreshOverlayImages::<Identity, OFFSET>,
            LoadNonloadedOverlayIdentifiers: LoadNonloadedOverlayIdentifiers::<Identity, OFFSET>,
            OverlayIndexFromImageIndex: OverlayIndexFromImageIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellIconOverlayManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IShellIconOverlayManager {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ITEMSPACING {
    pub cxSmall: i32,
    pub cySmall: i32,
    pub cxLarge: i32,
    pub cyLarge: i32,
}
windows_core::imp::define_interface!(IURLSearchHook, IURLSearchHook_Vtbl, 0xac60f6a0_0fd9_11d0_99cb_00c04fd64497);
windows_core::imp::interface_hierarchy!(IURLSearchHook, windows_core::IUnknown);
impl IURLSearchHook {
    pub unsafe fn Translate(&self, pwszsearchurl: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Translate)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszsearchurl.as_ptr()), pwszsearchurl.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IURLSearchHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Translate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
pub trait IURLSearchHook_Impl: windows_core::IUnknownImpl {
    fn Translate(&self, pwszsearchurl: windows_core::PWSTR, cchbuffersize: u32) -> windows_core::Result<()>;
}
impl IURLSearchHook_Vtbl {
    pub const fn new<Identity: IURLSearchHook_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Translate<Identity: IURLSearchHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsearchurl: windows_core::PWSTR, cchbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IURLSearchHook_Impl::Translate(this, core::mem::transmute_copy(&pwszsearchurl), core::mem::transmute_copy(&cchbuffersize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Translate: Translate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IURLSearchHook as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IURLSearchHook {}
windows_core::imp::define_interface!(IURLSearchHook2, IURLSearchHook2_Vtbl, 0x5ee44da4_6d32_46e3_86bc_07540dedd0e0);
impl core::ops::Deref for IURLSearchHook2 {
    type Target = IURLSearchHook;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IURLSearchHook2, windows_core::IUnknown, IURLSearchHook);
impl IURLSearchHook2 {
    pub unsafe fn TranslateWithSearchContext<P2>(&self, pwszsearchurl: &mut [u16], psearchcontext: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ISearchContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).TranslateWithSearchContext)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszsearchurl.as_ptr()), pwszsearchurl.len().try_into().unwrap(), psearchcontext.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IURLSearchHook2_Vtbl {
    pub base__: IURLSearchHook_Vtbl,
    pub TranslateWithSearchContext: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IURLSearchHook2_Impl: IURLSearchHook_Impl {
    fn TranslateWithSearchContext(&self, pwszsearchurl: windows_core::PWSTR, cchbuffersize: u32, psearchcontext: windows_core::Ref<ISearchContext>) -> windows_core::Result<()>;
}
impl IURLSearchHook2_Vtbl {
    pub const fn new<Identity: IURLSearchHook2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TranslateWithSearchContext<Identity: IURLSearchHook2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsearchurl: windows_core::PWSTR, cchbuffersize: u32, psearchcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IURLSearchHook2_Impl::TranslateWithSearchContext(this, core::mem::transmute_copy(&pwszsearchurl), core::mem::transmute_copy(&cchbuffersize), core::mem::transmute_copy(&psearchcontext)).into()
            }
        }
        Self { base__: IURLSearchHook_Vtbl::new::<Identity, OFFSET>(), TranslateWithSearchContext: TranslateWithSearchContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IURLSearchHook2 as windows_core::Interface>::IID || iid == &<IURLSearchHook as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IURLSearchHook2 {}
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
pub type LPFNDFMCALLBACK = Option<unsafe extern "system" fn(psf: windows_core::Ref<super::shobjidl_core::IShellFolder>, hwnd: super::windef::HWND, pdtobj: windows_core::Ref<super::objidl::IDataObject>, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
pub type LPFNVIEWCALLBACK = Option<unsafe extern "system" fn(psvouter: windows_core::Ref<super::shobjidl_core::IShellView>, psf: windows_core::Ref<super::shobjidl_core::IShellFolder>, hwndmain: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT>;
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub bFullScreen: windows_core::BOOL,
    pub bQuickEdit: windows_core::BOOL,
    pub bInsertMode: windows_core::BOOL,
    pub bAutoPosition: windows_core::BOOL,
    pub uHistoryBufferSize: u32,
    pub uNumberOfHistoryBuffers: u32,
    pub bHistoryNoDup: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OPENASINFO {
    pub pcszFile: windows_core::PCWSTR,
    pub pcszClass: windows_core::PCWSTR,
    pub oaifInFlags: OPEN_AS_INFO_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OPEN_AS_INFO_FLAGS(pub i32);
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SFV_CREATE {
    pub cbSize: u32,
    pub pshf: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub psvOuter: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellView>>,
    pub psfvcb: core::mem::ManuallyDrop<Option<IShellFolderViewCB>>,
}
pub type SHARD = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shobjidl_core")]
#[derive(Default)]
pub struct SHARDAPPIDINFO {
    pub psi: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellItem>>,
    pub pszAppID: windows_core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy, Default)]
pub struct SHARDAPPIDINFOIDLIST {
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub pszAppID: windows_core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shobjidl_core")]
#[derive(Default)]
pub struct SHARDAPPIDINFOLINK {
    pub psl: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellLinkA>>,
    pub pszAppID: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Default)]
pub struct SHChangeNotifyEntry {
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub fRecursive: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHDESCRIPTIONID {
    pub dwDescriptionId: u32,
    pub clsid: windows_core::GUID,
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
    pub _bitfield: windows_core::BOOL,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SHELLSTATEA {
    pub _bitfield1: windows_core::BOOL,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: windows_core::BOOL,
}
pub const SHELLSTATEVERSION_IE4: u32 = 9;
pub const SHELLSTATEVERSION_WIN2K: u32 = 10;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SHELLSTATEW {
    pub _bitfield1: windows_core::BOOL,
    pub dwWin95Unused: u32,
    pub uWin95Unused: u32,
    pub lParamSort: i32,
    pub iSortDirection: i32,
    pub version: u32,
    pub uNotUsed: u32,
    pub _bitfield2: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHFOLDERCUSTOMSETTINGS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub pvid: *mut super::shobjidl_core::SHELLVIEWID,
    pub pszWebViewTemplate: windows_core::PWSTR,
    pub cchWebViewTemplate: u32,
    pub pszWebViewTemplateVersion: windows_core::PWSTR,
    pub pszInfoTip: windows_core::PWSTR,
    pub cchInfoTip: u32,
    pub pclsid: *mut windows_core::GUID,
    pub dwFlags: u32,
    pub pszIconFile: windows_core::PWSTR,
    pub cchIconFile: u32,
    pub iIconIndex: i32,
    pub pszLogo: windows_core::PWSTR,
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
pub const STR_PARSE_PARTIAL_IDLIST: windows_core::PCWSTR = windows_core::w!("ParseOriginalItem");
pub const STR_PARSE_WITH_PROPERTIES: windows_core::PCWSTR = windows_core::w!("ParseWithProperties");
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
