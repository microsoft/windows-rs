#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateMappedBitmap(hinstance: super::minwindef::HINSTANCE, idbitmap: isize, wflags: u32, lpcolormap: Option<*const COLORMAP>, inummaps: i32) -> super::windef::HBITMAP {
    windows_core::link!("comctl32.dll" "system" fn CreateMappedBitmap(hinstance : super::minwindef::HINSTANCE, idbitmap : isize, wflags : u32, lpcolormap : *const COLORMAP, inummaps : i32) -> super::windef::HBITMAP);
    unsafe { CreateMappedBitmap(hinstance, idbitmap, wflags, lpcolormap.unwrap_or(core::mem::zeroed()) as _, inummaps) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateStatusWindowA<P1>(style: i32, lpsztext: P1, hwndparent: super::windef::HWND, wid: u32) -> super::windef::HWND
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn CreateStatusWindowA(style : i32, lpsztext : windows_core::PCSTR, hwndparent : super::windef::HWND, wid : u32) -> super::windef::HWND);
    unsafe { CreateStatusWindowA(style, lpsztext.param().abi(), hwndparent, wid) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateStatusWindowW<P1>(style: i32, lpsztext: P1, hwndparent: super::windef::HWND, wid: u32) -> super::windef::HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn CreateStatusWindowW(style : i32, lpsztext : windows_core::PCWSTR, hwndparent : super::windef::HWND, wid : u32) -> super::windef::HWND);
    unsafe { CreateStatusWindowW(style, lpsztext.param().abi(), hwndparent, wid) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateToolbarEx(hwnd: super::windef::HWND, ws: u32, wid: u32, nbitmaps: i32, hbminst: super::minwindef::HINSTANCE, wbmid: usize, lpbuttons: *const TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> super::windef::HWND {
    windows_core::link!("comctl32.dll" "system" fn CreateToolbarEx(hwnd : super::windef::HWND, ws : u32, wid : u32, nbitmaps : i32, hbminst : super::minwindef::HINSTANCE, wbmid : usize, lpbuttons : *const TBBUTTON, inumbuttons : i32, dxbutton : i32, dybutton : i32, dxbitmap : i32, dybitmap : i32, ustructsize : u32) -> super::windef::HWND);
    unsafe { CreateToolbarEx(hwnd, ws, wid, nbitmaps, hbminst, wbmid, lpbuttons, inumbuttons, dxbutton, dybutton, dxbitmap, dybitmap, ustructsize) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateUpDownControl(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: super::windef::HWND, nid: i32, hinst: super::minwindef::HINSTANCE, hbuddy: super::windef::HWND, nupper: i32, nlower: i32, npos: i32) -> super::windef::HWND {
    windows_core::link!("comctl32.dll" "system" fn CreateUpDownControl(dwstyle : u32, x : i32, y : i32, cx : i32, cy : i32, hparent : super::windef::HWND, nid : i32, hinst : super::minwindef::HINSTANCE, hbuddy : super::windef::HWND, nupper : i32, nlower : i32, npos : i32) -> super::windef::HWND);
    unsafe { CreateUpDownControl(dwstyle, x, y, cx, cy, hparent, nid, hinst, hbuddy, nupper, nlower, npos) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefSubclassProc(hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("comctl32.dll" "system" fn DefSubclassProc(hwnd : super::windef::HWND, umsg : u32, wparam : super::minwindef::WPARAM, lparam : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { DefSubclassProc(hwnd, umsg, wparam, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawInsert(handparent: super::windef::HWND, hlb: super::windef::HWND, nitem: i32) {
    windows_core::link!("comctl32.dll" "system" fn DrawInsert(handparent : super::windef::HWND, hlb : super::windef::HWND, nitem : i32));
    unsafe { DrawInsert(handparent, hlb, nitem) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawShadowText(hdc: super::windef::HDC, psztext: &[u16], prc: *const super::windef::RECT, dwflags: u32, crtext: super::windef::COLORREF, crshadow: super::windef::COLORREF, ixoffset: i32, iyoffset: i32) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DrawShadowText(hdc : super::windef::HDC, psztext : windows_core::PCWSTR, cch : u32, prc : *const super::windef::RECT, dwflags : u32, crtext : super::windef::COLORREF, crshadow : super::windef::COLORREF, ixoffset : i32, iyoffset : i32) -> i32);
    unsafe { DrawShadowText(hdc, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), prc, dwflags, crtext, crshadow, ixoffset, iyoffset) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawStatusTextA<P2>(hdc: super::windef::HDC, lprc: *const super::windef::RECT, psztext: P2, uflags: u32)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn DrawStatusTextA(hdc : super::windef::HDC, lprc : *const super::windef::RECT, psztext : windows_core::PCSTR, uflags : u32));
    unsafe { DrawStatusTextA(hdc, lprc, psztext.param().abi(), uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawStatusTextW<P2>(hdc: super::windef::HDC, lprc: *const super::windef::RECT, psztext: P2, uflags: u32)
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn DrawStatusTextW(hdc : super::windef::HDC, lprc : *const super::windef::RECT, psztext : windows_core::PCWSTR, uflags : u32));
    unsafe { DrawStatusTextW(hdc, lprc, psztext.param().abi(), uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_EnableScrollBar(param0: super::windef::HWND, param1: i32, param2: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_EnableScrollBar(param0 : super::windef::HWND, param1 : i32, param2 : u32) -> windows_core::BOOL);
    unsafe { FlatSB_EnableScrollBar(param0, param1, param2) }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn FlatSB_GetScrollInfo(param0: super::windef::HWND, code: i32, param2: *mut super::winuser::SCROLLINFO) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollInfo(param0 : super::windef::HWND, code : i32, param2 : *mut super::winuser::SCROLLINFO) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollInfo(param0, code, param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_GetScrollPos(param0: super::windef::HWND, code: i32) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollPos(param0 : super::windef::HWND, code : i32) -> i32);
    unsafe { FlatSB_GetScrollPos(param0, code) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_GetScrollProp(param0: super::windef::HWND, propindex: i32, param2: *mut i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollProp(param0 : super::windef::HWND, propindex : i32, param2 : *mut i32) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollProp(param0, propindex, param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_GetScrollRange(param0: super::windef::HWND, code: i32, param2: *mut i32, param3: *mut i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollRange(param0 : super::windef::HWND, code : i32, param2 : *mut i32, param3 : *mut i32) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollRange(param0, code, param2 as _, param3 as _) }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn FlatSB_SetScrollInfo(param0: super::windef::HWND, code: i32, psi: *mut super::winuser::SCROLLINFO, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollInfo(param0 : super::windef::HWND, code : i32, psi : *mut super::winuser::SCROLLINFO, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollInfo(param0, code, psi as _, fredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_SetScrollPos(param0: super::windef::HWND, code: i32, pos: i32, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollPos(param0 : super::windef::HWND, code : i32, pos : i32, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollPos(param0, code, pos, fredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_SetScrollProp(param0: super::windef::HWND, index: u32, newvalue: isize, param3: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollProp(param0 : super::windef::HWND, index : u32, newvalue : isize, param3 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FlatSB_SetScrollProp(param0, index, newvalue, param3.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_SetScrollRange(param0: super::windef::HWND, code: i32, min: i32, max: i32, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollRange(param0 : super::windef::HWND, code : i32, min : i32, max : i32, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollRange(param0, code, min, max, fredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlatSB_ShowScrollBar(param0: super::windef::HWND, code: i32, param2: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_ShowScrollBar(param0 : super::windef::HWND, code : i32, param2 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FlatSB_ShowScrollBar(param0, code, param2.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetEffectiveClientRect(hwnd: super::windef::HWND, lprc: *mut super::windef::RECT, lpinfo: *const i32) {
    windows_core::link!("comctl32.dll" "system" fn GetEffectiveClientRect(hwnd : super::windef::HWND, lprc : *mut super::windef::RECT, lpinfo : *const i32));
    unsafe { GetEffectiveClientRect(hwnd, lprc as _, lpinfo) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetMUILanguage() -> super::winnt::LANGID {
    windows_core::link!("comctl32.dll" "system" fn GetMUILanguage() -> super::winnt::LANGID);
    unsafe { GetMUILanguage() }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetWindowSubclass(hwnd: super::windef::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize, pdwrefdata: Option<*mut usize>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn GetWindowSubclass(hwnd : super::windef::HWND, pfnsubclass : SUBCLASSPROC, uidsubclass : usize, pdwrefdata : *mut usize) -> windows_core::BOOL);
    unsafe { GetWindowSubclass(hwnd, pfnsubclass, uidsubclass, pdwrefdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HIMAGELIST_QueryInterface(himl: *const _IMAGELIST, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("comctl32.dll" "system" fn HIMAGELIST_QueryInterface(himl : *const _IMAGELIST, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HIMAGELIST_QueryInterface(himl, riid, ppv as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_Add(himl: *const _IMAGELIST, hbmimage: super::windef::HBITMAP, hbmmask: Option<super::windef::HBITMAP>) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Add(himl : *const _IMAGELIST, hbmimage : super::windef::HBITMAP, hbmmask : super::windef::HBITMAP) -> i32);
    unsafe { ImageList_Add(himl, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_AddMasked(himl: *const _IMAGELIST, hbmimage: super::windef::HBITMAP, crmask: super::windef::COLORREF) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_AddMasked(himl : *const _IMAGELIST, hbmimage : super::windef::HBITMAP, crmask : super::windef::COLORREF) -> i32);
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
pub unsafe fn ImageList_DragEnter(hwndlock: super::windef::HWND, x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragEnter(hwndlock : super::windef::HWND, x : i32, y : i32) -> windows_core::BOOL);
    unsafe { ImageList_DragEnter(hwndlock, x, y) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_DragLeave(hwndlock: super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragLeave(hwndlock : super::windef::HWND) -> windows_core::BOOL);
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
pub unsafe fn ImageList_Draw(himl: *const _IMAGELIST, i: i32, hdcdst: super::windef::HDC, x: i32, y: i32, fstyle: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Draw(himl : *const _IMAGELIST, i : i32, hdcdst : super::windef::HDC, x : i32, y : i32, fstyle : u32) -> windows_core::BOOL);
    unsafe { ImageList_Draw(himl, i, hdcdst, x, y, fstyle) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_DrawEx(himl: *const _IMAGELIST, i: i32, hdcdst: super::windef::HDC, x: i32, y: i32, dx: i32, dy: i32, rgbbk: super::windef::COLORREF, rgbfg: super::windef::COLORREF, fstyle: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DrawEx(himl : *const _IMAGELIST, i : i32, hdcdst : super::windef::HDC, x : i32, y : i32, dx : i32, dy : i32, rgbbk : super::windef::COLORREF, rgbfg : super::windef::COLORREF, fstyle : u32) -> windows_core::BOOL);
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
pub unsafe fn ImageList_GetBkColor(himl: *const _IMAGELIST) -> super::windef::COLORREF {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetBkColor(himl : *const _IMAGELIST) -> super::windef::COLORREF);
    unsafe { ImageList_GetBkColor(himl) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_GetDragImage(ppt: Option<*mut super::windef::POINT>, ppthotspot: Option<*mut super::windef::POINT>) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetDragImage(ppt : *mut super::windef::POINT, ppthotspot : *mut super::windef::POINT) -> HIMAGELIST);
    unsafe { ImageList_GetDragImage(ppt.unwrap_or(core::mem::zeroed()) as _, ppthotspot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_GetIcon(himl: *const _IMAGELIST, i: i32, flags: u32) -> super::windef::HICON {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetIcon(himl : *const _IMAGELIST, i : i32, flags : u32) -> super::windef::HICON);
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
pub unsafe fn ImageList_LoadImageA<P1>(hi: super::minwindef::HINSTANCE, lpbmp: P1, cx: i32, cgrow: i32, crmask: super::windef::COLORREF, utype: u32, uflags: u32) -> HIMAGELIST
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_LoadImageA(hi : super::minwindef::HINSTANCE, lpbmp : windows_core::PCSTR, cx : i32, cgrow : i32, crmask : super::windef::COLORREF, utype : u32, uflags : u32) -> HIMAGELIST);
    unsafe { ImageList_LoadImageA(hi, lpbmp.param().abi(), cx, cgrow, crmask, utype, uflags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImageList_LoadImageW<P1>(hi: super::minwindef::HINSTANCE, lpbmp: P1, cx: i32, cgrow: i32, crmask: super::windef::COLORREF, utype: u32, uflags: u32) -> HIMAGELIST
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_LoadImageW(hi : super::minwindef::HINSTANCE, lpbmp : windows_core::PCWSTR, cx : i32, cgrow : i32, crmask : super::windef::COLORREF, utype : u32, uflags : u32) -> HIMAGELIST);
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
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_Read(pstm : *mut core::ffi::c_void) -> HIMAGELIST);
    unsafe { ImageList_Read(pstm.param().abi()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn ImageList_ReadEx<P1>(dwflags: u32, pstm: P1, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_ReadEx(dwflags : u32, pstm : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ImageList_ReadEx(dwflags, pstm.param().abi(), riid, ppv as _) }
}
#[inline]
pub unsafe fn ImageList_Remove(himl: *const _IMAGELIST, i: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Remove(himl : *const _IMAGELIST, i : i32) -> windows_core::BOOL);
    unsafe { ImageList_Remove(himl, i) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_Replace(himl: *const _IMAGELIST, i: i32, hbmimage: super::windef::HBITMAP, hbmmask: Option<super::windef::HBITMAP>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Replace(himl : *const _IMAGELIST, i : i32, hbmimage : super::windef::HBITMAP, hbmmask : super::windef::HBITMAP) -> windows_core::BOOL);
    unsafe { ImageList_Replace(himl, i, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_ReplaceIcon(himl: *const _IMAGELIST, i: i32, hicon: super::windef::HICON) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_ReplaceIcon(himl : *const _IMAGELIST, i : i32, hicon : super::windef::HICON) -> i32);
    unsafe { ImageList_ReplaceIcon(himl, i, hicon) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImageList_SetBkColor(himl: *const _IMAGELIST, clrbk: super::windef::COLORREF) -> super::windef::COLORREF {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetBkColor(himl : *const _IMAGELIST, clrbk : super::windef::COLORREF) -> super::windef::COLORREF);
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
    P1: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_Write(himl : *const _IMAGELIST, pstm : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImageList_Write(himl, pstm.param().abi()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn ImageList_WriteEx<P2>(himl: *const _IMAGELIST, dwflags: u32, pstm: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::objidlbase::IStream>,
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
pub unsafe fn InitMUILanguage(uilang: super::winnt::LANGID) {
    windows_core::link!("comctl32.dll" "system" fn InitMUILanguage(uilang : super::winnt::LANGID));
    unsafe { InitMUILanguage(uilang) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InitializeFlatSB(param0: super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn InitializeFlatSB(param0 : super::windef::HWND) -> windows_core::BOOL);
    unsafe { InitializeFlatSB(param0) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LBItemFromPt(hlb: super::windef::HWND, pt: super::windef::POINT, bautoscroll: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn LBItemFromPt(hlb : super::windef::HWND, pt : super::windef::POINT, bautoscroll : windows_core::BOOL) -> i32);
    unsafe { LBItemFromPt(hlb, core::mem::transmute(pt), bautoscroll.into()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadIconMetric<P1>(hinst: super::minwindef::HINSTANCE, pszname: P1, lims: i32) -> windows_core::Result<super::windef::HICON>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn LoadIconMetric(hinst : super::minwindef::HINSTANCE, pszname : windows_core::PCWSTR, lims : i32, phico : *mut super::windef::HICON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadIconMetric(hinst, pszname.param().abi(), lims, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadIconWithScaleDown<P1>(hinst: super::minwindef::HINSTANCE, pszname: P1, cx: i32, cy: i32) -> windows_core::Result<super::windef::HICON>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn LoadIconWithScaleDown(hinst : super::minwindef::HINSTANCE, pszname : windows_core::PCWSTR, cx : i32, cy : i32, phico : *mut super::windef::HICON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadIconWithScaleDown(hinst, pszname.param().abi(), cx, cy, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MakeDragList(hlb: super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn MakeDragList(hlb : super::windef::HWND) -> windows_core::BOOL);
    unsafe { MakeDragList(hlb) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn MenuHelp(umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, hmainmenu: super::windef::HMENU, hinst: super::minwindef::HINSTANCE, hwndstatus: super::windef::HWND, lpwids: *const u32) {
    windows_core::link!("comctl32.dll" "system" fn MenuHelp(umsg : u32, wparam : super::minwindef::WPARAM, lparam : super::minwindef::LPARAM, hmainmenu : super::windef::HMENU, hinst : super::minwindef::HINSTANCE, hwndstatus : super::windef::HWND, lpwids : *const u32));
    unsafe { MenuHelp(umsg, wparam, lparam, hmainmenu, hinst, hwndstatus, lpwids) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn RemoveWindowSubclass(hwnd: super::windef::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn RemoveWindowSubclass(hwnd : super::windef::HWND, pfnsubclass : SUBCLASSPROC, uidsubclass : usize) -> windows_core::BOOL);
    unsafe { RemoveWindowSubclass(hwnd, pfnsubclass, uidsubclass) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWindowSubclass(hwnd: super::windef::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize, dwrefdata: usize) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn SetWindowSubclass(hwnd : super::windef::HWND, pfnsubclass : SUBCLASSPROC, uidsubclass : usize, dwrefdata : usize) -> windows_core::BOOL);
    unsafe { SetWindowSubclass(hwnd, pfnsubclass, uidsubclass, dwrefdata) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShowHideMenuCtl(hwnd: super::windef::HWND, uflags: usize, lpinfo: *const i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ShowHideMenuCtl(hwnd : super::windef::HWND, uflags : usize, lpinfo : *const i32) -> windows_core::BOOL);
    unsafe { ShowHideMenuCtl(hwnd, uflags, lpinfo) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn TaskDialog<P2, P3, P4, P6>(hwndowner: Option<super::windef::HWND>, hinstance: Option<super::minwindef::HINSTANCE>, pszwindowtitle: P2, pszmaininstruction: P3, pszcontent: P4, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: P6, pnbutton: Option<*mut i32>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn TaskDialog(hwndowner : super::windef::HWND, hinstance : super::minwindef::HINSTANCE, pszwindowtitle : windows_core::PCWSTR, pszmaininstruction : windows_core::PCWSTR, pszcontent : windows_core::PCWSTR, dwcommonbuttons : TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon : windows_core::PCWSTR, pnbutton : *mut i32) -> windows_core::HRESULT);
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
pub unsafe fn UninitializeFlatSB(param0: super::windef::HWND) -> windows_core::HRESULT {
    windows_core::link!("comctl32.dll" "system" fn UninitializeFlatSB(param0 : super::windef::HWND) -> windows_core::HRESULT);
    unsafe { UninitializeFlatSB(param0) }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn _TrackMouseEvent(lpeventtrack: *mut super::winuser::TRACKMOUSEEVENT) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn _TrackMouseEvent(lpeventtrack : *mut super::winuser::TRACKMOUSEEVENT) -> windows_core::BOOL);
    unsafe { _TrackMouseEvent(lpeventtrack as _) }
}
pub const ACM_ISPLAYING: u32 = 1128;
pub const ACM_OPEN: u32 = 1124;
pub const ACM_OPENA: u32 = 1124;
pub const ACM_OPENW: u32 = 1127;
pub const ACM_PLAY: u32 = 1125;
pub const ACM_STOP: u32 = 1126;
pub const ACN_START: u32 = 1;
pub const ACN_STOP: u32 = 2;
pub const ACS_AUTOPLAY: u32 = 4;
pub const ACS_CENTER: u32 = 1;
pub const ACS_TIMER: u32 = 8;
pub const ACS_TRANSPARENT: u32 = 2;
pub const ANIMATE_CLASSA: windows_core::PCSTR = windows_core::s!("SysAnimate32");
pub const ANIMATE_CLASSW: windows_core::PCWSTR = windows_core::w!("SysAnimate32");
pub const BCM_FIRST: u32 = 5632;
pub const BCM_GETIDEALSIZE: u32 = 5633;
pub const BCM_GETIMAGELIST: u32 = 5635;
pub const BCM_GETNOTE: u32 = 5642;
pub const BCM_GETNOTELENGTH: u32 = 5643;
pub const BCM_GETSPLITINFO: u32 = 5640;
pub const BCM_GETTEXTMARGIN: u32 = 5637;
pub const BCM_SETDROPDOWNSTATE: u32 = 5638;
pub const BCM_SETIMAGELIST: u32 = 5634;
pub const BCM_SETNOTE: u32 = 5641;
pub const BCM_SETSHIELD: u32 = 5644;
pub const BCM_SETSPLITINFO: u32 = 5639;
pub const BCM_SETTEXTMARGIN: u32 = 5636;
pub const BCN_DROPDOWN: i32 = -1248;
pub const BCN_FIRST: i32 = -1250;
pub const BCN_HOTITEMCHANGE: i32 = -1249;
pub const BCN_LAST: i32 = -1350;
pub const BCSIF_GLYPH: u32 = 1;
pub const BCSIF_IMAGE: u32 = 2;
pub const BCSIF_SIZE: u32 = 8;
pub const BCSIF_STYLE: u32 = 4;
pub const BCSS_ALIGNLEFT: u32 = 4;
pub const BCSS_IMAGE: u32 = 8;
pub const BCSS_NOSPLIT: u32 = 1;
pub const BCSS_STRETCH: u32 = 2;
pub const BST_DROPDOWNPUSHED: u32 = 1024;
pub const BST_HOT: u32 = 512;
pub const BS_COMMANDLINK: u32 = 14;
pub const BS_DEFCOMMANDLINK: u32 = 15;
pub const BS_DEFSPLITBUTTON: u32 = 13;
pub const BS_SPLITBUTTON: u32 = 12;
pub const BTNS_AUTOSIZE: u32 = 16;
pub const BTNS_BUTTON: u32 = 0;
pub const BTNS_CHECK: u32 = 2;
pub const BTNS_CHECKGROUP: u32 = 6;
pub const BTNS_DROPDOWN: u32 = 8;
pub const BTNS_GROUP: u32 = 4;
pub const BTNS_NOPREFIX: u32 = 32;
pub const BTNS_SEP: u32 = 1;
pub const BTNS_SHOWTEXT: u32 = 64;
pub const BTNS_WHOLEDROPDOWN: u32 = 128;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BUTTON_IMAGELIST {
    pub himl: HIMAGELIST,
    pub margin: super::windef::RECT,
    pub uAlign: u32,
}
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: u32 = 3;
pub const BUTTON_IMAGELIST_ALIGN_CENTER: u32 = 4;
pub const BUTTON_IMAGELIST_ALIGN_LEFT: u32 = 0;
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: u32 = 1;
pub const BUTTON_IMAGELIST_ALIGN_TOP: u32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BUTTON_SPLITINFO {
    pub mask: u32,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: u32,
    pub size: super::windef::SIZE,
}
pub const CBEIF_DI_SETITEM: u32 = 268435456;
pub const CBEIF_IMAGE: u32 = 2;
pub const CBEIF_INDENT: u32 = 16;
pub const CBEIF_LPARAM: u32 = 32;
pub const CBEIF_OVERLAY: u32 = 8;
pub const CBEIF_SELECTEDIMAGE: u32 = 4;
pub const CBEIF_TEXT: u32 = 1;
pub const CBEMAXSTRLEN: u32 = 260;
pub const CBEM_DELETEITEM: u32 = 324;
pub const CBEM_GETCOMBOCONTROL: u32 = 1030;
pub const CBEM_GETEDITCONTROL: u32 = 1031;
pub const CBEM_GETEXSTYLE: u32 = 1033;
pub const CBEM_GETEXTENDEDSTYLE: u32 = 1033;
pub const CBEM_GETIMAGELIST: u32 = 1027;
pub const CBEM_GETITEM: u32 = 1028;
pub const CBEM_GETITEMA: u32 = 1028;
pub const CBEM_GETITEMW: u32 = 1037;
pub const CBEM_GETUNICODEFORMAT: u32 = 8198;
pub const CBEM_HASEDITCHANGED: u32 = 1034;
pub const CBEM_INSERTITEM: u32 = 1025;
pub const CBEM_INSERTITEMA: u32 = 1025;
pub const CBEM_INSERTITEMW: u32 = 1035;
pub const CBEM_SETEXSTYLE: u32 = 1032;
pub const CBEM_SETEXTENDEDSTYLE: u32 = 1038;
pub const CBEM_SETIMAGELIST: u32 = 1026;
pub const CBEM_SETITEM: u32 = 1029;
pub const CBEM_SETITEMA: u32 = 1029;
pub const CBEM_SETITEMW: u32 = 1036;
pub const CBEM_SETUNICODEFORMAT: u32 = 8197;
pub const CBEM_SETWINDOWTHEME: u32 = 8203;
pub const CBENF_DROPDOWN: u32 = 4;
pub const CBENF_ESCAPE: u32 = 3;
pub const CBENF_KILLFOCUS: u32 = 1;
pub const CBENF_RETURN: u32 = 2;
pub const CBEN_BEGINEDIT: i32 = -804;
pub const CBEN_DELETEITEM: i32 = -802;
pub const CBEN_DRAGBEGIN: i32 = -808;
pub const CBEN_DRAGBEGINA: i32 = -808;
pub const CBEN_DRAGBEGINW: i32 = -809;
pub const CBEN_ENDEDIT: i32 = -805;
pub const CBEN_ENDEDITA: i32 = -805;
pub const CBEN_ENDEDITW: i32 = -806;
pub const CBEN_FIRST: i32 = -800;
pub const CBEN_GETDISPINFO: i32 = -800;
pub const CBEN_GETDISPINFOA: i32 = -800;
pub const CBEN_GETDISPINFOW: i32 = -807;
pub const CBEN_INSERTITEM: i32 = -801;
pub const CBEN_LAST: i32 = -830;
pub const CBES_EX_CASESENSITIVE: u32 = 16;
pub const CBES_EX_NOEDITIMAGE: u32 = 1;
pub const CBES_EX_NOEDITIMAGEINDENT: u32 = 2;
pub const CBES_EX_NOSIZELIMIT: u32 = 8;
pub const CBES_EX_PATHWORDBREAKPROC: u32 = 4;
pub const CBES_EX_TEXTENDELLIPSIS: u32 = 32;
pub const CBM_FIRST: u32 = 5888;
pub const CB_GETCUEBANNER: u32 = 5892;
pub const CB_GETMINVISIBLE: u32 = 5890;
pub const CB_SETCUEBANNER: u32 = 5891;
pub const CB_SETMINVISIBLE: u32 = 5889;
pub const CCM_DPISCALE: u32 = 8204;
pub const CCM_FIRST: u32 = 8192;
pub const CCM_GETCOLORSCHEME: u32 = 8195;
pub const CCM_GETDROPTARGET: u32 = 8196;
pub const CCM_GETUNICODEFORMAT: u32 = 8198;
pub const CCM_GETVERSION: u32 = 8200;
pub const CCM_LAST: u32 = 8704;
pub const CCM_SETBKCOLOR: u32 = 8193;
pub const CCM_SETCOLORSCHEME: u32 = 8194;
pub const CCM_SETNOTIFYWINDOW: u32 = 8201;
pub const CCM_SETUNICODEFORMAT: u32 = 8197;
pub const CCM_SETVERSION: u32 = 8199;
pub const CCM_SETWINDOWTHEME: u32 = 8203;
pub const CCS_ADJUSTABLE: u32 = 32;
pub const CCS_BOTTOM: u32 = 3;
pub const CCS_LEFT: u32 = 129;
pub const CCS_NODIVIDER: u32 = 64;
pub const CCS_NOMOVEX: u32 = 130;
pub const CCS_NOMOVEY: u32 = 2;
pub const CCS_NOPARENTALIGN: u32 = 8;
pub const CCS_NORESIZE: u32 = 4;
pub const CCS_RIGHT: u32 = 131;
pub const CCS_TOP: u32 = 1;
pub const CCS_VERT: u32 = 128;
pub const CDDS_ITEM: u32 = 65536;
pub const CDDS_ITEMPOSTERASE: u32 = 65540;
pub const CDDS_ITEMPOSTPAINT: u32 = 65538;
pub const CDDS_ITEMPREERASE: u32 = 65539;
pub const CDDS_ITEMPREPAINT: u32 = 65537;
pub const CDDS_POSTERASE: u32 = 4;
pub const CDDS_POSTPAINT: u32 = 2;
pub const CDDS_PREERASE: u32 = 3;
pub const CDDS_PREPAINT: u32 = 1;
pub const CDDS_SUBITEM: u32 = 131072;
pub const CDIS_CHECKED: u32 = 8;
pub const CDIS_DEFAULT: u32 = 32;
pub const CDIS_DISABLED: u32 = 4;
pub const CDIS_DROPHILITED: u32 = 4096;
pub const CDIS_FOCUS: u32 = 16;
pub const CDIS_GRAYED: u32 = 2;
pub const CDIS_HOT: u32 = 64;
pub const CDIS_INDETERMINATE: u32 = 256;
pub const CDIS_MARKED: u32 = 128;
pub const CDIS_NEARHOT: u32 = 1024;
pub const CDIS_OTHERSIDEHOT: u32 = 2048;
pub const CDIS_SELECTED: u32 = 1;
pub const CDIS_SHOWKEYBOARDCUES: u32 = 512;
pub const CDRF_DODEFAULT: u32 = 0;
pub const CDRF_DOERASE: u32 = 8;
pub const CDRF_NEWFONT: u32 = 2;
pub const CDRF_NOTIFYITEMDRAW: u32 = 32;
pub const CDRF_NOTIFYPOSTERASE: u32 = 64;
pub const CDRF_NOTIFYPOSTPAINT: u32 = 16;
pub const CDRF_NOTIFYSUBITEMDRAW: u32 = 32;
pub const CDRF_SKIPDEFAULT: u32 = 4;
pub const CDRF_SKIPPOSTPAINT: u32 = 256;
pub const CLR_DEFAULT: u32 = 4278190080;
pub const CLR_HILIGHT: i32 = -16777216;
pub const CLR_NONE: u32 = 4294967295;
pub const CMB_MASKED: u32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORMAP {
    pub from: super::windef::COLORREF,
    pub to: super::windef::COLORREF,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORSCHEME {
    pub dwSize: u32,
    pub clrBtnHighlight: super::windef::COLORREF,
    pub clrBtnShadow: super::windef::COLORREF,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMBOBOXEXITEMA {
    pub mask: u32,
    pub iItem: isize,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMBOBOXEXITEMW {
    pub mask: u32,
    pub iItem: isize,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::minwindef::LPARAM,
}
pub const COMCTL32_VERSION: u32 = 6;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATETIMEPICKERINFO {
    pub cbSize: u32,
    pub rcCheck: super::windef::RECT,
    pub stateCheck: u32,
    pub rcButton: super::windef::RECT,
    pub stateButton: u32,
    pub hwndEdit: super::windef::HWND,
    pub hwndUD: super::windef::HWND,
    pub hwndDropDown: super::windef::HWND,
}
pub const DATETIMEPICK_CLASSA: windows_core::PCSTR = windows_core::s!("SysDateTimePick32");
pub const DATETIMEPICK_CLASSW: windows_core::PCWSTR = windows_core::w!("SysDateTimePick32");
pub const DL_BEGINDRAG: u32 = 1157;
pub const DL_CANCELDRAG: u32 = 1160;
pub const DL_COPYCURSOR: u32 = 2;
pub const DL_CURSORSET: u32 = 0;
pub const DL_DRAGGING: u32 = 1158;
pub const DL_DROPPED: u32 = 1159;
pub const DL_MOVECURSOR: u32 = 3;
pub const DL_STOPCURSOR: u32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRAGLISTINFO {
    pub uNotification: u32,
    pub hWnd: super::windef::HWND,
    pub ptCursor: super::windef::POINT,
}
pub const DTM_CLOSEMONTHCAL: u32 = 4109;
pub const DTM_FIRST: u32 = 4096;
pub const DTM_GETDATETIMEPICKERINFO: u32 = 4110;
pub const DTM_GETIDEALSIZE: u32 = 4111;
pub const DTM_GETMCCOLOR: u32 = 4103;
pub const DTM_GETMCFONT: u32 = 4106;
pub const DTM_GETMCSTYLE: u32 = 4108;
pub const DTM_GETMONTHCAL: u32 = 4104;
pub const DTM_GETRANGE: u32 = 4099;
pub const DTM_GETSYSTEMTIME: u32 = 4097;
pub const DTM_SETFORMAT: u32 = 4101;
pub const DTM_SETFORMATA: u32 = 4101;
pub const DTM_SETFORMATW: u32 = 4146;
pub const DTM_SETMCCOLOR: u32 = 4102;
pub const DTM_SETMCFONT: u32 = 4105;
pub const DTM_SETMCSTYLE: u32 = 4107;
pub const DTM_SETRANGE: u32 = 4100;
pub const DTM_SETSYSTEMTIME: u32 = 4098;
pub const DTN_CLOSEUP: i32 = -753;
pub const DTN_DATETIMECHANGE: i32 = -759;
pub const DTN_DROPDOWN: i32 = -754;
pub const DTN_FIRST: i32 = -740;
pub const DTN_FIRST2: i32 = -753;
pub const DTN_FORMAT: i32 = -756;
pub const DTN_FORMATA: i32 = -756;
pub const DTN_FORMATQUERY: i32 = -755;
pub const DTN_FORMATQUERYA: i32 = -755;
pub const DTN_FORMATQUERYW: i32 = -742;
pub const DTN_FORMATW: i32 = -743;
pub const DTN_LAST: i32 = -745;
pub const DTN_LAST2: i32 = -799;
pub const DTN_USERSTRING: i32 = -758;
pub const DTN_USERSTRINGA: i32 = -758;
pub const DTN_USERSTRINGW: i32 = -745;
pub const DTN_WMKEYDOWN: i32 = -757;
pub const DTN_WMKEYDOWNA: i32 = -757;
pub const DTN_WMKEYDOWNW: i32 = -744;
pub const DTS_APPCANPARSE: u32 = 16;
pub const DTS_LONGDATEFORMAT: u32 = 4;
pub const DTS_RIGHTALIGN: u32 = 32;
pub const DTS_SHORTDATECENTURYFORMAT: u32 = 12;
pub const DTS_SHORTDATEFORMAT: u32 = 0;
pub const DTS_SHOWNONE: u32 = 2;
pub const DTS_TIMEFORMAT: u32 = 9;
pub const DTS_UPDOWN: u32 = 1;
pub const ECM_FIRST: u32 = 5376;
pub type EC_ENDOFLINE = i32;
pub const EC_ENDOFLINE_CR: EC_ENDOFLINE = 2;
pub const EC_ENDOFLINE_CRLF: EC_ENDOFLINE = 1;
pub const EC_ENDOFLINE_DETECTFROMCONTENT: EC_ENDOFLINE = 0;
pub const EC_ENDOFLINE_LF: EC_ENDOFLINE = 3;
pub type EC_SEARCHWEB_ENTRYPOINT = i32;
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: EC_SEARCHWEB_ENTRYPOINT = 1;
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: EC_SEARCHWEB_ENTRYPOINT = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EDITBALLOONTIP {
    pub cbStruct: u32,
    pub pszTitle: windows_core::PCWSTR,
    pub pszText: windows_core::PCWSTR,
    pub ttiIcon: i32,
}
pub const EMF_CENTERED: u32 = 1;
pub const EM_ENABLESEARCHWEB: u32 = 5390;
pub const EM_FILELINEFROMCHAR: u32 = 5395;
pub const EM_FILELINEINDEX: u32 = 5396;
pub const EM_FILELINELENGTH: u32 = 5397;
pub const EM_GETCARETINDEX: u32 = 5394;
pub const EM_GETCUEBANNER: u32 = 5378;
pub const EM_GETENDOFLINE: u32 = 5389;
pub const EM_GETEXTENDEDSTYLE: u32 = 5387;
pub const EM_GETFILELINE: u32 = 5398;
pub const EM_GETFILELINECOUNT: u32 = 5399;
pub const EM_GETHILITE: u32 = 5382;
pub const EM_GETZOOM: u32 = 1248;
pub const EM_HIDEBALLOONTIP: u32 = 5380;
pub const EM_NOSETFOCUS: u32 = 5383;
pub const EM_SEARCHWEB: u32 = 5391;
pub const EM_SETCARETINDEX: u32 = 5393;
pub const EM_SETCUEBANNER: u32 = 5377;
pub const EM_SETENDOFLINE: u32 = 5388;
pub const EM_SETEXTENDEDSTYLE: u32 = 5386;
pub const EM_SETHILITE: u32 = 5381;
pub const EM_SETZOOM: u32 = 1249;
pub const EM_SHOWBALLOONTIP: u32 = 5379;
pub const EM_TAKEFOCUS: u32 = 5384;
pub const EN_FIRST: i32 = -1520;
pub const EN_LAST: i32 = -1540;
pub const EN_SEARCHWEB: i32 = -1520;
pub const ES_EX_ALLOWEOL_ALL: u32 = 3;
pub const ES_EX_ALLOWEOL_CR: u32 = 1;
pub const ES_EX_ALLOWEOL_LF: u32 = 2;
pub const ES_EX_CONVERT_EOL_ON_PASTE: u32 = 4;
pub const ES_EX_ZOOMABLE: u32 = 16;
pub const FSB_ENCARTA_MODE: u32 = 1;
pub const FSB_FLAT_MODE: u32 = 2;
pub const FSB_REGULAR_MODE: u32 = 0;
pub const GDTR_MAX: u32 = 2;
pub const GDTR_MIN: u32 = 1;
pub const GDT_ERROR: i32 = -1;
pub const GDT_NONE: u32 = 1;
pub const GDT_VALID: u32 = 0;
pub const GMR_DAYSTATE: u32 = 1;
pub const GMR_VISIBLE: u32 = 0;
#[cfg(feature = "windef")]
pub const HBITMAP_CALLBACK: super::windef::HBITMAP = super::windef::HBITMAP(-1 as _);
pub const HDFT_HASNOVALUE: u32 = 32768;
pub const HDFT_ISDATE: u32 = 2;
pub const HDFT_ISNUMBER: u32 = 1;
pub const HDFT_ISSTRING: u32 = 0;
pub const HDF_BITMAP: u32 = 8192;
pub const HDF_BITMAP_ON_RIGHT: u32 = 4096;
pub const HDF_CENTER: u32 = 2;
pub const HDF_CHECKBOX: u32 = 64;
pub const HDF_CHECKED: u32 = 128;
pub const HDF_FIXEDWIDTH: u32 = 256;
pub const HDF_IMAGE: u32 = 2048;
pub const HDF_JUSTIFYMASK: u32 = 3;
pub const HDF_LEFT: u32 = 0;
pub const HDF_OWNERDRAW: u32 = 32768;
pub const HDF_RIGHT: u32 = 1;
pub const HDF_RTLREADING: u32 = 4;
pub const HDF_SORTDOWN: u32 = 512;
pub const HDF_SORTUP: u32 = 1024;
pub const HDF_SPLITBUTTON: u32 = 16777216;
pub const HDF_STRING: u32 = 16384;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HDHITTESTINFO {
    pub pt: super::windef::POINT,
    pub flags: u32,
    pub iItem: i32,
}
pub const HDIS_FOCUSED: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HDITEMA {
    pub mask: u32,
    pub cxy: i32,
    pub pszText: windows_core::PSTR,
    pub hbm: super::windef::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: super::minwindef::LPARAM,
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
pub const HDITEMA_V1_SIZE: u32 = 40;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HDITEMW {
    pub mask: u32,
    pub cxy: i32,
    pub pszText: windows_core::PWSTR,
    pub hbm: super::windef::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: super::minwindef::LPARAM,
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
pub const HDITEMW_V1_SIZE: u32 = 40;
#[cfg(target_arch = "x86")]
pub const HDITEM_V1_SIZE: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const HDITEM_V1_SIZE: u32 = 40;
pub const HDI_BITMAP: u32 = 16;
pub const HDI_DI_SETITEM: u32 = 64;
pub const HDI_FILTER: u32 = 256;
pub const HDI_FORMAT: u32 = 4;
pub const HDI_HEIGHT: u32 = 1;
pub const HDI_IMAGE: u32 = 32;
pub const HDI_LPARAM: u32 = 8;
pub const HDI_ORDER: u32 = 128;
pub const HDI_STATE: u32 = 512;
pub const HDI_TEXT: u32 = 2;
pub const HDI_WIDTH: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HDLAYOUT {
    pub prc: *mut super::windef::RECT,
    pub pwpos: *mut super::winuser::WINDOWPOS,
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for HDLAYOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HDM_CLEARFILTER: u32 = 4632;
pub const HDM_CREATEDRAGIMAGE: u32 = 4624;
pub const HDM_DELETEITEM: u32 = 4610;
pub const HDM_EDITFILTER: u32 = 4631;
pub const HDM_FIRST: u32 = 4608;
pub const HDM_GETBITMAPMARGIN: u32 = 4629;
pub const HDM_GETFOCUSEDITEM: u32 = 4635;
pub const HDM_GETIMAGELIST: u32 = 4617;
pub const HDM_GETITEM: u32 = 4611;
pub const HDM_GETITEMA: u32 = 4611;
pub const HDM_GETITEMCOUNT: u32 = 4608;
pub const HDM_GETITEMDROPDOWNRECT: u32 = 4633;
pub const HDM_GETITEMRECT: u32 = 4615;
pub const HDM_GETITEMW: u32 = 4619;
pub const HDM_GETORDERARRAY: u32 = 4625;
pub const HDM_GETOVERFLOWRECT: u32 = 4634;
pub const HDM_GETUNICODEFORMAT: u32 = 8198;
pub const HDM_HITTEST: u32 = 4614;
pub const HDM_INSERTITEM: u32 = 4609;
pub const HDM_INSERTITEMA: u32 = 4609;
pub const HDM_INSERTITEMW: u32 = 4618;
pub const HDM_LAYOUT: u32 = 4613;
pub const HDM_ORDERTOINDEX: u32 = 4623;
pub const HDM_SETBITMAPMARGIN: u32 = 4628;
pub const HDM_SETFILTERCHANGETIMEOUT: u32 = 4630;
pub const HDM_SETFOCUSEDITEM: u32 = 4636;
pub const HDM_SETHOTDIVIDER: u32 = 4627;
pub const HDM_SETIMAGELIST: u32 = 4616;
pub const HDM_SETITEM: u32 = 4612;
pub const HDM_SETITEMA: u32 = 4612;
pub const HDM_SETITEMW: u32 = 4620;
pub const HDM_SETORDERARRAY: u32 = 4626;
pub const HDM_SETUNICODEFORMAT: u32 = 8197;
pub const HDN_BEGINDRAG: i32 = -310;
pub const HDN_BEGINFILTEREDIT: i32 = -314;
pub const HDN_BEGINTRACK: i32 = -306;
pub const HDN_BEGINTRACKA: i32 = -306;
pub const HDN_BEGINTRACKW: i32 = -326;
pub const HDN_DIVIDERDBLCLICK: i32 = -305;
pub const HDN_DIVIDERDBLCLICKA: i32 = -305;
pub const HDN_DIVIDERDBLCLICKW: i32 = -325;
pub const HDN_DROPDOWN: i32 = -318;
pub const HDN_ENDDRAG: i32 = -311;
pub const HDN_ENDFILTEREDIT: i32 = -315;
pub const HDN_ENDTRACK: i32 = -307;
pub const HDN_ENDTRACKA: i32 = -307;
pub const HDN_ENDTRACKW: i32 = -327;
pub const HDN_FILTERBTNCLICK: i32 = -313;
pub const HDN_FILTERCHANGE: i32 = -312;
pub const HDN_FIRST: i32 = -300;
pub const HDN_GETDISPINFO: i32 = -309;
pub const HDN_GETDISPINFOA: i32 = -309;
pub const HDN_GETDISPINFOW: i32 = -329;
pub const HDN_ITEMCHANGED: i32 = -301;
pub const HDN_ITEMCHANGEDA: i32 = -301;
pub const HDN_ITEMCHANGEDW: i32 = -321;
pub const HDN_ITEMCHANGING: i32 = -300;
pub const HDN_ITEMCHANGINGA: i32 = -300;
pub const HDN_ITEMCHANGINGW: i32 = -320;
pub const HDN_ITEMCLICK: i32 = -302;
pub const HDN_ITEMCLICKA: i32 = -302;
pub const HDN_ITEMCLICKW: i32 = -322;
pub const HDN_ITEMDBLCLICK: i32 = -303;
pub const HDN_ITEMDBLCLICKA: i32 = -303;
pub const HDN_ITEMDBLCLICKW: i32 = -323;
pub const HDN_ITEMKEYDOWN: i32 = -317;
pub const HDN_ITEMSTATEICONCLICK: i32 = -316;
pub const HDN_LAST: i32 = -399;
pub const HDN_OVERFLOWCLICK: i32 = -319;
pub const HDN_TRACK: i32 = -308;
pub const HDN_TRACKA: i32 = -308;
pub const HDN_TRACKW: i32 = -328;
pub const HDSIL_NORMAL: u32 = 0;
pub const HDSIL_STATE: u32 = 1;
pub const HDS_BUTTONS: u32 = 2;
pub const HDS_CHECKBOXES: u32 = 1024;
pub const HDS_DRAGDROP: u32 = 64;
pub const HDS_FILTERBAR: u32 = 256;
pub const HDS_FLAT: u32 = 512;
pub const HDS_FULLDRAG: u32 = 128;
pub const HDS_HIDDEN: u32 = 8;
pub const HDS_HORZ: u32 = 0;
pub const HDS_HOTTRACK: u32 = 4;
pub const HDS_NOSIZING: u32 = 2048;
pub const HDS_OVERFLOW: u32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HD_TEXTFILTERA {
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HD_TEXTFILTERW {
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
}
pub const HHT_ABOVE: u32 = 256;
pub const HHT_BELOW: u32 = 512;
pub const HHT_NOWHERE: u32 = 1;
pub const HHT_ONDIVIDER: u32 = 4;
pub const HHT_ONDIVOPEN: u32 = 8;
pub const HHT_ONDROPDOWN: u32 = 8192;
pub const HHT_ONFILTER: u32 = 16;
pub const HHT_ONFILTERBUTTON: u32 = 32;
pub const HHT_ONHEADER: u32 = 2;
pub const HHT_ONITEMSTATEICON: u32 = 4096;
pub const HHT_ONOVERFLOW: u32 = 16384;
pub const HHT_TOLEFT: u32 = 2048;
pub const HHT_TORIGHT: u32 = 1024;
pub const HICF_ACCELERATOR: u32 = 4;
pub const HICF_ARROWKEYS: u32 = 2;
pub const HICF_DUPACCEL: u32 = 8;
pub const HICF_ENTERING: u32 = 16;
pub const HICF_LEAVING: u32 = 32;
pub const HICF_LMOUSE: u32 = 128;
pub const HICF_MOUSE: u32 = 1;
pub const HICF_OTHER: u32 = 0;
pub const HICF_RESELECT: u32 = 64;
pub const HICF_TOGGLEDROPDOWN: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMAGELIST(pub *mut _IMAGELIST);
impl HIMAGELIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HIMAGELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub const HINST_COMMCTRL: super::minwindef::HINSTANCE = super::minwindef::HINSTANCE(-1 as _);
pub const HIST_ADDTOFAVORITES: u32 = 3;
pub const HIST_BACK: u32 = 0;
pub const HIST_FAVORITES: u32 = 2;
pub const HIST_FORWARD: u32 = 1;
pub const HIST_VIEWTREE: u32 = 4;
pub const HKCOMB_A: u32 = 8;
pub const HKCOMB_C: u32 = 4;
pub const HKCOMB_CA: u32 = 64;
pub const HKCOMB_NONE: u32 = 1;
pub const HKCOMB_S: u32 = 2;
pub const HKCOMB_SA: u32 = 32;
pub const HKCOMB_SC: u32 = 16;
pub const HKCOMB_SCA: u32 = 128;
pub const HKM_GETHOTKEY: u32 = 1026;
pub const HKM_SETHOTKEY: u32 = 1025;
pub const HKM_SETRULES: u32 = 1027;
pub const HOTKEYF_ALT: u32 = 4;
pub const HOTKEYF_CONTROL: u32 = 2;
pub const HOTKEYF_EXT: u32 = 8;
pub const HOTKEYF_SHIFT: u32 = 1;
pub const HOTKEY_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_hotkey32");
pub const HOTKEY_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_hotkey32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTREEITEM(pub *mut _TREEITEM);
impl HTREEITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HTREEITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICC_ANIMATE_CLASS: u32 = 128;
pub const ICC_BAR_CLASSES: u32 = 4;
pub const ICC_COOL_CLASSES: u32 = 1024;
pub const ICC_DATE_CLASSES: u32 = 256;
pub const ICC_HOTKEY_CLASS: u32 = 64;
pub const ICC_INTERNET_CLASSES: u32 = 2048;
pub const ICC_LINK_CLASS: u32 = 32768;
pub const ICC_LISTVIEW_CLASSES: u32 = 1;
pub const ICC_NATIVEFNTCTL_CLASS: u32 = 8192;
pub const ICC_PAGESCROLLER_CLASS: u32 = 4096;
pub const ICC_PROGRESS_CLASS: u32 = 32;
pub const ICC_STANDARD_CLASSES: u32 = 16384;
pub const ICC_TAB_CLASSES: u32 = 8;
pub const ICC_TREEVIEW_CLASSES: u32 = 2;
pub const ICC_UPDOWN_CLASS: u32 = 16;
pub const ICC_USEREX_CLASSES: u32 = 512;
pub const ICC_WIN95_CLASSES: u32 = 255;
pub const IDB_HIST_DISABLED: u32 = 14;
pub const IDB_HIST_HOT: u32 = 13;
pub const IDB_HIST_LARGE_COLOR: u32 = 9;
pub const IDB_HIST_NORMAL: u32 = 12;
pub const IDB_HIST_PRESSED: u32 = 15;
pub const IDB_HIST_SMALL_COLOR: u32 = 8;
pub const IDB_STD_LARGE_COLOR: u32 = 1;
pub const IDB_STD_SMALL_COLOR: u32 = 0;
pub const IDB_VIEW_LARGE_COLOR: u32 = 5;
pub const IDB_VIEW_SMALL_COLOR: u32 = 4;
pub const ILCF_MOVE: u32 = 0;
pub const ILCF_SWAP: u32 = 1;
pub const ILC_COLOR: u32 = 0;
pub const ILC_COLOR16: u32 = 16;
pub const ILC_COLOR24: u32 = 24;
pub const ILC_COLOR32: u32 = 32;
pub const ILC_COLOR4: u32 = 4;
pub const ILC_COLOR8: u32 = 8;
pub const ILC_COLORDDB: u32 = 254;
pub const ILC_HIGHQUALITYSCALE: u32 = 131072;
pub const ILC_MASK: u32 = 1;
pub const ILC_MIRROR: u32 = 8192;
pub const ILC_ORIGINALSIZE: u32 = 65536;
pub const ILC_PALETTE: u32 = 2048;
pub const ILC_PERITEMMIRROR: u32 = 32768;
pub const ILD_ASYNC: u32 = 32768;
pub const ILD_BLEND: u32 = 4;
pub const ILD_BLEND25: u32 = 2;
pub const ILD_BLEND50: u32 = 4;
pub const ILD_DPISCALE: u32 = 16384;
pub const ILD_FOCUS: u32 = 2;
pub const ILD_IMAGE: u32 = 32;
pub const ILD_MASK: u32 = 16;
pub const ILD_NORMAL: u32 = 0;
pub const ILD_OVERLAYMASK: u32 = 3840;
pub const ILD_PRESERVEALPHA: u32 = 4096;
pub const ILD_ROP: u32 = 64;
pub const ILD_SCALE: u32 = 8192;
pub const ILD_SELECTED: u32 = 4;
pub const ILD_TRANSPARENT: u32 = 1;
pub const ILGT_ASYNC: u32 = 1;
pub const ILGT_NORMAL: u32 = 0;
pub const ILP_DOWNLEVEL: u32 = 1;
pub const ILP_NORMAL: u32 = 0;
pub const ILS_ALPHA: u32 = 8;
pub const ILS_GLOW: u32 = 1;
pub const ILS_NORMAL: u32 = 0;
pub const ILS_SATURATE: u32 = 4;
pub const ILS_SHADOW: u32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGEINFO {
    pub hbmImage: super::windef::HBITMAP,
    pub hbmMask: super::windef::HBITMAP,
    pub Unused1: i32,
    pub Unused2: i32,
    pub rcImage: super::windef::RECT,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGELISTDRAWPARAMS {
    pub cbSize: u32,
    pub himl: HIMAGELIST,
    pub i: i32,
    pub hdcDst: super::windef::HDC,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub xBitmap: i32,
    pub yBitmap: i32,
    pub rgbBk: super::windef::COLORREF,
    pub rgbFg: super::windef::COLORREF,
    pub fStyle: u32,
    pub dwRop: u32,
    pub fState: u32,
    pub Frame: u32,
    pub crEffect: super::windef::COLORREF,
}
#[cfg(target_arch = "x86")]
pub const IMAGELISTDRAWPARAMS_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IMAGELISTDRAWPARAMS_V3_SIZE: u32 = 72;
pub const INFOTIPSIZE: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: u32,
}
pub const INVALID_LINK_INDEX: i32 = -1;
pub const IPM_CLEARADDRESS: u32 = 1124;
pub const IPM_GETADDRESS: u32 = 1126;
pub const IPM_ISBLANK: u32 = 1129;
pub const IPM_SETADDRESS: u32 = 1125;
pub const IPM_SETFOCUS: u32 = 1128;
pub const IPM_SETRANGE: u32 = 1127;
pub const IPN_FIELDCHANGED: i32 = -860;
pub const IPN_FIRST: i32 = -860;
pub const IPN_LAST: i32 = -879;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LHITTESTINFO {
    pub pt: super::windef::POINT,
    pub item: LITEM,
}
pub const LIF_ITEMID: u32 = 4;
pub const LIF_ITEMINDEX: u32 = 1;
pub const LIF_STATE: u32 = 2;
pub const LIF_URL: u32 = 8;
pub const LIM_LARGE: _LI_METRIC = 1;
pub const LIM_SMALL: _LI_METRIC = 0;
pub const LIS_DEFAULTCOLORS: u32 = 16;
pub const LIS_ENABLED: u32 = 2;
pub const LIS_FOCUSED: u32 = 1;
pub const LIS_HOTTRACK: u32 = 8;
pub const LIS_VISITED: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const LM_GETIDEALHEIGHT: u32 = 1793;
pub const LM_GETIDEALSIZE: u32 = 1793;
pub const LM_GETITEM: u32 = 1795;
pub const LM_HITTEST: u32 = 1792;
pub const LM_SETITEM: u32 = 1794;
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORMAP(pub *mut COLORMAP);
#[cfg(feature = "windef")]
impl LPCOLORMAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPCOLORMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORSCHEME(pub *mut COLORSCHEME);
#[cfg(feature = "windef")]
impl LPCOLORSCHEME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPCOLORSCHEME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCREBARBANDINFOA(pub *const REBARBANDINFOA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPCREBARBANDINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPCREBARBANDINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCREBARBANDINFOW(pub *const REBARBANDINFOW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPCREBARBANDINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPCREBARBANDINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCTBBUTTON(pub *const TBBUTTON);
impl LPCTBBUTTON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCTBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDATETIMEPICKERINFO(pub *mut DATETIMEPICKERINFO);
#[cfg(feature = "windef")]
impl LPDATETIMEPICKERINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPDATETIMEPICKERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDRAGLISTINFO(pub *mut DRAGLISTINFO);
#[cfg(feature = "windef")]
impl LPDRAGLISTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPDRAGLISTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFINDINFOA(pub *mut LVFINDINFOA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPFINDINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPFINDINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFINDINFOW(pub *mut LVFINDINFOW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPFINDINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPFINDINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHDHITTESTINFO(pub *mut HDHITTESTINFO);
#[cfg(feature = "windef")]
impl LPHDHITTESTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPHDHITTESTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHDITEMA(pub *mut HDITEMA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPHDITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPHDITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHDITEMW(pub *mut HDITEMW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPHDITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPHDITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHDLAYOUT(pub *mut HDLAYOUT);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPHDLAYOUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPHDLAYOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHD_TEXTFILTERA(pub *mut HD_TEXTFILTERA);
impl LPHD_TEXTFILTERA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHD_TEXTFILTERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHD_TEXTFILTERW(pub *mut HD_TEXTFILTERW);
impl LPHD_TEXTFILTERW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHD_TEXTFILTERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIMAGEINFO(pub *mut IMAGEINFO);
#[cfg(feature = "windef")]
impl LPIMAGEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPIMAGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIMAGELISTDRAWPARAMS(pub *mut IMAGELISTDRAWPARAMS);
#[cfg(feature = "windef")]
impl LPIMAGELISTDRAWPARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPIMAGELISTDRAWPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINITCOMMONCONTROLSEX(pub *mut INITCOMMONCONTROLSEX);
impl LPINITCOMMONCONTROLSEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINITCOMMONCONTROLSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVBKIMAGEA(pub *mut LVBKIMAGEA);
#[cfg(feature = "windef")]
impl LPLVBKIMAGEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPLVBKIMAGEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVBKIMAGEW(pub *mut LVBKIMAGEW);
#[cfg(feature = "windef")]
impl LPLVBKIMAGEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPLVBKIMAGEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVCOLUMNA(pub *mut LVCOLUMNA);
impl LPLVCOLUMNA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLVCOLUMNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVCOLUMNW(pub *mut LVCOLUMNW);
impl LPLVCOLUMNW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLVCOLUMNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVFOOTERINFO(pub *mut LVFOOTERINFO);
impl LPLVFOOTERINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLVFOOTERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVFOOTERITEM(pub *mut LVFOOTERITEM);
impl LPLVFOOTERITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLVFOOTERITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVHITTESTINFO(pub *mut LVHITTESTINFO);
#[cfg(feature = "windef")]
impl LPLVHITTESTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPLVHITTESTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVINSERTMARK(pub *mut LVINSERTMARK);
impl LPLVINSERTMARK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLVINSERTMARK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVITEMA(pub *mut LVITEMA);
#[cfg(feature = "minwindef")]
impl LPLVITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPLVITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLVITEMW(pub *mut LVITEMW);
#[cfg(feature = "minwindef")]
impl LPLVITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPLVITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMONTHDAYSTATE(pub *mut u32);
impl LPMONTHDAYSTATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMONTHDAYSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMBCDROPDOWN(pub *mut NMBCDROPDOWN);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMBCDROPDOWN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMBCDROPDOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMBCHOTITEM(pub *mut NMBCHOTITEM);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMBCHOTITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMBCHOTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCBEDRAGBEGINA(pub *mut NMCBEDRAGBEGINA);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMCBEDRAGBEGINA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMCBEDRAGBEGINA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCBEDRAGBEGINW(pub *mut NMCBEDRAGBEGINW);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMCBEDRAGBEGINW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMCBEDRAGBEGINW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCBEENDEDITA(pub *mut NMCBEENDEDITA);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMCBEENDEDITA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMCBEENDEDITA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCBEENDEDITW(pub *mut NMCBEENDEDITW);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMCBEENDEDITW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMCBEENDEDITW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCHAR(pub *mut NMCHAR);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPNMCLICK(pub LPNMMOUSE);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCUSTOMDRAW(pub *mut NMCUSTOMDRAW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMCUSTOMDRAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMCUSTOMDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCUSTOMSPLITRECTINFO(pub *mut NMCUSTOMSPLITRECTINFO);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMCUSTOMSPLITRECTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMCUSTOMSPLITRECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMCUSTOMTEXT(pub *mut NMCUSTOMTEXT);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMCUSTOMTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMCUSTOMTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMECHANGE(pub *mut NMDATETIMECHANGE);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDATETIMECHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMECHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMEFORMATA(pub *mut NMDATETIMEFORMATA);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDATETIMEFORMATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMEFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMEFORMATQUERYA(pub *mut NMDATETIMEFORMATQUERYA);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMDATETIMEFORMATQUERYA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMEFORMATQUERYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMEFORMATQUERYW(pub *mut NMDATETIMEFORMATQUERYW);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMDATETIMEFORMATQUERYW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMEFORMATQUERYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMEFORMATW(pub *mut NMDATETIMEFORMATW);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDATETIMEFORMATW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMEFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMESTRINGA(pub *mut NMDATETIMESTRINGA);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDATETIMESTRINGA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMESTRINGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMESTRINGW(pub *mut NMDATETIMESTRINGW);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDATETIMESTRINGW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMESTRINGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMEWMKEYDOWNA(pub *mut NMDATETIMEWMKEYDOWNA);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDATETIMEWMKEYDOWNA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMEWMKEYDOWNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDATETIMEWMKEYDOWNW(pub *mut NMDATETIMEWMKEYDOWNW);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDATETIMEWMKEYDOWNW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDATETIMEWMKEYDOWNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMDAYSTATE(pub *mut NMDAYSTATE);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMDAYSTATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMDAYSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMHDDISPINFOA(pub *mut NMHDDISPINFOA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMHDDISPINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMHDDISPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMHDDISPINFOW(pub *mut NMHDDISPINFOW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMHDDISPINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMHDDISPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMHDFILTERBTNCLICK(pub *mut NMHDFILTERBTNCLICK);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMHDFILTERBTNCLICK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMHDFILTERBTNCLICK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMHEADERA(pub *mut NMHEADERA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMHEADERA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMHEADERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMHEADERW(pub *mut NMHEADERW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMHEADERW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMHEADERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMIPADDRESS(pub *mut NMIPADDRESS);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMIPADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMIPADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMITEMACTIVATE(pub *mut NMITEMACTIVATE);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMITEMACTIVATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMITEMACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMKEY(pub *mut NMKEY);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMKEY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLISTVIEW(pub *mut NMLISTVIEW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLISTVIEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLISTVIEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVCACHEHINT(pub *mut NMLVCACHEHINT);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMLVCACHEHINT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMLVCACHEHINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVCUSTOMDRAW(pub *mut NMLVCUSTOMDRAW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLVCUSTOMDRAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVDISPINFOA(pub *mut NMLVDISPINFOA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLVDISPINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLVDISPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVDISPINFOW(pub *mut NMLVDISPINFOW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLVDISPINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLVDISPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVFINDITEMA(pub *mut NMLVFINDITEMA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLVFINDITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLVFINDITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVFINDITEMW(pub *mut NMLVFINDITEMW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLVFINDITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLVFINDITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVGETINFOTIPA(pub *mut NMLVGETINFOTIPA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLVGETINFOTIPA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLVGETINFOTIPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVGETINFOTIPW(pub *mut NMLVGETINFOTIPW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMLVGETINFOTIPW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMLVGETINFOTIPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVKEYDOWN(pub *mut NMLVKEYDOWN);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMLVKEYDOWN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMLVKEYDOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVODSTATECHANGE(pub *mut NMLVODSTATECHANGE);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMLVODSTATECHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMLVODSTATECHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMLVSCROLL(pub *mut NMLVSCROLL);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMLVSCROLL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMLVSCROLL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMMOUSE(pub *mut NMMOUSE);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMMOUSE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMMOUSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMOBJECTNOTIFY(pub *mut NMOBJECTNOTIFY);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMOBJECTNOTIFY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMOBJECTNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMPGCALCSIZE(pub *mut NMPGCALCSIZE);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMPGCALCSIZE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMPGCALCSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMPGHOTITEM(pub *mut NMPGHOTITEM);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMPGHOTITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMPGHOTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMPGSCROLL(pub *mut NMPGSCROLL);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMPGSCROLL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMPGSCROLL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMRBAUTOSIZE(pub *mut NMRBAUTOSIZE);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMRBAUTOSIZE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMRBAUTOSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMREBAR(pub *mut NMREBAR);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMREBAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMREBAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMREBARAUTOBREAK(pub *mut NMREBARAUTOBREAK);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMREBARAUTOBREAK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMREBARAUTOBREAK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMREBARCHEVRON(pub *mut NMREBARCHEVRON);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMREBARCHEVRON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMREBARCHEVRON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMREBARCHILDSIZE(pub *mut NMREBARCHILDSIZE);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMREBARCHILDSIZE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMREBARCHILDSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMREBARSPLITTER(pub *mut NMREBARSPLITTER);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMREBARSPLITTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMREBARSPLITTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMSELCHANGE(pub *mut NMSELCHANGE);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMSELCHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMSELCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMSELECT(pub *mut NMSELCHANGE);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl LPNMSELECT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
impl Default for LPNMSELECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBCUSTOMDRAW(pub *mut NMTBCUSTOMDRAW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTBCUSTOMDRAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTBCUSTOMDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBDISPINFOA(pub *mut NMTBDISPINFOA);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTBDISPINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTBDISPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBDISPINFOW(pub *mut NMTBDISPINFOW);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTBDISPINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTBDISPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBGETINFOTIPA(pub *mut NMTBGETINFOTIPA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTBGETINFOTIPA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTBGETINFOTIPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBGETINFOTIPW(pub *mut NMTBGETINFOTIPW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTBGETINFOTIPW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTBGETINFOTIPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBHOTITEM(pub *mut NMTBHOTITEM);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTBHOTITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTBHOTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBRESTORE(pub *mut NMTBRESTORE);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTBRESTORE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTBRESTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTBSAVE(pub *mut NMTBSAVE);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTBSAVE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTBSAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTOOLBARA(pub *mut NMTOOLBARA);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTOOLBARA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTOOLBARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTOOLBARW(pub *mut NMTOOLBARW);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTOOLBARW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTOOLBARW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTOOLTIPSCREATED(pub *mut NMTOOLTIPSCREATED);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTOOLTIPSCREATED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTOOLTIPSCREATED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTREEVIEWA(pub *mut NMTREEVIEWA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTREEVIEWA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTREEVIEWA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTREEVIEWW(pub *mut NMTREEVIEWW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTREEVIEWW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTREEVIEWW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTTCUSTOMDRAW(pub *mut NMTTCUSTOMDRAW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTTCUSTOMDRAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTTCUSTOMDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTTDISPINFOA(pub *mut NMTTDISPINFOA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTTDISPINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTTDISPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTTDISPINFOW(pub *mut NMTTDISPINFOW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTTDISPINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTTDISPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVCUSTOMDRAW(pub *mut NMTVCUSTOMDRAW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTVCUSTOMDRAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVDISPINFOA(pub *mut NMTVDISPINFOA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTVDISPINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTVDISPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVDISPINFOEXA(pub *mut NMTVDISPINFOEXA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTVDISPINFOEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTVDISPINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVDISPINFOEXW(pub *mut NMTVDISPINFOEXW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTVDISPINFOEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTVDISPINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVDISPINFOW(pub *mut NMTVDISPINFOW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTVDISPINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTVDISPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVGETINFOTIPA(pub *mut NMTVGETINFOTIPA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTVGETINFOTIPA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTVGETINFOTIPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVGETINFOTIPW(pub *mut NMTVGETINFOTIPW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPNMTVGETINFOTIPW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPNMTVGETINFOTIPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVKEYDOWN(pub *mut NMTVKEYDOWN);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTVKEYDOWN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTVKEYDOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMTVSTATEIMAGECHANGING(pub *mut NMTVSTATEIMAGECHANGING);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMTVSTATEIMAGECHANGING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMTVSTATEIMAGECHANGING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMUPDOWN(pub *mut NMUPDOWN);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMUPDOWN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMUPDOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMVIEWCHANGE(pub *mut NMVIEWCHANGE);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl LPNMVIEWCHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for LPNMVIEWCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPRBHITTESTINFO(pub *mut RBHITTESTINFO);
#[cfg(feature = "windef")]
impl LPRBHITTESTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPRBHITTESTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREBARBANDINFOA(pub *mut REBARBANDINFOA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPREBARBANDINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPREBARBANDINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREBARBANDINFOW(pub *mut REBARBANDINFOW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPREBARBANDINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPREBARBANDINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREBARINFO(pub *mut REBARINFO);
impl LPREBARINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREBARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LPSTR_TEXTCALLBACKA: windows_core::PCSTR = windows_core::PCSTR(-1 as _);
pub const LPSTR_TEXTCALLBACKW: windows_core::PCWSTR = windows_core::PCWSTR(-1 as _);
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBADDBITMAP(pub *mut TBADDBITMAP);
#[cfg(feature = "minwindef")]
impl LPTBADDBITMAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTBADDBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBBUTTON(pub *mut TBBUTTON);
impl LPTBBUTTON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBBUTTONINFOA(pub *mut TBBUTTONINFOA);
impl LPTBBUTTONINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTBBUTTONINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBBUTTONINFOW(pub *mut TBBUTTONINFOW);
impl LPTBBUTTONINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTBBUTTONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBINSERTMARK(pub *mut TBINSERTMARK);
impl LPTBINSERTMARK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTBINSERTMARK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBMETRICS(pub *mut TBMETRICS);
impl LPTBMETRICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTBMETRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBREPLACEBITMAP(pub *mut TBREPLACEBITMAP);
#[cfg(feature = "minwindef")]
impl LPTBREPLACEBITMAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTBREPLACEBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBSAVEPARAMSA(pub *mut TBSAVEPARAMSA);
#[cfg(feature = "minwindef")]
impl LPTBSAVEPARAMSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTBSAVEPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBSAVEPARAMW(pub *mut TBSAVEPARAMSW);
#[cfg(feature = "minwindef")]
impl LPTBSAVEPARAMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTBSAVEPARAMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTCHITTESTINFO(pub *mut TCHITTESTINFO);
#[cfg(feature = "windef")]
impl LPTCHITTESTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPTCHITTESTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTCITEMA(pub *mut TCITEMA);
#[cfg(feature = "minwindef")]
impl LPTCITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTCITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTCITEMHEADERA(pub *mut TCITEMHEADERA);
impl LPTCITEMHEADERA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTCITEMHEADERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTCITEMHEADERW(pub *mut TCITEMHEADERW);
impl LPTCITEMHEADERW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTCITEMHEADERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTCITEMW(pub *mut TCITEMW);
#[cfg(feature = "minwindef")]
impl LPTCITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTCITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTTHITTESTINFOA(pub *mut TTHITTESTINFOA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTTHITTESTINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTTHITTESTINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTTHITTESTINFOW(pub *mut TTHITTESTINFOW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTTHITTESTINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTTHITTESTINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTTTOOLINFOA(pub *mut TTTOOLINFOA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTTTOOLINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTTTOOLINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTTTOOLINFOW(pub *mut TTTOOLINFOW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTTTOOLINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTTTOOLINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVHITTESTINFO(pub *mut TVHITTESTINFO);
#[cfg(feature = "windef")]
impl LPTVHITTESTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPTVHITTESTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVINSERTSTRUCTA(pub *mut TVINSERTSTRUCTA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTVINSERTSTRUCTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTVINSERTSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVINSERTSTRUCTW(pub *mut TVINSERTSTRUCTW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTVINSERTSTRUCTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTVINSERTSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVITEMA(pub *mut TVITEMA);
#[cfg(feature = "minwindef")]
impl LPTVITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTVITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPTVITEMEX(pub LPTVITEMEXA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVITEMEXA(pub *mut TVITEMEXA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTVITEMEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTVITEMEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVITEMEXW(pub *mut TVITEMEXW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPTVITEMEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPTVITEMEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVITEMW(pub *mut TVITEMW);
#[cfg(feature = "minwindef")]
impl LPTVITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTVITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTVSORTCB(pub *mut TVSORTCB);
#[cfg(feature = "minwindef")]
impl LPTVSORTCB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPTVSORTCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUDACCEL(pub *mut UDACCEL);
impl LPUDACCEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPUDACCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LVA_ALIGNLEFT: u32 = 1;
pub const LVA_ALIGNTOP: u32 = 2;
pub const LVA_DEFAULT: u32 = 0;
pub const LVA_SNAPTOGRID: u32 = 5;
pub const LVBKIF_FLAG_ALPHABLEND: u32 = 536870912;
pub const LVBKIF_FLAG_TILEOFFSET: u32 = 256;
pub const LVBKIF_SOURCE_HBITMAP: u32 = 1;
pub const LVBKIF_SOURCE_MASK: u32 = 3;
pub const LVBKIF_SOURCE_NONE: u32 = 0;
pub const LVBKIF_SOURCE_URL: u32 = 2;
pub const LVBKIF_STYLE_MASK: u32 = 16;
pub const LVBKIF_STYLE_NORMAL: u32 = 0;
pub const LVBKIF_STYLE_TILE: u32 = 16;
pub const LVBKIF_TYPE_WATERMARK: u32 = 268435456;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVBKIMAGEA {
    pub ulFlags: u32,
    pub hbm: super::windef::HBITMAP,
    pub pszImage: windows_core::PSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVBKIMAGEW {
    pub ulFlags: u32,
    pub hbm: super::windef::HBITMAP,
    pub pszImage: windows_core::PWSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
pub const LVCDI_GROUP: u32 = 1;
pub const LVCDI_ITEM: u32 = 0;
pub const LVCDI_ITEMSLIST: u32 = 2;
pub const LVCDRF_NOGROUPFRAME: u32 = 131072;
pub const LVCDRF_NOSELECT: u32 = 65536;
pub const LVCFMT_BITMAP_ON_RIGHT: u32 = 4096;
pub const LVCFMT_CENTER: u32 = 2;
pub const LVCFMT_COL_HAS_IMAGES: u32 = 32768;
pub const LVCFMT_FILL: u32 = 2097152;
pub const LVCFMT_FIXED_RATIO: u32 = 524288;
pub const LVCFMT_FIXED_WIDTH: u32 = 256;
pub const LVCFMT_IMAGE: u32 = 2048;
pub const LVCFMT_JUSTIFYMASK: u32 = 3;
pub const LVCFMT_LEFT: u32 = 0;
pub const LVCFMT_LINE_BREAK: u32 = 1048576;
pub const LVCFMT_NO_DPI_SCALE: u32 = 262144;
pub const LVCFMT_NO_TITLE: u32 = 8388608;
pub const LVCFMT_RIGHT: u32 = 1;
pub const LVCFMT_SPLITBUTTON: u32 = 16777216;
pub const LVCFMT_TILE_PLACEMENTMASK: u32 = 3145728;
pub const LVCFMT_WRAP: u32 = 4194304;
pub const LVCF_DEFAULTWIDTH: u32 = 128;
pub const LVCF_FMT: u32 = 1;
pub const LVCF_IDEALWIDTH: u32 = 256;
pub const LVCF_IMAGE: u32 = 16;
pub const LVCF_MINWIDTH: u32 = 64;
pub const LVCF_ORDER: u32 = 32;
pub const LVCF_SUBITEM: u32 = 8;
pub const LVCF_TEXT: u32 = 4;
pub const LVCF_WIDTH: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const LVCOLUMNA_V1_SIZE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const LVCOLUMNW_V1_SIZE: u32 = 32;
#[cfg(target_arch = "x86")]
pub const LVCOLUMN_V1_SIZE: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVCOLUMN_V1_SIZE: u32 = 32;
pub const LVFF_ITEMCOUNT: u32 = 1;
pub const LVFIF_STATE: u32 = 2;
pub const LVFIF_TEXT: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFINDINFOA {
    pub flags: u32,
    pub psz: windows_core::PCSTR,
    pub lParam: super::minwindef::LPARAM,
    pub pt: super::windef::POINT,
    pub vkDirection: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFINDINFOW {
    pub flags: u32,
    pub psz: windows_core::PCWSTR,
    pub lParam: super::minwindef::LPARAM,
    pub pt: super::windef::POINT,
    pub vkDirection: u32,
}
pub const LVFIS_FOCUSED: u32 = 1;
pub const LVFI_NEARESTXY: u32 = 64;
pub const LVFI_PARAM: u32 = 1;
pub const LVFI_PARTIAL: u32 = 8;
pub const LVFI_STRING: u32 = 2;
pub const LVFI_SUBSTRING: u32 = 4;
pub const LVFI_WRAP: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFOOTERINFO {
    pub mask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub cItems: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFOOTERITEM {
    pub mask: u32,
    pub iItem: i32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub state: u32,
    pub stateMask: u32,
}
pub const LVGA_FOOTER_CENTER: u32 = 16;
pub const LVGA_FOOTER_LEFT: u32 = 8;
pub const LVGA_FOOTER_RIGHT: u32 = 32;
pub const LVGA_HEADER_CENTER: u32 = 2;
pub const LVGA_HEADER_LEFT: u32 = 1;
pub const LVGA_HEADER_RIGHT: u32 = 4;
pub const LVGF_ALIGN: u32 = 8;
pub const LVGF_DESCRIPTIONBOTTOM: u32 = 2048;
pub const LVGF_DESCRIPTIONTOP: u32 = 1024;
pub const LVGF_EXTENDEDIMAGE: u32 = 8192;
pub const LVGF_FOOTER: u32 = 2;
pub const LVGF_GROUPID: u32 = 16;
pub const LVGF_HEADER: u32 = 1;
pub const LVGF_ITEMS: u32 = 16384;
pub const LVGF_NONE: u32 = 0;
pub const LVGF_STATE: u32 = 4;
pub const LVGF_SUBSET: u32 = 32768;
pub const LVGF_SUBSETITEMS: u32 = 65536;
pub const LVGF_SUBTITLE: u32 = 256;
pub const LVGF_TASK: u32 = 512;
pub const LVGF_TITLEIMAGE: u32 = 4096;
pub const LVGGR_GROUP: u32 = 0;
pub const LVGGR_HEADER: u32 = 1;
pub const LVGGR_LABEL: u32 = 2;
pub const LVGGR_SUBSETLINK: u32 = 3;
pub const LVGIT_UNFOLDED: u32 = 1;
pub const LVGMF_BORDERCOLOR: u32 = 2;
pub const LVGMF_BORDERSIZE: u32 = 1;
pub const LVGMF_NONE: u32 = 0;
pub const LVGMF_TEXTCOLOR: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVGROUPMETRICS {
    pub cbSize: u32,
    pub mask: u32,
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub crLeft: super::windef::COLORREF,
    pub crTop: super::windef::COLORREF,
    pub crRight: super::windef::COLORREF,
    pub crBottom: super::windef::COLORREF,
    pub crHeader: super::windef::COLORREF,
    pub crFooter: super::windef::COLORREF,
}
#[cfg(target_arch = "x86")]
pub const LVGROUP_V5_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVGROUP_V5_SIZE: u32 = 52;
pub const LVGS_COLLAPSED: u32 = 1;
pub const LVGS_COLLAPSIBLE: u32 = 8;
pub const LVGS_FOCUSED: u32 = 16;
pub const LVGS_HIDDEN: u32 = 2;
pub const LVGS_NOHEADER: u32 = 4;
pub const LVGS_NORMAL: u32 = 0;
pub const LVGS_SELECTED: u32 = 32;
pub const LVGS_SUBSETED: u32 = 64;
pub const LVGS_SUBSETLINKFOCUSED: u32 = 128;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVHITTESTINFO {
    pub pt: super::windef::POINT,
    pub flags: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub iGroup: i32,
}
pub const LVHITTESTINFO_V1_SIZE: u32 = 16;
pub const LVHT_ABOVE: u32 = 8;
pub const LVHT_BELOW: u32 = 16;
pub const LVHT_EX_FOOTER: u32 = 134217728;
pub const LVHT_EX_GROUP: i32 = -218103808;
pub const LVHT_EX_GROUP_BACKGROUND: u32 = 2147483648;
pub const LVHT_EX_GROUP_COLLAPSE: u32 = 1073741824;
pub const LVHT_EX_GROUP_FOOTER: u32 = 536870912;
pub const LVHT_EX_GROUP_HEADER: u32 = 268435456;
pub const LVHT_EX_GROUP_STATEICON: u32 = 16777216;
pub const LVHT_EX_GROUP_SUBSETLINK: u32 = 33554432;
pub const LVHT_EX_ONCONTENTS: u32 = 67108864;
pub const LVHT_NOWHERE: u32 = 1;
pub const LVHT_ONITEM: u32 = 14;
pub const LVHT_ONITEMICON: u32 = 2;
pub const LVHT_ONITEMLABEL: u32 = 4;
pub const LVHT_ONITEMSTATEICON: u32 = 8;
pub const LVHT_TOLEFT: u32 = 64;
pub const LVHT_TORIGHT: u32 = 32;
pub const LVIF_COLFMT: u32 = 65536;
pub const LVIF_COLUMNS: u32 = 512;
pub const LVIF_DI_SETITEM: u32 = 4096;
pub const LVIF_GROUPID: u32 = 256;
pub const LVIF_IMAGE: u32 = 2;
pub const LVIF_INDENT: u32 = 16;
pub const LVIF_NORECOMPUTE: u32 = 2048;
pub const LVIF_PARAM: u32 = 4;
pub const LVIF_STATE: u32 = 8;
pub const LVIF_TEXT: u32 = 1;
pub const LVIM_AFTER: u32 = 1;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVINSERTMARK {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iItem: i32,
    pub dwReserved: u32,
}
pub const LVIR_BOUNDS: u32 = 0;
pub const LVIR_ICON: u32 = 1;
pub const LVIR_LABEL: u32 = 2;
pub const LVIR_SELECTBOUNDS: u32 = 3;
pub const LVIS_ACTIVATING: u32 = 32;
pub const LVIS_CUT: u32 = 4;
pub const LVIS_DROPHILITED: u32 = 8;
pub const LVIS_FOCUSED: u32 = 1;
pub const LVIS_GLOW: u32 = 16;
pub const LVIS_OVERLAYMASK: u32 = 3840;
pub const LVIS_SELECTED: u32 = 2;
pub const LVIS_STATEIMAGEMASK: u32 = 61440;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LVITEMA {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::minwindef::LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: super::minwindef::PUINT,
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
pub const LVITEMA_V1_SIZE: u32 = 48;
#[cfg(target_arch = "x86")]
pub const LVITEMA_V5_SIZE: u32 = 52;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEMA_V5_SIZE: u32 = 72;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVITEMINDEX {
    pub iItem: i32,
    pub iGroup: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LVITEMW {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::minwindef::LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: super::minwindef::PUINT,
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
pub const LVITEMW_V1_SIZE: u32 = 48;
#[cfg(target_arch = "x86")]
pub const LVITEMW_V5_SIZE: u32 = 52;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEMW_V5_SIZE: u32 = 72;
#[cfg(target_arch = "x86")]
pub const LVITEM_V1_SIZE: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEM_V1_SIZE: u32 = 48;
#[cfg(target_arch = "x86")]
pub const LVITEM_V5_SIZE: u32 = 52;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LVITEM_V5_SIZE: u32 = 72;
pub const LVKF_ALT: u32 = 1;
pub const LVKF_CONTROL: u32 = 2;
pub const LVKF_SHIFT: u32 = 4;
pub const LVM_APPROXIMATEVIEWRECT: u32 = 4160;
pub const LVM_ARRANGE: u32 = 4118;
pub const LVM_CANCELEDITLABEL: u32 = 4275;
pub const LVM_CREATEDRAGIMAGE: u32 = 4129;
pub const LVM_DELETEALLITEMS: u32 = 4105;
pub const LVM_DELETECOLUMN: u32 = 4124;
pub const LVM_DELETEITEM: u32 = 4104;
pub const LVM_EDITLABEL: u32 = 4119;
pub const LVM_EDITLABELA: u32 = 4119;
pub const LVM_EDITLABELW: u32 = 4214;
pub const LVM_ENABLEGROUPVIEW: u32 = 4253;
pub const LVM_ENSUREVISIBLE: u32 = 4115;
pub const LVM_FINDITEM: u32 = 4109;
pub const LVM_FINDITEMA: u32 = 4109;
pub const LVM_FINDITEMW: u32 = 4179;
pub const LVM_FIRST: u32 = 4096;
pub const LVM_GETBKCOLOR: u32 = 4096;
pub const LVM_GETBKIMAGE: u32 = 4165;
pub const LVM_GETBKIMAGEA: u32 = 4165;
pub const LVM_GETBKIMAGEW: u32 = 4235;
pub const LVM_GETCALLBACKMASK: u32 = 4106;
pub const LVM_GETCOLUMN: u32 = 4121;
pub const LVM_GETCOLUMNA: u32 = 4121;
pub const LVM_GETCOLUMNORDERARRAY: u32 = 4155;
pub const LVM_GETCOLUMNW: u32 = 4191;
pub const LVM_GETCOLUMNWIDTH: u32 = 4125;
pub const LVM_GETCOUNTPERPAGE: u32 = 4136;
pub const LVM_GETEDITCONTROL: u32 = 4120;
pub const LVM_GETEMPTYTEXT: u32 = 4300;
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: u32 = 4151;
pub const LVM_GETFOCUSEDGROUP: u32 = 4189;
pub const LVM_GETFOOTERINFO: u32 = 4302;
pub const LVM_GETFOOTERITEM: u32 = 4304;
pub const LVM_GETFOOTERITEMRECT: u32 = 4303;
pub const LVM_GETFOOTERRECT: u32 = 4301;
pub const LVM_GETGROUPCOUNT: u32 = 4248;
pub const LVM_GETGROUPINFO: u32 = 4245;
pub const LVM_GETGROUPINFOBYINDEX: u32 = 4249;
pub const LVM_GETGROUPMETRICS: u32 = 4252;
pub const LVM_GETGROUPRECT: u32 = 4194;
pub const LVM_GETGROUPSTATE: u32 = 4188;
pub const LVM_GETHEADER: u32 = 4127;
pub const LVM_GETHOTCURSOR: u32 = 4159;
pub const LVM_GETHOTITEM: u32 = 4157;
pub const LVM_GETHOVERTIME: u32 = 4168;
pub const LVM_GETIMAGELIST: u32 = 4098;
pub const LVM_GETINSERTMARK: u32 = 4263;
pub const LVM_GETINSERTMARKCOLOR: u32 = 4267;
pub const LVM_GETINSERTMARKRECT: u32 = 4265;
pub const LVM_GETISEARCHSTRING: u32 = 4148;
pub const LVM_GETISEARCHSTRINGA: u32 = 4148;
pub const LVM_GETISEARCHSTRINGW: u32 = 4213;
pub const LVM_GETITEM: u32 = 4101;
pub const LVM_GETITEMA: u32 = 4101;
pub const LVM_GETITEMCOUNT: u32 = 4100;
pub const LVM_GETITEMINDEXRECT: u32 = 4305;
pub const LVM_GETITEMPOSITION: u32 = 4112;
pub const LVM_GETITEMRECT: u32 = 4110;
pub const LVM_GETITEMSPACING: u32 = 4147;
pub const LVM_GETITEMSTATE: u32 = 4140;
pub const LVM_GETITEMTEXT: u32 = 4141;
pub const LVM_GETITEMTEXTA: u32 = 4141;
pub const LVM_GETITEMTEXTW: u32 = 4211;
pub const LVM_GETITEMW: u32 = 4171;
pub const LVM_GETNEXTITEM: u32 = 4108;
pub const LVM_GETNEXTITEMINDEX: u32 = 4307;
pub const LVM_GETNUMBEROFWORKAREAS: u32 = 4169;
pub const LVM_GETORIGIN: u32 = 4137;
pub const LVM_GETOUTLINECOLOR: u32 = 4272;
pub const LVM_GETSELECTEDCOLUMN: u32 = 4270;
pub const LVM_GETSELECTEDCOUNT: u32 = 4146;
pub const LVM_GETSELECTIONMARK: u32 = 4162;
pub const LVM_GETSTRINGWIDTH: u32 = 4113;
pub const LVM_GETSTRINGWIDTHA: u32 = 4113;
pub const LVM_GETSTRINGWIDTHW: u32 = 4183;
pub const LVM_GETSUBITEMRECT: u32 = 4152;
pub const LVM_GETTEXTBKCOLOR: u32 = 4133;
pub const LVM_GETTEXTCOLOR: u32 = 4131;
pub const LVM_GETTILEINFO: u32 = 4261;
pub const LVM_GETTILEVIEWINFO: u32 = 4259;
pub const LVM_GETTOOLTIPS: u32 = 4174;
pub const LVM_GETTOPINDEX: u32 = 4135;
pub const LVM_GETUNICODEFORMAT: u32 = 8198;
pub const LVM_GETVIEW: u32 = 4239;
pub const LVM_GETVIEWRECT: u32 = 4130;
pub const LVM_GETWORKAREAS: u32 = 4166;
pub const LVM_HASGROUP: u32 = 4257;
pub const LVM_HITTEST: u32 = 4114;
pub const LVM_INSERTCOLUMN: u32 = 4123;
pub const LVM_INSERTCOLUMNA: u32 = 4123;
pub const LVM_INSERTCOLUMNW: u32 = 4193;
pub const LVM_INSERTGROUP: u32 = 4241;
pub const LVM_INSERTGROUPSORTED: u32 = 4255;
pub const LVM_INSERTITEM: u32 = 4103;
pub const LVM_INSERTITEMA: u32 = 4103;
pub const LVM_INSERTITEMW: u32 = 4173;
pub const LVM_INSERTMARKHITTEST: u32 = 4264;
pub const LVM_ISGROUPVIEWENABLED: u32 = 4271;
pub const LVM_ISITEMVISIBLE: u32 = 4278;
pub const LVM_MAPIDTOINDEX: u32 = 4277;
pub const LVM_MAPINDEXTOID: u32 = 4276;
pub const LVM_MOVEGROUP: u32 = 4247;
pub const LVM_MOVEITEMTOGROUP: u32 = 4250;
pub const LVM_REDRAWITEMS: u32 = 4117;
pub const LVM_REMOVEALLGROUPS: u32 = 4256;
pub const LVM_REMOVEGROUP: u32 = 4246;
pub const LVM_SCROLL: u32 = 4116;
pub const LVM_SETBKCOLOR: u32 = 4097;
pub const LVM_SETBKIMAGE: u32 = 4164;
pub const LVM_SETBKIMAGEA: u32 = 4164;
pub const LVM_SETBKIMAGEW: u32 = 4234;
pub const LVM_SETCALLBACKMASK: u32 = 4107;
pub const LVM_SETCOLUMN: u32 = 4122;
pub const LVM_SETCOLUMNA: u32 = 4122;
pub const LVM_SETCOLUMNORDERARRAY: u32 = 4154;
pub const LVM_SETCOLUMNW: u32 = 4192;
pub const LVM_SETCOLUMNWIDTH: u32 = 4126;
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = 4150;
pub const LVM_SETGROUPINFO: u32 = 4243;
pub const LVM_SETGROUPMETRICS: u32 = 4251;
pub const LVM_SETHOTCURSOR: u32 = 4158;
pub const LVM_SETHOTITEM: u32 = 4156;
pub const LVM_SETHOVERTIME: u32 = 4167;
pub const LVM_SETICONSPACING: u32 = 4149;
pub const LVM_SETIMAGELIST: u32 = 4099;
pub const LVM_SETINFOTIP: u32 = 4269;
pub const LVM_SETINSERTMARK: u32 = 4262;
pub const LVM_SETINSERTMARKCOLOR: u32 = 4266;
pub const LVM_SETITEM: u32 = 4102;
pub const LVM_SETITEMA: u32 = 4102;
pub const LVM_SETITEMCOUNT: u32 = 4143;
pub const LVM_SETITEMINDEXSTATE: u32 = 4306;
pub const LVM_SETITEMPOSITION: u32 = 4111;
pub const LVM_SETITEMPOSITION32: u32 = 4145;
pub const LVM_SETITEMSTATE: u32 = 4139;
pub const LVM_SETITEMTEXT: u32 = 4142;
pub const LVM_SETITEMTEXTA: u32 = 4142;
pub const LVM_SETITEMTEXTW: u32 = 4212;
pub const LVM_SETITEMW: u32 = 4172;
pub const LVM_SETOUTLINECOLOR: u32 = 4273;
pub const LVM_SETSELECTEDCOLUMN: u32 = 4236;
pub const LVM_SETSELECTIONMARK: u32 = 4163;
pub const LVM_SETTEXTBKCOLOR: u32 = 4134;
pub const LVM_SETTEXTCOLOR: u32 = 4132;
pub const LVM_SETTILEINFO: u32 = 4260;
pub const LVM_SETTILEVIEWINFO: u32 = 4258;
pub const LVM_SETTOOLTIPS: u32 = 4170;
pub const LVM_SETUNICODEFORMAT: u32 = 8197;
pub const LVM_SETVIEW: u32 = 4238;
pub const LVM_SETWORKAREAS: u32 = 4161;
pub const LVM_SORTGROUPS: u32 = 4254;
pub const LVM_SORTITEMS: u32 = 4144;
pub const LVM_SORTITEMSEX: u32 = 4177;
pub const LVM_SUBITEMHITTEST: u32 = 4153;
pub const LVM_UPDATE: u32 = 4138;
pub const LVNI_ABOVE: u32 = 256;
pub const LVNI_ALL: u32 = 0;
pub const LVNI_BELOW: u32 = 512;
pub const LVNI_CUT: u32 = 4;
pub const LVNI_DIRECTIONMASK: u32 = 3840;
pub const LVNI_DROPHILITED: u32 = 8;
pub const LVNI_FOCUSED: u32 = 1;
pub const LVNI_PREVIOUS: u32 = 32;
pub const LVNI_SAMEGROUPONLY: u32 = 128;
pub const LVNI_SELECTED: u32 = 2;
pub const LVNI_STATEMASK: u32 = 15;
pub const LVNI_TOLEFT: u32 = 1024;
pub const LVNI_TORIGHT: u32 = 2048;
pub const LVNI_VISIBLEONLY: u32 = 64;
pub const LVNI_VISIBLEORDER: u32 = 16;
pub const LVNSCH_DEFAULT: i32 = -1;
pub const LVNSCH_ERROR: i32 = -2;
pub const LVNSCH_IGNORE: i32 = -3;
pub const LVN_BEGINDRAG: i32 = -109;
pub const LVN_BEGINLABELEDIT: i32 = -105;
pub const LVN_BEGINLABELEDITA: i32 = -105;
pub const LVN_BEGINLABELEDITW: i32 = -175;
pub const LVN_BEGINRDRAG: i32 = -111;
pub const LVN_BEGINSCROLL: i32 = -180;
pub const LVN_COLUMNCLICK: i32 = -108;
pub const LVN_COLUMNDROPDOWN: i32 = -164;
pub const LVN_COLUMNOVERFLOWCLICK: i32 = -166;
pub const LVN_DELETEALLITEMS: i32 = -104;
pub const LVN_DELETEITEM: i32 = -103;
pub const LVN_ENDLABELEDIT: i32 = -106;
pub const LVN_ENDLABELEDITA: i32 = -106;
pub const LVN_ENDLABELEDITW: i32 = -176;
pub const LVN_ENDSCROLL: i32 = -181;
pub const LVN_FIRST: i32 = -100;
pub const LVN_GETDISPINFO: i32 = -150;
pub const LVN_GETDISPINFOA: i32 = -150;
pub const LVN_GETDISPINFOW: i32 = -177;
pub const LVN_GETEMPTYMARKUP: i32 = -187;
pub const LVN_GETINFOTIP: i32 = -157;
pub const LVN_GETINFOTIPA: i32 = -157;
pub const LVN_GETINFOTIPW: i32 = -158;
pub const LVN_HOTTRACK: i32 = -121;
pub const LVN_INCREMENTALSEARCH: i32 = -162;
pub const LVN_INCREMENTALSEARCHA: i32 = -162;
pub const LVN_INCREMENTALSEARCHW: i32 = -163;
pub const LVN_INSERTITEM: i32 = -102;
pub const LVN_ITEMACTIVATE: i32 = -114;
pub const LVN_ITEMCHANGED: i32 = -101;
pub const LVN_ITEMCHANGING: i32 = -100;
pub const LVN_KEYDOWN: i32 = -155;
pub const LVN_LAST: i32 = -199;
pub const LVN_LINKCLICK: i32 = -184;
pub const LVN_MARQUEEBEGIN: i32 = -156;
pub const LVN_ODCACHEHINT: i32 = -113;
pub const LVN_ODFINDITEM: i32 = -152;
pub const LVN_ODFINDITEMA: i32 = -152;
pub const LVN_ODFINDITEMW: i32 = -179;
pub const LVN_ODSTATECHANGED: i32 = -115;
pub const LVN_SETDISPINFO: i32 = -151;
pub const LVN_SETDISPINFOA: i32 = -151;
pub const LVN_SETDISPINFOW: i32 = -178;
pub const LVSCW_AUTOSIZE: i32 = -1;
pub const LVSCW_AUTOSIZE_USEHEADER: i32 = -2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVSETINFOTIP {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pszText: windows_core::PWSTR,
    pub iItem: i32,
    pub iSubItem: i32,
}
pub const LVSICF_NOINVALIDATEALL: u32 = 1;
pub const LVSICF_NOSCROLL: u32 = 2;
pub const LVSIL_GROUPHEADER: u32 = 3;
pub const LVSIL_NORMAL: u32 = 0;
pub const LVSIL_SMALL: u32 = 1;
pub const LVSIL_STATE: u32 = 2;
pub const LVS_ALIGNLEFT: u32 = 2048;
pub const LVS_ALIGNMASK: u32 = 3072;
pub const LVS_ALIGNTOP: u32 = 0;
pub const LVS_AUTOARRANGE: u32 = 256;
pub const LVS_EDITLABELS: u32 = 512;
pub const LVS_EX_AUTOAUTOARRANGE: u32 = 16777216;
pub const LVS_EX_AUTOCHECKSELECT: u32 = 134217728;
pub const LVS_EX_AUTOSIZECOLUMNS: u32 = 268435456;
pub const LVS_EX_BORDERSELECT: u32 = 32768;
pub const LVS_EX_CHECKBOXES: u32 = 4;
pub const LVS_EX_COLUMNOVERFLOW: u32 = 2147483648;
pub const LVS_EX_COLUMNSNAPPOINTS: u32 = 1073741824;
pub const LVS_EX_DOUBLEBUFFER: u32 = 65536;
pub const LVS_EX_FLATSB: u32 = 256;
pub const LVS_EX_FULLROWSELECT: u32 = 32;
pub const LVS_EX_GRIDLINES: u32 = 1;
pub const LVS_EX_HEADERDRAGDROP: u32 = 16;
pub const LVS_EX_HEADERINALLVIEWS: u32 = 33554432;
pub const LVS_EX_HIDELABELS: u32 = 131072;
pub const LVS_EX_INFOTIP: u32 = 1024;
pub const LVS_EX_JUSTIFYCOLUMNS: u32 = 2097152;
pub const LVS_EX_LABELTIP: u32 = 16384;
pub const LVS_EX_MULTIWORKAREAS: u32 = 8192;
pub const LVS_EX_ONECLICKACTIVATE: u32 = 64;
pub const LVS_EX_REGIONAL: u32 = 512;
pub const LVS_EX_SIMPLESELECT: u32 = 1048576;
pub const LVS_EX_SINGLEROW: u32 = 262144;
pub const LVS_EX_SNAPTOGRID: u32 = 524288;
pub const LVS_EX_SUBITEMIMAGES: u32 = 2;
pub const LVS_EX_TRACKSELECT: u32 = 8;
pub const LVS_EX_TRANSPARENTBKGND: u32 = 4194304;
pub const LVS_EX_TRANSPARENTSHADOWTEXT: u32 = 8388608;
pub const LVS_EX_TWOCLICKACTIVATE: u32 = 128;
pub const LVS_EX_UNDERLINECOLD: u32 = 4096;
pub const LVS_EX_UNDERLINEHOT: u32 = 2048;
pub const LVS_ICON: u32 = 0;
pub const LVS_LIST: u32 = 3;
pub const LVS_NOCOLUMNHEADER: u32 = 16384;
pub const LVS_NOLABELWRAP: u32 = 128;
pub const LVS_NOSCROLL: u32 = 8192;
pub const LVS_NOSORTHEADER: u32 = 32768;
pub const LVS_OWNERDATA: u32 = 4096;
pub const LVS_OWNERDRAWFIXED: u32 = 1024;
pub const LVS_REPORT: u32 = 1;
pub const LVS_SHAREIMAGELISTS: u32 = 64;
pub const LVS_SHOWSELALWAYS: u32 = 8;
pub const LVS_SINGLESEL: u32 = 4;
pub const LVS_SMALLICON: u32 = 2;
pub const LVS_SORTASCENDING: u32 = 16;
pub const LVS_SORTDESCENDING: u32 = 32;
pub const LVS_TYPEMASK: u32 = 3;
pub const LVS_TYPESTYLEMASK: u32 = 64512;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LVTILEINFO {
    pub cbSize: u32,
    pub iItem: i32,
    pub cColumns: u32,
    pub puColumns: super::minwindef::PUINT,
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
pub const LVTILEINFO_V5_SIZE: u32 = 24;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVTILEVIEWINFO {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwFlags: u32,
    pub sizeTile: super::windef::SIZE,
    pub cLines: i32,
    pub rcLabelMargin: super::windef::RECT,
}
pub const LVTVIF_AUTOSIZE: u32 = 0;
pub const LVTVIF_EXTENDED: u32 = 4;
pub const LVTVIF_FIXEDHEIGHT: u32 = 2;
pub const LVTVIF_FIXEDSIZE: u32 = 3;
pub const LVTVIF_FIXEDWIDTH: u32 = 1;
pub const LVTVIM_COLUMNS: u32 = 2;
pub const LVTVIM_LABELMARGIN: u32 = 4;
pub const LVTVIM_TILESIZE: u32 = 1;
pub const LV_MAX_WORKAREAS: u32 = 16;
pub const LV_VIEW_DETAILS: u32 = 1;
pub const LV_VIEW_ICON: u32 = 0;
pub const LV_VIEW_LIST: u32 = 3;
pub const LV_VIEW_MAX: u32 = 4;
pub const LV_VIEW_SMALLICON: u32 = 2;
pub const LV_VIEW_TILE: u32 = 4;
pub const LWS_IGNORERETURN: u32 = 2;
pub const LWS_NOPREFIX: u32 = 4;
pub const LWS_RIGHT: u32 = 32;
pub const LWS_TRANSPARENT: u32 = 1;
pub const LWS_USECUSTOMTEXT: u32 = 16;
pub const LWS_USEVISUALSTYLE: u32 = 8;
pub const MAX_LINKID_TEXT: u32 = 48;
pub const MCGIF_DATE: u32 = 1;
pub const MCGIF_NAME: u32 = 4;
pub const MCGIF_RECT: u32 = 2;
pub const MCGIP_CALENDAR: u32 = 4;
pub const MCGIP_CALENDARBODY: u32 = 6;
pub const MCGIP_CALENDARCELL: u32 = 8;
pub const MCGIP_CALENDARCONTROL: u32 = 0;
pub const MCGIP_CALENDARHEADER: u32 = 5;
pub const MCGIP_CALENDARROW: u32 = 7;
pub const MCGIP_FOOTER: u32 = 3;
pub const MCGIP_NEXT: u32 = 1;
pub const MCGIP_PREV: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCGRIDINFO {
    pub cbSize: u32,
    pub dwPart: u32,
    pub dwFlags: u32,
    pub iCalendar: i32,
    pub iRow: i32,
    pub iCol: i32,
    pub bSelected: windows_core::BOOL,
    pub stStart: super::minwinbase::SYSTEMTIME,
    pub stEnd: super::minwinbase::SYSTEMTIME,
    pub rc: super::windef::RECT,
    pub pszName: windows_core::PWSTR,
    pub cchName: usize,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCHITTESTINFO {
    pub cbSize: u32,
    pub pt: super::windef::POINT,
    pub uHit: u32,
    pub st: super::minwinbase::SYSTEMTIME,
    pub rc: super::windef::RECT,
    pub iOffset: i32,
    pub iRow: i32,
    pub iCol: i32,
}
pub const MCHITTESTINFO_V1_SIZE: u32 = 32;
pub const MCHT_CALENDAR: u32 = 131072;
pub const MCHT_CALENDARBK: u32 = 131072;
pub const MCHT_CALENDARCONTROL: u32 = 1048576;
pub const MCHT_CALENDARDATE: u32 = 131073;
pub const MCHT_CALENDARDATEMAX: u32 = 131077;
pub const MCHT_CALENDARDATEMIN: u32 = 131076;
pub const MCHT_CALENDARDATENEXT: u32 = 16908289;
pub const MCHT_CALENDARDATEPREV: u32 = 33685505;
pub const MCHT_CALENDARDAY: u32 = 131074;
pub const MCHT_CALENDARWEEKNUM: u32 = 131075;
pub const MCHT_NEXT: u32 = 16777216;
pub const MCHT_NOWHERE: u32 = 0;
pub const MCHT_PREV: u32 = 33554432;
pub const MCHT_TITLE: u32 = 65536;
pub const MCHT_TITLEBK: u32 = 65536;
pub const MCHT_TITLEBTNNEXT: u32 = 16842755;
pub const MCHT_TITLEBTNPREV: u32 = 33619971;
pub const MCHT_TITLEMONTH: u32 = 65537;
pub const MCHT_TITLEYEAR: u32 = 65538;
pub const MCHT_TODAYLINK: u32 = 196608;
pub const MCMV_CENTURY: u32 = 3;
pub const MCMV_DECADE: u32 = 2;
pub const MCMV_MAX: u32 = 3;
pub const MCMV_MONTH: u32 = 0;
pub const MCMV_YEAR: u32 = 1;
pub const MCM_FIRST: u32 = 4096;
pub const MCM_GETCALENDARBORDER: u32 = 4127;
pub const MCM_GETCALENDARCOUNT: u32 = 4119;
pub const MCM_GETCALENDARGRIDINFO: u32 = 4120;
pub const MCM_GETCALID: u32 = 4123;
pub const MCM_GETCOLOR: u32 = 4107;
pub const MCM_GETCURRENTVIEW: u32 = 4118;
pub const MCM_GETCURSEL: u32 = 4097;
pub const MCM_GETFIRSTDAYOFWEEK: u32 = 4112;
pub const MCM_GETMAXSELCOUNT: u32 = 4099;
pub const MCM_GETMAXTODAYWIDTH: u32 = 4117;
pub const MCM_GETMINREQRECT: u32 = 4105;
pub const MCM_GETMONTHDELTA: u32 = 4115;
pub const MCM_GETMONTHRANGE: u32 = 4103;
pub const MCM_GETRANGE: u32 = 4113;
pub const MCM_GETSELRANGE: u32 = 4101;
pub const MCM_GETTODAY: u32 = 4109;
pub const MCM_GETUNICODEFORMAT: u32 = 8198;
pub const MCM_HITTEST: u32 = 4110;
pub const MCM_SETCALENDARBORDER: u32 = 4126;
pub const MCM_SETCALID: u32 = 4124;
pub const MCM_SETCOLOR: u32 = 4106;
pub const MCM_SETCURRENTVIEW: u32 = 4128;
pub const MCM_SETCURSEL: u32 = 4098;
pub const MCM_SETDAYSTATE: u32 = 4104;
pub const MCM_SETFIRSTDAYOFWEEK: u32 = 4111;
pub const MCM_SETMAXSELCOUNT: u32 = 4100;
pub const MCM_SETMONTHDELTA: u32 = 4116;
pub const MCM_SETRANGE: u32 = 4114;
pub const MCM_SETSELRANGE: u32 = 4102;
pub const MCM_SETTODAY: u32 = 4108;
pub const MCM_SETUNICODEFORMAT: u32 = 8197;
pub const MCM_SIZERECTTOMIN: u32 = 4125;
pub const MCN_FIRST: i32 = -746;
pub const MCN_GETDAYSTATE: i32 = -747;
pub const MCN_LAST: i32 = -752;
pub const MCN_SELCHANGE: i32 = -749;
pub const MCN_SELECT: i32 = -746;
pub const MCN_VIEWCHANGE: i32 = -750;
pub const MCSC_BACKGROUND: u32 = 0;
pub const MCSC_MONTHBK: u32 = 4;
pub const MCSC_TEXT: u32 = 1;
pub const MCSC_TITLEBK: u32 = 2;
pub const MCSC_TITLETEXT: u32 = 3;
pub const MCSC_TRAILINGTEXT: u32 = 5;
pub const MCS_DAYSTATE: u32 = 1;
pub const MCS_MULTISELECT: u32 = 2;
pub const MCS_NOSELCHANGEONNAV: u32 = 256;
pub const MCS_NOTODAY: u32 = 16;
pub const MCS_NOTODAYCIRCLE: u32 = 8;
pub const MCS_NOTRAILINGDATES: u32 = 64;
pub const MCS_SHORTDAYSOFWEEK: u32 = 128;
pub const MCS_WEEKNUMBERS: u32 = 4;
pub const MINSYSCOMMAND: u32 = 61440;
pub const MONTHCAL_CLASSA: windows_core::PCSTR = windows_core::s!("SysMonthCal32");
pub const MONTHCAL_CLASSW: windows_core::PCWSTR = windows_core::w!("SysMonthCal32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MONTHDAYSTATE(pub u32);
pub const MSGF_COMMCTRL_BEGINDRAG: u32 = 16896;
pub const MSGF_COMMCTRL_DRAGSELECT: u32 = 16898;
pub const MSGF_COMMCTRL_SIZEHEADER: u32 = 16897;
pub const MSGF_COMMCTRL_TOOLBARCUST: u32 = 16899;
pub const NFS_ALL: u32 = 16;
pub const NFS_BUTTON: u32 = 8;
pub const NFS_EDIT: u32 = 1;
pub const NFS_LISTCOMBO: u32 = 4;
pub const NFS_STATIC: u32 = 2;
pub const NFS_USEFONTASSOC: u32 = 32;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMBCDROPDOWN {
    pub hdr: super::winuser::NMHDR,
    pub rcButton: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMBCHOTITEM {
    pub hdr: super::winuser::NMHDR,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEDRAGBEGINA {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEDRAGBEGINW {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEENDEDITA {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEENDEDITW {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCHAR {
    pub hdr: super::winuser::NMHDR,
    pub ch: u32,
    pub dwItemPrev: u32,
    pub dwItemNext: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type NMCLICK = NMMOUSE;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCOMBOBOXEXA {
    pub hdr: super::winuser::NMHDR,
    pub ceItem: COMBOBOXEXITEMA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCOMBOBOXEXW {
    pub hdr: super::winuser::NMHDR,
    pub ceItem: COMBOBOXEXITEMW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCUSTOMDRAW {
    pub hdr: super::winuser::NMHDR,
    pub dwDrawStage: u32,
    pub hdc: super::windef::HDC,
    pub rc: super::windef::RECT,
    pub dwItemSpec: usize,
    pub uItemState: u32,
    pub lItemlParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCUSTOMSPLITRECTINFO {
    pub hdr: super::winuser::NMHDR,
    pub rcClient: super::windef::RECT,
    pub rcButton: super::windef::RECT,
    pub rcSplit: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCUSTOMTEXT {
    pub hdr: super::winuser::NMHDR,
    pub hDC: super::windef::HDC,
    pub lpString: windows_core::PCWSTR,
    pub nCount: i32,
    pub lpRect: super::windef::LPRECT,
    pub uFormat: u32,
    pub fLink: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMECHANGE {
    pub nmhdr: super::winuser::NMHDR,
    pub dwFlags: u32,
    pub st: super::minwinbase::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMDATETIMEFORMATA {
    pub nmhdr: super::winuser::NMHDR,
    pub pszFormat: windows_core::PCSTR,
    pub st: super::minwinbase::SYSTEMTIME,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEFORMATQUERYA {
    pub nmhdr: super::winuser::NMHDR,
    pub pszFormat: windows_core::PCSTR,
    pub szMax: super::windef::SIZE,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEFORMATQUERYW {
    pub nmhdr: super::winuser::NMHDR,
    pub pszFormat: windows_core::PCWSTR,
    pub szMax: super::windef::SIZE,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMDATETIMEFORMATW {
    pub nmhdr: super::winuser::NMHDR,
    pub pszFormat: windows_core::PCWSTR,
    pub st: super::minwinbase::SYSTEMTIME,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMESTRINGA {
    pub nmhdr: super::winuser::NMHDR,
    pub pszUserString: windows_core::PCSTR,
    pub st: super::minwinbase::SYSTEMTIME,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMESTRINGW {
    pub nmhdr: super::winuser::NMHDR,
    pub pszUserString: windows_core::PCWSTR,
    pub st: super::minwinbase::SYSTEMTIME,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEWMKEYDOWNA {
    pub nmhdr: super::winuser::NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: windows_core::PCSTR,
    pub st: super::minwinbase::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEWMKEYDOWNW {
    pub nmhdr: super::winuser::NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: windows_core::PCWSTR,
    pub st: super::minwinbase::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDAYSTATE {
    pub nmhdr: super::winuser::NMHDR,
    pub stStart: super::minwinbase::SYSTEMTIME,
    pub cDayState: i32,
    pub prgDayState: LPMONTHDAYSTATE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMHDDISPINFOA {
    pub hdr: super::winuser::NMHDR,
    pub iItem: i32,
    pub mask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMHDDISPINFOW {
    pub hdr: super::winuser::NMHDR,
    pub iItem: i32,
    pub mask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMHDFILTERBTNCLICK {
    pub hdr: super::winuser::NMHDR,
    pub iItem: i32,
    pub rc: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMHEADERA {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMHEADERW {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMIPADDRESS {
    pub hdr: super::winuser::NMHDR,
    pub iField: i32,
    pub iValue: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMITEMACTIVATE {
    pub hdr: super::winuser::NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::windef::POINT,
    pub lParam: super::minwindef::LPARAM,
    pub uKeyFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMKEY {
    pub hdr: super::winuser::NMHDR,
    pub nVKey: u32,
    pub uFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLINK {
    pub hdr: super::winuser::NMHDR,
    pub item: LITEM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLISTVIEW {
    pub hdr: super::winuser::NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::windef::POINT,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVCACHEHINT {
    pub hdr: super::winuser::NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::windef::COLORREF,
    pub clrTextBk: super::windef::COLORREF,
    pub iSubItem: i32,
    pub dwItemType: u32,
    pub clrFace: super::windef::COLORREF,
    pub iIconEffect: i32,
    pub iIconPhase: i32,
    pub iPartId: i32,
    pub iStateId: i32,
    pub rcText: super::windef::RECT,
    pub uAlign: u32,
}
#[cfg(target_arch = "x86")]
pub const NMLVCUSTOMDRAW_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMLVCUSTOMDRAW_V3_SIZE: u32 = 88;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVDISPINFOA {
    pub hdr: super::winuser::NMHDR,
    pub item: LVITEMA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVDISPINFOW {
    pub hdr: super::winuser::NMHDR,
    pub item: LVITEMW,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMLVEMPTYMARKUP {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVFINDITEMA {
    pub hdr: super::winuser::NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVFINDITEMW {
    pub hdr: super::winuser::NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVGETINFOTIPA {
    pub hdr: super::winuser::NMHDR,
    pub dwFlags: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVGETINFOTIPW {
    pub hdr: super::winuser::NMHDR,
    pub dwFlags: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct NMLVKEYDOWN {
    pub hdr: super::winuser::NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVLINK {
    pub hdr: super::winuser::NMHDR,
    pub link: LITEM,
    pub iItem: i32,
    pub iSubItem: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVODSTATECHANGE {
    pub hdr: super::winuser::NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
    pub uNewState: u32,
    pub uOldState: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVSCROLL {
    pub hdr: super::winuser::NMHDR,
    pub dx: i32,
    pub dy: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMMOUSE {
    pub hdr: super::winuser::NMHDR,
    pub dwItemSpec: usize,
    pub dwItemData: usize,
    pub pt: super::windef::POINT,
    pub dwHitInfo: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMOBJECTNOTIFY {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMPGCALCSIZE {
    pub hdr: super::winuser::NMHDR,
    pub dwFlag: u32,
    pub iWidth: i32,
    pub iHeight: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMPGHOTITEM {
    pub hdr: super::winuser::NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct NMPGSCROLL {
    pub hdr: super::winuser::NMHDR,
    pub fwKeys: u16,
    pub rcParent: super::windef::RECT,
    pub iDir: i32,
    pub iXpos: i32,
    pub iYpos: i32,
    pub iScroll: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMRBAUTOSIZE {
    pub hdr: super::winuser::NMHDR,
    pub fChanged: windows_core::BOOL,
    pub rcTarget: super::windef::RECT,
    pub rcActual: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBAR {
    pub hdr: super::winuser::NMHDR,
    pub dwMask: u32,
    pub uBand: u32,
    pub fStyle: u32,
    pub wID: u32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARAUTOBREAK {
    pub hdr: super::winuser::NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::minwindef::LPARAM,
    pub uMsg: u32,
    pub fStyleCurrent: u32,
    pub fAutoBreak: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARCHEVRON {
    pub hdr: super::winuser::NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::minwindef::LPARAM,
    pub rc: super::windef::RECT,
    pub lParamNM: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARCHILDSIZE {
    pub hdr: super::winuser::NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub rcChild: super::windef::RECT,
    pub rcBand: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARSPLITTER {
    pub hdr: super::winuser::NMHDR,
    pub rcSizing: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMSEARCHWEB {
    pub hdr: super::winuser::NMHDR,
    pub entrypoint: EC_SEARCHWEB_ENTRYPOINT,
    pub hasQueryText: windows_core::BOOL,
    pub invokeSucceeded: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMSELCHANGE {
    pub nmhdr: super::winuser::NMHDR,
    pub stSelStart: super::minwinbase::SYSTEMTIME,
    pub stSelEnd: super::minwinbase::SYSTEMTIME,
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winuser"))]
pub type NMSELECT = NMSELCHANGE;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: super::windef::HBRUSH,
    pub hbrLines: super::windef::HBRUSH,
    pub hpenLines: super::windef::HPEN,
    pub clrText: super::windef::COLORREF,
    pub clrMark: super::windef::COLORREF,
    pub clrTextHighlight: super::windef::COLORREF,
    pub clrBtnFace: super::windef::COLORREF,
    pub clrBtnHighlight: super::windef::COLORREF,
    pub clrHighlightHotTrack: super::windef::COLORREF,
    pub rcText: super::windef::RECT,
    pub nStringBkMode: i32,
    pub nHLStringBkMode: i32,
    pub iListGap: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBDISPINFOA {
    pub hdr: super::winuser::NMHDR,
    pub dwMask: u32,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: windows_core::PSTR,
    pub cchText: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBDISPINFOW {
    pub hdr: super::winuser::NMHDR,
    pub dwMask: u32,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: windows_core::PWSTR,
    pub cchText: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBGETINFOTIPA {
    pub hdr: super::winuser::NMHDR,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBGETINFOTIPW {
    pub hdr: super::winuser::NMHDR,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBHOTITEM {
    pub hdr: super::winuser::NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTBRESTORE {
    pub hdr: super::winuser::NMHDR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTBSAVE {
    pub hdr: super::winuser::NMHDR,
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
    pub hdr: super::winuser::NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTOOLBARA {
    pub hdr: super::winuser::NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: windows_core::PSTR,
    pub rcButton: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTOOLBARW {
    pub hdr: super::winuser::NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: windows_core::PWSTR,
    pub rcButton: super::windef::RECT,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTOOLTIPSCREATED {
    pub hdr: super::winuser::NMHDR,
    pub hwndToolTips: super::windef::HWND,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTRBTHUMBPOSCHANGING {
    pub hdr: super::winuser::NMHDR,
    pub dwPos: u32,
    pub nReason: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTREEVIEWA {
    pub hdr: super::winuser::NMHDR,
    pub action: u32,
    pub itemOld: TVITEMA,
    pub itemNew: TVITEMA,
    pub ptDrag: super::windef::POINT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTREEVIEWW {
    pub hdr: super::winuser::NMHDR,
    pub action: u32,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: super::windef::POINT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTTCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTTDISPINFOA {
    pub hdr: super::winuser::NMHDR,
    pub lpszText: windows_core::PSTR,
    pub szText: [i8; 80],
    pub hinst: super::minwindef::HINSTANCE,
    pub uFlags: u32,
    pub lParam: super::minwindef::LPARAM,
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
pub const NMTTDISPINFOA_V1_SIZE: u32 = 124;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTTDISPINFOW {
    pub hdr: super::winuser::NMHDR,
    pub lpszText: windows_core::PWSTR,
    pub szText: [u16; 80],
    pub hinst: super::minwindef::HINSTANCE,
    pub uFlags: u32,
    pub lParam: super::minwindef::LPARAM,
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
pub const NMTTDISPINFOW_V1_SIZE: u32 = 204;
#[cfg(target_arch = "x86")]
pub const NMTTDISPINFO_V1_SIZE: u32 = 104;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMTTDISPINFO_V1_SIZE: u32 = 124;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTVASYNCDRAW {
    pub hdr: super::winuser::NMHDR,
    pub pimldp: *mut IMAGELISTDRAWPARAMS,
    pub hr: windows_core::HRESULT,
    pub hItem: HTREEITEM,
    pub lParam: super::minwindef::LPARAM,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::windef::COLORREF,
    pub clrTextBk: super::windef::COLORREF,
    pub iLevel: i32,
}
#[cfg(target_arch = "x86")]
pub const NMTVCUSTOMDRAW_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NMTVCUSTOMDRAW_V3_SIZE: u32 = 88;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOA {
    pub hdr: super::winuser::NMHDR,
    pub item: TVITEMA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOEXA {
    pub hdr: super::winuser::NMHDR,
    pub item: TVITEMEXA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOEXW {
    pub hdr: super::winuser::NMHDR,
    pub item: TVITEMEXW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOW {
    pub hdr: super::winuser::NMHDR,
    pub item: TVITEMW,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVGETINFOTIPA {
    pub hdr: super::winuser::NMHDR,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVGETINFOTIPW {
    pub hdr: super::winuser::NMHDR,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVITEMCHANGE {
    pub hdr: super::winuser::NMHDR,
    pub uChanged: u32,
    pub hItem: HTREEITEM,
    pub uStateNew: u32,
    pub uStateOld: u32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct NMTVKEYDOWN {
    pub hdr: super::winuser::NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVSTATEIMAGECHANGING {
    pub hdr: super::winuser::NMHDR,
    pub hti: HTREEITEM,
    pub iOldStateImageIndex: i32,
    pub iNewStateImageIndex: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMUPDOWN {
    pub hdr: super::winuser::NMHDR,
    pub iPos: i32,
    pub iDelta: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMVIEWCHANGE {
    pub nmhdr: super::winuser::NMHDR,
    pub dwOldView: u32,
    pub dwNewView: u32,
}
pub const NM_CHAR: i32 = -18;
pub const NM_CLICK: i32 = -2;
pub const NM_CUSTOMDRAW: i32 = -12;
pub const NM_CUSTOMTEXT: i32 = -24;
pub const NM_DBLCLK: i32 = -3;
pub const NM_FIRST: u32 = 0;
pub const NM_FONTCHANGED: i32 = -23;
pub const NM_GETCUSTOMSPLITRECT: i32 = -1247;
pub const NM_HOVER: i32 = -13;
pub const NM_KEYDOWN: i32 = -15;
pub const NM_KILLFOCUS: i32 = -8;
pub const NM_LAST: i32 = -99;
pub const NM_LDOWN: i32 = -20;
pub const NM_NCHITTEST: i32 = -14;
pub const NM_OUTOFMEMORY: i32 = -1;
pub const NM_RCLICK: i32 = -5;
pub const NM_RDBLCLK: i32 = -6;
pub const NM_RDOWN: i32 = -21;
pub const NM_RELEASEDCAPTURE: i32 = -16;
pub const NM_RETURN: i32 = -4;
pub const NM_SETCURSOR: i32 = -17;
pub const NM_SETFOCUS: i32 = -7;
pub const NM_THEMECHANGED: i32 = -22;
pub const NM_TOOLTIPSCREATED: i32 = -19;
pub const NM_TVSTATEIMAGECHANGING: i32 = -24;
pub const ODT_HEADER: u32 = 100;
pub const ODT_LISTVIEW: u32 = 102;
pub const ODT_TAB: u32 = 101;
pub const PBM_DELTAPOS: u32 = 1027;
pub const PBM_GETBARCOLOR: u32 = 1039;
pub const PBM_GETBKCOLOR: u32 = 1038;
pub const PBM_GETPOS: u32 = 1032;
pub const PBM_GETRANGE: u32 = 1031;
pub const PBM_GETSTATE: u32 = 1041;
pub const PBM_GETSTEP: u32 = 1037;
pub const PBM_SETBARCOLOR: u32 = 1033;
pub const PBM_SETBKCOLOR: u32 = 8193;
pub const PBM_SETMARQUEE: u32 = 1034;
pub const PBM_SETPOS: u32 = 1026;
pub const PBM_SETRANGE: u32 = 1025;
pub const PBM_SETRANGE32: u32 = 1030;
pub const PBM_SETSTATE: u32 = 1040;
pub const PBM_SETSTEP: u32 = 1028;
pub const PBM_STEPIT: u32 = 1029;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PBRANGE {
    pub iLow: i32,
    pub iHigh: i32,
}
pub const PBST_ERROR: u32 = 2;
pub const PBST_NORMAL: u32 = 1;
pub const PBST_PAUSED: u32 = 3;
pub const PBS_MARQUEE: u32 = 8;
pub const PBS_SMOOTH: u32 = 1;
pub const PBS_SMOOTHREVERSE: u32 = 16;
pub const PBS_VERTICAL: u32 = 4;
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBUTTON_IMAGELIST(pub *mut BUTTON_IMAGELIST);
#[cfg(feature = "windef")]
impl PBUTTON_IMAGELIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PBUTTON_IMAGELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBUTTON_SPLITINFO(pub *mut BUTTON_SPLITINFO);
#[cfg(feature = "windef")]
impl PBUTTON_SPLITINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PBUTTON_SPLITINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCCOMBOEXITEMA(pub *const COMBOBOXEXITEMA);
#[cfg(feature = "minwindef")]
impl PCCOMBOEXITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PCCOMBOEXITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCCOMBOEXITEMW(pub *const COMBOBOXEXITEMW);
#[cfg(feature = "minwindef")]
impl PCCOMBOEXITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PCCOMBOEXITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOMBOBOXEXITEMA(pub *mut COMBOBOXEXITEMA);
#[cfg(feature = "minwindef")]
impl PCOMBOBOXEXITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PCOMBOBOXEXITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOMBOBOXEXITEMW(pub *mut COMBOBOXEXITEMW);
#[cfg(feature = "minwindef")]
impl PCOMBOBOXEXITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PCOMBOBOXEXITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEDITBALLOONTIP(pub *mut EDITBALLOONTIP);
impl PEDITBALLOONTIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEDITBALLOONTIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub type PFNLVCOMPARE = Option<unsafe extern "system" fn(param0: super::minwindef::LPARAM, param1: super::minwindef::LPARAM, param2: super::minwindef::LPARAM) -> i32>;
pub type PFNLVGROUPCOMPARE = Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: *mut core::ffi::c_void) -> i32>;
#[cfg(feature = "minwindef")]
pub type PFNTVCOMPARE = Option<unsafe extern "system" fn(lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM, lparamsort: super::minwindef::LPARAM) -> i32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PFTASKDIALOGCALLBACK = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, lprefdata: isize) -> windows_core::HRESULT>;
pub const PGB_BOTTOMORRIGHT: u32 = 1;
pub const PGB_TOPORLEFT: u32 = 0;
pub const PGF_CALCHEIGHT: u32 = 2;
pub const PGF_CALCWIDTH: u32 = 1;
pub const PGF_DEPRESSED: u32 = 4;
pub const PGF_GRAYED: u32 = 2;
pub const PGF_HOT: u32 = 8;
pub const PGF_INVISIBLE: u32 = 0;
pub const PGF_NORMAL: u32 = 1;
pub const PGF_SCROLLDOWN: u32 = 2;
pub const PGF_SCROLLLEFT: u32 = 4;
pub const PGF_SCROLLRIGHT: u32 = 8;
pub const PGF_SCROLLUP: u32 = 1;
pub const PGK_CONTROL: u32 = 2;
pub const PGK_MENU: u32 = 4;
pub const PGK_SHIFT: u32 = 1;
pub const PGM_FIRST: u32 = 5120;
pub const PGM_FORWARDMOUSE: u32 = 5123;
pub const PGM_GETBKCOLOR: u32 = 5125;
pub const PGM_GETBORDER: u32 = 5127;
pub const PGM_GETBUTTONSIZE: u32 = 5131;
pub const PGM_GETBUTTONSTATE: u32 = 5132;
pub const PGM_GETDROPTARGET: u32 = 8196;
pub const PGM_GETPOS: u32 = 5129;
pub const PGM_RECALCSIZE: u32 = 5122;
pub const PGM_SETBKCOLOR: u32 = 5124;
pub const PGM_SETBORDER: u32 = 5126;
pub const PGM_SETBUTTONSIZE: u32 = 5130;
pub const PGM_SETCHILD: u32 = 5121;
pub const PGM_SETPOS: u32 = 5128;
pub const PGM_SETSCROLLINFO: u32 = 5133;
pub const PGN_CALCSIZE: i32 = -902;
pub const PGN_FIRST: i32 = -900;
pub const PGN_HOTITEMCHANGE: i32 = -903;
pub const PGN_LAST: i32 = -950;
pub const PGN_SCROLL: i32 = -901;
pub const PGS_AUTOSCROLL: u32 = 2;
pub const PGS_DRAGNDROP: u32 = 4;
pub const PGS_HORZ: u32 = 1;
pub const PGS_VERT: u32 = 0;
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLHITTESTINFO(pub *mut LHITTESTINFO);
#[cfg(feature = "windef")]
impl PLHITTESTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PLHITTESTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLITEM(pub *mut LITEM);
impl PLITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLVGROUP(pub *mut LVGROUP);
impl PLVGROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLVGROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLVGROUPMETRICS(pub *mut LVGROUPMETRICS);
#[cfg(feature = "windef")]
impl PLVGROUPMETRICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PLVGROUPMETRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLVINSERTGROUPSORTED(pub *mut LVINSERTGROUPSORTED);
impl PLVINSERTGROUPSORTED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLVINSERTGROUPSORTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLVITEMINDEX(pub *mut LVITEMINDEX);
impl PLVITEMINDEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLVITEMINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLVSETINFOTIP(pub *mut LVSETINFOTIP);
impl PLVSETINFOTIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLVSETINFOTIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLVTILEINFO(pub *mut LVTILEINFO);
#[cfg(feature = "minwindef")]
impl PLVTILEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PLVTILEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLVTILEVIEWINFO(pub *mut LVTILEVIEWINFO);
#[cfg(feature = "windef")]
impl PLVTILEVIEWINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PLVTILEVIEWINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMCGRIDINFO(pub *mut MCGRIDINFO);
#[cfg(all(feature = "minwinbase", feature = "windef"))]
impl PMCGRIDINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef"))]
impl Default for PMCGRIDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMCHITTESTINFO(pub *mut MCHITTESTINFO);
#[cfg(all(feature = "minwinbase", feature = "windef"))]
impl PMCHITTESTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwinbase", feature = "windef"))]
impl Default for PMCHITTESTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMCBEDRAGBEGINA(pub *mut NMCBEDRAGBEGINA);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl PNMCBEDRAGBEGINA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for PNMCBEDRAGBEGINA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMCBEDRAGBEGINW(pub *mut NMCBEDRAGBEGINW);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl PNMCBEDRAGBEGINW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for PNMCBEDRAGBEGINW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMCBEENDEDITA(pub *mut NMCBEENDEDITA);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl PNMCBEENDEDITA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for PNMCBEENDEDITA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMCBEENDEDITW(pub *mut NMCBEENDEDITW);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl PNMCBEENDEDITW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for PNMCBEENDEDITW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMCOMBOBOXEXA(pub *mut NMCOMBOBOXEXA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl PNMCOMBOBOXEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for PNMCOMBOBOXEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMCOMBOBOXEXW(pub *mut NMCOMBOBOXEXW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl PNMCOMBOBOXEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for PNMCOMBOBOXEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMLINK(pub *mut NMLINK);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl PNMLINK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for PNMLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMLVLINK(pub *mut NMLVLINK);
#[cfg(all(feature = "windef", feature = "winuser"))]
impl PNMLVLINK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for PNMLVLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPBRANGE(pub *mut PBRANGE);
impl PPBRANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPBRANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROGRESS_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_progress32");
pub const PROGRESS_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_progress32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTBBUTTON(pub *mut TBBUTTON);
impl PTBBUTTON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOOLINFOA(pub *mut TTTOOLINFOA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl PTOOLINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PTOOLINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOOLINFOW(pub *mut TTTOOLINFOW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl PTOOLINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PTOOLINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTTGETTITLE(pub *mut TTGETTITLE);
impl PTTGETTITLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTTGETTITLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RBAB_ADDBAND: u32 = 2;
pub const RBAB_AUTOSIZE: u32 = 1;
pub const RBBIM_BACKGROUND: u32 = 128;
pub const RBBIM_CHEVRONLOCATION: u32 = 4096;
pub const RBBIM_CHEVRONSTATE: u32 = 8192;
pub const RBBIM_CHILD: u32 = 16;
pub const RBBIM_CHILDSIZE: u32 = 32;
pub const RBBIM_COLORS: u32 = 2;
pub const RBBIM_HEADERSIZE: u32 = 2048;
pub const RBBIM_ID: u32 = 256;
pub const RBBIM_IDEALSIZE: u32 = 512;
pub const RBBIM_IMAGE: u32 = 8;
pub const RBBIM_LPARAM: u32 = 1024;
pub const RBBIM_SIZE: u32 = 64;
pub const RBBIM_STYLE: u32 = 1;
pub const RBBIM_TEXT: u32 = 4;
pub const RBBS_BREAK: u32 = 1;
pub const RBBS_CHILDEDGE: u32 = 4;
pub const RBBS_FIXEDBMP: u32 = 32;
pub const RBBS_FIXEDSIZE: u32 = 2;
pub const RBBS_GRIPPERALWAYS: u32 = 128;
pub const RBBS_HIDDEN: u32 = 8;
pub const RBBS_HIDETITLE: u32 = 1024;
pub const RBBS_NOGRIPPER: u32 = 256;
pub const RBBS_NOVERT: u32 = 16;
pub const RBBS_TOPALIGN: u32 = 2048;
pub const RBBS_USECHEVRON: u32 = 512;
pub const RBBS_VARIABLEHEIGHT: u32 = 64;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RBHITTESTINFO {
    pub pt: super::windef::POINT,
    pub flags: u32,
    pub iBand: i32,
}
pub const RBHT_CAPTION: u32 = 2;
pub const RBHT_CHEVRON: u32 = 8;
pub const RBHT_CLIENT: u32 = 3;
pub const RBHT_GRABBER: u32 = 4;
pub const RBHT_NOWHERE: u32 = 1;
pub const RBHT_SPLITTER: u32 = 16;
pub const RBIM_IMAGELIST: u32 = 1;
pub const RBNM_ID: u32 = 1;
pub const RBNM_LPARAM: u32 = 4;
pub const RBNM_STYLE: u32 = 2;
pub const RBN_AUTOBREAK: i32 = -853;
pub const RBN_AUTOSIZE: i32 = -834;
pub const RBN_BEGINDRAG: i32 = -835;
pub const RBN_CHEVRONPUSHED: i32 = -841;
pub const RBN_CHILDSIZE: i32 = -839;
pub const RBN_DELETEDBAND: i32 = -838;
pub const RBN_DELETINGBAND: i32 = -837;
pub const RBN_ENDDRAG: i32 = -836;
pub const RBN_FIRST: i32 = -831;
pub const RBN_GETOBJECT: i32 = -832;
pub const RBN_HEIGHTCHANGE: i32 = -831;
pub const RBN_LAST: i32 = -859;
pub const RBN_LAYOUTCHANGED: i32 = -833;
pub const RBN_MINMAX: i32 = -852;
pub const RBN_SPLITTERDRAG: i32 = -842;
pub const RBSTR_CHANGERECT: u32 = 1;
pub const RBS_AUTOSIZE: u32 = 8192;
pub const RBS_BANDBORDERS: u32 = 1024;
pub const RBS_DBLCLKTOGGLE: u32 = 32768;
pub const RBS_FIXEDORDER: u32 = 2048;
pub const RBS_REGISTERDROP: u32 = 4096;
pub const RBS_TOOLTIPS: u32 = 256;
pub const RBS_VARHEIGHT: u32 = 512;
pub const RBS_VERTICALGRIPPER: u32 = 16384;
pub const RB_BEGINDRAG: u32 = 1048;
pub const RB_DELETEBAND: u32 = 1026;
pub const RB_DRAGMOVE: u32 = 1050;
pub const RB_ENDDRAG: u32 = 1049;
pub const RB_GETBANDBORDERS: u32 = 1058;
pub const RB_GETBANDCOUNT: u32 = 1036;
pub const RB_GETBANDINFO: u32 = 1053;
pub const RB_GETBANDINFOA: u32 = 1053;
pub const RB_GETBANDINFOW: u32 = 1052;
pub const RB_GETBANDMARGINS: u32 = 1064;
pub const RB_GETBARHEIGHT: u32 = 1051;
pub const RB_GETBARINFO: u32 = 1027;
pub const RB_GETBKCOLOR: u32 = 1044;
pub const RB_GETCOLORSCHEME: u32 = 8195;
pub const RB_GETDROPTARGET: u32 = 8196;
pub const RB_GETEXTENDEDSTYLE: u32 = 1066;
pub const RB_GETPALETTE: u32 = 1062;
pub const RB_GETRECT: u32 = 1033;
pub const RB_GETROWCOUNT: u32 = 1037;
pub const RB_GETROWHEIGHT: u32 = 1038;
pub const RB_GETTEXTCOLOR: u32 = 1046;
pub const RB_GETTOOLTIPS: u32 = 1041;
pub const RB_GETUNICODEFORMAT: u32 = 8198;
pub const RB_HITTEST: u32 = 1032;
pub const RB_IDTOINDEX: u32 = 1040;
pub const RB_INSERTBAND: u32 = 1025;
pub const RB_INSERTBANDA: u32 = 1025;
pub const RB_INSERTBANDW: u32 = 1034;
pub const RB_MAXIMIZEBAND: u32 = 1055;
pub const RB_MINIMIZEBAND: u32 = 1054;
pub const RB_MOVEBAND: u32 = 1063;
pub const RB_PUSHCHEVRON: u32 = 1067;
pub const RB_SETBANDINFO: u32 = 1030;
pub const RB_SETBANDINFOA: u32 = 1030;
pub const RB_SETBANDINFOW: u32 = 1035;
pub const RB_SETBANDWIDTH: u32 = 1068;
pub const RB_SETBARINFO: u32 = 1028;
pub const RB_SETBKCOLOR: u32 = 1043;
pub const RB_SETCOLORSCHEME: u32 = 8194;
pub const RB_SETEXTENDEDSTYLE: u32 = 1065;
pub const RB_SETPALETTE: u32 = 1061;
pub const RB_SETPARENT: u32 = 1031;
pub const RB_SETTEXTCOLOR: u32 = 1045;
pub const RB_SETTOOLTIPS: u32 = 1042;
pub const RB_SETUNICODEFORMAT: u32 = 8197;
pub const RB_SETWINDOWTHEME: u32 = 8203;
pub const RB_SHOWBAND: u32 = 1059;
pub const RB_SIZETORECT: u32 = 1047;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REBARBANDINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::windef::COLORREF,
    pub clrBack: super::windef::COLORREF,
    pub lpText: windows_core::PSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::windef::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::windef::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::minwindef::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::windef::RECT,
    pub uChevronState: u32,
}
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOA_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOA_V3_SIZE: u32 = 76;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOA_V6_SIZE: u32 = 80;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOA_V6_SIZE: u32 = 108;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REBARBANDINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::windef::COLORREF,
    pub clrBack: super::windef::COLORREF,
    pub lpText: windows_core::PWSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::windef::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::windef::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::minwindef::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::windef::RECT,
    pub uChevronState: u32,
}
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOW_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOW_V3_SIZE: u32 = 76;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFOW_V6_SIZE: u32 = 80;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFOW_V6_SIZE: u32 = 108;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFO_V3_SIZE: u32 = 56;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFO_V3_SIZE: u32 = 76;
#[cfg(target_arch = "x86")]
pub const REBARBANDINFO_V6_SIZE: u32 = 80;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const REBARBANDINFO_V6_SIZE: u32 = 108;
pub const REBARCLASSNAMEA: windows_core::PCSTR = windows_core::s!("ReBarWindow32");
pub const REBARCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("ReBarWindow32");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REBARINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub himl: HIMAGELIST,
}
pub const SBARS_SIZEGRIP: u32 = 256;
pub const SBARS_TOOLTIPS: u32 = 2048;
pub const SBN_FIRST: i32 = -880;
pub const SBN_LAST: i32 = -899;
pub const SBN_SIMPLEMODECHANGE: i32 = -880;
pub const SBT_NOBORDERS: u32 = 256;
pub const SBT_NOTABPARSING: u32 = 2048;
pub const SBT_OWNERDRAW: u32 = 4096;
pub const SBT_POPOUT: u32 = 512;
pub const SBT_RTLREADING: u32 = 1024;
pub const SBT_TOOLTIPS: u32 = 2048;
pub const SB_GETBORDERS: u32 = 1031;
pub const SB_GETICON: u32 = 1044;
pub const SB_GETPARTS: u32 = 1030;
pub const SB_GETRECT: u32 = 1034;
pub const SB_GETTEXT: u32 = 1026;
pub const SB_GETTEXTA: u32 = 1026;
pub const SB_GETTEXTLENGTH: u32 = 1027;
pub const SB_GETTEXTLENGTHA: u32 = 1027;
pub const SB_GETTEXTLENGTHW: u32 = 1036;
pub const SB_GETTEXTW: u32 = 1037;
pub const SB_GETTIPTEXT: u32 = 1042;
pub const SB_GETTIPTEXTA: u32 = 1042;
pub const SB_GETTIPTEXTW: u32 = 1043;
pub const SB_GETUNICODEFORMAT: u32 = 8198;
pub const SB_ISSIMPLE: u32 = 1038;
pub const SB_SETBKCOLOR: u32 = 8193;
pub const SB_SETICON: u32 = 1039;
pub const SB_SETMINHEIGHT: u32 = 1032;
pub const SB_SETPARTS: u32 = 1028;
pub const SB_SETTEXT: u32 = 1025;
pub const SB_SETTEXTA: u32 = 1025;
pub const SB_SETTEXTW: u32 = 1035;
pub const SB_SETTIPTEXT: u32 = 1040;
pub const SB_SETTIPTEXTA: u32 = 1040;
pub const SB_SETTIPTEXTW: u32 = 1041;
pub const SB_SETUNICODEFORMAT: u32 = 8197;
pub const SB_SIMPLE: u32 = 1033;
pub const SB_SIMPLEID: u32 = 255;
pub const STATUSCLASSNAMEA: windows_core::PCSTR = windows_core::s!("msctls_statusbar32");
pub const STATUSCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("msctls_statusbar32");
pub const STD_COPY: u32 = 1;
pub const STD_CUT: u32 = 0;
pub const STD_DELETE: u32 = 5;
pub const STD_FILENEW: u32 = 6;
pub const STD_FILEOPEN: u32 = 7;
pub const STD_FILESAVE: u32 = 8;
pub const STD_FIND: u32 = 12;
pub const STD_HELP: u32 = 11;
pub const STD_PASTE: u32 = 2;
pub const STD_PRINT: u32 = 14;
pub const STD_PRINTPRE: u32 = 9;
pub const STD_PROPERTIES: u32 = 10;
pub const STD_REDOW: u32 = 4;
pub const STD_REPLACE: u32 = 13;
pub const STD_UNDO: u32 = 3;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type SUBCLASSPROC = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, uidsubclass: usize, dwrefdata: usize) -> super::minwindef::LRESULT>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct TASKDIALOGCONFIG {
    pub cbSize: u32,
    pub hwndParent: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub hMainIcon: super::windef::HICON,
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
    pub hFooterIcon: super::windef::HICON,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TASKDIALOG_COMMON_BUTTON_FLAGS(pub i32);
pub type TASKDIALOG_ELEMENTS = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TASKDIALOG_FLAGS(pub i32);
pub type TASKDIALOG_ICON_ELEMENTS = i32;
pub type TASKDIALOG_MESSAGES = i32;
pub type TASKDIALOG_NOTIFICATIONS = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBADDBITMAP {
    pub hInst: super::minwindef::HINSTANCE,
    pub nID: usize,
}
pub const TBBF_LARGE: u32 = 1;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const TBCDRF_BLENDICON: u32 = 2097152;
pub const TBCDRF_HILITEHOTTRACK: u32 = 131072;
pub const TBCDRF_NOBACKGROUND: u32 = 4194304;
pub const TBCDRF_NOEDGES: u32 = 65536;
pub const TBCDRF_NOETCHEDEFFECT: u32 = 1048576;
pub const TBCDRF_NOMARK: u32 = 524288;
pub const TBCDRF_NOOFFSET: u32 = 262144;
pub const TBCDRF_USECDCOLORS: u32 = 8388608;
pub const TBCD_CHANNEL: u32 = 3;
pub const TBCD_THUMB: u32 = 2;
pub const TBCD_TICS: u32 = 1;
pub const TBDDRET_DEFAULT: u32 = 0;
pub const TBDDRET_NODEFAULT: u32 = 1;
pub const TBDDRET_TREATPRESSED: u32 = 2;
pub const TBIF_BYINDEX: u32 = 2147483648;
pub const TBIF_COMMAND: u32 = 32;
pub const TBIF_IMAGE: u32 = 1;
pub const TBIF_LPARAM: u32 = 16;
pub const TBIF_SIZE: u32 = 64;
pub const TBIF_STATE: u32 = 4;
pub const TBIF_STYLE: u32 = 8;
pub const TBIF_TEXT: u32 = 2;
pub const TBIMHT_AFTER: u32 = 1;
pub const TBIMHT_BACKGROUND: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBINSERTMARK {
    pub iButton: i32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const TBMF_BARPAD: u32 = 2;
pub const TBMF_BUTTONSPACING: u32 = 4;
pub const TBMF_PAD: u32 = 1;
pub const TBM_CLEARSEL: u32 = 1043;
pub const TBM_CLEARTICS: u32 = 1033;
pub const TBM_GETBUDDY: u32 = 1057;
pub const TBM_GETCHANNELRECT: u32 = 1050;
pub const TBM_GETLINESIZE: u32 = 1048;
pub const TBM_GETNUMTICS: u32 = 1040;
pub const TBM_GETPAGESIZE: u32 = 1046;
pub const TBM_GETPOS: u32 = 1024;
pub const TBM_GETPTICS: u32 = 1038;
pub const TBM_GETRANGEMAX: u32 = 1026;
pub const TBM_GETRANGEMIN: u32 = 1025;
pub const TBM_GETSELEND: u32 = 1042;
pub const TBM_GETSELSTART: u32 = 1041;
pub const TBM_GETTHUMBLENGTH: u32 = 1052;
pub const TBM_GETTHUMBRECT: u32 = 1049;
pub const TBM_GETTIC: u32 = 1027;
pub const TBM_GETTICPOS: u32 = 1039;
pub const TBM_GETTOOLTIPS: u32 = 1054;
pub const TBM_GETUNICODEFORMAT: u32 = 8198;
pub const TBM_SETBUDDY: u32 = 1056;
pub const TBM_SETLINESIZE: u32 = 1047;
pub const TBM_SETPAGESIZE: u32 = 1045;
pub const TBM_SETPOS: u32 = 1029;
pub const TBM_SETPOSNOTIFY: u32 = 1058;
pub const TBM_SETRANGE: u32 = 1030;
pub const TBM_SETRANGEMAX: u32 = 1032;
pub const TBM_SETRANGEMIN: u32 = 1031;
pub const TBM_SETSEL: u32 = 1034;
pub const TBM_SETSELEND: u32 = 1036;
pub const TBM_SETSELSTART: u32 = 1035;
pub const TBM_SETTHUMBLENGTH: u32 = 1051;
pub const TBM_SETTIC: u32 = 1028;
pub const TBM_SETTICFREQ: u32 = 1044;
pub const TBM_SETTIPSIDE: u32 = 1055;
pub const TBM_SETTOOLTIPS: u32 = 1053;
pub const TBM_SETUNICODEFORMAT: u32 = 8197;
pub const TBNF_DI_SETITEM: u32 = 268435456;
pub const TBNF_IMAGE: u32 = 1;
pub const TBNF_TEXT: u32 = 2;
pub const TBNRF_ENDCUSTOMIZE: u32 = 2;
pub const TBNRF_HIDEHELP: u32 = 1;
pub const TBN_BEGINADJUST: i32 = -703;
pub const TBN_BEGINDRAG: i32 = -701;
pub const TBN_CUSTHELP: i32 = -709;
pub const TBN_DELETINGBUTTON: i32 = -715;
pub const TBN_DRAGOUT: i32 = -714;
pub const TBN_DRAGOVER: i32 = -727;
pub const TBN_DROPDOWN: i32 = -710;
pub const TBN_DUPACCELERATOR: i32 = -725;
pub const TBN_ENDADJUST: i32 = -704;
pub const TBN_ENDDRAG: i32 = -702;
pub const TBN_FIRST: i32 = -700;
pub const TBN_GETBUTTONINFO: i32 = -700;
pub const TBN_GETBUTTONINFOA: i32 = -700;
pub const TBN_GETBUTTONINFOW: i32 = -720;
pub const TBN_GETDISPINFO: i32 = -716;
pub const TBN_GETDISPINFOA: i32 = -716;
pub const TBN_GETDISPINFOW: i32 = -717;
pub const TBN_GETINFOTIP: i32 = -718;
pub const TBN_GETINFOTIPA: i32 = -718;
pub const TBN_GETINFOTIPW: i32 = -719;
pub const TBN_GETOBJECT: i32 = -712;
pub const TBN_HOTITEMCHANGE: i32 = -713;
pub const TBN_INITCUSTOMIZE: i32 = -723;
pub const TBN_LAST: i32 = -720;
pub const TBN_MAPACCELERATOR: i32 = -728;
pub const TBN_QUERYDELETE: i32 = -707;
pub const TBN_QUERYINSERT: i32 = -706;
pub const TBN_RESET: i32 = -705;
pub const TBN_RESTORE: i32 = -721;
pub const TBN_SAVE: i32 = -722;
pub const TBN_TOOLBARCHANGE: i32 = -708;
pub const TBN_WRAPACCELERATOR: i32 = -726;
pub const TBN_WRAPHOTITEM: i32 = -724;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBREPLACEBITMAP {
    pub hInstOld: super::minwindef::HINSTANCE,
    pub nIDOld: usize,
    pub hInstNew: super::minwindef::HINSTANCE,
    pub nIDNew: usize,
    pub nButtons: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBSAVEPARAMSA {
    pub hkr: super::minwindef::HKEY,
    pub pszSubKey: windows_core::PCSTR,
    pub pszValueName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBSAVEPARAMSW {
    pub hkr: super::minwindef::HKEY,
    pub pszSubKey: windows_core::PCWSTR,
    pub pszValueName: windows_core::PCWSTR,
}
pub const TBSTATE_CHECKED: u32 = 1;
pub const TBSTATE_ELLIPSES: u32 = 64;
pub const TBSTATE_ENABLED: u32 = 4;
pub const TBSTATE_HIDDEN: u32 = 8;
pub const TBSTATE_INDETERMINATE: u32 = 16;
pub const TBSTATE_MARKED: u32 = 128;
pub const TBSTATE_PRESSED: u32 = 2;
pub const TBSTATE_WRAP: u32 = 32;
pub const TBSTYLE_ALTDRAG: u32 = 1024;
pub const TBSTYLE_AUTOSIZE: u32 = 16;
pub const TBSTYLE_BUTTON: u32 = 0;
pub const TBSTYLE_CHECK: u32 = 2;
pub const TBSTYLE_CHECKGROUP: u32 = 6;
pub const TBSTYLE_CUSTOMERASE: u32 = 8192;
pub const TBSTYLE_DROPDOWN: u32 = 8;
pub const TBSTYLE_EX_DOUBLEBUFFER: u32 = 128;
pub const TBSTYLE_EX_DRAWDDARROWS: u32 = 1;
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: u32 = 16;
pub const TBSTYLE_EX_MIXEDBUTTONS: u32 = 8;
pub const TBSTYLE_EX_MULTICOLUMN: u32 = 2;
pub const TBSTYLE_EX_VERTICAL: u32 = 4;
pub const TBSTYLE_FLAT: u32 = 2048;
pub const TBSTYLE_GROUP: u32 = 4;
pub const TBSTYLE_LIST: u32 = 4096;
pub const TBSTYLE_NOPREFIX: u32 = 32;
pub const TBSTYLE_REGISTERDROP: u32 = 16384;
pub const TBSTYLE_SEP: u32 = 1;
pub const TBSTYLE_TOOLTIPS: u32 = 256;
pub const TBSTYLE_TRANSPARENT: u32 = 32768;
pub const TBSTYLE_WRAPABLE: u32 = 512;
pub const TBS_AUTOTICKS: u32 = 1;
pub const TBS_BOTH: u32 = 8;
pub const TBS_BOTTOM: u32 = 0;
pub const TBS_DOWNISLEFT: u32 = 1024;
pub const TBS_ENABLESELRANGE: u32 = 32;
pub const TBS_FIXEDLENGTH: u32 = 64;
pub const TBS_HORZ: u32 = 0;
pub const TBS_LEFT: u32 = 4;
pub const TBS_NOTHUMB: u32 = 128;
pub const TBS_NOTICKS: u32 = 16;
pub const TBS_NOTIFYBEFOREMOVE: u32 = 2048;
pub const TBS_REVERSED: u32 = 512;
pub const TBS_RIGHT: u32 = 0;
pub const TBS_TOOLTIPS: u32 = 256;
pub const TBS_TOP: u32 = 4;
pub const TBS_TRANSPARENTBKGND: u32 = 4096;
pub const TBS_VERT: u32 = 2;
pub const TBTS_BOTTOM: u32 = 2;
pub const TBTS_LEFT: u32 = 1;
pub const TBTS_RIGHT: u32 = 3;
pub const TBTS_TOP: u32 = 0;
pub const TB_ADDBITMAP: u32 = 1043;
pub const TB_ADDBUTTONS: u32 = 1044;
pub const TB_ADDBUTTONSA: u32 = 1044;
pub const TB_ADDBUTTONSW: u32 = 1092;
pub const TB_ADDSTRING: u32 = 1052;
pub const TB_ADDSTRINGA: u32 = 1052;
pub const TB_ADDSTRINGW: u32 = 1101;
pub const TB_AUTOSIZE: u32 = 1057;
pub const TB_BOTTOM: u32 = 7;
pub const TB_BUTTONCOUNT: u32 = 1048;
pub const TB_BUTTONSTRUCTSIZE: u32 = 1054;
pub const TB_CHANGEBITMAP: u32 = 1067;
pub const TB_CHECKBUTTON: u32 = 1026;
pub const TB_COMMANDTOINDEX: u32 = 1049;
pub const TB_CUSTOMIZE: u32 = 1051;
pub const TB_DELETEBUTTON: u32 = 1046;
pub const TB_ENABLEBUTTON: u32 = 1025;
pub const TB_ENDTRACK: u32 = 8;
pub const TB_GETANCHORHIGHLIGHT: u32 = 1098;
pub const TB_GETBITMAP: u32 = 1068;
pub const TB_GETBITMAPFLAGS: u32 = 1065;
pub const TB_GETBUTTON: u32 = 1047;
pub const TB_GETBUTTONINFO: u32 = 1089;
pub const TB_GETBUTTONINFOA: u32 = 1089;
pub const TB_GETBUTTONINFOW: u32 = 1087;
pub const TB_GETBUTTONSIZE: u32 = 1082;
pub const TB_GETBUTTONTEXT: u32 = 1069;
pub const TB_GETBUTTONTEXTA: u32 = 1069;
pub const TB_GETBUTTONTEXTW: u32 = 1099;
pub const TB_GETCOLORSCHEME: u32 = 8195;
pub const TB_GETDISABLEDIMAGELIST: u32 = 1079;
pub const TB_GETEXTENDEDSTYLE: u32 = 1109;
pub const TB_GETHOTIMAGELIST: u32 = 1077;
pub const TB_GETHOTITEM: u32 = 1095;
pub const TB_GETIDEALSIZE: u32 = 1123;
pub const TB_GETIMAGELIST: u32 = 1073;
pub const TB_GETIMAGELISTCOUNT: u32 = 1122;
pub const TB_GETINSERTMARK: u32 = 1103;
pub const TB_GETINSERTMARKCOLOR: u32 = 1113;
pub const TB_GETITEMDROPDOWNRECT: u32 = 1127;
pub const TB_GETITEMRECT: u32 = 1053;
pub const TB_GETMAXSIZE: u32 = 1107;
pub const TB_GETMETRICS: u32 = 1125;
pub const TB_GETOBJECT: u32 = 1086;
pub const TB_GETPADDING: u32 = 1110;
pub const TB_GETPRESSEDIMAGELIST: u32 = 1129;
pub const TB_GETRECT: u32 = 1075;
pub const TB_GETROWS: u32 = 1064;
pub const TB_GETSTATE: u32 = 1042;
pub const TB_GETSTRING: u32 = 1116;
pub const TB_GETSTRINGA: u32 = 1116;
pub const TB_GETSTRINGW: u32 = 1115;
pub const TB_GETSTYLE: u32 = 1081;
pub const TB_GETTEXTROWS: u32 = 1085;
pub const TB_GETTOOLTIPS: u32 = 1059;
pub const TB_GETUNICODEFORMAT: u32 = 8198;
pub const TB_HASACCELERATOR: u32 = 1119;
pub const TB_HIDEBUTTON: u32 = 1028;
pub const TB_HITTEST: u32 = 1093;
pub const TB_INDETERMINATE: u32 = 1029;
pub const TB_INSERTBUTTON: u32 = 1045;
pub const TB_INSERTBUTTONA: u32 = 1045;
pub const TB_INSERTBUTTONW: u32 = 1091;
pub const TB_INSERTMARKHITTEST: u32 = 1105;
pub const TB_ISBUTTONCHECKED: u32 = 1034;
pub const TB_ISBUTTONENABLED: u32 = 1033;
pub const TB_ISBUTTONHIDDEN: u32 = 1036;
pub const TB_ISBUTTONHIGHLIGHTED: u32 = 1038;
pub const TB_ISBUTTONINDETERMINATE: u32 = 1037;
pub const TB_ISBUTTONPRESSED: u32 = 1035;
pub const TB_LINEDOWN: u32 = 1;
pub const TB_LINEUP: u32 = 0;
pub const TB_LOADIMAGES: u32 = 1074;
pub const TB_MAPACCELERATOR: u32 = 1102;
pub const TB_MAPACCELERATORA: u32 = 1102;
pub const TB_MAPACCELERATORW: u32 = 1114;
pub const TB_MARKBUTTON: u32 = 1030;
pub const TB_MOVEBUTTON: u32 = 1106;
pub const TB_PAGEDOWN: u32 = 3;
pub const TB_PAGEUP: u32 = 2;
pub const TB_PRESSBUTTON: u32 = 1027;
pub const TB_REPLACEBITMAP: u32 = 1070;
pub const TB_SAVERESTORE: u32 = 1050;
pub const TB_SAVERESTOREA: u32 = 1050;
pub const TB_SAVERESTOREW: u32 = 1100;
pub const TB_SETANCHORHIGHLIGHT: u32 = 1097;
pub const TB_SETBITMAPSIZE: u32 = 1056;
pub const TB_SETBOUNDINGSIZE: u32 = 1117;
pub const TB_SETBUTTONINFO: u32 = 1090;
pub const TB_SETBUTTONINFOA: u32 = 1090;
pub const TB_SETBUTTONINFOW: u32 = 1088;
pub const TB_SETBUTTONSIZE: u32 = 1055;
pub const TB_SETBUTTONWIDTH: u32 = 1083;
pub const TB_SETCMDID: u32 = 1066;
pub const TB_SETCOLORSCHEME: u32 = 8194;
pub const TB_SETDISABLEDIMAGELIST: u32 = 1078;
pub const TB_SETDRAWTEXTFLAGS: u32 = 1094;
pub const TB_SETEXTENDEDSTYLE: u32 = 1108;
pub const TB_SETHOTIMAGELIST: u32 = 1076;
pub const TB_SETHOTITEM: u32 = 1096;
pub const TB_SETHOTITEM2: u32 = 1118;
pub const TB_SETIMAGELIST: u32 = 1072;
pub const TB_SETINDENT: u32 = 1071;
pub const TB_SETINSERTMARK: u32 = 1104;
pub const TB_SETINSERTMARKCOLOR: u32 = 1112;
pub const TB_SETLISTGAP: u32 = 1120;
pub const TB_SETMAXTEXTROWS: u32 = 1084;
pub const TB_SETMETRICS: u32 = 1126;
pub const TB_SETPADDING: u32 = 1111;
pub const TB_SETPARENT: u32 = 1061;
pub const TB_SETPRESSEDIMAGELIST: u32 = 1128;
pub const TB_SETROWS: u32 = 1063;
pub const TB_SETSTATE: u32 = 1041;
pub const TB_SETSTYLE: u32 = 1080;
pub const TB_SETTOOLTIPS: u32 = 1060;
pub const TB_SETUNICODEFORMAT: u32 = 8197;
pub const TB_SETWINDOWTHEME: u32 = 8203;
pub const TB_THUMBPOSITION: u32 = 4;
pub const TB_THUMBTRACK: u32 = 5;
pub const TB_TOP: u32 = 6;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCHITTESTINFO {
    pub pt: super::windef::POINT,
    pub flags: u32,
}
pub const TCHT_NOWHERE: u32 = 1;
pub const TCHT_ONITEM: u32 = 6;
pub const TCHT_ONITEMICON: u32 = 2;
pub const TCHT_ONITEMLABEL: u32 = 4;
pub const TCIF_IMAGE: u32 = 2;
pub const TCIF_PARAM: u32 = 8;
pub const TCIF_RTLREADING: u32 = 4;
pub const TCIF_STATE: u32 = 16;
pub const TCIF_TEXT: u32 = 1;
pub const TCIS_BUTTONPRESSED: u32 = 1;
pub const TCIS_HIGHLIGHTED: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCITEMA {
    pub mask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::minwindef::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCITEMHEADERA {
    pub mask: u32,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCITEMW {
    pub mask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::minwindef::LPARAM,
}
pub const TCM_ADJUSTRECT: u32 = 4904;
pub const TCM_DELETEALLITEMS: u32 = 4873;
pub const TCM_DELETEITEM: u32 = 4872;
pub const TCM_DESELECTALL: u32 = 4914;
pub const TCM_FIRST: u32 = 4864;
pub const TCM_GETCURFOCUS: u32 = 4911;
pub const TCM_GETCURSEL: u32 = 4875;
pub const TCM_GETEXTENDEDSTYLE: u32 = 4917;
pub const TCM_GETIMAGELIST: u32 = 4866;
pub const TCM_GETITEM: u32 = 4869;
pub const TCM_GETITEMA: u32 = 4869;
pub const TCM_GETITEMCOUNT: u32 = 4868;
pub const TCM_GETITEMRECT: u32 = 4874;
pub const TCM_GETITEMW: u32 = 4924;
pub const TCM_GETROWCOUNT: u32 = 4908;
pub const TCM_GETTOOLTIPS: u32 = 4909;
pub const TCM_GETUNICODEFORMAT: u32 = 8198;
pub const TCM_HIGHLIGHTITEM: u32 = 4915;
pub const TCM_HITTEST: u32 = 4877;
pub const TCM_INSERTITEM: u32 = 4871;
pub const TCM_INSERTITEMA: u32 = 4871;
pub const TCM_INSERTITEMW: u32 = 4926;
pub const TCM_REMOVEIMAGE: u32 = 4906;
pub const TCM_SETCURFOCUS: u32 = 4912;
pub const TCM_SETCURSEL: u32 = 4876;
pub const TCM_SETEXTENDEDSTYLE: u32 = 4916;
pub const TCM_SETIMAGELIST: u32 = 4867;
pub const TCM_SETITEM: u32 = 4870;
pub const TCM_SETITEMA: u32 = 4870;
pub const TCM_SETITEMEXTRA: u32 = 4878;
pub const TCM_SETITEMSIZE: u32 = 4905;
pub const TCM_SETITEMW: u32 = 4925;
pub const TCM_SETMINTABWIDTH: u32 = 4913;
pub const TCM_SETPADDING: u32 = 4907;
pub const TCM_SETTOOLTIPS: u32 = 4910;
pub const TCM_SETUNICODEFORMAT: u32 = 8197;
pub const TCN_FIRST: i32 = -550;
pub const TCN_FOCUSCHANGE: i32 = -554;
pub const TCN_GETOBJECT: i32 = -553;
pub const TCN_KEYDOWN: i32 = -550;
pub const TCN_LAST: i32 = -580;
pub const TCN_SELCHANGE: i32 = -551;
pub const TCN_SELCHANGING: i32 = -552;
pub const TCS_BOTTOM: u32 = 2;
pub const TCS_BUTTONS: u32 = 256;
pub const TCS_EX_FLATSEPARATORS: u32 = 1;
pub const TCS_EX_REGISTERDROP: u32 = 2;
pub const TCS_FIXEDWIDTH: u32 = 1024;
pub const TCS_FLATBUTTONS: u32 = 8;
pub const TCS_FOCUSNEVER: u32 = 32768;
pub const TCS_FOCUSONBUTTONDOWN: u32 = 4096;
pub const TCS_FORCEICONLEFT: u32 = 16;
pub const TCS_FORCELABELLEFT: u32 = 32;
pub const TCS_HOTTRACK: u32 = 64;
pub const TCS_MULTILINE: u32 = 512;
pub const TCS_MULTISELECT: u32 = 4;
pub const TCS_OWNERDRAWFIXED: u32 = 8192;
pub const TCS_RAGGEDRIGHT: u32 = 2048;
pub const TCS_RIGHT: u32 = 2;
pub const TCS_RIGHTJUSTIFY: u32 = 0;
pub const TCS_SCROLLOPPOSITE: u32 = 1;
pub const TCS_SINGLELINE: u32 = 0;
pub const TCS_TABS: u32 = 0;
pub const TCS_TOOLTIPS: u32 = 16384;
pub const TCS_VERTICAL: u32 = 128;
pub const TDCBF_CANCEL_BUTTON: _TASKDIALOG_COMMON_BUTTON_FLAGS = 8;
pub const TDCBF_CLOSE_BUTTON: _TASKDIALOG_COMMON_BUTTON_FLAGS = 32;
pub const TDCBF_NO_BUTTON: _TASKDIALOG_COMMON_BUTTON_FLAGS = 4;
pub const TDCBF_OK_BUTTON: _TASKDIALOG_COMMON_BUTTON_FLAGS = 1;
pub const TDCBF_RETRY_BUTTON: _TASKDIALOG_COMMON_BUTTON_FLAGS = 16;
pub const TDCBF_YES_BUTTON: _TASKDIALOG_COMMON_BUTTON_FLAGS = 2;
pub const TDE_CONTENT: TASKDIALOG_ELEMENTS = 0;
pub const TDE_EXPANDED_INFORMATION: TASKDIALOG_ELEMENTS = 1;
pub const TDE_FOOTER: TASKDIALOG_ELEMENTS = 2;
pub const TDE_MAIN_INSTRUCTION: TASKDIALOG_ELEMENTS = 3;
pub const TDF_ALLOW_DIALOG_CANCELLATION: _TASKDIALOG_FLAGS = 8;
pub const TDF_CALLBACK_TIMER: _TASKDIALOG_FLAGS = 2048;
pub const TDF_CAN_BE_MINIMIZED: _TASKDIALOG_FLAGS = 32768;
pub const TDF_ENABLE_HYPERLINKS: _TASKDIALOG_FLAGS = 1;
pub const TDF_EXPANDED_BY_DEFAULT: _TASKDIALOG_FLAGS = 128;
pub const TDF_EXPAND_FOOTER_AREA: _TASKDIALOG_FLAGS = 64;
pub const TDF_NO_DEFAULT_RADIO_BUTTON: _TASKDIALOG_FLAGS = 16384;
pub const TDF_NO_SET_FOREGROUND: _TASKDIALOG_FLAGS = 65536;
pub const TDF_POSITION_RELATIVE_TO_WINDOW: _TASKDIALOG_FLAGS = 4096;
pub const TDF_RTL_LAYOUT: _TASKDIALOG_FLAGS = 8192;
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: _TASKDIALOG_FLAGS = 1024;
pub const TDF_SHOW_PROGRESS_BAR: _TASKDIALOG_FLAGS = 512;
pub const TDF_SIZE_TO_CONTENT: _TASKDIALOG_FLAGS = 16777216;
pub const TDF_USE_COMMAND_LINKS: _TASKDIALOG_FLAGS = 16;
pub const TDF_USE_COMMAND_LINKS_NO_ICON: _TASKDIALOG_FLAGS = 32;
pub const TDF_USE_HICON_FOOTER: _TASKDIALOG_FLAGS = 4;
pub const TDF_USE_HICON_MAIN: _TASKDIALOG_FLAGS = 2;
pub const TDF_VERIFICATION_FLAG_CHECKED: _TASKDIALOG_FLAGS = 256;
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
pub const TRBN_FIRST: i32 = -1501;
pub const TRBN_LAST: i32 = -1519;
pub const TRBN_THUMBPOSCHANGING: i32 = -1502;
pub const TTDT_AUTOMATIC: u32 = 0;
pub const TTDT_AUTOPOP: u32 = 2;
pub const TTDT_INITIAL: u32 = 3;
pub const TTDT_RESHOW: u32 = 1;
pub const TTF_ABSOLUTE: u32 = 128;
pub const TTF_CENTERTIP: u32 = 2;
pub const TTF_DI_SETITEM: u32 = 32768;
pub const TTF_IDISHWND: u32 = 1;
pub const TTF_PARSELINKS: u32 = 4096;
pub const TTF_RTLREADING: u32 = 4;
pub const TTF_SUBCLASS: u32 = 16;
pub const TTF_TRACK: u32 = 32;
pub const TTF_TRANSPARENT: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TTHITTESTINFOA {
    pub hwnd: super::windef::HWND,
    pub pt: super::windef::POINT,
    pub ti: TTTOOLINFOA,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TTHITTESTINFOW {
    pub hwnd: super::windef::HWND,
    pub pt: super::windef::POINT,
    pub ti: TTTOOLINFOW,
}
pub const TTI_ERROR: u32 = 3;
pub const TTI_ERROR_LARGE: u32 = 6;
pub const TTI_INFO: u32 = 1;
pub const TTI_INFO_LARGE: u32 = 4;
pub const TTI_NONE: u32 = 0;
pub const TTI_WARNING: u32 = 2;
pub const TTI_WARNING_LARGE: u32 = 5;
pub const TTM_ACTIVATE: u32 = 1025;
pub const TTM_ADDTOOL: u32 = 1028;
pub const TTM_ADDTOOLA: u32 = 1028;
pub const TTM_ADDTOOLW: u32 = 1074;
pub const TTM_ADJUSTRECT: u32 = 1055;
pub const TTM_DELTOOL: u32 = 1029;
pub const TTM_DELTOOLA: u32 = 1029;
pub const TTM_DELTOOLW: u32 = 1075;
pub const TTM_ENUMTOOLS: u32 = 1038;
pub const TTM_ENUMTOOLSA: u32 = 1038;
pub const TTM_ENUMTOOLSW: u32 = 1082;
pub const TTM_GETBUBBLESIZE: u32 = 1054;
pub const TTM_GETCURRENTTOOL: u32 = 1039;
pub const TTM_GETCURRENTTOOLA: u32 = 1039;
pub const TTM_GETCURRENTTOOLW: u32 = 1083;
pub const TTM_GETDELAYTIME: u32 = 1045;
pub const TTM_GETMARGIN: u32 = 1051;
pub const TTM_GETMAXTIPWIDTH: u32 = 1049;
pub const TTM_GETTEXT: u32 = 1035;
pub const TTM_GETTEXTA: u32 = 1035;
pub const TTM_GETTEXTW: u32 = 1080;
pub const TTM_GETTIPBKCOLOR: u32 = 1046;
pub const TTM_GETTIPTEXTCOLOR: u32 = 1047;
pub const TTM_GETTITLE: u32 = 1059;
pub const TTM_GETTOOLCOUNT: u32 = 1037;
pub const TTM_GETTOOLINFO: u32 = 1032;
pub const TTM_GETTOOLINFOA: u32 = 1032;
pub const TTM_GETTOOLINFOW: u32 = 1077;
pub const TTM_HITTEST: u32 = 1034;
pub const TTM_HITTESTA: u32 = 1034;
pub const TTM_HITTESTW: u32 = 1079;
pub const TTM_NEWTOOLRECT: u32 = 1030;
pub const TTM_NEWTOOLRECTA: u32 = 1030;
pub const TTM_NEWTOOLRECTW: u32 = 1076;
pub const TTM_POP: u32 = 1052;
pub const TTM_POPUP: u32 = 1058;
pub const TTM_RELAYEVENT: u32 = 1031;
pub const TTM_SETDELAYTIME: u32 = 1027;
pub const TTM_SETMARGIN: u32 = 1050;
pub const TTM_SETMAXTIPWIDTH: u32 = 1048;
pub const TTM_SETTIPBKCOLOR: u32 = 1043;
pub const TTM_SETTIPTEXTCOLOR: u32 = 1044;
pub const TTM_SETTITLE: u32 = 1056;
pub const TTM_SETTITLEA: u32 = 1056;
pub const TTM_SETTITLEW: u32 = 1057;
pub const TTM_SETTOOLINFO: u32 = 1033;
pub const TTM_SETTOOLINFOA: u32 = 1033;
pub const TTM_SETTOOLINFOW: u32 = 1078;
pub const TTM_SETWINDOWTHEME: u32 = 8203;
pub const TTM_TRACKACTIVATE: u32 = 1041;
pub const TTM_TRACKPOSITION: u32 = 1042;
pub const TTM_UPDATE: u32 = 1053;
pub const TTM_UPDATETIPTEXT: u32 = 1036;
pub const TTM_UPDATETIPTEXTA: u32 = 1036;
pub const TTM_UPDATETIPTEXTW: u32 = 1081;
pub const TTM_WINDOWFROMPOINT: u32 = 1040;
pub const TTN_FIRST: i32 = -520;
pub const TTN_GETDISPINFO: i32 = -520;
pub const TTN_GETDISPINFOA: i32 = -520;
pub const TTN_GETDISPINFOW: i32 = -530;
pub const TTN_LAST: i32 = -549;
pub const TTN_LINKCLICK: i32 = -523;
pub const TTN_NEEDTEXT: i32 = -520;
pub const TTN_NEEDTEXTA: i32 = -520;
pub const TTN_NEEDTEXTW: i32 = -530;
pub const TTN_POP: i32 = -522;
pub const TTN_SHOW: i32 = -521;
pub const TTS_ALWAYSTIP: u32 = 1;
pub const TTS_BALLOON: u32 = 64;
pub const TTS_CLOSE: u32 = 128;
pub const TTS_NOANIMATE: u32 = 16;
pub const TTS_NOFADE: u32 = 32;
pub const TTS_NOPREFIX: u32 = 2;
pub const TTS_USEVISUALSTYLE: u32 = 256;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTTOOLINFOA {
    pub cbSize: u32,
    pub uFlags: u32,
    pub hwnd: super::windef::HWND,
    pub uId: usize,
    pub rect: super::windef::RECT,
    pub hinst: super::minwindef::HINSTANCE,
    pub lpszText: windows_core::PSTR,
    pub lParam: super::minwindef::LPARAM,
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
pub const TTTOOLINFOA_V1_SIZE: u32 = 56;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOA_V2_SIZE: u32 = 44;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOA_V2_SIZE: u32 = 64;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOA_V3_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOA_V3_SIZE: u32 = 72;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTTOOLINFOW {
    pub cbSize: u32,
    pub uFlags: u32,
    pub hwnd: super::windef::HWND,
    pub uId: usize,
    pub rect: super::windef::RECT,
    pub hinst: super::minwindef::HINSTANCE,
    pub lpszText: windows_core::PWSTR,
    pub lParam: super::minwindef::LPARAM,
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
pub const TTTOOLINFOW_V1_SIZE: u32 = 56;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOW_V2_SIZE: u32 = 44;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOW_V2_SIZE: u32 = 64;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFOW_V3_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFOW_V3_SIZE: u32 = 72;
#[cfg(target_arch = "x86")]
pub const TTTOOLINFO_V1_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TTTOOLINFO_V1_SIZE: u32 = 56;
pub const TVCDRF_NOIMAGES: u32 = 65536;
pub const TVC_BYKEYBOARD: u32 = 2;
pub const TVC_BYMOUSE: u32 = 1;
pub const TVC_UNKNOWN: u32 = 0;
pub const TVE_COLLAPSE: u32 = 1;
pub const TVE_COLLAPSERESET: u32 = 32768;
pub const TVE_EXPAND: u32 = 2;
pub const TVE_EXPANDPARTIAL: u32 = 16384;
pub const TVE_TOGGLE: u32 = 3;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TVGETITEMPARTRECTINFO {
    pub hti: HTREEITEM,
    pub prc: *mut super::windef::RECT,
    pub partID: TVITEMPART,
}
#[cfg(feature = "windef")]
impl Default for TVGETITEMPARTRECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TVGIPR_BUTTON: TVITEMPART = 1;
pub const TVGN_CARET: u32 = 9;
pub const TVGN_CHILD: u32 = 4;
pub const TVGN_DROPHILITE: u32 = 8;
pub const TVGN_FIRSTVISIBLE: u32 = 5;
pub const TVGN_LASTVISIBLE: u32 = 10;
pub const TVGN_NEXT: u32 = 1;
pub const TVGN_NEXTSELECTED: u32 = 11;
pub const TVGN_NEXTVISIBLE: u32 = 6;
pub const TVGN_PARENT: u32 = 3;
pub const TVGN_PREVIOUS: u32 = 2;
pub const TVGN_PREVIOUSVISIBLE: u32 = 7;
pub const TVGN_ROOT: u32 = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TVHITTESTINFO {
    pub pt: super::windef::POINT,
    pub flags: u32,
    pub hItem: HTREEITEM,
}
pub const TVHT_ABOVE: u32 = 256;
pub const TVHT_BELOW: u32 = 512;
pub const TVHT_NOWHERE: u32 = 1;
pub const TVHT_ONITEM: u32 = 70;
pub const TVHT_ONITEMBUTTON: u32 = 16;
pub const TVHT_ONITEMICON: u32 = 2;
pub const TVHT_ONITEMINDENT: u32 = 8;
pub const TVHT_ONITEMLABEL: u32 = 4;
pub const TVHT_ONITEMRIGHT: u32 = 32;
pub const TVHT_ONITEMSTATEICON: u32 = 64;
pub const TVHT_TOLEFT: u32 = 2048;
pub const TVHT_TORIGHT: u32 = 1024;
pub const TVIF_CHILDREN: u32 = 64;
pub const TVIF_DI_SETITEM: u32 = 4096;
pub const TVIF_EXPANDEDIMAGE: u32 = 512;
pub const TVIF_HANDLE: u32 = 16;
pub const TVIF_IMAGE: u32 = 2;
pub const TVIF_INTEGRAL: u32 = 128;
pub const TVIF_PARAM: u32 = 4;
pub const TVIF_SELECTEDIMAGE: u32 = 32;
pub const TVIF_STATE: u32 = 8;
pub const TVIF_STATEEX: u32 = 256;
pub const TVIF_TEXT: u32 = 1;
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
pub const TVINSERTSTRUCTA_V1_SIZE: u32 = 72;
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
pub const TVINSERTSTRUCTW_V1_SIZE: u32 = 72;
#[cfg(target_arch = "x86")]
pub const TVINSERTSTRUCT_V1_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const TVINSERTSTRUCT_V1_SIZE: u32 = 72;
pub const TVIS_BOLD: u32 = 16;
pub const TVIS_CUT: u32 = 4;
pub const TVIS_DROPHILITED: u32 = 8;
pub const TVIS_EXPANDED: u32 = 32;
pub const TVIS_EXPANDEDONCE: u32 = 64;
pub const TVIS_EXPANDPARTIAL: u32 = 128;
pub const TVIS_EX_ALL: u32 = 2;
pub const TVIS_EX_DISABLED: u32 = 2;
pub const TVIS_EX_FLAT: u32 = 1;
pub const TVIS_OVERLAYMASK: u32 = 3840;
pub const TVIS_SELECTED: u32 = 2;
pub const TVIS_STATEIMAGEMASK: u32 = 61440;
pub const TVIS_USERMASK: u32 = 61440;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub lParam: super::minwindef::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type TVITEMEX = TVITEMEXA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub lParam: super::minwindef::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::windef::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub lParam: super::minwindef::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::windef::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
pub type TVITEMPART = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub lParam: super::minwindef::LPARAM,
}
pub const TVI_FIRST: HTREEITEM = HTREEITEM(-65535 as _);
pub const TVI_LAST: HTREEITEM = HTREEITEM(-65534 as _);
pub const TVI_ROOT: HTREEITEM = HTREEITEM(-65536 as _);
pub const TVI_SORT: HTREEITEM = HTREEITEM(-65533 as _);
pub const TVM_CREATEDRAGIMAGE: u32 = 4370;
pub const TVM_DELETEITEM: u32 = 4353;
pub const TVM_EDITLABEL: u32 = 4366;
pub const TVM_EDITLABELA: u32 = 4366;
pub const TVM_EDITLABELW: u32 = 4417;
pub const TVM_ENDEDITLABELNOW: u32 = 4374;
pub const TVM_ENSUREVISIBLE: u32 = 4372;
pub const TVM_EXPAND: u32 = 4354;
pub const TVM_GETBKCOLOR: u32 = 4383;
pub const TVM_GETCOUNT: u32 = 4357;
pub const TVM_GETEDITCONTROL: u32 = 4367;
pub const TVM_GETEXTENDEDSTYLE: u32 = 4397;
pub const TVM_GETIMAGELIST: u32 = 4360;
pub const TVM_GETINDENT: u32 = 4358;
pub const TVM_GETINSERTMARKCOLOR: u32 = 4390;
pub const TVM_GETISEARCHSTRING: u32 = 4375;
pub const TVM_GETISEARCHSTRINGA: u32 = 4375;
pub const TVM_GETISEARCHSTRINGW: u32 = 4416;
pub const TVM_GETITEM: u32 = 4364;
pub const TVM_GETITEMA: u32 = 4364;
pub const TVM_GETITEMHEIGHT: u32 = 4380;
pub const TVM_GETITEMPARTRECT: u32 = 4424;
pub const TVM_GETITEMRECT: u32 = 4356;
pub const TVM_GETITEMSTATE: u32 = 4391;
pub const TVM_GETITEMW: u32 = 4414;
pub const TVM_GETLINECOLOR: u32 = 4393;
pub const TVM_GETNEXTITEM: u32 = 4362;
pub const TVM_GETSCROLLTIME: u32 = 4386;
pub const TVM_GETSELECTEDCOUNT: u32 = 4422;
pub const TVM_GETTEXTCOLOR: u32 = 4384;
pub const TVM_GETTOOLTIPS: u32 = 4377;
pub const TVM_GETUNICODEFORMAT: u32 = 8198;
pub const TVM_GETVISIBLECOUNT: u32 = 4368;
pub const TVM_HITTEST: u32 = 4369;
pub const TVM_INSERTITEM: u32 = 4352;
pub const TVM_INSERTITEMA: u32 = 4352;
pub const TVM_INSERTITEMW: u32 = 4402;
pub const TVM_MAPACCIDTOHTREEITEM: u32 = 4394;
pub const TVM_MAPHTREEITEMTOACCID: u32 = 4395;
pub const TVM_SELECTITEM: u32 = 4363;
pub const TVM_SETAUTOSCROLLINFO: u32 = 4411;
pub const TVM_SETBKCOLOR: u32 = 4381;
pub const TVM_SETBORDER: u32 = 4387;
pub const TVM_SETEXTENDEDSTYLE: u32 = 4396;
pub const TVM_SETHOT: u32 = 4410;
pub const TVM_SETIMAGELIST: u32 = 4361;
pub const TVM_SETINDENT: u32 = 4359;
pub const TVM_SETINSERTMARK: u32 = 4378;
pub const TVM_SETINSERTMARKCOLOR: u32 = 4389;
pub const TVM_SETITEM: u32 = 4365;
pub const TVM_SETITEMA: u32 = 4365;
pub const TVM_SETITEMHEIGHT: u32 = 4379;
pub const TVM_SETITEMW: u32 = 4415;
pub const TVM_SETLINECOLOR: u32 = 4392;
pub const TVM_SETSCROLLTIME: u32 = 4385;
pub const TVM_SETTEXTCOLOR: u32 = 4382;
pub const TVM_SETTOOLTIPS: u32 = 4376;
pub const TVM_SETUNICODEFORMAT: u32 = 8197;
pub const TVM_SHOWINFOTIP: u32 = 4423;
pub const TVM_SORTCHILDREN: u32 = 4371;
pub const TVM_SORTCHILDRENCB: u32 = 4373;
pub const TVNRET_DEFAULT: u32 = 0;
pub const TVNRET_SKIPNEW: u32 = 2;
pub const TVNRET_SKIPOLD: u32 = 1;
pub const TVN_ASYNCDRAW: i32 = -420;
pub const TVN_BEGINDRAG: i32 = -407;
pub const TVN_BEGINDRAGA: i32 = -407;
pub const TVN_BEGINDRAGW: i32 = -456;
pub const TVN_BEGINLABELEDIT: i32 = -410;
pub const TVN_BEGINLABELEDITA: i32 = -410;
pub const TVN_BEGINLABELEDITW: i32 = -459;
pub const TVN_BEGINRDRAG: i32 = -408;
pub const TVN_BEGINRDRAGA: i32 = -408;
pub const TVN_BEGINRDRAGW: i32 = -457;
pub const TVN_DELETEITEM: i32 = -409;
pub const TVN_DELETEITEMA: i32 = -409;
pub const TVN_DELETEITEMW: i32 = -458;
pub const TVN_ENDLABELEDIT: i32 = -411;
pub const TVN_ENDLABELEDITA: i32 = -411;
pub const TVN_ENDLABELEDITW: i32 = -460;
pub const TVN_FIRST: i32 = -400;
pub const TVN_GETDISPINFO: i32 = -403;
pub const TVN_GETDISPINFOA: i32 = -403;
pub const TVN_GETDISPINFOW: i32 = -452;
pub const TVN_GETINFOTIP: i32 = -413;
pub const TVN_GETINFOTIPA: i32 = -413;
pub const TVN_GETINFOTIPW: i32 = -414;
pub const TVN_ITEMCHANGED: i32 = -418;
pub const TVN_ITEMCHANGEDA: i32 = -418;
pub const TVN_ITEMCHANGEDW: i32 = -419;
pub const TVN_ITEMCHANGING: i32 = -416;
pub const TVN_ITEMCHANGINGA: i32 = -416;
pub const TVN_ITEMCHANGINGW: i32 = -417;
pub const TVN_ITEMEXPANDED: i32 = -406;
pub const TVN_ITEMEXPANDEDA: i32 = -406;
pub const TVN_ITEMEXPANDEDW: i32 = -455;
pub const TVN_ITEMEXPANDING: i32 = -405;
pub const TVN_ITEMEXPANDINGA: i32 = -405;
pub const TVN_ITEMEXPANDINGW: i32 = -454;
pub const TVN_KEYDOWN: i32 = -412;
pub const TVN_LAST: i32 = -499;
pub const TVN_SELCHANGED: i32 = -402;
pub const TVN_SELCHANGEDA: i32 = -402;
pub const TVN_SELCHANGEDW: i32 = -451;
pub const TVN_SELCHANGING: i32 = -401;
pub const TVN_SELCHANGINGA: i32 = -401;
pub const TVN_SELCHANGINGW: i32 = -450;
pub const TVN_SETDISPINFO: i32 = -404;
pub const TVN_SETDISPINFOA: i32 = -404;
pub const TVN_SETDISPINFOW: i32 = -453;
pub const TVN_SINGLEEXPAND: i32 = -415;
pub const TVSBF_XBORDER: u32 = 1;
pub const TVSBF_YBORDER: u32 = 2;
pub const TVSIL_NORMAL: u32 = 0;
pub const TVSIL_STATE: u32 = 2;
pub const TVSI_NOSINGLEEXPAND: u32 = 32768;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default)]
pub struct TVSORTCB {
    pub hParent: HTREEITEM,
    pub lpfnCompare: PFNTVCOMPARE,
    pub lParam: super::minwindef::LPARAM,
}
pub const TVS_CHECKBOXES: u32 = 256;
pub const TVS_DISABLEDRAGDROP: u32 = 16;
pub const TVS_EDITLABELS: u32 = 8;
pub const TVS_EX_AUTOHSCROLL: u32 = 32;
pub const TVS_EX_DIMMEDCHECKBOXES: u32 = 512;
pub const TVS_EX_DOUBLEBUFFER: u32 = 4;
pub const TVS_EX_DRAWIMAGEASYNC: u32 = 1024;
pub const TVS_EX_EXCLUSIONCHECKBOXES: u32 = 256;
pub const TVS_EX_FADEINOUTEXPANDOS: u32 = 64;
pub const TVS_EX_MULTISELECT: u32 = 2;
pub const TVS_EX_NOINDENTSTATE: u32 = 8;
pub const TVS_EX_NOSINGLECOLLAPSE: u32 = 1;
pub const TVS_EX_PARTIALCHECKBOXES: u32 = 128;
pub const TVS_EX_RICHTOOLTIP: u32 = 16;
pub const TVS_FULLROWSELECT: u32 = 4096;
pub const TVS_HASBUTTONS: u32 = 1;
pub const TVS_HASLINES: u32 = 2;
pub const TVS_INFOTIP: u32 = 2048;
pub const TVS_LINESATROOT: u32 = 4;
pub const TVS_NOHSCROLL: u32 = 32768;
pub const TVS_NONEVENHEIGHT: u32 = 16384;
pub const TVS_NOSCROLL: u32 = 8192;
pub const TVS_NOTOOLTIPS: u32 = 128;
pub const TVS_RTLREADING: u32 = 64;
pub const TVS_SHOWSELALWAYS: u32 = 32;
pub const TVS_SINGLEEXPAND: u32 = 1024;
pub const TVS_TRACKSELECT: u32 = 512;
pub const TV_FIRST: u32 = 4352;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UDACCEL {
    pub nSec: u32,
    pub nInc: u32,
}
pub const UDM_GETACCEL: u32 = 1132;
pub const UDM_GETBASE: u32 = 1134;
pub const UDM_GETBUDDY: u32 = 1130;
pub const UDM_GETPOS: u32 = 1128;
pub const UDM_GETPOS32: u32 = 1138;
pub const UDM_GETRANGE: u32 = 1126;
pub const UDM_GETRANGE32: u32 = 1136;
pub const UDM_GETUNICODEFORMAT: u32 = 8198;
pub const UDM_SETACCEL: u32 = 1131;
pub const UDM_SETBASE: u32 = 1133;
pub const UDM_SETBUDDY: u32 = 1129;
pub const UDM_SETPOS: u32 = 1127;
pub const UDM_SETPOS32: u32 = 1137;
pub const UDM_SETRANGE: u32 = 1125;
pub const UDM_SETRANGE32: u32 = 1135;
pub const UDM_SETUNICODEFORMAT: u32 = 8197;
pub const UDN_DELTAPOS: i32 = -722;
pub const UDN_FIRST: i32 = -721;
pub const UDN_LAST: i32 = -729;
pub const UDS_ALIGNLEFT: u32 = 8;
pub const UDS_ALIGNRIGHT: u32 = 4;
pub const UDS_ARROWKEYS: u32 = 32;
pub const UDS_AUTOBUDDY: u32 = 16;
pub const UDS_HORZ: u32 = 64;
pub const UDS_HOTTRACK: u32 = 256;
pub const UDS_NOTHOUSANDS: u32 = 128;
pub const UDS_SETBUDDYINT: u32 = 2;
pub const UDS_WRAP: u32 = 1;
pub const UD_MAXVAL: u32 = 32767;
pub const UD_MINVAL: i32 = -32767;
pub const UPDOWN_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_updown32");
pub const UPDOWN_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_updown32");
pub const VIEW_DETAILS: u32 = 3;
pub const VIEW_LARGEICONS: u32 = 0;
pub const VIEW_LIST: u32 = 2;
pub const VIEW_NETCONNECT: u32 = 9;
pub const VIEW_NETDISCONNECT: u32 = 10;
pub const VIEW_NEWFOLDER: u32 = 11;
pub const VIEW_PARENTFOLDER: u32 = 8;
pub const VIEW_SMALLICONS: u32 = 1;
pub const VIEW_SORTDATE: u32 = 6;
pub const VIEW_SORTNAME: u32 = 4;
pub const VIEW_SORTSIZE: u32 = 5;
pub const VIEW_SORTTYPE: u32 = 7;
pub const VIEW_VIEWMENU: u32 = 12;
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
pub const WMN_FIRST: i32 = -1000;
pub const WMN_LAST: i32 = -1200;
pub const WSB_PROP_CXHSCROLL: u32 = 2;
pub const WSB_PROP_CXHTHUMB: u32 = 16;
pub const WSB_PROP_CXVSCROLL: u32 = 8;
pub const WSB_PROP_CYHSCROLL: u32 = 4;
pub const WSB_PROP_CYVSCROLL: u32 = 1;
pub const WSB_PROP_CYVTHUMB: u32 = 32;
pub const WSB_PROP_HBKGCOLOR: u32 = 128;
pub const WSB_PROP_HSTYLE: u32 = 512;
pub const WSB_PROP_MASK: u32 = 4095;
pub const WSB_PROP_PALETTE: u32 = 2048;
pub const WSB_PROP_VBKGCOLOR: u32 = 64;
pub const WSB_PROP_VSTYLE: u32 = 256;
pub const WSB_PROP_WINSTYLE: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _IMAGELIST(pub u8);
pub type _LI_METRIC = i32;
pub type _TASKDIALOG_COMMON_BUTTON_FLAGS = i32;
pub type _TASKDIALOG_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _TREEITEM(pub u8);
