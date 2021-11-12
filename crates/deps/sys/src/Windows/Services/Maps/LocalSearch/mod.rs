#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILocalCategoriesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalLocation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalLocationFinderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalLocationFinderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalLocationHoursOfOperationItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalLocationRatingInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaceInfoHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalCategories(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalLocationFinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalLocationFinderResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LocalLocationFinderStatus(i32);
#[repr(transparent)]
pub struct LocalLocationHoursOfOperationItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalLocationRatingInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaceInfoHelper(pub *mut ::core::ffi::c_void);
