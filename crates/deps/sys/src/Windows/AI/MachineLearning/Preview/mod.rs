#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct FeatureElementKindPreview(i32);
#[repr(transparent)]
pub struct IImageVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInferencingOptionsPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelBindingPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelBindingPreviewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelDescriptionPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelEvaluationResultPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISequenceVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InferencingOptionsPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelBindingPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelDescriptionPreview(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LearningModelDeviceKindPreview(i32);
#[repr(transparent)]
pub struct LearningModelEvaluationResultPreview(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LearningModelFeatureKindPreview(i32);
#[repr(transparent)]
pub struct LearningModelPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MachineLearningPreviewContract(i32);
#[repr(transparent)]
pub struct MapVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SequenceVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
