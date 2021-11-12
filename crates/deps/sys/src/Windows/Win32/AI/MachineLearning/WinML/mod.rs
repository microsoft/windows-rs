#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub fn MLCreateOperatorRegistry(registry: *mut IMLOperatorRegistry) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub fn WinMLCreateRuntime(runtime: *mut IWinMLRuntime) -> ::windows_sys::core::HRESULT;
}
pub struct IMLOperatorAttributes(i32);
pub struct IMLOperatorKernel(i32);
pub struct IMLOperatorKernelContext(i32);
pub struct IMLOperatorKernelCreationContext(i32);
pub struct IMLOperatorKernelFactory(i32);
pub struct IMLOperatorRegistry(i32);
pub struct IMLOperatorShapeInferenceContext(i32);
pub struct IMLOperatorShapeInferrer(i32);
pub struct IMLOperatorTensor(i32);
pub struct IMLOperatorTensorShapeDescription(i32);
pub struct IMLOperatorTypeInferenceContext(i32);
pub struct IMLOperatorTypeInferrer(i32);
pub struct IWinMLEvaluationContext(i32);
pub struct IWinMLModel(i32);
pub struct IWinMLRuntime(i32);
pub struct IWinMLRuntimeFactory(i32);
pub struct MLOperatorAttribute(i32);
pub struct MLOperatorAttributeNameValue(i32);
pub struct MLOperatorAttributeType(i32);
pub struct MLOperatorEdgeDescription(i32);
pub struct MLOperatorEdgeType(i32);
pub struct MLOperatorEdgeTypeConstraint(i32);
pub struct MLOperatorExecutionType(i32);
pub struct MLOperatorKernelDescription(i32);
pub struct MLOperatorKernelOptions(i32);
pub struct MLOperatorParameterOptions(i32);
pub struct MLOperatorSchemaDescription(i32);
pub struct MLOperatorSchemaEdgeDescription(i32);
pub struct MLOperatorSchemaEdgeTypeFormat(i32);
pub struct MLOperatorSetId(i32);
pub struct MLOperatorTensorDataType(i32);
pub struct WINML_BINDING_DESC(i32);
pub struct WINML_BINDING_TYPE(i32);
pub struct WINML_FEATURE_TYPE(i32);
pub struct WINML_IMAGE_BINDING_DESC(i32);
pub struct WINML_IMAGE_VARIABLE_DESC(i32);
pub struct WINML_MAP_BINDING_DESC(i32);
pub struct WINML_MAP_VARIABLE_DESC(i32);
pub struct WINML_MODEL_DESC(i32);
pub struct WINML_RESOURCE_BINDING_DESC(i32);
pub struct WINML_RUNTIME_TYPE(i32);
pub struct WINML_SEQUENCE_BINDING_DESC(i32);
pub struct WINML_SEQUENCE_VARIABLE_DESC(i32);
pub struct WINML_TENSOR_BINDING_DESC(i32);
pub struct WINML_TENSOR_DATA_TYPE(i32);
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
pub struct WINML_TENSOR_VARIABLE_DESC(i32);
pub struct WINML_VARIABLE_DESC(i32);
