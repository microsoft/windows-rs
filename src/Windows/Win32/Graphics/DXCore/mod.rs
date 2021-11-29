#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c47866b_7583_450d_f0f0_6bada895af4b);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x248e2800_a793_4724_abaa_23a6de1be090);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9ece4d_2f6e_4f01_8c96_e89e331b47b1);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
impl DXCoreAdapterMemoryBudget {}
impl ::core::default::Default for DXCoreAdapterMemoryBudget {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXCoreAdapterMemoryBudget {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXCoreAdapterMemoryBudget").field("budget", &self.budget).field("currentUsage", &self.currentUsage).field("availableForReservation", &self.availableForReservation).field("currentReservation", &self.currentReservation).finish()
    }
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudget {
    fn eq(&self, other: &Self) -> bool {
        self.budget == other.budget && self.currentUsage == other.currentUsage && self.availableForReservation == other.availableForReservation && self.currentReservation == other.currentReservation
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudget {}
unsafe impl ::windows::core::Abi for DXCoreAdapterMemoryBudget {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
impl DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
impl ::core::default::Default for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXCoreAdapterMemoryBudgetNodeSegmentGroup").field("nodeIndex", &self.nodeIndex).field("segmentGroup", &self.segmentGroup).finish()
    }
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn eq(&self, other: &Self) -> bool {
        self.nodeIndex == other.nodeIndex && self.segmentGroup == other.segmentGroup
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
unsafe impl ::windows::core::Abi for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXCoreAdapterPreference(pub u32);
pub const Hardware: DXCoreAdapterPreference = DXCoreAdapterPreference(0u32);
pub const MinimumPower: DXCoreAdapterPreference = DXCoreAdapterPreference(1u32);
pub const HighPerformance: DXCoreAdapterPreference = DXCoreAdapterPreference(2u32);
impl ::core::convert::From<u32> for DXCoreAdapterPreference {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterPreference {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXCoreAdapterPreference {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXCoreAdapterPreference {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXCoreAdapterPreference {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXCoreAdapterPreference {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXCoreAdapterPreference {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXCoreAdapterProperty(pub u32);
pub const InstanceLuid: DXCoreAdapterProperty = DXCoreAdapterProperty(0u32);
pub const DriverVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(1u32);
pub const DriverDescription: DXCoreAdapterProperty = DXCoreAdapterProperty(2u32);
pub const HardwareID: DXCoreAdapterProperty = DXCoreAdapterProperty(3u32);
pub const KmdModelVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(4u32);
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(5u32);
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(6u32);
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(7u32);
pub const DedicatedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(8u32);
pub const SharedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(9u32);
pub const AcgCompatible: DXCoreAdapterProperty = DXCoreAdapterProperty(10u32);
pub const IsHardware: DXCoreAdapterProperty = DXCoreAdapterProperty(11u32);
pub const IsIntegrated: DXCoreAdapterProperty = DXCoreAdapterProperty(12u32);
pub const IsDetachable: DXCoreAdapterProperty = DXCoreAdapterProperty(13u32);
pub const HardwareIDParts: DXCoreAdapterProperty = DXCoreAdapterProperty(14u32);
impl ::core::convert::From<u32> for DXCoreAdapterProperty {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterProperty {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXCoreAdapterProperty {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXCoreAdapterProperty {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXCoreAdapterProperty {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXCoreAdapterProperty {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXCoreAdapterProperty {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXCoreAdapterState(pub u32);
pub const IsDriverUpdateInProgress: DXCoreAdapterState = DXCoreAdapterState(0u32);
pub const AdapterMemoryBudget: DXCoreAdapterState = DXCoreAdapterState(1u32);
impl ::core::convert::From<u32> for DXCoreAdapterState {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterState {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXCoreAdapterState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXCoreAdapterState {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXCoreAdapterState {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXCoreAdapterState {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXCoreAdapterState {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn DXCoreCreateAdapterFactory<T: ::windows::core::Interface>() -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DXCoreCreateAdapterFactory(riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        DXCoreCreateAdapterFactory(&<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
impl DXCoreHardwareID {}
impl ::core::default::Default for DXCoreHardwareID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXCoreHardwareID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXCoreHardwareID").field("vendorID", &self.vendorID).field("deviceID", &self.deviceID).field("subSysID", &self.subSysID).field("revision", &self.revision).finish()
    }
}
impl ::core::cmp::PartialEq for DXCoreHardwareID {
    fn eq(&self, other: &Self) -> bool {
        self.vendorID == other.vendorID && self.deviceID == other.deviceID && self.subSysID == other.subSysID && self.revision == other.revision
    }
}
impl ::core::cmp::Eq for DXCoreHardwareID {}
unsafe impl ::windows::core::Abi for DXCoreHardwareID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
impl DXCoreHardwareIDParts {}
impl ::core::default::Default for DXCoreHardwareIDParts {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXCoreHardwareIDParts {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXCoreHardwareIDParts").field("vendorID", &self.vendorID).field("deviceID", &self.deviceID).field("subSystemID", &self.subSystemID).field("subVendorID", &self.subVendorID).field("revisionID", &self.revisionID).finish()
    }
}
impl ::core::cmp::PartialEq for DXCoreHardwareIDParts {
    fn eq(&self, other: &Self) -> bool {
        self.vendorID == other.vendorID && self.deviceID == other.deviceID && self.subSystemID == other.subSystemID && self.subVendorID == other.subVendorID && self.revisionID == other.revisionID
    }
}
impl ::core::cmp::Eq for DXCoreHardwareIDParts {}
unsafe impl ::windows::core::Abi for DXCoreHardwareIDParts {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXCoreNotificationType(pub u32);
pub const AdapterListStale: DXCoreNotificationType = DXCoreNotificationType(0u32);
pub const AdapterNoLongerValid: DXCoreNotificationType = DXCoreNotificationType(1u32);
pub const AdapterBudgetChange: DXCoreNotificationType = DXCoreNotificationType(2u32);
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = DXCoreNotificationType(3u32);
impl ::core::convert::From<u32> for DXCoreNotificationType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXCoreNotificationType {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXCoreNotificationType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXCoreNotificationType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXCoreNotificationType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXCoreNotificationType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXCoreNotificationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXCoreSegmentGroup(pub u32);
pub const Local: DXCoreSegmentGroup = DXCoreSegmentGroup(0u32);
pub const NonLocal: DXCoreSegmentGroup = DXCoreSegmentGroup(1u32);
impl ::core::convert::From<u32> for DXCoreSegmentGroup {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXCoreSegmentGroup {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXCoreSegmentGroup {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXCoreSegmentGroup {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXCoreSegmentGroup {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXCoreSegmentGroup {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXCoreSegmentGroup {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDXCoreAdapter(pub ::windows::core::IUnknown);
impl IDXCoreAdapter {
    pub unsafe fn IsValid(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn IsAttributeSupported(&self, attributeguid: *const ::windows::core::GUID) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(attributeguid)))
    }
    pub unsafe fn IsPropertySupported(&self, property: DXCoreAdapterProperty) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(property)))
    }
    pub unsafe fn GetProperty(&self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(property), ::core::mem::transmute(buffersize), ::core::mem::transmute(propertydata)).ok()
    }
    pub unsafe fn GetPropertySize(&self, property: DXCoreAdapterProperty) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(property), &mut result__).from_abi::<usize>(result__)
    }
    pub unsafe fn IsQueryStateSupported(&self, property: DXCoreAdapterState) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(property)))
    }
    pub unsafe fn QueryState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(inputstatedetailssize), ::core::mem::transmute(inputstatedetails), ::core::mem::transmute(outputbuffersize), ::core::mem::transmute(outputbuffer)).ok()
    }
    pub unsafe fn IsSetStateSupported(&self, property: DXCoreAdapterState) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(property)))
    }
    pub unsafe fn SetState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(state), ::core::mem::transmute(inputstatedetailssize), ::core::mem::transmute(inputstatedetails), ::core::mem::transmute(inputdatasize), ::core::mem::transmute(inputdata)).ok()
    }
    pub unsafe fn GetFactory<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IDXCoreAdapter {
    type Vtable = IDXCoreAdapter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0db4c7f_fe5a_42a2_bd62_f2a6cf6fc83e);
}
impl ::core::convert::From<IDXCoreAdapter> for ::windows::core::IUnknown {
    fn from(value: IDXCoreAdapter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDXCoreAdapter> for ::windows::core::IUnknown {
    fn from(value: &IDXCoreAdapter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXCoreAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXCoreAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributeguid: *const ::windows::core::GUID) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, property: DXCoreAdapterProperty) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, property: DXCoreAdapterState) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, property: DXCoreAdapterState) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDXCoreAdapterFactory(pub ::windows::core::IUnknown);
impl IDXCoreAdapterFactory {
    pub unsafe fn CreateAdapterList<T: ::windows::core::Interface>(&self, numattributes: u32, filterattributes: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(numattributes), ::core::mem::transmute(filterattributes), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterByLuid<T: ::windows::core::Interface>(&self, adapterluid: *const super::super::Foundation::LUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn IsNotificationTypeSupported(&self, notificationtype: DXCoreNotificationType) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(notificationtype)))
    }
    pub unsafe fn RegisterEventNotification<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, dxcoreobject: Param0, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), dxcoreobject.into_param().abi(), ::core::mem::transmute(notificationtype), ::core::mem::transmute(callbackfunction), ::core::mem::transmute(callbackcontext), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterEventNotification(&self, eventcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventcookie)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDXCoreAdapterFactory {
    type Vtable = IDXCoreAdapterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78ee5945_c36e_4b13_a669_005dd11c0f06);
}
impl ::core::convert::From<IDXCoreAdapterFactory> for ::windows::core::IUnknown {
    fn from(value: IDXCoreAdapterFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDXCoreAdapterFactory> for ::windows::core::IUnknown {
    fn from(value: &IDXCoreAdapterFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXCoreAdapterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXCoreAdapterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notificationtype: DXCoreNotificationType) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dxcoreobject: ::windows::core::RawPtr, notificationtype: DXCoreNotificationType, callbackfunction: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDXCoreAdapterList(pub ::windows::core::IUnknown);
impl IDXCoreAdapterList {
    pub unsafe fn GetAdapter<T: ::windows::core::Interface>(&self, index: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn IsStale(&self) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetFactory<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Sort(&self, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(numpreferences), ::core::mem::transmute(preferences)).ok()
    }
    pub unsafe fn IsAdapterPreferenceSupported(&self, preference: DXCoreAdapterPreference) -> bool {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(preference)))
    }
}
unsafe impl ::windows::core::Interface for IDXCoreAdapterList {
    type Vtable = IDXCoreAdapterList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x526c7776_40e9_459b_b711_f32ad76dfc28);
}
impl ::core::convert::From<IDXCoreAdapterList> for ::windows::core::IUnknown {
    fn from(value: IDXCoreAdapterList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDXCoreAdapterList> for ::windows::core::IUnknown {
    fn from(value: &IDXCoreAdapterList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXCoreAdapterList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDXCoreAdapterList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> bool,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preference: DXCoreAdapterPreference) -> bool,
);
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: ::windows::core::RawPtr, context: *const ::core::ffi::c_void)>;
pub const _FACDXCORE: u32 = 2176u32;
