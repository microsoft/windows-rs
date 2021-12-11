#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub fn MLCreateOperatorRegistry(registry: *mut IMLOperatorRegistry) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
    pub fn WinMLCreateRuntime(runtime: *mut IWinMLRuntime) -> ::windows_sys::core::HRESULT;
}
pub type IMLOperatorAttributes = *mut ::core::ffi::c_void;
pub type IMLOperatorKernel = *mut ::core::ffi::c_void;
pub type IMLOperatorKernelContext = *mut ::core::ffi::c_void;
pub type IMLOperatorKernelCreationContext = *mut ::core::ffi::c_void;
pub type IMLOperatorKernelFactory = *mut ::core::ffi::c_void;
pub type IMLOperatorRegistry = *mut ::core::ffi::c_void;
pub type IMLOperatorShapeInferenceContext = *mut ::core::ffi::c_void;
pub type IMLOperatorShapeInferrer = *mut ::core::ffi::c_void;
pub type IMLOperatorTensor = *mut ::core::ffi::c_void;
pub type IMLOperatorTensorShapeDescription = *mut ::core::ffi::c_void;
pub type IMLOperatorTypeInferenceContext = *mut ::core::ffi::c_void;
pub type IMLOperatorTypeInferrer = *mut ::core::ffi::c_void;
pub type IWinMLEvaluationContext = *mut ::core::ffi::c_void;
pub type IWinMLModel = *mut ::core::ffi::c_void;
pub type IWinMLRuntime = *mut ::core::ffi::c_void;
pub type IWinMLRuntimeFactory = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttribute {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: bool,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorAttribute {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorAttributeNameValue {
    pub name: super::super::super::Foundation::PSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorAttributeNameValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorAttributeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *mut ::core::ffi::c_void,
    pub ints: *mut i64,
    pub strings: *mut *mut i8,
    pub floats: *mut f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorAttributeNameValue_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorAttributeNameValue_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
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
impl ::core::marker::Copy for MLOperatorAttributeType {}
impl ::core::clone::Clone for MLOperatorAttributeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription {}
impl ::core::clone::Clone for MLOperatorEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription_0 {}
impl ::core::clone::Clone for MLOperatorEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: Self = Self(0u32);
    pub const Tensor: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorEdgeType {}
impl ::core::clone::Clone for MLOperatorEdgeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub allowedTypes: *mut MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorEdgeTypeConstraint {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorEdgeTypeConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: Self = Self(0u32);
    pub const Cpu: Self = Self(1u32);
    pub const D3D12: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorExecutionType {}
impl ::core::clone::Clone for MLOperatorExecutionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorKernelDescription {
    pub domain: super::super::super::Foundation::PSTR,
    pub name: super::super::super::Foundation::PSTR,
    pub minimumOperatorSetVersion: i32,
    pub executionType: MLOperatorExecutionType,
    pub typeConstraints: *mut MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub defaultAttributes: *mut MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
    pub options: MLOperatorKernelOptions,
    pub executionOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorKernelDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorKernelDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: Self = Self(0u32);
    pub const AllowDynamicInputShapes: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorKernelOptions {}
impl ::core::clone::Clone for MLOperatorKernelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: Self = Self(0u32);
    pub const Optional: Self = Self(1u32);
    pub const Variadic: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorParameterOptions {}
impl ::core::clone::Clone for MLOperatorParameterOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSchemaDescription {
    pub name: super::super::super::Foundation::PSTR,
    pub operatorSetVersionAtLastChange: i32,
    pub inputs: *mut MLOperatorSchemaEdgeDescription,
    pub inputCount: u32,
    pub outputs: *mut MLOperatorSchemaEdgeDescription,
    pub outputCount: u32,
    pub typeConstraints: *mut MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub attributes: *mut MLOperatorAttribute,
    pub attributeCount: u32,
    pub defaultAttributes: *mut MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorSchemaDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSchemaDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *mut ::core::ffi::c_void,
    pub typeLabel: super::super::super::Foundation::PSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
#[repr(transparent)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: Self = Self(0i32);
    pub const Label: Self = Self(1i32);
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeTypeFormat {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeTypeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MLOperatorSetId {
    pub domain: super::super::super::Foundation::PSTR,
    pub version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MLOperatorSetId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MLOperatorSetId {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
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
impl ::core::marker::Copy for MLOperatorTensorDataType {}
impl ::core::clone::Clone for MLOperatorTensorDataType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation', 'Win32_Graphics_Direct3D12'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct WINML_BINDING_DESC {
    pub Name: super::super::super::Foundation::PWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::marker::Copy for WINML_BINDING_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for WINML_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation', 'Win32_Graphics_Direct3D12'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub union WINML_BINDING_DESC_0 {
    pub Tensor: WINML_TENSOR_BINDING_DESC,
    pub Sequence: WINML_SEQUENCE_BINDING_DESC,
    pub Map: WINML_MAP_BINDING_DESC,
    pub Image: WINML_IMAGE_BINDING_DESC,
    pub Resource: WINML_RESOURCE_BINDING_DESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::marker::Copy for WINML_BINDING_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for WINML_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_BINDING_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = 0i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = 1i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = 2i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = 3i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = 4i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = 5i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_FEATURE_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_IMAGE_BINDING_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_IMAGE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_MAP_BINDING_DESC {
    pub ElementCount: u32,
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous1: WINML_MAP_BINDING_DESC_0,
    pub Fields: WINML_TENSOR_DATA_TYPE,
    pub Anonymous2: WINML_MAP_BINDING_DESC_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut super::super::super::Foundation::PWSTR,
    pub pIntKeys: *mut i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_MAP_BINDING_DESC_1 {
    pub pStringFields: *mut super::super::super::Foundation::PWSTR,
    pub pIntFields: *mut i64,
    pub pFloatFields: *mut f32,
    pub pDoubleFields: *mut f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_MAP_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_MAP_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_MODEL_DESC {
    pub Author: super::super::super::Foundation::PWSTR,
    pub Name: super::super::super::Foundation::PWSTR,
    pub Domain: super::super::super::Foundation::PWSTR,
    pub Description: super::super::super::Foundation::PWSTR,
    pub Version: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_MODEL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_MODEL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Graphics_Direct3D12'*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: super::super::super::Graphics::Direct3D12::ID3D12Resource,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for WINML_RESOURCE_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_RESOURCE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_RUNTIME_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_SEQUENCE_BINDING_DESC_0 {
    pub pStrings: *mut super::super::super::Foundation::PWSTR,
    pub pInts: *mut i64,
    pub pFloats: *mut f32,
    pub pDoubles: *mut f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_SEQUENCE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_TENSOR_BINDING_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub type WINML_TENSOR_DATA_TYPE = i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = 0i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = 1i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = 2i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = 3i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = 4i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = 5i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = 6i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = 7i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = 8i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = 9i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = 10i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = 11i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = 12i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = 13i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = 14i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = 15i32;
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML'*"]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_TENSOR_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_VARIABLE_DESC {
    pub Name: super::super::super::Foundation::PWSTR,
    pub Description: super::super::super::Foundation::PWSTR,
    pub FeatureType: WINML_FEATURE_TYPE,
    pub Required: super::super::super::Foundation::BOOL,
    pub Anonymous: WINML_VARIABLE_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_AI_MachineLearning_WinML', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
