#[inline]
pub unsafe fn DPA_Clone(hdpa: *const _DPA, hdpanew: Option<*mut _DPA>) -> HDPA {
    windows_core::link!("comctl32.dll" "system" fn DPA_Clone(hdpa : *const _DPA, hdpanew : *mut _DPA) -> HDPA);
    unsafe { DPA_Clone(hdpa, hdpanew.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_Create(citemgrow: i32) -> HDPA {
    windows_core::link!("comctl32.dll" "system" fn DPA_Create(citemgrow : i32) -> HDPA);
    unsafe { DPA_Create(citemgrow) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DPA_CreateEx(cpgrow: i32, hheap: Option<super::winnt::HANDLE>) -> HDPA {
    windows_core::link!("comctl32.dll" "system" fn DPA_CreateEx(cpgrow : i32, hheap : super::winnt::HANDLE) -> HDPA);
    unsafe { DPA_CreateEx(cpgrow, hheap.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_DeleteAllPtrs(hdpa: *mut _DPA) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_DeleteAllPtrs(hdpa : *mut _DPA) -> windows_core::BOOL);
    unsafe { DPA_DeleteAllPtrs(hdpa as _) }
}
#[inline]
pub unsafe fn DPA_DeletePtr(hdpa: *mut _DPA, i: i32) -> *mut core::ffi::c_void {
    windows_core::link!("comctl32.dll" "system" fn DPA_DeletePtr(hdpa : *mut _DPA, i : i32) -> *mut core::ffi::c_void);
    unsafe { DPA_DeletePtr(hdpa as _, i) }
}
#[inline]
pub unsafe fn DPA_Destroy(hdpa: Option<*mut _DPA>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Destroy(hdpa : *mut _DPA) -> windows_core::BOOL);
    unsafe { DPA_Destroy(hdpa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_DestroyCallback(hdpa: Option<*mut _DPA>, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DPA_DestroyCallback(hdpa : *mut _DPA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DPA_DestroyCallback(hdpa.unwrap_or(core::mem::zeroed()) as _, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_EnumCallback(hdpa: Option<*const _DPA>, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DPA_EnumCallback(hdpa : *const _DPA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DPA_EnumCallback(hdpa.unwrap_or(core::mem::zeroed()) as _, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_GetPtr(hdpa: *const _DPA, i: isize) -> *mut core::ffi::c_void {
    windows_core::link!("comctl32.dll" "system" fn DPA_GetPtr(hdpa : *const _DPA, i : isize) -> *mut core::ffi::c_void);
    unsafe { DPA_GetPtr(hdpa, i) }
}
#[inline]
pub unsafe fn DPA_GetPtrIndex(hdpa: *const _DPA, p: Option<*const core::ffi::c_void>) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DPA_GetPtrIndex(hdpa : *const _DPA, p : *const core::ffi::c_void) -> i32);
    unsafe { DPA_GetPtrIndex(hdpa, p.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_GetSize(hdpa: Option<*const _DPA>) -> u64 {
    windows_core::link!("comctl32.dll" "system" fn DPA_GetSize(hdpa : *const _DPA) -> u64);
    unsafe { DPA_GetSize(hdpa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_Grow(pdpa: *mut _DPA, cp: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Grow(pdpa : *mut _DPA, cp : i32) -> windows_core::BOOL);
    unsafe { DPA_Grow(pdpa as _, cp) }
}
#[inline]
pub unsafe fn DPA_InsertPtr(hdpa: *mut _DPA, i: i32, p: Option<*const core::ffi::c_void>) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DPA_InsertPtr(hdpa : *mut _DPA, i : i32, p : *const core::ffi::c_void) -> i32);
    unsafe { DPA_InsertPtr(hdpa as _, i, p.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn DPA_LoadStream<P2>(phdpa: *mut HDPA, pfn: PFNDPASTREAM, pstream: P2, pvinstdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn DPA_LoadStream(phdpa : *mut HDPA, pfn : PFNDPASTREAM, pstream : *mut core::ffi::c_void, pvinstdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DPA_LoadStream(phdpa as _, pfn, pstream.param().abi(), pvinstdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn DPA_Merge(hdpadest: *mut _DPA, hdpasrc: *const _DPA, dwflags: u32, pfncompare: PFNDACOMPARE, pfnmerge: PFNDPAMERGE, lparam: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Merge(hdpadest : *mut _DPA, hdpasrc : *const _DPA, dwflags : u32, pfncompare : PFNDACOMPARE, pfnmerge : PFNDPAMERGE, lparam : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { DPA_Merge(hdpadest as _, hdpasrc, dwflags, pfncompare, pfnmerge, lparam) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn DPA_SaveStream<P2>(hdpa: *const _DPA, pfn: PFNDPASTREAM, pstream: P2, pvinstdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn DPA_SaveStream(hdpa : *const _DPA, pfn : PFNDPASTREAM, pstream : *mut core::ffi::c_void, pvinstdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DPA_SaveStream(hdpa, pfn, pstream.param().abi(), pvinstdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn DPA_Search(hdpa: *const _DPA, pfind: Option<*const core::ffi::c_void>, istart: i32, pfncompare: PFNDACOMPARE, lparam: super::minwindef::LPARAM, options: u32) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DPA_Search(hdpa : *const _DPA, pfind : *const core::ffi::c_void, istart : i32, pfncompare : PFNDACOMPARE, lparam : super::minwindef::LPARAM, options : u32) -> i32);
    unsafe { DPA_Search(hdpa, pfind.unwrap_or(core::mem::zeroed()) as _, istart, pfncompare, lparam, options) }
}
#[inline]
pub unsafe fn DPA_SetPtr(hdpa: *mut _DPA, i: i32, p: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_SetPtr(hdpa : *mut _DPA, i : i32, p : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DPA_SetPtr(hdpa as _, i, p.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn DPA_Sort(hdpa: *mut _DPA, pfncompare: PFNDACOMPARE, lparam: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Sort(hdpa : *mut _DPA, pfncompare : PFNDACOMPARE, lparam : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { DPA_Sort(hdpa as _, pfncompare, lparam) }
}
#[inline]
pub unsafe fn DSA_Clone(hdsa: *const _DSA) -> HDSA {
    windows_core::link!("comctl32.dll" "system" fn DSA_Clone(hdsa : *const _DSA) -> HDSA);
    unsafe { DSA_Clone(hdsa) }
}
#[inline]
pub unsafe fn DSA_Create(cbitem: i32, citemgrow: i32) -> HDSA {
    windows_core::link!("comctl32.dll" "system" fn DSA_Create(cbitem : i32, citemgrow : i32) -> HDSA);
    unsafe { DSA_Create(cbitem, citemgrow) }
}
#[inline]
pub unsafe fn DSA_DeleteAllItems(hdsa: *mut _DSA) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_DeleteAllItems(hdsa : *mut _DSA) -> windows_core::BOOL);
    unsafe { DSA_DeleteAllItems(hdsa as _) }
}
#[inline]
pub unsafe fn DSA_DeleteItem(hdsa: *mut _DSA, i: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_DeleteItem(hdsa : *mut _DSA, i : i32) -> windows_core::BOOL);
    unsafe { DSA_DeleteItem(hdsa as _, i) }
}
#[inline]
pub unsafe fn DSA_Destroy(hdsa: Option<*mut _DSA>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_Destroy(hdsa : *mut _DSA) -> windows_core::BOOL);
    unsafe { DSA_Destroy(hdsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_DestroyCallback(hdsa: Option<*mut _DSA>, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DSA_DestroyCallback(hdsa : *mut _DSA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DSA_DestroyCallback(hdsa.unwrap_or(core::mem::zeroed()) as _, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_EnumCallback(hdsa: *const _DSA, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DSA_EnumCallback(hdsa : *const _DSA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DSA_EnumCallback(hdsa, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_GetItem(hdsa: *const _DSA, i: i32, pitem: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_GetItem(hdsa : *const _DSA, i : i32, pitem : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DSA_GetItem(hdsa, i, pitem as _) }
}
#[inline]
pub unsafe fn DSA_GetItemPtr(hdsa: *const _DSA, i: i32) -> *mut core::ffi::c_void {
    windows_core::link!("comctl32.dll" "system" fn DSA_GetItemPtr(hdsa : *const _DSA, i : i32) -> *mut core::ffi::c_void);
    unsafe { DSA_GetItemPtr(hdsa, i) }
}
#[inline]
pub unsafe fn DSA_GetSize(hdsa: Option<*const _DSA>) -> u64 {
    windows_core::link!("comctl32.dll" "system" fn DSA_GetSize(hdsa : *const _DSA) -> u64);
    unsafe { DSA_GetSize(hdsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_InsertItem(hdsa: *mut _DSA, i: i32, pitem: *const core::ffi::c_void) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DSA_InsertItem(hdsa : *mut _DSA, i : i32, pitem : *const core::ffi::c_void) -> i32);
    unsafe { DSA_InsertItem(hdsa as _, i, pitem) }
}
#[inline]
pub unsafe fn DSA_SetItem(hdsa: *mut _DSA, i: i32, pitem: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_SetItem(hdsa : *mut _DSA, i : i32, pitem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DSA_SetItem(hdsa as _, i, pitem) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn DSA_Sort(pdsa: *mut _DSA, pfncompare: PFNDACOMPARE, lparam: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_Sort(pdsa : *mut _DSA, pfncompare : PFNDACOMPARE, lparam : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { DSA_Sort(pdsa as _, pfncompare, lparam) }
}
#[inline]
pub unsafe fn Str_SetPtrW<P1>(ppsz: *mut windows_core::PWSTR, psz: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn Str_SetPtrW(ppsz : *mut windows_core::PWSTR, psz : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { Str_SetPtrW(ppsz as _, psz.param().abi()) }
}
pub const DA_ERR: i32 = -1;
pub const DA_LAST: u32 = 2147483647;
pub const DPAMM_DELETE: u32 = 2;
pub const DPAMM_INSERT: u32 = 3;
pub const DPAMM_MERGE: u32 = 1;
pub const DPAM_INTERSECT: u32 = 8;
pub const DPAM_NORMAL: u32 = 2;
pub const DPAM_SORTED: u32 = 1;
pub const DPAM_UNION: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DPASTREAMINFO {
    pub iPos: i32,
    pub pvItem: *mut core::ffi::c_void,
}
impl Default for DPASTREAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DPAS_INSERTAFTER: u32 = 4;
pub const DPAS_INSERTBEFORE: u32 = 2;
pub const DPAS_SORTED: u32 = 1;
pub const DPA_APPEND: u32 = 2147483647;
pub const DPA_ERR: i32 = -1;
pub const DSA_APPEND: u32 = 2147483647;
pub const DSA_ERR: i32 = -1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDPA(pub *mut _DPA);
impl HDPA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDSA(pub *mut _DSA);
impl HDSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
pub type PFNDACOMPARE = Option<unsafe extern "system" fn(p1: *const core::ffi::c_void, p2: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PFNDACOMPARECONST = Option<unsafe extern "system" fn(p1: *const core::ffi::c_void, p2: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> i32>;
pub type PFNDAENUMCALLBACK = Option<unsafe extern "system" fn(p: *const core::ffi::c_void, pdata: *const core::ffi::c_void) -> i32>;
pub type PFNDAENUMCALLBACKCONST = Option<unsafe extern "system" fn(p: *const core::ffi::c_void, pdata: *const core::ffi::c_void) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PFNDPAMERGE = Option<unsafe extern "system" fn(umsg: u32, pvdest: *const core::ffi::c_void, pvsrc: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_minwindef")]
pub type PFNDPAMERGECONST = Option<unsafe extern "system" fn(umsg: u32, pvdest: *const core::ffi::c_void, pvsrc: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> *const core::ffi::c_void>;
#[cfg(feature = "Win32_objidlbase")]
pub type PFNDPASTREAM = Option<unsafe extern "system" fn(pinfo: *const DPASTREAMINFO, pstream: windows_core::Ref<super::objidlbase::IStream>, pvinstdata: *const core::ffi::c_void) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _DPA(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _DSA(pub u8);
