#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CAccessiblityWinSAT(i32);
pub struct CInitiateWinSAT(i32);
pub struct CProvideWinSATVisuals(i32);
pub struct CQueryAllWinSAT(i32);
pub struct CQueryOEMWinSATCustomization(i32);
pub struct CQueryWinSAT(i32);
#[repr(transparent)]
pub struct IAccessibleWinSAT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitiateWinSATAssessment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideWinSATAssessmentInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideWinSATResultsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideWinSATVisuals(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryAllWinSATAssessments(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryOEMWinSATCustomization(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryRecentWinSATAssessment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWinSATInitiateEvents(pub *mut ::core::ffi::c_void);
pub struct WINSAT_ASSESSMENT_STATE(i32);
pub struct WINSAT_ASSESSMENT_TYPE(i32);
pub struct WINSAT_BITMAP_SIZE(i32);
pub struct WINSAT_OEM_DATA_TYPE(i32);
