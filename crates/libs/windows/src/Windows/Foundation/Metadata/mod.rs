#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
pub struct ApiInformation;
impl ApiInformation {
    pub fn IsTypePresent(typename: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsTypePresent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(typename), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsMethodPresent(typename: &::windows::core::HSTRING, methodname: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsMethodPresent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(typename), ::core::mem::transmute_copy(methodname), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsMethodPresentWithArity(typename: &::windows::core::HSTRING, methodname: &::windows::core::HSTRING, inputparametercount: u32) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsMethodPresentWithArity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(typename), ::core::mem::transmute_copy(methodname), inputparametercount, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsEventPresent(typename: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEventPresent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(typename), ::core::mem::transmute_copy(eventname), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsPropertyPresent(typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPropertyPresent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(typename), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsReadOnlyPropertyPresent(typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnlyPropertyPresent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(typename), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsWriteablePropertyPresent(typename: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWriteablePropertyPresent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(typename), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsEnumNamedValuePresent(enumtypename: &::windows::core::HSTRING, valuename: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEnumNamedValuePresent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(enumtypename), ::core::mem::transmute_copy(valuename), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsApiContractPresentByMajor(contractname: &::windows::core::HSTRING, majorversion: u16) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsApiContractPresentByMajor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(contractname), majorversion, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsApiContractPresentByMajorAndMinor(contractname: &::windows::core::HSTRING, majorversion: u16, minorversion: u16) -> ::windows::core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsApiContractPresentByMajorAndMinor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(contractname), majorversion, minorversion, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApiInformationStatics<R, F: FnOnce(&IApiInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApiInformation, IApiInformationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ApiInformation {
    const NAME: &'static str = "Windows.Foundation.Metadata.ApiInformation";
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for AttributeTargets {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AttributeTargets {
    type Abi = Self;
}
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
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for CompositionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CompositionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CompositionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.CompositionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for DeprecationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeprecationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeprecationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeprecationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeprecationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.DeprecationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for FeatureStage {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FeatureStage {
    type Abi = Self;
}
impl ::core::fmt::Debug for FeatureStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FeatureStage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FeatureStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.FeatureStage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for GCPressureAmount {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GCPressureAmount {
    type Abi = Self;
}
impl ::core::fmt::Debug for GCPressureAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GCPressureAmount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GCPressureAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.GCPressureAmount;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApiInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApiInformationStatics {
    type Vtable = IApiInformationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997439fe_f681_4a11_b416_c13a47e8ba36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApiInformationStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsTypePresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsMethodPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsMethodPresentWithArity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, methodname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputparametercount: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEventPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPropertyPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReadOnlyPropertyPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsWriteablePropertyPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEnumNamedValuePresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, valuename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsApiContractPresentByMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsApiContractPresentByMajorAndMinor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contractname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, majorversion: u16, minorversion: u16, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for MarshalingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MarshalingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MarshalingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarshalingType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarshalingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.MarshalingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for Platform {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Platform {
    type Abi = Self;
}
impl ::core::fmt::Debug for Platform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Platform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Platform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.Platform;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Metadata\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ThreadingModel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ThreadingModel {
    type Abi = Self;
}
impl ::core::fmt::Debug for ThreadingModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThreadingModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThreadingModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.ThreadingModel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
