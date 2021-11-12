#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ILocalCategoriesStatics(pub *mut ::core::ffi::c_void);
pub struct ILocalLocation(pub *mut ::core::ffi::c_void);
pub struct ILocalLocation2(pub *mut ::core::ffi::c_void);
pub struct ILocalLocationFinderResult(pub *mut ::core::ffi::c_void);
pub struct ILocalLocationFinderStatics(pub *mut ::core::ffi::c_void);
pub struct ILocalLocationHoursOfOperationItem(pub *mut ::core::ffi::c_void);
pub struct ILocalLocationRatingInfo(pub *mut ::core::ffi::c_void);
pub struct IPlaceInfoHelperStatics(pub *mut ::core::ffi::c_void);
pub struct LocalCategories(i32);
pub struct LocalLocation(i32);
pub struct LocalLocationFinder(i32);
pub struct LocalLocationFinderResult(i32);
pub struct LocalLocationFinderStatus(i32);
pub struct LocalLocationHoursOfOperationItem(i32);
pub struct LocalLocationRatingInfo(i32);
pub struct PlaceInfoHelper(i32);
