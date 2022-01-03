#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Foundation_Metadata'*"]
pub struct ApiInformation {}
impl ApiInformation {
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsTypePresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), typename.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsMethodPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, methodname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), typename.into_param().abi(), methodname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsMethodPresentWithArity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, methodname: Param1, inputparametercount: u32) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), typename.into_param().abi(), methodname.into_param().abi(), inputparametercount, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsEventPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, eventname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), typename.into_param().abi(), eventname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsPropertyPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsReadOnlyPropertyPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsWriteablePropertyPresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(typename: Param0, propertyname: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), typename.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsEnumNamedValuePresent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(enumtypename: Param0, valuename: Param1) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), enumtypename.into_param().abi(), valuename.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsApiContractPresentByMajor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contractname: Param0, majorversion: u16) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), contractname.into_param().abi(), majorversion, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Foundation_Metadata'*"]
    pub fn IsApiContractPresentByMajorAndMinor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contractname: Param0, majorversion: u16, minorversion: u16) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), contractname.into_param().abi(), majorversion, minorversion, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApiInformationStatics<R, F: FnOnce(&IApiInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApiInformation, IApiInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ApiInformation {
    const NAME: &'static str = "Windows.Foundation.Metadata.ApiInformation";
}
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct AttributeTargets(pub u32);
impl AttributeTargets {
    pub const All: Self = Self(4294967295u32);
    pub const Delegate: Self = Self(1u32);
    pub const Enum: Self = Self(2u32);
    pub const Event: Self = Self(4u32);
    pub const Field: Self = Self(8u32);
    pub const Interface: Self = Self(16u32);
    pub const Method: Self = Self(64u32);
    pub const Parameter: Self = Self(128u32);
    pub const Property: Self = Self(256u32);
    pub const RuntimeClass: Self = Self(512u32);
    pub const Struct: Self = Self(1024u32);
    pub const InterfaceImpl: Self = Self(2048u32);
    pub const ApiContract: Self = Self(8192u32);
}
impl ::core::marker::Copy for AttributeTargets {}
impl ::core::clone::Clone for AttributeTargets {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AttributeTargets {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AttributeTargets {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AttributeTargets {}
impl ::core::fmt::Debug for AttributeTargets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AttributeTargets").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AttributeTargets {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AttributeTargets {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AttributeTargets {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AttributeTargets {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AttributeTargets {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for AttributeTargets {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.AttributeTargets;u4)");
}
impl ::windows::core::DefaultType for AttributeTargets {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct CompositionType(pub i32);
impl CompositionType {
    pub const Protected: Self = Self(1i32);
    pub const Public: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionType {}
impl ::core::clone::Clone for CompositionType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CompositionType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CompositionType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionType {}
impl ::core::fmt::Debug for CompositionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.CompositionType;i4)");
}
impl ::windows::core::DefaultType for CompositionType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct DeprecationType(pub i32);
impl DeprecationType {
    pub const Deprecate: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
}
impl ::core::marker::Copy for DeprecationType {}
impl ::core::clone::Clone for DeprecationType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DeprecationType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DeprecationType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeprecationType {}
impl ::core::fmt::Debug for DeprecationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeprecationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeprecationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.DeprecationType;i4)");
}
impl ::windows::core::DefaultType for DeprecationType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct FeatureStage(pub i32);
impl FeatureStage {
    pub const AlwaysDisabled: Self = Self(0i32);
    pub const DisabledByDefault: Self = Self(1i32);
    pub const EnabledByDefault: Self = Self(2i32);
    pub const AlwaysEnabled: Self = Self(3i32);
}
impl ::core::marker::Copy for FeatureStage {}
impl ::core::clone::Clone for FeatureStage {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FeatureStage {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FeatureStage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FeatureStage {}
impl ::core::fmt::Debug for FeatureStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FeatureStage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FeatureStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.FeatureStage;i4)");
}
impl ::windows::core::DefaultType for FeatureStage {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct GCPressureAmount(pub i32);
impl GCPressureAmount {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl ::core::marker::Copy for GCPressureAmount {}
impl ::core::clone::Clone for GCPressureAmount {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GCPressureAmount {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GCPressureAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GCPressureAmount {}
impl ::core::fmt::Debug for GCPressureAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GCPressureAmount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GCPressureAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.GCPressureAmount;i4)");
}
impl ::windows::core::DefaultType for GCPressureAmount {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApiInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApiInformationStatics {
    type Vtable = IApiInformationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997439fe_f681_4a11_b416_c13a47e8ba36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApiInformationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputparametercount: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, valuename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, minorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct MarshalingType(pub i32);
impl MarshalingType {
    pub const None: Self = Self(1i32);
    pub const Agile: Self = Self(2i32);
    pub const Standard: Self = Self(3i32);
    pub const InvalidMarshaling: Self = Self(0i32);
}
impl ::core::marker::Copy for MarshalingType {}
impl ::core::clone::Clone for MarshalingType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MarshalingType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MarshalingType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MarshalingType {}
impl ::core::fmt::Debug for MarshalingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarshalingType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarshalingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.MarshalingType;i4)");
}
impl ::windows::core::DefaultType for MarshalingType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct Platform(pub i32);
impl Platform {
    pub const Windows: Self = Self(0i32);
    pub const WindowsPhone: Self = Self(1i32);
}
impl ::core::marker::Copy for Platform {}
impl ::core::clone::Clone for Platform {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Platform {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Platform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Platform {}
impl ::core::fmt::Debug for Platform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Platform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Platform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.Platform;i4)");
}
impl ::windows::core::DefaultType for Platform {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Foundation_Metadata'*"]
#[repr(transparent)]
pub struct ThreadingModel(pub i32);
impl ThreadingModel {
    pub const STA: Self = Self(1i32);
    pub const MTA: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const InvalidThreading: Self = Self(0i32);
}
impl ::core::marker::Copy for ThreadingModel {}
impl ::core::clone::Clone for ThreadingModel {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ThreadingModel {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ThreadingModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ThreadingModel {}
impl ::core::fmt::Debug for ThreadingModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThreadingModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThreadingModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.ThreadingModel;i4)");
}
impl ::windows::core::DefaultType for ThreadingModel {
    type DefaultType = Self;
}
