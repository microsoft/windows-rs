#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const CAccessiblityWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1847130566,
    data2: 41963,
    data3: 18778,
    data4: [137, 183, 149, 100, 130, 225, 159, 122],
};
pub const CInitiateWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1217606108,
    data2: 62944,
    data3: 17704,
    data4: [159, 218, 69, 51, 27, 244, 165, 113],
};
pub const CProvideWinSATVisuals: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2671213950,
    data2: 58705,
    data3: 17656,
    data4: [159, 148, 157, 179, 146, 176, 59, 123],
};
pub const CQueryAllWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 98536723, data2: 50005, data3: 18420, data4: [161, 30, 133, 27, 51, 140, 239, 184] };
pub const CQueryOEMWinSATCustomization: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3296346551,
    data2: 46889,
    data3: 16975,
    data4: [154, 249, 92, 179, 147, 79, 45, 250],
};
pub const CQueryWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4089314003,
    data2: 62070,
    data3: 18921,
    data4: [155, 23, 196, 116, 244, 143, 7, 100],
};
#[repr(transparent)]
pub struct IAccessibleWinSAT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccessibleWinSAT {}
impl ::core::clone::Clone for IAccessibleWinSAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInitiateWinSATAssessment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInitiateWinSATAssessment {}
impl ::core::clone::Clone for IInitiateWinSATAssessment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvideWinSATAssessmentInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvideWinSATAssessmentInfo {}
impl ::core::clone::Clone for IProvideWinSATAssessmentInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvideWinSATResultsInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvideWinSATResultsInfo {}
impl ::core::clone::Clone for IProvideWinSATResultsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvideWinSATVisuals(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvideWinSATVisuals {}
impl ::core::clone::Clone for IProvideWinSATVisuals {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQueryAllWinSATAssessments(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQueryAllWinSATAssessments {}
impl ::core::clone::Clone for IQueryAllWinSATAssessments {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQueryOEMWinSATCustomization(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQueryOEMWinSATCustomization {}
impl ::core::clone::Clone for IQueryOEMWinSATCustomization {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQueryRecentWinSATAssessment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQueryRecentWinSATAssessment {}
impl ::core::clone::Clone for IQueryRecentWinSATAssessment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWinSATInitiateEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWinSATInitiateEvents {}
impl ::core::clone::Clone for IWinSATInitiateEvents {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WINSAT_ASSESSMENT_STATE_MIN: i32 = 0i32;
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: i32 = 0i32;
pub const WINSAT_ASSESSMENT_STATE_VALID: i32 = 1i32;
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: i32 = 2i32;
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: i32 = 3i32;
pub const WINSAT_ASSESSMENT_STATE_INVALID: i32 = 4i32;
pub const WINSAT_ASSESSMENT_STATE_MAX: i32 = 4i32;
pub const WINSAT_ASSESSMENT_MEMORY: i32 = 0i32;
pub const WINSAT_ASSESSMENT_CPU: i32 = 1i32;
pub const WINSAT_ASSESSMENT_DISK: i32 = 2i32;
pub const WINSAT_ASSESSMENT_D3D: i32 = 3i32;
pub const WINSAT_ASSESSMENT_GRAPHICS: i32 = 4i32;
pub const WINSAT_BITMAP_SIZE_SMALL: i32 = 0i32;
pub const WINSAT_BITMAP_SIZE_NORMAL: i32 = 1i32;
pub const WINSAT_OEM_DATA_VALID: i32 = 0i32;
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: i32 = 1i32;
pub const WINSAT_OEM_DATA_INVALID: i32 = 2i32;
pub const WINSAT_OEM_NO_DATA_SUPPLIED: i32 = 3i32;
