#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for FeatureElementKindPreview {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FeatureElementKindPreview {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FeatureElementKindPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.FeatureElementKindPreview;i4)");
}
impl ::windows::core::DefaultType for FeatureElementKindPreview {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageVariableDescriptorPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IImageVariableDescriptorPreview {
    type Vtable = IImageVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ae1fa72_029e_4dc5_a2f8_5fb763154150);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageVariableDescriptorPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInferencingOptionsPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInferencingOptionsPreview {
    type Vtable = IInferencingOptionsPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47bc8205_4d36_47a9_8f68_ffcb339dd0fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInferencingOptionsPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LearningModelDeviceKindPreview) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: LearningModelDeviceKindPreview) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelBindingPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILearningModelBindingPreview {
    type Vtable = ILearningModelBindingPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93c901e8_6c78_4b4f_aec1_a6bb9e691624);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr, metadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelBindingPreviewFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILearningModelBindingPreviewFactory {
    type Vtable = ILearningModelBindingPreviewFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48b8219f_1e51_4d77_ae50_3ec164ad3480);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingPreviewFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, model: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelDescriptionPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILearningModelDescriptionPreview {
    type Vtable = ILearningModelDescriptionPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf52c09c6_8611_40ad_8e59_de3fd7030a40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDescriptionPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResultPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILearningModelEvaluationResultPreview {
    type Vtable = ILearningModelEvaluationResultPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf25ea9f_9863_4088_8498_87a1f4686f92);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResultPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILearningModelPreview {
    type Vtable = ILearningModelPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x049c266a_93b4_478c_aeb8_70157bf0ff94);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, binding: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, features: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelPreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILearningModelPreviewStatics {
    type Vtable = ILearningModelPreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x164bbb60_8465_4786_8b93_2c16a89289d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, modelfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, modelstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
pub struct ILearningModelVariableDescriptorPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILearningModelVariableDescriptorPreview {
    type Vtable = ILearningModelVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb13df682_fc30_492b_8ea0_ed1f53c0b038);
}
impl ILearningModelVariableDescriptorPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = self;
        unsafe {
            let mut result__: LearningModelFeatureKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ILearningModelVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b13df682-fc30-492b-8ea0-ed1f53c0b038}");
}
impl ::core::convert::From<ILearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: ILearningModelVariableDescriptorPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelVariableDescriptorPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: ILearningModelVariableDescriptorPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &ILearningModelVariableDescriptorPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelVariableDescriptorPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LearningModelFeatureKindPreview) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapVariableDescriptorPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMapVariableDescriptorPreview {
    type Vtable = IMapVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cb38370_c02b_4236_b3e8_6bdca49c3129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapVariableDescriptorPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISequenceVariableDescriptorPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISequenceVariableDescriptorPreview {
    type Vtable = ISequenceVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cd8f292_98b2_4530_a1b6_2ded5fecbc26);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceVariableDescriptorPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorVariableDescriptorPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITensorVariableDescriptorPreview {
    type Vtable = ITensorVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa80f501a_9aac_4233_9784_aceaf92510b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorVariableDescriptorPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ImageVariableDescriptorPreview(pub ::windows::core::IInspectable);
impl ImageVariableDescriptorPreview {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Graphics_Imaging`*"]
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::Imaging::BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ImageVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview;{7ae1fa72-029e-4dc5-a2f8-5fb763154150})");
}
unsafe impl ::windows::core::Interface for ImageVariableDescriptorPreview {
    type Vtable = IImageVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ae1fa72_029e_4dc5_a2f8_5fb763154150);
}
impl ::windows::core::RuntimeName for ImageVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview";
}
impl ::core::convert::From<ImageVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: ImageVariableDescriptorPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ImageVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ImageVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: ImageVariableDescriptorPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ImageVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ImageVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InferencingOptionsPreview(pub ::windows::core::IInspectable);
impl InferencingOptionsPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn PreferredDeviceKind(&self) -> ::windows::core::Result<LearningModelDeviceKindPreview> {
        let this = self;
        unsafe {
            let mut result__: LearningModelDeviceKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelDeviceKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn SetPreferredDeviceKind(&self, value: LearningModelDeviceKindPreview) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn IsTracingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn SetIsTracingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn MaxBatchSize(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn SetMaxBatchSize(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn MinimizeMemoryAllocation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn SetMinimizeMemoryAllocation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ReclaimMemoryAfterEvaluation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn SetReclaimMemoryAfterEvaluation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InferencingOptionsPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.InferencingOptionsPreview;{47bc8205-4d36-47a9-8f68-ffcb339dd0fc})");
}
unsafe impl ::windows::core::Interface for InferencingOptionsPreview {
    type Vtable = IInferencingOptionsPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47bc8205_4d36_47a9_8f68_ffcb339dd0fc);
}
impl ::windows::core::RuntimeName for InferencingOptionsPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.InferencingOptionsPreview";
}
impl ::core::convert::From<InferencingOptionsPreview> for ::windows::core::IUnknown {
    fn from(value: InferencingOptionsPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InferencingOptionsPreview> for ::windows::core::IUnknown {
    fn from(value: &InferencingOptionsPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InferencingOptionsPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InferencingOptionsPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InferencingOptionsPreview> for ::windows::core::IInspectable {
    fn from(value: InferencingOptionsPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InferencingOptionsPreview> for ::windows::core::IInspectable {
    fn from(value: &InferencingOptionsPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InferencingOptionsPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InferencingOptionsPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LearningModelBindingPreview(pub ::windows::core::IInspectable);
impl LearningModelBindingPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Bind<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn BindWithProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IPropertySet>>(&self, name: Param0, value: Param1, metadata: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn CreateFromModel<'a, Param0: ::windows::core::IntoParam<'a, LearningModelPreview>>(model: Param0) -> ::windows::core::Result<LearningModelBindingPreview> {
        Self::ILearningModelBindingPreviewFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), model.into_param().abi(), &mut result__).from_abi::<LearningModelBindingPreview>(result__)
        })
    }
    pub fn ILearningModelBindingPreviewFactory<R, F: FnOnce(&ILearningModelBindingPreviewFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelBindingPreview, ILearningModelBindingPreviewFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelBindingPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelBindingPreview;{93c901e8-6c78-4b4f-aec1-a6bb9e691624})");
}
unsafe impl ::windows::core::Interface for LearningModelBindingPreview {
    type Vtable = ILearningModelBindingPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93c901e8_6c78_4b4f_aec1_a6bb9e691624);
}
impl ::windows::core::RuntimeName for LearningModelBindingPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelBindingPreview";
}
impl ::core::convert::From<LearningModelBindingPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelBindingPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LearningModelBindingPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelBindingPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LearningModelBindingPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelBindingPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LearningModelBindingPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelBindingPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<LearningModelBindingPreview> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&LearningModelBindingPreview> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for &LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<LearningModelBindingPreview> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&LearningModelBindingPreview> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> for &LearningModelBindingPreview {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for LearningModelBindingPreview {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &LearningModelBindingPreview {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LearningModelDescriptionPreview(pub ::windows::core::IInspectable);
impl LearningModelDescriptionPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Version(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn Metadata(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn InputFeatures(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn OutputFeatures(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelDescriptionPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview;{f52c09c6-8611-40ad-8e59-de3fd7030a40})");
}
unsafe impl ::windows::core::Interface for LearningModelDescriptionPreview {
    type Vtable = ILearningModelDescriptionPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf52c09c6_8611_40ad_8e59_de3fd7030a40);
}
impl ::windows::core::RuntimeName for LearningModelDescriptionPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview";
}
impl ::core::convert::From<LearningModelDescriptionPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelDescriptionPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LearningModelDescriptionPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LearningModelDescriptionPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelDescriptionPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LearningModelDescriptionPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for LearningModelDeviceKindPreview {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LearningModelDeviceKindPreview {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LearningModelDeviceKindPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.LearningModelDeviceKindPreview;i4)");
}
impl ::windows::core::DefaultType for LearningModelDeviceKindPreview {
    type DefaultType = Self;
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LearningModelEvaluationResultPreview(pub ::windows::core::IInspectable);
impl LearningModelEvaluationResultPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn Outputs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelEvaluationResultPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview;{df25ea9f-9863-4088-8498-87a1f4686f92})");
}
unsafe impl ::windows::core::Interface for LearningModelEvaluationResultPreview {
    type Vtable = ILearningModelEvaluationResultPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf25ea9f_9863_4088_8498_87a1f4686f92);
}
impl ::windows::core::RuntimeName for LearningModelEvaluationResultPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview";
}
impl ::core::convert::From<LearningModelEvaluationResultPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelEvaluationResultPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LearningModelEvaluationResultPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelEvaluationResultPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LearningModelFeatureKindPreview(pub i32);
impl LearningModelFeatureKindPreview {
    pub const Undefined: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(0i32);
    pub const Tensor: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(1i32);
    pub const Sequence: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(2i32);
    pub const Map: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(3i32);
    pub const Image: LearningModelFeatureKindPreview = LearningModelFeatureKindPreview(4i32);
}
impl ::core::convert::From<i32> for LearningModelFeatureKindPreview {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LearningModelFeatureKindPreview {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LearningModelFeatureKindPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.LearningModelFeatureKindPreview;i4)");
}
impl ::windows::core::DefaultType for LearningModelFeatureKindPreview {
    type DefaultType = Self;
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LearningModelPreview(pub ::windows::core::IInspectable);
impl LearningModelPreview {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation`*"]
    pub fn EvaluateAsync<'a, Param0: ::windows::core::IntoParam<'a, LearningModelBindingPreview>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, binding: Param0, correlationid: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), binding.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation`, `Foundation_Collections`*"]
    pub fn EvaluateFeaturesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, features: Param0, correlationid: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), features.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<LearningModelDescriptionPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelDescriptionPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn InferencingOptions(&self) -> ::windows::core::Result<InferencingOptionsPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InferencingOptionsPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn SetInferencingOptions<'a, Param0: ::windows::core::IntoParam<'a, InferencingOptionsPreview>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation`, `Storage`*"]
    pub fn LoadModelFromStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>>(modelfile: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>> {
        Self::ILearningModelPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), modelfile.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadModelFromStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IRandomAccessStreamReference>>(modelstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>> {
        Self::ILearningModelPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), modelstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>(result__)
        })
    }
    pub fn ILearningModelPreviewStatics<R, F: FnOnce(&ILearningModelPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelPreview, ILearningModelPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelPreview;{049c266a-93b4-478c-aeb8-70157bf0ff94})");
}
unsafe impl ::windows::core::Interface for LearningModelPreview {
    type Vtable = ILearningModelPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x049c266a_93b4_478c_aeb8_70157bf0ff94);
}
impl ::windows::core::RuntimeName for LearningModelPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelPreview";
}
impl ::core::convert::From<LearningModelPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LearningModelPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LearningModelPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LearningModelPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LearningModelVariableDescriptorPreview(pub ::windows::core::IInspectable);
impl LearningModelVariableDescriptorPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = self;
        unsafe {
            let mut result__: LearningModelFeatureKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview;{b13df682-fc30-492b-8ea0-ed1f53c0b038})");
}
unsafe impl ::windows::core::Interface for LearningModelVariableDescriptorPreview {
    type Vtable = ILearningModelVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb13df682_fc30_492b_8ea0_ed1f53c0b038);
}
impl ::windows::core::RuntimeName for LearningModelVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview";
}
impl ::core::convert::From<LearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: LearningModelVariableDescriptorPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: LearningModelVariableDescriptorPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LearningModelVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    fn from(value: LearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct MachineLearningPreviewContract(pub u8);
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapVariableDescriptorPreview(pub ::windows::core::IInspectable);
impl MapVariableDescriptorPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn KeyKind(&self) -> ::windows::core::Result<FeatureElementKindPreview> {
        let this = self;
        unsafe {
            let mut result__: FeatureElementKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FeatureElementKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn ValidStringKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn ValidIntegerKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<i64>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Fields(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ILearningModelVariableDescriptorPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MapVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview;{3cb38370-c02b-4236-b3e8-6bdca49c3129})");
}
unsafe impl ::windows::core::Interface for MapVariableDescriptorPreview {
    type Vtable = IMapVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cb38370_c02b_4236_b3e8_6bdca49c3129);
}
impl ::windows::core::RuntimeName for MapVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview";
}
impl ::core::convert::From<MapVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: MapVariableDescriptorPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: MapVariableDescriptorPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MapVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: MapVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MapVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &MapVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SequenceVariableDescriptorPreview(pub ::windows::core::IInspectable);
impl SequenceVariableDescriptorPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ElementType(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ILearningModelVariableDescriptorPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SequenceVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview;{9cd8f292-98b2-4530-a1b6-2ded5fecbc26})");
}
unsafe impl ::windows::core::Interface for SequenceVariableDescriptorPreview {
    type Vtable = ISequenceVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cd8f292_98b2_4530_a1b6_2ded5fecbc26);
}
impl ::windows::core::RuntimeName for SequenceVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview";
}
impl ::core::convert::From<SequenceVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: SequenceVariableDescriptorPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SequenceVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: SequenceVariableDescriptorPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<SequenceVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: SequenceVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SequenceVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &SequenceVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `AI_MachineLearning_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TensorVariableDescriptorPreview(pub ::windows::core::IInspectable);
impl TensorVariableDescriptorPreview {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn DataType(&self) -> ::windows::core::Result<FeatureElementKindPreview> {
        let this = self;
        unsafe {
            let mut result__: FeatureElementKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FeatureElementKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterable<i64>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKindPreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `AI_MachineLearning_Preview`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TensorVariableDescriptorPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview;{a80f501a-9aac-4233-9784-aceaf92510b5})");
}
unsafe impl ::windows::core::Interface for TensorVariableDescriptorPreview {
    type Vtable = ITensorVariableDescriptorPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa80f501a_9aac_4233_9784_aceaf92510b5);
}
impl ::windows::core::RuntimeName for TensorVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview";
}
impl ::core::convert::From<TensorVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: TensorVariableDescriptorPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TensorVariableDescriptorPreview> for ::windows::core::IUnknown {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TensorVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: TensorVariableDescriptorPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TensorVariableDescriptorPreview> for ::windows::core::IInspectable {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<TensorVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorVariableDescriptorPreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
