#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
pub struct LocalLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalLocationFinderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: LocalLocationFinderStatus = LocalLocationFinderStatus(0i32);
    pub const UnknownError: LocalLocationFinderStatus = LocalLocationFinderStatus(1i32);
    pub const InvalidCredentials: LocalLocationFinderStatus = LocalLocationFinderStatus(2i32);
    pub const InvalidCategory: LocalLocationFinderStatus = LocalLocationFinderStatus(3i32);
    pub const InvalidSearchTerm: LocalLocationFinderStatus = LocalLocationFinderStatus(4i32);
    pub const InvalidSearchArea: LocalLocationFinderStatus = LocalLocationFinderStatus(5i32);
    pub const NetworkFailure: LocalLocationFinderStatus = LocalLocationFinderStatus(6i32);
    pub const NotSupported: LocalLocationFinderStatus = LocalLocationFinderStatus(7i32);
}
#[repr(transparent)]
pub struct LocalLocationHoursOfOperationItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalLocationRatingInfo(pub *mut ::core::ffi::c_void);
