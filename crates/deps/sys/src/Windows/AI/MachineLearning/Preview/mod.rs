#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FeatureElementKindPreview(pub i32);
impl FeatureElementKindPreview {
    pub const Undefined: Self = Self(0i32);
    pub const Float: Self = Self(1i32);
    pub const UInt8: Self = Self(2i32);
    pub const Int8: Self = Self(3i32);
    pub const UInt16: Self = Self(4i32);
    pub const Int16: Self = Self(5i32);
    pub const Int32: Self = Self(6i32);
    pub const Int64: Self = Self(7i32);
    pub const String: Self = Self(8i32);
    pub const Boolean: Self = Self(9i32);
    pub const Float16: Self = Self(10i32);
    pub const Double: Self = Self(11i32);
    pub const UInt32: Self = Self(12i32);
    pub const UInt64: Self = Self(13i32);
    pub const Complex64: Self = Self(14i32);
    pub const Complex128: Self = Self(15i32);
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
    pub const LearningDeviceAny: Self = Self(0i32);
    pub const LearningDeviceCpu: Self = Self(1i32);
    pub const LearningDeviceGpu: Self = Self(2i32);
    pub const LearningDeviceNpu: Self = Self(3i32);
    pub const LearningDeviceDsp: Self = Self(4i32);
    pub const LearningDeviceFpga: Self = Self(5i32);
}
#[repr(transparent)]
pub struct LearningModelEvaluationResultPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelFeatureKindPreview(pub i32);
impl LearningModelFeatureKindPreview {
    pub const Undefined: Self = Self(0i32);
    pub const Tensor: Self = Self(1i32);
    pub const Sequence: Self = Self(2i32);
    pub const Map: Self = Self(3i32);
    pub const Image: Self = Self(4i32);
}
#[repr(transparent)]
pub struct LearningModelPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SequenceVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
