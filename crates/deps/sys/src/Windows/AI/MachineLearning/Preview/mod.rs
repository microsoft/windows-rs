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
impl ::core::marker::Copy for FeatureElementKindPreview {}
impl ::core::clone::Clone for FeatureElementKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageVariableDescriptorPreview {}
impl ::core::clone::Clone for IImageVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInferencingOptionsPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInferencingOptionsPreview {}
impl ::core::clone::Clone for IInferencingOptionsPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelBindingPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelBindingPreview {}
impl ::core::clone::Clone for ILearningModelBindingPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelBindingPreviewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelBindingPreviewFactory {}
impl ::core::clone::Clone for ILearningModelBindingPreviewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelDescriptionPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelDescriptionPreview {}
impl ::core::clone::Clone for ILearningModelDescriptionPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelEvaluationResultPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelEvaluationResultPreview {}
impl ::core::clone::Clone for ILearningModelEvaluationResultPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelPreview {}
impl ::core::clone::Clone for ILearningModelPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelPreviewStatics {}
impl ::core::clone::Clone for ILearningModelPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelVariableDescriptorPreview {}
impl ::core::clone::Clone for ILearningModelVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapVariableDescriptorPreview {}
impl ::core::clone::Clone for IMapVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISequenceVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISequenceVariableDescriptorPreview {}
impl ::core::clone::Clone for ISequenceVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorVariableDescriptorPreview {}
impl ::core::clone::Clone for ITensorVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageVariableDescriptorPreview {}
impl ::core::clone::Clone for ImageVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InferencingOptionsPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InferencingOptionsPreview {}
impl ::core::clone::Clone for InferencingOptionsPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelBindingPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelBindingPreview {}
impl ::core::clone::Clone for LearningModelBindingPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelDescriptionPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelDescriptionPreview {}
impl ::core::clone::Clone for LearningModelDescriptionPreview {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for LearningModelDeviceKindPreview {}
impl ::core::clone::Clone for LearningModelDeviceKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelEvaluationResultPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelEvaluationResultPreview {}
impl ::core::clone::Clone for LearningModelEvaluationResultPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelFeatureKindPreview(pub i32);
impl LearningModelFeatureKindPreview {
    pub const Undefined: Self = Self(0i32);
    pub const Tensor: Self = Self(1i32);
    pub const Sequence: Self = Self(2i32);
    pub const Map: Self = Self(3i32);
    pub const Image: Self = Self(4i32);
}
impl ::core::marker::Copy for LearningModelFeatureKindPreview {}
impl ::core::clone::Clone for LearningModelFeatureKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelPreview {}
impl ::core::clone::Clone for LearningModelPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelVariableDescriptorPreview {}
impl ::core::clone::Clone for LearningModelVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapVariableDescriptorPreview {}
impl ::core::clone::Clone for MapVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SequenceVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SequenceVariableDescriptorPreview {}
impl ::core::clone::Clone for SequenceVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorVariableDescriptorPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorVariableDescriptorPreview {}
impl ::core::clone::Clone for TensorVariableDescriptorPreview {
    fn clone(&self) -> Self {
        *self
    }
}
