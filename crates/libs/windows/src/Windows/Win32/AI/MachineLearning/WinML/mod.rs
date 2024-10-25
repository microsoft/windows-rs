pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = 4i32;
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = 3i32;
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = 5i32;
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = 2i32;
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = 1i32;
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = 0i32;
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = 4i32;
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = 3i32;
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = 2i32;
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = 1i32;
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = 0i32;
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = 0i32;
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = 9i32;
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = 15i32;
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = 14i32;
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = 11i32;
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = 1i32;
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = 10i32;
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = 5i32;
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = 6i32;
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = 7i32;
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = 3i32;
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = 8i32;
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = 4i32;
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = 12i32;
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = 13i32;
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = 2i32;
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
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
impl windows_core::TypeKind for MLOperatorAttributeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: Self = Self(0u32);
    pub const Tensor: Self = Self(1u32);
}
impl windows_core::TypeKind for MLOperatorEdgeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: Self = Self(0u32);
    pub const Cpu: Self = Self(1u32);
    pub const D3D12: Self = Self(2u32);
}
impl windows_core::TypeKind for MLOperatorExecutionType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: Self = Self(0u32);
    pub const AllowDynamicInputShapes: Self = Self(1u32);
}
impl windows_core::TypeKind for MLOperatorKernelOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: Self = Self(0u32);
    pub const Optional: Self = Self(1u32);
    pub const Variadic: Self = Self(2u32);
}
impl windows_core::TypeKind for MLOperatorParameterOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: Self = Self(0i32);
    pub const Label: Self = Self(1i32);
}
impl windows_core::TypeKind for MLOperatorSchemaEdgeTypeFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
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
impl windows_core::TypeKind for MLOperatorTensorDataType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINML_BINDING_TYPE(pub i32);
impl windows_core::TypeKind for WINML_BINDING_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINML_FEATURE_TYPE(pub i32);
impl windows_core::TypeKind for WINML_FEATURE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINML_RUNTIME_TYPE(pub i32);
impl windows_core::TypeKind for WINML_RUNTIME_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WINML_TENSOR_DATA_TYPE(pub i32);
impl windows_core::TypeKind for WINML_TENSOR_DATA_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorAttribute {
    pub name: windows_core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: u8,
}
impl Default for MLOperatorAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorAttribute {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorAttributeNameValue {
    pub name: windows_core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
impl Default for MLOperatorAttributeNameValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorAttributeNameValue {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *const core::ffi::c_void,
    pub ints: *const i64,
    pub strings: *const *const i8,
    pub floats: *const f32,
}
impl Default for MLOperatorAttributeNameValue_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorAttributeNameValue_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl Default for MLOperatorEdgeDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorEdgeDescription {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl Default for MLOperatorEdgeDescription_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorEdgeDescription_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: windows_core::PCSTR,
    pub allowedTypes: *const MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
impl Default for MLOperatorEdgeTypeConstraint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorEdgeTypeConstraint {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorKernelDescription {
    pub domain: windows_core::PCSTR,
    pub name: windows_core::PCSTR,
    pub minimumOperatorSetVersion: i32,
    pub executionType: MLOperatorExecutionType,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
    pub options: MLOperatorKernelOptions,
    pub executionOptions: u32,
}
impl Default for MLOperatorKernelDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorKernelDescription {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorSchemaDescription {
    pub name: windows_core::PCSTR,
    pub operatorSetVersionAtLastChange: i32,
    pub inputs: *const MLOperatorSchemaEdgeDescription,
    pub inputCount: u32,
    pub outputs: *const MLOperatorSchemaEdgeDescription,
    pub outputCount: u32,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub attributes: *const MLOperatorAttribute,
    pub attributeCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
}
impl Default for MLOperatorSchemaDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorSchemaDescription {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
impl Default for MLOperatorSchemaEdgeDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorSchemaEdgeDescription {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *const core::ffi::c_void,
    pub typeLabel: windows_core::PCSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
impl Default for MLOperatorSchemaEdgeDescription_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorSchemaEdgeDescription_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorSetId {
    pub domain: windows_core::PCSTR,
    pub version: i32,
}
impl Default for MLOperatorSetId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MLOperatorSetId {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_BINDING_DESC {
    pub Name: windows_core::PCWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Default for WINML_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::TypeKind for WINML_BINDING_DESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union WINML_BINDING_DESC_0 {
    pub Tensor: WINML_TENSOR_BINDING_DESC,
    pub Sequence: WINML_SEQUENCE_BINDING_DESC,
    pub Map: WINML_MAP_BINDING_DESC,
    pub Image: WINML_IMAGE_BINDING_DESC,
    pub Resource: WINML_RESOURCE_BINDING_DESC,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Default for WINML_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::TypeKind for WINML_BINDING_DESC_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut core::ffi::c_void,
}
impl Default for WINML_IMAGE_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_IMAGE_BINDING_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl Default for WINML_IMAGE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_IMAGE_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_MAP_BINDING_DESC {
    pub ElementCount: u32,
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous1: WINML_MAP_BINDING_DESC_0,
    pub Fields: WINML_TENSOR_DATA_TYPE,
    pub Anonymous2: WINML_MAP_BINDING_DESC_1,
}
impl Default for WINML_MAP_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_MAP_BINDING_DESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut windows_core::PWSTR,
    pub pIntKeys: *mut i64,
}
impl Default for WINML_MAP_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_MAP_BINDING_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union WINML_MAP_BINDING_DESC_1 {
    pub pStringFields: *mut windows_core::PWSTR,
    pub pIntFields: *mut i64,
    pub pFloatFields: *mut f32,
    pub pDoubleFields: *mut f64,
}
impl Default for WINML_MAP_BINDING_DESC_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_MAP_BINDING_DESC_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
impl Default for WINML_MAP_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_MAP_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_MODEL_DESC {
    pub Author: windows_core::PWSTR,
    pub Name: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub Description: windows_core::PWSTR,
    pub Version: usize,
}
impl Default for WINML_MODEL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_MODEL_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Default for WINML_RESOURCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::TypeKind for WINML_RESOURCE_BINDING_DESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
impl Default for WINML_SEQUENCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_SEQUENCE_BINDING_DESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union WINML_SEQUENCE_BINDING_DESC_0 {
    pub pStrings: *mut windows_core::PWSTR,
    pub pInts: *mut i64,
    pub pFloats: *mut f32,
    pub pDoubles: *mut f64,
}
impl Default for WINML_SEQUENCE_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_SEQUENCE_BINDING_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
impl Default for WINML_SEQUENCE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_SEQUENCE_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut core::ffi::c_void,
}
impl Default for WINML_TENSOR_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_TENSOR_BINDING_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl Default for WINML_TENSOR_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_TENSOR_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_VARIABLE_DESC {
    pub Name: windows_core::PWSTR,
    pub Description: windows_core::PWSTR,
    pub FeatureType: WINML_FEATURE_TYPE,
    pub Required: super::super::super::Foundation::BOOL,
    pub Anonymous: WINML_VARIABLE_DESC_0,
}
impl Default for WINML_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_VARIABLE_DESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
impl Default for WINML_VARIABLE_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WINML_VARIABLE_DESC_0 {
    type TypeKind = windows_core::CloneType;
}
