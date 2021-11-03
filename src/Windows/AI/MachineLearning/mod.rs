#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "AI_MachineLearning_Preview")]
pub mod Preview;
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911574437, 5914, 18986, [152, 95, 38, 81, 89, 211, 137, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageFeatureDescriptor2 {
    type Vtable = IImageFeatureDescriptor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(724028583, 54579, 22626, [187, 152, 22, 17, 177, 85, 176, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LearningModelPixelRange) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageFeatureValue(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageFeatureValue {
    type Vtable = IImageFeatureValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4030812121, 51626, 17413, [183, 251, 148, 248, 124, 138, 48, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IImageFeatureValueStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageFeatureValueStatics {
    type Vtable = IImageFeatureValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(465770493, 9163, 17936, [176, 133, 200, 225, 200, 126, 186, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, image: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModel(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModel {
    type Vtable = ILearningModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1536051488, 18591, 20102, [145, 40, 38, 90, 50, 123, 120, 250]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelBinding(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelBinding {
    type Vtable = ILearningModelBinding_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3929091872, 5775, 20364, [148, 254, 46, 122, 195, 27, 74, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBinding_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::RawPtr, props: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelBindingFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelBindingFactory {
    type Vtable = ILearningModelBindingFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3378477690, 59272, 18270, [137, 23, 35, 170, 56, 31, 175, 11]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, session: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelDevice(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelDevice {
    type Vtable = ILearningModelDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4123183358, 16214, 19084, [172, 95, 253, 185, 45, 139, 130, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelDeviceFactory {
    type Vtable = ILearningModelDeviceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2634012493, 45541, 20256, [128, 173, 10, 86, 105, 13, 176, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicekind: LearningModelDeviceKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelDeviceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelDeviceStatics {
    type Vtable = ILearningModelDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1240670471, 43199, 17083, [146, 199, 16, 177, 45, 197, 210, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3002712013, 38414, 18880, [133, 147, 235, 25, 10, 227, 238, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `AI_MachineLearning`*"]
pub struct ILearningModelFeatureDescriptor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelFeatureDescriptor {
    type Vtable = ILearningModelFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3154694012, 28368, 16388, [151, 186, 185, 162, 238, 205, 43, 79]);
}
impl ILearningModelFeatureDescriptor {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn IsRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILearningModelFeatureDescriptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{bc08cf7c-6ed0-4004-97ba-b9a2eecd2b4f}");
}
impl ::std::convert::From<ILearningModelFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: ILearningModelFeatureDescriptor) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ILearningModelFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: &ILearningModelFeatureDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ILearningModelFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: ILearningModelFeatureDescriptor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILearningModelFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: &ILearningModelFeatureDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LearningModelFeatureKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `AI_MachineLearning`*"]
pub struct ILearningModelFeatureValue(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelFeatureValue {
    type Vtable = ILearningModelFeatureValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4111467995, 16517, 19966, [159, 237, 149, 235, 12, 12, 247, 92]);
}
impl ILearningModelFeatureValue {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILearningModelFeatureValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f51005db-4085-4dfe-9fed-95eb0c0cf75c}");
}
impl ::std::convert::From<ILearningModelFeatureValue> for ::windows::runtime::IUnknown {
    fn from(value: ILearningModelFeatureValue) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ILearningModelFeatureValue> for ::windows::runtime::IUnknown {
    fn from(value: &ILearningModelFeatureValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ILearningModelFeatureValue> for ::windows::runtime::IInspectable {
    fn from(value: ILearningModelFeatureValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILearningModelFeatureValue> for ::windows::runtime::IInspectable {
    fn from(value: &ILearningModelFeatureValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LearningModelFeatureKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `AI_MachineLearning`*"]
pub struct ILearningModelOperatorProvider(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelOperatorProvider {
    type Vtable = ILearningModelOperatorProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(706883165, 44977, 18413, [191, 173, 181, 179, 164, 89, 236, 4]);
}
impl ILearningModelOperatorProvider {}
unsafe impl ::windows::runtime::RuntimeType for ILearningModelOperatorProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2a222e5d-afb1-47ed-bfad-b5b3a459ec04}");
}
impl ::std::convert::From<ILearningModelOperatorProvider> for ::windows::runtime::IUnknown {
    fn from(value: ILearningModelOperatorProvider) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ILearningModelOperatorProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ILearningModelOperatorProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ILearningModelOperatorProvider> for ::windows::runtime::IInspectable {
    fn from(value: ILearningModelOperatorProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILearningModelOperatorProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ILearningModelOperatorProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelSession(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelSession {
    type Vtable = ILearningModelSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2388195574, 46983, 19473, [144, 240, 113, 41, 174, 202, 116, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bindings: ::windows::runtime::RawPtr, correlationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, features: ::windows::runtime::RawPtr, correlationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bindings: ::windows::runtime::RawPtr, correlationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, features: ::windows::runtime::RawPtr, correlationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelSessionFactory {
    type Vtable = ILearningModelSessionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(258705437, 7323, 18358, [191, 224, 241, 207, 98, 166, 117, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, model: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, model: ::windows::runtime::RawPtr, devicetorunon: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelSessionFactory2 {
    type Vtable = ILearningModelSessionFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1314687167, 2591, 24556, [173, 224, 47, 217, 30, 78, 242, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, model: ::windows::runtime::RawPtr, devicetorunon: ::windows::runtime::RawPtr, learningmodelsessionoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3103145889, 4941, 20787, [140, 255, 58, 92, 60, 38, 59, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelSessionOptions2 {
    type Vtable = ILearningModelSessionOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1875713476, 5983, 23506, [141, 229, 47, 32, 6, 162, 90, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelSessionOptions3 {
    type Vtable = ILearningModelSessionOptions3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1491164398, 55490, 22268, [146, 232, 118, 215, 81, 8, 16, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, dimension: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILearningModelStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILearningModelStatics {
    type Vtable = ILearningModelStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820582888, 26962, 20039, [142, 244, 31, 127, 7, 137, 124, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modelfile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modelstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modelstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modelfile: ::windows::runtime::RawPtr, operatorprovider: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modelstream: ::windows::runtime::RawPtr, operatorprovider: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, operatorprovider: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modelstream: ::windows::runtime::RawPtr, operatorprovider: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapFeatureDescriptor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1392780477, 41559, 17261, [158, 96, 194, 152, 31, 124, 197, 196]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapFeatureDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TensorKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISequenceFeatureDescriptor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2230752346, 22059, 19810, [168, 81, 115, 154, 206, 217, 102, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceFeatureDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `AI_MachineLearning`*"]
pub struct ITensor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensor {
    type Vtable = ITensor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(88642963, 41733, 18981, [173, 9, 68, 1, 25, 180, 183, 246]);
}
impl ITensor {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITensor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{05489593-a305-4a25-ad09-440119b4b7f6}");
}
impl ::std::convert::From<ITensor> for ::windows::runtime::IUnknown {
    fn from(value: ITensor) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ITensor> for ::windows::runtime::IUnknown {
    fn from(value: &ITensor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ITensor> for ::windows::runtime::IInspectable {
    fn from(value: ITensor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITensor> for ::windows::runtime::IInspectable {
    fn from(value: &ITensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ITensor> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ITensor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ITensor> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ITensor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for ITensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &ITensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TensorKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorBoolean(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorBoolean {
    type Vtable = ITensorBoolean_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1358107117, 10729, 19036, [164, 77, 143, 197, 18, 88, 78, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBoolean_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorBooleanStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorBooleanStatics {
    type Vtable = ITensorBooleanStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664176172, 9047, 18855, [180, 118, 208, 170, 61, 254, 104, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorBooleanStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorBooleanStatics2 {
    type Vtable = ITensorBooleanStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2745476353, 27181, 21207, [176, 75, 196, 53, 186, 238, 1, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorDouble(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorDouble {
    type Vtable = ITensorDouble_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2447643218, 31375, 20238, [162, 143, 150, 55, 255, 200, 163, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDouble_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorDoubleStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorDoubleStatics {
    type Vtable = ITensorDoubleStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2825294789, 38200, 17639, [163, 202, 93, 243, 116, 165, 167, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorDoubleStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorDoubleStatics2 {
    type Vtable = ITensorDoubleStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2477093086, 24218, 20628, [133, 200, 89, 44, 101, 94, 104, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorFeatureDescriptor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950702720, 37994, 17168, [161, 156, 238, 10, 240, 40, 252, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFeatureDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TensorKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorFloat(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorFloat {
    type Vtable = ITensorFloat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4062719362, 43522, 17096, [160, 200, 223, 30, 252, 150, 118, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorFloat16Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(179934460, 23433, 19516, [181, 228, 82, 130, 165, 49, 108, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorFloat16BitStatics {
    type Vtable = ITensorFloat16BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2771236597, 12682, 17620, [130, 11, 12, 220, 112, 84, 168, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorFloat16BitStatics2 {
    type Vtable = ITensorFloat16BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750357798, 11719, 20927, [180, 112, 11, 52, 76, 194, 161, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorFloatStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorFloatStatics {
    type Vtable = ITensorFloatStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3687659867, 15267, 17711, [177, 13, 60, 19, 94, 87, 63, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorFloatStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorFloatStatics2 {
    type Vtable = ITensorFloatStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(610339777, 24132, 22291, [178, 129, 143, 74, 212, 213, 85, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt16Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt16Bit {
    type Vtable = ITensorInt16Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2560830777, 59094, 17583, [138, 250, 186, 235, 196, 77, 192, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt16BitStatics {
    type Vtable = ITensorInt16BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2556715667, 9838, 19226, [130, 31, 230, 13, 112, 137, 139, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const i16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt16BitStatics2 {
    type Vtable = ITensorInt16BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(215420148, 26988, 24159, [149, 216, 94, 191, 150, 112, 20, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt32Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt32Bit {
    type Vtable = ITensorInt32Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(738994387, 8316, 17542, [167, 210, 136, 69, 34, 197, 229, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt32BitStatics {
    type Vtable = ITensorInt32BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1698268747, 21242, 20021, [144, 124, 131, 76, 172, 65, 123, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt32BitStatics2 {
    type Vtable = ITensorInt32BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2085291930, 59734, 23776, [163, 189, 21, 125, 157, 121, 181, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt64Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt64Bit {
    type Vtable = ITensorInt64Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1234593210, 8098, 17837, [175, 37, 160, 189, 155, 218, 76, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt64BitStatics {
    type Vtable = ITensorInt64BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2521345437, 4504, 19828, [149, 23, 120, 58, 182, 43, 156, 194]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const i64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt64BitStatics2 {
    type Vtable = ITensorInt64BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1832754635, 65344, 24258, [137, 254, 8, 78, 43, 107, 198, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt8Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt8Bit {
    type Vtable = ITensorInt8Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3453851589, 65496, 20463, [174, 251, 48, 225, 164, 133, 178, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt8BitStatics {
    type Vtable = ITensorInt8BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2980127364, 2396, 19574, [166, 97, 172, 76, 238, 31, 62, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorInt8BitStatics2 {
    type Vtable = ITensorInt8BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3235223095, 50280, 22267, [149, 53, 192, 82, 189, 185, 61, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorString(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorString {
    type Vtable = ITensorString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1478702536, 48561, 17936, [188, 117, 53, 233, 203, 240, 9, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorString_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorStringStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorStringStatics {
    type Vtable = ITensorStringStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2204250916, 53030, 20247, [162, 212, 32, 239, 141, 9, 125, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorStringStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorStringStatics2 {
    type Vtable = ITensorStringStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654297808, 51426, 21076, [145, 55, 1, 147, 163, 102, 143, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt16Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1746145099, 9152, 17139, [129, 246, 168, 145, 192, 17, 188, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt16BitStatics {
    type Vtable = ITensorUInt16BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1576486365, 650, 18458, [162, 124, 199, 230, 67, 94, 82, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt16BitStatics2 {
    type Vtable = ITensorUInt16BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2331249764, 54943, 21269, [147, 72, 73, 8, 119, 187, 214, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt32Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3637101311, 29969, 17827, [191, 172, 195, 143, 55, 13, 34, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt32BitStatics {
    type Vtable = ITensorUInt32BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1098659895, 59251, 17272, [142, 127, 12, 195, 61, 190, 166, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt32BitStatics2 {
    type Vtable = ITensorUInt32BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4011466524, 12622, 22173, [180, 150, 92, 132, 71, 210, 12, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt64Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(779157421, 1215, 18469, [131, 154, 130, 186, 239, 140, 120, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt64BitStatics {
    type Vtable = ITensorUInt64BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2055086315, 9263, 18379, [169, 198, 246, 2, 236, 251, 254, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt64BitStatics2 {
    type Vtable = ITensorUInt64BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(140142717, 26593, 23326, [178, 50, 79, 171, 233, 202, 32, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt8Bit(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1491185191, 25131, 18659, [190, 34, 216, 103, 174, 209, 218, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8Bit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt8BitStatics {
    type Vtable = ITensorUInt8BitStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(100038019, 48164, 16928, [138, 65, 45, 205, 140, 94, 211, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data_array_size: u32, data: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITensorUInt8BitStatics2 {
    type Vtable = ITensorUInt8BitStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(731923158, 14142, 23098, [162, 252, 166, 196, 27, 213, 39, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shape_array_size: u32, shape: *const i64, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ImageFeatureDescriptor(pub ::windows::runtime::IInspectable);
impl ImageFeatureDescriptor {
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `AI_MachineLearning`, `Graphics_Imaging`*"]
    pub fn BitmapPixelFormat(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapPixelFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `AI_MachineLearning`, `Graphics_Imaging`*"]
    pub fn BitmapAlphaMode(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapAlphaMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn IsRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn PixelRange(&self) -> ::windows::runtime::Result<LearningModelPixelRange> {
        let this = &::windows::runtime::Interface::cast::<IImageFeatureDescriptor2>(self)?;
        unsafe {
            let mut result__: LearningModelPixelRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelPixelRange>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageFeatureDescriptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureDescriptor;{365585a5-171a-4a2a-985f-265159d3895a})");
}
unsafe impl ::windows::runtime::Interface for ImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911574437, 5914, 18986, [152, 95, 38, 81, 89, 211, 137, 90]);
}
impl ::windows::runtime::RuntimeName for ImageFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureDescriptor";
}
impl ::std::convert::From<ImageFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: ImageFeatureDescriptor) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ImageFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: &ImageFeatureDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ImageFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: ImageFeatureDescriptor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ImageFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: &ImageFeatureDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ImageFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ImageFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ImageFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ImageFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for &ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::std::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ImageFeatureDescriptor {}
unsafe impl ::std::marker::Sync for ImageFeatureDescriptor {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ImageFeatureValue(pub ::windows::runtime::IInspectable);
impl ImageFeatureValue {
    #[cfg(feature = "Media")]
    #[doc = "*Required features: `AI_MachineLearning`, `Media`*"]
    pub fn VideoFrame(&self) -> ::windows::runtime::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::VideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[cfg(feature = "Media")]
    #[doc = "*Required features: `AI_MachineLearning`, `Media`*"]
    pub fn CreateFromVideoFrame<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Media::VideoFrame>>(image: Param0) -> ::windows::runtime::Result<ImageFeatureValue> {
        Self::IImageFeatureValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), image.into_param().abi(), &mut result__).from_abi::<ImageFeatureValue>(result__)
        })
    }
    pub fn IImageFeatureValueStatics<R, F: FnOnce(&IImageFeatureValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ImageFeatureValue, IImageFeatureValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageFeatureValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureValue;{f0414fd9-c9aa-4405-b7fb-94f87c8a3037})");
}
unsafe impl ::windows::runtime::Interface for ImageFeatureValue {
    type Vtable = IImageFeatureValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4030812121, 51626, 17413, [183, 251, 148, 248, 124, 138, 48, 55]);
}
impl ::windows::runtime::RuntimeName for ImageFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureValue";
}
impl ::std::convert::From<ImageFeatureValue> for ::windows::runtime::IUnknown {
    fn from(value: ImageFeatureValue) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ImageFeatureValue> for ::windows::runtime::IUnknown {
    fn from(value: &ImageFeatureValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ImageFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ImageFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ImageFeatureValue> for ::windows::runtime::IInspectable {
    fn from(value: ImageFeatureValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ImageFeatureValue> for ::windows::runtime::IInspectable {
    fn from(value: &ImageFeatureValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ImageFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ImageFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ImageFeatureValue> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ImageFeatureValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ImageFeatureValue> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ImageFeatureValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for ImageFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &ImageFeatureValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ImageFeatureValue {}
unsafe impl ::std::marker::Sync for ImageFeatureValue {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LearningModel(pub ::windows::runtime::IInspectable);
impl LearningModel {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Author(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Domain(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Metadata(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn InputFeatures(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn OutputFeatures(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`, `Storage`*"]
    pub fn LoadFromStorageFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(modelfile: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), modelfile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadFromStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(modelstream: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), modelstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn LoadFromFilePath<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(filepath: Param0) -> ::windows::runtime::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn LoadFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(modelstream: Param0) -> ::windows::runtime::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), modelstream.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`, `Storage`*"]
    pub fn LoadFromStorageFileWithOperatorProviderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, ILearningModelOperatorProvider>>(modelfile: Param0, operatorprovider: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), modelfile.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadFromStreamWithOperatorProviderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows::runtime::IntoParam<'a, ILearningModelOperatorProvider>>(modelstream: Param0, operatorprovider: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), modelstream.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn LoadFromFilePathWithOperatorProvider<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ILearningModelOperatorProvider>>(filepath: Param0, operatorprovider: Param1) -> ::windows::runtime::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), filepath.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn LoadFromStreamWithOperatorProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows::runtime::IntoParam<'a, ILearningModelOperatorProvider>>(modelstream: Param0, operatorprovider: Param1) -> ::windows::runtime::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), modelstream.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    pub fn ILearningModelStatics<R, F: FnOnce(&ILearningModelStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LearningModel, ILearningModelStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LearningModel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModel;{5b8e4920-489f-4e86-9128-265a327b78fa})");
}
unsafe impl ::windows::runtime::Interface for LearningModel {
    type Vtable = ILearningModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1536051488, 18591, 20102, [145, 40, 38, 90, 50, 123, 120, 250]);
}
impl ::windows::runtime::RuntimeName for LearningModel {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModel";
}
impl ::std::convert::From<LearningModel> for ::windows::runtime::IUnknown {
    fn from(value: LearningModel) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LearningModel> for ::windows::runtime::IUnknown {
    fn from(value: &LearningModel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LearningModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LearningModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LearningModel> for ::windows::runtime::IInspectable {
    fn from(value: LearningModel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LearningModel> for ::windows::runtime::IInspectable {
    fn from(value: &LearningModel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LearningModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LearningModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<LearningModel> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LearningModel) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&LearningModel> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LearningModel) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for LearningModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &LearningModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LearningModel {}
unsafe impl ::std::marker::Sync for LearningModel {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LearningModelBinding(pub ::windows::runtime::IInspectable);
impl LearningModelBinding {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Bind<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn BindWithProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, name: Param0, value: Param1, props: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), props.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::std::option::Option<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>, second: &mut ::std::option::Option<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromSession<'a, Param0: ::windows::runtime::IntoParam<'a, LearningModelSession>>(session: Param0) -> ::windows::runtime::Result<LearningModelBinding> {
        Self::ILearningModelBindingFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), session.into_param().abi(), &mut result__).from_abi::<LearningModelBinding>(result__)
        })
    }
    pub fn ILearningModelBindingFactory<R, F: FnOnce(&ILearningModelBindingFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LearningModelBinding, ILearningModelBindingFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelBinding {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelBinding;{ea312f20-168f-4f8c-94fe-2e7ac31b4aa8})");
}
unsafe impl ::windows::runtime::Interface for LearningModelBinding {
    type Vtable = ILearningModelBinding_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3929091872, 5775, 20364, [148, 254, 46, 122, 195, 27, 74, 168]);
}
impl ::windows::runtime::RuntimeName for LearningModelBinding {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelBinding";
}
impl ::std::convert::From<LearningModelBinding> for ::windows::runtime::IUnknown {
    fn from(value: LearningModelBinding) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LearningModelBinding> for ::windows::runtime::IUnknown {
    fn from(value: &LearningModelBinding) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LearningModelBinding> for ::windows::runtime::IInspectable {
    fn from(value: LearningModelBinding) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LearningModelBinding> for ::windows::runtime::IInspectable {
    fn from(value: &LearningModelBinding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<LearningModelBinding> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LearningModelBinding) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&LearningModelBinding> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LearningModelBinding) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<LearningModelBinding> for super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LearningModelBinding) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&LearningModelBinding> for super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LearningModelBinding) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &LearningModelBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LearningModelBinding {}
unsafe impl ::std::marker::Sync for LearningModelBinding {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LearningModelDevice(pub ::windows::runtime::IInspectable);
impl LearningModelDevice {
    #[cfg(feature = "Graphics")]
    #[doc = "*Required features: `AI_MachineLearning`, `Graphics`*"]
    pub fn AdapterId(&self) -> ::windows::runtime::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::DisplayAdapterId = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `AI_MachineLearning`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Direct3D11Device(&self) -> ::windows::runtime::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create(devicekind: LearningModelDeviceKind) -> ::windows::runtime::Result<LearningModelDevice> {
        Self::ILearningModelDeviceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), devicekind, &mut result__).from_abi::<LearningModelDevice>(result__)
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `AI_MachineLearning`, `Graphics_DirectX_Direct3D11`*"]
    pub fn CreateFromDirect3D11Device<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(device: Param0) -> ::windows::runtime::Result<LearningModelDevice> {
        Self::ILearningModelDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), device.into_param().abi(), &mut result__).from_abi::<LearningModelDevice>(result__)
        })
    }
    pub fn ILearningModelDeviceFactory<R, F: FnOnce(&ILearningModelDeviceFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LearningModelDevice, ILearningModelDeviceFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILearningModelDeviceStatics<R, F: FnOnce(&ILearningModelDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LearningModelDevice, ILearningModelDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelDevice;{f5c2c8fe-3f56-4a8c-ac5f-fdb92d8b8252})");
}
unsafe impl ::windows::runtime::Interface for LearningModelDevice {
    type Vtable = ILearningModelDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4123183358, 16214, 19084, [172, 95, 253, 185, 45, 139, 130, 82]);
}
impl ::windows::runtime::RuntimeName for LearningModelDevice {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelDevice";
}
impl ::std::convert::From<LearningModelDevice> for ::windows::runtime::IUnknown {
    fn from(value: LearningModelDevice) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LearningModelDevice> for ::windows::runtime::IUnknown {
    fn from(value: &LearningModelDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LearningModelDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LearningModelDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LearningModelDevice> for ::windows::runtime::IInspectable {
    fn from(value: LearningModelDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LearningModelDevice> for ::windows::runtime::IInspectable {
    fn from(value: &LearningModelDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LearningModelDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LearningModelDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LearningModelDevice {}
unsafe impl ::std::marker::Sync for LearningModelDevice {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LearningModelDeviceKind(pub i32);
impl LearningModelDeviceKind {
    pub const Default: LearningModelDeviceKind = LearningModelDeviceKind(0i32);
    pub const Cpu: LearningModelDeviceKind = LearningModelDeviceKind(1i32);
    pub const DirectX: LearningModelDeviceKind = LearningModelDeviceKind(2i32);
    pub const DirectXHighPerformance: LearningModelDeviceKind = LearningModelDeviceKind(3i32);
    pub const DirectXMinPower: LearningModelDeviceKind = LearningModelDeviceKind(4i32);
}
impl ::std::convert::From<i32> for LearningModelDeviceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LearningModelDeviceKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelDeviceKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelDeviceKind;i4)");
}
impl ::windows::runtime::DefaultType for LearningModelDeviceKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LearningModelEvaluationResult(pub ::windows::runtime::IInspectable);
impl LearningModelEvaluationResult {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CorrelationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn ErrorStatus(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Outputs(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelEvaluationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelEvaluationResult;{b2f9bfcd-960e-49c0-8593-eb190ae3eee2})");
}
unsafe impl ::windows::runtime::Interface for LearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3002712013, 38414, 18880, [133, 147, 235, 25, 10, 227, 238, 226]);
}
impl ::windows::runtime::RuntimeName for LearningModelEvaluationResult {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelEvaluationResult";
}
impl ::std::convert::From<LearningModelEvaluationResult> for ::windows::runtime::IUnknown {
    fn from(value: LearningModelEvaluationResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LearningModelEvaluationResult> for ::windows::runtime::IUnknown {
    fn from(value: &LearningModelEvaluationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LearningModelEvaluationResult> for ::windows::runtime::IInspectable {
    fn from(value: LearningModelEvaluationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LearningModelEvaluationResult> for ::windows::runtime::IInspectable {
    fn from(value: &LearningModelEvaluationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LearningModelEvaluationResult {}
unsafe impl ::std::marker::Sync for LearningModelEvaluationResult {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LearningModelFeatureKind(pub i32);
impl LearningModelFeatureKind {
    pub const Tensor: LearningModelFeatureKind = LearningModelFeatureKind(0i32);
    pub const Sequence: LearningModelFeatureKind = LearningModelFeatureKind(1i32);
    pub const Map: LearningModelFeatureKind = LearningModelFeatureKind(2i32);
    pub const Image: LearningModelFeatureKind = LearningModelFeatureKind(3i32);
}
impl ::std::convert::From<i32> for LearningModelFeatureKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LearningModelFeatureKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelFeatureKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelFeatureKind;i4)");
}
impl ::windows::runtime::DefaultType for LearningModelFeatureKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LearningModelPixelRange(pub i32);
impl LearningModelPixelRange {
    pub const ZeroTo255: LearningModelPixelRange = LearningModelPixelRange(0i32);
    pub const ZeroToOne: LearningModelPixelRange = LearningModelPixelRange(1i32);
    pub const MinusOneToOne: LearningModelPixelRange = LearningModelPixelRange(2i32);
}
impl ::std::convert::From<i32> for LearningModelPixelRange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LearningModelPixelRange {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelPixelRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelPixelRange;i4)");
}
impl ::windows::runtime::DefaultType for LearningModelPixelRange {
    type DefaultType = Self;
}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LearningModelSession(pub ::windows::runtime::IInspectable);
impl LearningModelSession {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Model(&self) -> ::windows::runtime::Result<LearningModel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModel>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Device(&self) -> ::windows::runtime::Result<LearningModelDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelDevice>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn EvaluationProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn EvaluateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, LearningModelBinding>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, bindings: Param0, correlationid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bindings.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`, `Foundation_Collections`*"]
    pub fn EvaluateFeaturesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, features: Param0, correlationid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), features.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Evaluate<'a, Param0: ::windows::runtime::IntoParam<'a, LearningModelBinding>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, bindings: Param0, correlationid: Param1) -> ::windows::runtime::Result<LearningModelEvaluationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), bindings.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<LearningModelEvaluationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn EvaluateFeatures<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, features: Param0, correlationid: Param1) -> ::windows::runtime::Result<LearningModelEvaluationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), features.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<LearningModelEvaluationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromModel<'a, Param0: ::windows::runtime::IntoParam<'a, LearningModel>>(model: Param0) -> ::windows::runtime::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), model.into_param().abi(), &mut result__).from_abi::<LearningModelSession>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromModelOnDevice<'a, Param0: ::windows::runtime::IntoParam<'a, LearningModel>, Param1: ::windows::runtime::IntoParam<'a, LearningModelDevice>>(model: Param0, devicetorunon: Param1) -> ::windows::runtime::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), model.into_param().abi(), devicetorunon.into_param().abi(), &mut result__).from_abi::<LearningModelSession>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromModelOnDeviceWithSessionOptions<'a, Param0: ::windows::runtime::IntoParam<'a, LearningModel>, Param1: ::windows::runtime::IntoParam<'a, LearningModelDevice>, Param2: ::windows::runtime::IntoParam<'a, LearningModelSessionOptions>>(model: Param0, devicetorunon: Param1, learningmodelsessionoptions: Param2) -> ::windows::runtime::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), model.into_param().abi(), devicetorunon.into_param().abi(), learningmodelsessionoptions.into_param().abi(), &mut result__).from_abi::<LearningModelSession>(result__)
        })
    }
    pub fn ILearningModelSessionFactory<R, F: FnOnce(&ILearningModelSessionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LearningModelSession, ILearningModelSessionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILearningModelSessionFactory2<R, F: FnOnce(&ILearningModelSessionFactory2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LearningModelSession, ILearningModelSessionFactory2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSession;{8e58f8f6-b787-4c11-90f0-7129aeca74a9})");
}
unsafe impl ::windows::runtime::Interface for LearningModelSession {
    type Vtable = ILearningModelSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2388195574, 46983, 19473, [144, 240, 113, 41, 174, 202, 116, 169]);
}
impl ::windows::runtime::RuntimeName for LearningModelSession {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSession";
}
impl ::std::convert::From<LearningModelSession> for ::windows::runtime::IUnknown {
    fn from(value: LearningModelSession) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LearningModelSession> for ::windows::runtime::IUnknown {
    fn from(value: &LearningModelSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LearningModelSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LearningModelSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LearningModelSession> for ::windows::runtime::IInspectable {
    fn from(value: LearningModelSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LearningModelSession> for ::windows::runtime::IInspectable {
    fn from(value: &LearningModelSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LearningModelSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LearningModelSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<LearningModelSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LearningModelSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&LearningModelSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LearningModelSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for LearningModelSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &LearningModelSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LearningModelSession {}
unsafe impl ::std::marker::Sync for LearningModelSession {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LearningModelSessionOptions(pub ::windows::runtime::IInspectable);
impl LearningModelSessionOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LearningModelSessionOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn BatchSizeOverride(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn SetBatchSizeOverride(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CloseModelOnSessionCreation(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn SetCloseModelOnSessionCreation(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn OverrideNamedDimension<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, dimension: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelSessionOptions3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), dimension).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LearningModelSessionOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSessionOptions;{b8f63fa1-134d-5133-8cff-3a5c3c263beb})");
}
unsafe impl ::windows::runtime::Interface for LearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3103145889, 4941, 20787, [140, 255, 58, 92, 60, 38, 59, 235]);
}
impl ::windows::runtime::RuntimeName for LearningModelSessionOptions {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSessionOptions";
}
impl ::std::convert::From<LearningModelSessionOptions> for ::windows::runtime::IUnknown {
    fn from(value: LearningModelSessionOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LearningModelSessionOptions> for ::windows::runtime::IUnknown {
    fn from(value: &LearningModelSessionOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LearningModelSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LearningModelSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LearningModelSessionOptions> for ::windows::runtime::IInspectable {
    fn from(value: LearningModelSessionOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LearningModelSessionOptions> for ::windows::runtime::IInspectable {
    fn from(value: &LearningModelSessionOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LearningModelSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LearningModelSessionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LearningModelSessionOptions {}
unsafe impl ::std::marker::Sync for LearningModelSessionOptions {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct MachineLearningContract(pub u8);
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MapFeatureDescriptor(pub ::windows::runtime::IInspectable);
impl MapFeatureDescriptor {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn KeyKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn ValueDescriptor(&self) -> ::windows::runtime::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ILearningModelFeatureDescriptor>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn IsRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapFeatureDescriptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.MapFeatureDescriptor;{530424bd-a257-436d-9e60-c2981f7cc5c4})");
}
unsafe impl ::windows::runtime::Interface for MapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1392780477, 41559, 17261, [158, 96, 194, 152, 31, 124, 197, 196]);
}
impl ::windows::runtime::RuntimeName for MapFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.MapFeatureDescriptor";
}
impl ::std::convert::From<MapFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: MapFeatureDescriptor) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&MapFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: &MapFeatureDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<MapFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: MapFeatureDescriptor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MapFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: &MapFeatureDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MapFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MapFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MapFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MapFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for MapFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for &MapFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::std::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MapFeatureDescriptor {}
unsafe impl ::std::marker::Sync for MapFeatureDescriptor {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SequenceFeatureDescriptor(pub ::windows::runtime::IInspectable);
impl SequenceFeatureDescriptor {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn ElementDescriptor(&self) -> ::windows::runtime::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ILearningModelFeatureDescriptor>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn IsRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SequenceFeatureDescriptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.SequenceFeatureDescriptor;{84f6945a-562b-4d62-a851-739aced96668})");
}
unsafe impl ::windows::runtime::Interface for SequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2230752346, 22059, 19810, [168, 81, 115, 154, 206, 217, 102, 104]);
}
impl ::windows::runtime::RuntimeName for SequenceFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.SequenceFeatureDescriptor";
}
impl ::std::convert::From<SequenceFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: SequenceFeatureDescriptor) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SequenceFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: &SequenceFeatureDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SequenceFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: SequenceFeatureDescriptor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SequenceFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: &SequenceFeatureDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SequenceFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SequenceFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SequenceFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SequenceFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for &SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::std::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SequenceFeatureDescriptor {}
unsafe impl ::std::marker::Sync for SequenceFeatureDescriptor {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorBoolean(pub ::windows::runtime::IInspectable);
impl TensorBoolean {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<bool>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<bool as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<bool>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<bool as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    pub fn ITensorBooleanStatics<R, F: FnOnce(&ITensorBooleanStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorBoolean, ITensorBooleanStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorBooleanStatics2<R, F: FnOnce(&ITensorBooleanStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorBoolean, ITensorBooleanStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorBoolean {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorBoolean;{50f311ed-29e9-4a5c-a44d-8fc512584eed})");
}
unsafe impl ::windows::runtime::Interface for TensorBoolean {
    type Vtable = ITensorBoolean_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1358107117, 10729, 19036, [164, 77, 143, 197, 18, 88, 78, 237]);
}
impl ::windows::runtime::RuntimeName for TensorBoolean {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorBoolean";
}
impl ::std::convert::From<TensorBoolean> for ::windows::runtime::IUnknown {
    fn from(value: TensorBoolean) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorBoolean> for ::windows::runtime::IUnknown {
    fn from(value: &TensorBoolean) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorBoolean> for ::windows::runtime::IInspectable {
    fn from(value: TensorBoolean) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorBoolean> for ::windows::runtime::IInspectable {
    fn from(value: &TensorBoolean) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorBoolean> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorBoolean> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorBoolean> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorBoolean> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorBoolean> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorBoolean> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorBoolean> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorBoolean> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorBoolean {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorBoolean {}
unsafe impl ::std::marker::Sync for TensorBoolean {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorDouble(pub ::windows::runtime::IInspectable);
impl TensorDouble {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f64>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<f64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<f64>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<f64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    pub fn ITensorDoubleStatics<R, F: FnOnce(&ITensorDoubleStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorDouble, ITensorDoubleStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorDoubleStatics2<R, F: FnOnce(&ITensorDoubleStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorDouble, ITensorDoubleStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorDouble {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorDouble;{91e41252-7a8f-4f0e-a28f-9637ffc8a3d0})");
}
unsafe impl ::windows::runtime::Interface for TensorDouble {
    type Vtable = ITensorDouble_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2447643218, 31375, 20238, [162, 143, 150, 55, 255, 200, 163, 208]);
}
impl ::windows::runtime::RuntimeName for TensorDouble {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorDouble";
}
impl ::std::convert::From<TensorDouble> for ::windows::runtime::IUnknown {
    fn from(value: TensorDouble) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorDouble> for ::windows::runtime::IUnknown {
    fn from(value: &TensorDouble) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorDouble> for ::windows::runtime::IInspectable {
    fn from(value: TensorDouble) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorDouble> for ::windows::runtime::IInspectable {
    fn from(value: &TensorDouble) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorDouble> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorDouble) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorDouble> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorDouble) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorDouble> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorDouble) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorDouble> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorDouble) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorDouble> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorDouble) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorDouble> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorDouble) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorDouble> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorDouble) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorDouble> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorDouble) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorDouble {}
unsafe impl ::std::marker::Sync for TensorDouble {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorFeatureDescriptor(pub ::windows::runtime::IInspectable);
impl TensorFeatureDescriptor {
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn IsRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorFeatureDescriptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFeatureDescriptor;{74455c80-946a-4310-a19c-ee0af028fce4})");
}
unsafe impl ::windows::runtime::Interface for TensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950702720, 37994, 17168, [161, 156, 238, 10, 240, 40, 252, 228]);
}
impl ::windows::runtime::RuntimeName for TensorFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFeatureDescriptor";
}
impl ::std::convert::From<TensorFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: TensorFeatureDescriptor) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorFeatureDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: &TensorFeatureDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: TensorFeatureDescriptor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorFeatureDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: &TensorFeatureDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFeatureDescriptor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureDescriptor> for &TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureDescriptor> {
        ::std::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorFeatureDescriptor {}
unsafe impl ::std::marker::Sync for TensorFeatureDescriptor {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorFloat(pub ::windows::runtime::IInspectable);
impl TensorFloat {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<f32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<f32>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<f32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    pub fn ITensorFloatStatics<R, F: FnOnce(&ITensorFloatStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorFloat, ITensorFloatStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorFloatStatics2<R, F: FnOnce(&ITensorFloatStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorFloat, ITensorFloatStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorFloat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat;{f2282d82-aa02-42c8-a0c8-df1efc9676e1})");
}
unsafe impl ::windows::runtime::Interface for TensorFloat {
    type Vtable = ITensorFloat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4062719362, 43522, 17096, [160, 200, 223, 30, 252, 150, 118, 225]);
}
impl ::windows::runtime::RuntimeName for TensorFloat {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat";
}
impl ::std::convert::From<TensorFloat> for ::windows::runtime::IUnknown {
    fn from(value: TensorFloat) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorFloat> for ::windows::runtime::IUnknown {
    fn from(value: &TensorFloat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorFloat> for ::windows::runtime::IInspectable {
    fn from(value: TensorFloat) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorFloat> for ::windows::runtime::IInspectable {
    fn from(value: &TensorFloat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorFloat> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorFloat> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorFloat> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorFloat> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorFloat> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorFloat> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorFloat> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorFloat> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorFloat {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorFloat {}
unsafe impl ::std::marker::Sync for TensorFloat {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorFloat16Bit(pub ::windows::runtime::IInspectable);
impl TensorFloat16Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<f32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<f32>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<f32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    pub fn ITensorFloat16BitStatics<R, F: FnOnce(&ITensorFloat16BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorFloat16BitStatics2<R, F: FnOnce(&ITensorFloat16BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorFloat16Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat16Bit;{0ab994fc-5b89-4c3c-b5e4-5282a5316c0a})");
}
unsafe impl ::windows::runtime::Interface for TensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(179934460, 23433, 19516, [181, 228, 82, 130, 165, 49, 108, 10]);
}
impl ::windows::runtime::RuntimeName for TensorFloat16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat16Bit";
}
impl ::std::convert::From<TensorFloat16Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorFloat16Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorFloat16Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorFloat16Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorFloat16Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorFloat16Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorFloat16Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorFloat16Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorFloat16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorFloat16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorFloat16Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorFloat16Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorFloat16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorFloat16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorFloat16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorFloat16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorFloat16Bit {}
unsafe impl ::std::marker::Sync for TensorFloat16Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorInt16Bit(pub ::windows::runtime::IInspectable);
impl TensorInt16Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i16>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<i16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i16>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<i16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    pub fn ITensorInt16BitStatics<R, F: FnOnce(&ITensorInt16BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorInt16BitStatics2<R, F: FnOnce(&ITensorInt16BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorInt16Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt16Bit;{98a32d39-e6d6-44af-8afa-baebc44dc020})");
}
unsafe impl ::windows::runtime::Interface for TensorInt16Bit {
    type Vtable = ITensorInt16Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2560830777, 59094, 17583, [138, 250, 186, 235, 196, 77, 192, 32]);
}
impl ::windows::runtime::RuntimeName for TensorInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt16Bit";
}
impl ::std::convert::From<TensorInt16Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorInt16Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorInt16Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorInt16Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorInt16Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorInt16Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorInt16Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorInt16Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorInt16Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt16Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorInt16Bit {}
unsafe impl ::std::marker::Sync for TensorInt16Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorInt32Bit(pub ::windows::runtime::IInspectable);
impl TensorInt32Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i32>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    pub fn ITensorInt32BitStatics<R, F: FnOnce(&ITensorInt32BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorInt32BitStatics2<R, F: FnOnce(&ITensorInt32BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorInt32Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt32Bit;{2c0c28d3-207c-4486-a7d2-884522c5e589})");
}
unsafe impl ::windows::runtime::Interface for TensorInt32Bit {
    type Vtable = ITensorInt32Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(738994387, 8316, 17542, [167, 210, 136, 69, 34, 197, 229, 137]);
}
impl ::windows::runtime::RuntimeName for TensorInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt32Bit";
}
impl ::std::convert::From<TensorInt32Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorInt32Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorInt32Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorInt32Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorInt32Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorInt32Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorInt32Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorInt32Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorInt32Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt32Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorInt32Bit {}
unsafe impl ::std::marker::Sync for TensorInt32Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorInt64Bit(pub ::windows::runtime::IInspectable);
impl TensorInt64Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<i64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<i64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    pub fn ITensorInt64BitStatics<R, F: FnOnce(&ITensorInt64BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorInt64BitStatics2<R, F: FnOnce(&ITensorInt64BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorInt64Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt64Bit;{499665ba-1fa2-45ad-af25-a0bd9bda4c87})");
}
unsafe impl ::windows::runtime::Interface for TensorInt64Bit {
    type Vtable = ITensorInt64Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1234593210, 8098, 17837, [175, 37, 160, 189, 155, 218, 76, 135]);
}
impl ::windows::runtime::RuntimeName for TensorInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt64Bit";
}
impl ::std::convert::From<TensorInt64Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorInt64Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorInt64Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorInt64Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorInt64Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorInt64Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorInt64Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorInt64Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorInt64Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt64Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorInt64Bit {}
unsafe impl ::std::marker::Sync for TensorInt64Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorInt8Bit(pub ::windows::runtime::IInspectable);
impl TensorInt8Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u8>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u8>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    pub fn ITensorInt8BitStatics<R, F: FnOnce(&ITensorInt8BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorInt8BitStatics2<R, F: FnOnce(&ITensorInt8BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorInt8Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt8Bit;{cddd97c5-ffd8-4fef-aefb-30e1a485b2ee})");
}
unsafe impl ::windows::runtime::Interface for TensorInt8Bit {
    type Vtable = ITensorInt8Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3453851589, 65496, 20463, [174, 251, 48, 225, 164, 133, 178, 238]);
}
impl ::windows::runtime::RuntimeName for TensorInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt8Bit";
}
impl ::std::convert::From<TensorInt8Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorInt8Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorInt8Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorInt8Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorInt8Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorInt8Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorInt8Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorInt8Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorInt8Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorInt8Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorInt8Bit {}
unsafe impl ::std::marker::Sync for TensorInt8Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TensorKind(pub i32);
impl TensorKind {
    pub const Undefined: TensorKind = TensorKind(0i32);
    pub const Float: TensorKind = TensorKind(1i32);
    pub const UInt8: TensorKind = TensorKind(2i32);
    pub const Int8: TensorKind = TensorKind(3i32);
    pub const UInt16: TensorKind = TensorKind(4i32);
    pub const Int16: TensorKind = TensorKind(5i32);
    pub const Int32: TensorKind = TensorKind(6i32);
    pub const Int64: TensorKind = TensorKind(7i32);
    pub const String: TensorKind = TensorKind(8i32);
    pub const Boolean: TensorKind = TensorKind(9i32);
    pub const Float16: TensorKind = TensorKind(10i32);
    pub const Double: TensorKind = TensorKind(11i32);
    pub const UInt32: TensorKind = TensorKind(12i32);
    pub const UInt64: TensorKind = TensorKind(13i32);
    pub const Complex64: TensorKind = TensorKind(14i32);
    pub const Complex128: TensorKind = TensorKind(15i32);
}
impl ::std::convert::From<i32> for TensorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TensorKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TensorKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.TensorKind;i4)");
}
impl ::windows::runtime::DefaultType for TensorKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorString(pub ::windows::runtime::IInspectable);
impl TensorString {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorString> {
        Self::ITensorStringStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    pub fn ITensorStringStatics<R, F: FnOnce(&ITensorStringStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorString, ITensorStringStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorStringStatics2<R, F: FnOnce(&ITensorStringStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorString, ITensorStringStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorString {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorString;{582335c8-bdb1-4610-bc75-35e9cbf009b7})");
}
unsafe impl ::windows::runtime::Interface for TensorString {
    type Vtable = ITensorString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1478702536, 48561, 17936, [188, 117, 53, 233, 203, 240, 9, 183]);
}
impl ::windows::runtime::RuntimeName for TensorString {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorString";
}
impl ::std::convert::From<TensorString> for ::windows::runtime::IUnknown {
    fn from(value: TensorString) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorString> for ::windows::runtime::IUnknown {
    fn from(value: &TensorString) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorString> for ::windows::runtime::IInspectable {
    fn from(value: TensorString) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorString> for ::windows::runtime::IInspectable {
    fn from(value: &TensorString) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorString> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorString) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorString> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorString) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorString> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorString) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorString> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorString) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorString> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorString) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorString> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorString) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorString> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorString) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorString> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorString) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorString {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorString {}
unsafe impl ::std::marker::Sync for TensorString {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorUInt16Bit(pub ::windows::runtime::IInspectable);
impl TensorUInt16Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u16>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u16>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    pub fn ITensorUInt16BitStatics<R, F: FnOnce(&ITensorUInt16BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorUInt16BitStatics2<R, F: FnOnce(&ITensorUInt16BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorUInt16Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt16Bit;{68140f4b-23c0-42f3-81f6-a891c011bc3f})");
}
unsafe impl ::windows::runtime::Interface for TensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1746145099, 9152, 17139, [129, 246, 168, 145, 192, 17, 188, 63]);
}
impl ::windows::runtime::RuntimeName for TensorUInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt16Bit";
}
impl ::std::convert::From<TensorUInt16Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorUInt16Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorUInt16Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorUInt16Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorUInt16Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorUInt16Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorUInt16Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorUInt16Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorUInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorUInt16Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt16Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorUInt16Bit {}
unsafe impl ::std::marker::Sync for TensorUInt16Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorUInt32Bit(pub ::windows::runtime::IInspectable);
impl TensorUInt32Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<u32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<u32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    pub fn ITensorUInt32BitStatics<R, F: FnOnce(&ITensorUInt32BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorUInt32BitStatics2<R, F: FnOnce(&ITensorUInt32BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorUInt32Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt32Bit;{d8c9c2ff-7511-45a3-bfac-c38f370d2237})");
}
unsafe impl ::windows::runtime::Interface for TensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3637101311, 29969, 17827, [191, 172, 195, 143, 55, 13, 34, 55]);
}
impl ::windows::runtime::RuntimeName for TensorUInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt32Bit";
}
impl ::std::convert::From<TensorUInt32Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorUInt32Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorUInt32Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorUInt32Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorUInt32Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorUInt32Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorUInt32Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorUInt32Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorUInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorUInt32Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt32Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorUInt32Bit {}
unsafe impl ::std::marker::Sync for TensorUInt32Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorUInt64Bit(pub ::windows::runtime::IInspectable);
impl TensorUInt64Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u64>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<u64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u64>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<u64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    pub fn ITensorUInt64BitStatics<R, F: FnOnce(&ITensorUInt64BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorUInt64BitStatics2<R, F: FnOnce(&ITensorUInt64BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorUInt64Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt64Bit;{2e70ffad-04bf-4825-839a-82baef8c7886})");
}
unsafe impl ::windows::runtime::Interface for TensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(779157421, 1215, 18469, [131, 154, 130, 186, 239, 140, 120, 134]);
}
impl ::windows::runtime::RuntimeName for TensorUInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt64Bit";
}
impl ::std::convert::From<TensorUInt64Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorUInt64Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorUInt64Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorUInt64Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorUInt64Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorUInt64Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorUInt64Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorUInt64Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorUInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorUInt64Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt64Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorUInt64Bit {}
unsafe impl ::std::marker::Sync for TensorUInt64Bit {}
#[doc = "*Required features: `AI_MachineLearning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TensorUInt8Bit(pub ::windows::runtime::IInspectable);
impl TensorUInt8Bit {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn GetAsVectorView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u8>>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<LearningModelFeatureKind> {
        let this = &::windows::runtime::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn TensorKind(&self) -> ::windows::runtime::Result<TensorKind> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Shape(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::runtime::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn Create() -> ::windows::runtime::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn Create2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::runtime::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromArray<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `AI_MachineLearning`, `Foundation_Collections`*"]
    pub fn CreateFromIterable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u8>>>(shape: Param0, data: Param1) -> ::windows::runtime::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `AI_MachineLearning`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], data: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), data.len() as u32, ::std::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `AI_MachineLearning`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], buffer: Param1) -> ::windows::runtime::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shape.len() as u32, ::std::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    pub fn ITensorUInt8BitStatics<R, F: FnOnce(&ITensorUInt8BitStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITensorUInt8BitStatics2<R, F: FnOnce(&ITensorUInt8BitStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TensorUInt8Bit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt8Bit;{58e1ae27-622b-48e3-be22-d867aed1daac})");
}
unsafe impl ::windows::runtime::Interface for TensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1491185191, 25131, 18659, [190, 34, 216, 103, 174, 209, 218, 172]);
}
impl ::windows::runtime::RuntimeName for TensorUInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt8Bit";
}
impl ::std::convert::From<TensorUInt8Bit> for ::windows::runtime::IUnknown {
    fn from(value: TensorUInt8Bit) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TensorUInt8Bit> for ::windows::runtime::IUnknown {
    fn from(value: &TensorUInt8Bit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TensorUInt8Bit> for ::windows::runtime::IInspectable {
    fn from(value: TensorUInt8Bit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TensorUInt8Bit> for ::windows::runtime::IInspectable {
    fn from(value: &TensorUInt8Bit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<TensorUInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILearningModelFeatureValue> {
        ::std::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<TensorUInt8Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&TensorUInt8Bit> for ITensor {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITensor> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITensor> {
        ::std::convert::TryInto::<ITensor>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<TensorUInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&TensorUInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::std::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for TensorUInt8Bit {}
unsafe impl ::std::marker::Sync for TensorUInt8Bit {}
