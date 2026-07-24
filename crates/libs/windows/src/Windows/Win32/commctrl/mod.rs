#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateMappedBitmap(hinstance: super::HINSTANCE, idbitmap: isize, wflags: u32, lpcolormap: Option<*const COLORMAP>, inummaps: i32) -> super::HBITMAP {
    windows_core::link!("comctl32.dll" "system" fn CreateMappedBitmap(hinstance : super::HINSTANCE, idbitmap : isize, wflags : u32, lpcolormap : *const COLORMAP, inummaps : i32) -> super::HBITMAP);
    unsafe { CreateMappedBitmap(hinstance, idbitmap, wflags, lpcolormap.unwrap_or(core::mem::zeroed()) as _, inummaps) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateStatusWindowA<P1>(style: i32, lpsztext: P1, hwndparent: super::HWND, wid: u32) -> super::HWND
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn CreateStatusWindowA(style : i32, lpsztext : windows_core::PCSTR, hwndparent : super::HWND, wid : u32) -> super::HWND);
    unsafe { CreateStatusWindowA(style, lpsztext.param().abi(), hwndparent, wid) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateStatusWindowW<P1>(style: i32, lpsztext: P1, hwndparent: super::HWND, wid: u32) -> super::HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn CreateStatusWindowW(style : i32, lpsztext : windows_core::PCWSTR, hwndparent : super::HWND, wid : u32) -> super::HWND);
    unsafe { CreateStatusWindowW(style, lpsztext.param().abi(), hwndparent, wid) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateToolbarEx(hwnd: super::HWND, ws: u32, wid: u32, nbitmaps: i32, hbminst: super::HINSTANCE, wbmid: usize, lpbuttons: *const TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> super::HWND {
    windows_core::link!("comctl32.dll" "system" fn CreateToolbarEx(hwnd : super::HWND, ws : u32, wid : u32, nbitmaps : i32, hbminst : super::HINSTANCE, wbmid : usize, lpbuttons : *const TBBUTTON, inumbuttons : i32, dxbutton : i32, dybutton : i32, dxbitmap : i32, dybitmap : i32, ustructsize : u32) -> super::HWND);
    unsafe { CreateToolbarEx(hwnd, ws, wid, nbitmaps, hbminst, wbmid, lpbuttons, inumbuttons, dxbutton, dybutton, dxbitmap, dybitmap, ustructsize) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateUpDownControl(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: super::HWND, nid: i32, hinst: super::HINSTANCE, hbuddy: super::HWND, nupper: i32, nlower: i32, npos: i32) -> super::HWND {
    windows_core::link!("comctl32.dll" "system" fn CreateUpDownControl(dwstyle : u32, x : i32, y : i32, cx : i32, cy : i32, hparent : super::HWND, nid : i32, hinst : super::HINSTANCE, hbuddy : super::HWND, nupper : i32, nlower : i32, npos : i32) -> super::HWND);
    unsafe { CreateUpDownControl(dwstyle, x, y, cx, cy, hparent, nid, hinst, hbuddy, nupper, nlower, npos) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefSubclassProc(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("comctl32.dll" "system" fn DefSubclassProc(hwnd : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefSubclassProc(hwnd, umsg, wparam, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawInsert(handparent: super::HWND, hlb: super::HWND, nitem: i32) {
    windows_core::link!("comctl32.dll" "system" fn DrawInsert(handparent : super::HWND, hlb : super::HWND, nitem : i32));
    unsafe { DrawInsert(handparent, hlb, nitem) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawShadowText(hdc: super::HDC, psztext: &[u16], prc: *const super::RECT, dwflags: u32, crtext: super::COLORREF, crshadow: super::COLORREF, ixoffset: i32, iyoffset: i32) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DrawShadowText(hdc : super::HDC, psztext : windows_core::PCWSTR, cch : u32, prc : *const super::RECT, dwflags : u32, crtext : super::COLORREF, crshadow : super::COLORREF, ixoffset : i32, iyoffset : i32) -> i32);
    unsafe { DrawShadowText(hdc, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), prc, dwflags, crtext, crshadow, ixoffset, iyoffset) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawStatusTextA<P2>(hdc: super::HDC, lprc: *const super::RECT, psztext: P2, uflags: u32)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn DrawStatusTextA(hdc : super::HDC, lprc : *const super::RECT, psztext : windows_core::PCSTR, uflags : u32));
    unsafe { DrawStatusTextA(hdc, lprc, psztext.param().abi(), uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawStatusTextW<P2>(hdc: super::HDC, lprc: *const super::RECT, psztext: P2, uflags: u32)
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn DrawStatusTextW(hdc : super::HDC, lprc : *const super::RECT, psztext : windows_core::PCWSTR, uflags : u32));
    unsafe { DrawStatusTextW(hdc, lprc, psztext.param().abi(), uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_EnableScrollBar(param0: super::HWND, param1: i32, param2: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_EnableScrollBar(param0 : super::HWND, param1 : i32, param2 : u32) -> windows_core::BOOL);
    unsafe { FlatSB_EnableScrollBar(param0, param1, param2) }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn FlatSB_GetScrollInfo(param0: super::HWND, code: i32, param2: *mut super::SCROLLINFO) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollInfo(param0 : super::HWND, code : i32, param2 : *mut super::SCROLLINFO) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollInfo(param0, code, param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_GetScrollPos(param0: super::HWND, code: i32) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollPos(param0 : super::HWND, code : i32) -> i32);
    unsafe { FlatSB_GetScrollPos(param0, code) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_GetScrollProp(param0: super::HWND, propindex: i32, param2: *mut i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollProp(param0 : super::HWND, propindex : i32, param2 : *mut i32) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollProp(param0, propindex, param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_GetScrollRange(param0: super::HWND, code: i32, param2: *mut i32, param3: *mut i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollRange(param0 : super::HWND, code : i32, param2 : *mut i32, param3 : *mut i32) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollRange(param0, code, param2 as _, param3 as _) }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn FlatSB_SetScrollInfo(param0: super::HWND, code: i32, psi: *mut super::SCROLLINFO, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollInfo(param0 : super::HWND, code : i32, psi : *mut super::SCROLLINFO, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollInfo(param0, code, psi as _, fredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_SetScrollPos(param0: super::HWND, code: i32, pos: i32, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollPos(param0 : super::HWND, code : i32, pos : i32, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollPos(param0, code, pos, fredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_SetScrollProp(param0: super::HWND, index: u32, newvalue: isize, param3: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollProp(param0 : super::HWND, index : u32, newvalue : isize, param3 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FlatSB_SetScrollProp(param0, index, newvalue, param3.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_SetScrollRange(param0: super::HWND, code: i32, min: i32, max: i32, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollRange(param0 : super::HWND, code : i32, min : i32, max : i32, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollRange(param0, code, min, max, fredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_ShowScrollBar(param0: super::HWND, code: i32, param2: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_ShowScrollBar(param0 : super::HWND, code : i32, param2 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FlatSB_ShowScrollBar(param0, code, param2.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetEffectiveClientRect(hwnd: super::HWND, lprc: *mut super::RECT, lpinfo: *const i32) {
    windows_core::link!("comctl32.dll" "system" fn GetEffectiveClientRect(hwnd : super::HWND, lprc : *mut super::RECT, lpinfo : *const i32));
    unsafe { GetEffectiveClientRect(hwnd, lprc as _, lpinfo) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetMUILanguage() -> super::LANGID {
    windows_core::link!("comctl32.dll" "system" fn GetMUILanguage() -> super::LANGID);
    unsafe { GetMUILanguage() }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetWindowSubclass(hwnd: super::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize, pdwrefdata: Option<*mut usize>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn GetWindowSubclass(hwnd : super::HWND, pfnsubclass : SUBCLASSPROC, uidsubclass : usize, pdwrefdata : *mut usize) -> windows_core::BOOL);
    unsafe { GetWindowSubclass(hwnd, pfnsubclass, uidsubclass, pdwrefdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HIMAGELIST_QueryInterface<T>(himl: *const _IMAGELIST) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("comctl32.dll" "system" fn HIMAGELIST_QueryInterface(himl : *const _IMAGELIST, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { HIMAGELIST_QueryInterface(himl, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_Add(himl: *const _IMAGELIST, hbmimage: super::HBITMAP, hbmmask: Option<super::HBITMAP>) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Add(himl : *const _IMAGELIST, hbmimage : super::HBITMAP, hbmmask : super::HBITMAP) -> i32);
    unsafe { ImageList_Add(himl, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_AddMasked(himl: *const _IMAGELIST, hbmimage: super::HBITMAP, crmask: super::COLORREF) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_AddMasked(himl : *const _IMAGELIST, hbmimage : super::HBITMAP, crmask : super::COLORREF) -> i32);
    unsafe { ImageList_AddMasked(himl, hbmimage, crmask) }
}
#[inline]
pub unsafe fn ImageList_BeginDrag(himltrack: *const _IMAGELIST, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_BeginDrag(himltrack : *const _IMAGELIST, itrack : i32, dxhotspot : i32, dyhotspot : i32) -> windows_core::BOOL);
    unsafe { ImageList_BeginDrag(himltrack, itrack, dxhotspot, dyhotspot) }
}
#[inline]
pub unsafe fn ImageList_Copy(himldst: *const _IMAGELIST, idst: i32, himlsrc: *const _IMAGELIST, isrc: i32, uflags: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Copy(himldst : *const _IMAGELIST, idst : i32, himlsrc : *const _IMAGELIST, isrc : i32, uflags : u32) -> windows_core::BOOL);
    unsafe { ImageList_Copy(himldst, idst, himlsrc, isrc, uflags) }
}
#[inline]
pub unsafe fn ImageList_Create(cx: i32, cy: i32, flags: u32, cinitial: i32, cgrow: i32) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Create(cx : i32, cy : i32, flags : u32, cinitial : i32, cgrow : i32) -> HIMAGELIST);
    unsafe { ImageList_Create(cx, cy, flags, cinitial, cgrow) }
}
#[inline]
pub unsafe fn ImageList_Destroy(himl: Option<*const _IMAGELIST>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Destroy(himl : *const _IMAGELIST) -> windows_core::BOOL);
    unsafe { ImageList_Destroy(himl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_DragEnter(hwndlock: super::HWND, x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragEnter(hwndlock : super::HWND, x : i32, y : i32) -> windows_core::BOOL);
    unsafe { ImageList_DragEnter(hwndlock, x, y) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_DragLeave(hwndlock: super::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragLeave(hwndlock : super::HWND) -> windows_core::BOOL);
    unsafe { ImageList_DragLeave(hwndlock) }
}
#[inline]
pub unsafe fn ImageList_DragMove(x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragMove(x : i32, y : i32) -> windows_core::BOOL);
    unsafe { ImageList_DragMove(x, y) }
}
#[inline]
pub unsafe fn ImageList_DragShowNolock(fshow: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragShowNolock(fshow : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ImageList_DragShowNolock(fshow.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_Draw(himl: *const _IMAGELIST, i: i32, hdcdst: super::HDC, x: i32, y: i32, fstyle: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Draw(himl : *const _IMAGELIST, i : i32, hdcdst : super::HDC, x : i32, y : i32, fstyle : u32) -> windows_core::BOOL);
    unsafe { ImageList_Draw(himl, i, hdcdst, x, y, fstyle) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_DrawEx(himl: *const _IMAGELIST, i: i32, hdcdst: super::HDC, x: i32, y: i32, dx: i32, dy: i32, rgbbk: super::COLORREF, rgbfg: super::COLORREF, fstyle: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DrawEx(himl : *const _IMAGELIST, i : i32, hdcdst : super::HDC, x : i32, y : i32, dx : i32, dy : i32, rgbbk : super::COLORREF, rgbfg : super::COLORREF, fstyle : u32) -> windows_core::BOOL);
    unsafe { ImageList_DrawEx(himl, i, hdcdst, x, y, dx, dy, rgbbk, rgbfg, fstyle) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DrawIndirect(pimldp : *const IMAGELISTDRAWPARAMS) -> windows_core::BOOL);
    unsafe { ImageList_DrawIndirect(pimldp) }
}
#[inline]
pub unsafe fn ImageList_Duplicate(himl: *const _IMAGELIST) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Duplicate(himl : *const _IMAGELIST) -> HIMAGELIST);
    unsafe { ImageList_Duplicate(himl) }
}
#[inline]
pub unsafe fn ImageList_EndDrag() {
    windows_core::link!("comctl32.dll" "system" fn ImageList_EndDrag());
    unsafe { ImageList_EndDrag() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_GetBkColor(himl: *const _IMAGELIST) -> super::COLORREF {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetBkColor(himl : *const _IMAGELIST) -> super::COLORREF);
    unsafe { ImageList_GetBkColor(himl) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_GetDragImage(ppt: Option<*mut super::POINT>, ppthotspot: Option<*mut super::POINT>) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetDragImage(ppt : *mut super::POINT, ppthotspot : *mut super::POINT) -> HIMAGELIST);
    unsafe { ImageList_GetDragImage(ppt.unwrap_or(core::mem::zeroed()) as _, ppthotspot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_GetIcon(himl: *const _IMAGELIST, i: i32, flags: u32) -> super::HICON {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetIcon(himl : *const _IMAGELIST, i : i32, flags : u32) -> super::HICON);
    unsafe { ImageList_GetIcon(himl, i, flags) }
}
#[inline]
pub unsafe fn ImageList_GetIconSize(himl: *const _IMAGELIST, cx: Option<*mut i32>, cy: Option<*mut i32>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetIconSize(himl : *const _IMAGELIST, cx : *mut i32, cy : *mut i32) -> windows_core::BOOL);
    unsafe { ImageList_GetIconSize(himl, cx.unwrap_or(core::mem::zeroed()) as _, cy.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ImageList_GetImageCount(himl: *const _IMAGELIST) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetImageCount(himl : *const _IMAGELIST) -> i32);
    unsafe { ImageList_GetImageCount(himl) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_GetImageInfo(himl: *const _IMAGELIST, i: i32, pimageinfo: *mut IMAGEINFO) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetImageInfo(himl : *const _IMAGELIST, i : i32, pimageinfo : *mut IMAGEINFO) -> windows_core::BOOL);
    unsafe { ImageList_GetImageInfo(himl, i, pimageinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImageList_LoadImageA<P1>(hi: super::HINSTANCE, lpbmp: P1, cx: i32, cgrow: i32, crmask: super::COLORREF, utype: u32, uflags: u32) -> HIMAGELIST
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_LoadImageA(hi : super::HINSTANCE, lpbmp : windows_core::PCSTR, cx : i32, cgrow : i32, crmask : super::COLORREF, utype : u32, uflags : u32) -> HIMAGELIST);
    unsafe { ImageList_LoadImageA(hi, lpbmp.param().abi(), cx, cgrow, crmask, utype, uflags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImageList_LoadImageW<P1>(hi: super::HINSTANCE, lpbmp: P1, cx: i32, cgrow: i32, crmask: super::COLORREF, utype: u32, uflags: u32) -> HIMAGELIST
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_LoadImageW(hi : super::HINSTANCE, lpbmp : windows_core::PCWSTR, cx : i32, cgrow : i32, crmask : super::COLORREF, utype : u32, uflags : u32) -> HIMAGELIST);
    unsafe { ImageList_LoadImageW(hi, lpbmp.param().abi(), cx, cgrow, crmask, utype, uflags) }
}
#[inline]
pub unsafe fn ImageList_Merge(himl1: *const _IMAGELIST, i1: i32, himl2: *const _IMAGELIST, i2: i32, dx: i32, dy: i32) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Merge(himl1 : *const _IMAGELIST, i1 : i32, himl2 : *const _IMAGELIST, i2 : i32, dx : i32, dy : i32) -> HIMAGELIST);
    unsafe { ImageList_Merge(himl1, i1, himl2, i2, dx, dy) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn ImageList_Read<P0>(pstm: P0) -> HIMAGELIST
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_Read(pstm : *mut core::ffi::c_void) -> HIMAGELIST);
    unsafe { ImageList_Read(pstm.param().abi()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn ImageList_ReadEx<P1, T>(dwflags: u32, pstm: P1) -> windows_core::Result<T>
where
    P1: windows_core::Param<super::IStream>,
    T: windows_core::Interface,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_ReadEx(dwflags : u32, pstm : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { ImageList_ReadEx(dwflags, pstm.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn ImageList_Remove(himl: *const _IMAGELIST, i: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Remove(himl : *const _IMAGELIST, i : i32) -> windows_core::BOOL);
    unsafe { ImageList_Remove(himl, i) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_Replace(himl: *const _IMAGELIST, i: i32, hbmimage: super::HBITMAP, hbmmask: Option<super::HBITMAP>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Replace(himl : *const _IMAGELIST, i : i32, hbmimage : super::HBITMAP, hbmmask : super::HBITMAP) -> windows_core::BOOL);
    unsafe { ImageList_Replace(himl, i, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_ReplaceIcon(himl: *const _IMAGELIST, i: i32, hicon: super::HICON) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_ReplaceIcon(himl : *const _IMAGELIST, i : i32, hicon : super::HICON) -> i32);
    unsafe { ImageList_ReplaceIcon(himl, i, hicon) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_SetBkColor(himl: *const _IMAGELIST, clrbk: super::COLORREF) -> super::COLORREF {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetBkColor(himl : *const _IMAGELIST, clrbk : super::COLORREF) -> super::COLORREF);
    unsafe { ImageList_SetBkColor(himl, clrbk) }
}
#[inline]
pub unsafe fn ImageList_SetDragCursorImage(himldrag: *const _IMAGELIST, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetDragCursorImage(himldrag : *const _IMAGELIST, idrag : i32, dxhotspot : i32, dyhotspot : i32) -> windows_core::BOOL);
    unsafe { ImageList_SetDragCursorImage(himldrag, idrag, dxhotspot, dyhotspot) }
}
#[inline]
pub unsafe fn ImageList_SetIconSize(himl: *const _IMAGELIST, cx: i32, cy: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetIconSize(himl : *const _IMAGELIST, cx : i32, cy : i32) -> windows_core::BOOL);
    unsafe { ImageList_SetIconSize(himl, cx, cy) }
}
#[inline]
pub unsafe fn ImageList_SetImageCount(himl: *const _IMAGELIST, unewcount: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetImageCount(himl : *const _IMAGELIST, unewcount : u32) -> windows_core::BOOL);
    unsafe { ImageList_SetImageCount(himl, unewcount) }
}
#[inline]
pub unsafe fn ImageList_SetOverlayImage(himl: *const _IMAGELIST, iimage: i32, ioverlay: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetOverlayImage(himl : *const _IMAGELIST, iimage : i32, ioverlay : i32) -> windows_core::BOOL);
    unsafe { ImageList_SetOverlayImage(himl, iimage, ioverlay) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn ImageList_Write<P1>(himl: *const _IMAGELIST, pstm: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<super::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_Write(himl : *const _IMAGELIST, pstm : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImageList_Write(himl, pstm.param().abi()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn ImageList_WriteEx<P2>(himl: *const _IMAGELIST, dwflags: u32, pstm: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_WriteEx(himl : *const _IMAGELIST, dwflags : u32, pstm : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ImageList_WriteEx(himl, dwflags, pstm.param().abi()) }
}
#[inline]
pub unsafe fn InitCommonControls() {
    windows_core::link!("comctl32.dll" "system" fn InitCommonControls());
    unsafe { InitCommonControls() }
}
#[inline]
pub unsafe fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn InitCommonControlsEx(picce : *const INITCOMMONCONTROLSEX) -> windows_core::BOOL);
    unsafe { InitCommonControlsEx(picce) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitMUILanguage(uilang: super::LANGID) {
    windows_core::link!("comctl32.dll" "system" fn InitMUILanguage(uilang : super::LANGID));
    unsafe { InitMUILanguage(uilang) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InitializeFlatSB(param0: super::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn InitializeFlatSB(param0 : super::HWND) -> windows_core::BOOL);
    unsafe { InitializeFlatSB(param0) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LBItemFromPt(hlb: super::HWND, pt: super::POINT, bautoscroll: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn LBItemFromPt(hlb : super::HWND, pt : super::POINT, bautoscroll : windows_core::BOOL) -> i32);
    unsafe { LBItemFromPt(hlb, pt, bautoscroll.into()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadIconMetric<P1>(hinst: super::HINSTANCE, pszname: P1, lims: i32) -> windows_core::Result<super::HICON>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn LoadIconMetric(hinst : super::HINSTANCE, pszname : windows_core::PCWSTR, lims : i32, phico : *mut super::HICON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadIconMetric(hinst, pszname.param().abi(), lims, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadIconWithScaleDown<P1>(hinst: super::HINSTANCE, pszname: P1, cx: i32, cy: i32) -> windows_core::Result<super::HICON>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn LoadIconWithScaleDown(hinst : super::HINSTANCE, pszname : windows_core::PCWSTR, cx : i32, cy : i32, phico : *mut super::HICON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadIconWithScaleDown(hinst, pszname.param().abi(), cx, cy, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MakeDragList(hlb: super::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn MakeDragList(hlb : super::HWND) -> windows_core::BOOL);
    unsafe { MakeDragList(hlb) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn MenuHelp(umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM, hmainmenu: super::HMENU, hinst: super::HINSTANCE, hwndstatus: super::HWND, lpwids: *const u32) {
    windows_core::link!("comctl32.dll" "system" fn MenuHelp(umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM, hmainmenu : super::HMENU, hinst : super::HINSTANCE, hwndstatus : super::HWND, lpwids : *const u32));
    unsafe { MenuHelp(umsg, wparam, lparam, hmainmenu, hinst, hwndstatus, lpwids) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn RemoveWindowSubclass(hwnd: super::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn RemoveWindowSubclass(hwnd : super::HWND, pfnsubclass : SUBCLASSPROC, uidsubclass : usize) -> windows_core::BOOL);
    unsafe { RemoveWindowSubclass(hwnd, pfnsubclass, uidsubclass) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWindowSubclass(hwnd: super::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize, dwrefdata: usize) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn SetWindowSubclass(hwnd : super::HWND, pfnsubclass : SUBCLASSPROC, uidsubclass : usize, dwrefdata : usize) -> windows_core::BOOL);
    unsafe { SetWindowSubclass(hwnd, pfnsubclass, uidsubclass, dwrefdata) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShowHideMenuCtl(hwnd: super::HWND, uflags: usize, lpinfo: *const i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ShowHideMenuCtl(hwnd : super::HWND, uflags : usize, lpinfo : *const i32) -> windows_core::BOOL);
    unsafe { ShowHideMenuCtl(hwnd, uflags, lpinfo) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn TaskDialog<P2, P3, P4, P6>(hwndowner: Option<super::HWND>, hinstance: Option<super::HINSTANCE>, pszwindowtitle: P2, pszmaininstruction: P3, pszcontent: P4, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: P6, pnbutton: Option<*mut i32>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn TaskDialog(hwndowner : super::HWND, hinstance : super::HINSTANCE, pszwindowtitle : windows_core::PCWSTR, pszmaininstruction : windows_core::PCWSTR, pszcontent : windows_core::PCWSTR, dwcommonbuttons : TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon : windows_core::PCWSTR, pnbutton : *mut i32) -> windows_core::HRESULT);
    unsafe { TaskDialog(hwndowner.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, pszwindowtitle.param().abi(), pszmaininstruction.param().abi(), pszcontent.param().abi(), dwcommonbuttons, pszicon.param().abi(), pnbutton.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn TaskDialogIndirect(ptaskconfig: *const TASKDIALOGCONFIG, pnbutton: Option<*mut i32>, pnradiobutton: Option<*mut i32>, pfverificationflagchecked: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
    windows_core::link!("comctl32.dll" "system" fn TaskDialogIndirect(ptaskconfig : *const TASKDIALOGCONFIG, pnbutton : *mut i32, pnradiobutton : *mut i32, pfverificationflagchecked : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { TaskDialogIndirect(ptaskconfig, pnbutton.unwrap_or(core::mem::zeroed()) as _, pnradiobutton.unwrap_or(core::mem::zeroed()) as _, pfverificationflagchecked.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UninitializeFlatSB(param0: super::HWND) -> windows_core::HRESULT {
    windows_core::link!("comctl32.dll" "system" fn UninitializeFlatSB(param0 : super::HWND) -> windows_core::HRESULT);
    unsafe { UninitializeFlatSB(param0) }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn _TrackMouseEvent(lpeventtrack: *mut super::TRACKMOUSEEVENT) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn _TrackMouseEvent(lpeventtrack : *mut super::TRACKMOUSEEVENT) -> windows_core::BOOL);
    unsafe { _TrackMouseEvent(lpeventtrack as _) }
}
pub const ACM_ISPLAYING: i32 = 1128;
pub const ACM_OPEN: i32 = 1124;
pub const ACM_OPENA: i32 = 1124;
pub const ACM_OPENW: i32 = 1127;
pub const ACM_PLAY: i32 = 1125;
pub const ACM_STOP: i32 = 1126;
pub const ACN_START: i32 = 1;
pub const ACN_STOP: i32 = 2;
pub const ACS_AUTOPLAY: i32 = 4;
pub const ACS_CENTER: i32 = 1;
pub const ACS_TIMER: i32 = 8;
pub const ACS_TRANSPARENT: i32 = 2;
pub const ANIMATE_CLASSA: windows_core::PCSTR = windows_core::s!("SysAnimate32");
pub const ANIMATE_CLASSW: windows_core::PCWSTR = windows_core::w!("SysAnimate32");
pub const BCM_FIRST: i32 = 5632;
pub const BCM_GETIDEALSIZE: i32 = 5633;
pub const BCM_GETIMAGELIST: i32 = 5635;
pub const BCM_GETNOTE: i32 = 5642;
pub const BCM_GETNOTELENGTH: i32 = 5643;
pub const BCM_GETSPLITINFO: i32 = 5640;
pub const BCM_GETTEXTMARGIN: i32 = 5637;
pub const BCM_SETDROPDOWNSTATE: i32 = 5638;
pub const BCM_SETIMAGELIST: i32 = 5634;
pub const BCM_SETNOTE: i32 = 5641;
pub const BCM_SETSHIELD: i32 = 5644;
pub const BCM_SETSPLITINFO: i32 = 5639;
pub const BCM_SETTEXTMARGIN: i32 = 5636;
pub const BCN_DROPDOWN: u32 = 4294966048;
pub const BCN_FIRST: u32 = 4294966046;
pub const BCN_HOTITEMCHANGE: u32 = 4294966047;
pub const BCN_LAST: u32 = 4294965946;
pub const BCSIF_GLYPH: i32 = 1;
pub const BCSIF_IMAGE: i32 = 2;
pub const BCSIF_SIZE: i32 = 8;
pub const BCSIF_STYLE: i32 = 4;
pub const BCSS_ALIGNLEFT: i32 = 4;
pub const BCSS_IMAGE: i32 = 8;
pub const BCSS_NOSPLIT: i32 = 1;
pub const BCSS_STRETCH: i32 = 2;
pub const BST_DROPDOWNPUSHED: i32 = 1024;
pub const BST_HOT: i32 = 512;
pub const BS_COMMANDLINK: i32 = 14;
pub const BS_DEFCOMMANDLINK: i32 = 15;
pub const BS_DEFSPLITBUTTON: i32 = 13;
pub const BS_SPLITBUTTON: i32 = 12;
pub const BTNS_AUTOSIZE: i32 = 16;
pub const BTNS_BUTTON: i32 = 0;
pub const BTNS_CHECK: i32 = 2;
pub const BTNS_CHECKGROUP: i32 = 6;
pub const BTNS_DROPDOWN: i32 = 8;
pub const BTNS_GROUP: i32 = 4;
pub const BTNS_NOPREFIX: i32 = 32;
pub const BTNS_SEP: i32 = 1;
pub const BTNS_SHOWTEXT: i32 = 64;
pub const BTNS_WHOLEDROPDOWN: i32 = 128;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BUTTON_IMAGELIST {
    pub himl: HIMAGELIST,
    pub margin: super::RECT,
    pub uAlign: u32,
}
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: i32 = 3;
pub const BUTTON_IMAGELIST_ALIGN_CENTER: i32 = 4;
pub const BUTTON_IMAGELIST_ALIGN_LEFT: i32 = 0;
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: i32 = 1;
pub const BUTTON_IMAGELIST_ALIGN_TOP: i32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BUTTON_SPLITINFO {
    pub mask: u32,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: u32,
    pub size: super::SIZE,
}
pub const CBEIF_DI_SETITEM: i32 = 268435456;
pub const CBEIF_IMAGE: i32 = 2;
pub const CBEIF_INDENT: i32 = 16;
pub const CBEIF_LPARAM: i32 = 32;
pub const CBEIF_OVERLAY: i32 = 8;
pub const CBEIF_SELECTEDIMAGE: i32 = 4;
pub const CBEIF_TEXT: i32 = 1;
pub const CBEMAXSTRLEN: i32 = 260;
pub const CBEM_DELETEITEM: i32 = 324;
pub const CBEM_GETCOMBOCONTROL: i32 = 1030;
pub const CBEM_GETEDITCONTROL: i32 = 1031;
pub const CBEM_GETEXSTYLE: i32 = 1033;
pub const CBEM_GETEXTENDEDSTYLE: i32 = 1033;
pub const CBEM_GETIMAGELIST: i32 = 1027;
pub const CBEM_GETITEM: i32 = 1028;
pub const CBEM_GETITEMA: i32 = 1028;
pub const CBEM_GETITEMW: i32 = 1037;
pub const CBEM_GETUNICODEFORMAT: i32 = 8198;
pub const CBEM_HASEDITCHANGED: i32 = 1034;
pub const CBEM_INSERTITEM: i32 = 1025;
pub const CBEM_INSERTITEMA: i32 = 1025;
pub const CBEM_INSERTITEMW: i32 = 1035;
pub const CBEM_SETEXSTYLE: i32 = 1032;
pub const CBEM_SETEXTENDEDSTYLE: i32 = 1038;
pub const CBEM_SETIMAGELIST: i32 = 1026;
pub const CBEM_SETITEM: i32 = 1029;
pub const CBEM_SETITEMA: i32 = 1029;
pub const CBEM_SETITEMW: i32 = 1036;
pub const CBEM_SETUNICODEFORMAT: i32 = 8197;
pub const CBEM_SETWINDOWTHEME: i32 = 8203;
pub const CBENF_DROPDOWN: i32 = 4;
pub const CBENF_ESCAPE: i32 = 3;
pub const CBENF_KILLFOCUS: i32 = 1;
pub const CBENF_RETURN: i32 = 2;
pub const CBEN_BEGINEDIT: u32 = 4294966492;
pub const CBEN_DELETEITEM: u32 = 4294966494;
pub const CBEN_DRAGBEGIN: u32 = 4294966488;
pub const CBEN_DRAGBEGINA: u32 = 4294966488;
pub const CBEN_DRAGBEGINW: u32 = 4294966487;
pub const CBEN_ENDEDIT: u32 = 4294966491;
pub const CBEN_ENDEDITA: u32 = 4294966491;
pub const CBEN_ENDEDITW: u32 = 4294966490;
pub const CBEN_FIRST: u32 = 4294966496;
pub const CBEN_GETDISPINFO: u32 = 4294966496;
pub const CBEN_GETDISPINFOA: u32 = 4294966496;
pub const CBEN_GETDISPINFOW: u32 = 4294966489;
pub const CBEN_INSERTITEM: u32 = 4294966495;
pub const CBEN_LAST: u32 = 4294966466;
pub const CBES_EX_CASESENSITIVE: i32 = 16;
pub const CBES_EX_NOEDITIMAGE: i32 = 1;
pub const CBES_EX_NOEDITIMAGEINDENT: i32 = 2;
pub const CBES_EX_NOSIZELIMIT: i32 = 8;
pub const CBES_EX_PATHWORDBREAKPROC: i32 = 4;
pub const CBES_EX_TEXTENDELLIPSIS: i32 = 32;
pub const CBM_FIRST: i32 = 5888;
pub const CB_GETCUEBANNER: i32 = 5892;
pub const CB_GETMINVISIBLE: i32 = 5890;
pub const CB_SETCUEBANNER: i32 = 5891;
pub const CB_SETMINVISIBLE: i32 = 5889;
pub const CCM_DPISCALE: i32 = 8204;
pub const CCM_FIRST: i32 = 8192;
pub const CCM_GETCOLORSCHEME: i32 = 8195;
pub const CCM_GETDROPTARGET: i32 = 8196;
pub const CCM_GETUNICODEFORMAT: i32 = 8198;
pub const CCM_GETVERSION: i32 = 8200;
pub const CCM_LAST: i32 = 8704;
pub const CCM_SETBKCOLOR: i32 = 8193;
pub const CCM_SETCOLORSCHEME: i32 = 8194;
pub const CCM_SETNOTIFYWINDOW: i32 = 8201;
pub const CCM_SETUNICODEFORMAT: i32 = 8197;
pub const CCM_SETVERSION: i32 = 8199;
pub const CCM_SETWINDOWTHEME: i32 = 8203;
pub const CCS_ADJUSTABLE: i32 = 32;
pub const CCS_BOTTOM: i32 = 3;
pub const CCS_LEFT: i32 = 129;
pub const CCS_NODIVIDER: i32 = 64;
pub const CCS_NOMOVEX: i32 = 130;
pub const CCS_NOMOVEY: i32 = 2;
pub const CCS_NOPARENTALIGN: i32 = 8;
pub const CCS_NORESIZE: i32 = 4;
pub const CCS_RIGHT: i32 = 131;
pub const CCS_TOP: i32 = 1;
pub const CCS_VERT: i32 = 128;
pub const CDDS_ITEM: i32 = 65536;
pub const CDDS_ITEMPOSTERASE: i32 = 65540;
pub const CDDS_ITEMPOSTPAINT: i32 = 65538;
pub const CDDS_ITEMPREERASE: i32 = 65539;
pub const CDDS_ITEMPREPAINT: i32 = 65537;
pub const CDDS_POSTERASE: i32 = 4;
pub const CDDS_POSTPAINT: i32 = 2;
pub const CDDS_PREERASE: i32 = 3;
pub const CDDS_PREPAINT: i32 = 1;
pub const CDDS_SUBITEM: i32 = 131072;
pub const CDIS_CHECKED: i32 = 8;
pub const CDIS_DEFAULT: i32 = 32;
pub const CDIS_DISABLED: i32 = 4;
pub const CDIS_DROPHILITED: i32 = 4096;
pub const CDIS_FOCUS: i32 = 16;
pub const CDIS_GRAYED: i32 = 2;
pub const CDIS_HOT: i32 = 64;
pub const CDIS_INDETERMINATE: i32 = 256;
pub const CDIS_MARKED: i32 = 128;
pub const CDIS_NEARHOT: i32 = 1024;
pub const CDIS_OTHERSIDEHOT: i32 = 2048;
pub const CDIS_SELECTED: i32 = 1;
pub const CDIS_SHOWKEYBOARDCUES: i32 = 512;
pub const CDRF_DODEFAULT: i32 = 0;
pub const CDRF_DOERASE: i32 = 8;
pub const CDRF_NEWFONT: i32 = 2;
pub const CDRF_NOTIFYITEMDRAW: i32 = 32;
pub const CDRF_NOTIFYPOSTERASE: i32 = 64;
pub const CDRF_NOTIFYPOSTPAINT: i32 = 16;
pub const CDRF_NOTIFYSUBITEMDRAW: i32 = 32;
pub const CDRF_SKIPDEFAULT: i32 = 4;
pub const CDRF_SKIPPOSTPAINT: i32 = 256;
pub const CLR_DEFAULT: u32 = 4278190080;
pub const CLR_HILIGHT: u32 = 4278190080;
pub const CLR_NONE: u32 = 4294967295;
pub const CMB_MASKED: i32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORMAP {
    pub from: super::COLORREF,
    pub to: super::COLORREF,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORSCHEME {
    pub dwSize: u32,
    pub clrBtnHighlight: super::COLORREF,
    pub clrBtnShadow: super::COLORREF,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMBOBOXEXITEMA {
    pub mask: u32,
    pub iItem: isize,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMBOBOXEXITEMW {
    pub mask: u32,
    pub iItem: isize,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::LPARAM,
}
pub const COMCTL32_VERSION: i32 = 6;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DATETIMEPICKERINFO {
    pub cbSize: u32,
    pub rcCheck: super::RECT,
    pub stateCheck: u32,
    pub rcButton: super::RECT,
    pub stateButton: u32,
    pub hwndEdit: super::HWND,
    pub hwndUD: super::HWND,
    pub hwndDropDown: super::HWND,
}
pub const DATETIMEPICK_CLASSA: windows_core::PCSTR = windows_core::s!("SysDateTimePick32");
pub const DATETIMEPICK_CLASSW: windows_core::PCWSTR = windows_core::w!("SysDateTimePick32");
pub const DL_BEGINDRAG: i32 = 1157;
pub const DL_CANCELDRAG: i32 = 1160;
pub const DL_COPYCURSOR: i32 = 2;
pub const DL_CURSORSET: i32 = 0;
pub const DL_DRAGGING: i32 = 1158;
pub const DL_DROPPED: i32 = 1159;
pub const DL_MOVECURSOR: i32 = 3;
pub const DL_STOPCURSOR: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAGLISTINFO {
    pub uNotification: u32,
    pub hWnd: super::HWND,
    pub ptCursor: super::POINT,
}
pub const DTM_CLOSEMONTHCAL: i32 = 4109;
pub const DTM_FIRST: i32 = 4096;
pub const DTM_GETDATETIMEPICKERINFO: i32 = 4110;
pub const DTM_GETIDEALSIZE: i32 = 4111;
pub const DTM_GETMCCOLOR: i32 = 4103;
pub const DTM_GETMCFONT: i32 = 4106;
pub const DTM_GETMCSTYLE: i32 = 4108;
pub const DTM_GETMONTHCAL: i32 = 4104;
pub const DTM_GETRANGE: i32 = 4099;
pub const DTM_GETSYSTEMTIME: i32 = 4097;
pub const DTM_SETFORMAT: i32 = 4101;
pub const DTM_SETFORMATA: i32 = 4101;
pub const DTM_SETFORMATW: i32 = 4146;
pub const DTM_SETMCCOLOR: i32 = 4102;
pub const DTM_SETMCFONT: i32 = 4105;
pub const DTM_SETMCSTYLE: i32 = 4107;
pub const DTM_SETRANGE: i32 = 4100;
pub const DTM_SETSYSTEMTIME: i32 = 4098;
pub const DTN_CLOSEUP: u32 = 4294966543;
pub const DTN_DATETIMECHANGE: u32 = 4294966537;
pub const DTN_DROPDOWN: u32 = 4294966542;
pub const DTN_FIRST: u32 = 4294966556;
pub const DTN_FIRST2: u32 = 4294966543;
pub const DTN_FORMAT: u32 = 4294966540;
pub const DTN_FORMATA: u32 = 4294966540;
pub const DTN_FORMATQUERY: u32 = 4294966541;
pub const DTN_FORMATQUERYA: u32 = 4294966541;
pub const DTN_FORMATQUERYW: u32 = 4294966554;
pub const DTN_FORMATW: u32 = 4294966553;
pub const DTN_LAST: u32 = 4294966551;
pub const DTN_LAST2: u32 = 4294966497;
pub const DTN_USERSTRING: u32 = 4294966538;
pub const DTN_USERSTRINGA: u32 = 4294966538;
pub const DTN_USERSTRINGW: u32 = 4294966551;
pub const DTN_WMKEYDOWN: u32 = 4294966539;
pub const DTN_WMKEYDOWNA: u32 = 4294966539;
pub const DTN_WMKEYDOWNW: u32 = 4294966552;
pub const DTS_APPCANPARSE: i32 = 16;
pub const DTS_LONGDATEFORMAT: i32 = 4;
pub const DTS_RIGHTALIGN: i32 = 32;
pub const DTS_SHORTDATECENTURYFORMAT: i32 = 12;
pub const DTS_SHORTDATEFORMAT: i32 = 0;
pub const DTS_SHOWNONE: i32 = 2;
pub const DTS_TIMEFORMAT: i32 = 9;
pub const DTS_UPDOWN: i32 = 1;
pub const ECM_FIRST: i32 = 5376;
pub type EC_ENDOFLINE = i32;
pub const EC_ENDOFLINE_CR: EC_ENDOFLINE = 2;
pub const EC_ENDOFLINE_CRLF: EC_ENDOFLINE = 1;
pub const EC_ENDOFLINE_DETECTFROMCONTENT: EC_ENDOFLINE = 0;
pub const EC_ENDOFLINE_LF: EC_ENDOFLINE = 3;
pub type EC_SEARCHWEB_ENTRYPOINT = i32;
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: EC_SEARCHWEB_ENTRYPOINT = 1;
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: EC_SEARCHWEB_ENTRYPOINT = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITBALLOONTIP {
    pub cbStruct: u32,
    pub pszTitle: windows_core::PCWSTR,
    pub pszText: windows_core::PCWSTR,
    pub ttiIcon: i32,
}
pub const EMF_CENTERED: i32 = 1;
pub const EM_ENABLESEARCHWEB: i32 = 5390;
pub const EM_FILELINEFROMCHAR: i32 = 5395;
pub const EM_FILELINEINDEX: i32 = 5396;
pub const EM_FILELINELENGTH: i32 = 5397;
pub const EM_GETCARETINDEX: i32 = 5394;
pub const EM_GETCUEBANNER: i32 = 5378;
pub const EM_GETENDOFLINE: i32 = 5389;
pub const EM_GETEXTENDEDSTYLE: i32 = 5387;
pub const EM_GETFILELINE: i32 = 5398;
pub const EM_GETFILELINECOUNT: i32 = 5399;
pub const EM_GETHILITE: i32 = 5382;
pub const EM_GETZOOM: i32 = 1248;
pub const EM_HIDEBALLOONTIP: i32 = 5380;
pub const EM_NOSETFOCUS: i32 = 5383;
pub const EM_SEARCHWEB: i32 = 5391;
pub const EM_SETCARETINDEX: i32 = 5393;
pub const EM_SETCUEBANNER: i32 = 5377;
pub const EM_SETENDOFLINE: i32 = 5388;
pub const EM_SETEXTENDEDSTYLE: i32 = 5386;
pub const EM_SETHILITE: i32 = 5381;
pub const EM_SETZOOM: i32 = 1249;
pub const EM_SHOWBALLOONTIP: i32 = 5379;
pub const EM_TAKEFOCUS: i32 = 5384;
pub const EN_FIRST: u32 = 4294965776;
pub const EN_LAST: u32 = 4294965756;
pub const EN_SEARCHWEB: u32 = 4294965776;
pub const ES_EX_ALLOWEOL_ALL: i32 = 3;
pub const ES_EX_ALLOWEOL_CR: i32 = 1;
pub const ES_EX_ALLOWEOL_LF: i32 = 2;
pub const ES_EX_CONVERT_EOL_ON_PASTE: i32 = 4;
pub const ES_EX_ZOOMABLE: i32 = 16;
pub const FSB_ENCARTA_MODE: i32 = 1;
pub const FSB_FLAT_MODE: i32 = 2;
pub const FSB_REGULAR_MODE: i32 = 0;
pub const GDTR_MAX: i32 = 2;
pub const GDTR_MIN: i32 = 1;
pub const GDT_ERROR: i32 = -1;
pub const GDT_NONE: i32 = 1;
pub const GDT_VALID: i32 = 0;
pub const GMR_DAYSTATE: i32 = 1;
pub const GMR_VISIBLE: i32 = 0;
#[cfg(feature = "windef")]
pub const HBITMAP_CALLBACK: super::HBITMAP = super::HBITMAP(-1 as _);
pub const HDFT_HASNOVALUE: i32 = 32768;
pub const HDFT_ISDATE: i32 = 2;
pub const HDFT_ISNUMBER: i32 = 1;
pub const HDFT_ISSTRING: i32 = 0;
pub const HDF_BITMAP: i32 = 8192;
pub const HDF_BITMAP_ON_RIGHT: i32 = 4096;
pub const HDF_CENTER: i32 = 2;
pub const HDF_CHECKBOX: i32 = 64;
pub const HDF_CHECKED: i32 = 128;
pub const HDF_FIXEDWIDTH: i32 = 256;
pub const HDF_IMAGE: i32 = 2048;
pub const HDF_JUSTIFYMASK: i32 = 3;
pub const HDF_LEFT: i32 = 0;
pub const HDF_OWNERDRAW: i32 = 32768;
pub const HDF_RIGHT: i32 = 1;
pub const HDF_RTLREADING: i32 = 4;
pub const HDF_SORTDOWN: i32 = 512;
pub const HDF_SORTUP: i32 = 1024;
pub const HDF_SPLITBUTTON: i32 = 16777216;
pub const HDF_STRING: i32 = 16384;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HDHITTESTINFO {
    pub pt: super::POINT,
    pub flags: u32,
    pub iItem: i32,
}
pub const HDIS_FOCUSED: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HDITEMA {
    pub mask: u32,
    pub cxy: i32,
    pub pszText: windows_core::PSTR,
    pub hbm: super::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: super::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: u32,
    pub pvFilter: *mut core::ffi::c_void,
    pub state: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for HDITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const HDITEMA_V1_SIZE: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const HDITEMA_V1_SIZE: u64 = 40;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HDITEMW {
    pub mask: u32,
    pub cxy: i32,
    pub pszText: windows_core::PWSTR,
    pub hbm: super::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: super::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: u32,
    pub pvFilter: *mut core::ffi::c_void,
    pub state: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for HDITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const HDITEMW_V1_SIZE: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const HDITEMW_V1_SIZE: u64 = 40;
#[cfg(target_arch = "x86")]
pub const HDITEM_V1_SIZE: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const HDITEM_V1_SIZE: u64 = 40;
pub const HDI_BITMAP: i32 = 16;
pub const HDI_DI_SETITEM: i32 = 64;
pub const HDI_FILTER: i32 = 256;
pub const HDI_FORMAT: i32 = 4;
pub const HDI_HEIGHT: i32 = 1;
pub const HDI_IMAGE: i32 = 32;
pub const HDI_LPARAM: i32 = 8;
pub const HDI_ORDER: i32 = 128;
pub const HDI_STATE: i32 = 512;
pub const HDI_TEXT: i32 = 2;
pub const HDI_WIDTH: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HDLAYOUT {
    pub prc: *mut super::RECT,
    pub pwpos: *mut super::WINDOWPOS,
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for HDLAYOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HDM_CLEARFILTER: i32 = 4632;
pub const HDM_CREATEDRAGIMAGE: i32 = 4624;
pub const HDM_DELETEITEM: i32 = 4610;
pub const HDM_EDITFILTER: i32 = 4631;
pub const HDM_FIRST: i32 = 4608;
pub const HDM_GETBITMAPMARGIN: i32 = 4629;
pub const HDM_GETFOCUSEDITEM: i32 = 4635;
pub const HDM_GETIMAGELIST: i32 = 4617;
pub const HDM_GETITEM: i32 = 4611;
pub const HDM_GETITEMA: i32 = 4611;
pub const HDM_GETITEMCOUNT: i32 = 4608;
pub const HDM_GETITEMDROPDOWNRECT: i32 = 4633;
pub const HDM_GETITEMRECT: i32 = 4615;
pub const HDM_GETITEMW: i32 = 4619;
pub const HDM_GETORDERARRAY: i32 = 4625;
pub const HDM_GETOVERFLOWRECT: i32 = 4634;
pub const HDM_GETUNICODEFORMAT: i32 = 8198;
pub const HDM_HITTEST: i32 = 4614;
pub const HDM_INSERTITEM: i32 = 4609;
pub const HDM_INSERTITEMA: i32 = 4609;
pub const HDM_INSERTITEMW: i32 = 4618;
pub const HDM_LAYOUT: i32 = 4613;
pub const HDM_ORDERTOINDEX: i32 = 4623;
pub const HDM_SETBITMAPMARGIN: i32 = 4628;
pub const HDM_SETFILTERCHANGETIMEOUT: i32 = 4630;
pub const HDM_SETFOCUSEDITEM: i32 = 4636;
pub const HDM_SETHOTDIVIDER: i32 = 4627;
pub const HDM_SETIMAGELIST: i32 = 4616;
pub const HDM_SETITEM: i32 = 4612;
pub const HDM_SETITEMA: i32 = 4612;
pub const HDM_SETITEMW: i32 = 4620;
pub const HDM_SETORDERARRAY: i32 = 4626;
pub const HDM_SETUNICODEFORMAT: i32 = 8197;
pub const HDN_BEGINDRAG: u32 = 4294966986;
pub const HDN_BEGINFILTEREDIT: u32 = 4294966982;
pub const HDN_BEGINTRACK: u32 = 4294966990;
pub const HDN_BEGINTRACKA: u32 = 4294966990;
pub const HDN_BEGINTRACKW: u32 = 4294966970;
pub const HDN_DIVIDERDBLCLICK: u32 = 4294966991;
pub const HDN_DIVIDERDBLCLICKA: u32 = 4294966991;
pub const HDN_DIVIDERDBLCLICKW: u32 = 4294966971;
pub const HDN_DROPDOWN: u32 = 4294966978;
pub const HDN_ENDDRAG: u32 = 4294966985;
pub const HDN_ENDFILTEREDIT: u32 = 4294966981;
pub const HDN_ENDTRACK: u32 = 4294966989;
pub const HDN_ENDTRACKA: u32 = 4294966989;
pub const HDN_ENDTRACKW: u32 = 4294966969;
pub const HDN_FILTERBTNCLICK: u32 = 4294966983;
pub const HDN_FILTERCHANGE: u32 = 4294966984;
pub const HDN_FIRST: u32 = 4294966996;
pub const HDN_GETDISPINFO: u32 = 4294966987;
pub const HDN_GETDISPINFOA: u32 = 4294966987;
pub const HDN_GETDISPINFOW: u32 = 4294966967;
pub const HDN_ITEMCHANGED: u32 = 4294966995;
pub const HDN_ITEMCHANGEDA: u32 = 4294966995;
pub const HDN_ITEMCHANGEDW: u32 = 4294966975;
pub const HDN_ITEMCHANGING: u32 = 4294966996;
pub const HDN_ITEMCHANGINGA: u32 = 4294966996;
pub const HDN_ITEMCHANGINGW: u32 = 4294966976;
pub const HDN_ITEMCLICK: u32 = 4294966994;
pub const HDN_ITEMCLICKA: u32 = 4294966994;
pub const HDN_ITEMCLICKW: u32 = 4294966974;
pub const HDN_ITEMDBLCLICK: u32 = 4294966993;
pub const HDN_ITEMDBLCLICKA: u32 = 4294966993;
pub const HDN_ITEMDBLCLICKW: u32 = 4294966973;
pub const HDN_ITEMKEYDOWN: u32 = 4294966979;
pub const HDN_ITEMSTATEICONCLICK: u32 = 4294966980;
pub const HDN_LAST: u32 = 4294966897;
pub const HDN_OVERFLOWCLICK: u32 = 4294966977;
pub const HDN_TRACK: u32 = 4294966988;
pub const HDN_TRACKA: u32 = 4294966988;
pub const HDN_TRACKW: u32 = 4294966968;
pub const HDSIL_NORMAL: i32 = 0;
pub const HDSIL_STATE: i32 = 1;
pub const HDS_BUTTONS: i32 = 2;
pub const HDS_CHECKBOXES: i32 = 1024;
pub const HDS_DRAGDROP: i32 = 64;
pub const HDS_FILTERBAR: i32 = 256;
pub const HDS_FLAT: i32 = 512;
pub const HDS_FULLDRAG: i32 = 128;
pub const HDS_HIDDEN: i32 = 8;
pub const HDS_HORZ: i32 = 0;
pub const HDS_HOTTRACK: i32 = 4;
pub const HDS_NOSIZING: i32 = 2048;
pub const HDS_OVERFLOW: i32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HD_TEXTFILTERA {
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HD_TEXTFILTERW {
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
}
pub const HHT_ABOVE: i32 = 256;
pub const HHT_BELOW: i32 = 512;
pub const HHT_NOWHERE: i32 = 1;
pub const HHT_ONDIVIDER: i32 = 4;
pub const HHT_ONDIVOPEN: i32 = 8;
pub const HHT_ONDROPDOWN: i32 = 8192;
pub const HHT_ONFILTER: i32 = 16;
pub const HHT_ONFILTERBUTTON: i32 = 32;
pub const HHT_ONHEADER: i32 = 2;
pub const HHT_ONITEMSTATEICON: i32 = 4096;
pub const HHT_ONOVERFLOW: i32 = 16384;
pub const HHT_TOLEFT: i32 = 2048;
pub const HHT_TORIGHT: i32 = 1024;
pub const HICF_ACCELERATOR: i32 = 4;
pub const HICF_ARROWKEYS: i32 = 2;
pub const HICF_DUPACCEL: i32 = 8;
pub const HICF_ENTERING: i32 = 16;
pub const HICF_LEAVING: i32 = 32;
pub const HICF_LMOUSE: i32 = 128;
pub const HICF_MOUSE: i32 = 1;
pub const HICF_OTHER: i32 = 0;
pub const HICF_RESELECT: i32 = 64;
pub const HICF_TOGGLEDROPDOWN: i32 = 256;
pub type HIMAGELIST = *mut _IMAGELIST;
#[cfg(feature = "minwindef")]
pub const HINST_COMMCTRL: super::HINSTANCE = super::HINSTANCE(-1 as _);
pub const HIST_ADDTOFAVORITES: i32 = 3;
pub const HIST_BACK: i32 = 0;
pub const HIST_FAVORITES: i32 = 2;
pub const HIST_FORWARD: i32 = 1;
pub const HIST_VIEWTREE: i32 = 4;
pub const HKCOMB_A: i32 = 8;
pub const HKCOMB_C: i32 = 4;
pub const HKCOMB_CA: i32 = 64;
pub const HKCOMB_NONE: i32 = 1;
pub const HKCOMB_S: i32 = 2;
pub const HKCOMB_SA: i32 = 32;
pub const HKCOMB_SC: i32 = 16;
pub const HKCOMB_SCA: i32 = 128;
pub const HKM_GETHOTKEY: i32 = 1026;
pub const HKM_SETHOTKEY: i32 = 1025;
pub const HKM_SETRULES: i32 = 1027;
pub const HOTKEYF_ALT: i32 = 4;
pub const HOTKEYF_CONTROL: i32 = 2;
pub const HOTKEYF_EXT: i32 = 8;
pub const HOTKEYF_SHIFT: i32 = 1;
pub const HOTKEY_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_hotkey32");
pub const HOTKEY_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_hotkey32");
pub type HTREEITEM = *mut _TREEITEM;
pub const ICC_ANIMATE_CLASS: i32 = 128;
pub const ICC_BAR_CLASSES: i32 = 4;
pub const ICC_COOL_CLASSES: i32 = 1024;
pub const ICC_DATE_CLASSES: i32 = 256;
pub const ICC_HOTKEY_CLASS: i32 = 64;
pub const ICC_INTERNET_CLASSES: i32 = 2048;
pub const ICC_LINK_CLASS: i32 = 32768;
pub const ICC_LISTVIEW_CLASSES: i32 = 1;
pub const ICC_NATIVEFNTCTL_CLASS: i32 = 8192;
pub const ICC_PAGESCROLLER_CLASS: i32 = 4096;
pub const ICC_PROGRESS_CLASS: i32 = 32;
pub const ICC_STANDARD_CLASSES: i32 = 16384;
pub const ICC_TAB_CLASSES: i32 = 8;
pub const ICC_TREEVIEW_CLASSES: i32 = 2;
pub const ICC_UPDOWN_CLASS: i32 = 16;
pub const ICC_USEREX_CLASSES: i32 = 512;
pub const ICC_WIN95_CLASSES: i32 = 255;
pub const IDB_HIST_DISABLED: i32 = 14;
pub const IDB_HIST_HOT: i32 = 13;
pub const IDB_HIST_LARGE_COLOR: i32 = 9;
pub const IDB_HIST_NORMAL: i32 = 12;
pub const IDB_HIST_PRESSED: i32 = 15;
pub const IDB_HIST_SMALL_COLOR: i32 = 8;
pub const IDB_STD_LARGE_COLOR: i32 = 1;
pub const IDB_STD_SMALL_COLOR: i32 = 0;
pub const IDB_VIEW_LARGE_COLOR: i32 = 5;
pub const IDB_VIEW_SMALL_COLOR: i32 = 4;
pub const ILCF_MOVE: i32 = 0;
pub const ILCF_SWAP: i32 = 1;
pub const ILC_COLOR: i32 = 0;
pub const ILC_COLOR16: i32 = 16;
pub const ILC_COLOR24: i32 = 24;
pub const ILC_COLOR32: i32 = 32;
pub const ILC_COLOR4: i32 = 4;
pub const ILC_COLOR8: i32 = 8;
pub const ILC_COLORDDB: i32 = 254;
pub const ILC_HIGHQUALITYSCALE: i32 = 131072;
pub const ILC_MASK: i32 = 1;
pub const ILC_MIRROR: i32 = 8192;
pub const ILC_ORIGINALSIZE: i32 = 65536;
pub const ILC_PALETTE: i32 = 2048;
pub const ILC_PERITEMMIRROR: i32 = 32768;
pub const ILD_ASYNC: i32 = 32768;
pub const ILD_BLEND: i32 = 4;
pub const ILD_BLEND25: i32 = 2;
pub const ILD_BLEND50: i32 = 4;
pub const ILD_DPISCALE: i32 = 16384;
pub const ILD_FOCUS: i32 = 2;
pub const ILD_IMAGE: i32 = 32;
pub const ILD_MASK: i32 = 16;
pub const ILD_NORMAL: i32 = 0;
pub const ILD_OVERLAYMASK: i32 = 3840;
pub const ILD_PRESERVEALPHA: i32 = 4096;
pub const ILD_ROP: i32 = 64;
pub const ILD_SCALE: i32 = 8192;
pub const ILD_SELECTED: i32 = 4;
pub const ILD_TRANSPARENT: i32 = 1;
pub const ILGT_ASYNC: i32 = 1;
pub const ILGT_NORMAL: i32 = 0;
pub const ILP_DOWNLEVEL: i32 = 1;
pub const ILP_NORMAL: i32 = 0;
pub const ILS_ALPHA: i32 = 8;
pub const ILS_GLOW: i32 = 1;
pub const ILS_NORMAL: i32 = 0;
pub const ILS_SATURATE: i32 = 4;
pub const ILS_SHADOW: i32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGEINFO {
    pub hbmImage: super::HBITMAP,
    pub hbmMask: super::HBITMAP,
    pub Unused1: i32,
    pub Unused2: i32,
    pub rcImage: super::RECT,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGELISTDRAWPARAMS {
    pub cbSize: u32,
    pub himl: HIMAGELIST,
    pub i: i32,
    pub hdcDst: super::HDC,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub xBitmap: i32,
    pub yBitmap: i32,
    pub rgbBk: super::COLORREF,
    pub rgbFg: super::COLORREF,
    pub fStyle: u32,
    pub dwRop: u32,
    pub fState: u32,
    pub Frame: u32,
    pub crEffect: super::COLORREF,
}
#[cfg(target_arch = "x86")]
pub const IMAGELISTDRAWPARAMS_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IMAGELISTDRAWPARAMS_V3_SIZE: u64 = 72;
pub const INFOTIPSIZE: i32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: u32,
}
pub const INVALID_LINK_INDEX: i32 = -1;
pub const IPM_CLEARADDRESS: i32 = 1124;
pub const IPM_GETADDRESS: i32 = 1126;
pub const IPM_ISBLANK: i32 = 1129;
pub const IPM_SETADDRESS: i32 = 1125;
pub const IPM_SETFOCUS: i32 = 1128;
pub const IPM_SETRANGE: i32 = 1127;
pub const IPN_FIELDCHANGED: u32 = 4294966436;
pub const IPN_FIRST: u32 = 4294966436;
pub const IPN_LAST: u32 = 4294966417;
pub const I_CHILDRENAUTO: i32 = -2;
pub const I_CHILDRENCALLBACK: i32 = -1;
pub const I_COLUMNSCALLBACK: u32 = 4294967295;
pub const I_GROUPIDCALLBACK: i32 = -1;
pub const I_GROUPIDNONE: i32 = -2;
pub const I_IMAGECALLBACK: i32 = -1;
pub const I_IMAGENONE: i32 = -2;
pub const I_INDENTCALLBACK: i32 = -1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LHITTESTINFO {
    pub pt: super::POINT,
    pub item: LITEM,
}
pub const LIF_ITEMID: i32 = 4;
pub const LIF_ITEMINDEX: i32 = 1;
pub const LIF_STATE: i32 = 2;
pub const LIF_URL: i32 = 8;
pub const LIM_LARGE: _LI_METRIC = 1;
pub const LIM_SMALL: _LI_METRIC = 0;
pub const LIS_DEFAULTCOLORS: i32 = 16;
pub const LIS_ENABLED: i32 = 2;
pub const LIS_FOCUSED: i32 = 1;
pub const LIS_HOTTRACK: i32 = 8;
pub const LIS_VISITED: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LITEM {
    pub mask: u32,
    pub iLink: i32,
    pub state: u32,
    pub stateMask: u32,
    pub szID: [u16; 48],
    pub szUrl: [u16; 2084],
}
impl Default for LITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LM_GETIDEALHEIGHT: i32 = 1793;
pub const LM_GETIDEALSIZE: i32 = 1793;
pub const LM_GETITEM: i32 = 1795;
pub const LM_HITTEST: i32 = 1792;
pub const LM_SETITEM: i32 = 1794;
#[cfg(feature = "windef")]
pub type LPCOLORMAP = *mut COLORMAP;
#[cfg(feature = "windef")]
pub type LPCOLORSCHEME = *mut COLORSCHEME;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCREBARBANDINFOA = *const REBARBANDINFOA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCREBARBANDINFOW = *const REBARBANDINFOW;
pub type LPCTBBUTTON = *const TBBUTTON;
#[cfg(feature = "windef")]
pub type LPDATETIMEPICKERINFO = *mut DATETIMEPICKERINFO;
#[cfg(feature = "windef")]
pub type LPDRAGLISTINFO = *mut DRAGLISTINFO;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFINDINFOA = *mut LVFINDINFOA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFINDINFOW = *mut LVFINDINFOW;
#[cfg(feature = "windef")]
pub type LPHDHITTESTINFO = *mut HDHITTESTINFO;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPHDITEMA = *mut HDITEMA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPHDITEMW = *mut HDITEMW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPHDLAYOUT = *mut HDLAYOUT;
pub type LPHD_TEXTFILTERA = *mut HD_TEXTFILTERA;
pub type LPHD_TEXTFILTERW = *mut HD_TEXTFILTERW;
#[cfg(feature = "windef")]
pub type LPIMAGEINFO = *mut IMAGEINFO;
#[cfg(feature = "windef")]
pub type LPIMAGELISTDRAWPARAMS = *mut IMAGELISTDRAWPARAMS;
pub type LPINITCOMMONCONTROLSEX = *mut INITCOMMONCONTROLSEX;
#[cfg(feature = "windef")]
pub type LPLVBKIMAGEA = *mut LVBKIMAGEA;
#[cfg(feature = "windef")]
pub type LPLVBKIMAGEW = *mut LVBKIMAGEW;
pub type LPLVCOLUMNA = *mut LVCOLUMNA;
pub type LPLVCOLUMNW = *mut LVCOLUMNW;
pub type LPLVFOOTERINFO = *mut LVFOOTERINFO;
pub type LPLVFOOTERITEM = *mut LVFOOTERITEM;
#[cfg(feature = "windef")]
pub type LPLVHITTESTINFO = *mut LVHITTESTINFO;
pub type LPLVINSERTMARK = *mut LVINSERTMARK;
#[cfg(feature = "minwindef")]
pub type LPLVITEMA = *mut LVITEMA;
#[cfg(feature = "minwindef")]
pub type LPLVITEMW = *mut LVITEMW;
pub type LPMONTHDAYSTATE = *mut u32;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMBCDROPDOWN = *mut NMBCDROPDOWN;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMBCHOTITEM = *mut NMBCHOTITEM;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMCBEDRAGBEGINA = *mut NMCBEDRAGBEGINA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMCBEDRAGBEGINW = *mut NMCBEDRAGBEGINW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMCBEENDEDITA = *mut NMCBEENDEDITA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMCBEENDEDITW = *mut NMCBEENDEDITW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMCHAR = *mut NMCHAR;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMCLICK = LPNMMOUSE;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMCUSTOMDRAW = *mut NMCUSTOMDRAW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMCUSTOMSPLITRECTINFO = *mut NMCUSTOMSPLITRECTINFO;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMCUSTOMTEXT = *mut NMCUSTOMTEXT;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMECHANGE = *mut NMDATETIMECHANGE;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMEFORMATA = *mut NMDATETIMEFORMATA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMEFORMATQUERYA = *mut NMDATETIMEFORMATQUERYA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMEFORMATQUERYW = *mut NMDATETIMEFORMATQUERYW;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMEFORMATW = *mut NMDATETIMEFORMATW;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMESTRINGA = *mut NMDATETIMESTRINGA;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMESTRINGW = *mut NMDATETIMESTRINGW;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMEWMKEYDOWNA = *mut NMDATETIMEWMKEYDOWNA;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDATETIMEWMKEYDOWNW = *mut NMDATETIMEWMKEYDOWNW;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMDAYSTATE = *mut NMDAYSTATE;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMHDDISPINFOA = *mut NMHDDISPINFOA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMHDDISPINFOW = *mut NMHDDISPINFOW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMHDFILTERBTNCLICK = *mut NMHDFILTERBTNCLICK;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMHEADERA = *mut NMHEADERA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMHEADERW = *mut NMHEADERW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMIPADDRESS = *mut NMIPADDRESS;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMITEMACTIVATE = *mut NMITEMACTIVATE;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMKEY = *mut NMKEY;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLISTVIEW = *mut NMLISTVIEW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMLVCACHEHINT = *mut NMLVCACHEHINT;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLVCUSTOMDRAW = *mut NMLVCUSTOMDRAW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLVDISPINFOA = *mut NMLVDISPINFOA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLVDISPINFOW = *mut NMLVDISPINFOW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLVFINDITEMA = *mut NMLVFINDITEMA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLVFINDITEMW = *mut NMLVFINDITEMW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLVGETINFOTIPA = *mut NMLVGETINFOTIPA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMLVGETINFOTIPW = *mut NMLVGETINFOTIPW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMLVKEYDOWN = *mut NMLVKEYDOWN;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMLVODSTATECHANGE = *mut NMLVODSTATECHANGE;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMLVSCROLL = *mut NMLVSCROLL;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMMOUSE = *mut NMMOUSE;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMOBJECTNOTIFY = *mut NMOBJECTNOTIFY;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMPGCALCSIZE = *mut NMPGCALCSIZE;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMPGHOTITEM = *mut NMPGHOTITEM;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMPGSCROLL = *mut NMPGSCROLL;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMRBAUTOSIZE = *mut NMRBAUTOSIZE;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMREBAR = *mut NMREBAR;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMREBARAUTOBREAK = *mut NMREBARAUTOBREAK;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMREBARCHEVRON = *mut NMREBARCHEVRON;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMREBARCHILDSIZE = *mut NMREBARCHILDSIZE;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMREBARSPLITTER = *mut NMREBARSPLITTER;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMSELCHANGE = *mut NMSELCHANGE;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type LPNMSELECT = *mut NMSELCHANGE;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTBCUSTOMDRAW = *mut NMTBCUSTOMDRAW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTBDISPINFOA = *mut NMTBDISPINFOA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTBDISPINFOW = *mut NMTBDISPINFOW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTBGETINFOTIPA = *mut NMTBGETINFOTIPA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTBGETINFOTIPW = *mut NMTBGETINFOTIPW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTBHOTITEM = *mut NMTBHOTITEM;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTBRESTORE = *mut NMTBRESTORE;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTBSAVE = *mut NMTBSAVE;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTOOLBARA = *mut NMTOOLBARA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTOOLBARW = *mut NMTOOLBARW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTOOLTIPSCREATED = *mut NMTOOLTIPSCREATED;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTREEVIEWA = *mut NMTREEVIEWA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTREEVIEWW = *mut NMTREEVIEWW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTTCUSTOMDRAW = *mut NMTTCUSTOMDRAW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTTDISPINFOA = *mut NMTTDISPINFOA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTTDISPINFOW = *mut NMTTDISPINFOW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTVCUSTOMDRAW = *mut NMTVCUSTOMDRAW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTVDISPINFOA = *mut NMTVDISPINFOA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTVDISPINFOEXA = *mut NMTVDISPINFOEXA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTVDISPINFOEXW = *mut NMTVDISPINFOEXW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTVDISPINFOW = *mut NMTVDISPINFOW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTVGETINFOTIPA = *mut NMTVGETINFOTIPA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPNMTVGETINFOTIPW = *mut NMTVGETINFOTIPW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTVKEYDOWN = *mut NMTVKEYDOWN;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMTVSTATEIMAGECHANGING = *mut NMTVSTATEIMAGECHANGING;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMUPDOWN = *mut NMUPDOWN;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type LPNMVIEWCHANGE = *mut NMVIEWCHANGE;
#[cfg(feature = "windef")]
pub type LPRBHITTESTINFO = *mut RBHITTESTINFO;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPREBARBANDINFOA = *mut REBARBANDINFOA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPREBARBANDINFOW = *mut REBARBANDINFOW;
pub type LPREBARINFO = *mut REBARINFO;
pub const LPSTR_TEXTCALLBACKA: windows_core::PCSTR = windows_core::PCSTR(-1 as _);
pub const LPSTR_TEXTCALLBACKW: windows_core::PCWSTR = windows_core::PCWSTR(-1 as _);
#[cfg(feature = "minwindef")]
pub type LPTBADDBITMAP = *mut TBADDBITMAP;
pub type LPTBBUTTON = *mut TBBUTTON;
pub type LPTBBUTTONINFOA = *mut TBBUTTONINFOA;
pub type LPTBBUTTONINFOW = *mut TBBUTTONINFOW;
pub type LPTBINSERTMARK = *mut TBINSERTMARK;
pub type LPTBMETRICS = *mut TBMETRICS;
#[cfg(feature = "minwindef")]
pub type LPTBREPLACEBITMAP = *mut TBREPLACEBITMAP;
#[cfg(feature = "minwindef")]
pub type LPTBSAVEPARAMSA = *mut TBSAVEPARAMSA;
#[cfg(feature = "minwindef")]
pub type LPTBSAVEPARAMW = *mut TBSAVEPARAMSW;
#[cfg(feature = "windef")]
pub type LPTCHITTESTINFO = *mut TCHITTESTINFO;
#[cfg(feature = "minwindef")]
pub type LPTCITEMA = *mut TCITEMA;
pub type LPTCITEMHEADERA = *mut TCITEMHEADERA;
pub type LPTCITEMHEADERW = *mut TCITEMHEADERW;
#[cfg(feature = "minwindef")]
pub type LPTCITEMW = *mut TCITEMW;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTTHITTESTINFOA = *mut TTHITTESTINFOA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTTHITTESTINFOW = *mut TTHITTESTINFOW;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTTTOOLINFOA = *mut TTTOOLINFOA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTTTOOLINFOW = *mut TTTOOLINFOW;
#[cfg(feature = "windef")]
pub type LPTVHITTESTINFO = *mut TVHITTESTINFO;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTVINSERTSTRUCTA = *mut TVINSERTSTRUCTA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTVINSERTSTRUCTW = *mut TVINSERTSTRUCTW;
#[cfg(feature = "minwindef")]
pub type LPTVITEMA = *mut TVITEMA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTVITEMEX = LPTVITEMEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTVITEMEXA = *mut TVITEMEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPTVITEMEXW = *mut TVITEMEXW;
#[cfg(feature = "minwindef")]
pub type LPTVITEMW = *mut TVITEMW;
#[cfg(feature = "minwindef")]
pub type LPTVSORTCB = *mut TVSORTCB;
pub type LPUDACCEL = *mut UDACCEL;
pub const LVA_ALIGNLEFT: i32 = 1;
pub const LVA_ALIGNTOP: i32 = 2;
pub const LVA_DEFAULT: i32 = 0;
pub const LVA_SNAPTOGRID: i32 = 5;
pub const LVBKIF_FLAG_ALPHABLEND: i32 = 536870912;
pub const LVBKIF_FLAG_TILEOFFSET: i32 = 256;
pub const LVBKIF_SOURCE_HBITMAP: i32 = 1;
pub const LVBKIF_SOURCE_MASK: i32 = 3;
pub const LVBKIF_SOURCE_NONE: i32 = 0;
pub const LVBKIF_SOURCE_URL: i32 = 2;
pub const LVBKIF_STYLE_MASK: i32 = 16;
pub const LVBKIF_STYLE_NORMAL: i32 = 0;
pub const LVBKIF_STYLE_TILE: i32 = 16;
pub const LVBKIF_TYPE_WATERMARK: i32 = 268435456;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVBKIMAGEA {
    pub ulFlags: u32,
    pub hbm: super::HBITMAP,
    pub pszImage: windows_core::PSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVBKIMAGEW {
    pub ulFlags: u32,
    pub hbm: super::HBITMAP,
    pub pszImage: windows_core::PWSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
pub const LVCDI_GROUP: i32 = 1;
pub const LVCDI_ITEM: i32 = 0;
pub const LVCDI_ITEMSLIST: i32 = 2;
pub const LVCDRF_NOGROUPFRAME: i32 = 131072;
pub const LVCDRF_NOSELECT: i32 = 65536;
pub const LVCFMT_BITMAP_ON_RIGHT: i32 = 4096;
pub const LVCFMT_CENTER: i32 = 2;
pub const LVCFMT_COL_HAS_IMAGES: i32 = 32768;
pub const LVCFMT_FILL: i32 = 2097152;
pub const LVCFMT_FIXED_RATIO: i32 = 524288;
pub const LVCFMT_FIXED_WIDTH: i32 = 256;
pub const LVCFMT_IMAGE: i32 = 2048;
pub const LVCFMT_JUSTIFYMASK: i32 = 3;
pub const LVCFMT_LEFT: i32 = 0;
pub const LVCFMT_LINE_BREAK: i32 = 1048576;
pub const LVCFMT_NO_DPI_SCALE: i32 = 262144;
pub const LVCFMT_NO_TITLE: i32 = 8388608;
pub const LVCFMT_RIGHT: i32 = 1;
pub const LVCFMT_SPLITBUTTON: i32 = 16777216;
pub const LVCFMT_TILE_PLACEMENTMASK: i32 = 3145728;
pub const LVCFMT_WRAP: i32 = 4194304;
pub const LVCF_DEFAULTWIDTH: i32 = 128;
pub const LVCF_FMT: i32 = 1;
pub const LVCF_IDEALWIDTH: i32 = 256;
pub const LVCF_IMAGE: i32 = 16;
pub const LVCF_MINWIDTH: i32 = 64;
pub const LVCF_ORDER: i32 = 32;
pub const LVCF_SUBITEM: i32 = 8;
pub const LVCF_TEXT: i32 = 4;
pub const LVCF_WIDTH: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVCOLUMNA {
    pub mask: u32,
    pub fmt: i32,
    pub cx: i32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
#[cfg(target_arch = "x86")]
pub const LVCOLUMNA_V1_SIZE: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVCOLUMNA_V1_SIZE: u64 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVCOLUMNW {
    pub mask: u32,
    pub fmt: i32,
    pub cx: i32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
#[cfg(target_arch = "x86")]
pub const LVCOLUMNW_V1_SIZE: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVCOLUMNW_V1_SIZE: u64 = 32;
#[cfg(target_arch = "x86")]
pub const LVCOLUMN_V1_SIZE: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVCOLUMN_V1_SIZE: u64 = 32;
pub const LVFF_ITEMCOUNT: i32 = 1;
pub const LVFIF_STATE: i32 = 2;
pub const LVFIF_TEXT: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVFINDINFOA {
    pub flags: u32,
    pub psz: windows_core::PCSTR,
    pub lParam: super::LPARAM,
    pub pt: super::POINT,
    pub vkDirection: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVFINDINFOW {
    pub flags: u32,
    pub psz: windows_core::PCWSTR,
    pub lParam: super::LPARAM,
    pub pt: super::POINT,
    pub vkDirection: u32,
}
pub const LVFIS_FOCUSED: i32 = 1;
pub const LVFI_NEARESTXY: i32 = 64;
pub const LVFI_PARAM: i32 = 1;
pub const LVFI_PARTIAL: i32 = 8;
pub const LVFI_STRING: i32 = 2;
pub const LVFI_SUBSTRING: i32 = 4;
pub const LVFI_WRAP: i32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVFOOTERINFO {
    pub mask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub cItems: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVFOOTERITEM {
    pub mask: u32,
    pub iItem: i32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub state: u32,
    pub stateMask: u32,
}
pub const LVGA_FOOTER_CENTER: i32 = 16;
pub const LVGA_FOOTER_LEFT: i32 = 8;
pub const LVGA_FOOTER_RIGHT: i32 = 32;
pub const LVGA_HEADER_CENTER: i32 = 2;
pub const LVGA_HEADER_LEFT: i32 = 1;
pub const LVGA_HEADER_RIGHT: i32 = 4;
pub const LVGF_ALIGN: i32 = 8;
pub const LVGF_DESCRIPTIONBOTTOM: i32 = 2048;
pub const LVGF_DESCRIPTIONTOP: i32 = 1024;
pub const LVGF_EXTENDEDIMAGE: i32 = 8192;
pub const LVGF_FOOTER: i32 = 2;
pub const LVGF_GROUPID: i32 = 16;
pub const LVGF_HEADER: i32 = 1;
pub const LVGF_ITEMS: i32 = 16384;
pub const LVGF_NONE: i32 = 0;
pub const LVGF_STATE: i32 = 4;
pub const LVGF_SUBSET: i32 = 32768;
pub const LVGF_SUBSETITEMS: i32 = 65536;
pub const LVGF_SUBTITLE: i32 = 256;
pub const LVGF_TASK: i32 = 512;
pub const LVGF_TITLEIMAGE: i32 = 4096;
pub const LVGGR_GROUP: i32 = 0;
pub const LVGGR_HEADER: i32 = 1;
pub const LVGGR_LABEL: i32 = 2;
pub const LVGGR_SUBSETLINK: i32 = 3;
pub const LVGIT_UNFOLDED: i32 = 1;
pub const LVGMF_BORDERCOLOR: i32 = 2;
pub const LVGMF_BORDERSIZE: i32 = 1;
pub const LVGMF_NONE: i32 = 0;
pub const LVGMF_TEXTCOLOR: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVGROUP {
    pub cbSize: u32,
    pub mask: u32,
    pub pszHeader: windows_core::PWSTR,
    pub cchHeader: i32,
    pub pszFooter: windows_core::PWSTR,
    pub cchFooter: i32,
    pub iGroupId: i32,
    pub stateMask: u32,
    pub state: u32,
    pub uAlign: u32,
    pub pszSubtitle: windows_core::PWSTR,
    pub cchSubtitle: u32,
    pub pszTask: windows_core::PWSTR,
    pub cchTask: u32,
    pub pszDescriptionTop: windows_core::PWSTR,
    pub cchDescriptionTop: u32,
    pub pszDescriptionBottom: windows_core::PWSTR,
    pub cchDescriptionBottom: u32,
    pub iTitleImage: i32,
    pub iExtendedImage: i32,
    pub iFirstItem: i32,
    pub cItems: u32,
    pub pszSubsetTitle: windows_core::PWSTR,
    pub cchSubsetTitle: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVGROUPMETRICS {
    pub cbSize: u32,
    pub mask: u32,
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub crLeft: super::COLORREF,
    pub crTop: super::COLORREF,
    pub crRight: super::COLORREF,
    pub crBottom: super::COLORREF,
    pub crHeader: super::COLORREF,
    pub crFooter: super::COLORREF,
}
#[cfg(target_arch = "x86")]
pub const LVGROUP_V5_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVGROUP_V5_SIZE: u64 = 52;
pub const LVGS_COLLAPSED: i32 = 1;
pub const LVGS_COLLAPSIBLE: i32 = 8;
pub const LVGS_FOCUSED: i32 = 16;
pub const LVGS_HIDDEN: i32 = 2;
pub const LVGS_NOHEADER: i32 = 4;
pub const LVGS_NORMAL: i32 = 0;
pub const LVGS_SELECTED: i32 = 32;
pub const LVGS_SUBSETED: i32 = 64;
pub const LVGS_SUBSETLINKFOCUSED: i32 = 128;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVHITTESTINFO {
    pub pt: super::POINT,
    pub flags: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub iGroup: i32,
}
#[cfg(target_arch = "x86")]
pub const LVHITTESTINFO_V1_SIZE: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVHITTESTINFO_V1_SIZE: u64 = 16;
pub const LVHT_ABOVE: i32 = 8;
pub const LVHT_BELOW: i32 = 16;
pub const LVHT_EX_FOOTER: i32 = 134217728;
pub const LVHT_EX_GROUP: u32 = 4076863488;
pub const LVHT_EX_GROUP_BACKGROUND: u32 = 2147483648;
pub const LVHT_EX_GROUP_COLLAPSE: i32 = 1073741824;
pub const LVHT_EX_GROUP_FOOTER: i32 = 536870912;
pub const LVHT_EX_GROUP_HEADER: i32 = 268435456;
pub const LVHT_EX_GROUP_STATEICON: i32 = 16777216;
pub const LVHT_EX_GROUP_SUBSETLINK: i32 = 33554432;
pub const LVHT_EX_ONCONTENTS: i32 = 67108864;
pub const LVHT_NOWHERE: i32 = 1;
pub const LVHT_ONITEM: i32 = 14;
pub const LVHT_ONITEMICON: i32 = 2;
pub const LVHT_ONITEMLABEL: i32 = 4;
pub const LVHT_ONITEMSTATEICON: i32 = 8;
pub const LVHT_TOLEFT: i32 = 64;
pub const LVHT_TORIGHT: i32 = 32;
pub const LVIF_COLFMT: i32 = 65536;
pub const LVIF_COLUMNS: i32 = 512;
pub const LVIF_DI_SETITEM: i32 = 4096;
pub const LVIF_GROUPID: i32 = 256;
pub const LVIF_IMAGE: i32 = 2;
pub const LVIF_INDENT: i32 = 16;
pub const LVIF_NORECOMPUTE: i32 = 2048;
pub const LVIF_PARAM: i32 = 4;
pub const LVIF_STATE: i32 = 8;
pub const LVIF_TEXT: i32 = 1;
pub const LVIM_AFTER: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LVINSERTGROUPSORTED {
    pub pfnGroupCompare: PFNLVGROUPCOMPARE,
    pub pvData: *mut core::ffi::c_void,
    pub lvGroup: LVGROUP,
}
impl Default for LVINSERTGROUPSORTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVINSERTMARK {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iItem: i32,
    pub dwReserved: u32,
}
pub const LVIR_BOUNDS: i32 = 0;
pub const LVIR_ICON: i32 = 1;
pub const LVIR_LABEL: i32 = 2;
pub const LVIR_SELECTBOUNDS: i32 = 3;
pub const LVIS_ACTIVATING: i32 = 32;
pub const LVIS_CUT: i32 = 4;
pub const LVIS_DROPHILITED: i32 = 8;
pub const LVIS_FOCUSED: i32 = 1;
pub const LVIS_GLOW: i32 = 16;
pub const LVIS_OVERLAYMASK: i32 = 3840;
pub const LVIS_SELECTED: i32 = 2;
pub const LVIS_STATEIMAGEMASK: i32 = 61440;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LVITEMA {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: super::PUINT,
    pub piColFmt: *mut i32,
    pub iGroup: i32,
}
#[cfg(feature = "minwindef")]
impl Default for LVITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const LVITEMA_V1_SIZE: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEMA_V1_SIZE: u64 = 48;
#[cfg(target_arch = "x86")]
pub const LVITEMA_V5_SIZE: u32 = 52;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEMA_V5_SIZE: u64 = 72;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVITEMINDEX {
    pub iItem: i32,
    pub iGroup: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LVITEMW {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: super::PUINT,
    pub piColFmt: *mut i32,
    pub iGroup: i32,
}
#[cfg(feature = "minwindef")]
impl Default for LVITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const LVITEMW_V1_SIZE: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEMW_V1_SIZE: u64 = 48;
#[cfg(target_arch = "x86")]
pub const LVITEMW_V5_SIZE: u32 = 52;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEMW_V5_SIZE: u64 = 72;
#[cfg(target_arch = "x86")]
pub const LVITEM_V1_SIZE: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEM_V1_SIZE: u64 = 48;
#[cfg(target_arch = "x86")]
pub const LVITEM_V5_SIZE: u32 = 52;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEM_V5_SIZE: u64 = 72;
pub const LVKF_ALT: i32 = 1;
pub const LVKF_CONTROL: i32 = 2;
pub const LVKF_SHIFT: i32 = 4;
pub const LVM_APPROXIMATEVIEWRECT: i32 = 4160;
pub const LVM_ARRANGE: i32 = 4118;
pub const LVM_CANCELEDITLABEL: i32 = 4275;
pub const LVM_CREATEDRAGIMAGE: i32 = 4129;
pub const LVM_DELETEALLITEMS: i32 = 4105;
pub const LVM_DELETECOLUMN: i32 = 4124;
pub const LVM_DELETEITEM: i32 = 4104;
pub const LVM_EDITLABEL: i32 = 4119;
pub const LVM_EDITLABELA: i32 = 4119;
pub const LVM_EDITLABELW: i32 = 4214;
pub const LVM_ENABLEGROUPVIEW: i32 = 4253;
pub const LVM_ENSUREVISIBLE: i32 = 4115;
pub const LVM_FINDITEM: i32 = 4109;
pub const LVM_FINDITEMA: i32 = 4109;
pub const LVM_FINDITEMW: i32 = 4179;
pub const LVM_FIRST: i32 = 4096;
pub const LVM_GETBKCOLOR: i32 = 4096;
pub const LVM_GETBKIMAGE: i32 = 4165;
pub const LVM_GETBKIMAGEA: i32 = 4165;
pub const LVM_GETBKIMAGEW: i32 = 4235;
pub const LVM_GETCALLBACKMASK: i32 = 4106;
pub const LVM_GETCOLUMN: i32 = 4121;
pub const LVM_GETCOLUMNA: i32 = 4121;
pub const LVM_GETCOLUMNORDERARRAY: i32 = 4155;
pub const LVM_GETCOLUMNW: i32 = 4191;
pub const LVM_GETCOLUMNWIDTH: i32 = 4125;
pub const LVM_GETCOUNTPERPAGE: i32 = 4136;
pub const LVM_GETEDITCONTROL: i32 = 4120;
pub const LVM_GETEMPTYTEXT: i32 = 4300;
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: i32 = 4151;
pub const LVM_GETFOCUSEDGROUP: i32 = 4189;
pub const LVM_GETFOOTERINFO: i32 = 4302;
pub const LVM_GETFOOTERITEM: i32 = 4304;
pub const LVM_GETFOOTERITEMRECT: i32 = 4303;
pub const LVM_GETFOOTERRECT: i32 = 4301;
pub const LVM_GETGROUPCOUNT: i32 = 4248;
pub const LVM_GETGROUPINFO: i32 = 4245;
pub const LVM_GETGROUPINFOBYINDEX: i32 = 4249;
pub const LVM_GETGROUPMETRICS: i32 = 4252;
pub const LVM_GETGROUPRECT: i32 = 4194;
pub const LVM_GETGROUPSTATE: i32 = 4188;
pub const LVM_GETHEADER: i32 = 4127;
pub const LVM_GETHOTCURSOR: i32 = 4159;
pub const LVM_GETHOTITEM: i32 = 4157;
pub const LVM_GETHOVERTIME: i32 = 4168;
pub const LVM_GETIMAGELIST: i32 = 4098;
pub const LVM_GETINSERTMARK: i32 = 4263;
pub const LVM_GETINSERTMARKCOLOR: i32 = 4267;
pub const LVM_GETINSERTMARKRECT: i32 = 4265;
pub const LVM_GETISEARCHSTRING: i32 = 4148;
pub const LVM_GETISEARCHSTRINGA: i32 = 4148;
pub const LVM_GETISEARCHSTRINGW: i32 = 4213;
pub const LVM_GETITEM: i32 = 4101;
pub const LVM_GETITEMA: i32 = 4101;
pub const LVM_GETITEMCOUNT: i32 = 4100;
pub const LVM_GETITEMINDEXRECT: i32 = 4305;
pub const LVM_GETITEMPOSITION: i32 = 4112;
pub const LVM_GETITEMRECT: i32 = 4110;
pub const LVM_GETITEMSPACING: i32 = 4147;
pub const LVM_GETITEMSTATE: i32 = 4140;
pub const LVM_GETITEMTEXT: i32 = 4141;
pub const LVM_GETITEMTEXTA: i32 = 4141;
pub const LVM_GETITEMTEXTW: i32 = 4211;
pub const LVM_GETITEMW: i32 = 4171;
pub const LVM_GETNEXTITEM: i32 = 4108;
pub const LVM_GETNEXTITEMINDEX: i32 = 4307;
pub const LVM_GETNUMBEROFWORKAREAS: i32 = 4169;
pub const LVM_GETORIGIN: i32 = 4137;
pub const LVM_GETOUTLINECOLOR: i32 = 4272;
pub const LVM_GETSELECTEDCOLUMN: i32 = 4270;
pub const LVM_GETSELECTEDCOUNT: i32 = 4146;
pub const LVM_GETSELECTIONMARK: i32 = 4162;
pub const LVM_GETSTRINGWIDTH: i32 = 4113;
pub const LVM_GETSTRINGWIDTHA: i32 = 4113;
pub const LVM_GETSTRINGWIDTHW: i32 = 4183;
pub const LVM_GETSUBITEMRECT: i32 = 4152;
pub const LVM_GETTEXTBKCOLOR: i32 = 4133;
pub const LVM_GETTEXTCOLOR: i32 = 4131;
pub const LVM_GETTILEINFO: i32 = 4261;
pub const LVM_GETTILEVIEWINFO: i32 = 4259;
pub const LVM_GETTOOLTIPS: i32 = 4174;
pub const LVM_GETTOPINDEX: i32 = 4135;
pub const LVM_GETUNICODEFORMAT: i32 = 8198;
pub const LVM_GETVIEW: i32 = 4239;
pub const LVM_GETVIEWRECT: i32 = 4130;
pub const LVM_GETWORKAREAS: i32 = 4166;
pub const LVM_HASGROUP: i32 = 4257;
pub const LVM_HITTEST: i32 = 4114;
pub const LVM_INSERTCOLUMN: i32 = 4123;
pub const LVM_INSERTCOLUMNA: i32 = 4123;
pub const LVM_INSERTCOLUMNW: i32 = 4193;
pub const LVM_INSERTGROUP: i32 = 4241;
pub const LVM_INSERTGROUPSORTED: i32 = 4255;
pub const LVM_INSERTITEM: i32 = 4103;
pub const LVM_INSERTITEMA: i32 = 4103;
pub const LVM_INSERTITEMW: i32 = 4173;
pub const LVM_INSERTMARKHITTEST: i32 = 4264;
pub const LVM_ISGROUPVIEWENABLED: i32 = 4271;
pub const LVM_ISITEMVISIBLE: i32 = 4278;
pub const LVM_MAPIDTOINDEX: i32 = 4277;
pub const LVM_MAPINDEXTOID: i32 = 4276;
pub const LVM_MOVEGROUP: i32 = 4247;
pub const LVM_MOVEITEMTOGROUP: i32 = 4250;
pub const LVM_REDRAWITEMS: i32 = 4117;
pub const LVM_REMOVEALLGROUPS: i32 = 4256;
pub const LVM_REMOVEGROUP: i32 = 4246;
pub const LVM_SCROLL: i32 = 4116;
pub const LVM_SETBKCOLOR: i32 = 4097;
pub const LVM_SETBKIMAGE: i32 = 4164;
pub const LVM_SETBKIMAGEA: i32 = 4164;
pub const LVM_SETBKIMAGEW: i32 = 4234;
pub const LVM_SETCALLBACKMASK: i32 = 4107;
pub const LVM_SETCOLUMN: i32 = 4122;
pub const LVM_SETCOLUMNA: i32 = 4122;
pub const LVM_SETCOLUMNORDERARRAY: i32 = 4154;
pub const LVM_SETCOLUMNW: i32 = 4192;
pub const LVM_SETCOLUMNWIDTH: i32 = 4126;
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: i32 = 4150;
pub const LVM_SETGROUPINFO: i32 = 4243;
pub const LVM_SETGROUPMETRICS: i32 = 4251;
pub const LVM_SETHOTCURSOR: i32 = 4158;
pub const LVM_SETHOTITEM: i32 = 4156;
pub const LVM_SETHOVERTIME: i32 = 4167;
pub const LVM_SETICONSPACING: i32 = 4149;
pub const LVM_SETIMAGELIST: i32 = 4099;
pub const LVM_SETINFOTIP: i32 = 4269;
pub const LVM_SETINSERTMARK: i32 = 4262;
pub const LVM_SETINSERTMARKCOLOR: i32 = 4266;
pub const LVM_SETITEM: i32 = 4102;
pub const LVM_SETITEMA: i32 = 4102;
pub const LVM_SETITEMCOUNT: i32 = 4143;
pub const LVM_SETITEMINDEXSTATE: i32 = 4306;
pub const LVM_SETITEMPOSITION: i32 = 4111;
pub const LVM_SETITEMPOSITION32: i32 = 4145;
pub const LVM_SETITEMSTATE: i32 = 4139;
pub const LVM_SETITEMTEXT: i32 = 4142;
pub const LVM_SETITEMTEXTA: i32 = 4142;
pub const LVM_SETITEMTEXTW: i32 = 4212;
pub const LVM_SETITEMW: i32 = 4172;
pub const LVM_SETOUTLINECOLOR: i32 = 4273;
pub const LVM_SETSELECTEDCOLUMN: i32 = 4236;
pub const LVM_SETSELECTIONMARK: i32 = 4163;
pub const LVM_SETTEXTBKCOLOR: i32 = 4134;
pub const LVM_SETTEXTCOLOR: i32 = 4132;
pub const LVM_SETTILEINFO: i32 = 4260;
pub const LVM_SETTILEVIEWINFO: i32 = 4258;
pub const LVM_SETTOOLTIPS: i32 = 4170;
pub const LVM_SETUNICODEFORMAT: i32 = 8197;
pub const LVM_SETVIEW: i32 = 4238;
pub const LVM_SETWORKAREAS: i32 = 4161;
pub const LVM_SORTGROUPS: i32 = 4254;
pub const LVM_SORTITEMS: i32 = 4144;
pub const LVM_SORTITEMSEX: i32 = 4177;
pub const LVM_SUBITEMHITTEST: i32 = 4153;
pub const LVM_UPDATE: i32 = 4138;
pub const LVNI_ABOVE: i32 = 256;
pub const LVNI_ALL: i32 = 0;
pub const LVNI_BELOW: i32 = 512;
pub const LVNI_CUT: i32 = 4;
pub const LVNI_DIRECTIONMASK: i32 = 3840;
pub const LVNI_DROPHILITED: i32 = 8;
pub const LVNI_FOCUSED: i32 = 1;
pub const LVNI_PREVIOUS: i32 = 32;
pub const LVNI_SAMEGROUPONLY: i32 = 128;
pub const LVNI_SELECTED: i32 = 2;
pub const LVNI_STATEMASK: i32 = 15;
pub const LVNI_TOLEFT: i32 = 1024;
pub const LVNI_TORIGHT: i32 = 2048;
pub const LVNI_VISIBLEONLY: i32 = 64;
pub const LVNI_VISIBLEORDER: i32 = 16;
pub const LVNSCH_DEFAULT: i32 = -1;
pub const LVNSCH_ERROR: i32 = -2;
pub const LVNSCH_IGNORE: i32 = -3;
pub const LVN_BEGINDRAG: u32 = 4294967187;
pub const LVN_BEGINLABELEDIT: u32 = 4294967191;
pub const LVN_BEGINLABELEDITA: u32 = 4294967191;
pub const LVN_BEGINLABELEDITW: u32 = 4294967121;
pub const LVN_BEGINRDRAG: u32 = 4294967185;
pub const LVN_BEGINSCROLL: u32 = 4294967116;
pub const LVN_COLUMNCLICK: u32 = 4294967188;
pub const LVN_COLUMNDROPDOWN: u32 = 4294967132;
pub const LVN_COLUMNOVERFLOWCLICK: u32 = 4294967130;
pub const LVN_DELETEALLITEMS: u32 = 4294967192;
pub const LVN_DELETEITEM: u32 = 4294967193;
pub const LVN_ENDLABELEDIT: u32 = 4294967190;
pub const LVN_ENDLABELEDITA: u32 = 4294967190;
pub const LVN_ENDLABELEDITW: u32 = 4294967120;
pub const LVN_ENDSCROLL: u32 = 4294967115;
pub const LVN_FIRST: u32 = 4294967196;
pub const LVN_GETDISPINFO: u32 = 4294967146;
pub const LVN_GETDISPINFOA: u32 = 4294967146;
pub const LVN_GETDISPINFOW: u32 = 4294967119;
pub const LVN_GETEMPTYMARKUP: u32 = 4294967109;
pub const LVN_GETINFOTIP: u32 = 4294967139;
pub const LVN_GETINFOTIPA: u32 = 4294967139;
pub const LVN_GETINFOTIPW: u32 = 4294967138;
pub const LVN_HOTTRACK: u32 = 4294967175;
pub const LVN_INCREMENTALSEARCH: u32 = 4294967134;
pub const LVN_INCREMENTALSEARCHA: u32 = 4294967134;
pub const LVN_INCREMENTALSEARCHW: u32 = 4294967133;
pub const LVN_INSERTITEM: u32 = 4294967194;
pub const LVN_ITEMACTIVATE: u32 = 4294967182;
pub const LVN_ITEMCHANGED: u32 = 4294967195;
pub const LVN_ITEMCHANGING: u32 = 4294967196;
pub const LVN_KEYDOWN: u32 = 4294967141;
pub const LVN_LAST: u32 = 4294967097;
pub const LVN_LINKCLICK: u32 = 4294967112;
pub const LVN_MARQUEEBEGIN: u32 = 4294967140;
pub const LVN_ODCACHEHINT: u32 = 4294967183;
pub const LVN_ODFINDITEM: u32 = 4294967144;
pub const LVN_ODFINDITEMA: u32 = 4294967144;
pub const LVN_ODFINDITEMW: u32 = 4294967117;
pub const LVN_ODSTATECHANGED: u32 = 4294967181;
pub const LVN_SETDISPINFO: u32 = 4294967145;
pub const LVN_SETDISPINFOA: u32 = 4294967145;
pub const LVN_SETDISPINFOW: u32 = 4294967118;
pub const LVSCW_AUTOSIZE: i32 = -1;
pub const LVSCW_AUTOSIZE_USEHEADER: i32 = -2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVSETINFOTIP {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pszText: windows_core::PWSTR,
    pub iItem: i32,
    pub iSubItem: i32,
}
pub const LVSICF_NOINVALIDATEALL: i32 = 1;
pub const LVSICF_NOSCROLL: i32 = 2;
pub const LVSIL_GROUPHEADER: i32 = 3;
pub const LVSIL_NORMAL: i32 = 0;
pub const LVSIL_SMALL: i32 = 1;
pub const LVSIL_STATE: i32 = 2;
pub const LVS_ALIGNLEFT: i32 = 2048;
pub const LVS_ALIGNMASK: i32 = 3072;
pub const LVS_ALIGNTOP: i32 = 0;
pub const LVS_AUTOARRANGE: i32 = 256;
pub const LVS_EDITLABELS: i32 = 512;
pub const LVS_EX_AUTOAUTOARRANGE: i32 = 16777216;
pub const LVS_EX_AUTOCHECKSELECT: i32 = 134217728;
pub const LVS_EX_AUTOSIZECOLUMNS: i32 = 268435456;
pub const LVS_EX_BORDERSELECT: i32 = 32768;
pub const LVS_EX_CHECKBOXES: i32 = 4;
pub const LVS_EX_COLUMNOVERFLOW: u32 = 2147483648;
pub const LVS_EX_COLUMNSNAPPOINTS: i32 = 1073741824;
pub const LVS_EX_DOUBLEBUFFER: i32 = 65536;
pub const LVS_EX_FLATSB: i32 = 256;
pub const LVS_EX_FULLROWSELECT: i32 = 32;
pub const LVS_EX_GRIDLINES: i32 = 1;
pub const LVS_EX_HEADERDRAGDROP: i32 = 16;
pub const LVS_EX_HEADERINALLVIEWS: i32 = 33554432;
pub const LVS_EX_HIDELABELS: i32 = 131072;
pub const LVS_EX_INFOTIP: i32 = 1024;
pub const LVS_EX_JUSTIFYCOLUMNS: i32 = 2097152;
pub const LVS_EX_LABELTIP: i32 = 16384;
pub const LVS_EX_MULTIWORKAREAS: i32 = 8192;
pub const LVS_EX_ONECLICKACTIVATE: i32 = 64;
pub const LVS_EX_REGIONAL: i32 = 512;
pub const LVS_EX_SIMPLESELECT: i32 = 1048576;
pub const LVS_EX_SINGLEROW: i32 = 262144;
pub const LVS_EX_SNAPTOGRID: i32 = 524288;
pub const LVS_EX_SUBITEMIMAGES: i32 = 2;
pub const LVS_EX_TRACKSELECT: i32 = 8;
pub const LVS_EX_TRANSPARENTBKGND: i32 = 4194304;
pub const LVS_EX_TRANSPARENTSHADOWTEXT: i32 = 8388608;
pub const LVS_EX_TWOCLICKACTIVATE: i32 = 128;
pub const LVS_EX_UNDERLINECOLD: i32 = 4096;
pub const LVS_EX_UNDERLINEHOT: i32 = 2048;
pub const LVS_ICON: i32 = 0;
pub const LVS_LIST: i32 = 3;
pub const LVS_NOCOLUMNHEADER: i32 = 16384;
pub const LVS_NOLABELWRAP: i32 = 128;
pub const LVS_NOSCROLL: i32 = 8192;
pub const LVS_NOSORTHEADER: i32 = 32768;
pub const LVS_OWNERDATA: i32 = 4096;
pub const LVS_OWNERDRAWFIXED: i32 = 1024;
pub const LVS_REPORT: i32 = 1;
pub const LVS_SHAREIMAGELISTS: i32 = 64;
pub const LVS_SHOWSELALWAYS: i32 = 8;
pub const LVS_SINGLESEL: i32 = 4;
pub const LVS_SMALLICON: i32 = 2;
pub const LVS_SORTASCENDING: i32 = 16;
pub const LVS_SORTDESCENDING: i32 = 32;
pub const LVS_TYPEMASK: i32 = 3;
pub const LVS_TYPESTYLEMASK: i32 = 64512;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LVTILEINFO {
    pub cbSize: u32,
    pub iItem: i32,
    pub cColumns: u32,
    pub puColumns: super::PUINT,
    pub piColFmt: *mut i32,
}
#[cfg(feature = "minwindef")]
impl Default for LVTILEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const LVTILEINFO_V5_SIZE: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVTILEINFO_V5_SIZE: u64 = 24;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVTILEVIEWINFO {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwFlags: u32,
    pub sizeTile: super::SIZE,
    pub cLines: i32,
    pub rcLabelMargin: super::RECT,
}
pub const LVTVIF_AUTOSIZE: i32 = 0;
pub const LVTVIF_EXTENDED: i32 = 4;
pub const LVTVIF_FIXEDHEIGHT: i32 = 2;
pub const LVTVIF_FIXEDSIZE: i32 = 3;
pub const LVTVIF_FIXEDWIDTH: i32 = 1;
pub const LVTVIM_COLUMNS: i32 = 2;
pub const LVTVIM_LABELMARGIN: i32 = 4;
pub const LVTVIM_TILESIZE: i32 = 1;
pub const LV_MAX_WORKAREAS: i32 = 16;
pub const LV_VIEW_DETAILS: i32 = 1;
pub const LV_VIEW_ICON: i32 = 0;
pub const LV_VIEW_LIST: i32 = 3;
pub const LV_VIEW_MAX: i32 = 4;
pub const LV_VIEW_SMALLICON: i32 = 2;
pub const LV_VIEW_TILE: i32 = 4;
pub const LWS_IGNORERETURN: i32 = 2;
pub const LWS_NOPREFIX: i32 = 4;
pub const LWS_RIGHT: i32 = 32;
pub const LWS_TRANSPARENT: i32 = 1;
pub const LWS_USECUSTOMTEXT: i32 = 16;
pub const LWS_USEVISUALSTYLE: i32 = 8;
pub const MAX_LINKID_TEXT: i32 = 48;
pub const MCGIF_DATE: i32 = 1;
pub const MCGIF_NAME: i32 = 4;
pub const MCGIF_RECT: i32 = 2;
pub const MCGIP_CALENDAR: i32 = 4;
pub const MCGIP_CALENDARBODY: i32 = 6;
pub const MCGIP_CALENDARCELL: i32 = 8;
pub const MCGIP_CALENDARCONTROL: i32 = 0;
pub const MCGIP_CALENDARHEADER: i32 = 5;
pub const MCGIP_CALENDARROW: i32 = 7;
pub const MCGIP_FOOTER: i32 = 3;
pub const MCGIP_NEXT: i32 = 1;
pub const MCGIP_PREV: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MCGRIDINFO {
    pub cbSize: u32,
    pub dwPart: u32,
    pub dwFlags: u32,
    pub iCalendar: i32,
    pub iRow: i32,
    pub iCol: i32,
    pub bSelected: windows_core::BOOL,
    pub stStart: super::SYSTEMTIME,
    pub stEnd: super::SYSTEMTIME,
    pub rc: super::RECT,
    pub pszName: windows_core::PWSTR,
    pub cchName: usize,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MCHITTESTINFO {
    pub cbSize: u32,
    pub pt: super::POINT,
    pub uHit: u32,
    pub st: super::SYSTEMTIME,
    pub rc: super::RECT,
    pub iOffset: i32,
    pub iRow: i32,
    pub iCol: i32,
}
#[cfg(target_arch = "x86")]
pub const MCHITTESTINFO_V1_SIZE: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MCHITTESTINFO_V1_SIZE: u64 = 32;
pub const MCHT_CALENDAR: i32 = 131072;
pub const MCHT_CALENDARBK: i32 = 131072;
pub const MCHT_CALENDARCONTROL: i32 = 1048576;
pub const MCHT_CALENDARDATE: i32 = 131073;
pub const MCHT_CALENDARDATEMAX: i32 = 131077;
pub const MCHT_CALENDARDATEMIN: i32 = 131076;
pub const MCHT_CALENDARDATENEXT: i32 = 16908289;
pub const MCHT_CALENDARDATEPREV: i32 = 33685505;
pub const MCHT_CALENDARDAY: i32 = 131074;
pub const MCHT_CALENDARWEEKNUM: i32 = 131075;
pub const MCHT_NEXT: i32 = 16777216;
pub const MCHT_NOWHERE: i32 = 0;
pub const MCHT_PREV: i32 = 33554432;
pub const MCHT_TITLE: i32 = 65536;
pub const MCHT_TITLEBK: i32 = 65536;
pub const MCHT_TITLEBTNNEXT: i32 = 16842755;
pub const MCHT_TITLEBTNPREV: i32 = 33619971;
pub const MCHT_TITLEMONTH: i32 = 65537;
pub const MCHT_TITLEYEAR: i32 = 65538;
pub const MCHT_TODAYLINK: i32 = 196608;
pub const MCMV_CENTURY: i32 = 3;
pub const MCMV_DECADE: i32 = 2;
pub const MCMV_MAX: i32 = 3;
pub const MCMV_MONTH: i32 = 0;
pub const MCMV_YEAR: i32 = 1;
pub const MCM_FIRST: i32 = 4096;
pub const MCM_GETCALENDARBORDER: i32 = 4127;
pub const MCM_GETCALENDARCOUNT: i32 = 4119;
pub const MCM_GETCALENDARGRIDINFO: i32 = 4120;
pub const MCM_GETCALID: i32 = 4123;
pub const MCM_GETCOLOR: i32 = 4107;
pub const MCM_GETCURRENTVIEW: i32 = 4118;
pub const MCM_GETCURSEL: i32 = 4097;
pub const MCM_GETFIRSTDAYOFWEEK: i32 = 4112;
pub const MCM_GETMAXSELCOUNT: i32 = 4099;
pub const MCM_GETMAXTODAYWIDTH: i32 = 4117;
pub const MCM_GETMINREQRECT: i32 = 4105;
pub const MCM_GETMONTHDELTA: i32 = 4115;
pub const MCM_GETMONTHRANGE: i32 = 4103;
pub const MCM_GETRANGE: i32 = 4113;
pub const MCM_GETSELRANGE: i32 = 4101;
pub const MCM_GETTODAY: i32 = 4109;
pub const MCM_GETUNICODEFORMAT: i32 = 8198;
pub const MCM_HITTEST: i32 = 4110;
pub const MCM_SETCALENDARBORDER: i32 = 4126;
pub const MCM_SETCALID: i32 = 4124;
pub const MCM_SETCOLOR: i32 = 4106;
pub const MCM_SETCURRENTVIEW: i32 = 4128;
pub const MCM_SETCURSEL: i32 = 4098;
pub const MCM_SETDAYSTATE: i32 = 4104;
pub const MCM_SETFIRSTDAYOFWEEK: i32 = 4111;
pub const MCM_SETMAXSELCOUNT: i32 = 4100;
pub const MCM_SETMONTHDELTA: i32 = 4116;
pub const MCM_SETRANGE: i32 = 4114;
pub const MCM_SETSELRANGE: i32 = 4102;
pub const MCM_SETTODAY: i32 = 4108;
pub const MCM_SETUNICODEFORMAT: i32 = 8197;
pub const MCM_SIZERECTTOMIN: i32 = 4125;
pub const MCN_FIRST: u32 = 4294966550;
pub const MCN_GETDAYSTATE: u32 = 4294966549;
pub const MCN_LAST: u32 = 4294966544;
pub const MCN_SELCHANGE: u32 = 4294966547;
pub const MCN_SELECT: u32 = 4294966550;
pub const MCN_VIEWCHANGE: u32 = 4294966546;
pub const MCSC_BACKGROUND: i32 = 0;
pub const MCSC_MONTHBK: i32 = 4;
pub const MCSC_TEXT: i32 = 1;
pub const MCSC_TITLEBK: i32 = 2;
pub const MCSC_TITLETEXT: i32 = 3;
pub const MCSC_TRAILINGTEXT: i32 = 5;
pub const MCS_DAYSTATE: i32 = 1;
pub const MCS_MULTISELECT: i32 = 2;
pub const MCS_NOSELCHANGEONNAV: i32 = 256;
pub const MCS_NOTODAY: i32 = 16;
pub const MCS_NOTODAYCIRCLE: i32 = 8;
pub const MCS_NOTRAILINGDATES: i32 = 64;
pub const MCS_SHORTDAYSOFWEEK: i32 = 128;
pub const MCS_WEEKNUMBERS: i32 = 4;
pub const MINSYSCOMMAND: i32 = 61440;
pub const MONTHCAL_CLASSA: windows_core::PCSTR = windows_core::s!("SysMonthCal32");
pub const MONTHCAL_CLASSW: windows_core::PCWSTR = windows_core::w!("SysMonthCal32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MONTHDAYSTATE(pub u32);
pub const MSGF_COMMCTRL_BEGINDRAG: i32 = 16896;
pub const MSGF_COMMCTRL_DRAGSELECT: i32 = 16898;
pub const MSGF_COMMCTRL_SIZEHEADER: i32 = 16897;
pub const MSGF_COMMCTRL_TOOLBARCUST: i32 = 16899;
pub const NFS_ALL: i32 = 16;
pub const NFS_BUTTON: i32 = 8;
pub const NFS_EDIT: i32 = 1;
pub const NFS_LISTCOMBO: i32 = 4;
pub const NFS_STATIC: i32 = 2;
pub const NFS_USEFONTASSOC: i32 = 32;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMBCDROPDOWN {
    pub hdr: super::NMHDR,
    pub rcButton: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMBCHOTITEM {
    pub hdr: super::NMHDR,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMCBEDRAGBEGINA {
    pub hdr: super::NMHDR,
    pub iItemid: i32,
    pub szText: [i8; 260],
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMCBEDRAGBEGINA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMCBEDRAGBEGINW {
    pub hdr: super::NMHDR,
    pub iItemid: i32,
    pub szText: [u16; 260],
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMCBEDRAGBEGINW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMCBEENDEDITA {
    pub hdr: super::NMHDR,
    pub fChanged: windows_core::BOOL,
    pub iNewSelection: i32,
    pub szText: [i8; 260],
    pub iWhy: i32,
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMCBEENDEDITA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMCBEENDEDITW {
    pub hdr: super::NMHDR,
    pub fChanged: windows_core::BOOL,
    pub iNewSelection: i32,
    pub szText: [u16; 260],
    pub iWhy: i32,
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMCBEENDEDITW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCHAR {
    pub hdr: super::NMHDR,
    pub ch: u32,
    pub dwItemPrev: u32,
    pub dwItemNext: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type NMCLICK = NMMOUSE;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCOMBOBOXEXA {
    pub hdr: super::NMHDR,
    pub ceItem: COMBOBOXEXITEMA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCOMBOBOXEXW {
    pub hdr: super::NMHDR,
    pub ceItem: COMBOBOXEXITEMW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCUSTOMDRAW {
    pub hdr: super::NMHDR,
    pub dwDrawStage: u32,
    pub hdc: super::HDC,
    pub rc: super::RECT,
    pub dwItemSpec: usize,
    pub uItemState: u32,
    pub lItemlParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCUSTOMSPLITRECTINFO {
    pub hdr: super::NMHDR,
    pub rcClient: super::RECT,
    pub rcButton: super::RECT,
    pub rcSplit: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCUSTOMTEXT {
    pub hdr: super::NMHDR,
    pub hDC: super::HDC,
    pub lpString: windows_core::PCWSTR,
    pub nCount: i32,
    pub lpRect: super::LPRECT,
    pub uFormat: u32,
    pub fLink: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMECHANGE {
    pub nmhdr: super::NMHDR,
    pub dwFlags: u32,
    pub st: super::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMDATETIMEFORMATA {
    pub nmhdr: super::NMHDR,
    pub pszFormat: windows_core::PCSTR,
    pub st: super::SYSTEMTIME,
    pub pszDisplay: windows_core::PCSTR,
    pub szDisplay: [i8; 64],
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for NMDATETIMEFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMEFORMATQUERYA {
    pub nmhdr: super::NMHDR,
    pub pszFormat: windows_core::PCSTR,
    pub szMax: super::SIZE,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMEFORMATQUERYW {
    pub nmhdr: super::NMHDR,
    pub pszFormat: windows_core::PCWSTR,
    pub szMax: super::SIZE,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMDATETIMEFORMATW {
    pub nmhdr: super::NMHDR,
    pub pszFormat: windows_core::PCWSTR,
    pub st: super::SYSTEMTIME,
    pub pszDisplay: windows_core::PCWSTR,
    pub szDisplay: [u16; 64],
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for NMDATETIMEFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMESTRINGA {
    pub nmhdr: super::NMHDR,
    pub pszUserString: windows_core::PCSTR,
    pub st: super::SYSTEMTIME,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMESTRINGW {
    pub nmhdr: super::NMHDR,
    pub pszUserString: windows_core::PCWSTR,
    pub st: super::SYSTEMTIME,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMEWMKEYDOWNA {
    pub nmhdr: super::NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: windows_core::PCSTR,
    pub st: super::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMEWMKEYDOWNW {
    pub nmhdr: super::NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: windows_core::PCWSTR,
    pub st: super::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDAYSTATE {
    pub nmhdr: super::NMHDR,
    pub stStart: super::SYSTEMTIME,
    pub cDayState: i32,
    pub prgDayState: LPMONTHDAYSTATE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMHDDISPINFOA {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub mask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMHDDISPINFOW {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub mask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMHDFILTERBTNCLICK {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub rc: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMHEADERA {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub iButton: i32,
    pub pitem: *mut HDITEMA,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for NMHEADERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMHEADERW {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub iButton: i32,
    pub pitem: *mut HDITEMW,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for NMHEADERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMIPADDRESS {
    pub hdr: super::NMHDR,
    pub iField: i32,
    pub iValue: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMITEMACTIVATE {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::POINT,
    pub lParam: super::LPARAM,
    pub uKeyFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMKEY {
    pub hdr: super::NMHDR,
    pub nVKey: u32,
    pub uFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLINK {
    pub hdr: super::NMHDR,
    pub item: LITEM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLISTVIEW {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::POINT,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVCACHEHINT {
    pub hdr: super::NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::COLORREF,
    pub clrTextBk: super::COLORREF,
    pub iSubItem: i32,
    pub dwItemType: u32,
    pub clrFace: super::COLORREF,
    pub iIconEffect: i32,
    pub iIconPhase: i32,
    pub iPartId: i32,
    pub iStateId: i32,
    pub rcText: super::RECT,
    pub uAlign: u32,
}
#[cfg(target_arch = "x86")]
pub const NMLVCUSTOMDRAW_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMLVCUSTOMDRAW_V3_SIZE: u64 = 88;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVDISPINFOA {
    pub hdr: super::NMHDR,
    pub item: LVITEMA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVDISPINFOW {
    pub hdr: super::NMHDR,
    pub item: LVITEMW,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMLVEMPTYMARKUP {
    pub hdr: super::NMHDR,
    pub dwFlags: u32,
    pub szMarkup: [u16; 2084],
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMLVEMPTYMARKUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVFINDITEMA {
    pub hdr: super::NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVFINDITEMW {
    pub hdr: super::NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVGETINFOTIPA {
    pub hdr: super::NMHDR,
    pub dwFlags: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVGETINFOTIPW {
    pub hdr: super::NMHDR,
    pub dwFlags: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::LPARAM,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct NMLVKEYDOWN {
    pub hdr: super::NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVLINK {
    pub hdr: super::NMHDR,
    pub link: LITEM,
    pub iItem: i32,
    pub iSubItem: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVODSTATECHANGE {
    pub hdr: super::NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
    pub uNewState: u32,
    pub uOldState: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVSCROLL {
    pub hdr: super::NMHDR,
    pub dx: i32,
    pub dy: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMMOUSE {
    pub hdr: super::NMHDR,
    pub dwItemSpec: usize,
    pub dwItemData: usize,
    pub pt: super::POINT,
    pub dwHitInfo: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMOBJECTNOTIFY {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub piid: *const windows_core::GUID,
    pub pObject: *mut core::ffi::c_void,
    pub hResult: windows_core::HRESULT,
    pub dwFlags: u32,
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMOBJECTNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMPGCALCSIZE {
    pub hdr: super::NMHDR,
    pub dwFlag: u32,
    pub iWidth: i32,
    pub iHeight: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMPGHOTITEM {
    pub hdr: super::NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct NMPGSCROLL {
    pub hdr: super::NMHDR,
    pub fwKeys: u16,
    pub rcParent: super::RECT,
    pub iDir: i32,
    pub iXpos: i32,
    pub iYpos: i32,
    pub iScroll: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMRBAUTOSIZE {
    pub hdr: super::NMHDR,
    pub fChanged: windows_core::BOOL,
    pub rcTarget: super::RECT,
    pub rcActual: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMREBAR {
    pub hdr: super::NMHDR,
    pub dwMask: u32,
    pub uBand: u32,
    pub fStyle: u32,
    pub wID: u32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMREBARAUTOBREAK {
    pub hdr: super::NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::LPARAM,
    pub uMsg: u32,
    pub fStyleCurrent: u32,
    pub fAutoBreak: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMREBARCHEVRON {
    pub hdr: super::NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::LPARAM,
    pub rc: super::RECT,
    pub lParamNM: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMREBARCHILDSIZE {
    pub hdr: super::NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub rcChild: super::RECT,
    pub rcBand: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMREBARSPLITTER {
    pub hdr: super::NMHDR,
    pub rcSizing: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMSEARCHWEB {
    pub hdr: super::NMHDR,
    pub entrypoint: EC_SEARCHWEB_ENTRYPOINT,
    pub hasQueryText: windows_core::BOOL,
    pub invokeSucceeded: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMSELCHANGE {
    pub nmhdr: super::NMHDR,
    pub stSelStart: super::SYSTEMTIME,
    pub stSelEnd: super::SYSTEMTIME,
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type NMSELECT = NMSELCHANGE;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: super::HBRUSH,
    pub hbrLines: super::HBRUSH,
    pub hpenLines: super::HPEN,
    pub clrText: super::COLORREF,
    pub clrMark: super::COLORREF,
    pub clrTextHighlight: super::COLORREF,
    pub clrBtnFace: super::COLORREF,
    pub clrBtnHighlight: super::COLORREF,
    pub clrHighlightHotTrack: super::COLORREF,
    pub rcText: super::RECT,
    pub nStringBkMode: i32,
    pub nHLStringBkMode: i32,
    pub iListGap: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBDISPINFOA {
    pub hdr: super::NMHDR,
    pub dwMask: u32,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: windows_core::PSTR,
    pub cchText: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBDISPINFOW {
    pub hdr: super::NMHDR,
    pub dwMask: u32,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: windows_core::PWSTR,
    pub cchText: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBGETINFOTIPA {
    pub hdr: super::NMHDR,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBGETINFOTIPW {
    pub hdr: super::NMHDR,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBHOTITEM {
    pub hdr: super::NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMTBRESTORE {
    pub hdr: super::NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub cbBytesPerRecord: i32,
    pub tbButton: TBBUTTON,
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMTBRESTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMTBSAVE {
    pub hdr: super::NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub tbButton: TBBUTTON,
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for NMTBSAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct NMTCKEYDOWN {
    pub hdr: super::NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTOOLBARA {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: windows_core::PSTR,
    pub rcButton: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTOOLBARW {
    pub hdr: super::NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: windows_core::PWSTR,
    pub rcButton: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTOOLTIPSCREATED {
    pub hdr: super::NMHDR,
    pub hwndToolTips: super::HWND,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTRBTHUMBPOSCHANGING {
    pub hdr: super::NMHDR,
    pub dwPos: u32,
    pub nReason: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTREEVIEWA {
    pub hdr: super::NMHDR,
    pub action: u32,
    pub itemOld: TVITEMA,
    pub itemNew: TVITEMA,
    pub ptDrag: super::POINT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTREEVIEWW {
    pub hdr: super::NMHDR,
    pub action: u32,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: super::POINT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTTCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMTTDISPINFOA {
    pub hdr: super::NMHDR,
    pub lpszText: windows_core::PSTR,
    pub szText: [i8; 80],
    pub hinst: super::HINSTANCE,
    pub uFlags: u32,
    pub lParam: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for NMTTDISPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const NMTTDISPINFOA_V1_SIZE: u32 = 104;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMTTDISPINFOA_V1_SIZE: u64 = 124;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMTTDISPINFOW {
    pub hdr: super::NMHDR,
    pub lpszText: windows_core::PWSTR,
    pub szText: [u16; 80],
    pub hinst: super::HINSTANCE,
    pub uFlags: u32,
    pub lParam: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for NMTTDISPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const NMTTDISPINFOW_V1_SIZE: u32 = 184;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMTTDISPINFOW_V1_SIZE: u64 = 204;
#[cfg(target_arch = "x86")]
pub const NMTTDISPINFO_V1_SIZE: u32 = 104;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMTTDISPINFO_V1_SIZE: u64 = 124;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NMTVASYNCDRAW {
    pub hdr: super::NMHDR,
    pub pimldp: *mut IMAGELISTDRAWPARAMS,
    pub hr: windows_core::HRESULT,
    pub hItem: HTREEITEM,
    pub lParam: super::LPARAM,
    pub dwRetFlags: u32,
    pub iRetImageIndex: i32,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for NMTVASYNCDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::COLORREF,
    pub clrTextBk: super::COLORREF,
    pub iLevel: i32,
}
#[cfg(target_arch = "x86")]
pub const NMTVCUSTOMDRAW_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMTVCUSTOMDRAW_V3_SIZE: u64 = 88;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVDISPINFOA {
    pub hdr: super::NMHDR,
    pub item: TVITEMA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVDISPINFOEXA {
    pub hdr: super::NMHDR,
    pub item: TVITEMEXA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVDISPINFOEXW {
    pub hdr: super::NMHDR,
    pub item: TVITEMEXW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVDISPINFOW {
    pub hdr: super::NMHDR,
    pub item: TVITEMW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVGETINFOTIPA {
    pub hdr: super::NMHDR,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVGETINFOTIPW {
    pub hdr: super::NMHDR,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVITEMCHANGE {
    pub hdr: super::NMHDR,
    pub uChanged: u32,
    pub hItem: HTREEITEM,
    pub uStateNew: u32,
    pub uStateOld: u32,
    pub lParam: super::LPARAM,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct NMTVKEYDOWN {
    pub hdr: super::NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTVSTATEIMAGECHANGING {
    pub hdr: super::NMHDR,
    pub hti: HTREEITEM,
    pub iOldStateImageIndex: i32,
    pub iNewStateImageIndex: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMUPDOWN {
    pub hdr: super::NMHDR,
    pub iPos: i32,
    pub iDelta: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMVIEWCHANGE {
    pub nmhdr: super::NMHDR,
    pub dwOldView: u32,
    pub dwNewView: u32,
}
pub const NM_CHAR: u32 = 4294967278;
pub const NM_CLICK: u32 = 4294967294;
pub const NM_CUSTOMDRAW: u32 = 4294967284;
pub const NM_CUSTOMTEXT: u32 = 4294967272;
pub const NM_DBLCLK: u32 = 4294967293;
pub const NM_FIRST: u32 = 0;
pub const NM_FONTCHANGED: u32 = 4294967273;
pub const NM_GETCUSTOMSPLITRECT: u32 = 4294966049;
pub const NM_HOVER: u32 = 4294967283;
pub const NM_KEYDOWN: u32 = 4294967281;
pub const NM_KILLFOCUS: u32 = 4294967288;
pub const NM_LAST: u32 = 4294967197;
pub const NM_LDOWN: u32 = 4294967276;
pub const NM_NCHITTEST: u32 = 4294967282;
pub const NM_OUTOFMEMORY: u32 = 4294967295;
pub const NM_RCLICK: u32 = 4294967291;
pub const NM_RDBLCLK: u32 = 4294967290;
pub const NM_RDOWN: u32 = 4294967275;
pub const NM_RELEASEDCAPTURE: u32 = 4294967280;
pub const NM_RETURN: u32 = 4294967292;
pub const NM_SETCURSOR: u32 = 4294967279;
pub const NM_SETFOCUS: u32 = 4294967289;
pub const NM_THEMECHANGED: u32 = 4294967274;
pub const NM_TOOLTIPSCREATED: u32 = 4294967277;
pub const NM_TVSTATEIMAGECHANGING: u32 = 4294967272;
pub const ODT_HEADER: i32 = 100;
pub const ODT_LISTVIEW: i32 = 102;
pub const ODT_TAB: i32 = 101;
pub const PBM_DELTAPOS: i32 = 1027;
pub const PBM_GETBARCOLOR: i32 = 1039;
pub const PBM_GETBKCOLOR: i32 = 1038;
pub const PBM_GETPOS: i32 = 1032;
pub const PBM_GETRANGE: i32 = 1031;
pub const PBM_GETSTATE: i32 = 1041;
pub const PBM_GETSTEP: i32 = 1037;
pub const PBM_SETBARCOLOR: i32 = 1033;
pub const PBM_SETBKCOLOR: i32 = 8193;
pub const PBM_SETMARQUEE: i32 = 1034;
pub const PBM_SETPOS: i32 = 1026;
pub const PBM_SETRANGE: i32 = 1025;
pub const PBM_SETRANGE32: i32 = 1030;
pub const PBM_SETSTATE: i32 = 1040;
pub const PBM_SETSTEP: i32 = 1028;
pub const PBM_STEPIT: i32 = 1029;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PBRANGE {
    pub iLow: i32,
    pub iHigh: i32,
}
pub const PBST_ERROR: i32 = 2;
pub const PBST_NORMAL: i32 = 1;
pub const PBST_PAUSED: i32 = 3;
pub const PBS_MARQUEE: i32 = 8;
pub const PBS_SMOOTH: i32 = 1;
pub const PBS_SMOOTHREVERSE: i32 = 16;
pub const PBS_VERTICAL: i32 = 4;
#[cfg(feature = "windef")]
pub type PBUTTON_IMAGELIST = *mut BUTTON_IMAGELIST;
#[cfg(feature = "windef")]
pub type PBUTTON_SPLITINFO = *mut BUTTON_SPLITINFO;
#[cfg(feature = "minwindef")]
pub type PCCOMBOEXITEMA = *const COMBOBOXEXITEMA;
#[cfg(feature = "minwindef")]
pub type PCCOMBOEXITEMW = *const COMBOBOXEXITEMW;
#[cfg(feature = "minwindef")]
pub type PCOMBOBOXEXITEMA = *mut COMBOBOXEXITEMA;
#[cfg(feature = "minwindef")]
pub type PCOMBOBOXEXITEMW = *mut COMBOBOXEXITEMW;
pub type PEDITBALLOONTIP = *mut EDITBALLOONTIP;
#[cfg(feature = "minwindef")]
pub type PFNLVCOMPARE = Option<unsafe extern "system" fn(param0: super::LPARAM, param1: super::LPARAM, param2: super::LPARAM) -> i32>;
pub type PFNLVGROUPCOMPARE = Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: *mut core::ffi::c_void) -> i32>;
#[cfg(feature = "minwindef")]
pub type PFNTVCOMPARE = Option<unsafe extern "system" fn(lparam1: super::LPARAM, lparam2: super::LPARAM, lparamsort: super::LPARAM) -> i32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PFTASKDIALOGCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, lprefdata: isize) -> windows_core::HRESULT>;
pub const PGB_BOTTOMORRIGHT: i32 = 1;
pub const PGB_TOPORLEFT: i32 = 0;
pub const PGF_CALCHEIGHT: i32 = 2;
pub const PGF_CALCWIDTH: i32 = 1;
pub const PGF_DEPRESSED: i32 = 4;
pub const PGF_GRAYED: i32 = 2;
pub const PGF_HOT: i32 = 8;
pub const PGF_INVISIBLE: i32 = 0;
pub const PGF_NORMAL: i32 = 1;
pub const PGF_SCROLLDOWN: i32 = 2;
pub const PGF_SCROLLLEFT: i32 = 4;
pub const PGF_SCROLLRIGHT: i32 = 8;
pub const PGF_SCROLLUP: i32 = 1;
pub const PGK_CONTROL: i32 = 2;
pub const PGK_MENU: i32 = 4;
pub const PGK_SHIFT: i32 = 1;
pub const PGM_FIRST: i32 = 5120;
pub const PGM_FORWARDMOUSE: i32 = 5123;
pub const PGM_GETBKCOLOR: i32 = 5125;
pub const PGM_GETBORDER: i32 = 5127;
pub const PGM_GETBUTTONSIZE: i32 = 5131;
pub const PGM_GETBUTTONSTATE: i32 = 5132;
pub const PGM_GETDROPTARGET: i32 = 8196;
pub const PGM_GETPOS: i32 = 5129;
pub const PGM_RECALCSIZE: i32 = 5122;
pub const PGM_SETBKCOLOR: i32 = 5124;
pub const PGM_SETBORDER: i32 = 5126;
pub const PGM_SETBUTTONSIZE: i32 = 5130;
pub const PGM_SETCHILD: i32 = 5121;
pub const PGM_SETPOS: i32 = 5128;
pub const PGM_SETSCROLLINFO: i32 = 5133;
pub const PGN_CALCSIZE: u32 = 4294966394;
pub const PGN_FIRST: u32 = 4294966396;
pub const PGN_HOTITEMCHANGE: u32 = 4294966393;
pub const PGN_LAST: u32 = 4294966346;
pub const PGN_SCROLL: u32 = 4294966395;
pub const PGS_AUTOSCROLL: i32 = 2;
pub const PGS_DRAGNDROP: i32 = 4;
pub const PGS_HORZ: i32 = 1;
pub const PGS_VERT: i32 = 0;
#[cfg(feature = "windef")]
pub type PLHITTESTINFO = *mut LHITTESTINFO;
pub type PLITEM = *mut LITEM;
pub type PLVGROUP = *mut LVGROUP;
#[cfg(feature = "windef")]
pub type PLVGROUPMETRICS = *mut LVGROUPMETRICS;
pub type PLVINSERTGROUPSORTED = *mut LVINSERTGROUPSORTED;
pub type PLVITEMINDEX = *mut LVITEMINDEX;
pub type PLVSETINFOTIP = *mut LVSETINFOTIP;
#[cfg(feature = "minwindef")]
pub type PLVTILEINFO = *mut LVTILEINFO;
#[cfg(feature = "windef")]
pub type PLVTILEVIEWINFO = *mut LVTILEVIEWINFO;
#[cfg(all(feature = "minwinbase", feature = "windef"))]
pub type PMCGRIDINFO = *mut MCGRIDINFO;
#[cfg(all(feature = "minwinbase", feature = "windef"))]
pub type PMCHITTESTINFO = *mut MCHITTESTINFO;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type PNMCBEDRAGBEGINA = *mut NMCBEDRAGBEGINA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type PNMCBEDRAGBEGINW = *mut NMCBEDRAGBEGINW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type PNMCBEENDEDITA = *mut NMCBEENDEDITA;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type PNMCBEENDEDITW = *mut NMCBEENDEDITW;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type PNMCOMBOBOXEXA = *mut NMCOMBOBOXEXA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type PNMCOMBOBOXEXW = *mut NMCOMBOBOXEXW;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type PNMLINK = *mut NMLINK;
#[cfg(all(feature = "windef", feature = "winuser"))]
pub type PNMLVLINK = *mut NMLVLINK;
pub type PPBRANGE = *mut PBRANGE;
pub const PROGRESS_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_progress32");
pub const PROGRESS_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_progress32");
pub type PTBBUTTON = *mut TBBUTTON;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PTOOLINFOA = *mut TTTOOLINFOA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PTOOLINFOW = *mut TTTOOLINFOW;
pub type PTTGETTITLE = *mut TTGETTITLE;
pub const RBAB_ADDBAND: i32 = 2;
pub const RBAB_AUTOSIZE: i32 = 1;
pub const RBBIM_BACKGROUND: i32 = 128;
pub const RBBIM_CHEVRONLOCATION: i32 = 4096;
pub const RBBIM_CHEVRONSTATE: i32 = 8192;
pub const RBBIM_CHILD: i32 = 16;
pub const RBBIM_CHILDSIZE: i32 = 32;
pub const RBBIM_COLORS: i32 = 2;
pub const RBBIM_HEADERSIZE: i32 = 2048;
pub const RBBIM_ID: i32 = 256;
pub const RBBIM_IDEALSIZE: i32 = 512;
pub const RBBIM_IMAGE: i32 = 8;
pub const RBBIM_LPARAM: i32 = 1024;
pub const RBBIM_SIZE: i32 = 64;
pub const RBBIM_STYLE: i32 = 1;
pub const RBBIM_TEXT: i32 = 4;
pub const RBBS_BREAK: i32 = 1;
pub const RBBS_CHILDEDGE: i32 = 4;
pub const RBBS_FIXEDBMP: i32 = 32;
pub const RBBS_FIXEDSIZE: i32 = 2;
pub const RBBS_GRIPPERALWAYS: i32 = 128;
pub const RBBS_HIDDEN: i32 = 8;
pub const RBBS_HIDETITLE: i32 = 1024;
pub const RBBS_NOGRIPPER: i32 = 256;
pub const RBBS_NOVERT: i32 = 16;
pub const RBBS_TOPALIGN: i32 = 2048;
pub const RBBS_USECHEVRON: i32 = 512;
pub const RBBS_VARIABLEHEIGHT: i32 = 64;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RBHITTESTINFO {
    pub pt: super::POINT,
    pub flags: u32,
    pub iBand: i32,
}
pub const RBHT_CAPTION: i32 = 2;
pub const RBHT_CHEVRON: i32 = 8;
pub const RBHT_CLIENT: i32 = 3;
pub const RBHT_GRABBER: i32 = 4;
pub const RBHT_NOWHERE: i32 = 1;
pub const RBHT_SPLITTER: i32 = 16;
pub const RBIM_IMAGELIST: i32 = 1;
pub const RBNM_ID: i32 = 1;
pub const RBNM_LPARAM: i32 = 4;
pub const RBNM_STYLE: i32 = 2;
pub const RBN_AUTOBREAK: u32 = 4294966443;
pub const RBN_AUTOSIZE: u32 = 4294966462;
pub const RBN_BEGINDRAG: u32 = 4294966461;
pub const RBN_CHEVRONPUSHED: u32 = 4294966455;
pub const RBN_CHILDSIZE: u32 = 4294966457;
pub const RBN_DELETEDBAND: u32 = 4294966458;
pub const RBN_DELETINGBAND: u32 = 4294966459;
pub const RBN_ENDDRAG: u32 = 4294966460;
pub const RBN_FIRST: u32 = 4294966465;
pub const RBN_GETOBJECT: u32 = 4294966464;
pub const RBN_HEIGHTCHANGE: u32 = 4294966465;
pub const RBN_LAST: u32 = 4294966437;
pub const RBN_LAYOUTCHANGED: u32 = 4294966463;
pub const RBN_MINMAX: u32 = 4294966444;
pub const RBN_SPLITTERDRAG: u32 = 4294966454;
pub const RBSTR_CHANGERECT: i32 = 1;
pub const RBS_AUTOSIZE: i32 = 8192;
pub const RBS_BANDBORDERS: i32 = 1024;
pub const RBS_DBLCLKTOGGLE: i32 = 32768;
pub const RBS_FIXEDORDER: i32 = 2048;
pub const RBS_REGISTERDROP: i32 = 4096;
pub const RBS_TOOLTIPS: i32 = 256;
pub const RBS_VARHEIGHT: i32 = 512;
pub const RBS_VERTICALGRIPPER: i32 = 16384;
pub const RB_BEGINDRAG: i32 = 1048;
pub const RB_DELETEBAND: i32 = 1026;
pub const RB_DRAGMOVE: i32 = 1050;
pub const RB_ENDDRAG: i32 = 1049;
pub const RB_GETBANDBORDERS: i32 = 1058;
pub const RB_GETBANDCOUNT: i32 = 1036;
pub const RB_GETBANDINFO: i32 = 1053;
pub const RB_GETBANDINFOA: i32 = 1053;
pub const RB_GETBANDINFOW: i32 = 1052;
pub const RB_GETBANDMARGINS: i32 = 1064;
pub const RB_GETBARHEIGHT: i32 = 1051;
pub const RB_GETBARINFO: i32 = 1027;
pub const RB_GETBKCOLOR: i32 = 1044;
pub const RB_GETCOLORSCHEME: i32 = 8195;
pub const RB_GETDROPTARGET: i32 = 8196;
pub const RB_GETEXTENDEDSTYLE: i32 = 1066;
pub const RB_GETPALETTE: i32 = 1062;
pub const RB_GETRECT: i32 = 1033;
pub const RB_GETROWCOUNT: i32 = 1037;
pub const RB_GETROWHEIGHT: i32 = 1038;
pub const RB_GETTEXTCOLOR: i32 = 1046;
pub const RB_GETTOOLTIPS: i32 = 1041;
pub const RB_GETUNICODEFORMAT: i32 = 8198;
pub const RB_HITTEST: i32 = 1032;
pub const RB_IDTOINDEX: i32 = 1040;
pub const RB_INSERTBAND: i32 = 1025;
pub const RB_INSERTBANDA: i32 = 1025;
pub const RB_INSERTBANDW: i32 = 1034;
pub const RB_MAXIMIZEBAND: i32 = 1055;
pub const RB_MINIMIZEBAND: i32 = 1054;
pub const RB_MOVEBAND: i32 = 1063;
pub const RB_PUSHCHEVRON: i32 = 1067;
pub const RB_SETBANDINFO: i32 = 1030;
pub const RB_SETBANDINFOA: i32 = 1030;
pub const RB_SETBANDINFOW: i32 = 1035;
pub const RB_SETBANDWIDTH: i32 = 1068;
pub const RB_SETBARINFO: i32 = 1028;
pub const RB_SETBKCOLOR: i32 = 1043;
pub const RB_SETCOLORSCHEME: i32 = 8194;
pub const RB_SETEXTENDEDSTYLE: i32 = 1065;
pub const RB_SETPALETTE: i32 = 1061;
pub const RB_SETPARENT: i32 = 1031;
pub const RB_SETTEXTCOLOR: i32 = 1045;
pub const RB_SETTOOLTIPS: i32 = 1042;
pub const RB_SETUNICODEFORMAT: i32 = 8197;
pub const RB_SETWINDOWTHEME: i32 = 8203;
pub const RB_SHOWBAND: i32 = 1059;
pub const RB_SIZETORECT: i32 = 1047;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REBARBANDINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::COLORREF,
    pub clrBack: super::COLORREF,
    pub lpText: windows_core::PSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::RECT,
    pub uChevronState: u32,
}
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOA_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOA_V3_SIZE: u64 = 76;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOA_V6_SIZE: u32 = 80;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOA_V6_SIZE: u64 = 108;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REBARBANDINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::COLORREF,
    pub clrBack: super::COLORREF,
    pub lpText: windows_core::PWSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::RECT,
    pub uChevronState: u32,
}
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOW_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOW_V3_SIZE: u64 = 76;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOW_V6_SIZE: u32 = 80;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOW_V6_SIZE: u64 = 108;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFO_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFO_V3_SIZE: u64 = 76;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFO_V6_SIZE: u32 = 80;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFO_V6_SIZE: u64 = 108;
pub const REBARCLASSNAMEA: windows_core::PCSTR = windows_core::s!("ReBarWindow32");
pub const REBARCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("ReBarWindow32");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REBARINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub himl: HIMAGELIST,
}
pub const SBARS_SIZEGRIP: i32 = 256;
pub const SBARS_TOOLTIPS: i32 = 2048;
pub const SBN_FIRST: u32 = 4294966416;
pub const SBN_LAST: u32 = 4294966397;
pub const SBN_SIMPLEMODECHANGE: u32 = 4294966416;
pub const SBT_NOBORDERS: i32 = 256;
pub const SBT_NOTABPARSING: i32 = 2048;
pub const SBT_OWNERDRAW: i32 = 4096;
pub const SBT_POPOUT: i32 = 512;
pub const SBT_RTLREADING: i32 = 1024;
pub const SBT_TOOLTIPS: i32 = 2048;
pub const SB_GETBORDERS: i32 = 1031;
pub const SB_GETICON: i32 = 1044;
pub const SB_GETPARTS: i32 = 1030;
pub const SB_GETRECT: i32 = 1034;
pub const SB_GETTEXT: i32 = 1026;
pub const SB_GETTEXTA: i32 = 1026;
pub const SB_GETTEXTLENGTH: i32 = 1027;
pub const SB_GETTEXTLENGTHA: i32 = 1027;
pub const SB_GETTEXTLENGTHW: i32 = 1036;
pub const SB_GETTEXTW: i32 = 1037;
pub const SB_GETTIPTEXT: i32 = 1042;
pub const SB_GETTIPTEXTA: i32 = 1042;
pub const SB_GETTIPTEXTW: i32 = 1043;
pub const SB_GETUNICODEFORMAT: i32 = 8198;
pub const SB_ISSIMPLE: i32 = 1038;
pub const SB_SETBKCOLOR: i32 = 8193;
pub const SB_SETICON: i32 = 1039;
pub const SB_SETMINHEIGHT: i32 = 1032;
pub const SB_SETPARTS: i32 = 1028;
pub const SB_SETTEXT: i32 = 1025;
pub const SB_SETTEXTA: i32 = 1025;
pub const SB_SETTEXTW: i32 = 1035;
pub const SB_SETTIPTEXT: i32 = 1040;
pub const SB_SETTIPTEXTA: i32 = 1040;
pub const SB_SETTIPTEXTW: i32 = 1041;
pub const SB_SETUNICODEFORMAT: i32 = 8197;
pub const SB_SIMPLE: i32 = 1033;
pub const SB_SIMPLEID: i32 = 255;
pub const STATUSCLASSNAMEA: windows_core::PCSTR = windows_core::s!("msctls_statusbar32");
pub const STATUSCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("msctls_statusbar32");
pub const STD_COPY: i32 = 1;
pub const STD_CUT: i32 = 0;
pub const STD_DELETE: i32 = 5;
pub const STD_FILENEW: i32 = 6;
pub const STD_FILEOPEN: i32 = 7;
pub const STD_FILESAVE: i32 = 8;
pub const STD_FIND: i32 = 12;
pub const STD_HELP: i32 = 11;
pub const STD_PASTE: i32 = 2;
pub const STD_PRINT: i32 = 14;
pub const STD_PRINTPRE: i32 = 9;
pub const STD_PROPERTIES: i32 = 10;
pub const STD_REDOW: i32 = 4;
pub const STD_REPLACE: i32 = 13;
pub const STD_UNDO: i32 = 3;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type SUBCLASSPROC = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM, uidsubclass: usize, dwrefdata: usize) -> super::LRESULT>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct TASKDIALOGCONFIG {
    pub cbSize: u32,
    pub hwndParent: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub dwFlags: TASKDIALOG_FLAGS,
    pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub pszWindowTitle: windows_core::PCWSTR,
    pub Anonymous: TASKDIALOGCONFIG_0,
    pub pszMainInstruction: windows_core::PCWSTR,
    pub pszContent: windows_core::PCWSTR,
    pub cButtons: u32,
    pub pButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultButton: i32,
    pub cRadioButtons: u32,
    pub pRadioButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultRadioButton: i32,
    pub pszVerificationText: windows_core::PCWSTR,
    pub pszExpandedInformation: windows_core::PCWSTR,
    pub pszExpandedControlText: windows_core::PCWSTR,
    pub pszCollapsedControlText: windows_core::PCWSTR,
    pub Anonymous2: TASKDIALOGCONFIG_1,
    pub pszFooter: windows_core::PCWSTR,
    pub pfCallback: PFTASKDIALOGCALLBACK,
    pub lpCallbackData: isize,
    pub cxWidth: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TASKDIALOGCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub union TASKDIALOGCONFIG_0 {
    pub hMainIcon: super::HICON,
    pub pszMainIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TASKDIALOGCONFIG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub union TASKDIALOGCONFIG_1 {
    pub hFooterIcon: super::HICON,
    pub pszFooterIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TASKDIALOGCONFIG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TASKDIALOG_BUTTON {
    pub nButtonID: i32,
    pub pszButtonText: windows_core::PCWSTR,
}
pub type TASKDIALOG_COMMON_BUTTON_FLAGS = i32;
pub type TASKDIALOG_ELEMENTS = i32;
pub type TASKDIALOG_FLAGS = i32;
pub type TASKDIALOG_ICON_ELEMENTS = i32;
pub type TASKDIALOG_MESSAGES = i32;
pub type TASKDIALOG_NOTIFICATIONS = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBADDBITMAP {
    pub hInst: super::HINSTANCE,
    pub nID: usize,
}
pub const TBBF_LARGE: i32 = 1;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 2],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(target_arch = "x86")]
impl Default for TBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 6],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for TBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBBUTTONINFOA {
    pub cbSize: u32,
    pub dwMask: u32,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: windows_core::PSTR,
    pub cchText: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBBUTTONINFOW {
    pub cbSize: u32,
    pub dwMask: u32,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: windows_core::PWSTR,
    pub cchText: i32,
}
pub const TBCDRF_BLENDICON: i32 = 2097152;
pub const TBCDRF_HILITEHOTTRACK: i32 = 131072;
pub const TBCDRF_NOBACKGROUND: i32 = 4194304;
pub const TBCDRF_NOEDGES: i32 = 65536;
pub const TBCDRF_NOETCHEDEFFECT: i32 = 1048576;
pub const TBCDRF_NOMARK: i32 = 524288;
pub const TBCDRF_NOOFFSET: i32 = 262144;
pub const TBCDRF_USECDCOLORS: i32 = 8388608;
pub const TBCD_CHANNEL: i32 = 3;
pub const TBCD_THUMB: i32 = 2;
pub const TBCD_TICS: i32 = 1;
pub const TBDDRET_DEFAULT: i32 = 0;
pub const TBDDRET_NODEFAULT: i32 = 1;
pub const TBDDRET_TREATPRESSED: i32 = 2;
pub const TBIF_BYINDEX: u32 = 2147483648;
pub const TBIF_COMMAND: i32 = 32;
pub const TBIF_IMAGE: i32 = 1;
pub const TBIF_LPARAM: i32 = 16;
pub const TBIF_SIZE: i32 = 64;
pub const TBIF_STATE: i32 = 4;
pub const TBIF_STYLE: i32 = 8;
pub const TBIF_TEXT: i32 = 2;
pub const TBIMHT_AFTER: i32 = 1;
pub const TBIMHT_BACKGROUND: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBINSERTMARK {
    pub iButton: i32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBMETRICS {
    pub cbSize: u32,
    pub dwMask: u32,
    pub cxPad: i32,
    pub cyPad: i32,
    pub cxBarPad: i32,
    pub cyBarPad: i32,
    pub cxButtonSpacing: i32,
    pub cyButtonSpacing: i32,
}
pub const TBMF_BARPAD: i32 = 2;
pub const TBMF_BUTTONSPACING: i32 = 4;
pub const TBMF_PAD: i32 = 1;
pub const TBM_CLEARSEL: i32 = 1043;
pub const TBM_CLEARTICS: i32 = 1033;
pub const TBM_GETBUDDY: i32 = 1057;
pub const TBM_GETCHANNELRECT: i32 = 1050;
pub const TBM_GETLINESIZE: i32 = 1048;
pub const TBM_GETNUMTICS: i32 = 1040;
pub const TBM_GETPAGESIZE: i32 = 1046;
pub const TBM_GETPOS: i32 = 1024;
pub const TBM_GETPTICS: i32 = 1038;
pub const TBM_GETRANGEMAX: i32 = 1026;
pub const TBM_GETRANGEMIN: i32 = 1025;
pub const TBM_GETSELEND: i32 = 1042;
pub const TBM_GETSELSTART: i32 = 1041;
pub const TBM_GETTHUMBLENGTH: i32 = 1052;
pub const TBM_GETTHUMBRECT: i32 = 1049;
pub const TBM_GETTIC: i32 = 1027;
pub const TBM_GETTICPOS: i32 = 1039;
pub const TBM_GETTOOLTIPS: i32 = 1054;
pub const TBM_GETUNICODEFORMAT: i32 = 8198;
pub const TBM_SETBUDDY: i32 = 1056;
pub const TBM_SETLINESIZE: i32 = 1047;
pub const TBM_SETPAGESIZE: i32 = 1045;
pub const TBM_SETPOS: i32 = 1029;
pub const TBM_SETPOSNOTIFY: i32 = 1058;
pub const TBM_SETRANGE: i32 = 1030;
pub const TBM_SETRANGEMAX: i32 = 1032;
pub const TBM_SETRANGEMIN: i32 = 1031;
pub const TBM_SETSEL: i32 = 1034;
pub const TBM_SETSELEND: i32 = 1036;
pub const TBM_SETSELSTART: i32 = 1035;
pub const TBM_SETTHUMBLENGTH: i32 = 1051;
pub const TBM_SETTIC: i32 = 1028;
pub const TBM_SETTICFREQ: i32 = 1044;
pub const TBM_SETTIPSIDE: i32 = 1055;
pub const TBM_SETTOOLTIPS: i32 = 1053;
pub const TBM_SETUNICODEFORMAT: i32 = 8197;
pub const TBNF_DI_SETITEM: i32 = 268435456;
pub const TBNF_IMAGE: i32 = 1;
pub const TBNF_TEXT: i32 = 2;
pub const TBNRF_ENDCUSTOMIZE: i32 = 2;
pub const TBNRF_HIDEHELP: i32 = 1;
pub const TBN_BEGINADJUST: u32 = 4294966593;
pub const TBN_BEGINDRAG: u32 = 4294966595;
pub const TBN_CUSTHELP: u32 = 4294966587;
pub const TBN_DELETINGBUTTON: u32 = 4294966581;
pub const TBN_DRAGOUT: u32 = 4294966582;
pub const TBN_DRAGOVER: u32 = 4294966569;
pub const TBN_DROPDOWN: u32 = 4294966586;
pub const TBN_DUPACCELERATOR: u32 = 4294966571;
pub const TBN_ENDADJUST: u32 = 4294966592;
pub const TBN_ENDDRAG: u32 = 4294966594;
pub const TBN_FIRST: u32 = 4294966596;
pub const TBN_GETBUTTONINFO: u32 = 4294966596;
pub const TBN_GETBUTTONINFOA: u32 = 4294966596;
pub const TBN_GETBUTTONINFOW: u32 = 4294966576;
pub const TBN_GETDISPINFO: u32 = 4294966580;
pub const TBN_GETDISPINFOA: u32 = 4294966580;
pub const TBN_GETDISPINFOW: u32 = 4294966579;
pub const TBN_GETINFOTIP: u32 = 4294966578;
pub const TBN_GETINFOTIPA: u32 = 4294966578;
pub const TBN_GETINFOTIPW: u32 = 4294966577;
pub const TBN_GETOBJECT: u32 = 4294966584;
pub const TBN_HOTITEMCHANGE: u32 = 4294966583;
pub const TBN_INITCUSTOMIZE: u32 = 4294966573;
pub const TBN_LAST: u32 = 4294966576;
pub const TBN_MAPACCELERATOR: u32 = 4294966568;
pub const TBN_QUERYDELETE: u32 = 4294966589;
pub const TBN_QUERYINSERT: u32 = 4294966590;
pub const TBN_RESET: u32 = 4294966591;
pub const TBN_RESTORE: u32 = 4294966575;
pub const TBN_SAVE: u32 = 4294966574;
pub const TBN_TOOLBARCHANGE: u32 = 4294966588;
pub const TBN_WRAPACCELERATOR: u32 = 4294966570;
pub const TBN_WRAPHOTITEM: u32 = 4294966572;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBREPLACEBITMAP {
    pub hInstOld: super::HINSTANCE,
    pub nIDOld: usize,
    pub hInstNew: super::HINSTANCE,
    pub nIDNew: usize,
    pub nButtons: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBSAVEPARAMSA {
    pub hkr: super::HKEY,
    pub pszSubKey: windows_core::PCSTR,
    pub pszValueName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBSAVEPARAMSW {
    pub hkr: super::HKEY,
    pub pszSubKey: windows_core::PCWSTR,
    pub pszValueName: windows_core::PCWSTR,
}
pub const TBSTATE_CHECKED: i32 = 1;
pub const TBSTATE_ELLIPSES: i32 = 64;
pub const TBSTATE_ENABLED: i32 = 4;
pub const TBSTATE_HIDDEN: i32 = 8;
pub const TBSTATE_INDETERMINATE: i32 = 16;
pub const TBSTATE_MARKED: i32 = 128;
pub const TBSTATE_PRESSED: i32 = 2;
pub const TBSTATE_WRAP: i32 = 32;
pub const TBSTYLE_ALTDRAG: i32 = 1024;
pub const TBSTYLE_AUTOSIZE: i32 = 16;
pub const TBSTYLE_BUTTON: i32 = 0;
pub const TBSTYLE_CHECK: i32 = 2;
pub const TBSTYLE_CHECKGROUP: i32 = 6;
pub const TBSTYLE_CUSTOMERASE: i32 = 8192;
pub const TBSTYLE_DROPDOWN: i32 = 8;
pub const TBSTYLE_EX_DOUBLEBUFFER: i32 = 128;
pub const TBSTYLE_EX_DRAWDDARROWS: i32 = 1;
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: i32 = 16;
pub const TBSTYLE_EX_MIXEDBUTTONS: i32 = 8;
pub const TBSTYLE_EX_MULTICOLUMN: i32 = 2;
pub const TBSTYLE_EX_VERTICAL: i32 = 4;
pub const TBSTYLE_FLAT: i32 = 2048;
pub const TBSTYLE_GROUP: i32 = 4;
pub const TBSTYLE_LIST: i32 = 4096;
pub const TBSTYLE_NOPREFIX: i32 = 32;
pub const TBSTYLE_REGISTERDROP: i32 = 16384;
pub const TBSTYLE_SEP: i32 = 1;
pub const TBSTYLE_TOOLTIPS: i32 = 256;
pub const TBSTYLE_TRANSPARENT: i32 = 32768;
pub const TBSTYLE_WRAPABLE: i32 = 512;
pub const TBS_AUTOTICKS: i32 = 1;
pub const TBS_BOTH: i32 = 8;
pub const TBS_BOTTOM: i32 = 0;
pub const TBS_DOWNISLEFT: i32 = 1024;
pub const TBS_ENABLESELRANGE: i32 = 32;
pub const TBS_FIXEDLENGTH: i32 = 64;
pub const TBS_HORZ: i32 = 0;
pub const TBS_LEFT: i32 = 4;
pub const TBS_NOTHUMB: i32 = 128;
pub const TBS_NOTICKS: i32 = 16;
pub const TBS_NOTIFYBEFOREMOVE: i32 = 2048;
pub const TBS_REVERSED: i32 = 512;
pub const TBS_RIGHT: i32 = 0;
pub const TBS_TOOLTIPS: i32 = 256;
pub const TBS_TOP: i32 = 4;
pub const TBS_TRANSPARENTBKGND: i32 = 4096;
pub const TBS_VERT: i32 = 2;
pub const TBTS_BOTTOM: i32 = 2;
pub const TBTS_LEFT: i32 = 1;
pub const TBTS_RIGHT: i32 = 3;
pub const TBTS_TOP: i32 = 0;
pub const TB_ADDBITMAP: i32 = 1043;
pub const TB_ADDBUTTONS: i32 = 1044;
pub const TB_ADDBUTTONSA: i32 = 1044;
pub const TB_ADDBUTTONSW: i32 = 1092;
pub const TB_ADDSTRING: i32 = 1052;
pub const TB_ADDSTRINGA: i32 = 1052;
pub const TB_ADDSTRINGW: i32 = 1101;
pub const TB_AUTOSIZE: i32 = 1057;
pub const TB_BOTTOM: i32 = 7;
pub const TB_BUTTONCOUNT: i32 = 1048;
pub const TB_BUTTONSTRUCTSIZE: i32 = 1054;
pub const TB_CHANGEBITMAP: i32 = 1067;
pub const TB_CHECKBUTTON: i32 = 1026;
pub const TB_COMMANDTOINDEX: i32 = 1049;
pub const TB_CUSTOMIZE: i32 = 1051;
pub const TB_DELETEBUTTON: i32 = 1046;
pub const TB_ENABLEBUTTON: i32 = 1025;
pub const TB_ENDTRACK: i32 = 8;
pub const TB_GETANCHORHIGHLIGHT: i32 = 1098;
pub const TB_GETBITMAP: i32 = 1068;
pub const TB_GETBITMAPFLAGS: i32 = 1065;
pub const TB_GETBUTTON: i32 = 1047;
pub const TB_GETBUTTONINFO: i32 = 1089;
pub const TB_GETBUTTONINFOA: i32 = 1089;
pub const TB_GETBUTTONINFOW: i32 = 1087;
pub const TB_GETBUTTONSIZE: i32 = 1082;
pub const TB_GETBUTTONTEXT: i32 = 1069;
pub const TB_GETBUTTONTEXTA: i32 = 1069;
pub const TB_GETBUTTONTEXTW: i32 = 1099;
pub const TB_GETCOLORSCHEME: i32 = 8195;
pub const TB_GETDISABLEDIMAGELIST: i32 = 1079;
pub const TB_GETEXTENDEDSTYLE: i32 = 1109;
pub const TB_GETHOTIMAGELIST: i32 = 1077;
pub const TB_GETHOTITEM: i32 = 1095;
pub const TB_GETIDEALSIZE: i32 = 1123;
pub const TB_GETIMAGELIST: i32 = 1073;
pub const TB_GETIMAGELISTCOUNT: i32 = 1122;
pub const TB_GETINSERTMARK: i32 = 1103;
pub const TB_GETINSERTMARKCOLOR: i32 = 1113;
pub const TB_GETITEMDROPDOWNRECT: i32 = 1127;
pub const TB_GETITEMRECT: i32 = 1053;
pub const TB_GETMAXSIZE: i32 = 1107;
pub const TB_GETMETRICS: i32 = 1125;
pub const TB_GETOBJECT: i32 = 1086;
pub const TB_GETPADDING: i32 = 1110;
pub const TB_GETPRESSEDIMAGELIST: i32 = 1129;
pub const TB_GETRECT: i32 = 1075;
pub const TB_GETROWS: i32 = 1064;
pub const TB_GETSTATE: i32 = 1042;
pub const TB_GETSTRING: i32 = 1116;
pub const TB_GETSTRINGA: i32 = 1116;
pub const TB_GETSTRINGW: i32 = 1115;
pub const TB_GETSTYLE: i32 = 1081;
pub const TB_GETTEXTROWS: i32 = 1085;
pub const TB_GETTOOLTIPS: i32 = 1059;
pub const TB_GETUNICODEFORMAT: i32 = 8198;
pub const TB_HASACCELERATOR: i32 = 1119;
pub const TB_HIDEBUTTON: i32 = 1028;
pub const TB_HITTEST: i32 = 1093;
pub const TB_INDETERMINATE: i32 = 1029;
pub const TB_INSERTBUTTON: i32 = 1045;
pub const TB_INSERTBUTTONA: i32 = 1045;
pub const TB_INSERTBUTTONW: i32 = 1091;
pub const TB_INSERTMARKHITTEST: i32 = 1105;
pub const TB_ISBUTTONCHECKED: i32 = 1034;
pub const TB_ISBUTTONENABLED: i32 = 1033;
pub const TB_ISBUTTONHIDDEN: i32 = 1036;
pub const TB_ISBUTTONHIGHLIGHTED: i32 = 1038;
pub const TB_ISBUTTONINDETERMINATE: i32 = 1037;
pub const TB_ISBUTTONPRESSED: i32 = 1035;
pub const TB_LINEDOWN: i32 = 1;
pub const TB_LINEUP: i32 = 0;
pub const TB_LOADIMAGES: i32 = 1074;
pub const TB_MAPACCELERATOR: i32 = 1102;
pub const TB_MAPACCELERATORA: i32 = 1102;
pub const TB_MAPACCELERATORW: i32 = 1114;
pub const TB_MARKBUTTON: i32 = 1030;
pub const TB_MOVEBUTTON: i32 = 1106;
pub const TB_PAGEDOWN: i32 = 3;
pub const TB_PAGEUP: i32 = 2;
pub const TB_PRESSBUTTON: i32 = 1027;
pub const TB_REPLACEBITMAP: i32 = 1070;
pub const TB_SAVERESTORE: i32 = 1050;
pub const TB_SAVERESTOREA: i32 = 1050;
pub const TB_SAVERESTOREW: i32 = 1100;
pub const TB_SETANCHORHIGHLIGHT: i32 = 1097;
pub const TB_SETBITMAPSIZE: i32 = 1056;
pub const TB_SETBOUNDINGSIZE: i32 = 1117;
pub const TB_SETBUTTONINFO: i32 = 1090;
pub const TB_SETBUTTONINFOA: i32 = 1090;
pub const TB_SETBUTTONINFOW: i32 = 1088;
pub const TB_SETBUTTONSIZE: i32 = 1055;
pub const TB_SETBUTTONWIDTH: i32 = 1083;
pub const TB_SETCMDID: i32 = 1066;
pub const TB_SETCOLORSCHEME: i32 = 8194;
pub const TB_SETDISABLEDIMAGELIST: i32 = 1078;
pub const TB_SETDRAWTEXTFLAGS: i32 = 1094;
pub const TB_SETEXTENDEDSTYLE: i32 = 1108;
pub const TB_SETHOTIMAGELIST: i32 = 1076;
pub const TB_SETHOTITEM: i32 = 1096;
pub const TB_SETHOTITEM2: i32 = 1118;
pub const TB_SETIMAGELIST: i32 = 1072;
pub const TB_SETINDENT: i32 = 1071;
pub const TB_SETINSERTMARK: i32 = 1104;
pub const TB_SETINSERTMARKCOLOR: i32 = 1112;
pub const TB_SETLISTGAP: i32 = 1120;
pub const TB_SETMAXTEXTROWS: i32 = 1084;
pub const TB_SETMETRICS: i32 = 1126;
pub const TB_SETPADDING: i32 = 1111;
pub const TB_SETPARENT: i32 = 1061;
pub const TB_SETPRESSEDIMAGELIST: i32 = 1128;
pub const TB_SETROWS: i32 = 1063;
pub const TB_SETSTATE: i32 = 1041;
pub const TB_SETSTYLE: i32 = 1080;
pub const TB_SETTOOLTIPS: i32 = 1060;
pub const TB_SETUNICODEFORMAT: i32 = 8197;
pub const TB_SETWINDOWTHEME: i32 = 8203;
pub const TB_THUMBPOSITION: i32 = 4;
pub const TB_THUMBTRACK: i32 = 5;
pub const TB_TOP: i32 = 6;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCHITTESTINFO {
    pub pt: super::POINT,
    pub flags: u32,
}
pub const TCHT_NOWHERE: i32 = 1;
pub const TCHT_ONITEM: i32 = 6;
pub const TCHT_ONITEMICON: i32 = 2;
pub const TCHT_ONITEMLABEL: i32 = 4;
pub const TCIF_IMAGE: i32 = 2;
pub const TCIF_PARAM: i32 = 8;
pub const TCIF_RTLREADING: i32 = 4;
pub const TCIF_STATE: i32 = 16;
pub const TCIF_TEXT: i32 = 1;
pub const TCIS_BUTTONPRESSED: i32 = 1;
pub const TCIS_HIGHLIGHTED: i32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCITEMA {
    pub mask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCITEMHEADERA {
    pub mask: u32,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCITEMHEADERW {
    pub mask: u32,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCITEMW {
    pub mask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::LPARAM,
}
pub const TCM_ADJUSTRECT: i32 = 4904;
pub const TCM_DELETEALLITEMS: i32 = 4873;
pub const TCM_DELETEITEM: i32 = 4872;
pub const TCM_DESELECTALL: i32 = 4914;
pub const TCM_FIRST: i32 = 4864;
pub const TCM_GETCURFOCUS: i32 = 4911;
pub const TCM_GETCURSEL: i32 = 4875;
pub const TCM_GETEXTENDEDSTYLE: i32 = 4917;
pub const TCM_GETIMAGELIST: i32 = 4866;
pub const TCM_GETITEM: i32 = 4869;
pub const TCM_GETITEMA: i32 = 4869;
pub const TCM_GETITEMCOUNT: i32 = 4868;
pub const TCM_GETITEMRECT: i32 = 4874;
pub const TCM_GETITEMW: i32 = 4924;
pub const TCM_GETROWCOUNT: i32 = 4908;
pub const TCM_GETTOOLTIPS: i32 = 4909;
pub const TCM_GETUNICODEFORMAT: i32 = 8198;
pub const TCM_HIGHLIGHTITEM: i32 = 4915;
pub const TCM_HITTEST: i32 = 4877;
pub const TCM_INSERTITEM: i32 = 4871;
pub const TCM_INSERTITEMA: i32 = 4871;
pub const TCM_INSERTITEMW: i32 = 4926;
pub const TCM_REMOVEIMAGE: i32 = 4906;
pub const TCM_SETCURFOCUS: i32 = 4912;
pub const TCM_SETCURSEL: i32 = 4876;
pub const TCM_SETEXTENDEDSTYLE: i32 = 4916;
pub const TCM_SETIMAGELIST: i32 = 4867;
pub const TCM_SETITEM: i32 = 4870;
pub const TCM_SETITEMA: i32 = 4870;
pub const TCM_SETITEMEXTRA: i32 = 4878;
pub const TCM_SETITEMSIZE: i32 = 4905;
pub const TCM_SETITEMW: i32 = 4925;
pub const TCM_SETMINTABWIDTH: i32 = 4913;
pub const TCM_SETPADDING: i32 = 4907;
pub const TCM_SETTOOLTIPS: i32 = 4910;
pub const TCM_SETUNICODEFORMAT: i32 = 8197;
pub const TCN_FIRST: u32 = 4294966746;
pub const TCN_FOCUSCHANGE: u32 = 4294966742;
pub const TCN_GETOBJECT: u32 = 4294966743;
pub const TCN_KEYDOWN: u32 = 4294966746;
pub const TCN_LAST: u32 = 4294966716;
pub const TCN_SELCHANGE: u32 = 4294966745;
pub const TCN_SELCHANGING: u32 = 4294966744;
pub const TCS_BOTTOM: i32 = 2;
pub const TCS_BUTTONS: i32 = 256;
pub const TCS_EX_FLATSEPARATORS: i32 = 1;
pub const TCS_EX_REGISTERDROP: i32 = 2;
pub const TCS_FIXEDWIDTH: i32 = 1024;
pub const TCS_FLATBUTTONS: i32 = 8;
pub const TCS_FOCUSNEVER: i32 = 32768;
pub const TCS_FOCUSONBUTTONDOWN: i32 = 4096;
pub const TCS_FORCEICONLEFT: i32 = 16;
pub const TCS_FORCELABELLEFT: i32 = 32;
pub const TCS_HOTTRACK: i32 = 64;
pub const TCS_MULTILINE: i32 = 512;
pub const TCS_MULTISELECT: i32 = 4;
pub const TCS_OWNERDRAWFIXED: i32 = 8192;
pub const TCS_RAGGEDRIGHT: i32 = 2048;
pub const TCS_RIGHT: i32 = 2;
pub const TCS_RIGHTJUSTIFY: i32 = 0;
pub const TCS_SCROLLOPPOSITE: i32 = 1;
pub const TCS_SINGLELINE: i32 = 0;
pub const TCS_TABS: i32 = 0;
pub const TCS_TOOLTIPS: i32 = 16384;
pub const TCS_VERTICAL: i32 = 128;
pub const TDCBF_CANCEL_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = 8;
pub const TDCBF_CLOSE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = 32;
pub const TDCBF_NO_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = 4;
pub const TDCBF_OK_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = 1;
pub const TDCBF_RETRY_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = 16;
pub const TDCBF_YES_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = 2;
pub const TDE_CONTENT: TASKDIALOG_ELEMENTS = 0;
pub const TDE_EXPANDED_INFORMATION: TASKDIALOG_ELEMENTS = 1;
pub const TDE_FOOTER: TASKDIALOG_ELEMENTS = 2;
pub const TDE_MAIN_INSTRUCTION: TASKDIALOG_ELEMENTS = 3;
pub const TDF_ALLOW_DIALOG_CANCELLATION: TASKDIALOG_FLAGS = 8;
pub const TDF_CALLBACK_TIMER: TASKDIALOG_FLAGS = 2048;
pub const TDF_CAN_BE_MINIMIZED: TASKDIALOG_FLAGS = 32768;
pub const TDF_ENABLE_HYPERLINKS: TASKDIALOG_FLAGS = 1;
pub const TDF_EXPANDED_BY_DEFAULT: TASKDIALOG_FLAGS = 128;
pub const TDF_EXPAND_FOOTER_AREA: TASKDIALOG_FLAGS = 64;
pub const TDF_NO_DEFAULT_RADIO_BUTTON: TASKDIALOG_FLAGS = 16384;
pub const TDF_NO_SET_FOREGROUND: TASKDIALOG_FLAGS = 65536;
pub const TDF_POSITION_RELATIVE_TO_WINDOW: TASKDIALOG_FLAGS = 4096;
pub const TDF_RTL_LAYOUT: TASKDIALOG_FLAGS = 8192;
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: TASKDIALOG_FLAGS = 1024;
pub const TDF_SHOW_PROGRESS_BAR: TASKDIALOG_FLAGS = 512;
pub const TDF_SIZE_TO_CONTENT: TASKDIALOG_FLAGS = 16777216;
pub const TDF_USE_COMMAND_LINKS: TASKDIALOG_FLAGS = 16;
pub const TDF_USE_COMMAND_LINKS_NO_ICON: TASKDIALOG_FLAGS = 32;
pub const TDF_USE_HICON_FOOTER: TASKDIALOG_FLAGS = 4;
pub const TDF_USE_HICON_MAIN: TASKDIALOG_FLAGS = 2;
pub const TDF_VERIFICATION_FLAG_CHECKED: TASKDIALOG_FLAGS = 256;
pub const TDIE_ICON_FOOTER: TASKDIALOG_ICON_ELEMENTS = 1;
pub const TDIE_ICON_MAIN: TASKDIALOG_ICON_ELEMENTS = 0;
pub const TDM_CLICK_BUTTON: TASKDIALOG_MESSAGES = 1126;
pub const TDM_CLICK_RADIO_BUTTON: TASKDIALOG_MESSAGES = 1134;
pub const TDM_CLICK_VERIFICATION: TASKDIALOG_MESSAGES = 1137;
pub const TDM_ENABLE_BUTTON: TASKDIALOG_MESSAGES = 1135;
pub const TDM_ENABLE_RADIO_BUTTON: TASKDIALOG_MESSAGES = 1136;
pub const TDM_NAVIGATE_PAGE: TASKDIALOG_MESSAGES = 1125;
pub const TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE: TASKDIALOG_MESSAGES = 1139;
pub const TDM_SET_ELEMENT_TEXT: TASKDIALOG_MESSAGES = 1132;
pub const TDM_SET_MARQUEE_PROGRESS_BAR: TASKDIALOG_MESSAGES = 1127;
pub const TDM_SET_PROGRESS_BAR_MARQUEE: TASKDIALOG_MESSAGES = 1131;
pub const TDM_SET_PROGRESS_BAR_POS: TASKDIALOG_MESSAGES = 1130;
pub const TDM_SET_PROGRESS_BAR_RANGE: TASKDIALOG_MESSAGES = 1129;
pub const TDM_SET_PROGRESS_BAR_STATE: TASKDIALOG_MESSAGES = 1128;
pub const TDM_UPDATE_ELEMENT_TEXT: TASKDIALOG_MESSAGES = 1138;
pub const TDM_UPDATE_ICON: TASKDIALOG_MESSAGES = 1140;
pub const TDN_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = 2;
pub const TDN_CREATED: TASKDIALOG_NOTIFICATIONS = 0;
pub const TDN_DESTROYED: TASKDIALOG_NOTIFICATIONS = 5;
pub const TDN_DIALOG_CONSTRUCTED: TASKDIALOG_NOTIFICATIONS = 7;
pub const TDN_EXPANDO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = 10;
pub const TDN_HELP: TASKDIALOG_NOTIFICATIONS = 9;
pub const TDN_HYPERLINK_CLICKED: TASKDIALOG_NOTIFICATIONS = 3;
pub const TDN_NAVIGATED: TASKDIALOG_NOTIFICATIONS = 1;
pub const TDN_RADIO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = 6;
pub const TDN_TIMER: TASKDIALOG_NOTIFICATIONS = 4;
pub const TDN_VERIFICATION_CLICKED: TASKDIALOG_NOTIFICATIONS = 8;
pub const TD_ERROR_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65534 as _);
pub const TD_INFORMATION_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65533 as _);
pub const TD_SHIELD_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65532 as _);
pub const TD_WARNING_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65535 as _);
pub const TOOLBARCLASSNAMEA: windows_core::PCSTR = windows_core::s!("ToolbarWindow32");
pub const TOOLBARCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("ToolbarWindow32");
pub const TOOLTIPS_CLASSA: windows_core::PCSTR = windows_core::s!("tooltips_class32");
pub const TOOLTIPS_CLASSW: windows_core::PCWSTR = windows_core::w!("tooltips_class32");
pub const TRACKBAR_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_trackbar32");
pub const TRACKBAR_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_trackbar32");
pub const TRBN_FIRST: u32 = 4294965795;
pub const TRBN_LAST: u32 = 4294965777;
pub const TRBN_THUMBPOSCHANGING: u32 = 4294965794;
pub const TTDT_AUTOMATIC: i32 = 0;
pub const TTDT_AUTOPOP: i32 = 2;
pub const TTDT_INITIAL: i32 = 3;
pub const TTDT_RESHOW: i32 = 1;
pub const TTF_ABSOLUTE: i32 = 128;
pub const TTF_CENTERTIP: i32 = 2;
pub const TTF_DI_SETITEM: i32 = 32768;
pub const TTF_IDISHWND: i32 = 1;
pub const TTF_PARSELINKS: i32 = 4096;
pub const TTF_RTLREADING: i32 = 4;
pub const TTF_SUBCLASS: i32 = 16;
pub const TTF_TRACK: i32 = 32;
pub const TTF_TRANSPARENT: i32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TTGETTITLE {
    pub dwSize: u32,
    pub uTitleBitmap: u32,
    pub cch: u32,
    pub pszTitle: *mut u16,
}
impl Default for TTGETTITLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TTHITTESTINFOA {
    pub hwnd: super::HWND,
    pub pt: super::POINT,
    pub ti: TTTOOLINFOA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TTHITTESTINFOW {
    pub hwnd: super::HWND,
    pub pt: super::POINT,
    pub ti: TTTOOLINFOW,
}
pub const TTI_ERROR: i32 = 3;
pub const TTI_ERROR_LARGE: i32 = 6;
pub const TTI_INFO: i32 = 1;
pub const TTI_INFO_LARGE: i32 = 4;
pub const TTI_NONE: i32 = 0;
pub const TTI_WARNING: i32 = 2;
pub const TTI_WARNING_LARGE: i32 = 5;
pub const TTM_ACTIVATE: i32 = 1025;
pub const TTM_ADDTOOL: i32 = 1028;
pub const TTM_ADDTOOLA: i32 = 1028;
pub const TTM_ADDTOOLW: i32 = 1074;
pub const TTM_ADJUSTRECT: i32 = 1055;
pub const TTM_DELTOOL: i32 = 1029;
pub const TTM_DELTOOLA: i32 = 1029;
pub const TTM_DELTOOLW: i32 = 1075;
pub const TTM_ENUMTOOLS: i32 = 1038;
pub const TTM_ENUMTOOLSA: i32 = 1038;
pub const TTM_ENUMTOOLSW: i32 = 1082;
pub const TTM_GETBUBBLESIZE: i32 = 1054;
pub const TTM_GETCURRENTTOOL: i32 = 1039;
pub const TTM_GETCURRENTTOOLA: i32 = 1039;
pub const TTM_GETCURRENTTOOLW: i32 = 1083;
pub const TTM_GETDELAYTIME: i32 = 1045;
pub const TTM_GETMARGIN: i32 = 1051;
pub const TTM_GETMAXTIPWIDTH: i32 = 1049;
pub const TTM_GETTEXT: i32 = 1035;
pub const TTM_GETTEXTA: i32 = 1035;
pub const TTM_GETTEXTW: i32 = 1080;
pub const TTM_GETTIPBKCOLOR: i32 = 1046;
pub const TTM_GETTIPTEXTCOLOR: i32 = 1047;
pub const TTM_GETTITLE: i32 = 1059;
pub const TTM_GETTOOLCOUNT: i32 = 1037;
pub const TTM_GETTOOLINFO: i32 = 1032;
pub const TTM_GETTOOLINFOA: i32 = 1032;
pub const TTM_GETTOOLINFOW: i32 = 1077;
pub const TTM_HITTEST: i32 = 1034;
pub const TTM_HITTESTA: i32 = 1034;
pub const TTM_HITTESTW: i32 = 1079;
pub const TTM_NEWTOOLRECT: i32 = 1030;
pub const TTM_NEWTOOLRECTA: i32 = 1030;
pub const TTM_NEWTOOLRECTW: i32 = 1076;
pub const TTM_POP: i32 = 1052;
pub const TTM_POPUP: i32 = 1058;
pub const TTM_RELAYEVENT: i32 = 1031;
pub const TTM_SETDELAYTIME: i32 = 1027;
pub const TTM_SETMARGIN: i32 = 1050;
pub const TTM_SETMAXTIPWIDTH: i32 = 1048;
pub const TTM_SETTIPBKCOLOR: i32 = 1043;
pub const TTM_SETTIPTEXTCOLOR: i32 = 1044;
pub const TTM_SETTITLE: i32 = 1056;
pub const TTM_SETTITLEA: i32 = 1056;
pub const TTM_SETTITLEW: i32 = 1057;
pub const TTM_SETTOOLINFO: i32 = 1033;
pub const TTM_SETTOOLINFOA: i32 = 1033;
pub const TTM_SETTOOLINFOW: i32 = 1078;
pub const TTM_SETWINDOWTHEME: i32 = 8203;
pub const TTM_TRACKACTIVATE: i32 = 1041;
pub const TTM_TRACKPOSITION: i32 = 1042;
pub const TTM_UPDATE: i32 = 1053;
pub const TTM_UPDATETIPTEXT: i32 = 1036;
pub const TTM_UPDATETIPTEXTA: i32 = 1036;
pub const TTM_UPDATETIPTEXTW: i32 = 1081;
pub const TTM_WINDOWFROMPOINT: i32 = 1040;
pub const TTN_FIRST: u32 = 4294966776;
pub const TTN_GETDISPINFO: u32 = 4294966776;
pub const TTN_GETDISPINFOA: u32 = 4294966776;
pub const TTN_GETDISPINFOW: u32 = 4294966766;
pub const TTN_LAST: u32 = 4294966747;
pub const TTN_LINKCLICK: u32 = 4294966773;
pub const TTN_NEEDTEXT: u32 = 4294966776;
pub const TTN_NEEDTEXTA: u32 = 4294966776;
pub const TTN_NEEDTEXTW: u32 = 4294966766;
pub const TTN_POP: u32 = 4294966774;
pub const TTN_SHOW: u32 = 4294966775;
pub const TTS_ALWAYSTIP: i32 = 1;
pub const TTS_BALLOON: i32 = 64;
pub const TTS_CLOSE: i32 = 128;
pub const TTS_NOANIMATE: i32 = 16;
pub const TTS_NOFADE: i32 = 32;
pub const TTS_NOPREFIX: i32 = 2;
pub const TTS_USEVISUALSTYLE: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TTTOOLINFOA {
    pub cbSize: u32,
    pub uFlags: u32,
    pub hwnd: super::HWND,
    pub uId: usize,
    pub rect: super::RECT,
    pub hinst: super::HINSTANCE,
    pub lpszText: windows_core::PSTR,
    pub lParam: super::LPARAM,
    pub lpReserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TTTOOLINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOA_V1_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOA_V1_SIZE: u64 = 56;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOA_V2_SIZE: u32 = 44;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOA_V2_SIZE: u64 = 64;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOA_V3_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOA_V3_SIZE: u64 = 72;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TTTOOLINFOW {
    pub cbSize: u32,
    pub uFlags: u32,
    pub hwnd: super::HWND,
    pub uId: usize,
    pub rect: super::RECT,
    pub hinst: super::HINSTANCE,
    pub lpszText: windows_core::PWSTR,
    pub lParam: super::LPARAM,
    pub lpReserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TTTOOLINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOW_V1_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOW_V1_SIZE: u64 = 56;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOW_V2_SIZE: u32 = 44;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOW_V2_SIZE: u64 = 64;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOW_V3_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOW_V3_SIZE: u64 = 72;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFO_V1_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFO_V1_SIZE: u64 = 56;
pub const TVCDRF_NOIMAGES: i32 = 65536;
pub const TVC_BYKEYBOARD: i32 = 2;
pub const TVC_BYMOUSE: i32 = 1;
pub const TVC_UNKNOWN: i32 = 0;
pub const TVE_COLLAPSE: i32 = 1;
pub const TVE_COLLAPSERESET: i32 = 32768;
pub const TVE_EXPAND: i32 = 2;
pub const TVE_EXPANDPARTIAL: i32 = 16384;
pub const TVE_TOGGLE: i32 = 3;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TVGETITEMPARTRECTINFO {
    pub hti: HTREEITEM,
    pub prc: *mut super::RECT,
    pub partID: TVITEMPART,
}
#[cfg(feature = "windef")]
impl Default for TVGETITEMPARTRECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TVGIPR_BUTTON: TVITEMPART = 1;
pub const TVGN_CARET: i32 = 9;
pub const TVGN_CHILD: i32 = 4;
pub const TVGN_DROPHILITE: i32 = 8;
pub const TVGN_FIRSTVISIBLE: i32 = 5;
pub const TVGN_LASTVISIBLE: i32 = 10;
pub const TVGN_NEXT: i32 = 1;
pub const TVGN_NEXTSELECTED: i32 = 11;
pub const TVGN_NEXTVISIBLE: i32 = 6;
pub const TVGN_PARENT: i32 = 3;
pub const TVGN_PREVIOUS: i32 = 2;
pub const TVGN_PREVIOUSVISIBLE: i32 = 7;
pub const TVGN_ROOT: i32 = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVHITTESTINFO {
    pub pt: super::POINT,
    pub flags: u32,
    pub hItem: HTREEITEM,
}
pub const TVHT_ABOVE: i32 = 256;
pub const TVHT_BELOW: i32 = 512;
pub const TVHT_NOWHERE: i32 = 1;
pub const TVHT_ONITEM: i32 = 70;
pub const TVHT_ONITEMBUTTON: i32 = 16;
pub const TVHT_ONITEMICON: i32 = 2;
pub const TVHT_ONITEMINDENT: i32 = 8;
pub const TVHT_ONITEMLABEL: i32 = 4;
pub const TVHT_ONITEMRIGHT: i32 = 32;
pub const TVHT_ONITEMSTATEICON: i32 = 64;
pub const TVHT_TOLEFT: i32 = 2048;
pub const TVHT_TORIGHT: i32 = 1024;
pub const TVIF_CHILDREN: i32 = 64;
pub const TVIF_DI_SETITEM: i32 = 4096;
pub const TVIF_EXPANDEDIMAGE: i32 = 512;
pub const TVIF_HANDLE: i32 = 16;
pub const TVIF_IMAGE: i32 = 2;
pub const TVIF_INTEGRAL: i32 = 128;
pub const TVIF_PARAM: i32 = 4;
pub const TVIF_SELECTEDIMAGE: i32 = 32;
pub const TVIF_STATE: i32 = 8;
pub const TVIF_STATEEX: i32 = 256;
pub const TVIF_TEXT: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct TVINSERTSTRUCTA {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTA_0,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TVINSERTSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub union TVINSERTSTRUCTA_0 {
    pub itemex: TVITEMEXA,
    pub item: TVITEMA,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TVINSERTSTRUCTA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const TVINSERTSTRUCTA_V1_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TVINSERTSTRUCTA_V1_SIZE: u64 = 72;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct TVINSERTSTRUCTW {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTW_0,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TVINSERTSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub union TVINSERTSTRUCTW_0 {
    pub itemex: TVITEMEXW,
    pub item: TVITEMW,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for TVINSERTSTRUCTW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const TVINSERTSTRUCTW_V1_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TVINSERTSTRUCTW_V1_SIZE: u64 = 72;
#[cfg(target_arch = "x86")]
pub const TVINSERTSTRUCT_V1_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TVINSERTSTRUCT_V1_SIZE: u64 = 72;
pub const TVIS_BOLD: i32 = 16;
pub const TVIS_CUT: i32 = 4;
pub const TVIS_DROPHILITED: i32 = 8;
pub const TVIS_EXPANDED: i32 = 32;
pub const TVIS_EXPANDEDONCE: i32 = 64;
pub const TVIS_EXPANDPARTIAL: i32 = 128;
pub const TVIS_EX_ALL: i32 = 2;
pub const TVIS_EX_DISABLED: i32 = 2;
pub const TVIS_EX_FLAT: i32 = 1;
pub const TVIS_OVERLAYMASK: i32 = 3840;
pub const TVIS_SELECTED: i32 = 2;
pub const TVIS_STATEIMAGEMASK: i32 = 61440;
pub const TVIS_USERMASK: i32 = 61440;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVITEMA {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type TVITEMEX = TVITEMEXA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVITEMEXA {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: super::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVITEMEXW {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: super::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
pub type TVITEMPART = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVITEMW {
    pub mask: u32,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: i32,
    pub lParam: super::LPARAM,
}
pub const TVI_FIRST: HTREEITEM = -65535 as _;
pub const TVI_LAST: HTREEITEM = -65534 as _;
pub const TVI_ROOT: HTREEITEM = -65536 as _;
pub const TVI_SORT: HTREEITEM = -65533 as _;
pub const TVM_CREATEDRAGIMAGE: i32 = 4370;
pub const TVM_DELETEITEM: i32 = 4353;
pub const TVM_EDITLABEL: i32 = 4366;
pub const TVM_EDITLABELA: i32 = 4366;
pub const TVM_EDITLABELW: i32 = 4417;
pub const TVM_ENDEDITLABELNOW: i32 = 4374;
pub const TVM_ENSUREVISIBLE: i32 = 4372;
pub const TVM_EXPAND: i32 = 4354;
pub const TVM_GETBKCOLOR: i32 = 4383;
pub const TVM_GETCOUNT: i32 = 4357;
pub const TVM_GETEDITCONTROL: i32 = 4367;
pub const TVM_GETEXTENDEDSTYLE: i32 = 4397;
pub const TVM_GETIMAGELIST: i32 = 4360;
pub const TVM_GETINDENT: i32 = 4358;
pub const TVM_GETINSERTMARKCOLOR: i32 = 4390;
pub const TVM_GETISEARCHSTRING: i32 = 4375;
pub const TVM_GETISEARCHSTRINGA: i32 = 4375;
pub const TVM_GETISEARCHSTRINGW: i32 = 4416;
pub const TVM_GETITEM: i32 = 4364;
pub const TVM_GETITEMA: i32 = 4364;
pub const TVM_GETITEMHEIGHT: i32 = 4380;
pub const TVM_GETITEMPARTRECT: i32 = 4424;
pub const TVM_GETITEMRECT: i32 = 4356;
pub const TVM_GETITEMSTATE: i32 = 4391;
pub const TVM_GETITEMW: i32 = 4414;
pub const TVM_GETLINECOLOR: i32 = 4393;
pub const TVM_GETNEXTITEM: i32 = 4362;
pub const TVM_GETSCROLLTIME: i32 = 4386;
pub const TVM_GETSELECTEDCOUNT: i32 = 4422;
pub const TVM_GETTEXTCOLOR: i32 = 4384;
pub const TVM_GETTOOLTIPS: i32 = 4377;
pub const TVM_GETUNICODEFORMAT: i32 = 8198;
pub const TVM_GETVISIBLECOUNT: i32 = 4368;
pub const TVM_HITTEST: i32 = 4369;
pub const TVM_INSERTITEM: i32 = 4352;
pub const TVM_INSERTITEMA: i32 = 4352;
pub const TVM_INSERTITEMW: i32 = 4402;
pub const TVM_MAPACCIDTOHTREEITEM: i32 = 4394;
pub const TVM_MAPHTREEITEMTOACCID: i32 = 4395;
pub const TVM_SELECTITEM: i32 = 4363;
pub const TVM_SETAUTOSCROLLINFO: i32 = 4411;
pub const TVM_SETBKCOLOR: i32 = 4381;
pub const TVM_SETBORDER: i32 = 4387;
pub const TVM_SETEXTENDEDSTYLE: i32 = 4396;
pub const TVM_SETHOT: i32 = 4410;
pub const TVM_SETIMAGELIST: i32 = 4361;
pub const TVM_SETINDENT: i32 = 4359;
pub const TVM_SETINSERTMARK: i32 = 4378;
pub const TVM_SETINSERTMARKCOLOR: i32 = 4389;
pub const TVM_SETITEM: i32 = 4365;
pub const TVM_SETITEMA: i32 = 4365;
pub const TVM_SETITEMHEIGHT: i32 = 4379;
pub const TVM_SETITEMW: i32 = 4415;
pub const TVM_SETLINECOLOR: i32 = 4392;
pub const TVM_SETSCROLLTIME: i32 = 4385;
pub const TVM_SETTEXTCOLOR: i32 = 4382;
pub const TVM_SETTOOLTIPS: i32 = 4376;
pub const TVM_SETUNICODEFORMAT: i32 = 8197;
pub const TVM_SHOWINFOTIP: i32 = 4423;
pub const TVM_SORTCHILDREN: i32 = 4371;
pub const TVM_SORTCHILDRENCB: i32 = 4373;
pub const TVNRET_DEFAULT: i32 = 0;
pub const TVNRET_SKIPNEW: i32 = 2;
pub const TVNRET_SKIPOLD: i32 = 1;
pub const TVN_ASYNCDRAW: u32 = 4294966876;
pub const TVN_BEGINDRAG: u32 = 4294966889;
pub const TVN_BEGINDRAGA: u32 = 4294966889;
pub const TVN_BEGINDRAGW: u32 = 4294966840;
pub const TVN_BEGINLABELEDIT: u32 = 4294966886;
pub const TVN_BEGINLABELEDITA: u32 = 4294966886;
pub const TVN_BEGINLABELEDITW: u32 = 4294966837;
pub const TVN_BEGINRDRAG: u32 = 4294966888;
pub const TVN_BEGINRDRAGA: u32 = 4294966888;
pub const TVN_BEGINRDRAGW: u32 = 4294966839;
pub const TVN_DELETEITEM: u32 = 4294966887;
pub const TVN_DELETEITEMA: u32 = 4294966887;
pub const TVN_DELETEITEMW: u32 = 4294966838;
pub const TVN_ENDLABELEDIT: u32 = 4294966885;
pub const TVN_ENDLABELEDITA: u32 = 4294966885;
pub const TVN_ENDLABELEDITW: u32 = 4294966836;
pub const TVN_FIRST: u32 = 4294966896;
pub const TVN_GETDISPINFO: u32 = 4294966893;
pub const TVN_GETDISPINFOA: u32 = 4294966893;
pub const TVN_GETDISPINFOW: u32 = 4294966844;
pub const TVN_GETINFOTIP: u32 = 4294966883;
pub const TVN_GETINFOTIPA: u32 = 4294966883;
pub const TVN_GETINFOTIPW: u32 = 4294966882;
pub const TVN_ITEMCHANGED: u32 = 4294966878;
pub const TVN_ITEMCHANGEDA: u32 = 4294966878;
pub const TVN_ITEMCHANGEDW: u32 = 4294966877;
pub const TVN_ITEMCHANGING: u32 = 4294966880;
pub const TVN_ITEMCHANGINGA: u32 = 4294966880;
pub const TVN_ITEMCHANGINGW: u32 = 4294966879;
pub const TVN_ITEMEXPANDED: u32 = 4294966890;
pub const TVN_ITEMEXPANDEDA: u32 = 4294966890;
pub const TVN_ITEMEXPANDEDW: u32 = 4294966841;
pub const TVN_ITEMEXPANDING: u32 = 4294966891;
pub const TVN_ITEMEXPANDINGA: u32 = 4294966891;
pub const TVN_ITEMEXPANDINGW: u32 = 4294966842;
pub const TVN_KEYDOWN: u32 = 4294966884;
pub const TVN_LAST: u32 = 4294966797;
pub const TVN_SELCHANGED: u32 = 4294966894;
pub const TVN_SELCHANGEDA: u32 = 4294966894;
pub const TVN_SELCHANGEDW: u32 = 4294966845;
pub const TVN_SELCHANGING: u32 = 4294966895;
pub const TVN_SELCHANGINGA: u32 = 4294966895;
pub const TVN_SELCHANGINGW: u32 = 4294966846;
pub const TVN_SETDISPINFO: u32 = 4294966892;
pub const TVN_SETDISPINFOA: u32 = 4294966892;
pub const TVN_SETDISPINFOW: u32 = 4294966843;
pub const TVN_SINGLEEXPAND: u32 = 4294966881;
pub const TVSBF_XBORDER: i32 = 1;
pub const TVSBF_YBORDER: i32 = 2;
pub const TVSIL_NORMAL: i32 = 0;
pub const TVSIL_STATE: i32 = 2;
pub const TVSI_NOSINGLEEXPAND: i32 = 32768;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default)]
pub struct TVSORTCB {
    pub hParent: HTREEITEM,
    pub lpfnCompare: PFNTVCOMPARE,
    pub lParam: super::LPARAM,
}
pub const TVS_CHECKBOXES: i32 = 256;
pub const TVS_DISABLEDRAGDROP: i32 = 16;
pub const TVS_EDITLABELS: i32 = 8;
pub const TVS_EX_AUTOHSCROLL: i32 = 32;
pub const TVS_EX_DIMMEDCHECKBOXES: i32 = 512;
pub const TVS_EX_DOUBLEBUFFER: i32 = 4;
pub const TVS_EX_DRAWIMAGEASYNC: i32 = 1024;
pub const TVS_EX_EXCLUSIONCHECKBOXES: i32 = 256;
pub const TVS_EX_FADEINOUTEXPANDOS: i32 = 64;
pub const TVS_EX_MULTISELECT: i32 = 2;
pub const TVS_EX_NOINDENTSTATE: i32 = 8;
pub const TVS_EX_NOSINGLECOLLAPSE: i32 = 1;
pub const TVS_EX_PARTIALCHECKBOXES: i32 = 128;
pub const TVS_EX_RICHTOOLTIP: i32 = 16;
pub const TVS_FULLROWSELECT: i32 = 4096;
pub const TVS_HASBUTTONS: i32 = 1;
pub const TVS_HASLINES: i32 = 2;
pub const TVS_INFOTIP: i32 = 2048;
pub const TVS_LINESATROOT: i32 = 4;
pub const TVS_NOHSCROLL: i32 = 32768;
pub const TVS_NONEVENHEIGHT: i32 = 16384;
pub const TVS_NOSCROLL: i32 = 8192;
pub const TVS_NOTOOLTIPS: i32 = 128;
pub const TVS_RTLREADING: i32 = 64;
pub const TVS_SHOWSELALWAYS: i32 = 32;
pub const TVS_SINGLEEXPAND: i32 = 1024;
pub const TVS_TRACKSELECT: i32 = 512;
pub const TV_FIRST: i32 = 4352;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDACCEL {
    pub nSec: u32,
    pub nInc: u32,
}
pub const UDM_GETACCEL: i32 = 1132;
pub const UDM_GETBASE: i32 = 1134;
pub const UDM_GETBUDDY: i32 = 1130;
pub const UDM_GETPOS: i32 = 1128;
pub const UDM_GETPOS32: i32 = 1138;
pub const UDM_GETRANGE: i32 = 1126;
pub const UDM_GETRANGE32: i32 = 1136;
pub const UDM_GETUNICODEFORMAT: i32 = 8198;
pub const UDM_SETACCEL: i32 = 1131;
pub const UDM_SETBASE: i32 = 1133;
pub const UDM_SETBUDDY: i32 = 1129;
pub const UDM_SETPOS: i32 = 1127;
pub const UDM_SETPOS32: i32 = 1137;
pub const UDM_SETRANGE: i32 = 1125;
pub const UDM_SETRANGE32: i32 = 1135;
pub const UDM_SETUNICODEFORMAT: i32 = 8197;
pub const UDN_DELTAPOS: u32 = 4294966574;
pub const UDN_FIRST: u32 = 4294966575;
pub const UDN_LAST: u32 = 4294966567;
pub const UDS_ALIGNLEFT: i32 = 8;
pub const UDS_ALIGNRIGHT: i32 = 4;
pub const UDS_ARROWKEYS: i32 = 32;
pub const UDS_AUTOBUDDY: i32 = 16;
pub const UDS_HORZ: i32 = 64;
pub const UDS_HOTTRACK: i32 = 256;
pub const UDS_NOTHOUSANDS: i32 = 128;
pub const UDS_SETBUDDYINT: i32 = 2;
pub const UDS_WRAP: i32 = 1;
pub const UD_MAXVAL: i32 = 32767;
pub const UD_MINVAL: i32 = -32767;
pub const UPDOWN_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_updown32");
pub const UPDOWN_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_updown32");
pub const VIEW_DETAILS: i32 = 3;
pub const VIEW_LARGEICONS: i32 = 0;
pub const VIEW_LIST: i32 = 2;
pub const VIEW_NETCONNECT: i32 = 9;
pub const VIEW_NETDISCONNECT: i32 = 10;
pub const VIEW_NEWFOLDER: i32 = 11;
pub const VIEW_PARENTFOLDER: i32 = 8;
pub const VIEW_SMALLICONS: i32 = 1;
pub const VIEW_SORTDATE: i32 = 6;
pub const VIEW_SORTNAME: i32 = 4;
pub const VIEW_SORTSIZE: i32 = 5;
pub const VIEW_SORTTYPE: i32 = 7;
pub const VIEW_VIEWMENU: i32 = 12;
pub const WC_BUTTONA: windows_core::PCSTR = windows_core::s!("Button");
pub const WC_BUTTONW: windows_core::PCWSTR = windows_core::w!("Button");
pub const WC_COMBOBOXA: windows_core::PCSTR = windows_core::s!("ComboBox");
pub const WC_COMBOBOXEXA: windows_core::PCSTR = windows_core::s!("ComboBoxEx32");
pub const WC_COMBOBOXEXW: windows_core::PCWSTR = windows_core::w!("ComboBoxEx32");
pub const WC_COMBOBOXW: windows_core::PCWSTR = windows_core::w!("ComboBox");
pub const WC_EDITA: windows_core::PCSTR = windows_core::s!("Edit");
pub const WC_EDITW: windows_core::PCWSTR = windows_core::w!("Edit");
pub const WC_HEADERA: windows_core::PCSTR = windows_core::s!("SysHeader32");
pub const WC_HEADERW: windows_core::PCWSTR = windows_core::w!("SysHeader32");
pub const WC_IPADDRESSA: windows_core::PCSTR = windows_core::s!("SysIPAddress32");
pub const WC_IPADDRESSW: windows_core::PCWSTR = windows_core::w!("SysIPAddress32");
pub const WC_LINK: windows_core::PCWSTR = windows_core::w!("SysLink");
pub const WC_LISTBOXA: windows_core::PCSTR = windows_core::s!("ListBox");
pub const WC_LISTBOXW: windows_core::PCWSTR = windows_core::w!("ListBox");
pub const WC_LISTVIEWA: windows_core::PCSTR = windows_core::s!("SysListView32");
pub const WC_LISTVIEWW: windows_core::PCWSTR = windows_core::w!("SysListView32");
pub const WC_NATIVEFONTCTLA: windows_core::PCSTR = windows_core::s!("NativeFontCtl");
pub const WC_NATIVEFONTCTLW: windows_core::PCWSTR = windows_core::w!("NativeFontCtl");
pub const WC_PAGESCROLLERA: windows_core::PCSTR = windows_core::s!("SysPager");
pub const WC_PAGESCROLLERW: windows_core::PCWSTR = windows_core::w!("SysPager");
pub const WC_SCROLLBARA: windows_core::PCSTR = windows_core::s!("ScrollBar");
pub const WC_SCROLLBARW: windows_core::PCWSTR = windows_core::w!("ScrollBar");
pub const WC_STATICA: windows_core::PCSTR = windows_core::s!("Static");
pub const WC_STATICW: windows_core::PCWSTR = windows_core::w!("Static");
pub const WC_TABCONTROLA: windows_core::PCSTR = windows_core::s!("SysTabControl32");
pub const WC_TABCONTROLW: windows_core::PCWSTR = windows_core::w!("SysTabControl32");
pub const WC_TREEVIEWA: windows_core::PCSTR = windows_core::s!("SysTreeView32");
pub const WC_TREEVIEWW: windows_core::PCWSTR = windows_core::w!("SysTreeView32");
pub const WMN_FIRST: u32 = 4294966296;
pub const WMN_LAST: u32 = 4294966096;
pub const WSB_PROP_CXHSCROLL: i32 = 2;
pub const WSB_PROP_CXHTHUMB: i32 = 16;
pub const WSB_PROP_CXVSCROLL: i32 = 8;
pub const WSB_PROP_CYHSCROLL: i32 = 4;
pub const WSB_PROP_CYVSCROLL: i32 = 1;
pub const WSB_PROP_CYVTHUMB: i32 = 32;
pub const WSB_PROP_HBKGCOLOR: i32 = 128;
pub const WSB_PROP_HSTYLE: i32 = 512;
pub const WSB_PROP_MASK: i32 = 4095;
pub const WSB_PROP_PALETTE: i32 = 2048;
pub const WSB_PROP_VBKGCOLOR: i32 = 64;
pub const WSB_PROP_VSTYLE: i32 = 256;
pub const WSB_PROP_WINSTYLE: i32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _IMAGELIST(pub u8);
pub type _LI_METRIC = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _TREEITEM(pub u8);
