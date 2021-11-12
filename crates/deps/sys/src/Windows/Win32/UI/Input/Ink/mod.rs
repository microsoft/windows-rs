#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInkCommitRequestHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkD2DRenderer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkD2DRenderer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDesktopHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkHostWorkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterDesktop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
impl ::core::marker::Copy for INK_HIGH_CONTRAST_ADJUSTMENT {}
impl ::core::clone::Clone for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkD2DRenderer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1078257164, data2: 31489, data3: 18033, data4: [169, 124, 4, 224, 33, 10, 7, 165] };
pub const InkDesktopHost: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 103122086, data2: 63536, data3: 19420, data4: [164, 210, 10, 16, 171, 6, 43, 29] };
