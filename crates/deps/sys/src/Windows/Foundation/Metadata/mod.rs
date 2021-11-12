#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ApiInformation(pub *mut ::core::ffi::c_void);
pub struct AttributeTargets(i32);
pub struct CompositionType(i32);
pub struct DeprecationType(i32);
pub struct FeatureStage(i32);
pub struct GCPressureAmount(i32);
#[repr(transparent)]
pub struct IApiInformationStatics(pub *mut ::core::ffi::c_void);
pub struct MarshalingType(i32);
pub struct Platform(i32);
pub struct ThreadingModel(i32);
