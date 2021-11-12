#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AttributeTargets(i32);
#[repr(C)]
pub struct CompositionType(i32);
#[repr(C)]
pub struct DeprecationType(i32);
#[repr(C)]
pub struct FeatureStage(i32);
#[repr(C)]
pub struct GCPressureAmount(i32);
#[repr(transparent)]
pub struct IApiInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MarshalingType(i32);
#[repr(C)]
pub struct Platform(i32);
#[repr(C)]
pub struct ThreadingModel(i32);
