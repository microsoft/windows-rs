#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "AI_MachineLearning_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IImageFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageFeatureDescriptor {}
impl ::core::clone::Clone for IImageFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageFeatureDescriptor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageFeatureDescriptor2 {}
impl ::core::clone::Clone for IImageFeatureDescriptor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageFeatureValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageFeatureValue {}
impl ::core::clone::Clone for IImageFeatureValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageFeatureValueStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageFeatureValueStatics {}
impl ::core::clone::Clone for IImageFeatureValueStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModel {}
impl ::core::clone::Clone for ILearningModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelBinding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelBinding {}
impl ::core::clone::Clone for ILearningModelBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelBindingFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelBindingFactory {}
impl ::core::clone::Clone for ILearningModelBindingFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelDevice {}
impl ::core::clone::Clone for ILearningModelDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelDeviceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelDeviceFactory {}
impl ::core::clone::Clone for ILearningModelDeviceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelDeviceStatics {}
impl ::core::clone::Clone for ILearningModelDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelEvaluationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelEvaluationResult {}
impl ::core::clone::Clone for ILearningModelEvaluationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelFeatureDescriptor {}
impl ::core::clone::Clone for ILearningModelFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelFeatureValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelFeatureValue {}
impl ::core::clone::Clone for ILearningModelFeatureValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelOperatorProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelOperatorProvider {}
impl ::core::clone::Clone for ILearningModelOperatorProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelSession {}
impl ::core::clone::Clone for ILearningModelSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelSessionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelSessionFactory {}
impl ::core::clone::Clone for ILearningModelSessionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelSessionFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelSessionFactory2 {}
impl ::core::clone::Clone for ILearningModelSessionFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelSessionOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelSessionOptions {}
impl ::core::clone::Clone for ILearningModelSessionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelSessionOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelSessionOptions2 {}
impl ::core::clone::Clone for ILearningModelSessionOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelSessionOptions3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelSessionOptions3 {}
impl ::core::clone::Clone for ILearningModelSessionOptions3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelStatics {}
impl ::core::clone::Clone for ILearningModelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapFeatureDescriptor {}
impl ::core::clone::Clone for IMapFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISequenceFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISequenceFeatureDescriptor {}
impl ::core::clone::Clone for ISequenceFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensor {}
impl ::core::clone::Clone for ITensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorBoolean(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorBoolean {}
impl ::core::clone::Clone for ITensorBoolean {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorBooleanStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorBooleanStatics {}
impl ::core::clone::Clone for ITensorBooleanStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorBooleanStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorBooleanStatics2 {}
impl ::core::clone::Clone for ITensorBooleanStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorDouble(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorDouble {}
impl ::core::clone::Clone for ITensorDouble {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorDoubleStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorDoubleStatics {}
impl ::core::clone::Clone for ITensorDoubleStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorDoubleStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorDoubleStatics2 {}
impl ::core::clone::Clone for ITensorDoubleStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorFeatureDescriptor {}
impl ::core::clone::Clone for ITensorFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorFloat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorFloat {}
impl ::core::clone::Clone for ITensorFloat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorFloat16Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorFloat16Bit {}
impl ::core::clone::Clone for ITensorFloat16Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorFloat16BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorFloat16BitStatics {}
impl ::core::clone::Clone for ITensorFloat16BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorFloat16BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorFloat16BitStatics2 {}
impl ::core::clone::Clone for ITensorFloat16BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorFloatStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorFloatStatics {}
impl ::core::clone::Clone for ITensorFloatStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorFloatStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorFloatStatics2 {}
impl ::core::clone::Clone for ITensorFloatStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt16Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt16Bit {}
impl ::core::clone::Clone for ITensorInt16Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt16BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt16BitStatics {}
impl ::core::clone::Clone for ITensorInt16BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt16BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt16BitStatics2 {}
impl ::core::clone::Clone for ITensorInt16BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt32Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt32Bit {}
impl ::core::clone::Clone for ITensorInt32Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt32BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt32BitStatics {}
impl ::core::clone::Clone for ITensorInt32BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt32BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt32BitStatics2 {}
impl ::core::clone::Clone for ITensorInt32BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt64Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt64Bit {}
impl ::core::clone::Clone for ITensorInt64Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt64BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt64BitStatics {}
impl ::core::clone::Clone for ITensorInt64BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt64BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt64BitStatics2 {}
impl ::core::clone::Clone for ITensorInt64BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt8Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt8Bit {}
impl ::core::clone::Clone for ITensorInt8Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt8BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt8BitStatics {}
impl ::core::clone::Clone for ITensorInt8BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorInt8BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorInt8BitStatics2 {}
impl ::core::clone::Clone for ITensorInt8BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorString(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorString {}
impl ::core::clone::Clone for ITensorString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorStringStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorStringStatics {}
impl ::core::clone::Clone for ITensorStringStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorStringStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorStringStatics2 {}
impl ::core::clone::Clone for ITensorStringStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt16Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt16Bit {}
impl ::core::clone::Clone for ITensorUInt16Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt16BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt16BitStatics {}
impl ::core::clone::Clone for ITensorUInt16BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt16BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt16BitStatics2 {}
impl ::core::clone::Clone for ITensorUInt16BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt32Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt32Bit {}
impl ::core::clone::Clone for ITensorUInt32Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt32BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt32BitStatics {}
impl ::core::clone::Clone for ITensorUInt32BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt32BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt32BitStatics2 {}
impl ::core::clone::Clone for ITensorUInt32BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt64Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt64Bit {}
impl ::core::clone::Clone for ITensorUInt64Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt64BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt64BitStatics {}
impl ::core::clone::Clone for ITensorUInt64BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt64BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt64BitStatics2 {}
impl ::core::clone::Clone for ITensorUInt64BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt8Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt8Bit {}
impl ::core::clone::Clone for ITensorUInt8Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt8BitStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt8BitStatics {}
impl ::core::clone::Clone for ITensorUInt8BitStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorUInt8BitStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorUInt8BitStatics2 {}
impl ::core::clone::Clone for ITensorUInt8BitStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageFeatureDescriptor {}
impl ::core::clone::Clone for ImageFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageFeatureValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageFeatureValue {}
impl ::core::clone::Clone for ImageFeatureValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModel {}
impl ::core::clone::Clone for LearningModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelBinding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelBinding {}
impl ::core::clone::Clone for LearningModelBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelDevice {}
impl ::core::clone::Clone for LearningModelDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelDeviceKind(pub i32);
impl LearningModelDeviceKind {
    pub const Default: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const DirectX: Self = Self(2i32);
    pub const DirectXHighPerformance: Self = Self(3i32);
    pub const DirectXMinPower: Self = Self(4i32);
}
impl ::core::marker::Copy for LearningModelDeviceKind {}
impl ::core::clone::Clone for LearningModelDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelEvaluationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelEvaluationResult {}
impl ::core::clone::Clone for LearningModelEvaluationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelFeatureKind(pub i32);
impl LearningModelFeatureKind {
    pub const Tensor: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
    pub const Map: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl ::core::marker::Copy for LearningModelFeatureKind {}
impl ::core::clone::Clone for LearningModelFeatureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelPixelRange(pub i32);
impl LearningModelPixelRange {
    pub const ZeroTo255: Self = Self(0i32);
    pub const ZeroToOne: Self = Self(1i32);
    pub const MinusOneToOne: Self = Self(2i32);
}
impl ::core::marker::Copy for LearningModelPixelRange {}
impl ::core::clone::Clone for LearningModelPixelRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelSession {}
impl ::core::clone::Clone for LearningModelSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LearningModelSessionOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LearningModelSessionOptions {}
impl ::core::clone::Clone for LearningModelSessionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapFeatureDescriptor {}
impl ::core::clone::Clone for MapFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SequenceFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SequenceFeatureDescriptor {}
impl ::core::clone::Clone for SequenceFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorBoolean(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorBoolean {}
impl ::core::clone::Clone for TensorBoolean {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorDouble(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorDouble {}
impl ::core::clone::Clone for TensorDouble {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorFeatureDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorFeatureDescriptor {}
impl ::core::clone::Clone for TensorFeatureDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorFloat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorFloat {}
impl ::core::clone::Clone for TensorFloat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorFloat16Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorFloat16Bit {}
impl ::core::clone::Clone for TensorFloat16Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorInt16Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorInt16Bit {}
impl ::core::clone::Clone for TensorInt16Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorInt32Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorInt32Bit {}
impl ::core::clone::Clone for TensorInt32Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorInt64Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorInt64Bit {}
impl ::core::clone::Clone for TensorInt64Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorInt8Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorInt8Bit {}
impl ::core::clone::Clone for TensorInt8Bit {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TensorKind {}
impl ::core::clone::Clone for TensorKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorString(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorString {}
impl ::core::clone::Clone for TensorString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorUInt16Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorUInt16Bit {}
impl ::core::clone::Clone for TensorUInt16Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorUInt32Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorUInt32Bit {}
impl ::core::clone::Clone for TensorUInt32Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorUInt64Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorUInt64Bit {}
impl ::core::clone::Clone for TensorUInt64Bit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TensorUInt8Bit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TensorUInt8Bit {}
impl ::core::clone::Clone for TensorUInt8Bit {
    fn clone(&self) -> Self {
        *self
    }
}
