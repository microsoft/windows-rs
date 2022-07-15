#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FeatureElementKindPreview(pub i32);
#[cfg(feature = "deprecated")]
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
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for FeatureElementKindPreview {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FeatureElementKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for FeatureElementKindPreview {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for FeatureElementKindPreview {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FeatureElementKindPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FeatureElementKindPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for FeatureElementKindPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.FeatureElementKindPreview;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IImageVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IImageVariableDescriptorPreview {
    type Vtable = IImageVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ae1fa72_029e_4dc5_a2f8_5fb763154150);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IImageVariableDescriptorPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "deprecated")]
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Width: usize,
    #[cfg(feature = "deprecated")]
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Height: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IInferencingOptionsPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IInferencingOptionsPreview {
    type Vtable = IInferencingOptionsPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47bc8205_4d36_47a9_8f68_ffcb339dd0fc);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IInferencingOptionsPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub PreferredDeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelDeviceKindPreview) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreferredDeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub SetPreferredDeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LearningModelDeviceKindPreview) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPreferredDeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub IsTracingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsTracingEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsTracingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsTracingEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxBatchSize: usize,
    #[cfg(feature = "deprecated")]
    pub SetMaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetMaxBatchSize: usize,
    #[cfg(feature = "deprecated")]
    pub MinimizeMemoryAllocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MinimizeMemoryAllocation: usize,
    #[cfg(feature = "deprecated")]
    pub SetMinimizeMemoryAllocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetMinimizeMemoryAllocation: usize,
    #[cfg(feature = "deprecated")]
    pub ReclaimMemoryAfterEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ReclaimMemoryAfterEvaluation: usize,
    #[cfg(feature = "deprecated")]
    pub SetReclaimMemoryAfterEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetReclaimMemoryAfterEvaluation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ILearningModelBindingPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ILearningModelBindingPreview {
    type Vtable = ILearningModelBindingPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93c901e8_6c78_4b4f_aec1_a6bb9e691624);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Bind: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub BindWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, metadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    BindWithProperties: usize,
    #[cfg(feature = "deprecated")]
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Clear: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ILearningModelBindingPreviewFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ILearningModelBindingPreviewFactory {
    type Vtable = ILearningModelBindingPreviewFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48b8219f_1e51_4d77_ae50_3ec164ad3480);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingPreviewFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub CreateFromModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateFromModel: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ILearningModelDescriptionPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ILearningModelDescriptionPreview {
    type Vtable = ILearningModelDescriptionPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf52c09c6_8611_40ad_8e59_de3fd7030a40);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDescriptionPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Author: usize,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Domain: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Version: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Metadata: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub InputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    InputFeatures: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub OutputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    OutputFeatures: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ILearningModelEvaluationResultPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ILearningModelEvaluationResultPreview {
    type Vtable = ILearningModelEvaluationResultPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf25ea9f_9863_4088_8498_87a1f4686f92);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResultPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CorrelationId: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Outputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Outputs: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ILearningModelPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ILearningModelPreview {
    type Vtable = ILearningModelPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x049c266a_93b4_478c_aeb8_70157bf0ff94);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EvaluateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: *mut ::core::ffi::c_void, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EvaluateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: *mut ::core::ffi::c_void, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    EvaluateFeaturesAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub InferencingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InferencingOptions: usize,
    #[cfg(feature = "deprecated")]
    pub SetInferencingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetInferencingOptions: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ILearningModelPreviewStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ILearningModelPreviewStatics {
    type Vtable = ILearningModelPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x164bbb60_8465_4786_8b93_2c16a89289d7);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub LoadModelFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    LoadModelFromStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub LoadModelFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    LoadModelFromStreamAsync: usize,
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ILearningModelVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl ILearningModelVariableDescriptorPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ModelFeatureKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<ILearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: ILearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a ILearningModelVariableDescriptorPreview> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ILearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<ILearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: ILearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::From<&'a ILearningModelVariableDescriptorPreview> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ILearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ILearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &ILearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ILearningModelVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ILearningModelVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ILearningModelVariableDescriptorPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ILearningModelVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for ILearningModelVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b13df682-fc30-492b-8ea0-ed1f53c0b038}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ILearningModelVariableDescriptorPreview {
    type Vtable = ILearningModelVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb13df682_fc30_492b_8ea0_ed1f53c0b038);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelVariableDescriptorPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub ModelFeatureKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKindPreview) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelFeatureKind: usize,
    #[cfg(feature = "deprecated")]
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsRequired: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IMapVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IMapVariableDescriptorPreview {
    type Vtable = IMapVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cb38370_c02b_4236_b3e8_6bdca49c3129);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IMapVariableDescriptorPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub KeyKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    KeyKind: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ValidStringKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ValidStringKeys: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ValidIntegerKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ValidIntegerKeys: usize,
    #[cfg(feature = "deprecated")]
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Fields: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISequenceVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISequenceVariableDescriptorPreview {
    type Vtable = ISequenceVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cd8f292_98b2_4530_a1b6_2ded5fecbc26);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceVariableDescriptorPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub ElementType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ElementType: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ITensorVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ITensorVariableDescriptorPreview {
    type Vtable = ITensorVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa80f501a_9aac_4233_9784_aceaf92510b5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ITensorVariableDescriptorPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DataType: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Shape: usize,
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ImageVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl ImageVariableDescriptorPreview {
    #[doc = "*Required features: `\"Graphics_Imaging\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BitmapPixelFormat)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ModelFeatureKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ImageVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ImageVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ImageVariableDescriptorPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ImageVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for ImageVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview;{7ae1fa72-029e-4dc5-a2f8-5fb763154150})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ImageVariableDescriptorPreview {
    type Vtable = IImageVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = <IImageVariableDescriptorPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ImageVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<ImageVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: ImageVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ImageVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ImageVariableDescriptorPreview> for &::windows::core::IUnknown {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<ImageVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: ImageVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ImageVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ImageVariableDescriptorPreview> for &::windows::core::IInspectable {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<ImageVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&ImageVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&ImageVariableDescriptorPreview> for ::windows::core::InParam<'a, ILearningModelVariableDescriptorPreview> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct InferencingOptionsPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl InferencingOptionsPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PreferredDeviceKind(&self) -> ::windows::core::Result<LearningModelDeviceKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreferredDeviceKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelDeviceKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetPreferredDeviceKind(&self, value: LearningModelDeviceKindPreview) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredDeviceKind)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsTracingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsTracingEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetIsTracingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsTracingEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxBatchSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetMaxBatchSize(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxBatchSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MinimizeMemoryAllocation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MinimizeMemoryAllocation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetMinimizeMemoryAllocation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinimizeMemoryAllocation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ReclaimMemoryAfterEvaluation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReclaimMemoryAfterEvaluation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetReclaimMemoryAfterEvaluation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReclaimMemoryAfterEvaluation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for InferencingOptionsPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for InferencingOptionsPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for InferencingOptionsPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for InferencingOptionsPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InferencingOptionsPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for InferencingOptionsPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.InferencingOptionsPreview;{47bc8205-4d36-47a9-8f68-ffcb339dd0fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for InferencingOptionsPreview {
    type Vtable = IInferencingOptionsPreview_Vtbl;
    const IID: ::windows::core::GUID = <IInferencingOptionsPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for InferencingOptionsPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.InferencingOptionsPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<InferencingOptionsPreview> for ::windows::core::IUnknown {
    fn from(value: InferencingOptionsPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&InferencingOptionsPreview> for ::windows::core::IUnknown {
    fn from(value: &InferencingOptionsPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&InferencingOptionsPreview> for &::windows::core::IUnknown {
    fn from(value: &InferencingOptionsPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<InferencingOptionsPreview> for ::windows::core::IInspectable {
    fn from(value: InferencingOptionsPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&InferencingOptionsPreview> for ::windows::core::IInspectable {
    fn from(value: &InferencingOptionsPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&InferencingOptionsPreview> for &::windows::core::IInspectable {
    fn from(value: &InferencingOptionsPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct LearningModelBindingPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl LearningModelBindingPreview {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Bind<'a, P0>(&self, name: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Bind)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn BindWithProperties<'a, P0, P1, E1>(&self, name: &::windows::core::HSTRING, value: P0, metadata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IPropertySet>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BindWithProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.into().abi(), metadata.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateFromModel<'a, P0>(model: P0) -> ::windows::core::Result<LearningModelBindingPreview>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, LearningModelPreview>>,
    {
        Self::ILearningModelBindingPreviewFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromModel)(::windows::core::Interface::as_raw(this), model.into().abi(), result__.as_mut_ptr()).from_abi::<LearningModelBindingPreview>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Split)(::windows::core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ILearningModelBindingPreviewFactory<R, F: FnOnce(&ILearningModelBindingPreviewFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LearningModelBindingPreview, ILearningModelBindingPreviewFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelBindingPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for LearningModelBindingPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for LearningModelBindingPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for LearningModelBindingPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelBindingPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for LearningModelBindingPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelBindingPreview;{93c901e8-6c78-4b4f-aec1-a6bb9e691624})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for LearningModelBindingPreview {
    type Vtable = ILearningModelBindingPreview_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelBindingPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for LearningModelBindingPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelBindingPreview";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::iter::IntoIterator for LearningModelBindingPreview {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::iter::IntoIterator for &LearningModelBindingPreview {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelBindingPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelBindingPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelBindingPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelBindingPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelBindingPreview> for &::windows::core::IUnknown {
    fn from(value: &LearningModelBindingPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelBindingPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelBindingPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelBindingPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelBindingPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelBindingPreview> for &::windows::core::IInspectable {
    fn from(value: &LearningModelBindingPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::convert::TryFrom<LearningModelBindingPreview> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::convert::TryFrom<&LearningModelBindingPreview> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&LearningModelBindingPreview> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::convert::TryFrom<LearningModelBindingPreview> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::convert::TryFrom<&LearningModelBindingPreview> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&LearningModelBindingPreview> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct LearningModelDescriptionPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl LearningModelDescriptionPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Author)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Domain)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Version(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Version)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Metadata(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Metadata)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn InputFeatures(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InputFeatures)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn OutputFeatures(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OutputFeatures)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelDescriptionPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for LearningModelDescriptionPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for LearningModelDescriptionPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for LearningModelDescriptionPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDescriptionPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for LearningModelDescriptionPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview;{f52c09c6-8611-40ad-8e59-de3fd7030a40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for LearningModelDescriptionPreview {
    type Vtable = ILearningModelDescriptionPreview_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelDescriptionPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for LearningModelDescriptionPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelDescriptionPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelDescriptionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelDescriptionPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelDescriptionPreview> for &::windows::core::IUnknown {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelDescriptionPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelDescriptionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelDescriptionPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelDescriptionPreview> for &::windows::core::IInspectable {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LearningModelDeviceKindPreview(pub i32);
#[cfg(feature = "deprecated")]
impl LearningModelDeviceKindPreview {
    pub const LearningDeviceAny: Self = Self(0i32);
    pub const LearningDeviceCpu: Self = Self(1i32);
    pub const LearningDeviceGpu: Self = Self(2i32);
    pub const LearningDeviceNpu: Self = Self(3i32);
    pub const LearningDeviceDsp: Self = Self(4i32);
    pub const LearningDeviceFpga: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for LearningModelDeviceKindPreview {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelDeviceKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for LearningModelDeviceKindPreview {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for LearningModelDeviceKindPreview {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for LearningModelDeviceKindPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDeviceKindPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for LearningModelDeviceKindPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.LearningModelDeviceKindPreview;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct LearningModelEvaluationResultPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl LearningModelEvaluationResultPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Outputs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Outputs)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelEvaluationResultPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for LearningModelEvaluationResultPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for LearningModelEvaluationResultPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for LearningModelEvaluationResultPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelEvaluationResultPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for LearningModelEvaluationResultPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview;{df25ea9f-9863-4088-8498-87a1f4686f92})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for LearningModelEvaluationResultPreview {
    type Vtable = ILearningModelEvaluationResultPreview_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelEvaluationResultPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for LearningModelEvaluationResultPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelEvaluationResultPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelEvaluationResultPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for &::windows::core::IUnknown {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelEvaluationResultPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelEvaluationResultPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for &::windows::core::IInspectable {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LearningModelFeatureKindPreview(pub i32);
#[cfg(feature = "deprecated")]
impl LearningModelFeatureKindPreview {
    pub const Undefined: Self = Self(0i32);
    pub const Tensor: Self = Self(1i32);
    pub const Sequence: Self = Self(2i32);
    pub const Map: Self = Self(3i32);
    pub const Image: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for LearningModelFeatureKindPreview {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelFeatureKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for LearningModelFeatureKindPreview {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for LearningModelFeatureKindPreview {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for LearningModelFeatureKindPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelFeatureKindPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for LearningModelFeatureKindPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.LearningModelFeatureKindPreview;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct LearningModelPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl LearningModelPreview {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn EvaluateAsync<'a, P0>(&self, binding: P0, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, LearningModelBindingPreview>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EvaluateAsync)(::windows::core::Interface::as_raw(this), binding.into().abi(), ::core::mem::transmute_copy(correlationid), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn EvaluateFeaturesAsync<'a, P0, E0>(&self, features: P0, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EvaluateFeaturesAsync)(::windows::core::Interface::as_raw(this), features.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(correlationid), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<LearningModelDescriptionPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelDescriptionPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn InferencingOptions(&self) -> ::windows::core::Result<InferencingOptionsPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InferencingOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InferencingOptionsPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetInferencingOptions<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, InferencingOptionsPreview>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInferencingOptions)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn LoadModelFromStorageFileAsync<'a, P0, E0>(modelfile: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILearningModelPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LoadModelFromStorageFileAsync)(::windows::core::Interface::as_raw(this), modelfile.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn LoadModelFromStreamAsync<'a, P0, E0>(modelstream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILearningModelPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LoadModelFromStreamAsync)(::windows::core::Interface::as_raw(this), modelstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ILearningModelPreviewStatics<R, F: FnOnce(&ILearningModelPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LearningModelPreview, ILearningModelPreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for LearningModelPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for LearningModelPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for LearningModelPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for LearningModelPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelPreview;{049c266a-93b4-478c-aeb8-70157bf0ff94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for LearningModelPreview {
    type Vtable = ILearningModelPreview_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for LearningModelPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelPreview> for &::windows::core::IUnknown {
    fn from(value: &LearningModelPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelPreview> for &::windows::core::IInspectable {
    fn from(value: &LearningModelPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct LearningModelVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl LearningModelVariableDescriptorPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ModelFeatureKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for LearningModelVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for LearningModelVariableDescriptorPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for LearningModelVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for LearningModelVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview;{b13df682-fc30-492b-8ea0-ed1f53c0b038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for LearningModelVariableDescriptorPreview {
    type Vtable = ILearningModelVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelVariableDescriptorPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for LearningModelVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for &::windows::core::IUnknown {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<LearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for &::windows::core::IInspectable {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<LearningModelVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&LearningModelVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&LearningModelVariableDescriptorPreview> for ::windows::core::InParam<'a, ILearningModelVariableDescriptorPreview> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct MapVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl MapVariableDescriptorPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ModelFeatureKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn KeyKind(&self) -> ::windows::core::Result<FeatureElementKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FeatureElementKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ValidStringKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ValidStringKeys)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ValidIntegerKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ValidIntegerKeys)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterable<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Fields(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Fields)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ILearningModelVariableDescriptorPreview>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for MapVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for MapVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for MapVariableDescriptorPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for MapVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for MapVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview;{3cb38370-c02b-4236-b3e8-6bdca49c3129})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for MapVariableDescriptorPreview {
    type Vtable = IMapVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = <IMapVariableDescriptorPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for MapVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<MapVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: MapVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&MapVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&MapVariableDescriptorPreview> for &::windows::core::IUnknown {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<MapVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: MapVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&MapVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&MapVariableDescriptorPreview> for &::windows::core::IInspectable {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<MapVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: MapVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&MapVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &MapVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&MapVariableDescriptorPreview> for ::windows::core::InParam<'a, ILearningModelVariableDescriptorPreview> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MapVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SequenceVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SequenceVariableDescriptorPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ModelFeatureKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ElementType(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ElementType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ILearningModelVariableDescriptorPreview>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SequenceVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SequenceVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SequenceVariableDescriptorPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SequenceVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SequenceVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SequenceVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview;{9cd8f292-98b2-4530-a1b6-2ded5fecbc26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SequenceVariableDescriptorPreview {
    type Vtable = ISequenceVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = <ISequenceVariableDescriptorPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SequenceVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SequenceVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: SequenceVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for &::windows::core::IUnknown {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SequenceVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: SequenceVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for &::windows::core::IInspectable {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<SequenceVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: SequenceVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&SequenceVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &SequenceVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&SequenceVariableDescriptorPreview> for ::windows::core::InParam<'a, ILearningModelVariableDescriptorPreview> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SequenceVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct TensorVariableDescriptorPreview(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl TensorVariableDescriptorPreview {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ModelFeatureKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DataType(&self) -> ::windows::core::Result<FeatureElementKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DataType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FeatureElementKindPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterable<i64>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for TensorVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for TensorVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for TensorVariableDescriptorPreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for TensorVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for TensorVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview;{a80f501a-9aac-4233-9784-aceaf92510b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for TensorVariableDescriptorPreview {
    type Vtable = ITensorVariableDescriptorPreview_Vtbl;
    const IID: ::windows::core::GUID = <ITensorVariableDescriptorPreview as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for TensorVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TensorVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: TensorVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TensorVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TensorVariableDescriptorPreview> for &::windows::core::IUnknown {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<TensorVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: TensorVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TensorVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&TensorVariableDescriptorPreview> for &::windows::core::IInspectable {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<TensorVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&TensorVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::core::convert::TryFrom<&TensorVariableDescriptorPreview> for ::windows::core::InParam<'a, ILearningModelVariableDescriptorPreview> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
