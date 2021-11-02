#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Foundation_Metadata`*"]
pub struct ApiInformation {}
impl ApiInformation {
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsTypePresent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(typename: Param0) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), typename.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsMethodPresent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(typename: Param0, methodname: Param1) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), typename.into_param().abi(), methodname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsMethodPresentWithArity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(typename: Param0, methodname: Param1, inputparametercount: u32) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), typename.into_param().abi(), methodname.into_param().abi(), inputparametercount, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsEventPresent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(typename: Param0, eventname: Param1) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), typename.into_param().abi(), eventname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsPropertyPresent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsReadOnlyPropertyPresent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsWriteablePropertyPresent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsEnumNamedValuePresent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(enumtypename: Param0, valuename: Param1) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), enumtypename.into_param().abi(), valuename.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsApiContractPresentByMajor<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contractname: Param0, majorversion: u16) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), contractname.into_param().abi(), majorversion, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Metadata`*"]
    pub fn IsApiContractPresentByMajorAndMinor<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contractname: Param0, majorversion: u16, minorversion: u16) -> ::windows::runtime::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), contractname.into_param().abi(), majorversion, minorversion, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IApiInformationStatics<R, F: FnOnce(&IApiInformationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ApiInformation, IApiInformationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ApiInformation {
    const NAME: &'static str = "Windows.Foundation.Metadata.ApiInformation";
}
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AttributeTargets(pub u32);
impl AttributeTargets {
    pub const All: AttributeTargets = AttributeTargets(4294967295u32);
    pub const Delegate: AttributeTargets = AttributeTargets(1u32);
    pub const Enum: AttributeTargets = AttributeTargets(2u32);
    pub const Event: AttributeTargets = AttributeTargets(4u32);
    pub const Field: AttributeTargets = AttributeTargets(8u32);
    pub const Interface: AttributeTargets = AttributeTargets(16u32);
    pub const Method: AttributeTargets = AttributeTargets(64u32);
    pub const Parameter: AttributeTargets = AttributeTargets(128u32);
    pub const Property: AttributeTargets = AttributeTargets(256u32);
    pub const RuntimeClass: AttributeTargets = AttributeTargets(512u32);
    pub const Struct: AttributeTargets = AttributeTargets(1024u32);
    pub const InterfaceImpl: AttributeTargets = AttributeTargets(2048u32);
    pub const ApiContract: AttributeTargets = AttributeTargets(8192u32);
}
impl ::std::convert::From<u32> for AttributeTargets {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AttributeTargets {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AttributeTargets {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.AttributeTargets;u4)");
}
impl ::std::ops::BitOr for AttributeTargets {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for AttributeTargets {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for AttributeTargets {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for AttributeTargets {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for AttributeTargets {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CompositionType(pub i32);
impl CompositionType {
    pub const Protected: CompositionType = CompositionType(1i32);
    pub const Public: CompositionType = CompositionType(2i32);
}
impl ::std::convert::From<i32> for CompositionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CompositionType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CompositionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.CompositionType;i4)");
}
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeprecationType(pub i32);
impl DeprecationType {
    pub const Deprecate: DeprecationType = DeprecationType(0i32);
    pub const Remove: DeprecationType = DeprecationType(1i32);
}
impl ::std::convert::From<i32> for DeprecationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeprecationType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeprecationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.DeprecationType;i4)");
}
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FeatureStage(pub i32);
impl FeatureStage {
    pub const AlwaysDisabled: FeatureStage = FeatureStage(0i32);
    pub const DisabledByDefault: FeatureStage = FeatureStage(1i32);
    pub const EnabledByDefault: FeatureStage = FeatureStage(2i32);
    pub const AlwaysEnabled: FeatureStage = FeatureStage(3i32);
}
impl ::std::convert::From<i32> for FeatureStage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FeatureStage {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FeatureStage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.FeatureStage;i4)");
}
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GCPressureAmount(pub i32);
impl GCPressureAmount {
    pub const Low: GCPressureAmount = GCPressureAmount(0i32);
    pub const Medium: GCPressureAmount = GCPressureAmount(1i32);
    pub const High: GCPressureAmount = GCPressureAmount(2i32);
}
impl ::std::convert::From<i32> for GCPressureAmount {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GCPressureAmount {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GCPressureAmount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.GCPressureAmount;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IApiInformationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IApiInformationStatics {
    type Vtable = IApiInformationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2574531070, 63105, 18961, [180, 22, 193, 58, 71, 232, 186, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApiInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, methodname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, methodname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, inputparametercount: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, eventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumtypename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, valuename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contractname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, majorversion: u16, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contractname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, majorversion: u16, minorversion: u16, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MarshalingType(pub i32);
impl MarshalingType {
    pub const None: MarshalingType = MarshalingType(1i32);
    pub const Agile: MarshalingType = MarshalingType(2i32);
    pub const Standard: MarshalingType = MarshalingType(3i32);
    pub const InvalidMarshaling: MarshalingType = MarshalingType(0i32);
}
impl ::std::convert::From<i32> for MarshalingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MarshalingType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MarshalingType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.MarshalingType;i4)");
}
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Platform(pub i32);
impl Platform {
    pub const Windows: Platform = Platform(0i32);
    pub const WindowsPhone: Platform = Platform(1i32);
}
impl ::std::convert::From<i32> for Platform {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Platform {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Platform {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.Platform;i4)");
}
#[doc = "*Required features: `Foundation_Metadata`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ThreadingModel(pub i32);
impl ThreadingModel {
    pub const STA: ThreadingModel = ThreadingModel(1i32);
    pub const MTA: ThreadingModel = ThreadingModel(2i32);
    pub const Both: ThreadingModel = ThreadingModel(3i32);
    pub const InvalidThreading: ThreadingModel = ThreadingModel(0i32);
}
impl ::std::convert::From<i32> for ThreadingModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ThreadingModel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ThreadingModel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.ThreadingModel;i4)");
}
