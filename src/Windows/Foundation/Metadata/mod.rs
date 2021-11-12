#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub struct ApiInformation {}
impl ApiInformation {
    pub fn IsTypePresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), typename.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsMethodPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, methodname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), typename.into_param().abi(), methodname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsMethodPresentWithArity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, methodname: Param1, inputparametercount: u32) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), typename.into_param().abi(), methodname.into_param().abi(), inputparametercount, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsEventPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, eventname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), typename.into_param().abi(), eventname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsPropertyPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsReadOnlyPropertyPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsWriteablePropertyPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsEnumNamedValuePresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(enumtypename: Param0, valuename: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), enumtypename.into_param().abi(), valuename.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsApiContractPresentByMajor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contractname: Param0, majorversion: u16) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), contractname.into_param().abi(), majorversion, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsApiContractPresentByMajorAndMinor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contractname: Param0, majorversion: u16, minorversion: u16) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), contractname.into_param().abi(), majorversion, minorversion, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IApiInformationStatics<R, F: FnOnce(&IApiInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApiInformation, IApiInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ApiInformation {
    const NAME: &'static str = "Windows.Foundation.Metadata.ApiInformation";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<u32> for AttributeTargets {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AttributeTargets {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AttributeTargets {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.AttributeTargets;u4)");
}
impl ::windows::core::DefaultType for AttributeTargets {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for AttributeTargets {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AttributeTargets {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AttributeTargets {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AttributeTargets {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AttributeTargets {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CompositionType(pub i32);
impl CompositionType {
    pub const Protected: CompositionType = CompositionType(1i32);
    pub const Public: CompositionType = CompositionType(2i32);
}
impl ::core::convert::From<i32> for CompositionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CompositionType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CompositionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.CompositionType;i4)");
}
impl ::windows::core::DefaultType for CompositionType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeprecationType(pub i32);
impl DeprecationType {
    pub const Deprecate: DeprecationType = DeprecationType(0i32);
    pub const Remove: DeprecationType = DeprecationType(1i32);
}
impl ::core::convert::From<i32> for DeprecationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DeprecationType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeprecationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.DeprecationType;i4)");
}
impl ::windows::core::DefaultType for DeprecationType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FeatureStage(pub i32);
impl FeatureStage {
    pub const AlwaysDisabled: FeatureStage = FeatureStage(0i32);
    pub const DisabledByDefault: FeatureStage = FeatureStage(1i32);
    pub const EnabledByDefault: FeatureStage = FeatureStage(2i32);
    pub const AlwaysEnabled: FeatureStage = FeatureStage(3i32);
}
impl ::core::convert::From<i32> for FeatureStage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FeatureStage {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FeatureStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.FeatureStage;i4)");
}
impl ::windows::core::DefaultType for FeatureStage {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GCPressureAmount(pub i32);
impl GCPressureAmount {
    pub const Low: GCPressureAmount = GCPressureAmount(0i32);
    pub const Medium: GCPressureAmount = GCPressureAmount(1i32);
    pub const High: GCPressureAmount = GCPressureAmount(2i32);
}
impl ::core::convert::From<i32> for GCPressureAmount {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GCPressureAmount {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GCPressureAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.GCPressureAmount;i4)");
}
impl ::windows::core::DefaultType for GCPressureAmount {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IApiInformationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApiInformationStatics {
    type Vtable = IApiInformationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997439fe_f681_4a11_b416_c13a47e8ba36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApiInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputparametercount: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enumtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, valuename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, minorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MarshalingType(pub i32);
impl MarshalingType {
    pub const None: MarshalingType = MarshalingType(1i32);
    pub const Agile: MarshalingType = MarshalingType(2i32);
    pub const Standard: MarshalingType = MarshalingType(3i32);
    pub const InvalidMarshaling: MarshalingType = MarshalingType(0i32);
}
impl ::core::convert::From<i32> for MarshalingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MarshalingType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MarshalingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.MarshalingType;i4)");
}
impl ::windows::core::DefaultType for MarshalingType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Platform(pub i32);
impl Platform {
    pub const Windows: Platform = Platform(0i32);
    pub const WindowsPhone: Platform = Platform(1i32);
}
impl ::core::convert::From<i32> for Platform {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for Platform {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Platform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.Platform;i4)");
}
impl ::windows::core::DefaultType for Platform {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ThreadingModel(pub i32);
impl ThreadingModel {
    pub const STA: ThreadingModel = ThreadingModel(1i32);
    pub const MTA: ThreadingModel = ThreadingModel(2i32);
    pub const Both: ThreadingModel = ThreadingModel(3i32);
    pub const InvalidThreading: ThreadingModel = ThreadingModel(0i32);
}
impl ::core::convert::From<i32> for ThreadingModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ThreadingModel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ThreadingModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.ThreadingModel;i4)");
}
impl ::windows::core::DefaultType for ThreadingModel {
    type DefaultType = Self;
}
