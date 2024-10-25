pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = 1i32;
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = 3i32;
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = 2i32;
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = 4i32;
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = 0i32;
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = 2i32;
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = 4i32;
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = 4i32;
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = 0i32;
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = 3i32;
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = 0i32;
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = 1i32;
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = 1i32;
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = 0i32;
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_CUSTOMIZATION_STATE = 2i32;
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_CUSTOMIZATION_STATE = 1i32;
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_CUSTOMIZATION_STATE = 0i32;
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_CUSTOMIZATION_STATE = 3i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
impl windows_core::TypeKind for WINSAT_ASSESSMENT_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
impl windows_core::TypeKind for WINSAT_ASSESSMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
impl windows_core::TypeKind for WINSAT_BITMAP_SIZE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINSAT_OEM_CUSTOMIZATION_STATE(pub i32);
impl windows_core::TypeKind for WINSAT_OEM_CUSTOMIZATION_STATE {
    type TypeKind = windows_core::CopyType;
}
pub const CAccessiblityWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0x6e18f9c6_a3eb_495a_89b7_956482e19f7a);
pub const CInitiateWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0x489331dc_f5e0_4528_9fda_45331bf4a571);
pub const CProvideWinSATVisuals: windows_core::GUID = windows_core::GUID::from_u128(0x9f377d7e_e551_44f8_9f94_9db392b03b7b);
pub const CQueryAllWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0x05df8d13_c355_47f4_a11e_851b338cefb8);
pub const CQueryOEMWinSATCustomization: windows_core::GUID = windows_core::GUID::from_u128(0xc47a41b7_b729_424f_9af9_5cb3934f2dfa);
pub const CQueryWinSAT: windows_core::GUID = windows_core::GUID::from_u128(0xf3bdfad3_f276_49e9_9b17_c474f48f0764);
