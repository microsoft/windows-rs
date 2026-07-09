windows_link::link!("comctl32.dll" "system" fn DPA_Clone(hdpa : *const _DPA, hdpanew : *mut _DPA) -> HDPA);
windows_link::link!("comctl32.dll" "system" fn DPA_Create(citemgrow : i32) -> HDPA);
#[cfg(feature = "winnt")]
windows_link::link!("comctl32.dll" "system" fn DPA_CreateEx(cpgrow : i32, hheap : super::winnt::HANDLE) -> HDPA);
windows_link::link!("comctl32.dll" "system" fn DPA_DeleteAllPtrs(hdpa : *mut _DPA) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DPA_DeletePtr(hdpa : *mut _DPA, i : i32) -> *mut core::ffi::c_void);
windows_link::link!("comctl32.dll" "system" fn DPA_Destroy(hdpa : *mut _DPA) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DPA_DestroyCallback(hdpa : *mut _DPA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
windows_link::link!("comctl32.dll" "system" fn DPA_EnumCallback(hdpa : *const _DPA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
windows_link::link!("comctl32.dll" "system" fn DPA_GetPtr(hdpa : *const _DPA, i : isize) -> *mut core::ffi::c_void);
windows_link::link!("comctl32.dll" "system" fn DPA_GetPtrIndex(hdpa : *const _DPA, p : *const core::ffi::c_void) -> i32);
windows_link::link!("comctl32.dll" "system" fn DPA_GetSize(hdpa : *const _DPA) -> u64);
windows_link::link!("comctl32.dll" "system" fn DPA_Grow(pdpa : *mut _DPA, cp : i32) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DPA_InsertPtr(hdpa : *mut _DPA, i : i32, p : *const core::ffi::c_void) -> i32);
#[cfg(feature = "objidlbase")]
windows_link::link!("comctl32.dll" "system" fn DPA_LoadStream(phdpa : *mut HDPA, pfn : PFNDPASTREAM, pstream : *mut core::ffi::c_void, pvinstdata : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("comctl32.dll" "system" fn DPA_Merge(hdpadest : *mut _DPA, hdpasrc : *const _DPA, dwflags : u32, pfncompare : PFNDACOMPARE, pfnmerge : PFNDPAMERGE, lparam : super::minwindef::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "objidlbase")]
windows_link::link!("comctl32.dll" "system" fn DPA_SaveStream(hdpa : *const _DPA, pfn : PFNDPASTREAM, pstream : *mut core::ffi::c_void, pvinstdata : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("comctl32.dll" "system" fn DPA_Search(hdpa : *const _DPA, pfind : *const core::ffi::c_void, istart : i32, pfncompare : PFNDACOMPARE, lparam : super::minwindef::LPARAM, options : u32) -> i32);
windows_link::link!("comctl32.dll" "system" fn DPA_SetPtr(hdpa : *mut _DPA, i : i32, p : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("comctl32.dll" "system" fn DPA_Sort(hdpa : *mut _DPA, pfncompare : PFNDACOMPARE, lparam : super::minwindef::LPARAM) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DSA_Clone(hdsa : *const _DSA) -> HDSA);
windows_link::link!("comctl32.dll" "system" fn DSA_Create(cbitem : i32, citemgrow : i32) -> HDSA);
windows_link::link!("comctl32.dll" "system" fn DSA_DeleteAllItems(hdsa : *mut _DSA) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DSA_DeleteItem(hdsa : *mut _DSA, i : i32) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DSA_Destroy(hdsa : *mut _DSA) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DSA_DestroyCallback(hdsa : *mut _DSA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
windows_link::link!("comctl32.dll" "system" fn DSA_EnumCallback(hdsa : *const _DSA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
windows_link::link!("comctl32.dll" "system" fn DSA_GetItem(hdsa : *const _DSA, i : i32, pitem : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn DSA_GetItemPtr(hdsa : *const _DSA, i : i32) -> *mut core::ffi::c_void);
windows_link::link!("comctl32.dll" "system" fn DSA_GetSize(hdsa : *const _DSA) -> u64);
windows_link::link!("comctl32.dll" "system" fn DSA_InsertItem(hdsa : *mut _DSA, i : i32, pitem : *const core::ffi::c_void) -> i32);
windows_link::link!("comctl32.dll" "system" fn DSA_SetItem(hdsa : *mut _DSA, i : i32, pitem : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("comctl32.dll" "system" fn DSA_Sort(pdsa : *mut _DSA, pfncompare : PFNDACOMPARE, lparam : super::minwindef::LPARAM) -> windows_sys::core::BOOL);
windows_link::link!("comctl32.dll" "system" fn Str_SetPtrW(ppsz : *mut windows_sys::core::PWSTR, psz : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
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
#[derive(Clone, Copy)]
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
pub type HDPA = *mut _DPA;
pub type HDSA = *mut _DSA;
#[cfg(feature = "minwindef")]
pub type PFNDACOMPARE = Option<unsafe extern "system" fn(p1: *const core::ffi::c_void, p2: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> i32>;
#[cfg(feature = "minwindef")]
pub type PFNDACOMPARECONST = Option<unsafe extern "system" fn(p1: *const core::ffi::c_void, p2: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> i32>;
pub type PFNDAENUMCALLBACK = Option<unsafe extern "system" fn(p: *const core::ffi::c_void, pdata: *const core::ffi::c_void) -> i32>;
pub type PFNDAENUMCALLBACKCONST = Option<unsafe extern "system" fn(p: *const core::ffi::c_void, pdata: *const core::ffi::c_void) -> i32>;
#[cfg(feature = "minwindef")]
pub type PFNDPAMERGE = Option<unsafe extern "system" fn(umsg: u32, pvdest: *const core::ffi::c_void, pvsrc: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> *mut core::ffi::c_void>;
#[cfg(feature = "minwindef")]
pub type PFNDPAMERGECONST = Option<unsafe extern "system" fn(umsg: u32, pvdest: *const core::ffi::c_void, pvsrc: *const core::ffi::c_void, lparam: super::minwindef::LPARAM) -> *const core::ffi::c_void>;
#[cfg(feature = "objidlbase")]
pub type PFNDPASTREAM = Option<unsafe extern "system" fn(pinfo: *const DPASTREAMINFO, pstream: *mut core::ffi::c_void, pvinstdata: *const core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _DPA(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _DSA(pub u8);
