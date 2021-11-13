#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILocalCategoriesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalCategoriesStatics {}
impl ::core::clone::Clone for ILocalCategoriesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocalLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalLocation {}
impl ::core::clone::Clone for ILocalLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocalLocation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalLocation2 {}
impl ::core::clone::Clone for ILocalLocation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocalLocationFinderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalLocationFinderResult {}
impl ::core::clone::Clone for ILocalLocationFinderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocalLocationFinderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalLocationFinderStatics {}
impl ::core::clone::Clone for ILocalLocationFinderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocalLocationHoursOfOperationItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalLocationHoursOfOperationItem {}
impl ::core::clone::Clone for ILocalLocationHoursOfOperationItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocalLocationRatingInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocalLocationRatingInfo {}
impl ::core::clone::Clone for ILocalLocationRatingInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaceInfoHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaceInfoHelperStatics {}
impl ::core::clone::Clone for IPlaceInfoHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocalLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LocalLocation {}
impl ::core::clone::Clone for LocalLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocalLocationFinderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LocalLocationFinderResult {}
impl ::core::clone::Clone for LocalLocationFinderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const InvalidCategory: Self = Self(3i32);
    pub const InvalidSearchTerm: Self = Self(4i32);
    pub const InvalidSearchArea: Self = Self(5i32);
    pub const NetworkFailure: Self = Self(6i32);
    pub const NotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for LocalLocationFinderStatus {}
impl ::core::clone::Clone for LocalLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocalLocationHoursOfOperationItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LocalLocationHoursOfOperationItem {}
impl ::core::clone::Clone for LocalLocationHoursOfOperationItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocalLocationRatingInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LocalLocationRatingInfo {}
impl ::core::clone::Clone for LocalLocationRatingInfo {
    fn clone(&self) -> Self {
        *self
    }
}
