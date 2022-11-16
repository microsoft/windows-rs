#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[inline]
pub unsafe fn DXCoreCreateAdapterFactory<T>() -> ::windows::core::Result<T>
where
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "dxcore.dll""system" fn DXCoreCreateAdapterFactory ( riid : *const :: windows::core::GUID , ppvfactory : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::option::Option::None;
    DXCoreCreateAdapterFactory(&<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
pub struct IDXCoreAdapter(::windows::core::IUnknown);
impl IDXCoreAdapter {
    pub unsafe fn IsValid(&self) -> bool {
        (::windows::core::Vtable::vtable(self).IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn IsAttributeSupported(&self, attributeguid: *const ::windows::core::GUID) -> bool {
        (::windows::core::Vtable::vtable(self).IsAttributeSupported)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(attributeguid))
    }
    pub unsafe fn IsPropertySupported(&self, property: DXCoreAdapterProperty) -> bool {
        (::windows::core::Vtable::vtable(self).IsPropertySupported)(::windows::core::Vtable::as_raw(self), property)
    }
    pub unsafe fn GetProperty(&self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), property, buffersize, ::core::mem::transmute(propertydata)).ok()
    }
    pub unsafe fn GetPropertySize(&self, property: DXCoreAdapterProperty) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropertySize)(::windows::core::Vtable::as_raw(self), property, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn IsQueryStateSupported(&self, property: DXCoreAdapterState) -> bool {
        (::windows::core::Vtable::vtable(self).IsQueryStateSupported)(::windows::core::Vtable::as_raw(self), property)
    }
    pub unsafe fn QueryState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: ::core::option::Option<*const ::core::ffi::c_void>, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryState)(::windows::core::Vtable::as_raw(self), state, inputstatedetailssize, ::core::mem::transmute(inputstatedetails.unwrap_or(::std::ptr::null())), outputbuffersize, ::core::mem::transmute(outputbuffer)).ok()
    }
    pub unsafe fn IsSetStateSupported(&self, property: DXCoreAdapterState) -> bool {
        (::windows::core::Vtable::vtable(self).IsSetStateSupported)(::windows::core::Vtable::as_raw(self), property)
    }
    pub unsafe fn SetState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: ::core::option::Option<*const ::core::ffi::c_void>, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetState)(::windows::core::Vtable::as_raw(self), state, inputstatedetailssize, ::core::mem::transmute(inputstatedetails.unwrap_or(::std::ptr::null())), inputdatasize, ::core::mem::transmute(inputdata)).ok()
    }
    pub unsafe fn GetFactory<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetFactory)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
::windows::core::interface_hierarchy!(IDXCoreAdapter, ::windows::core::IUnknown);
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
impl ::core::fmt::Debug for IDXCoreAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXCoreAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDXCoreAdapter {
    type Vtable = IDXCoreAdapter_Vtbl;
}
unsafe impl ::windows::core::Interface for IDXCoreAdapter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0db4c7f_fe5a_42a2_bd62_f2a6cf6fc83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub IsAttributeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributeguid: *const ::windows::core::GUID) -> bool,
    pub IsPropertySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty) -> bool,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows::core::HRESULT,
    pub IsQueryStateSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool,
    pub QueryState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSetStateSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
pub struct IDXCoreAdapterFactory(::windows::core::IUnknown);
impl IDXCoreAdapterFactory {
    pub unsafe fn CreateAdapterList<T>(&self, filterattributes: &[::windows::core::GUID]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).CreateAdapterList)(::windows::core::Vtable::as_raw(self), filterattributes.len() as _, ::core::mem::transmute(filterattributes.as_ptr()), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterByLuid<T>(&self, adapterluid: *const super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetAdapterByLuid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn IsNotificationTypeSupported(&self, notificationtype: DXCoreNotificationType) -> bool {
        (::windows::core::Vtable::vtable(self).IsNotificationTypeSupported)(::windows::core::Vtable::as_raw(self), notificationtype)
    }
    pub unsafe fn RegisterEventNotification<'a, P0>(&self, dxcoreobject: P0, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterEventNotification)(::windows::core::Vtable::as_raw(self), dxcoreobject.into().abi(), notificationtype, ::core::mem::transmute(callbackfunction), ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterEventNotification(&self, eventcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterEventNotification)(::windows::core::Vtable::as_raw(self), eventcookie).ok()
    }
}
::windows::core::interface_hierarchy!(IDXCoreAdapterFactory, ::windows::core::IUnknown);
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
impl ::core::fmt::Debug for IDXCoreAdapterFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXCoreAdapterFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDXCoreAdapterFactory {
    type Vtable = IDXCoreAdapterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IDXCoreAdapterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78ee5945_c36e_4b13_a669_005dd11c0f06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateAdapterList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAdapterByLuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAdapterByLuid: usize,
    pub IsNotificationTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool,
    pub RegisterEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnregisterEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
pub struct IDXCoreAdapterList(::windows::core::IUnknown);
impl IDXCoreAdapterList {
    pub unsafe fn GetAdapter<T>(&self, index: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetAdapter)(::windows::core::Vtable::as_raw(self), index, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).GetAdapterCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn IsStale(&self) -> bool {
        (::windows::core::Vtable::vtable(self).IsStale)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFactory<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).GetFactory)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Sort(&self, preferences: &[DXCoreAdapterPreference]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Sort)(::windows::core::Vtable::as_raw(self), preferences.len() as _, ::core::mem::transmute(preferences.as_ptr())).ok()
    }
    pub unsafe fn IsAdapterPreferenceSupported(&self, preference: DXCoreAdapterPreference) -> bool {
        (::windows::core::Vtable::vtable(self).IsAdapterPreferenceSupported)(::windows::core::Vtable::as_raw(self), preference)
    }
}
::windows::core::interface_hierarchy!(IDXCoreAdapterList, ::windows::core::IUnknown);
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
impl ::core::fmt::Debug for IDXCoreAdapterList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXCoreAdapterList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDXCoreAdapterList {
    type Vtable = IDXCoreAdapterList_Vtbl;
}
unsafe impl ::windows::core::Interface for IDXCoreAdapterList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x526c7776_40e9_459b_b711_f32ad76dfc28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub IsStale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub GetFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Sort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::HRESULT,
    pub IsAdapterPreferenceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool,
}
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c47866b_7583_450d_f0f0_6bada895af4b);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x248e2800_a793_4724_abaa_23a6de1be090);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9ece4d_2f6e_4f01_8c96_e89e331b47b1);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const _FACDXCORE: u32 = 2176u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXCoreAdapterPreference(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const Hardware: DXCoreAdapterPreference = DXCoreAdapterPreference(0u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const MinimumPower: DXCoreAdapterPreference = DXCoreAdapterPreference(1u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HighPerformance: DXCoreAdapterPreference = DXCoreAdapterPreference(2u32);
impl ::core::marker::Copy for DXCoreAdapterPreference {}
impl ::core::clone::Clone for DXCoreAdapterPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXCoreAdapterPreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterPreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXCoreAdapterPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreAdapterPreference").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXCoreAdapterProperty(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const InstanceLuid: DXCoreAdapterProperty = DXCoreAdapterProperty(0u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DriverVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(1u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DriverDescription: DXCoreAdapterProperty = DXCoreAdapterProperty(2u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HardwareID: DXCoreAdapterProperty = DXCoreAdapterProperty(3u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const KmdModelVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(4u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(5u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(6u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(7u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DedicatedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(8u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const SharedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(9u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AcgCompatible: DXCoreAdapterProperty = DXCoreAdapterProperty(10u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsHardware: DXCoreAdapterProperty = DXCoreAdapterProperty(11u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsIntegrated: DXCoreAdapterProperty = DXCoreAdapterProperty(12u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsDetachable: DXCoreAdapterProperty = DXCoreAdapterProperty(13u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HardwareIDParts: DXCoreAdapterProperty = DXCoreAdapterProperty(14u32);
impl ::core::marker::Copy for DXCoreAdapterProperty {}
impl ::core::clone::Clone for DXCoreAdapterProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXCoreAdapterProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXCoreAdapterProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreAdapterProperty").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXCoreAdapterState(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsDriverUpdateInProgress: DXCoreAdapterState = DXCoreAdapterState(0u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterMemoryBudget: DXCoreAdapterState = DXCoreAdapterState(1u32);
impl ::core::marker::Copy for DXCoreAdapterState {}
impl ::core::clone::Clone for DXCoreAdapterState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXCoreAdapterState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXCoreAdapterState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreAdapterState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXCoreNotificationType(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterListStale: DXCoreNotificationType = DXCoreNotificationType(0u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterNoLongerValid: DXCoreNotificationType = DXCoreNotificationType(1u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterBudgetChange: DXCoreNotificationType = DXCoreNotificationType(2u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = DXCoreNotificationType(3u32);
impl ::core::marker::Copy for DXCoreNotificationType {}
impl ::core::clone::Clone for DXCoreNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXCoreNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXCoreNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXCoreNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreNotificationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXCoreSegmentGroup(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const Local: DXCoreSegmentGroup = DXCoreSegmentGroup(0u32);
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const NonLocal: DXCoreSegmentGroup = DXCoreSegmentGroup(1u32);
impl ::core::marker::Copy for DXCoreSegmentGroup {}
impl ::core::clone::Clone for DXCoreSegmentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXCoreSegmentGroup {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXCoreSegmentGroup {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXCoreSegmentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreSegmentGroup").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
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
impl ::core::fmt::Debug for DXCoreAdapterMemoryBudget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreAdapterMemoryBudget").field("budget", &self.budget).field("currentUsage", &self.currentUsage).field("availableForReservation", &self.availableForReservation).field("currentReservation", &self.currentReservation).finish()
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterMemoryBudget {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudget {
    fn eq(&self, other: &Self) -> bool {
        self.budget == other.budget && self.currentUsage == other.currentUsage && self.availableForReservation == other.availableForReservation && self.currentReservation == other.currentReservation
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudget {}
impl ::core::default::Default for DXCoreAdapterMemoryBudget {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
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
impl ::core::fmt::Debug for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreAdapterMemoryBudgetNodeSegmentGroup").field("nodeIndex", &self.nodeIndex).field("segmentGroup", &self.segmentGroup).finish()
    }
}
unsafe impl ::windows::core::Abi for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn eq(&self, other: &Self) -> bool {
        self.nodeIndex == other.nodeIndex && self.segmentGroup == other.segmentGroup
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
impl ::core::default::Default for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
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
impl ::core::fmt::Debug for DXCoreHardwareID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreHardwareID").field("vendorID", &self.vendorID).field("deviceID", &self.deviceID).field("subSysID", &self.subSysID).field("revision", &self.revision).finish()
    }
}
unsafe impl ::windows::core::Abi for DXCoreHardwareID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreHardwareID {
    fn eq(&self, other: &Self) -> bool {
        self.vendorID == other.vendorID && self.deviceID == other.deviceID && self.subSysID == other.subSysID && self.revision == other.revision
    }
}
impl ::core::cmp::Eq for DXCoreHardwareID {}
impl ::core::default::Default for DXCoreHardwareID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
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
impl ::core::fmt::Debug for DXCoreHardwareIDParts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreHardwareIDParts").field("vendorID", &self.vendorID).field("deviceID", &self.deviceID).field("subSystemID", &self.subSystemID).field("subVendorID", &self.subVendorID).field("revisionID", &self.revisionID).finish()
    }
}
unsafe impl ::windows::core::Abi for DXCoreHardwareIDParts {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXCoreHardwareIDParts {
    fn eq(&self, other: &Self) -> bool {
        self.vendorID == other.vendorID && self.deviceID == other.deviceID && self.subSystemID == other.subSystemID && self.subVendorID == other.subVendorID && self.revisionID == other.revisionID
    }
}
impl ::core::cmp::Eq for DXCoreHardwareIDParts {}
impl ::core::default::Default for DXCoreHardwareIDParts {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: ::core::option::Option<::windows::core::IUnknown>, context: *const ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
