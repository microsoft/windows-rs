#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[inline]
pub unsafe fn DMProcessConfigXMLFiltered<P0>(pszxmlin: P0, rgszallowedcspnodes: &[::windows::core::PCWSTR]) -> ::windows::core::Result<::windows::core::BSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dmprocessxmlfiltered.dll""system" fn DMProcessConfigXMLFiltered ( pszxmlin : ::windows::core::PCWSTR , rgszallowedcspnodes : *const ::windows::core::PCWSTR , dwnumallowedcspnodes : u32 , pbstrxmlout : *mut ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
    DMProcessConfigXMLFiltered(pszxmlin.into_param().abi(), ::core::mem::transmute(rgszallowedcspnodes.as_ptr()), rgszallowedcspnodes.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IConnectionRequestCallback(::windows::core::IUnknown);
impl IConnectionRequestCallback {
    pub unsafe fn OnComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnComplete)(::windows::core::Interface::as_raw(self), hrstatus).ok()
    }
}
::windows::imp::interface_hierarchy!(IConnectionRequestCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IConnectionRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionRequestCallback {}
impl ::core::fmt::Debug for IConnectionRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionRequestCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConnectionRequestCallback {
    type Vtable = IConnectionRequestCallback_Vtbl;
}
impl ::core::clone::Clone for IConnectionRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConnectionRequestCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272c9ae0_7161_4ae0_91bd_9f448ee9c427);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IEnumPortableDeviceConnectors(::windows::core::IUnknown);
impl IEnumPortableDeviceConnectors {
    pub unsafe fn Next(&self, pconnectors: &mut [::core::option::Option<IPortableDeviceConnector>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pconnectors.len() as _, ::core::mem::transmute(pconnectors.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cconnectors: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cconnectors).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumPortableDeviceConnectors> {
        let mut result__ = ::windows::core::zeroed::<IEnumPortableDeviceConnectors>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumPortableDeviceConnectors, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumPortableDeviceConnectors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPortableDeviceConnectors {}
impl ::core::fmt::Debug for IEnumPortableDeviceConnectors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPortableDeviceConnectors").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumPortableDeviceConnectors {
    type Vtable = IEnumPortableDeviceConnectors_Vtbl;
}
impl ::core::clone::Clone for IEnumPortableDeviceConnectors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumPortableDeviceConnectors {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfdef549_9247_454f_bd82_06fe80853faa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceConnectors_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IEnumPortableDeviceObjectIDs(::windows::core::IUnknown);
impl IEnumPortableDeviceObjectIDs {
    pub unsafe fn Next(&self, pobjids: &mut [::windows::core::PWSTR], pcfetched: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pobjids.len() as _, ::core::mem::transmute(pobjids.as_ptr()), pcfetched)
    }
    pub unsafe fn Skip(&self, cobjects: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cobjects)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__ = ::windows::core::zeroed::<IEnumPortableDeviceObjectIDs>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumPortableDeviceObjectIDs, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumPortableDeviceObjectIDs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPortableDeviceObjectIDs {}
impl ::core::fmt::Debug for IEnumPortableDeviceObjectIDs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPortableDeviceObjectIDs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumPortableDeviceObjectIDs {
    type Vtable = IEnumPortableDeviceObjectIDs_Vtbl;
}
impl ::core::clone::Clone for IEnumPortableDeviceObjectIDs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumPortableDeviceObjectIDs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ece955_cf41_4728_bfa0_41eedf1bbf19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceObjectIDs_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut ::windows::core::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IMediaRadioManager(::windows::core::IUnknown);
impl IMediaRadioManager {
    pub unsafe fn GetRadioInstances(&self) -> ::windows::core::Result<IRadioInstanceCollection> {
        let mut result__ = ::windows::core::zeroed::<IRadioInstanceCollection>();
        (::windows::core::Interface::vtable(self).GetRadioInstances)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSystemRadioStateChange)(::windows::core::Interface::as_raw(self), sysradiostate, utimeoutsec).ok()
    }
}
::windows::imp::interface_hierarchy!(IMediaRadioManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMediaRadioManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaRadioManager {}
impl ::core::fmt::Debug for IMediaRadioManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaRadioManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMediaRadioManager {
    type Vtable = IMediaRadioManager_Vtbl;
}
impl ::core::clone::Clone for IMediaRadioManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaRadioManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cfdcab5_fc47_42a5_9241_074b58830e73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRadioInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSystemRadioStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IMediaRadioManagerNotifySink(::windows::core::IUnknown);
impl IMediaRadioManagerNotifySink {
    pub unsafe fn OnInstanceAdd<P0>(&self, pradioinstance: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRadioInstance>,
    {
        (::windows::core::Interface::vtable(self).OnInstanceAdd)(::windows::core::Interface::as_raw(self), pradioinstance.into_param().abi()).ok()
    }
    pub unsafe fn OnInstanceRemove<P0>(&self, bstrradioinstanceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).OnInstanceRemove)(::windows::core::Interface::as_raw(self), bstrradioinstanceid.into_param().abi()).ok()
    }
    pub unsafe fn OnInstanceRadioChange<P0>(&self, bstrradioinstanceid: P0, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).OnInstanceRadioChange)(::windows::core::Interface::as_raw(self), bstrradioinstanceid.into_param().abi(), radiostate).ok()
    }
}
::windows::imp::interface_hierarchy!(IMediaRadioManagerNotifySink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMediaRadioManagerNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaRadioManagerNotifySink {}
impl ::core::fmt::Debug for IMediaRadioManagerNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaRadioManagerNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMediaRadioManagerNotifySink {
    type Vtable = IMediaRadioManagerNotifySink_Vtbl;
}
impl ::core::clone::Clone for IMediaRadioManagerNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaRadioManagerNotifySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89d81f5f_c147_49ed_a11c_77b20c31e7c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManagerNotifySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnInstanceAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pradioinstance: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnInstanceRemove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub OnInstanceRadioChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDevice(::windows::core::IUnknown);
impl IPortableDevice {
    pub unsafe fn Open<P0, P1>(&self, pszpnpdeviceid: P0, pclientinfo: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    pub unsafe fn SendCommand<P0>(&self, dwflags: u32, pparameters: P0) -> ::windows::core::Result<IPortableDeviceValues>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).SendCommand)(::windows::core::Interface::as_raw(self), dwflags, pparameters.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Content(&self) -> ::windows::core::Result<IPortableDeviceContent> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceContent>();
        (::windows::core::Interface::vtable(self).Content)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Capabilities(&self) -> ::windows::core::Result<IPortableDeviceCapabilities> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceCapabilities>();
        (::windows::core::Interface::vtable(self).Capabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<P0, P1>(&self, dwflags: u32, pcallback: P0, pparameters: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceEventCallback>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), dwflags, pcallback.into_param().abi(), pparameters.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise<P0>(&self, pszcookie: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), pszcookie.into_param().abi()).ok()
    }
    pub unsafe fn GetPnPDeviceID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetPnPDeviceID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPortableDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevice {}
impl ::core::fmt::Debug for IPortableDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDevice {
    type Vtable = IPortableDevice_Vtbl;
}
impl ::core::clone::Clone for IPortableDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcookie: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetPnPDeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceCapabilities(::windows::core::IUnknown);
impl IPortableDeviceCapabilities {
    pub unsafe fn GetSupportedCommands(&self) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceKeyCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedCommands)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetCommandOptions)(::windows::core::Interface::as_raw(self), command, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFunctionalCategories(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetFunctionalCategories)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFunctionalObjects(&self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetFunctionalObjects)(::windows::core::Interface::as_raw(self), category, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedContentTypes(&self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedContentTypes)(::windows::core::Interface::as_raw(self), category, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedFormats(&self, contenttype: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedFormats)(::windows::core::Interface::as_raw(self), contenttype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceKeyCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedFormatProperties)(::windows::core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetFixedPropertyAttributes(&self, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetFixedPropertyAttributes)(::windows::core::Interface::as_raw(self), format, key, &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSupportedEvents(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedEvents)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEventOptions(&self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetEventOptions)(::windows::core::Interface::as_raw(self), event, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceCapabilities, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceCapabilities {}
impl ::core::fmt::Debug for IPortableDeviceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceCapabilities {
    type Vtable = IPortableDeviceCapabilities_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c8c6dbf_e3dc_4061_becc_8542e810d126);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceCapabilities_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSupportedCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcommands: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetCommandOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetCommandOptions: usize,
    pub GetFunctionalCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcategories: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFunctionalObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppobjectids: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSupportedContentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppcontenttypes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSupportedFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *const ::windows::core::GUID, ppformats: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetFixedPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetFixedPropertyAttributes: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSupportedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppevents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEventOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceConnector(::windows::core::IUnknown);
impl IPortableDeviceConnector {
    pub unsafe fn Connect<P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IConnectionRequestCallback>,
    {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect<P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IConnectionRequestCallback>,
    {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn Cancel<P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IConnectionRequestCallback>,
    {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub unsafe fn GetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), ppropertykey, ppropertytype, ppdata, pcbdata).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub unsafe fn SetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), ppropertykey, propertytype, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _).ok()
    }
    pub unsafe fn GetPnPID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetPnPID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceConnector, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceConnector {}
impl ::core::fmt::Debug for IPortableDeviceConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceConnector {
    type Vtable = IPortableDeviceConnector_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceConnector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Devices_Properties")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Devices_Properties")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))]
    SetProperty: usize,
    pub GetPnPID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceContent(::windows::core::IUnknown);
impl IPortableDeviceContent {
    pub unsafe fn EnumObjects<P0, P1>(&self, dwflags: u32, pszparentobjectid: P0, pfilter: P1) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumPortableDeviceObjectIDs>();
        (::windows::core::Interface::vtable(self).EnumObjects)(::windows::core::Interface::as_raw(self), dwflags, pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IPortableDeviceProperties> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceProperties>();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Transfer(&self) -> ::windows::core::Result<IPortableDeviceResources> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceResources>();
        (::windows::core::Interface::vtable(self).Transfer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateObjectWithPropertiesOnly<P0>(&self, pvalues: P0, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).CreateObjectWithPropertiesOnly)(::windows::core::Interface::as_raw(self), pvalues.into_param().abi(), ppszobjectid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateObjectWithPropertiesAndData<P0>(&self, pvalues: P0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).CreateObjectWithPropertiesAndData)(::windows::core::Interface::as_raw(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), pdwoptimalwritebuffersize, ppszcookie).ok()
    }
    pub unsafe fn Delete<P0>(&self, dwoptions: u32, pobjectids: P0, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), dwoptions, pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<P0>(&self, ppersistentuniqueids: P0) -> ::windows::core::Result<IPortableDevicePropVariantCollection>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetObjectIDsFromPersistentUniqueIDs)(::windows::core::Interface::as_raw(self), ppersistentuniqueids.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Move)(::windows::core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn Copy<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Copy)(::windows::core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceContent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceContent {}
impl ::core::fmt::Debug for IPortableDeviceContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceContent {
    type Vtable = IPortableDeviceContent_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceContent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a96ed84_7c73_4480_9938_bf5af477d426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: ::windows::core::PCWSTR, pfilter: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Transfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateObjectWithPropertiesOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: *mut ::core::ffi::c_void, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateObjectWithPropertiesAndData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateObjectWithPropertiesAndData: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectIDsFromPersistentUniqueIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppersistentuniqueids: *mut ::core::ffi::c_void, ppobjectids: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: *mut ::core::ffi::c_void, pszdestinationfolderobjectid: ::windows::core::PCWSTR, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: *mut ::core::ffi::c_void, pszdestinationfolderobjectid: ::windows::core::PCWSTR, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceContent2(::windows::core::IUnknown);
impl IPortableDeviceContent2 {
    pub unsafe fn EnumObjects<P0, P1>(&self, dwflags: u32, pszparentobjectid: P0, pfilter: P1) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumPortableDeviceObjectIDs>();
        (::windows::core::Interface::vtable(self).base__.EnumObjects)(::windows::core::Interface::as_raw(self), dwflags, pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IPortableDeviceProperties> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceProperties>();
        (::windows::core::Interface::vtable(self).base__.Properties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Transfer(&self) -> ::windows::core::Result<IPortableDeviceResources> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceResources>();
        (::windows::core::Interface::vtable(self).base__.Transfer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateObjectWithPropertiesOnly<P0>(&self, pvalues: P0, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateObjectWithPropertiesOnly)(::windows::core::Interface::as_raw(self), pvalues.into_param().abi(), ppszobjectid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateObjectWithPropertiesAndData<P0>(&self, pvalues: P0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateObjectWithPropertiesAndData)(::windows::core::Interface::as_raw(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), pdwoptimalwritebuffersize, ppszcookie).ok()
    }
    pub unsafe fn Delete<P0>(&self, dwoptions: u32, pobjectids: P0, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
    {
        (::windows::core::Interface::vtable(self).base__.Delete)(::windows::core::Interface::as_raw(self), dwoptions, pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<P0>(&self, ppersistentuniqueids: P0) -> ::windows::core::Result<IPortableDevicePropVariantCollection>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).base__.GetObjectIDsFromPersistentUniqueIDs)(::windows::core::Interface::as_raw(self), ppersistentuniqueids.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Move)(::windows::core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn Copy<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Copy)(::windows::core::Interface::as_raw(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateObjectWithPropertiesAndData<P0, P1>(&self, pszobjectid: P0, pproperties: P1, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).UpdateObjectWithPropertiesAndData)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), pproperties.into_param().abi(), ::core::mem::transmute(ppdata), pdwoptimalwritebuffersize).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceContent2, ::windows::core::IUnknown, IPortableDeviceContent);
impl ::core::cmp::PartialEq for IPortableDeviceContent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceContent2 {}
impl ::core::fmt::Debug for IPortableDeviceContent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceContent2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceContent2 {
    type Vtable = IPortableDeviceContent2_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceContent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceContent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b4add96_f6bf_4034_8708_eca72bf10554);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent2_Vtbl {
    pub base__: IPortableDeviceContent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateObjectWithPropertiesAndData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pproperties: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateObjectWithPropertiesAndData: usize,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPortableDeviceDataStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDataStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetObjectID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IPortableDeviceDataStream, ::windows::core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPortableDeviceDataStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPortableDeviceDataStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPortableDeviceDataStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceDataStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPortableDeviceDataStream {
    type Vtable = IPortableDeviceDataStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPortableDeviceDataStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IPortableDeviceDataStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88e04db3_1012_4d64_9996_f703a950d3f4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDataStream_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub GetObjectID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceDispatchFactory(::windows::core::IUnknown);
impl IPortableDeviceDispatchFactory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceDispatch<P0>(&self, pszpnpdeviceid: P0) -> ::windows::core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).GetDeviceDispatch)(::windows::core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceDispatchFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceDispatchFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceDispatchFactory {}
impl ::core::fmt::Debug for IPortableDeviceDispatchFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceDispatchFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceDispatchFactory {
    type Vtable = IPortableDeviceDispatchFactory_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceDispatchFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceDispatchFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e1eafc3_e3d7_4132_96fa_759c0f9d1e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDispatchFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceDispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, ppdevicedispatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceDispatch: usize,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceEventCallback(::windows::core::IUnknown);
impl IPortableDeviceEventCallback {
    pub unsafe fn OnEvent<P0>(&self, peventparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).OnEvent)(::windows::core::Interface::as_raw(self), peventparameters.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceEventCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceEventCallback {}
impl ::core::fmt::Debug for IPortableDeviceEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceEventCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceEventCallback {
    type Vtable = IPortableDeviceEventCallback_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceEventCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8792a31_f385_493c_a893_40f64eb45f6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceEventCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceKeyCollection(::windows::core::IUnknown);
impl IPortableDeviceKeyCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelems).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), dwindex, pkey).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::windows::core::Interface::as_raw(self), dwindex).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceKeyCollection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceKeyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceKeyCollection {}
impl ::core::fmt::Debug for IPortableDeviceKeyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceKeyCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceKeyCollection {
    type Vtable = IPortableDeviceKeyCollection_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceKeyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceKeyCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdada2357_e0ad_492e_98db_dd61c53ba353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceKeyCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetAt: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceManager(::windows::core::IUnknown);
impl IPortableDeviceManager {
    pub unsafe fn GetDevices(&self, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDevices)(::windows::core::Interface::as_raw(self), ppnpdeviceids, pcpnpdeviceids).ok()
    }
    pub unsafe fn RefreshDeviceList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshDeviceList)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceFriendlyName<P0>(&self, pszpnpdeviceid: P0, pdevicefriendlyname: ::windows::core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDeviceFriendlyName)(::windows::core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(pdevicefriendlyname), pcchdevicefriendlyname).ok()
    }
    pub unsafe fn GetDeviceDescription<P0>(&self, pszpnpdeviceid: P0, pdevicedescription: ::windows::core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDeviceDescription)(::windows::core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(pdevicedescription), pcchdevicedescription).ok()
    }
    pub unsafe fn GetDeviceManufacturer<P0>(&self, pszpnpdeviceid: P0, pdevicemanufacturer: ::windows::core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDeviceManufacturer)(::windows::core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(pdevicemanufacturer), pcchdevicemanufacturer).ok()
    }
    pub unsafe fn GetDeviceProperty<P0, P1>(&self, pszpnpdeviceid: P0, pszdevicepropertyname: P1, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDeviceProperty)(::windows::core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), pszdevicepropertyname.into_param().abi(), pdata, pcbdata, pdwtype).ok()
    }
    pub unsafe fn GetPrivateDevices(&self, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateDevices)(::windows::core::Interface::as_raw(self), ppnpdeviceids, pcpnpdeviceids).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceManager {}
impl ::core::fmt::Debug for IPortableDeviceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceManager {
    type Vtable = IPortableDeviceManager_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1567595_4c2f_4574_a6fa_ecef917b9a40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT,
    pub RefreshDeviceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pdevicefriendlyname: ::windows::core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pdevicedescription: ::windows::core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pdevicemanufacturer: ::windows::core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pszdevicepropertyname: ::windows::core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetPrivateDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDevicePropVariantCollection(::windows::core::IUnknown);
impl IPortableDevicePropVariantCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelems).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), dwindex, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Add(&self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), pvalue).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ChangeType(&self, vt: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangeType)(::windows::core::Interface::as_raw(self), vt).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::windows::core::Interface::as_raw(self), dwindex).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDevicePropVariantCollection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDevicePropVariantCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropVariantCollection {}
impl ::core::fmt::Debug for IPortableDevicePropVariantCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropVariantCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDevicePropVariantCollection {
    type Vtable = IPortableDevicePropVariantCollection_Vtbl;
}
impl ::core::clone::Clone for IPortableDevicePropVariantCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDevicePropVariantCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89b2e422_4f1b_4316_bcef_a44afea83eb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropVariantCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Add: usize,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows::core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceProperties(::windows::core::IUnknown);
impl IPortableDeviceProperties {
    pub unsafe fn GetSupportedProperties<P0>(&self, pszobjectid: P0) -> ::windows::core::Result<IPortableDeviceKeyCollection>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceKeyCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedProperties)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetPropertyAttributes<P0>(&self, pszobjectid: P0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetPropertyAttributes)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), key, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetValues<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> ::windows::core::Result<IPortableDeviceValues>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceKeyCollection>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetValues)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), pkeys.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetValues<P0, P1>(&self, pszobjectid: P0, pvalues: P1) -> ::windows::core::Result<IPortableDeviceValues>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).SetValues)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), pvalues.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceKeyCollection>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceProperties, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceProperties {}
impl ::core::fmt::Debug for IPortableDeviceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceProperties {
    type Vtable = IPortableDeviceProperties_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f6d695c_03df_4439_a809_59266beee3a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSupportedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetPropertyAttributes: usize,
    pub GetValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pkeys: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pvalues: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pkeys: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDevicePropertiesBulk(::windows::core::IUnknown);
impl IPortableDevicePropertiesBulk {
    pub unsafe fn QueueGetValuesByObjectList<P0, P1, P2>(&self, pobjectids: P0, pkeys: P1, pcallback: P2) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
        P1: ::windows::core::IntoParam<IPortableDeviceKeyCollection>,
        P2: ::windows::core::IntoParam<IPortableDevicePropertiesBulkCallback>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).QueueGetValuesByObjectList)(::windows::core::Interface::as_raw(self), pobjectids.into_param().abi(), pkeys.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueueGetValuesByObjectFormat<P0, P1, P2>(&self, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: P0, dwdepth: u32, pkeys: P1, pcallback: P2) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceKeyCollection>,
        P2: ::windows::core::IntoParam<IPortableDevicePropertiesBulkCallback>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).QueueGetValuesByObjectFormat)(::windows::core::Interface::as_raw(self), pguidobjectformat, pszparentobjectid.into_param().abi(), dwdepth, pkeys.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueueSetValuesByObjectList<P0, P1>(&self, pobjectvalues: P0, pcallback: P1) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValuesCollection>,
        P1: ::windows::core::IntoParam<IPortableDevicePropertiesBulkCallback>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).QueueSetValuesByObjectList)(::windows::core::Interface::as_raw(self), pobjectvalues.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), pcontext).ok()
    }
    pub unsafe fn Cancel(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self), pcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDevicePropertiesBulk, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDevicePropertiesBulk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropertiesBulk {}
impl ::core::fmt::Debug for IPortableDevicePropertiesBulk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropertiesBulk").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDevicePropertiesBulk {
    type Vtable = IPortableDevicePropertiesBulk_Vtbl;
}
impl ::core::clone::Clone for IPortableDevicePropertiesBulk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDevicePropertiesBulk {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x482b05c0_4056_44ed_9e0f_5e23b009da93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulk_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueueGetValuesByObjectList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: *mut ::core::ffi::c_void, pkeys: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub QueueGetValuesByObjectFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: ::windows::core::PCWSTR, dwdepth: u32, pkeys: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub QueueSetValuesByObjectList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectvalues: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDevicePropertiesBulkCallback(::windows::core::IUnknown);
impl IPortableDevicePropertiesBulkCallback {
    pub unsafe fn OnStart(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStart)(::windows::core::Interface::as_raw(self), pcontext).ok()
    }
    pub unsafe fn OnProgress<P0>(&self, pcontext: *const ::windows::core::GUID, presults: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValuesCollection>,
    {
        (::windows::core::Interface::vtable(self).OnProgress)(::windows::core::Interface::as_raw(self), pcontext, presults.into_param().abi()).ok()
    }
    pub unsafe fn OnEnd(&self, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEnd)(::windows::core::Interface::as_raw(self), pcontext, hrstatus).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDevicePropertiesBulkCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDevicePropertiesBulkCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropertiesBulkCallback {}
impl ::core::fmt::Debug for IPortableDevicePropertiesBulkCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropertiesBulkCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDevicePropertiesBulkCallback {
    type Vtable = IPortableDevicePropertiesBulkCallback_Vtbl;
}
impl ::core::clone::Clone for IPortableDevicePropertiesBulkCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDevicePropertiesBulkCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9deacb80_11e8_40e3_a9f3_f557986a7845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulkCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, presults: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceResources(::windows::core::IUnknown);
impl IPortableDeviceResources {
    pub unsafe fn GetSupportedResources<P0>(&self, pszobjectid: P0) -> ::windows::core::Result<IPortableDeviceKeyCollection>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceKeyCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedResources)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetResourceAttributes<P0>(&self, pszobjectid: P0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetResourceAttributes)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetStream<P0>(&self, pszobjectid: P0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetStream)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), key, dwmode, pdwoptimalbuffersize, ::core::mem::transmute(ppstream)).ok()
    }
    pub unsafe fn Delete<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceKeyCollection>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResource<P0>(&self, presourceattributes: P0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).CreateResource)(::windows::core::Interface::as_raw(self), presourceattributes.into_param().abi(), ::core::mem::transmute(ppdata), pdwoptimalwritebuffersize, ppszcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceResources, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceResources {}
impl ::core::fmt::Debug for IPortableDeviceResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceResources").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceResources {
    type Vtable = IPortableDeviceResources_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceResources {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd8878ac_d841_4d17_891c_e6829cdb6934);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceResources_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSupportedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetResourceAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetResourceAttributes: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetStream: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pkeys: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourceattributes: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResource: usize,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceService(::windows::core::IUnknown);
impl IPortableDeviceService {
    pub unsafe fn Open<P0, P1>(&self, pszpnpserviceid: P0, pclientinfo: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    pub unsafe fn Capabilities(&self) -> ::windows::core::Result<IPortableDeviceServiceCapabilities> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceServiceCapabilities>();
        (::windows::core::Interface::vtable(self).Capabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Content(&self) -> ::windows::core::Result<IPortableDeviceContent2> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceContent2>();
        (::windows::core::Interface::vtable(self).Content)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Methods(&self) -> ::windows::core::Result<IPortableDeviceServiceMethods> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceServiceMethods>();
        (::windows::core::Interface::vtable(self).Methods)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetServiceObjectID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetServiceObjectID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPnPServiceID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetPnPServiceID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Advise<P0, P1>(&self, dwflags: u32, pcallback: P0, pparameters: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceEventCallback>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), dwflags, pcallback.into_param().abi(), pparameters.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise<P0>(&self, pszcookie: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), pszcookie.into_param().abi()).ok()
    }
    pub unsafe fn SendCommand<P0>(&self, dwflags: u32, pparameters: P0) -> ::windows::core::Result<IPortableDeviceValues>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).SendCommand)(::windows::core::Interface::as_raw(self), dwflags, pparameters.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceService {}
impl ::core::fmt::Debug for IPortableDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceService {
    type Vtable = IPortableDeviceService_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3bd3a44_d7b5_40a9_98b7_2fa4d01dec08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows::core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Methods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetServiceObjectID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPnPServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcookie: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceActivation(::windows::core::IUnknown);
impl IPortableDeviceServiceActivation {
    pub unsafe fn OpenAsync<P0, P1, P2>(&self, pszpnpserviceid: P0, pclientinfo: P1, pcallback: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IPortableDeviceValues>,
        P2: ::windows::core::IntoParam<IPortableDeviceServiceOpenCallback>,
    {
        (::windows::core::Interface::vtable(self).OpenAsync)(::windows::core::Interface::as_raw(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelOpenAsync(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelOpenAsync)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceServiceActivation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceServiceActivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceActivation {}
impl ::core::fmt::Debug for IPortableDeviceServiceActivation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceActivation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceServiceActivation {
    type Vtable = IPortableDeviceServiceActivation_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceServiceActivation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceServiceActivation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe56b0534_d9b9_425c_9b99_75f97cb3d7c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceActivation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows::core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelOpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceCapabilities(::windows::core::IUnknown);
impl IPortableDeviceServiceCapabilities {
    pub unsafe fn GetSupportedMethods(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedMethods)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedMethodsByFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedMethodsByFormat)(::windows::core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMethodAttributes(&self, method: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetMethodAttributes)(::windows::core::Interface::as_raw(self), method, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetMethodParameterAttributes(&self, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetMethodParameterAttributes)(::windows::core::Interface::as_raw(self), method, parameter, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedFormats(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedFormats)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFormatAttributes(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetFormatAttributes)(::windows::core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceKeyCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedFormatProperties)(::windows::core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetFormatPropertyAttributes(&self, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetFormatPropertyAttributes)(::windows::core::Interface::as_raw(self), format, property, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedEvents(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedEvents)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEventAttributes(&self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetEventAttributes)(::windows::core::Interface::as_raw(self), event, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetEventParameterAttributes(&self, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetEventParameterAttributes)(::windows::core::Interface::as_raw(self), event, parameter, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInheritedServices(&self, dwinheritancetype: u32) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetInheritedServices)(::windows::core::Interface::as_raw(self), dwinheritancetype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFormatRenderingProfiles(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValuesCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValuesCollection>();
        (::windows::core::Interface::vtable(self).GetFormatRenderingProfiles)(::windows::core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedCommands(&self) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceKeyCollection>();
        (::windows::core::Interface::vtable(self).GetSupportedCommands)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetCommandOptions)(::windows::core::Interface::as_raw(self), command, &mut result__).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceServiceCapabilities, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceServiceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceCapabilities {}
impl ::core::fmt::Debug for IPortableDeviceServiceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceServiceCapabilities {
    type Vtable = IPortableDeviceServiceCapabilities_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceServiceCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceServiceCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24dbd89d_413e_43e0_bd5b_197f3c56c886);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceCapabilities_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSupportedMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSupportedMethodsByFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMethodAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetMethodParameterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetMethodParameterAttributes: usize,
    pub GetSupportedFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformats: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFormatAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetFormatPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetFormatPropertyAttributes: usize,
    pub GetSupportedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppevents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEventAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetEventParameterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetEventParameterAttributes: usize,
    pub GetInheritedServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFormatRenderingProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, pprenderingprofiles: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSupportedCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcommands: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetCommandOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetCommandOptions: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceManager(::windows::core::IUnknown);
impl IPortableDeviceServiceManager {
    pub unsafe fn GetDeviceServices<P0>(&self, pszpnpdeviceid: P0, guidservicecategory: *const ::windows::core::GUID, pservices: *mut ::windows::core::PWSTR, pcservices: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDeviceServices)(::windows::core::Interface::as_raw(self), pszpnpdeviceid.into_param().abi(), guidservicecategory, pservices, pcservices).ok()
    }
    pub unsafe fn GetDeviceForService<P0>(&self, pszpnpserviceid: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDeviceForService)(::windows::core::Interface::as_raw(self), pszpnpserviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceServiceManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceServiceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceManager {}
impl ::core::fmt::Debug for IPortableDeviceServiceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceServiceManager {
    type Vtable = IPortableDeviceServiceManager_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceServiceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceServiceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8abc4e9_a84a_47a9_80b3_c5d9b172a961);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDeviceServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, guidservicecategory: *const ::windows::core::GUID, pservices: *mut ::windows::core::PWSTR, pcservices: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceForService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows::core::PCWSTR, ppszpnpdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceMethodCallback(::windows::core::IUnknown);
impl IPortableDeviceServiceMethodCallback {
    pub unsafe fn OnComplete<P0>(&self, hrstatus: ::windows::core::HRESULT, presults: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).OnComplete)(::windows::core::Interface::as_raw(self), hrstatus, presults.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceServiceMethodCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceServiceMethodCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceMethodCallback {}
impl ::core::fmt::Debug for IPortableDeviceServiceMethodCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceMethodCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceServiceMethodCallback {
    type Vtable = IPortableDeviceServiceMethodCallback_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceServiceMethodCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceServiceMethodCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc424233c_afce_4828_a756_7ed7a2350083);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethodCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, presults: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceMethods(::windows::core::IUnknown);
impl IPortableDeviceServiceMethods {
    pub unsafe fn Invoke<P0>(&self, method: *const ::windows::core::GUID, pparameters: P0, ppresults: *mut ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).Invoke)(::windows::core::Interface::as_raw(self), method, pparameters.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn InvokeAsync<P0, P1>(&self, method: *const ::windows::core::GUID, pparameters: P0, pcallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
        P1: ::windows::core::IntoParam<IPortableDeviceServiceMethodCallback>,
    {
        (::windows::core::Interface::vtable(self).InvokeAsync)(::windows::core::Interface::as_raw(self), method, pparameters.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn Cancel<P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceServiceMethodCallback>,
    {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceServiceMethods, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceServiceMethods {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceMethods {}
impl ::core::fmt::Debug for IPortableDeviceServiceMethods {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceMethods").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceServiceMethods {
    type Vtable = IPortableDeviceServiceMethods_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceServiceMethods {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceServiceMethods {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe20333c9_fd34_412d_a381_cc6f2d820df7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethods_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InvokeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceOpenCallback(::windows::core::IUnknown);
impl IPortableDeviceServiceOpenCallback {
    pub unsafe fn OnComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnComplete)(::windows::core::Interface::as_raw(self), hrstatus).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceServiceOpenCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceServiceOpenCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceOpenCallback {}
impl ::core::fmt::Debug for IPortableDeviceServiceOpenCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceOpenCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceServiceOpenCallback {
    type Vtable = IPortableDeviceServiceOpenCallback_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceServiceOpenCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceServiceOpenCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbced49c8_8efe_41ed_960b_61313abd47a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceOpenCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceUnitsStream(::windows::core::IUnknown);
impl IPortableDeviceUnitsStream {
    pub unsafe fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SeekInUnits)(::windows::core::Interface::as_raw(self), dlibmove, units, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceUnitsStream, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceUnitsStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceUnitsStream {}
impl ::core::fmt::Debug for IPortableDeviceUnitsStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceUnitsStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceUnitsStream {
    type Vtable = IPortableDeviceUnitsStream_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceUnitsStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceUnitsStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e98025f_bfc4_47a2_9a5f_bc900a507c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceUnitsStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SeekInUnits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceValues(::windows::core::IUnknown);
impl IPortableDeviceValues {
    pub unsafe fn GetCount(&self, pcelt: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelt).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetAt(&self, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), index, pkey, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), key, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetStringValue<P0>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStringValue)(::windows::core::Interface::as_raw(self), key, value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetStringValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetStringValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUnsignedIntegerValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetUnsignedIntegerValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSignedIntegerValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetSignedIntegerValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUnsignedLargeIntegerValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetUnsignedLargeIntegerValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSignedLargeIntegerValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetSignedLargeIntegerValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFloatValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetFloatValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetErrorValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
        (::windows::core::Interface::vtable(self).GetErrorValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetKeyValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKeyValue)(::windows::core::Interface::as_raw(self), key, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetBoolValue<P0>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBoolValue)(::windows::core::Interface::as_raw(self), key, value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetBoolValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetBoolValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIUnknownValue<P0>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetIUnknownValue)(::windows::core::Interface::as_raw(self), key, pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIUnknownValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetIUnknownValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGuidValue)(::windows::core::Interface::as_raw(self), key, value).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetGuidValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferValue)(::windows::core::Interface::as_raw(self), key, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferValue)(::windows::core::Interface::as_raw(self), key, ppvalue, pcbvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDeviceValuesValue<P0>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).SetIPortableDeviceValuesValue)(::windows::core::Interface::as_raw(self), key, pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDeviceValuesValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetIPortableDeviceValuesValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDevicePropVariantCollectionValue<P0>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDevicePropVariantCollection>,
    {
        (::windows::core::Interface::vtable(self).SetIPortableDevicePropVariantCollectionValue)(::windows::core::Interface::as_raw(self), key, pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDevicePropVariantCollection>();
        (::windows::core::Interface::vtable(self).GetIPortableDevicePropVariantCollectionValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDeviceKeyCollectionValue<P0>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceKeyCollection>,
    {
        (::windows::core::Interface::vtable(self).SetIPortableDeviceKeyCollectionValue)(::windows::core::Interface::as_raw(self), key, pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceKeyCollection>();
        (::windows::core::Interface::vtable(self).GetIPortableDeviceKeyCollectionValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDeviceValuesCollectionValue<P0>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValuesCollection>,
    {
        (::windows::core::Interface::vtable(self).SetIPortableDeviceValuesCollectionValue)(::windows::core::Interface::as_raw(self), key, pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValuesCollection> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValuesCollection>();
        (::windows::core::Interface::vtable(self).GetIPortableDeviceValuesCollectionValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn RemoveValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveValue)(::windows::core::Interface::as_raw(self), key).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CopyValuesFromPropertyStore<P0>(&self, pstore: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (::windows::core::Interface::vtable(self).CopyValuesFromPropertyStore)(::windows::core::Interface::as_raw(self), pstore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CopyValuesToPropertyStore<P0>(&self, pstore: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (::windows::core::Interface::vtable(self).CopyValuesToPropertyStore)(::windows::core::Interface::as_raw(self), pstore.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceValues, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceValues {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceValues {}
impl ::core::fmt::Debug for IPortableDeviceValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceValues").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceValues {
    type Vtable = IPortableDeviceValues_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceValues {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6848f6f2_3155_4f86_b6f5_263eeeab3143);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValues_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetStringValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetStringValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetUnsignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetUnsignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetUnsignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetUnsignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetSignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetSignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSignedIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSignedIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetUnsignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetUnsignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetUnsignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetUnsignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetSignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetSignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSignedLargeIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSignedLargeIntegerValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetFloatValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetFloatValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetFloatValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetFloatValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetErrorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetErrorValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetErrorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetErrorValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetKeyValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetKeyValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetBoolValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetBoolValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetBoolValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetBoolValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIUnknownValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIUnknownValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIUnknownValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIUnknownValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetGuidValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetGuidValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetGuidValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetGuidValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetBufferValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetBufferValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetBufferValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetBufferValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDeviceValuesValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDeviceValuesValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDeviceValuesValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDeviceValuesValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub RemoveValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    RemoveValue: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CopyValuesFromPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CopyValuesFromPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CopyValuesToPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CopyValuesToPropertyStore: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IPortableDeviceValuesCollection(::windows::core::IUnknown);
impl IPortableDeviceValuesCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelems).ok()
    }
    pub unsafe fn GetAt(&self, dwindex: u32) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn Add<P0>(&self, pvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), pvalues.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::windows::core::Interface::as_raw(self), dwindex).ok()
    }
}
::windows::imp::interface_hierarchy!(IPortableDeviceValuesCollection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPortableDeviceValuesCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceValuesCollection {}
impl ::core::fmt::Debug for IPortableDeviceValuesCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceValuesCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceValuesCollection {
    type Vtable = IPortableDeviceValuesCollection_Vtbl;
}
impl ::core::clone::Clone for IPortableDeviceValuesCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPortableDeviceValuesCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e3f2d79_4e07_48c4_8208_d8c2e5af4a99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValuesCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPortableDeviceWebControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceWebControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceFromId<P0>(&self, deviceid: P0) -> ::windows::core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).GetDeviceFromId)(::windows::core::Interface::as_raw(self), deviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceFromIdAsync<P0, P1, P2>(&self, deviceid: P0, pcompletionhandler: P1, perrorhandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).GetDeviceFromIdAsync)(::windows::core::Interface::as_raw(self), deviceid.into_param().abi(), pcompletionhandler.into_param().abi(), perrorhandler.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IPortableDeviceWebControl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPortableDeviceWebControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPortableDeviceWebControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPortableDeviceWebControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceWebControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPortableDeviceWebControl {
    type Vtable = IPortableDeviceWebControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPortableDeviceWebControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IPortableDeviceWebControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94fc7953_5ca1_483a_8aee_df52e7747d00);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceWebControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceFromId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcompletionhandler: *mut ::core::ffi::c_void, perrorhandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceFromIdAsync: usize,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IRadioInstance(::windows::core::IUnknown);
impl IRadioInstance {
    pub unsafe fn GetRadioManagerSignature(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetRadioManagerSignature)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInstanceSignature(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetInstanceSignature)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFriendlyName(&self, lcid: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetFriendlyName)(::windows::core::Interface::as_raw(self), lcid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRadioState(&self) -> ::windows::core::Result<DEVICE_RADIO_STATE> {
        let mut result__ = ::windows::core::zeroed::<DEVICE_RADIO_STATE>();
        (::windows::core::Interface::vtable(self).GetRadioState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRadioState)(::windows::core::Interface::as_raw(self), radiostate, utimeoutsec).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMultiComm(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsMultiComm)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAssociatingDevice(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsAssociatingDevice)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IRadioInstance, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRadioInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadioInstance {}
impl ::core::fmt::Debug for IRadioInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadioInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRadioInstance {
    type Vtable = IRadioInstance_Vtbl;
}
impl ::core::clone::Clone for IRadioInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRadioInstance {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70aa1c9e_f2b4_4c61_86d3_6b9fb75fd1a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstance_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRadioManagerSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetInstanceSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows::core::HRESULT,
    pub SetRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMultiComm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMultiComm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAssociatingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAssociatingDevice: usize,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IRadioInstanceCollection(::windows::core::IUnknown);
impl IRadioInstanceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, uindex: u32) -> ::windows::core::Result<IRadioInstance> {
        let mut result__ = ::windows::core::zeroed::<IRadioInstance>();
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRadioInstanceCollection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRadioInstanceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadioInstanceCollection {}
impl ::core::fmt::Debug for IRadioInstanceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadioInstanceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRadioInstanceCollection {
    type Vtable = IRadioInstanceCollection_Vtbl;
}
impl ::core::clone::Clone for IRadioInstanceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRadioInstanceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5791fae_5665_4e0c_95be_5fde31644185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstanceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
pub struct IWpdSerializer(::windows::core::IUnknown);
impl IWpdSerializer {
    pub unsafe fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: &[u8]) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__ = ::windows::core::zeroed::<IPortableDeviceValues>();
        (::windows::core::Interface::vtable(self).GetIPortableDeviceValuesFromBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteIPortableDeviceValuesToBuffer<P0>(&self, presults: P0, pbuffer: &mut [u8], pdwbyteswritten: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).WriteIPortableDeviceValuesToBuffer)(::windows::core::Interface::as_raw(self), pbuffer.len() as _, presults.into_param().abi(), ::core::mem::transmute(pbuffer.as_ptr()), pdwbyteswritten).ok()
    }
    pub unsafe fn GetBufferFromIPortableDeviceValues<P0>(&self, psource: P0, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        (::windows::core::Interface::vtable(self).GetBufferFromIPortableDeviceValues)(::windows::core::Interface::as_raw(self), psource.into_param().abi(), ppbuffer, pdwbuffersize).ok()
    }
    pub unsafe fn GetSerializedSize<P0>(&self, psource: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IPortableDeviceValues>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSerializedSize)(::windows::core::Interface::as_raw(self), psource.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWpdSerializer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWpdSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWpdSerializer {}
impl ::core::fmt::Debug for IWpdSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWpdSerializer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWpdSerializer {
    type Vtable = IWpdSerializer_Vtbl;
}
impl ::core::clone::Clone for IWpdSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWpdSerializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32f4002_bb27_45ff_af4f_06631c1e8dad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWpdSerializer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetIPortableDeviceValuesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteIPortableDeviceValuesToBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: *mut ::core::ffi::c_void, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::HRESULT,
    pub GetBufferFromIPortableDeviceValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerializedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const CLSID_WPD_NAMESPACE_EXTENSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35786d3c_b075_49b9_88dd_029876e11c01);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_MTPBTH_IsConnected: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0xea1237fa_589d_4472_84e4_0abe36fd62ef), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DEVSVCTYPE_ABSTRACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DEVSVCTYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DEVSVC_SERVICEINFO_VERSION: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_AnchorStateInvalid: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_AnchorStateNormal: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_AnchorStateOld: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateChanged: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateCreated: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateDeleted: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateInvalid: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_AnchorResults_ItemStateUpdated: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusBusy: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusFree: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusOutOfOffice: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_CalendarObj_BusyStatusTentative: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_DeviceMetadataObj_DefaultCABFalse: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_DeviceMetadataObj_DefaultCABTrue: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceFirst: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceFourth: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceLast: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceNone: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceSecond: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternInstanceThird: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeDaily: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeMonthly: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeWeekly: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PatternTypeYearly: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PriorityHighest: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PriorityLowest: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_PriorityNormal: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_ReadFalse: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_MessageObj_ReadTrue: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_ChargingActive: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_ChargingInactive: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_ChargingUnknown: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_RoamingActive: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_RoamingInactive: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_StatusSvc_RoamingUnknown: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_SyncSvc_SyncObjectReferencesDisabled: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_SyncSvc_SyncObjectReferencesEnabled: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_TaskObj_CompleteFalse: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const ENUM_TaskObj_CompleteTrue: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_DEVICE_ALREADY_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731135i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_DEVICE_IS_HUNG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731130i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_DEVICE_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731134i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731133i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730934i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731132i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730933i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_OBJECT_NOT_COMMITED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731131i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SERVICE_ALREADY_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730936i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SERVICE_BAD_PARAMETER_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730932i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SERVICE_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730935i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SMS_INVALID_MESSAGE_BODY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731035i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SMS_INVALID_RECIPIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731036i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const E_WPD_SMS_SERVICE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731034i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const EnumBthMtpConnectors: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1570149_e645_4f43_8b0d_409b061db2fc);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FACILITY_WPD: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekFriday: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekMonday: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekNone: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekSaturday: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekSunday: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekThursday: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekTuesday: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const FLAG_MessageObj_DayOfWeekWednesday: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const GUID_DEVINTERFACE_WPD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac27878_a6fa_4155_ba85_f98f491d4f33);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const GUID_DEVINTERFACE_WPD_PRIVATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba0c718f_4ded_49b7_bdd3_fabe28661211);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const GUID_DEVINTERFACE_WPD_SERVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ef44f80_3d64_4246_a6aa_206f328d1edc);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const IOCTL_WPD_MESSAGE_READWRITE_ACCESS: u32 = 4243720u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const IOCTL_WPD_MESSAGE_READ_ACCESS: u32 = 4210952u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_3GPP2File: ::windows::core::PCWSTR = ::windows::core::w!("3GPP2File");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_3GPPFile: ::windows::core::PCWSTR = ::windows::core::w!("3GPPFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AACFile: ::windows::core::PCWSTR = ::windows::core::w!("AACFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AIFFFile: ::windows::core::PCWSTR = ::windows::core::w!("AIFFFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AMRFile: ::windows::core::PCWSTR = ::windows::core::w!("AMRFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ASFFile: ::windows::core::PCWSTR = ::windows::core::w!("ASFFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ASXPlaylist: ::windows::core::PCWSTR = ::windows::core::w!("ASXPlaylist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ATSCTSFile: ::windows::core::PCWSTR = ::windows::core::w!("ATSCTSFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AVCHDFile: ::windows::core::PCWSTR = ::windows::core::w!("AVCHDFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AVIFile: ::windows::core::PCWSTR = ::windows::core::w!("AVIFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractActivity: ::windows::core::PCWSTR = ::windows::core::w!("AbstractActivity");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractActivityOccurrence: ::windows::core::PCWSTR = ::windows::core::w!("AbstractActivityOccurrence");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractAudioAlbum: ::windows::core::PCWSTR = ::windows::core::w!("AbstractAudioAlbum");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractAudioPlaylist: ::windows::core::PCWSTR = ::windows::core::w!("AbstractAudioPlaylist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractAudioVideoAlbum: ::windows::core::PCWSTR = ::windows::core::w!("AbstractAudioVideoAlbum");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractChapteredProduction: ::windows::core::PCWSTR = ::windows::core::w!("AbstractChapteredProduction");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractContact: ::windows::core::PCWSTR = ::windows::core::w!("AbstractContact");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractContactGroup: ::windows::core::PCWSTR = ::windows::core::w!("AbstractContactGroup");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractDocument: ::windows::core::PCWSTR = ::windows::core::w!("AbstractDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractImageAlbum: ::windows::core::PCWSTR = ::windows::core::w!("AbstractImageAlbum");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMediacast: ::windows::core::PCWSTR = ::windows::core::w!("AbstractMediacast");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMessage: ::windows::core::PCWSTR = ::windows::core::w!("AbstractMessage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMessageFolder: ::windows::core::PCWSTR = ::windows::core::w!("AbstractMessageFolder");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractMultimediaAlbum: ::windows::core::PCWSTR = ::windows::core::w!("AbstractMultimediaAlbum");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractNote: ::windows::core::PCWSTR = ::windows::core::w!("AbstractNote");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractTask: ::windows::core::PCWSTR = ::windows::core::w!("AbstractTask");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractVideoAlbum: ::windows::core::PCWSTR = ::windows::core::w!("AbstractVideoAlbum");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AbstractVideoPlaylist: ::windows::core::PCWSTR = ::windows::core::w!("AbstractVideoPlaylist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults: ::windows::core::PCWSTR = ::windows::core::w!("AnchorResults");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults_Anchor: ::windows::core::PCWSTR = ::windows::core::w!("Anchor");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults_AnchorState: ::windows::core::PCWSTR = ::windows::core::w!("AnchorState");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorResults_ResultObjectID: ::windows::core::PCWSTR = ::windows::core::w!("ResultObjectID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncKnowledge: ::windows::core::PCWSTR = ::windows::core::w!("AnchorSyncKnowledge");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc: ::windows::core::PCWSTR = ::windows::core::w!("AnchorSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_BeginSync: ::windows::core::PCWSTR = ::windows::core::w!("BeginSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_CurrentAnchor: ::windows::core::PCWSTR = ::windows::core::w!("AnchorCurrentAnchor");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_EndSync: ::windows::core::PCWSTR = ::windows::core::w!("EndSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_FilterType: ::windows::core::PCWSTR = ::windows::core::w!("FilterType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_GetChangesSinceAnchor: ::windows::core::PCWSTR = ::windows::core::w!("GetChangesSinceAnchor");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_KnowledgeObjectID: ::windows::core::PCWSTR = ::windows::core::w!("AnchorKnowledgeObjectID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_LastSyncProxyID: ::windows::core::PCWSTR = ::windows::core::w!("AnchorLastSyncProxyID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_LocalOnlyDelete: ::windows::core::PCWSTR = ::windows::core::w!("LocalOnlyDelete");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_ProviderVersion: ::windows::core::PCWSTR = ::windows::core::w!("AnchorProviderVersion");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_ReplicaID: ::windows::core::PCWSTR = ::windows::core::w!("AnchorReplicaID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_SyncFormat: ::windows::core::PCWSTR = ::windows::core::w!("SyncFormat");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AnchorSyncSvc_VersionProps: ::windows::core::PCWSTR = ::windows::core::w!("AnchorVersionProps");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Association: ::windows::core::PCWSTR = ::windows::core::w!("Association");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudibleFile: ::windows::core::PCWSTR = ::windows::core::w!("AudibleFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioBitDepth: ::windows::core::PCWSTR = ::windows::core::w!("AudioBitDepth");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioBitRate: ::windows::core::PCWSTR = ::windows::core::w!("AudioBitRate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioBlockAlignment: ::windows::core::PCWSTR = ::windows::core::w!("AudioBlockAlignment");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_AudioFormatCode: ::windows::core::PCWSTR = ::windows::core::w!("AudioFormatCode");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_Channels: ::windows::core::PCWSTR = ::windows::core::w!("Channels");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_AudioObj_Lyrics: ::windows::core::PCWSTR = ::windows::core::w!("Lyrics");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_BMPImage: ::windows::core::PCWSTR = ::windows::core::w!("BMPImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CIFFImage: ::windows::core::PCWSTR = ::windows::core::w!("CIFFImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Accepted: ::windows::core::PCWSTR = ::windows::core::w!("Accepted");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_BeginDateTime: ::windows::core::PCWSTR = ::windows::core::w!("BeginDateTime");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_BusyStatus: ::windows::core::PCWSTR = ::windows::core::w!("BusyStatus");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Declined: ::windows::core::PCWSTR = ::windows::core::w!("Declined");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_EndDateTime: ::windows::core::PCWSTR = ::windows::core::w!("EndDateTime");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Location: ::windows::core::PCWSTR = ::windows::core::w!("Location");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_PatternDuration: ::windows::core::PCWSTR = ::windows::core::w!("PatternDuration");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_PatternStartTime: ::windows::core::PCWSTR = ::windows::core::w!("PatternStartTime");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_ReminderOffset: ::windows::core::PCWSTR = ::windows::core::w!("ReminderOffset");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_Tentative: ::windows::core::PCWSTR = ::windows::core::w!("Tentative");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarObj_TimeZone: ::windows::core::PCWSTR = ::windows::core::w!("TimeZone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarSvc: ::windows::core::PCWSTR = ::windows::core::w!("Calendar");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarSvc_SyncWindowEnd: ::windows::core::PCWSTR = ::windows::core::w!("SyncWindowEnd");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_CalendarSvc_SyncWindowStart: ::windows::core::PCWSTR = ::windows::core::w!("SyncWindowStart");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_AnniversaryDate: ::windows::core::PCWSTR = ::windows::core::w!("AnniversaryDate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Assistant: ::windows::core::PCWSTR = ::windows::core::w!("Assistant");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Birthdate: ::windows::core::PCWSTR = ::windows::core::w!("Birthdate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressCity: ::windows::core::PCWSTR = ::windows::core::w!("BusinessAddressCity");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressCountry: ::windows::core::PCWSTR = ::windows::core::w!("BusinessAddressCountry");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressFull: ::windows::core::PCWSTR = ::windows::core::w!("BusinessAddressFull");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressLine2: ::windows::core::PCWSTR = ::windows::core::w!("BusinessAddressLine2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressPostalCode: ::windows::core::PCWSTR = ::windows::core::w!("BusinessAddressPostalCode");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressRegion: ::windows::core::PCWSTR = ::windows::core::w!("BusinessAddressRegion");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessAddressStreet: ::windows::core::PCWSTR = ::windows::core::w!("BusinessAddressStreet");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessEmail: ::windows::core::PCWSTR = ::windows::core::w!("BusinessEmail");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessEmail2: ::windows::core::PCWSTR = ::windows::core::w!("BusinessEmail2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessFax: ::windows::core::PCWSTR = ::windows::core::w!("BusinessFax");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessPhone: ::windows::core::PCWSTR = ::windows::core::w!("BusinessPhone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessPhone2: ::windows::core::PCWSTR = ::windows::core::w!("BusinessPhone2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_BusinessWebAddress: ::windows::core::PCWSTR = ::windows::core::w!("BusinessWebAddress");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Children: ::windows::core::PCWSTR = ::windows::core::w!("Children");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Email: ::windows::core::PCWSTR = ::windows::core::w!("Email");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_FamilyName: ::windows::core::PCWSTR = ::windows::core::w!("FamilyName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Fax: ::windows::core::PCWSTR = ::windows::core::w!("Fax");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_GivenName: ::windows::core::PCWSTR = ::windows::core::w!("GivenName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_IMAddress: ::windows::core::PCWSTR = ::windows::core::w!("IMAddress");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_IMAddress2: ::windows::core::PCWSTR = ::windows::core::w!("IMAddress2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_IMAddress3: ::windows::core::PCWSTR = ::windows::core::w!("IMAddress3");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_MiddleNames: ::windows::core::PCWSTR = ::windows::core::w!("MiddleNames");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_MobilePhone: ::windows::core::PCWSTR = ::windows::core::w!("MobilePhone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_MobilePhone2: ::windows::core::PCWSTR = ::windows::core::w!("MobilePhone2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Organization: ::windows::core::PCWSTR = ::windows::core::w!("Organization");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressCity: ::windows::core::PCWSTR = ::windows::core::w!("OtherAddressCity");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressCountry: ::windows::core::PCWSTR = ::windows::core::w!("OtherAddressCountry");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressFull: ::windows::core::PCWSTR = ::windows::core::w!("OtherAddressFull");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressLine2: ::windows::core::PCWSTR = ::windows::core::w!("OtherAddressLine2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressPostalCode: ::windows::core::PCWSTR = ::windows::core::w!("OtherAddressPostalCode");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressRegion: ::windows::core::PCWSTR = ::windows::core::w!("OtherAddressRegion");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherAddressStreet: ::windows::core::PCWSTR = ::windows::core::w!("OtherAddressStreet");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherEmail: ::windows::core::PCWSTR = ::windows::core::w!("OtherEmail");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_OtherPhone: ::windows::core::PCWSTR = ::windows::core::w!("OtherPhone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Pager: ::windows::core::PCWSTR = ::windows::core::w!("Pager");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressCity: ::windows::core::PCWSTR = ::windows::core::w!("PersonalAddressCity");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressCountry: ::windows::core::PCWSTR = ::windows::core::w!("PersonalAddressCountry");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressFull: ::windows::core::PCWSTR = ::windows::core::w!("PersonalAddressFull");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressLine2: ::windows::core::PCWSTR = ::windows::core::w!("PersonalAddressLine2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressPostalCode: ::windows::core::PCWSTR = ::windows::core::w!("PersonalAddressPostalCode");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressRegion: ::windows::core::PCWSTR = ::windows::core::w!("PersonalAddressRegion");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalAddressStreet: ::windows::core::PCWSTR = ::windows::core::w!("PersonalAddressStreet");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalEmail: ::windows::core::PCWSTR = ::windows::core::w!("PersonalEmail");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalEmail2: ::windows::core::PCWSTR = ::windows::core::w!("PersonalEmail2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalFax: ::windows::core::PCWSTR = ::windows::core::w!("PersonalFax");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalPhone: ::windows::core::PCWSTR = ::windows::core::w!("PersonalPhone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalPhone2: ::windows::core::PCWSTR = ::windows::core::w!("PersonalPhone2");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PersonalWebAddress: ::windows::core::PCWSTR = ::windows::core::w!("PersonalWebAddress");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Phone: ::windows::core::PCWSTR = ::windows::core::w!("Phone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PhoneticFamilyName: ::windows::core::PCWSTR = ::windows::core::w!("PhoneticFamilyName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PhoneticGivenName: ::windows::core::PCWSTR = ::windows::core::w!("PhoneticGivenName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_PhoneticOrganization: ::windows::core::PCWSTR = ::windows::core::w!("PhoneticOrganization");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Ringtone: ::windows::core::PCWSTR = ::windows::core::w!("Ringtone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Role: ::windows::core::PCWSTR = ::windows::core::w!("Role");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Spouse: ::windows::core::PCWSTR = ::windows::core::w!("Spouse");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Suffix: ::windows::core::PCWSTR = ::windows::core::w!("Suffix");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_Title: ::windows::core::PCWSTR = ::windows::core::w!("Title");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactObj_WebAddress: ::windows::core::PCWSTR = ::windows::core::w!("WebAddress");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactSvc_SyncWithPhoneOnly: ::windows::core::PCWSTR = ::windows::core::w!("FilterType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ContactsSvc: ::windows::core::PCWSTR = ::windows::core::w!("Contacts");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DPOFDocument: ::windows::core::PCWSTR = ::windows::core::w!("DPOFDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DVBTSFile: ::windows::core::PCWSTR = ::windows::core::w!("DVBTSFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceExecutable: ::windows::core::PCWSTR = ::windows::core::w!("DeviceExecutable");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataCAB: ::windows::core::PCWSTR = ::windows::core::w!("DeviceMetadataCAB");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataObj_ContentID: ::windows::core::PCWSTR = ::windows::core::w!("ContentID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataObj_DefaultCAB: ::windows::core::PCWSTR = ::windows::core::w!("DefaultCAB");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceMetadataSvc: ::windows::core::PCWSTR = ::windows::core::w!("Metadata");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_DeviceScript: ::windows::core::PCWSTR = ::windows::core::w!("DeviceScript");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_EXIFImage: ::windows::core::PCWSTR = ::windows::core::w!("EXIFImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ExcelDocument: ::windows::core::PCWSTR = ::windows::core::w!("ExcelDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FLACFile: ::windows::core::PCWSTR = ::windows::core::w!("FLACFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FirmwareFile: ::windows::core::PCWSTR = ::windows::core::w!("FirmwareFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FlashPixImage: ::windows::core::PCWSTR = ::windows::core::w!("FlashPixImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncKnowledge: ::windows::core::PCWSTR = ::windows::core::w!("FullEnumSyncKnowledge");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc: ::windows::core::PCWSTR = ::windows::core::w!("FullEnumSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_BeginSync: ::windows::core::PCWSTR = ::windows::core::w!("BeginSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_EndSync: ::windows::core::PCWSTR = ::windows::core::w!("EndSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_FilterType: ::windows::core::PCWSTR = ::windows::core::w!("FilterType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_KnowledgeObjectID: ::windows::core::PCWSTR = ::windows::core::w!("FullEnumKnowledgeObjectID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_LastSyncProxyID: ::windows::core::PCWSTR = ::windows::core::w!("FullEnumLastSyncProxyID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_LocalOnlyDelete: ::windows::core::PCWSTR = ::windows::core::w!("LocalOnlyDelete");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_ProviderVersion: ::windows::core::PCWSTR = ::windows::core::w!("FullEnumProviderVersion");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_ReplicaID: ::windows::core::PCWSTR = ::windows::core::w!("FullEnumReplicaID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_SyncFormat: ::windows::core::PCWSTR = ::windows::core::w!("SyncFormat");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_FullEnumSyncSvc_VersionProps: ::windows::core::PCWSTR = ::windows::core::w!("FullEnumVersionProps");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GIFImage: ::windows::core::PCWSTR = ::windows::core::w!("GIFImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_AllowedFolderContents: ::windows::core::PCWSTR = ::windows::core::w!("AllowedFolderContents");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_AssociationDesc: ::windows::core::PCWSTR = ::windows::core::w!("AssociationDesc");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_AssociationType: ::windows::core::PCWSTR = ::windows::core::w!("AssociationType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Copyright: ::windows::core::PCWSTR = ::windows::core::w!("Copyright");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Corrupt: ::windows::core::PCWSTR = ::windows::core::w!("Corrupt");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DRMStatus: ::windows::core::PCWSTR = ::windows::core::w!("DRMStatus");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateAccessed: ::windows::core::PCWSTR = ::windows::core::w!("DateAccessed");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateAdded: ::windows::core::PCWSTR = ::windows::core::w!("DateAdded");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateAuthored: ::windows::core::PCWSTR = ::windows::core::w!("DateAuthored");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateCreated: ::windows::core::PCWSTR = ::windows::core::w!("DateCreated");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateModified: ::windows::core::PCWSTR = ::windows::core::w!("DateModified");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_DateRevised: ::windows::core::PCWSTR = ::windows::core::w!("DateRevised");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Description: ::windows::core::PCWSTR = ::windows::core::w!("Description");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Hidden: ::windows::core::PCWSTR = ::windows::core::w!("Hidden");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Keywords: ::windows::core::PCWSTR = ::windows::core::w!("Keywords");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_LanguageLocale: ::windows::core::PCWSTR = ::windows::core::w!("LanguageLocale");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_Name: ::windows::core::PCWSTR = ::windows::core::w!("Name");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_NonConsumable: ::windows::core::PCWSTR = ::windows::core::w!("NonConsumable");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectFileName: ::windows::core::PCWSTR = ::windows::core::w!("ObjectFileName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectFormat: ::windows::core::PCWSTR = ::windows::core::w!("ObjectFormat");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectID: ::windows::core::PCWSTR = ::windows::core::w!("ObjectID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ObjectSize: ::windows::core::PCWSTR = ::windows::core::w!("ObjectSize");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ParentID: ::windows::core::PCWSTR = ::windows::core::w!("ParentID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_PersistentUID: ::windows::core::PCWSTR = ::windows::core::w!("PersistentUID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_PropertyBag: ::windows::core::PCWSTR = ::windows::core::w!("PropertyBag");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ProtectionStatus: ::windows::core::PCWSTR = ::windows::core::w!("ProtectionStatus");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_ReferenceParentID: ::windows::core::PCWSTR = ::windows::core::w!("ReferenceParentID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_StorageID: ::windows::core::PCWSTR = ::windows::core::w!("StorageID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_SubDescription: ::windows::core::PCWSTR = ::windows::core::w!("SubDescription");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_SyncID: ::windows::core::PCWSTR = ::windows::core::w!("SyncID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_SystemObject: ::windows::core::PCWSTR = ::windows::core::w!("SystemObject");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_GenericObj_TimeToLive: ::windows::core::PCWSTR = ::windows::core::w!("TimeToLive");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_HDPhotoImage: ::windows::core::PCWSTR = ::windows::core::w!("HDPhotoImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_HTMLDocument: ::windows::core::PCWSTR = ::windows::core::w!("HTMLDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_HintsSvc: ::windows::core::PCWSTR = ::windows::core::w!("Hints");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ICalendarActivity: ::windows::core::PCWSTR = ::windows::core::w!("ICalendar");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_Aperature: ::windows::core::PCWSTR = ::windows::core::w!("Aperature");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_Exposure: ::windows::core::PCWSTR = ::windows::core::w!("Exposure");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_ISOSpeed: ::windows::core::PCWSTR = ::windows::core::w!("ISOSpeed");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_ImageBitDepth: ::windows::core::PCWSTR = ::windows::core::w!("ImageBitDepth");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_IsColorCorrected: ::windows::core::PCWSTR = ::windows::core::w!("IsColorCorrected");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_ImageObj_IsCropped: ::windows::core::PCWSTR = ::windows::core::w!("IsCropped");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JFIFImage: ::windows::core::PCWSTR = ::windows::core::w!("JFIFImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JP2Image: ::windows::core::PCWSTR = ::windows::core::w!("JP2Image");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JPEGXRImage: ::windows::core::PCWSTR = ::windows::core::w!("JPEGXRImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_JPXImage: ::windows::core::PCWSTR = ::windows::core::w!("JPXImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_M3UPlaylist: ::windows::core::PCWSTR = ::windows::core::w!("M3UPlaylist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MHTDocument: ::windows::core::PCWSTR = ::windows::core::w!("MHTDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MP3File: ::windows::core::PCWSTR = ::windows::core::w!("MP3File");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPEG2File: ::windows::core::PCWSTR = ::windows::core::w!("MPEG2File");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPEG4File: ::windows::core::PCWSTR = ::windows::core::w!("MPEG4File");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPEGFile: ::windows::core::PCWSTR = ::windows::core::w!("MPEGFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MPLPlaylist: ::windows::core::PCWSTR = ::windows::core::w!("MPLPlaylist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_AlbumArtist: ::windows::core::PCWSTR = ::windows::core::w!("AlbumArtist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_AlbumName: ::windows::core::PCWSTR = ::windows::core::w!("AlbumName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Artist: ::windows::core::PCWSTR = ::windows::core::w!("Artist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_AudioEncodingProfile: ::windows::core::PCWSTR = ::windows::core::w!("AudioEncodingProfile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BitRateType: ::windows::core::PCWSTR = ::windows::core::w!("BitRateType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BookmarkByte: ::windows::core::PCWSTR = ::windows::core::w!("BookmarkByte");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BookmarkObject: ::windows::core::PCWSTR = ::windows::core::w!("BookmarkObject");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BookmarkTime: ::windows::core::PCWSTR = ::windows::core::w!("BookmarkTime");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_BufferSize: ::windows::core::PCWSTR = ::windows::core::w!("BufferSize");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Composer: ::windows::core::PCWSTR = ::windows::core::w!("Composer");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Credits: ::windows::core::PCWSTR = ::windows::core::w!("Credits");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_DateOriginalRelease: ::windows::core::PCWSTR = ::windows::core::w!("DateOriginalRelease");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Duration: ::windows::core::PCWSTR = ::windows::core::w!("Duration");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Editor: ::windows::core::PCWSTR = ::windows::core::w!("Editor");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_EffectiveRating: ::windows::core::PCWSTR = ::windows::core::w!("EffectiveRating");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_EncodingProfile: ::windows::core::PCWSTR = ::windows::core::w!("EncodingProfile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_EncodingQuality: ::windows::core::PCWSTR = ::windows::core::w!("EncodingQuality");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Genre: ::windows::core::PCWSTR = ::windows::core::w!("Genre");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_GeographicOrigin: ::windows::core::PCWSTR = ::windows::core::w!("GeographicOrigin");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Height: ::windows::core::PCWSTR = ::windows::core::w!("Height");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_MediaType: ::windows::core::PCWSTR = ::windows::core::w!("MediaType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_MediaUID: ::windows::core::PCWSTR = ::windows::core::w!("MediaUID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Mood: ::windows::core::PCWSTR = ::windows::core::w!("Mood");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Owner: ::windows::core::PCWSTR = ::windows::core::w!("Owner");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_ParentalRating: ::windows::core::PCWSTR = ::windows::core::w!("ParentalRating");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Producer: ::windows::core::PCWSTR = ::windows::core::w!("Producer");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_SampleRate: ::windows::core::PCWSTR = ::windows::core::w!("SampleRate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_SkipCount: ::windows::core::PCWSTR = ::windows::core::w!("SkipCount");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_SubscriptionContentID: ::windows::core::PCWSTR = ::windows::core::w!("SubscriptionContentID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Subtitle: ::windows::core::PCWSTR = ::windows::core::w!("Subtitle");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_TotalBitRate: ::windows::core::PCWSTR = ::windows::core::w!("TotalBitRate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Track: ::windows::core::PCWSTR = ::windows::core::w!("Track");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_URLLink: ::windows::core::PCWSTR = ::windows::core::w!("URLLink");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_URLSource: ::windows::core::PCWSTR = ::windows::core::w!("URLSource");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_UseCount: ::windows::core::PCWSTR = ::windows::core::w!("UseCount");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_UserRating: ::windows::core::PCWSTR = ::windows::core::w!("UserRating");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_WebMaster: ::windows::core::PCWSTR = ::windows::core::w!("WebMaster");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MediaObj_Width: ::windows::core::PCWSTR = ::windows::core::w!("Width");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_BCC: ::windows::core::PCWSTR = ::windows::core::w!("BCC");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Body: ::windows::core::PCWSTR = ::windows::core::w!("Body");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_CC: ::windows::core::PCWSTR = ::windows::core::w!("CC");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Category: ::windows::core::PCWSTR = ::windows::core::w!("Category");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternDayOfMonth: ::windows::core::PCWSTR = ::windows::core::w!("PatternDayOfMonth");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternDayOfWeek: ::windows::core::PCWSTR = ::windows::core::w!("PatternDayOfWeek");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternDeleteDates: ::windows::core::PCWSTR = ::windows::core::w!("PatternDeleteDates");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternInstance: ::windows::core::PCWSTR = ::windows::core::w!("PatternInstance");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternMonthOfYear: ::windows::core::PCWSTR = ::windows::core::w!("PatternMonthOfYear");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternOriginalDateTime: ::windows::core::PCWSTR = ::windows::core::w!("PatternOriginalDateTime");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternPeriod: ::windows::core::PCWSTR = ::windows::core::w!("PatternPeriod");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternType: ::windows::core::PCWSTR = ::windows::core::w!("PatternType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternValidEndDate: ::windows::core::PCWSTR = ::windows::core::w!("PatternValidEndDate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_PatternValidStartDate: ::windows::core::PCWSTR = ::windows::core::w!("PatternValidStartDate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Priority: ::windows::core::PCWSTR = ::windows::core::w!("Priority");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Read: ::windows::core::PCWSTR = ::windows::core::w!("Read");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_ReceivedTime: ::windows::core::PCWSTR = ::windows::core::w!("ReceivedTime");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Sender: ::windows::core::PCWSTR = ::windows::core::w!("Sender");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_Subject: ::windows::core::PCWSTR = ::windows::core::w!("Subject");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageObj_To: ::windows::core::PCWSTR = ::windows::core::w!("To");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_MessageSvc: ::windows::core::PCWSTR = ::windows::core::w!("Message");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_NotesSvc: ::windows::core::PCWSTR = ::windows::core::w!("Notes");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_OGGFile: ::windows::core::PCWSTR = ::windows::core::w!("OGGFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PCDImage: ::windows::core::PCWSTR = ::windows::core::w!("PCDImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PICTImage: ::windows::core::PCWSTR = ::windows::core::w!("PICTImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PNGImage: ::windows::core::PCWSTR = ::windows::core::w!("PNGImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PSLPlaylist: ::windows::core::PCWSTR = ::windows::core::w!("PSLPlaylist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_PowerPointDocument: ::windows::core::PCWSTR = ::windows::core::w!("PowerPointDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_QCELPFile: ::windows::core::PCWSTR = ::windows::core::w!("QCELPFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_RingtonesSvc: ::windows::core::PCWSTR = ::windows::core::w!("Ringtones");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_RingtonesSvc_DefaultRingtone: ::windows::core::PCWSTR = ::windows::core::w!("DefaultRingtone");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Services_ServiceDisplayName: ::windows::core::PCWSTR = ::windows::core::w!("ServiceDisplayName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Services_ServiceIcon: ::windows::core::PCWSTR = ::windows::core::w!("ServiceIcon");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Services_ServiceLocale: ::windows::core::PCWSTR = ::windows::core::w!("ServiceLocale");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc: ::windows::core::PCWSTR = ::windows::core::w!("Status");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_BatteryLife: ::windows::core::PCWSTR = ::windows::core::w!("BatteryLife");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_ChargingState: ::windows::core::PCWSTR = ::windows::core::w!("ChargingState");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_MissedCalls: ::windows::core::PCWSTR = ::windows::core::w!("MissedCalls");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_NetworkName: ::windows::core::PCWSTR = ::windows::core::w!("NetworkName");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_NetworkType: ::windows::core::PCWSTR = ::windows::core::w!("NetworkType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_NewPictures: ::windows::core::PCWSTR = ::windows::core::w!("NewPictures");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_Roaming: ::windows::core::PCWSTR = ::windows::core::w!("Roaming");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_SignalStrength: ::windows::core::PCWSTR = ::windows::core::w!("SignalStrength");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_StorageCapacity: ::windows::core::PCWSTR = ::windows::core::w!("StorageCapacity");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_StorageFreeSpace: ::windows::core::PCWSTR = ::windows::core::w!("StorageFreeSpace");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_TextMessages: ::windows::core::PCWSTR = ::windows::core::w!("TextMessages");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_StatusSvc_VoiceMail: ::windows::core::PCWSTR = ::windows::core::w!("VoiceMail");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncObj_LastAuthorProxyID: ::windows::core::PCWSTR = ::windows::core::w!("LastAuthorProxyID");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_BeginSync: ::windows::core::PCWSTR = ::windows::core::w!("BeginSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_EndSync: ::windows::core::PCWSTR = ::windows::core::w!("EndSync");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_FilterType: ::windows::core::PCWSTR = ::windows::core::w!("FilterType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_LocalOnlyDelete: ::windows::core::PCWSTR = ::windows::core::w!("LocalOnlyDelete");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_SyncFormat: ::windows::core::PCWSTR = ::windows::core::w!("SyncFormat");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_SyncSvc_SyncObjectReferences: ::windows::core::PCWSTR = ::windows::core::w!("SyncObjectReferences");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TIFFEPImage: ::windows::core::PCWSTR = ::windows::core::w!("TIFFEPImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TIFFITImage: ::windows::core::PCWSTR = ::windows::core::w!("TIFFITImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TIFFImage: ::windows::core::PCWSTR = ::windows::core::w!("TIFFImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_BeginDate: ::windows::core::PCWSTR = ::windows::core::w!("BeginDate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_Complete: ::windows::core::PCWSTR = ::windows::core::w!("Complete");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_EndDate: ::windows::core::PCWSTR = ::windows::core::w!("EndDate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TaskObj_ReminderDateTime: ::windows::core::PCWSTR = ::windows::core::w!("ReminderDateTime");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TasksSvc: ::windows::core::PCWSTR = ::windows::core::w!("Tasks");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TasksSvc_SyncActiveOnly: ::windows::core::PCWSTR = ::windows::core::w!("FilterType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_TextDocument: ::windows::core::PCWSTR = ::windows::core::w!("TextDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_Undefined: ::windows::core::PCWSTR = ::windows::core::w!("Undefined");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedAudio: ::windows::core::PCWSTR = ::windows::core::w!("UndefinedAudio");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedCollection: ::windows::core::PCWSTR = ::windows::core::w!("UndefinedCollection");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedDocument: ::windows::core::PCWSTR = ::windows::core::w!("UndefinedDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UndefinedVideo: ::windows::core::PCWSTR = ::windows::core::w!("UndefinedVideo");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_UnknownImage: ::windows::core::PCWSTR = ::windows::core::w!("UnknownImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VCalendar1Activity: ::windows::core::PCWSTR = ::windows::core::w!("VCalendar1");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VCard2Contact: ::windows::core::PCWSTR = ::windows::core::w!("VCard2Contact");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VCard3Contact: ::windows::core::PCWSTR = ::windows::core::w!("VCard3Contact");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_KeyFrameDistance: ::windows::core::PCWSTR = ::windows::core::w!("KeyFrameDistance");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_ScanType: ::windows::core::PCWSTR = ::windows::core::w!("ScanType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_Source: ::windows::core::PCWSTR = ::windows::core::w!("Source");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_VideoBitRate: ::windows::core::PCWSTR = ::windows::core::w!("VideoBitRate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_VideoFormatCode: ::windows::core::PCWSTR = ::windows::core::w!("VideoFormatCode");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_VideoObj_VideoFrameRate: ::windows::core::PCWSTR = ::windows::core::w!("VideoFrameRate");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WAVFile: ::windows::core::PCWSTR = ::windows::core::w!("WAVFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WBMPImage: ::windows::core::PCWSTR = ::windows::core::w!("WBMPImage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WMAFile: ::windows::core::PCWSTR = ::windows::core::w!("WMAFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WMVFile: ::windows::core::PCWSTR = ::windows::core::w!("WMVFile");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WPLPlaylist: ::windows::core::PCWSTR = ::windows::core::w!("WPLPlaylist");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_WordDocument: ::windows::core::PCWSTR = ::windows::core::w!("WordDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const NAME_XMLDocument: ::windows::core::PCWSTR = ::windows::core::w!("XMLDocument");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DRM_SCHEME_PDDRM: ::windows::core::PCWSTR = ::windows::core::w!("PDDRM");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DRM_SCHEME_WMDRM10_PD: ::windows::core::PCWSTR = ::windows::core::w!("WMDRM10-PD");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_ICON: ::windows::core::PCWSTR = ::windows::core::w!("Icons");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_IS_MASS_STORAGE: ::windows::core::PCWSTR = ::windows::core::w!("PortableDeviceIsMassStorage");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_NAMESPACE_EXCLUDE_FROM_SHELL: ::windows::core::PCWSTR = ::windows::core::w!("PortableDeviceNameSpaceExcludeFromShell");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_NAMESPACE_THUMBNAIL_CONTENT_TYPES: ::windows::core::PCWSTR = ::windows::core::w!("PortableDeviceNameSpaceThumbnailContentTypes");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_NAMESPACE_TIMEOUT: ::windows::core::PCWSTR = ::windows::core::w!("PortableDeviceNameSpaceTimeout");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_TYPE: ::windows::core::PCWSTR = ::windows::core::w!("PortableDeviceType");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x728a21c5_3d9e_48d7_9810_864848f0f404);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceDispatchFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43232233_8338_4658_ae01_0b4ae830b6b0);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceFTM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7c0039a_4762_488a_b4b3_760ef9a1ba9b);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceKeyCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde2d022d_2480_43be_97f0_d1fa2cf98f4f);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0af10cec_2ecd_4b92_9581_34f6ae0637f3);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDevicePropVariantCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08a99e2f_6d6d_4b80_af5a_baf2bcbe4cb9);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5db4c2_9312_422c_9152_411cd9c4dd84);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceServiceFTM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1649b154_c794_497a_9b03_f3f0121302f3);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceValues: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c15d503_d017_47ce_9016_7b3f978721cc);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceValuesCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3882134d_14cf_4220_9cb4_435f86d83f60);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PortableDeviceWebControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x186dd02c_2dec_41b5_a7d4_b59056fade51);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_MessageObj_PatternDayOfMonth: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_MessageObj_PatternMonthOfYear: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_BatteryLife: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_MissedCalls: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_NewPictures: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_SignalStrength: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_TextMessages: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMAX_StatusSvc_VoiceMail: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_StatusSvc_BatteryLife: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGEMIN_StatusSvc_SignalStrength: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_StatusSvc_BatteryLife: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const RANGESTEP_StatusSvc_SignalStrength: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const STR_WPDNSE_FAST_ENUM: ::windows::core::PCWSTR = ::windows::core::w!("WPDNSE Fast Enum");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const STR_WPDNSE_SIMPLE_ITEM: ::windows::core::PCWSTR = ::windows::core::w!("WPDNSE SimpleItem");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_CONTACTS_WITH_PHONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SYNCSVC_FILTER_TASK_ACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_AnchorSyncSvc: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_CalendarSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_ContactsSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_DeviceMetadataSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_FullEnumSyncSvc: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_HintsSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_MessageSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_NotesSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_RingtonesSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_StatusSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const TYPE_TasksSvc: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_DETAILS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_GENERAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_REFERENCES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_CONTENT_RESOURCES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_DEVICE_GENERAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPDNSE_PROPSHEET_STORAGE_GENERAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_API_OPTIONS_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_IOCTL_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_USE_CLEAR_DATA_STREAM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_ACCEPTED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_DECLINED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_APPOINTMENT_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_OPTIONAL_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_REQUIRED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TENTATIVE_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BIT_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BLOCK_ALIGNMENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_CHANNEL_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_FORMAT_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_CAPABILITIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_COMMON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_DEVICE_HINTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_MEDIA_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_NETWORK_CONFIGURATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_NULL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_OBJECT_ENUMERATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_OBJECT_MANAGEMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_OBJECT_PROPERTIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_OBJECT_PROPERTIES_BULK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_OBJECT_RESOURCES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_SERVICE_CAPABILITIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_SERVICE_COMMON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_SERVICE_METHODS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_SMS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_STILL_IMAGE_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CATEGORY_STORAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DEVICE_IDENTIFICATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DONT_REGISTER_WPD_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_MULTITRANSPORT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_REGISTER_WPD_PRIVATE_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SILENCE_AUTOPLAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_TRANSPORT_BANDWIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_V3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CLASS_EXTENSION_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CLASS_EXTENSION_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_DESIRED_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_EVENT_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CLIENT_INFORMATION_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MAJOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MANUAL_CLOSE_ON_DISCONNECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINIMUM_RESULTS_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_REVISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SECURITY_QUALITY_OF_SERVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SHARE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_PRIVATE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FIXED_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_REGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_UNREGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_WRITE_DEVICE_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMIT_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_GET_OBJECT_IDS_FROM_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_RESET_DEVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_SAVE_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_DEVICE_HINTS_GET_CONTENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_GENERATE_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_PAUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_STOP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 17 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 18 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_READ_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_WRITE_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 16 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_END_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_FIND_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_START_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COMMIT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COPY_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_ONLY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_DELETE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_MOVE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_REVERT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_UPDATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_WRITE_OBJECT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ALL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_SET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CLOSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_COMMIT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CREATE_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_OPEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_REVERT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK_IN_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_PROCESS_WIRELESS_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 16 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS_BY_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_COMMON_GET_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_CANCEL_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_END_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_START_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SMS_SEND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STILL_IMAGE_CAPTURE_INITIATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_EJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_BODY_TEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_END_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_NOTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMON_INFORMATION_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_PRIORITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_START_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_SUBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ANNIVERSARY_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 62 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ASSISTANT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 61 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BIRTHDATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 57 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 34 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 35 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 45 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 17 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 40 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 41 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 20 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 23 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 18 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 19 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 22 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 21 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 50 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_CHILDREN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 60 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 54 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 51 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 52 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 53 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MIDDLE_NAMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 42 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 43 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTACT_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_EMAILS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 36 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 24 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_PHONES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 47 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 27 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 25 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 26 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 29 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 30 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 28 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PAGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 46 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 33 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 44 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 38 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 39 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 16 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 49 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 55 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PREFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_EMAIL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 31 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 58 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 37 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 48 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_RINGTONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 63 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ROLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 56 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SPOUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 59 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SUFFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e170d2_1055_4a3e_b952_82cc4f8a8689);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_APPOINTMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fed060e_8793_4b1e_90c9_48ac389ac631);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ad2c85e_5e2d_45e5_8864_4f229e3c6cf0);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_AUDIO_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa18737e_5009_48fa_ae21_85f24383b4e6);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_CALENDAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1fd5967_6023_49a0_9df1_f8060be751b0);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_CERTIFICATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc3876e8_a948_4060_9050_cbd77e8a3d87);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_CONTACT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeaba8313_4525_4707_9f0e_87c6808e9435);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_CONTACT_GROUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x346b8932_4c36_40d8_9415_1828291f9de9);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_DOCUMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x680adf52_950a_4041_9b41_65e393648155);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_EMAIL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8038044a_7e51_4f8f_883d_1d0623d14533);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_FOLDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27e2e392_a111_48e0_ab0c_e17705a05f85);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99ed0160_17ff_4c44_9d98_1d7a6f941921);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_GENERIC_FILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0085e0a6_8d34_45d7_bc5c_447e59c73d48);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_GENERIC_MESSAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe80eaaf8_b2db_4133_b67e_1bef4b4a6e5f);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef2107d5_a52a_4243_a26b_62d4176d7603);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_IMAGE_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75793148_15f5_4a30_a813_54ed8a37e226);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_MEDIA_CAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e88b3cc_3e65_4e62_bfff_229495253ab0);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_MEMO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cd20ecf_3b50_414f_a641_e473ffe45751);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_MIXED_CONTENT_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f0c3ac_a593_49ac_9219_24abca5a2563);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_NETWORK_ASSOCIATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x031da7ee_18c8_4205_847e_89a11261d0f3);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_PLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a33f7e4_af13_48f5_994e_77369dfe04a3);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_PROGRAM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd269f96a_247c_4bff_98fb_97f3c49220e6);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_SECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x821089f5_1d91_4dc9_be3c_bbb1b35b18ce);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_TASK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63252f2c_887f_4cb6_b1ac_d29855dcef6c);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_TELEVISION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60a169cf_f2ae_4e21_9375_9677f11c1c6e);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_UNSPECIFIED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28d8d31e_249c_454e_aabc_34883168e634);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9261b03c_3d78_4519_85e3_02c5e1f50bb9);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_VIDEO_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x012b0db7_d4c1_45d6_b081_94b87779614f);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTENT_TYPE_WIRELESS_PROFILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bac070a_9f5f_4da4_a8f6_3de44d68fd6c);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CONTROL_FUNCTION_GENERIC_MESSAGE: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_EDP_IDENTITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed), pid: 1 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FIRMWARE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FUNCTIONAL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_NETWORK_IDENTIFIER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 16 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_OBJECT_ID: ::windows::core::PCWSTR = ::windows::core::w!("DEVICE");
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_LEVEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_SOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_PROPERTIES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_PROPERTIES_V3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_PROTOCOL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_DRM_SCHEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTS_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SYNC_PARTNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TRANSPORT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_USE_DEVICE_STAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DOCUMENT_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b110203_eb95_4f02_93e0_97c631493ad5);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_BCC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_CC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_ATTACHMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_BEEN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EMAIL_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_RECEIVED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_SENDER_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_TO_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_DEVICE_CAPABILITIES_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36885aa1_cd54_4daa_b3d0_afb3e03f5999);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_DEVICE_REMOVED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4cbca1b_6918_48b9_85ee_02be7c850af9);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_DEVICE_RESET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7755cf53_c1ed_44f3_b5a2_451e2c376b27);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_5738_4ff2_8445_be3126691059);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_NOTIFICATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ba2e40a_6b4c_4295_bb43_26322b99aeb2);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_OBJECT_ADDED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa726da95_e207_4b02_8d44_bef2e86cbffc);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_OBJECT_REMOVED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe82ab88_a52c_4823_96e5_d0272671fc38);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_OBJECT_TRANSFER_REQUESTED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d16a0a1_f2c6_41da_8f19_5e53721adbf2);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_OBJECT_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1445a759_2e01_485d_9f27_ff07dae697ab);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_OPTIONS_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_AUTOPLAY_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_BROADCAST_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_CHILD_HIERARCHY_CHANGED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_EVENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_CREATION_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_PARENT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_PROGRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_PNP_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_PROPERTIES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_SERVICE_METHOD_COMPLETE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a33f5f8_0acc_4d9b_9cc4_112d353b86ca);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EVENT_STORAGE_FORMAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3782616b_22bc_4474_a251_3070f8d38857);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FOLDER_CONTENT_TYPES_ALLOWED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOLDER_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FORMAT_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_MIMETYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d8a6512_a74c_448e_ba8a_f4ac07c49399);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_AUDIO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f2a1919_c7c2_4a00_855d_f57cf06debbb);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08ea466b_e3a4_4336_a1f3_a44d2b5c438c);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_NETWORK_CONFIGURATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48f4db72_7c6a_4ab0_9e1a_470e3cdbf26a);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_RENDERING_INFORMATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08600ba4_a7ba_4a01_ab0e_0065d0a356d3);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_SMS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0044a0b1_c1e9_4afd_b358_a62c6117c9cf);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_STILL_IMAGE_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x613ca327_ab93_4900_b4fa_895bb5874b79);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_STORAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23f05bbc_15de_4c2a_a55b_a9af5ce412ef);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_CATEGORY_VIDEO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe23e5f6b_7243_43aa_8df1_0eb3d968a918);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FUNCTIONAL_OBJECT_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_BITDEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_COLOR_CORRECTED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_CROPPED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_HORIZONTAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_IMAGE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_VERTICAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ALBUM_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 25 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 24 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_AUDIO_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 49 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BITRATE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BUY_NOW: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 20 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BYTE_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 36 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COMPOSER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COPYRIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 31 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESTINATION_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 30 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DURATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 19 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 21 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 32 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 38 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 23 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_ACCESSED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_BUILD_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 35 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_MANAGING_EDITOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 27 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_META_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OBJECT_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 34 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 26 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_PARENTAL_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_MEDIA_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_RELEASE_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SAMPLE_RATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SKIP_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SOURCE_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 29 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_STAR_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 16 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUBSCRIPTION_CONTENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 39 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 33 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_TO_LIVE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 37 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 18 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TOTAL_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USER_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 17 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WEBMASTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 28 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 22 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_MEMO_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ffbfc7b_7483_41ad_afb9_da3f4e592b8d);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_METHOD_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ASSOCIATED_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_ALBUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_LYRICS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_MOOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_MUSIC_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_TRACK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_HOST_NETWORK_IDENTIFIERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_NETWORK_ASSOCIATION_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_X509V3SEQUENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_BACK_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 21 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 26 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTAINER_FUNCTIONAL_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 23 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_AUTHORED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 20 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_CREATED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 18 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_MODIFIED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 19 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_3G2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9850000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_3G2A: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a11202d_8759_4e34_ba5e_b1211087eee4);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_3GP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9840000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_3GPA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5172730_f971_41ef_a10b_2271a0019d7a);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_AAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9030000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb810000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT_GROUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba060000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ABSTRACT_MEDIA_CAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba0b0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_AIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30070000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1f62eb2_4bb3_479c_9cfa_05b5f3a57b22);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_AMR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9080000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ASF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x300c0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ASXPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba130000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ATSCTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9870000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_AUDIBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9040000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_AVCHD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9860000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_AVI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x300a0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_BMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38040000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_CIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38050000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_DPOF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30060000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_DVBTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9880000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_EXECUTABLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30030000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_EXIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38010000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_FLAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9060000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_FLASHPIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38030000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_GIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38070000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_HTML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30050000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ICALENDAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe030000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_ICON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077232ed_102c_4638_9c22_83f142bfc822);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_JFIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38080000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_JP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380f0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_JPEGXR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8040000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_JPX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38100000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_M3UPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba110000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_M4A: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30aba7ac_6ffd_4c23_a359_3e9b52f3f1c8);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MHT_COMPILED_HTML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba840000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MICROSOFT_EXCEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba850000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MICROSOFT_POWERPOINT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba860000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MICROSOFT_WFC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1040000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MICROSOFT_WORD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba830000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MKV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9900000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9830000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MP3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30090000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MP4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9820000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MPEG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x300b0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_MPLPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba120000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_NETWORK_ASSOCIATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1020000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_OGG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9020000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_PCD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38090000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_PICT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380a0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_PLSPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba140000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_PNG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380b0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_PROPERTIES_ONLY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30010000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_QCELP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9070000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_SCRIPT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30020000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_TEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30040000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_TIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380d0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_TIFFEP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38020000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_TIFFIT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380e0000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_UNSPECIFIED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30000000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_VCALENDAR1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe020000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_VCARD2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb820000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_VCARD3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb830000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_WAVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30080000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_WBMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8030000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_WINDOWSIMAGEFORMAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8810000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_WMA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9010000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_WMV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9810000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_WPLPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba100000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_X509V3CERTIFICATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1030000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_FORMAT_XML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba820000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_GENERATE_THUMBNAIL_FROM_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 24 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_HINT_LOCATION_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 25 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISHIDDEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISSYSTEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_IS_DRM_PROTECTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 17 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_KEYWORDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_LANGUAGE_LOCALE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 27 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ORIGINAL_FILE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OBJECT_PROPERTIES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SUPPORTED_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SYNC_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 16 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_MANAGEMENT_RECURSIVE_DELETE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_NO_INPUT_BUFFER_ON_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_READ_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_WRITE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_SMS_BINARY_MESSAGE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 5001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_VALID_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 5001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ORDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d545058_8900_40b3_8f1d_dc246e1e8370);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d545058_4fce_4578_95c8_8698a9bc0f49);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FAST_PROPERTY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1008 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1007 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1014 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1015 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1010 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1009 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1012 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1011 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1013 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_REGISTRATION_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_ACTIVITY_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1011 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1009 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1010 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_TARGET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1008 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1007 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_LOCATIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_ef88_4e4d_95c3_4f327f728a96), pid: 1011 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1013 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1012 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1009 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1008 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1010 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1011 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1007 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1014 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_NULL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_FILTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_NUM_OBJECTS_REQUESTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_COPY_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1013 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CREATION_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1007 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1010 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DESTINATION_FOLDER_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1011 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_MOVE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1012 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1016 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1009 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1008 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1015 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_UPDATE_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1014 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1007 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PARENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1008 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_ACCESS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1010 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1007 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1008 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1009 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1011 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_POSITION_FROM_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1014 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1012 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_ORIGIN_FLAG: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1013 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_STREAM_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1016 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SUPPORTS_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1015 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_PUBLIC_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1018 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1019 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1012 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1013 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1007 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1008 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITANCE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1014 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1015 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1006 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1010 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1009 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1016 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1017 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1011 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1005 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_PARAMETER_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_RESULT_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_BINARY_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1004 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_MESSAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_RECIPIENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_TEXT_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1003 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_DESTINATION_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1002 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1001 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_RENDERING_INFORMATION_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_CREATABLE_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf02aa354_2300_4e2d_a1b9_3b6730f7fa21), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_RESOURCE_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_READ_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_WRITE_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_RESOURCE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_TOTAL_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3bc13982_85b1_48e0_95a6_8d3ad06be117), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_BRANDING_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb633b1ae_6caf_4a87_9589_22ded6dd5899), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2c4d6803_80ea_4580_af9a_5be1a23eddcb), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_DEFAULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe81e79be_34f0_41bf_b53f_f1a06ae87842), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_GENERIC: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb9b9f515_ba70_4647_94dc_fa4925e95a07), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf195fed8_aa28_4ee3_b153_e182dd5edc39), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc7c407ba_98fa_46b5_9960_23fec124cfde), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_VIDEO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb566ee42_6368_4290_8662_70182fb79f20), pid: 0 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_REFERENCED_OBJECT_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SECTION_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SERVICE_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SERVICE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_ENCODING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_MAX_PAYLOAD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SMS_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_PROVIDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_TIMEOUT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 29 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 24 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 23 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 31 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 30 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_DELAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 17 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 18 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STILL_IMAGE_CAPTURE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_COMPRESSION_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CONTRAST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 19 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_DIGITAL_ZOOM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 21 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EFFECT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 22 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_BIAS_COMPENSATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 16 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_PROGRAM_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FLASH_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCAL_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 27 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_RGB_GAIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_SHARPNESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 20 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 26 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 25 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_UPLOAD_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 28 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_WHITE_BALANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_ACCESS_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FILE_SYSTEM_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 3 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_BYTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_MAX_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_TASK_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_PERCENT_COMPLETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_REMINDER_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 6 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_AUTHOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 2 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 13 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 8 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_CREDITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 9 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FOURCC_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 14 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FRAMERATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 15 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_KEY_FRAME_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 10 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_QUALITY_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 11 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_CHANNEL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 5 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_REPEAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 7 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_STATION_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 4 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_SCAN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 12 };
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WpdSerializer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b91a74b_ad7c_4a9d_b563_29eef9167172);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DELETE_OBJECT_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DELETE_NO_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const PORTABLE_DEVICE_DELETE_WITH_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(1i32);
impl ::core::marker::Copy for DELETE_OBJECT_OPTIONS {}
impl ::core::clone::Clone for DELETE_OBJECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DELETE_OBJECT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DELETE_OBJECT_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DELETE_OBJECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DELETE_OBJECT_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVICE_RADIO_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_RADIO_ON: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_SW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_SW_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_HW_RADIO_ON_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_RADIO_INVALID: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(5i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_HW_RADIO_OFF_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const DRS_RADIO_MAX: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
impl ::core::marker::Copy for DEVICE_RADIO_STATE {}
impl ::core::clone::Clone for DEVICE_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEVICE_RADIO_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEVICE_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_RADIO_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SMS_MESSAGE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_TEXT_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_BINARY_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(1i32);
impl ::core::marker::Copy for SMS_MESSAGE_TYPES {}
impl ::core::clone::Clone for SMS_MESSAGE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SMS_MESSAGE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SMS_MESSAGE_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SMS_MESSAGE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMS_MESSAGE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_RADIO_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SRS_RADIO_ENABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SRS_RADIO_DISABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(1i32);
impl ::core::marker::Copy for SYSTEM_RADIO_STATE {}
impl ::core::clone::Clone for SYSTEM_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYSTEM_RADIO_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYSTEM_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_RADIO_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_BITRATE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_UNUSED: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_DISCRETE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_VARIABLE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_BITRATE_TYPE_FREE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(3i32);
impl ::core::marker::Copy for WPD_BITRATE_TYPES {}
impl ::core::clone::Clone for WPD_BITRATE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_BITRATE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_BITRATE_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_BITRATE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_BITRATE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_CAPTURE_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_UNDEFINED: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_NORMAL: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_BURST: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CAPTURE_MODE_TIMELAPSE: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(3i32);
impl ::core::marker::Copy for WPD_CAPTURE_MODES {}
impl ::core::clone::Clone for WPD_CAPTURE_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_CAPTURE_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_CAPTURE_MODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_CAPTURE_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_CAPTURE_MODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_COLOR_CORRECTED_STATUS_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COLOR_CORRECTED_STATUS_NOT_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COLOR_CORRECTED_STATUS_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COLOR_CORRECTED_STATUS_SHOULD_NOT_BE_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(2i32);
impl ::core::marker::Copy for WPD_COLOR_CORRECTED_STATUS_VALUES {}
impl ::core::clone::Clone for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_COLOR_CORRECTED_STATUS_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_COLOR_CORRECTED_STATUS_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_COMMAND_ACCESS_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_READ: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_READWRITE: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_STGM_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_FILE_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(8i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_COMMAND_ACCESS_FROM_ATTRIBUTE_WITH_METHOD_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(16i32);
impl ::core::marker::Copy for WPD_COMMAND_ACCESS_TYPES {}
impl ::core::clone::Clone for WPD_COMMAND_ACCESS_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_COMMAND_ACCESS_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_COMMAND_ACCESS_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_COMMAND_ACCESS_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_COMMAND_ACCESS_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_CROPPED_STATUS_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CROPPED_STATUS_NOT_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CROPPED_STATUS_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_CROPPED_STATUS_SHOULD_NOT_BE_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(2i32);
impl ::core::marker::Copy for WPD_CROPPED_STATUS_VALUES {}
impl ::core::clone::Clone for WPD_CROPPED_STATUS_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_CROPPED_STATUS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_CROPPED_STATUS_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_CROPPED_STATUS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_CROPPED_STATUS_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_DEVICE_TRANSPORTS(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_UNSPECIFIED: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_USB: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_IP: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TRANSPORT_BLUETOOTH: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(3i32);
impl ::core::marker::Copy for WPD_DEVICE_TRANSPORTS {}
impl ::core::clone::Clone for WPD_DEVICE_TRANSPORTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_DEVICE_TRANSPORTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_DEVICE_TRANSPORTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_DEVICE_TRANSPORTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_DEVICE_TRANSPORTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_DEVICE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_GENERIC: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_CAMERA: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_MEDIA_PLAYER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_PHONE: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_VIDEO: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(5i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_DEVICE_TYPE_AUDIO_RECORDER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(6i32);
impl ::core::marker::Copy for WPD_DEVICE_TYPES {}
impl ::core::clone::Clone for WPD_DEVICE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_DEVICE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_DEVICE_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_DEVICE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_DEVICE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_EFFECT_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_UNDEFINED: WPD_EFFECT_MODES = WPD_EFFECT_MODES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_COLOR: WPD_EFFECT_MODES = WPD_EFFECT_MODES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_BLACK_AND_WHITE: WPD_EFFECT_MODES = WPD_EFFECT_MODES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EFFECT_MODE_SEPIA: WPD_EFFECT_MODES = WPD_EFFECT_MODES(3i32);
impl ::core::marker::Copy for WPD_EFFECT_MODES {}
impl ::core::clone::Clone for WPD_EFFECT_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_EFFECT_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_EFFECT_MODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_EFFECT_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EFFECT_MODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_EXPOSURE_METERING_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_UNDEFINED: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_CENTER_WEIGHTED_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_MULTI_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_METERING_MODE_CENTER_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(4i32);
impl ::core::marker::Copy for WPD_EXPOSURE_METERING_MODES {}
impl ::core::clone::Clone for WPD_EXPOSURE_METERING_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_EXPOSURE_METERING_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_EXPOSURE_METERING_MODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_EXPOSURE_METERING_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EXPOSURE_METERING_MODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_EXPOSURE_PROGRAM_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_UNDEFINED: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_MANUAL: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_AUTO: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_APERTURE_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_SHUTTER_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_CREATIVE: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(5i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_ACTION: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(6i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_PORTRAIT: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(7i32);
impl ::core::marker::Copy for WPD_EXPOSURE_PROGRAM_MODES {}
impl ::core::clone::Clone for WPD_EXPOSURE_PROGRAM_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_EXPOSURE_PROGRAM_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_EXPOSURE_PROGRAM_MODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_EXPOSURE_PROGRAM_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EXPOSURE_PROGRAM_MODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_FLASH_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_UNDEFINED: WPD_FLASH_MODES = WPD_FLASH_MODES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_OFF: WPD_FLASH_MODES = WPD_FLASH_MODES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_RED_EYE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_RED_EYE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(5i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FLASH_MODE_EXTERNAL_SYNC: WPD_FLASH_MODES = WPD_FLASH_MODES(6i32);
impl ::core::marker::Copy for WPD_FLASH_MODES {}
impl ::core::clone::Clone for WPD_FLASH_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_FLASH_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_FLASH_MODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_FLASH_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FLASH_MODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_FOCUS_METERING_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_METERING_MODE_UNDEFINED: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_METERING_MODE_CENTER_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_METERING_MODE_MULTI_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(2i32);
impl ::core::marker::Copy for WPD_FOCUS_METERING_MODES {}
impl ::core::clone::Clone for WPD_FOCUS_METERING_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_FOCUS_METERING_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_FOCUS_METERING_MODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_FOCUS_METERING_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FOCUS_METERING_MODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_FOCUS_MODES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_UNDEFINED: WPD_FOCUS_MODES = WPD_FOCUS_MODES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_MANUAL: WPD_FOCUS_MODES = WPD_FOCUS_MODES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_AUTOMATIC: WPD_FOCUS_MODES = WPD_FOCUS_MODES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_FOCUS_AUTOMATIC_MACRO: WPD_FOCUS_MODES = WPD_FOCUS_MODES(3i32);
impl ::core::marker::Copy for WPD_FOCUS_MODES {}
impl ::core::clone::Clone for WPD_FOCUS_MODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_FOCUS_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_FOCUS_MODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_FOCUS_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FOCUS_MODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_META_GENRES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_UNUSED: WPD_META_GENRES = WPD_META_GENRES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_NON_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(17i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_AUDIO_BOOK_FILES: WPD_META_GENRES = WPD_META_GENRES(18i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_FILES_NON_AUDIO_BOOK: WPD_META_GENRES = WPD_META_GENRES(19i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_NEWS: WPD_META_GENRES = WPD_META_GENRES(20i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_SPOKEN_WORD_TALK_SHOWS: WPD_META_GENRES = WPD_META_GENRES(21i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(33i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_NEWS_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(34i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_MUSIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(35i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_HOME_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(36i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_FEATURE_FILM_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(37i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_TELEVISION_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(38i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_TRAINING_EDUCATIONAL_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(39i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_PHOTO_MONTAGE_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(40i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_GENERIC_NON_AUDIO_NON_VIDEO: WPD_META_GENRES = WPD_META_GENRES(48i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_AUDIO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(64i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_VIDEO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(65i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_META_GENRE_MIXED_PODCAST: WPD_META_GENRES = WPD_META_GENRES(66i32);
impl ::core::marker::Copy for WPD_META_GENRES {}
impl ::core::clone::Clone for WPD_META_GENRES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_META_GENRES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_META_GENRES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_META_GENRES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_META_GENRES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_OPERATION_STATES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_UNSPECIFIED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_STARTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_RUNNING: WPD_OPERATION_STATES = WPD_OPERATION_STATES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_PAUSED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_CANCELLED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_FINISHED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(5i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_OPERATION_STATE_ABORTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(6i32);
impl ::core::marker::Copy for WPD_OPERATION_STATES {}
impl ::core::clone::Clone for WPD_OPERATION_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_OPERATION_STATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_OPERATION_STATES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_OPERATION_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_OPERATION_STATES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_PARAMETER_USAGE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_RETURN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_IN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_OUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_USAGE_INOUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(3i32);
impl ::core::marker::Copy for WPD_PARAMETER_USAGE_TYPES {}
impl ::core::clone::Clone for WPD_PARAMETER_USAGE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_PARAMETER_USAGE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_PARAMETER_USAGE_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_PARAMETER_USAGE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_PARAMETER_USAGE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_POWER_SOURCES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_POWER_SOURCE_BATTERY: WPD_POWER_SOURCES = WPD_POWER_SOURCES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_POWER_SOURCE_EXTERNAL: WPD_POWER_SOURCES = WPD_POWER_SOURCES(1i32);
impl ::core::marker::Copy for WPD_POWER_SOURCES {}
impl ::core::clone::Clone for WPD_POWER_SOURCES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_POWER_SOURCES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_POWER_SOURCES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_POWER_SOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_POWER_SOURCES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_OBJECT: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_RESOURCE: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(1i32);
impl ::core::marker::Copy for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {}
impl ::core::clone::Clone for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_SECTION_DATA_UNITS_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SECTION_DATA_UNITS_BYTES: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SECTION_DATA_UNITS_MILLISECONDS: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(1i32);
impl ::core::marker::Copy for WPD_SECTION_DATA_UNITS_VALUES {}
impl ::core::clone::Clone for WPD_SECTION_DATA_UNITS_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_SECTION_DATA_UNITS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_SECTION_DATA_UNITS_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_SECTION_DATA_UNITS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SECTION_DATA_UNITS_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_SERVICE_INHERITANCE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_SERVICE_INHERITANCE_IMPLEMENTATION: WPD_SERVICE_INHERITANCE_TYPES = WPD_SERVICE_INHERITANCE_TYPES(0i32);
impl ::core::marker::Copy for WPD_SERVICE_INHERITANCE_TYPES {}
impl ::core::clone::Clone for WPD_SERVICE_INHERITANCE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_SERVICE_INHERITANCE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_SERVICE_INHERITANCE_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_SERVICE_INHERITANCE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SERVICE_INHERITANCE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_SMS_ENCODING_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_ENCODING_7_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_ENCODING_8_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const SMS_ENCODING_UTF_16: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(2i32);
impl ::core::marker::Copy for WPD_SMS_ENCODING_TYPES {}
impl ::core::clone::Clone for WPD_SMS_ENCODING_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_SMS_ENCODING_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_SMS_ENCODING_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_SMS_ENCODING_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SMS_ENCODING_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_STORAGE_ACCESS_CAPABILITY_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READWRITE: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITHOUT_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITH_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(2i32);
impl ::core::marker::Copy for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {}
impl ::core::clone::Clone for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STORAGE_ACCESS_CAPABILITY_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_STORAGE_TYPE_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_UNDEFINED: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_FIXED_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_REMOVABLE_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_FIXED_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STORAGE_TYPE_REMOVABLE_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(4i32);
impl ::core::marker::Copy for WPD_STORAGE_TYPE_VALUES {}
impl ::core::clone::Clone for WPD_STORAGE_TYPE_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_STORAGE_TYPE_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_STORAGE_TYPE_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_STORAGE_TYPE_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STORAGE_TYPE_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_STREAM_UNITS(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(8i32);
impl ::core::marker::Copy for WPD_STREAM_UNITS {}
impl ::core::clone::Clone for WPD_STREAM_UNITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_STREAM_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_STREAM_UNITS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_STREAM_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STREAM_UNITS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_VIDEO_SCAN_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_UNUSED: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(5i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(6i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE_AND_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(7i32);
impl ::core::marker::Copy for WPD_VIDEO_SCAN_TYPES {}
impl ::core::clone::Clone for WPD_VIDEO_SCAN_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_VIDEO_SCAN_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_VIDEO_SCAN_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_VIDEO_SCAN_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_VIDEO_SCAN_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPD_WHITE_BALANCE_SETTINGS(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_UNDEFINED: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_MANUAL: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_ONE_PUSH_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_DAYLIGHT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(4i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_FLORESCENT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(5i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_TUNGSTEN: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(6i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_WHITE_BALANCE_FLASH: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(7i32);
impl ::core::marker::Copy for WPD_WHITE_BALANCE_SETTINGS {}
impl ::core::clone::Clone for WPD_WHITE_BALANCE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPD_WHITE_BALANCE_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WPD_WHITE_BALANCE_SETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WPD_WHITE_BALANCE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_WHITE_BALANCE_SETTINGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WpdAttributeForm(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_UNSPECIFIED: WpdAttributeForm = WpdAttributeForm(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_RANGE: WpdAttributeForm = WpdAttributeForm(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_ENUMERATION: WpdAttributeForm = WpdAttributeForm(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdAttributeForm = WpdAttributeForm(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdAttributeForm = WpdAttributeForm(4i32);
impl ::core::marker::Copy for WpdAttributeForm {}
impl ::core::clone::Clone for WpdAttributeForm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WpdAttributeForm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WpdAttributeForm {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WpdAttributeForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WpdAttributeForm").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WpdParameterAttributeForm(pub i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_UNSPECIFIED: WpdParameterAttributeForm = WpdParameterAttributeForm(0i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_RANGE: WpdParameterAttributeForm = WpdParameterAttributeForm(1i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_ENUMERATION: WpdParameterAttributeForm = WpdParameterAttributeForm(2i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdParameterAttributeForm = WpdParameterAttributeForm(3i32);
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdParameterAttributeForm = WpdParameterAttributeForm(4i32);
impl ::core::marker::Copy for WpdParameterAttributeForm {}
impl ::core::clone::Clone for WpdParameterAttributeForm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WpdParameterAttributeForm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WpdParameterAttributeForm {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WpdParameterAttributeForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WpdParameterAttributeForm").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    pub Command: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    pub AccessType: u32,
    pub AccessProperty: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WPD_COMMAND_ACCESS_LOOKUP_ENTRY").field("Command", &self.Command).field("AccessType", &self.AccessType).field("AccessProperty", &self.AccessProperty).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::TypeKind for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command && self.AccessType == other.AccessType && self.AccessProperty == other.AccessProperty
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
