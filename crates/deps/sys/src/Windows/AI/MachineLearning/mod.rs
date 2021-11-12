#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "AI_MachineLearning_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IImageFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageFeatureDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageFeatureValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageFeatureValueStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelBindingFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelDeviceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelEvaluationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelFeatureValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelOperatorProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelSessionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelSessionFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelSessionOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelSessionOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelSessionOptions3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISequenceFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorBoolean(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorBooleanStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorBooleanStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorDouble(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorDoubleStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorDoubleStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorFloat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorFloat16Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorFloat16BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorFloat16BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorFloatStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorFloatStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt16Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt16BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt16BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt32Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt32BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt32BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt64Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt64BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt64BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt8Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt8BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorInt8BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorStringStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorStringStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt16Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt16BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt16BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt32Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt32BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt32BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt64Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt64BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt64BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt8Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt8BitStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorUInt8BitStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageFeatureValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelDeviceKind(pub i32);
impl LearningModelDeviceKind {
    pub const Default: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const DirectX: Self = Self(2i32);
    pub const DirectXHighPerformance: Self = Self(3i32);
    pub const DirectXMinPower: Self = Self(4i32);
}
#[repr(transparent)]
pub struct LearningModelEvaluationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelFeatureKind(pub i32);
impl LearningModelFeatureKind {
    pub const Tensor: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
    pub const Map: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
#[repr(transparent)]
pub struct LearningModelPixelRange(pub i32);
impl LearningModelPixelRange {
    pub const ZeroTo255: Self = Self(0i32);
    pub const ZeroToOne: Self = Self(1i32);
    pub const MinusOneToOne: Self = Self(2i32);
}
#[repr(transparent)]
pub struct LearningModelSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LearningModelSessionOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MachineLearningContract(i32);
#[repr(transparent)]
pub struct MapFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SequenceFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorBoolean(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorDouble(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorFeatureDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorFloat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorFloat16Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorInt16Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorInt32Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorInt64Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorInt8Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorKind(pub i32);
impl TensorKind {
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
pub struct TensorString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorUInt16Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorUInt32Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorUInt64Bit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TensorUInt8Bit(pub *mut ::core::ffi::c_void);
