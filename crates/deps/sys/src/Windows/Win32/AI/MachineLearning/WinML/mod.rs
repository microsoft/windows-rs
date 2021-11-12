#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(C)]
pub struct MLOperatorAttribute(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MLOperatorAttributeNameValue(i32);
#[repr(transparent)]
pub struct MLOperatorAttributeType(pub u32);
impl MLOperatorAttributeType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(2u32);
    pub const Int: Self = Self(3u32);
    pub const String: Self = Self(4u32);
    pub const FloatArray: Self = Self(7u32);
    pub const IntArray: Self = Self(8u32);
    pub const StringArray: Self = Self(9u32);
}
#[repr(C)]
pub struct MLOperatorEdgeDescription(i32);
#[repr(transparent)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: Self = Self(0u32);
    pub const Tensor: Self = Self(1u32);
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MLOperatorEdgeTypeConstraint(i32);
#[repr(transparent)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: Self = Self(0u32);
    pub const Cpu: Self = Self(1u32);
    pub const D3D12: Self = Self(2u32);
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MLOperatorKernelDescription(i32);
#[repr(transparent)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: Self = Self(0u32);
    pub const AllowDynamicInputShapes: Self = Self(1u32);
}
#[repr(transparent)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: Self = Self(0u32);
    pub const Optional: Self = Self(1u32);
    pub const Variadic: Self = Self(2u32);
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MLOperatorSchemaDescription(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MLOperatorSchemaEdgeDescription(i32);
#[repr(transparent)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: Self = Self(0i32);
    pub const Label: Self = Self(1i32);
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MLOperatorSetId(i32);
#[repr(transparent)]
pub struct MLOperatorTensorDataType(pub u32);
impl MLOperatorTensorDataType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(1u32);
    pub const UInt8: Self = Self(2u32);
    pub const Int8: Self = Self(3u32);
    pub const UInt16: Self = Self(4u32);
    pub const Int16: Self = Self(5u32);
    pub const Int32: Self = Self(6u32);
    pub const Int64: Self = Self(7u32);
    pub const String: Self = Self(8u32);
    pub const Bool: Self = Self(9u32);
    pub const Float16: Self = Self(10u32);
    pub const Double: Self = Self(11u32);
    pub const UInt32: Self = Self(12u32);
    pub const UInt64: Self = Self(13u32);
    pub const Complex64: Self = Self(14u32);
    pub const Complex128: Self = Self(15u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
#[repr(C)]
pub struct WINML_BINDING_DESC(i32);
#[repr(transparent)]
pub struct WINML_BINDING_TYPE(pub i32);
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = WINML_BINDING_TYPE(0i32);
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = WINML_BINDING_TYPE(1i32);
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(2i32);
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = WINML_BINDING_TYPE(3i32);
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(4i32);
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(5i32);
#[repr(transparent)]
pub struct WINML_FEATURE_TYPE(pub i32);
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(0i32);
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(1i32);
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(2i32);
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(3i32);
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(4i32);
#[repr(C)]
pub struct WINML_IMAGE_BINDING_DESC(i32);
#[repr(C)]
pub struct WINML_IMAGE_VARIABLE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINML_MAP_BINDING_DESC(i32);
#[repr(C)]
pub struct WINML_MAP_VARIABLE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINML_MODEL_DESC(i32);
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[repr(C)]
pub struct WINML_RESOURCE_BINDING_DESC(i32);
#[repr(transparent)]
pub struct WINML_RUNTIME_TYPE(pub i32);
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = WINML_RUNTIME_TYPE(0i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINML_SEQUENCE_BINDING_DESC(i32);
#[repr(C)]
pub struct WINML_SEQUENCE_VARIABLE_DESC(i32);
#[repr(C)]
pub struct WINML_TENSOR_BINDING_DESC(i32);
#[repr(transparent)]
pub struct WINML_TENSOR_DATA_TYPE(pub i32);
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(0i32);
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(1i32);
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(2i32);
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(3i32);
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(4i32);
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(5i32);
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(6i32);
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(7i32);
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(8i32);
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(9i32);
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(10i32);
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(11i32);
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(12i32);
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(13i32);
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(14i32);
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(15i32);
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[repr(C)]
pub struct WINML_TENSOR_VARIABLE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINML_VARIABLE_DESC(i32);
