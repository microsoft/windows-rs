#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameKindStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownPerceptionFrameKindStatics {
    type Vtable = IKnownPerceptionFrameKindStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(988172758, 38505, 16646, [159, 174, 72, 53, 193, 185, 97, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameKindStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionControlGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionControlGroup {
    type Vtable = IPerceptionControlGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(388778114, 12249, 19534, [186, 52, 253, 242, 10, 115, 221, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlGroup_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionControlGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionControlGroupFactory {
    type Vtable = IPerceptionControlGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(790295264, 47857, 17723, [190, 212, 205, 157, 70, 25, 21, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionCorrelation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionCorrelation {
    type Vtable = IPerceptionCorrelation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3021150850, 57333, 16455, [138, 25, 59, 77, 128, 95, 113, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionCorrelationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionCorrelationFactory {
    type Vtable = IPerceptionCorrelationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3567698981, 10372, 19087, [129, 52, 40, 53, 215, 40, 108, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, position: super::super::super::Foundation::Numerics::Vector3, orientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionCorrelationGroup {
    type Vtable = IPerceptionCorrelationGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1965689094, 13991, 18363, [155, 121, 86, 204, 107, 116, 103, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroup_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionCorrelationGroupFactory {
    type Vtable = IPerceptionCorrelationGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2113806472, 25567, 18669, [131, 177, 74, 184, 41, 19, 41, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativelocations: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionFaceAuthenticationGroup {
    type Vtable = IPerceptionFaceAuthenticationGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3892418580, 19089, 16816, [131, 166, 136, 26, 23, 117, 53, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroup_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionFaceAuthenticationGroupFactory {
    type Vtable = IPerceptionFaceAuthenticationGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3867805140, 46604, 16628, [188, 185, 242, 77, 70, 70, 115, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ids: ::windows::runtime::RawPtr, starthandler: ::windows::runtime::RawPtr, stophandler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionFrame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionFrame {
    type Vtable = IPerceptionFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2097051685, 21691, 19869, [190, 197, 142, 246, 97, 81, 210, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_Perception_Provider`*"]
pub struct IPerceptionFrameProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionFrameProvider {
    type Vtable = IPerceptionFrameProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2035251897, 45949, 15155, [161, 13, 48, 98, 100, 25, 206, 101]);
}
impl IPerceptionFrameProvider {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn FrameProviderInfo(&self) -> ::windows::runtime::Result<PerceptionFrameProviderInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PerceptionFrameProviderInfo>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Available(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, PerceptionPropertyChangeRequest>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPerceptionFrameProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{794f7ab9-b37d-3b33-a10d-30626419ce65}");
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IPerceptionFrameProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPerceptionFrameProvider) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IPerceptionFrameProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPerceptionFrameProvider) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &IPerceptionFrameProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionFrameProviderInfo {
    type Vtable = IPerceptionFrameProviderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3433650664, 31102, 20099, [155, 135, 3, 106, 116, 20, 47, 196]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_Perception_Provider`*"]
pub struct IPerceptionFrameProviderManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionFrameProviderManager {
    type Vtable = IPerceptionFrameProviderManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2841234951, 60115, 13279, [142, 193, 185, 36, 171, 224, 25, 196]);
}
impl IPerceptionFrameProviderManager {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn GetFrameProvider<'a, Param0: ::windows::runtime::IntoParam<'a, PerceptionFrameProviderInfo>>(&self, frameproviderinfo: Param0) -> ::windows::runtime::Result<IPerceptionFrameProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), frameproviderinfo.into_param().abi(), &mut result__).from_abi::<IPerceptionFrameProvider>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPerceptionFrameProviderManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a959ce07-ead3-33df-8ec1-b924abe019c4}");
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IPerceptionFrameProviderManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPerceptionFrameProviderManager) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IPerceptionFrameProviderManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPerceptionFrameProviderManager) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameproviderinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderManagerServiceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionFrameProviderManagerServiceStatics {
    type Vtable = IPerceptionFrameProviderManagerServiceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2927855334, 51929, 17241, [143, 150, 142, 174, 81, 129, 5, 38]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderManagerServiceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, frameproviderinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, frameproviderinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, faceauthenticationgroup: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, faceauthenticationgroup: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, controlgroup: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, controlgroup: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, correlationgroup: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manager: ::windows::runtime::RawPtr, correlationgroup: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, available: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, frame: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionPropertyChangeRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionPropertyChangeRequest {
    type Vtable = IPerceptionPropertyChangeRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1012591441, 13579, 19960, [148, 20, 89, 224, 152, 21, 81, 11]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionPropertyChangeRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionVideoFrameAllocator {
    type Vtable = IPerceptionVideoFrameAllocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1278781402, 64984, 20180, [160, 57, 42, 111, 155, 35, 80, 56]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frame: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocatorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerceptionVideoFrameAllocatorFactory {
    type Vtable = IPerceptionVideoFrameAllocatorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(442020065, 59674, 18462, [184, 118, 168, 158, 43, 188, 107, 51]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocatorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[doc = "*Required features: `Devices_Perception_Provider`*"]
pub struct KnownPerceptionFrameKind {}
impl KnownPerceptionFrameKind {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Color() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Depth() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Infrared() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownPerceptionFrameKindStatics<R, F: FnOnce(&IKnownPerceptionFrameKindStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownPerceptionFrameKind, IKnownPerceptionFrameKindStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownPerceptionFrameKind {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.KnownPerceptionFrameKind";
}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionControlGroup(::windows::runtime::IInspectable);
impl PerceptionControlGroup {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn FrameProviderIds(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(ids: Param0) -> ::windows::runtime::Result<PerceptionControlGroup> {
        Self::IPerceptionControlGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ids.into_param().abi(), &mut result__).from_abi::<PerceptionControlGroup>(result__)
        })
    }
    pub fn IPerceptionControlGroupFactory<R, F: FnOnce(&IPerceptionControlGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionControlGroup, IPerceptionControlGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionControlGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionControlGroup;{172c4882-2fd9-4c4e-ba34-fdf20a73dde5})");
}
unsafe impl ::windows::runtime::Interface for PerceptionControlGroup {
    type Vtable = IPerceptionControlGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(388778114, 12249, 19534, [186, 52, 253, 242, 10, 115, 221, 229]);
}
impl ::windows::runtime::RuntimeName for PerceptionControlGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionControlGroup";
}
impl ::std::convert::From<PerceptionControlGroup> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionControlGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionControlGroup> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionControlGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionControlGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionControlGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionControlGroup> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionControlGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionControlGroup> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionControlGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionControlGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionControlGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionControlGroup {}
unsafe impl ::std::marker::Sync for PerceptionControlGroup {}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionCorrelation(::windows::runtime::IInspectable);
impl PerceptionCorrelation {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn TargetId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Numerics`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Numerics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(targetid: Param0, position: Param1, orientation: Param2) -> ::windows::runtime::Result<PerceptionCorrelation> {
        Self::IPerceptionCorrelationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), targetid.into_param().abi(), position.into_param().abi(), orientation.into_param().abi(), &mut result__).from_abi::<PerceptionCorrelation>(result__)
        })
    }
    pub fn IPerceptionCorrelationFactory<R, F: FnOnce(&IPerceptionCorrelationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionCorrelation, IPerceptionCorrelationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionCorrelation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionCorrelation;{b4131a82-dff5-4047-8a19-3b4d805f7176})");
}
unsafe impl ::windows::runtime::Interface for PerceptionCorrelation {
    type Vtable = IPerceptionCorrelation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3021150850, 57333, 16455, [138, 25, 59, 77, 128, 95, 113, 118]);
}
impl ::windows::runtime::RuntimeName for PerceptionCorrelation {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionCorrelation";
}
impl ::std::convert::From<PerceptionCorrelation> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionCorrelation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionCorrelation> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionCorrelation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionCorrelation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionCorrelation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionCorrelation> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionCorrelation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionCorrelation> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionCorrelation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionCorrelation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionCorrelation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionCorrelation {}
unsafe impl ::std::marker::Sync for PerceptionCorrelation {}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionCorrelationGroup(::windows::runtime::IInspectable);
impl PerceptionCorrelationGroup {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn RelativeLocations(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation>>>(relativelocations: Param0) -> ::windows::runtime::Result<PerceptionCorrelationGroup> {
        Self::IPerceptionCorrelationGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), relativelocations.into_param().abi(), &mut result__).from_abi::<PerceptionCorrelationGroup>(result__)
        })
    }
    pub fn IPerceptionCorrelationGroupFactory<R, F: FnOnce(&IPerceptionCorrelationGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionCorrelationGroup, IPerceptionCorrelationGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionCorrelationGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionCorrelationGroup;{752a0906-36a7-47bb-9b79-56cc6b746770})");
}
unsafe impl ::windows::runtime::Interface for PerceptionCorrelationGroup {
    type Vtable = IPerceptionCorrelationGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1965689094, 13991, 18363, [155, 121, 86, 204, 107, 116, 103, 112]);
}
impl ::windows::runtime::RuntimeName for PerceptionCorrelationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionCorrelationGroup";
}
impl ::std::convert::From<PerceptionCorrelationGroup> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionCorrelationGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionCorrelationGroup> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionCorrelationGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionCorrelationGroup> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionCorrelationGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionCorrelationGroup> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionCorrelationGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionCorrelationGroup {}
unsafe impl ::std::marker::Sync for PerceptionCorrelationGroup {}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionFaceAuthenticationGroup(::windows::runtime::IInspectable);
impl PerceptionFaceAuthenticationGroup {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn FrameProviderIds(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, PerceptionStartFaceAuthenticationHandler>, Param2: ::windows::runtime::IntoParam<'a, PerceptionStopFaceAuthenticationHandler>>(ids: Param0, starthandler: Param1, stophandler: Param2) -> ::windows::runtime::Result<PerceptionFaceAuthenticationGroup> {
        Self::IPerceptionFaceAuthenticationGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ids.into_param().abi(), starthandler.into_param().abi(), stophandler.into_param().abi(), &mut result__).from_abi::<PerceptionFaceAuthenticationGroup>(result__)
        })
    }
    pub fn IPerceptionFaceAuthenticationGroupFactory<R, F: FnOnce(&IPerceptionFaceAuthenticationGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionFaceAuthenticationGroup, IPerceptionFaceAuthenticationGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionFaceAuthenticationGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup;{e8019814-4a91-41b0-83a6-881a1775353e})");
}
unsafe impl ::windows::runtime::Interface for PerceptionFaceAuthenticationGroup {
    type Vtable = IPerceptionFaceAuthenticationGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3892418580, 19089, 16816, [131, 166, 136, 26, 23, 117, 53, 62]);
}
impl ::windows::runtime::RuntimeName for PerceptionFaceAuthenticationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup";
}
impl ::std::convert::From<PerceptionFaceAuthenticationGroup> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionFaceAuthenticationGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionFaceAuthenticationGroup> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionFaceAuthenticationGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionFaceAuthenticationGroup> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionFaceAuthenticationGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionFaceAuthenticationGroup> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionFaceAuthenticationGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionFaceAuthenticationGroup {}
unsafe impl ::std::marker::Sync for PerceptionFaceAuthenticationGroup {}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionFrame(::windows::runtime::IInspectable);
impl PerceptionFrame {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`*"]
    pub fn RelativeTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`*"]
    pub fn SetRelativeTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`*"]
    pub fn FrameData(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IMemoryBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IMemoryBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFrame;{7cfe7825-54bb-4d9d-bec5-8ef66151d2ac})");
}
unsafe impl ::windows::runtime::Interface for PerceptionFrame {
    type Vtable = IPerceptionFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2097051685, 21691, 19869, [190, 197, 142, 246, 97, 81, 210, 172]);
}
impl ::windows::runtime::RuntimeName for PerceptionFrame {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrame";
}
impl ::std::convert::From<PerceptionFrame> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionFrame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionFrame> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionFrame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionFrame> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionFrame) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionFrame> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionFrame {}
unsafe impl ::std::marker::Sync for PerceptionFrame {}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionFrameProviderInfo(::windows::runtime::IInspectable);
impl PerceptionFrameProviderInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionFrameProviderInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn DeviceKind(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn SetDeviceKind<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn FrameKind(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn SetFrameKind<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Hidden(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn SetHidden(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionFrameProviderInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo;{cca959e8-797e-4e83-9b87-036a74142fc4})");
}
unsafe impl ::windows::runtime::Interface for PerceptionFrameProviderInfo {
    type Vtable = IPerceptionFrameProviderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3433650664, 31102, 20099, [155, 135, 3, 106, 116, 20, 47, 196]);
}
impl ::windows::runtime::RuntimeName for PerceptionFrameProviderInfo {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo";
}
impl ::std::convert::From<PerceptionFrameProviderInfo> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionFrameProviderInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionFrameProviderInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionFrameProviderInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionFrameProviderInfo> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionFrameProviderInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionFrameProviderInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionFrameProviderInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionFrameProviderInfo {}
unsafe impl ::std::marker::Sync for PerceptionFrameProviderInfo {}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
pub struct PerceptionFrameProviderManagerService {}
impl PerceptionFrameProviderManagerService {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn RegisterFrameProviderInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionFrameProviderInfo>>(manager: Param0, frameproviderinfo: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), manager.into_param().abi(), frameproviderinfo.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn UnregisterFrameProviderInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionFrameProviderInfo>>(manager: Param0, frameproviderinfo: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), manager.into_param().abi(), frameproviderinfo.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn RegisterFaceAuthenticationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(manager: Param0, faceauthenticationgroup: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), manager.into_param().abi(), faceauthenticationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn UnregisterFaceAuthenticationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(manager: Param0, faceauthenticationgroup: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), manager.into_param().abi(), faceauthenticationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn RegisterControlGroup<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionControlGroup>>(manager: Param0, controlgroup: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), manager.into_param().abi(), controlgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn UnregisterControlGroup<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionControlGroup>>(manager: Param0, controlgroup: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), manager.into_param().abi(), controlgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn RegisterCorrelationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionCorrelationGroup>>(manager: Param0, correlationgroup: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), manager.into_param().abi(), correlationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn UnregisterCorrelationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows::runtime::IntoParam<'a, PerceptionCorrelationGroup>>(manager: Param0, correlationgroup: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), manager.into_param().abi(), correlationgroup.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn UpdateAvailabilityForProvider<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProvider>>(provider: Param0, available: bool) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), provider.into_param().abi(), available).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn PublishFrameForProvider<'a, Param0: ::windows::runtime::IntoParam<'a, IPerceptionFrameProvider>, Param1: ::windows::runtime::IntoParam<'a, PerceptionFrame>>(provider: Param0, frame: Param1) -> ::windows::runtime::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), provider.into_param().abi(), frame.into_param().abi()).ok() })
    }
    pub fn IPerceptionFrameProviderManagerServiceStatics<R, F: FnOnce(&IPerceptionFrameProviderManagerServiceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionFrameProviderManagerService, IPerceptionFrameProviderManagerServiceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PerceptionFrameProviderManagerService {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService";
}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionPropertyChangeRequest(::windows::runtime::IInspectable);
impl PerceptionPropertyChangeRequest {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::PerceptionFrameSourcePropertyChangeStatus> {
        let this = self;
        unsafe {
            let mut result__: super::PerceptionFrameSourcePropertyChangeStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::PerceptionFrameSourcePropertyChangeStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn SetStatus(&self, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionPropertyChangeRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest;{3c5aeb51-350b-4df8-9414-59e09815510b})");
}
unsafe impl ::windows::runtime::Interface for PerceptionPropertyChangeRequest {
    type Vtable = IPerceptionPropertyChangeRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1012591441, 13579, 19960, [148, 20, 89, 224, 152, 21, 81, 11]);
}
impl ::windows::runtime::RuntimeName for PerceptionPropertyChangeRequest {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest";
}
impl ::std::convert::From<PerceptionPropertyChangeRequest> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionPropertyChangeRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionPropertyChangeRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionPropertyChangeRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionPropertyChangeRequest> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionPropertyChangeRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionPropertyChangeRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionPropertyChangeRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PerceptionPropertyChangeRequest {}
unsafe impl ::std::marker::Sync for PerceptionPropertyChangeRequest {}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionStartFaceAuthenticationHandler(::windows::runtime::IUnknown);
impl PerceptionStartFaceAuthenticationHandler {
    pub fn new<F: FnMut(&::std::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::runtime::Result<bool> + 'static>(invoke: F) -> Self {
        let com = PerceptionStartFaceAuthenticationHandler_box::<F> {
            vtable: &PerceptionStartFaceAuthenticationHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(&self, sender: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionStartFaceAuthenticationHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({74816d2a-2090-4670-8c48-ef39e7ff7c26})");
}
unsafe impl ::windows::runtime::Interface for PerceptionStartFaceAuthenticationHandler {
    type Vtable = PerceptionStartFaceAuthenticationHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1954639146, 8336, 18032, [140, 72, 239, 57, 231, 255, 124, 38]);
}
#[repr(C)]
#[doc(hidden)]
pub struct PerceptionStartFaceAuthenticationHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct PerceptionStartFaceAuthenticationHandler_box<F: FnMut(&::std::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::runtime::Result<bool> + 'static> {
    vtable: *const PerceptionStartFaceAuthenticationHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::runtime::Result<bool> + 'static> PerceptionStartFaceAuthenticationHandler_box<F> {
    const VTABLE: PerceptionStartFaceAuthenticationHandler_abi = PerceptionStartFaceAuthenticationHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<PerceptionStartFaceAuthenticationHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        match ((*this).invoke)(&*(&sender as *const <PerceptionFaceAuthenticationGroup as ::windows::runtime::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::runtime::Abi>::DefaultType)) {
            ::std::result::Result::Ok(ok__) => {
                *result__ = ::std::mem::transmute_copy(&ok__);
                ::std::mem::forget(ok__);
                ::windows::runtime::HRESULT(0)
            }
            ::std::result::Result::Err(err) => err.into(),
        }
    }
}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionStopFaceAuthenticationHandler(::windows::runtime::IUnknown);
impl PerceptionStopFaceAuthenticationHandler {
    pub fn new<F: FnMut(&::std::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = PerceptionStopFaceAuthenticationHandler_box::<F> {
            vtable: &PerceptionStopFaceAuthenticationHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionStopFaceAuthenticationHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({387ee6aa-89cd-481e-aade-dd92f70b2ad7})");
}
unsafe impl ::windows::runtime::Interface for PerceptionStopFaceAuthenticationHandler {
    type Vtable = PerceptionStopFaceAuthenticationHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(947840682, 35277, 18462, [170, 222, 221, 146, 247, 11, 42, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct PerceptionStopFaceAuthenticationHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct PerceptionStopFaceAuthenticationHandler_box<F: FnMut(&::std::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const PerceptionStopFaceAuthenticationHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::runtime::Result<()> + 'static> PerceptionStopFaceAuthenticationHandler_box<F> {
    const VTABLE: PerceptionStopFaceAuthenticationHandler_abi = PerceptionStopFaceAuthenticationHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<PerceptionStopFaceAuthenticationHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <PerceptionFaceAuthenticationGroup as ::windows::runtime::Abi>::Abi as *const <PerceptionFaceAuthenticationGroup as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Devices_Perception_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PerceptionVideoFrameAllocator(::windows::runtime::IInspectable);
impl PerceptionVideoFrameAllocator {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Perception_Provider`*"]
    pub fn AllocateFrame(&self) -> ::windows::runtime::Result<PerceptionFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PerceptionFrame>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Media`*"]
    pub fn CopyFromVideoFrame<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Media::VideoFrame>>(&self, frame: Param0) -> ::windows::runtime::Result<PerceptionFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), frame.into_param().abi(), &mut result__).from_abi::<PerceptionFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Devices_Perception_Provider`, `Foundation`, `Graphics_Imaging`*"]
    pub fn Create<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Size>>(maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: Param2, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::runtime::Result<PerceptionVideoFrameAllocator> {
        Self::IPerceptionVideoFrameAllocatorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), maxoutstandingframecountforwrite, format, resolution.into_param().abi(), alpha, &mut result__).from_abi::<PerceptionVideoFrameAllocator>(result__)
        })
    }
    pub fn IPerceptionVideoFrameAllocatorFactory<R, F: FnOnce(&IPerceptionVideoFrameAllocatorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PerceptionVideoFrameAllocator, IPerceptionVideoFrameAllocatorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerceptionVideoFrameAllocator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator;{4c38a7da-fdd8-4ed4-a039-2a6f9b235038})");
}
unsafe impl ::windows::runtime::Interface for PerceptionVideoFrameAllocator {
    type Vtable = IPerceptionVideoFrameAllocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1278781402, 64984, 20180, [160, 57, 42, 111, 155, 35, 80, 56]);
}
impl ::windows::runtime::RuntimeName for PerceptionVideoFrameAllocator {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator";
}
impl ::std::convert::From<PerceptionVideoFrameAllocator> for ::windows::runtime::IUnknown {
    fn from(value: PerceptionVideoFrameAllocator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PerceptionVideoFrameAllocator> for ::windows::runtime::IUnknown {
    fn from(value: &PerceptionVideoFrameAllocator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PerceptionVideoFrameAllocator> for ::windows::runtime::IInspectable {
    fn from(value: PerceptionVideoFrameAllocator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PerceptionVideoFrameAllocator> for ::windows::runtime::IInspectable {
    fn from(value: &PerceptionVideoFrameAllocator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<PerceptionVideoFrameAllocator> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PerceptionVideoFrameAllocator) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&PerceptionVideoFrameAllocator> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PerceptionVideoFrameAllocator) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PerceptionVideoFrameAllocator {}
unsafe impl ::std::marker::Sync for PerceptionVideoFrameAllocator {}
