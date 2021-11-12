#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn MLCreateOperatorRegistry(registry: *mut IMLOperatorRegistry) -> ::windows_sys::core::HRESULT;
    pub fn WinMLCreateRuntime(runtime: *mut IWinMLRuntime) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IMLOperatorAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorKernel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorKernelContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorKernelCreationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorKernelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorRegistry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorShapeInferenceContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorShapeInferrer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorTensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorTensorShapeDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorTypeInferenceContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLOperatorTypeInferrer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWinMLEvaluationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWinMLModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWinMLRuntime(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWinMLRuntimeFactory(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttribute(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttributeNameValue(i32);
pub struct MLOperatorAttributeType(i32);
pub struct MLOperatorEdgeDescription(i32);
pub struct MLOperatorEdgeType(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorEdgeTypeConstraint(i32);
pub struct MLOperatorExecutionType(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorKernelDescription(i32);
pub struct MLOperatorKernelOptions(i32);
pub struct MLOperatorParameterOptions(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSchemaDescription(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSchemaEdgeDescription(i32);
pub struct MLOperatorSchemaEdgeTypeFormat(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSetId(i32);
pub struct MLOperatorTensorDataType(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct WINML_BINDING_DESC(i32);
pub struct WINML_BINDING_TYPE(i32);
pub struct WINML_FEATURE_TYPE(i32);
pub struct WINML_IMAGE_BINDING_DESC(i32);
pub struct WINML_IMAGE_VARIABLE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_MAP_BINDING_DESC(i32);
pub struct WINML_MAP_VARIABLE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_MODEL_DESC(i32);
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_RESOURCE_BINDING_DESC(i32);
pub struct WINML_RUNTIME_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_SEQUENCE_BINDING_DESC(i32);
pub struct WINML_SEQUENCE_VARIABLE_DESC(i32);
pub struct WINML_TENSOR_BINDING_DESC(i32);
pub struct WINML_TENSOR_DATA_TYPE(i32);
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
pub struct WINML_TENSOR_VARIABLE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_VARIABLE_DESC(i32);
