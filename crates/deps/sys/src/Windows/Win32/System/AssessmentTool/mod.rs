#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CAccessiblityWinSAT(i32);
#[repr(C)]
pub struct CInitiateWinSAT(i32);
#[repr(C)]
pub struct CProvideWinSATVisuals(i32);
#[repr(C)]
pub struct CQueryAllWinSAT(i32);
#[repr(C)]
pub struct CQueryOEMWinSATCustomization(i32);
#[repr(C)]
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
#[repr(transparent)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(1i32);
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(2i32);
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(3i32);
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
#[repr(transparent)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
#[repr(transparent)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
#[repr(transparent)]
pub struct WINSAT_OEM_DATA_TYPE(pub i32);
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(0i32);
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(1i32);
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(2i32);
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(3i32);
