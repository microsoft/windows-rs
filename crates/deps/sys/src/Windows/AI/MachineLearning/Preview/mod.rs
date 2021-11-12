#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FeatureElementKindPreview(pub i32);
impl FeatureElementKindPreview {
    pub const Undefined: FeatureElementKindPreview = FeatureElementKindPreview(0i32);
    pub const Float: FeatureElementKindPreview = FeatureElementKindPreview(1i32);
    pub const UInt8: FeatureElementKindPreview = FeatureElementKindPreview(2i32);
    pub const Int8: FeatureElementKindPreview = FeatureElementKindPreview(3i32);
    pub const UInt16: FeatureElementKindPreview = FeatureElementKindPreview(4i32);
    pub const Int16: FeatureElementKindPreview = FeatureElementKindPreview(5i32);
    pub const Int32: FeatureElementKindPreview = FeatureElementKindPreview(6i32);
    pub const Int64: FeatureElementKindPreview = FeatureElementKindPreview(7i32);
    pub const String: FeatureElementKindPreview = FeatureElementKindPreview(8i32);
    pub const Boolean: FeatureElementKindPreview = FeatureElementKindPreview(9i32);
    pub const Float16: FeatureElementKindPreview = FeatureElementKindPreview(10i32);
    pub const Double: FeatureElementKindPreview = FeatureElementKindPreview(11i32);
    pub const UInt32: FeatureElementKindPreview = FeatureElementKindPreview(12i32);
    pub const UInt64: FeatureElementKindPreview = FeatureElementKindPreview(13i32);
    pub const Complex64: FeatureElementKindPreview = FeatureElementKindPreview(14i32);
    pub const Complex128: FeatureElementKindPreview = FeatureElementKindPreview(15i32);
}
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
#[repr(transparent)]
pub struct LearningModelDeviceKindPreview(pub i32);
impl LearningModelDeviceKindPreview {
    pub const LearningDeviceAny: LearningModelDeviceKindPreview = LearningModelDeviceKindPreview(0i32);
    pub const LearningDeviceCpu: LearningModelDeviceKindPreview = LearningModelDeviceKindPreview(1i32);
    pub const LearningDeviceGpu: LearningModelDeviceKindPreview = LearningModelDeviceKindPreview(2i32);
    pub const LearningDeviceNpu: LearningModelDeviceKindPreview = LearningModelDeviceKindPreview(3i32);
    pub const LearningDeviceDsp: LearningModelDeviceKindPreview = LearningModelDeviceKindPreview(4i32);
    pub const LearningDeviceFpga: LearningModelDeviceKindPreview = LearningModelDeviceKindPreview(5i32);
}
#[repr(transparent)]
pub struct LearningModelEvaluationResultPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelFeatureKindPreview(pub i32);
impl LearningModelFeatureKindPreview {
    pub const Undefined: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(0i32);
    pub const Tensor: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(1i32);
    pub const Sequence: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(2i32);
    pub const Map: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(3i32);
    pub const Image: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(4i32);
}
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
