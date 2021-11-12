#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const CAccessiblityWinSAT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1847130566,
    data2: 41963,
    data3: 18778,
    data4: [137, 183, 149, 100, 130, 225, 159, 122],
};
pub const CInitiateWinSAT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1217606108,
    data2: 62944,
    data3: 17704,
    data4: [159, 218, 69, 51, 27, 244, 165, 113],
};
pub const CProvideWinSATVisuals: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2671213950,
    data2: 58705,
    data3: 17656,
    data4: [159, 148, 157, 179, 146, 176, 59, 123],
};
pub const CQueryAllWinSAT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 98536723, data2: 50005, data3: 18420, data4: [161, 30, 133, 27, 51, 140, 239, 184] };
pub const CQueryOEMWinSATCustomization: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3296346551,
    data2: 46889,
    data3: 16975,
    data4: [154, 249, 92, 179, 147, 79, 45, 250],
};
pub const CQueryWinSAT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4089314003,
    data2: 62070,
    data3: 18921,
    data4: [155, 23, 196, 116, 244, 143, 7, 100],
};
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
impl ::core::marker::Copy for WINSAT_ASSESSMENT_STATE {}
impl ::core::clone::Clone for WINSAT_ASSESSMENT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
impl ::core::marker::Copy for WINSAT_ASSESSMENT_TYPE {}
impl ::core::clone::Clone for WINSAT_ASSESSMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
impl ::core::marker::Copy for WINSAT_BITMAP_SIZE {}
impl ::core::clone::Clone for WINSAT_BITMAP_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WINSAT_OEM_DATA_TYPE(pub i32);
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(0i32);
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(1i32);
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(2i32);
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(3i32);
impl ::core::marker::Copy for WINSAT_OEM_DATA_TYPE {}
impl ::core::clone::Clone for WINSAT_OEM_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
