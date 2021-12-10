#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c47866b_7583_450d_f0f0_6bada895af4b);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x248e2800_a793_4724_abaa_23a6de1be090);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9ece4d_2f6e_4f01_8c96_e89e331b47b1);
#[repr(C)]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
impl ::core::marker::Copy for DXCoreAdapterMemoryBudget {}
impl ::core::clone::Clone for DXCoreAdapterMemoryBudget {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterMemoryBudget {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudget {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXCoreAdapterMemoryBudget>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudget {}
impl ::core::default::Default for DXCoreAdapterMemoryBudget {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
impl ::core::marker::Copy for DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
impl ::core::clone::Clone for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXCoreAdapterMemoryBudgetNodeSegmentGroup>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
impl ::core::default::Default for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DXCoreAdapterPreference = u32;
pub const Hardware: DXCoreAdapterPreference = 0u32;
pub const MinimumPower: DXCoreAdapterPreference = 1u32;
pub const HighPerformance: DXCoreAdapterPreference = 2u32;
pub type DXCoreAdapterProperty = u32;
pub const InstanceLuid: DXCoreAdapterProperty = 0u32;
pub const DriverVersion: DXCoreAdapterProperty = 1u32;
pub const DriverDescription: DXCoreAdapterProperty = 2u32;
pub const HardwareID: DXCoreAdapterProperty = 3u32;
pub const KmdModelVersion: DXCoreAdapterProperty = 4u32;
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = 5u32;
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = 6u32;
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = 7u32;
pub const DedicatedSystemMemory: DXCoreAdapterProperty = 8u32;
pub const SharedSystemMemory: DXCoreAdapterProperty = 9u32;
pub const AcgCompatible: DXCoreAdapterProperty = 10u32;
pub const IsHardware: DXCoreAdapterProperty = 11u32;
pub const IsIntegrated: DXCoreAdapterProperty = 12u32;
pub const IsDetachable: DXCoreAdapterProperty = 13u32;
pub const HardwareIDParts: DXCoreAdapterProperty = 14u32;
pub type DXCoreAdapterState = u32;
pub const IsDriverUpdateInProgress: DXCoreAdapterState = 0u32;
pub const AdapterMemoryBudget: DXCoreAdapterState = 1u32;
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
#[repr(C)]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
impl ::core::marker::Copy for DXCoreHardwareID {}
impl ::core::clone::Clone for DXCoreHardwareID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DXCoreHardwareID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreHardwareID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXCoreHardwareID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXCoreHardwareID {}
impl ::core::default::Default for DXCoreHardwareID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
impl ::core::marker::Copy for DXCoreHardwareIDParts {}
impl ::core::clone::Clone for DXCoreHardwareIDParts {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DXCoreHardwareIDParts {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreHardwareIDParts {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DXCoreHardwareIDParts>()) == 0 }
    }
}
impl ::core::cmp::Eq for DXCoreHardwareIDParts {}
impl ::core::default::Default for DXCoreHardwareIDParts {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DXCoreNotificationType = u32;
pub const AdapterListStale: DXCoreNotificationType = 0u32;
pub const AdapterNoLongerValid: DXCoreNotificationType = 1u32;
pub const AdapterBudgetChange: DXCoreNotificationType = 2u32;
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = 3u32;
pub type DXCoreSegmentGroup = u32;
pub const Local: DXCoreSegmentGroup = 0u32;
pub const NonLocal: DXCoreSegmentGroup = 1u32;
#[repr(transparent)]
pub struct IDXCoreAdapter(::windows::core::IUnknown);
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
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(property), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
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
impl ::core::convert::From<IDXCoreAdapter> for ::windows::core::IUnknown {
    fn from(value: IDXCoreAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXCoreAdapter> for ::windows::core::IUnknown {
    fn from(value: &IDXCoreAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXCoreAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDXCoreAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXCoreAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXCoreAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXCoreAdapter {}
unsafe impl ::windows::core::Interface for IDXCoreAdapter {
    type Vtable = IDXCoreAdapterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0db4c7f_fe5a_42a2_bd62_f2a6cf6fc83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributeguid: *const ::windows::core::GUID) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDXCoreAdapterFactory(::windows::core::IUnknown);
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
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), dxcoreobject.into_param().abi(), ::core::mem::transmute(notificationtype), ::core::mem::transmute(callbackfunction), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterEventNotification(&self, eventcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventcookie)).ok()
    }
}
impl ::core::convert::From<IDXCoreAdapterFactory> for ::windows::core::IUnknown {
    fn from(value: IDXCoreAdapterFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXCoreAdapterFactory> for ::windows::core::IUnknown {
    fn from(value: &IDXCoreAdapterFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXCoreAdapterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDXCoreAdapterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXCoreAdapterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXCoreAdapterFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXCoreAdapterFactory {}
unsafe impl ::windows::core::Interface for IDXCoreAdapterFactory {
    type Vtable = IDXCoreAdapterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78ee5945_c36e_4b13_a669_005dd11c0f06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDXCoreAdapterList(::windows::core::IUnknown);
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
impl ::core::convert::From<IDXCoreAdapterList> for ::windows::core::IUnknown {
    fn from(value: IDXCoreAdapterList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDXCoreAdapterList> for ::windows::core::IUnknown {
    fn from(value: &IDXCoreAdapterList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDXCoreAdapterList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDXCoreAdapterList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDXCoreAdapterList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDXCoreAdapterList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXCoreAdapterList {}
unsafe impl ::windows::core::Interface for IDXCoreAdapterList {
    type Vtable = IDXCoreAdapterListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x526c7776_40e9_459b_b711_f32ad76dfc28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool,
);
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: ::core::option::Option<::windows::core::IUnknown>, context: *const ::core::ffi::c_void)>;
pub const _FACDXCORE: u32 = 2176u32;
