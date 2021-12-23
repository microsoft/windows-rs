#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_WPD_NAMESPACE_EXTENSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35786d3c_b075_49b9_88dd_029876e11c01);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type DELETE_OBJECT_OPTIONS = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const PORTABLE_DEVICE_DELETE_NO_RECURSION: DELETE_OBJECT_OPTIONS = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const PORTABLE_DEVICE_DELETE_WITH_RECURSION: DELETE_OBJECT_OPTIONS = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type DEVICE_RADIO_STATE = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_RADIO_ON: DEVICE_RADIO_STATE = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_SW_RADIO_OFF: DEVICE_RADIO_STATE = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_HW_RADIO_OFF: DEVICE_RADIO_STATE = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_SW_HW_RADIO_OFF: DEVICE_RADIO_STATE = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_HW_RADIO_ON_UNCONTROLLABLE: DEVICE_RADIO_STATE = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_RADIO_INVALID: DEVICE_RADIO_STATE = 5i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_HW_RADIO_OFF_UNCONTROLLABLE: DEVICE_RADIO_STATE = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DRS_RADIO_MAX: DEVICE_RADIO_STATE = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_MTPBTH_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xea1237fa_589d_4472_84e4_0abe36fd62ef), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DEVSVCTYPE_ABSTRACT: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DEVSVCTYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const DEVSVC_SERVICEINFO_VERSION: u32 = 100u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DMProcessConfigXMLFiltered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszxmlin: Param0, rgszallowedcspnodes: *const super::super::Foundation::PWSTR, dwnumallowedcspnodes: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMProcessConfigXMLFiltered(pszxmlin: super::super::Foundation::PWSTR, rgszallowedcspnodes: *const super::super::Foundation::PWSTR, dwnumallowedcspnodes: u32, pbstrxmlout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        DMProcessConfigXMLFiltered(pszxmlin.into_param().abi(), ::core::mem::transmute(rgszallowedcspnodes), ::core::mem::transmute(dwnumallowedcspnodes), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_AnchorStateInvalid: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_AnchorStateNormal: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_AnchorStateOld: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_ItemStateChanged: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_ItemStateCreated: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_ItemStateDeleted: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_ItemStateInvalid: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_AnchorResults_ItemStateUpdated: u32 = 3u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_CalendarObj_BusyStatusBusy: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_CalendarObj_BusyStatusFree: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_CalendarObj_BusyStatusOutOfOffice: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_CalendarObj_BusyStatusTentative: u32 = 3u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_DeviceMetadataObj_DefaultCABFalse: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_DeviceMetadataObj_DefaultCABTrue: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternInstanceFirst: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternInstanceFourth: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternInstanceLast: u32 = 5u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternInstanceNone: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternInstanceSecond: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternInstanceThird: u32 = 3u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternTypeDaily: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternTypeMonthly: u32 = 3u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternTypeWeekly: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PatternTypeYearly: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PriorityHighest: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PriorityLowest: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_PriorityNormal: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_ReadFalse: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_MessageObj_ReadTrue: u32 = 255u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_StatusSvc_ChargingActive: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_StatusSvc_ChargingInactive: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_StatusSvc_ChargingUnknown: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_StatusSvc_RoamingActive: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_StatusSvc_RoamingInactive: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_StatusSvc_RoamingUnknown: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_SyncSvc_SyncObjectReferencesDisabled: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_SyncSvc_SyncObjectReferencesEnabled: u32 = 255u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_TaskObj_CompleteFalse: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const ENUM_TaskObj_CompleteTrue: u32 = 255u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_DEVICE_ALREADY_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731135i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_DEVICE_IS_HUNG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731130i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_DEVICE_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731134i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731133i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730934i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731132i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730933i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_OBJECT_NOT_COMMITED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731131i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_SERVICE_ALREADY_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730936i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_SERVICE_BAD_PARAMETER_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730932i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_SERVICE_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144730935i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_SMS_INVALID_MESSAGE_BODY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731035i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_SMS_INVALID_RECIPIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731036i32);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const E_WPD_SMS_SERVICE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144731034i32);
pub const EnumBthMtpConnectors: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1570149_e645_4f43_8b0d_409b061db2fc);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FACILITY_WPD: u32 = 42u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekFriday: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekMonday: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekNone: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekSaturday: u32 = 64u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekSunday: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekThursday: u32 = 16u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekTuesday: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const FLAG_MessageObj_DayOfWeekWednesday: u32 = 8u32;
pub const GUID_DEVINTERFACE_WPD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac27878_a6fa_4155_ba85_f98f491d4f33);
pub const GUID_DEVINTERFACE_WPD_PRIVATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba0c718f_4ded_49b7_bdd3_fabe28661211);
pub const GUID_DEVINTERFACE_WPD_SERVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ef44f80_3d64_4246_a6aa_206f328d1edc);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IConnectionRequestCallback(::windows::core::IUnknown);
impl IConnectionRequestCallback {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrstatus)).ok()
    }
}
impl ::core::convert::From<IConnectionRequestCallback> for ::windows::core::IUnknown {
    fn from(value: IConnectionRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionRequestCallback> for ::windows::core::IUnknown {
    fn from(value: &IConnectionRequestCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectionRequestCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConnectionRequestCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IConnectionRequestCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272c9ae0_7161_4ae0_91bd_9f448ee9c427);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IEnumPortableDeviceConnectors(::windows::core::IUnknown);
impl IEnumPortableDeviceConnectors {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Next(&self, crequested: u32, pconnectors: *mut ::core::option::Option<IPortableDeviceConnector>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequested), ::core::mem::transmute(pconnectors), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Skip(&self, cconnectors: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnectors)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumPortableDeviceConnectors> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumPortableDeviceConnectors>(result__)
    }
}
impl ::core::convert::From<IEnumPortableDeviceConnectors> for ::windows::core::IUnknown {
    fn from(value: IEnumPortableDeviceConnectors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumPortableDeviceConnectors> for ::windows::core::IUnknown {
    fn from(value: &IEnumPortableDeviceConnectors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumPortableDeviceConnectors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumPortableDeviceConnectors {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumPortableDeviceConnectors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IEnumPortableDeviceConnectorsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfdef549_9247_454f_bd82_06fe80853faa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceConnectorsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IEnumPortableDeviceObjectIDs(::windows::core::IUnknown);
impl IEnumPortableDeviceObjectIDs {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cobjects), ::core::mem::transmute(pobjids), ::core::mem::transmute(pcfetched)))
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Skip(&self, cobjects: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cobjects)))
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IEnumPortableDeviceObjectIDs> for ::windows::core::IUnknown {
    fn from(value: IEnumPortableDeviceObjectIDs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumPortableDeviceObjectIDs> for ::windows::core::IUnknown {
    fn from(value: &IEnumPortableDeviceObjectIDs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumPortableDeviceObjectIDs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumPortableDeviceObjectIDs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumPortableDeviceObjectIDs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IEnumPortableDeviceObjectIDsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ece955_cf41_4728_bfa0_41eedf1bbf19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceObjectIDsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IMediaRadioManager(::windows::core::IUnknown);
impl IMediaRadioManager {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetRadioInstances(&self) -> ::windows::core::Result<IRadioInstanceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IRadioInstanceCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sysradiostate), ::core::mem::transmute(utimeoutsec)).ok()
    }
}
impl ::core::convert::From<IMediaRadioManager> for ::windows::core::IUnknown {
    fn from(value: IMediaRadioManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaRadioManager> for ::windows::core::IUnknown {
    fn from(value: &IMediaRadioManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaRadioManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMediaRadioManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaRadioManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IMediaRadioManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cfdcab5_fc47_42a5_9241_074b58830e73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IMediaRadioManagerNotifySink(::windows::core::IUnknown);
impl IMediaRadioManagerNotifySink {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnInstanceAdd<'a, Param0: ::windows::core::IntoParam<'a, IRadioInstance>>(&self, pradioinstance: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pradioinstance.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInstanceRemove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrradioinstanceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrradioinstanceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInstanceRadioChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrradioinstanceid: Param0, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrradioinstanceid.into_param().abi(), ::core::mem::transmute(radiostate)).ok()
    }
}
impl ::core::convert::From<IMediaRadioManagerNotifySink> for ::windows::core::IUnknown {
    fn from(value: IMediaRadioManagerNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaRadioManagerNotifySink> for ::windows::core::IUnknown {
    fn from(value: &IMediaRadioManagerNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaRadioManagerNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMediaRadioManagerNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaRadioManagerNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IMediaRadioManagerNotifySinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89d81f5f_c147_49ed_a11c_77b20c31e7c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManagerNotifySinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pradioinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const IOCTL_WPD_MESSAGE_READWRITE_ACCESS: u32 = 4243720u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const IOCTL_WPD_MESSAGE_READ_ACCESS: u32 = 4210952u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDevice(::windows::core::IUnknown);
impl IPortableDevice {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pszpnpdeviceid: Param0, pclientinfo: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn SendCommand<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pparameters: Param1) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pparameters.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Content(&self) -> ::windows::core::Result<IPortableDeviceContent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceContent>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Capabilities(&self) -> ::windows::core::Result<IPortableDeviceCapabilities> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceCapabilities>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Advise<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceEventCallback>, Param2: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pcallback: Param1, pparameters: Param2) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi(), pparameters.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unadvise<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcookie: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszcookie.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPnPDeviceID(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPortableDevice> for ::windows::core::IUnknown {
    fn from(value: IPortableDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevice> for ::windows::core::IUnknown {
    fn from(value: &IPortableDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceCapabilities(::windows::core::IUnknown);
impl IPortableDeviceCapabilities {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedCommands(&self) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(command), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetFunctionalCategories(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetFunctionalObjects(&self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedContentTypes(&self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedFormats(&self, contenttype: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(contenttype), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetFixedPropertyAttributes(&self, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedEvents(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetEventOptions(&self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceCapabilities> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceCapabilities> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c8c6dbf_e3dc_4061_becc_8542e810d126);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcategories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppcontenttypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *const ::windows::core::GUID, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceConnector(::windows::core::IUnknown);
impl IPortableDeviceConnector {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Disconnect<'a, Param0: ::windows::core::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel<'a, Param0: ::windows::core::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Devices_Properties'*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub unsafe fn GetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropertykey), ::core::mem::transmute(ppropertytype), ::core::mem::transmute(ppdata), ::core::mem::transmute(pcbdata)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Devices_Properties'*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub unsafe fn SetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(pdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPnPID(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceConnector> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceConnector> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceConnectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceConnectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Devices_Properties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))] usize,
    #[cfg(feature = "Win32_Devices_Properties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceContent(::windows::core::IUnknown);
impl IPortableDeviceContent {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pszparentobjectid: Param1, pfilter: Param2) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IPortableDeviceProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceProperties>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Transfer(&self) -> ::windows::core::Result<IPortableDeviceResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceResources>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateObjectWithPropertiesOnly<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppszobjectid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateObjectWithPropertiesAndData<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Delete<'a, Param1: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, dwoptions: u32, pobjectids: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions), pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<'a, Param0: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, ppersistentuniqueids: Param0) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ppersistentuniqueids.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Move<'a, Param0: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Copy<'a, Param0: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceContent> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceContent> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a96ed84_7c73_4480_9938_bf5af477d426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppersistentuniqueids: ::windows::core::RawPtr, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceContent2(::windows::core::IUnknown);
impl IPortableDeviceContent2 {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pszparentobjectid: Param1, pfilter: Param2) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IPortableDeviceProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceProperties>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Transfer(&self) -> ::windows::core::Result<IPortableDeviceResources> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceResources>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateObjectWithPropertiesOnly<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppszobjectid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateObjectWithPropertiesAndData<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Delete<'a, Param1: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, dwoptions: u32, pobjectids: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions), pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<'a, Param0: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, ppersistentuniqueids: Param0) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ppersistentuniqueids.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Move<'a, Param0: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Copy<'a, Param0: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateObjectWithPropertiesAndData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pszobjectid: Param0, pproperties: Param1, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pproperties.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceContent2> for IPortableDeviceContent {
    fn from(value: IPortableDeviceContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceContent2> for IPortableDeviceContent {
    fn from(value: &IPortableDeviceContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPortableDeviceContent> for IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPortableDeviceContent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPortableDeviceContent> for &IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPortableDeviceContent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPortableDeviceContent2> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceContent2> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceContent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceContent2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b4add96_f6bf_4034_8708_eca72bf10554);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppersistentuniqueids: ::windows::core::RawPtr, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pproperties: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceDataStream(::windows::core::IUnknown);
impl IPortableDeviceDataStream {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::StructuredStorage::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectID(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPortableDeviceDataStream> for super::super::System::Com::IStream {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPortableDeviceDataStream> for super::super::System::Com::IStream {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IStream> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IStream> for &IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPortableDeviceDataStream> for super::super::System::Com::ISequentialStream {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPortableDeviceDataStream> for super::super::System::Com::ISequentialStream {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::ISequentialStream> for &IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPortableDeviceDataStream> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceDataStream> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceDataStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceDataStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceDataStream {}
impl ::core::fmt::Debug for IPortableDeviceDataStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceDataStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceDataStream {
    type Vtable = IPortableDeviceDataStreamVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88e04db3_1012_4d64_9996_f703a950d3f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDataStreamVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: super::super::System::Com::StructuredStorage::STGC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceDispatchFactory(::windows::core::IUnknown);
impl IPortableDeviceDispatchFactory {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetDeviceDispatch<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceDispatchFactory> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceDispatchFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceDispatchFactory> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceDispatchFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceDispatchFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceDispatchFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceDispatchFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceDispatchFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e1eafc3_e3d7_4132_96fa_759c0f9d1e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDispatchFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, ppdevicedispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceEventCallback(::windows::core::IUnknown);
impl IPortableDeviceEventCallback {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnEvent<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, peventparameters: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), peventparameters.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPortableDeviceEventCallback> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceEventCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceEventCallback> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceEventCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceEventCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8792a31_f385_493c_a893_40f64eb45f6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceEventCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceKeyCollection(::windows::core::IUnknown);
impl IPortableDeviceKeyCollection {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pkey)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceKeyCollection> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceKeyCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceKeyCollection> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceKeyCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceKeyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceKeyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceKeyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceKeyCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdada2357_e0ad_492e_98db_dd61c53ba353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceKeyCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceManager(::windows::core::IUnknown);
impl IPortableDeviceManager {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDevices(&self, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppnpdeviceids), ::core::mem::transmute(pcpnpdeviceids)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn RefreshDeviceList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pdevicefriendlyname: Param1, pcchdevicefriendlyname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pdevicefriendlyname.into_param().abi(), ::core::mem::transmute(pcchdevicefriendlyname)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pdevicedescription: Param1, pcchdevicedescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pdevicedescription.into_param().abi(), ::core::mem::transmute(pcchdevicedescription)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceManufacturer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pdevicemanufacturer: Param1, pcchdevicemanufacturer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchdevicemanufacturer)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pszdevicepropertyname: Param1, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pszdevicepropertyname.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(pcbdata), ::core::mem::transmute(pdwtype)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrivateDevices(&self, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppnpdeviceids), ::core::mem::transmute(pcpnpdeviceids)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceManager> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceManager> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1567595_4c2f_4574_a6fa_ecef917b9a40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicefriendlyname: super::super::Foundation::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicedescription: super::super::Foundation::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicemanufacturer: super::super::Foundation::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pszdevicepropertyname: super::super::Foundation::PWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDevicePropVariantCollection(::windows::core::IUnknown);
impl IPortableDevicePropVariantCollection {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Add(&self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn ChangeType(&self, vt: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(vt)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<IPortableDevicePropVariantCollection> for ::windows::core::IUnknown {
    fn from(value: IPortableDevicePropVariantCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevicePropVariantCollection> for ::windows::core::IUnknown {
    fn from(value: &IPortableDevicePropVariantCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDevicePropVariantCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDevicePropVariantCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevicePropVariantCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDevicePropVariantCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89b2e422_4f1b_4316_bcef_a44afea83eb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropVariantCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceProperties(::windows::core::IUnknown);
impl IPortableDeviceProperties {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetPropertyAttributes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pkeys.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pszobjectid: Param0, pvalues: Param1) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pvalues.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceProperties> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceProperties> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDevicePropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f6d695c_03df_4439_a809_59266beee3a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pvalues: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDevicePropertiesBulk(::windows::core::IUnknown);
impl IPortableDevicePropertiesBulk {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn QueueGetValuesByObjectList<'a, Param0: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceKeyCollection>, Param2: ::windows::core::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pobjectids: Param0, pkeys: Param1, pcallback: Param2) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pkeys.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGetValuesByObjectFormat<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IPortableDeviceKeyCollection>, Param4: ::windows::core::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: Param1, dwdepth: u32, pkeys: Param3, pcallback: Param4) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidobjectformat), pszparentobjectid.into_param().abi(), ::core::mem::transmute(dwdepth), pkeys.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn QueueSetValuesByObjectList<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValuesCollection>, Param1: ::windows::core::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pobjectvalues: Param0, pcallback: Param1) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pobjectvalues.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Start(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext)).ok()
    }
}
impl ::core::convert::From<IPortableDevicePropertiesBulk> for ::windows::core::IUnknown {
    fn from(value: IPortableDevicePropertiesBulk) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevicePropertiesBulk> for ::windows::core::IUnknown {
    fn from(value: &IPortableDevicePropertiesBulk) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDevicePropertiesBulk {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDevicePropertiesBulk {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevicePropertiesBulk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDevicePropertiesBulkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x482b05c0_4056_44ed_9e0f_5e23b009da93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: super::super::Foundation::PWSTR, dwdepth: u32, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectvalues: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDevicePropertiesBulkCallback(::windows::core::IUnknown);
impl IPortableDevicePropertiesBulkCallback {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnStart(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnProgress<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValuesCollection>>(&self, pcontext: *const ::windows::core::GUID, presults: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext), presults.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnEnd(&self, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext), ::core::mem::transmute(hrstatus)).ok()
    }
}
impl ::core::convert::From<IPortableDevicePropertiesBulkCallback> for ::windows::core::IUnknown {
    fn from(value: IPortableDevicePropertiesBulkCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDevicePropertiesBulkCallback> for ::windows::core::IUnknown {
    fn from(value: &IPortableDevicePropertiesBulkCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDevicePropertiesBulkCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDevicePropertiesBulkCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDevicePropertiesBulkCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDevicePropertiesBulkCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9deacb80_11e8_40e3_a9f3_f557986a7845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulkCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceResources(::windows::core::IUnknown);
impl IPortableDeviceResources {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedResources<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetResourceAttributes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdwoptimalbuffersize), ::core::mem::transmute(ppstream)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateResource<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, presourceattributes: Param0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presourceattributes.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceResources> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceResources) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceResources> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceResources) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceResources {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceResources {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceResourcesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd8878ac_d841_4d17_891c_e6829cdb6934);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceResourcesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourceattributes: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceService(::windows::core::IUnknown);
impl IPortableDeviceService {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pszpnpserviceid: Param0, pclientinfo: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Capabilities(&self) -> ::windows::core::Result<IPortableDeviceServiceCapabilities> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceServiceCapabilities>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Content(&self) -> ::windows::core::Result<IPortableDeviceContent2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceContent2>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Methods(&self) -> ::windows::core::Result<IPortableDeviceServiceMethods> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceServiceMethods>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServiceObjectID(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPnPServiceID(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Advise<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceEventCallback>, Param2: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pcallback: Param1, pparameters: Param2) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi(), pparameters.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unadvise<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcookie: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszcookie.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn SendCommand<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pparameters: Param1) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pparameters.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceService> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceService> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3bd3a44_d7b5_40a9_98b7_2fa4d01dec08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceActivation(::windows::core::IUnknown);
impl IPortableDeviceServiceActivation {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>, Param2: ::windows::core::IntoParam<'a, IPortableDeviceServiceOpenCallback>>(&self, pszpnpserviceid: Param0, pclientinfo: Param1, pcallback: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn CancelOpenAsync(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceActivation> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceServiceActivation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceActivation> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceServiceActivation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceServiceActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceServiceActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceActivation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceServiceActivationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe56b0534_d9b9_425c_9b99_75f97cb3d7c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceActivationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceCapabilities(::windows::core::IUnknown);
impl IPortableDeviceServiceCapabilities {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedMethods(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedMethodsByFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetMethodAttributes(&self, method: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetMethodParameterAttributes(&self, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), ::core::mem::transmute(parameter), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedFormats(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetFormatAttributes(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetFormatPropertyAttributes(&self, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(property), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedEvents(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetEventAttributes(&self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetEventParameterAttributes(&self, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), ::core::mem::transmute(parameter), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetInheritedServices(&self, dwinheritancetype: u32) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinheritancetype), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetFormatRenderingProfiles(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValuesCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValuesCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSupportedCommands(&self) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(command), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceCapabilities> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceServiceCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceCapabilities> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceServiceCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceServiceCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceServiceCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceServiceCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24dbd89d_413e_43e0_bd5b_197f3c56c886);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, pprenderingprofiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceManager(::windows::core::IUnknown);
impl IPortableDeviceServiceManager {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceServices<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, guidservicecategory: *const ::windows::core::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(guidservicecategory), ::core::mem::transmute(pservices), ::core::mem::transmute(pcservices)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceForService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpserviceid: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpnpserviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPortableDeviceServiceManager> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceServiceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceManager> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceServiceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceServiceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceServiceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceServiceManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8abc4e9_a84a_47a9_80b3_c5d9b172a961);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, guidservicecategory: *const ::windows::core::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceMethodCallback(::windows::core::IUnknown);
impl IPortableDeviceServiceMethodCallback {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnComplete<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, hrstatus: ::windows::core::HRESULT, presults: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrstatus), presults.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceMethodCallback> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceServiceMethodCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceMethodCallback> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceServiceMethodCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceServiceMethodCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceServiceMethodCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceMethodCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceServiceMethodCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc424233c_afce_4828_a756_7ed7a2350083);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethodCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceMethods(::windows::core::IUnknown);
impl IPortableDeviceServiceMethods {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Invoke<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, method: *const ::windows::core::GUID, pparameters: Param1, ppresults: *mut ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), pparameters.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn InvokeAsync<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>, Param2: ::windows::core::IntoParam<'a, IPortableDeviceServiceMethodCallback>>(&self, method: *const ::windows::core::GUID, pparameters: Param1, pcallback: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), pparameters.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceServiceMethodCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceMethods> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceServiceMethods) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceMethods> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceServiceMethods) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceServiceMethods {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceServiceMethods {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceMethods {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceServiceMethodsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe20333c9_fd34_412d_a381_cc6f2d820df7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethodsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceServiceOpenCallback(::windows::core::IUnknown);
impl IPortableDeviceServiceOpenCallback {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn OnComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrstatus)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceServiceOpenCallback> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceServiceOpenCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceServiceOpenCallback> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceServiceOpenCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceServiceOpenCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceServiceOpenCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceServiceOpenCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceServiceOpenCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbced49c8_8efe_41ed_960b_61313abd47a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceOpenCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceUnitsStream(::windows::core::IUnknown);
impl IPortableDeviceUnitsStream {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(units), ::core::mem::transmute(dworigin), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceUnitsStream> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceUnitsStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceUnitsStream> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceUnitsStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceUnitsStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceUnitsStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceUnitsStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceUnitsStreamVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e98025f_bfc4_47a2_9a5f_bc900a507c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceUnitsStreamVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceValues(::windows::core::IUnknown);
impl IPortableDeviceValues {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetCount(&self, pcelt: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetAt(&self, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pkey), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Com_StructuredStorage', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetStringValue<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetStringValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetBoolValue<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetBoolValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIUnknownValue<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIUnknownValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(pvalue), ::core::mem::transmute(cbvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(ppvalue), ::core::mem::transmute(pcbvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDeviceValuesValue<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDeviceValuesValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDevicePropVariantCollectionValue<'a, Param1: ::windows::core::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDevicePropVariantCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDeviceKeyCollectionValue<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceKeyCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetIPortableDeviceValuesCollectionValue<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValuesCollection>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValuesCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValuesCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn RemoveValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CopyValuesFromPropertyStore<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, pstore: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), pstore.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CopyValuesToPropertyStore<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, pstore: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), pstore.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceValues> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceValues) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceValues> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceValues) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceValuesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6848f6f2_3155_4f86_b6f5_263eeeab3143);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValuesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceValuesCollection(::windows::core::IUnknown);
impl IPortableDeviceValuesCollection {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetAt(&self, dwindex: u32) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pvalues.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<IPortableDeviceValuesCollection> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceValuesCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceValuesCollection> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceValuesCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceValuesCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceValuesCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceValuesCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IPortableDeviceValuesCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e3f2d79_4e07_48c4_8208_d8c2e5af4a99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValuesCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IPortableDeviceWebControl(::windows::core::IUnknown);
impl IPortableDeviceWebControl {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetDeviceFromId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, deviceid: Param0) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), deviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetDeviceFromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, deviceid: Param0, pcompletionhandler: Param1, perrorhandler: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), deviceid.into_param().abi(), pcompletionhandler.into_param().abi(), perrorhandler.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPortableDeviceWebControl> for super::super::System::Com::IDispatch {
    fn from(value: IPortableDeviceWebControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPortableDeviceWebControl> for super::super::System::Com::IDispatch {
    fn from(value: &IPortableDeviceWebControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPortableDeviceWebControl> for ::windows::core::IUnknown {
    fn from(value: IPortableDeviceWebControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceWebControl> for ::windows::core::IUnknown {
    fn from(value: &IPortableDeviceWebControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPortableDeviceWebControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceWebControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceWebControl {}
impl ::core::fmt::Debug for IPortableDeviceWebControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceWebControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPortableDeviceWebControl {
    type Vtable = IPortableDeviceWebControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94fc7953_5ca1_483a_8aee_df52e7747d00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceWebControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcompletionhandler: ::windows::core::RawPtr, perrorhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IRadioInstance(::windows::core::IUnknown);
impl IRadioInstance {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetRadioManagerSignature(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInstanceSignature(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName(&self, lcid: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetRadioState(&self) -> ::windows::core::Result<DEVICE_RADIO_STATE> {
        let mut result__: DEVICE_RADIO_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DEVICE_RADIO_STATE>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiostate), ::core::mem::transmute(utimeoutsec)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMultiComm(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAssociatingDevice(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IRadioInstance> for ::windows::core::IUnknown {
    fn from(value: IRadioInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadioInstance> for ::windows::core::IUnknown {
    fn from(value: &IRadioInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRadioInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRadioInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRadioInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IRadioInstanceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70aa1c9e_f2b4_4c61_86d3_6b9fb75fd1a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstanceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IRadioInstanceCollection(::windows::core::IUnknown);
impl IRadioInstanceCollection {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetAt(&self, uindex: u32) -> ::windows::core::Result<IRadioInstance> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<IRadioInstance>(result__)
    }
}
impl ::core::convert::From<IRadioInstanceCollection> for ::windows::core::IUnknown {
    fn from(value: IRadioInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadioInstanceCollection> for ::windows::core::IUnknown {
    fn from(value: &IRadioInstanceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRadioInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRadioInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRadioInstanceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IRadioInstanceCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5791fae_5665_4e0c_95be_5fde31644185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstanceCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
#[repr(transparent)]
pub struct IWpdSerializer(::windows::core::IUnknown);
impl IWpdSerializer {
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: *const u8, dwinputbufferlength: u32) -> ::windows::core::Result<IPortableDeviceValues> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwinputbufferlength), ::core::mem::transmute(&mut result__)).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn WriteIPortableDeviceValuesToBuffer<'a, Param1: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, dwoutputbufferlength: u32, presults: Param1, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputbufferlength), presults.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwbyteswritten)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetBufferFromIPortableDeviceValues<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, psource: Param0, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psource.into_param().abi(), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pdwbuffersize)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
    pub unsafe fn GetSerializedSize<'a, Param0: ::windows::core::IntoParam<'a, IPortableDeviceValues>>(&self, psource: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), psource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWpdSerializer> for ::windows::core::IUnknown {
    fn from(value: IWpdSerializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWpdSerializer> for ::windows::core::IUnknown {
    fn from(value: &IWpdSerializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWpdSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWpdSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWpdSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    type Vtable = IWpdSerializerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32f4002_bb27_45ff_af4f_06631c1e8dad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWpdSerializerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: ::windows::core::RawPtr, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, pdwsize: *mut u32) -> ::windows::core::HRESULT,
);
pub const PortableDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x728a21c5_3d9e_48d7_9810_864848f0f404);
pub const PortableDeviceDispatchFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43232233_8338_4658_ae01_0b4ae830b6b0);
pub const PortableDeviceFTM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7c0039a_4762_488a_b4b3_760ef9a1ba9b);
pub const PortableDeviceKeyCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde2d022d_2480_43be_97f0_d1fa2cf98f4f);
pub const PortableDeviceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0af10cec_2ecd_4b92_9581_34f6ae0637f3);
pub const PortableDevicePropVariantCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08a99e2f_6d6d_4b80_af5a_baf2bcbe4cb9);
pub const PortableDeviceService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5db4c2_9312_422c_9152_411cd9c4dd84);
pub const PortableDeviceServiceFTM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1649b154_c794_497a_9b03_f3f0121302f3);
pub const PortableDeviceValues: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c15d503_d017_47ce_9016_7b3f978721cc);
pub const PortableDeviceValuesCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3882134d_14cf_4220_9cb4_435f86d83f60);
pub const PortableDeviceWebControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x186dd02c_2dec_41b5_a7d4_b59056fade51);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_MessageObj_PatternDayOfMonth: u32 = 31u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_MessageObj_PatternMonthOfYear: u32 = 12u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_StatusSvc_BatteryLife: u32 = 100u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_StatusSvc_MissedCalls: u32 = 255u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_StatusSvc_NewPictures: u32 = 65535u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_StatusSvc_SignalStrength: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_StatusSvc_TextMessages: u32 = 255u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMAX_StatusSvc_VoiceMail: u32 = 255u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMIN_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMIN_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMIN_StatusSvc_BatteryLife: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGEMIN_StatusSvc_SignalStrength: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGESTEP_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGESTEP_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGESTEP_StatusSvc_BatteryLife: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const RANGESTEP_StatusSvc_SignalStrength: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type SMS_MESSAGE_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SMS_TEXT_MESSAGE: SMS_MESSAGE_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SMS_BINARY_MESSAGE: SMS_MESSAGE_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE: u32 = 3u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SYNCSVC_FILTER_CONTACTS_WITH_PHONE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SYNCSVC_FILTER_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SYNCSVC_FILTER_TASK_ACTIVE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type SYSTEM_RADIO_STATE = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SRS_RADIO_ENABLED: SYSTEM_RADIO_STATE = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SRS_RADIO_DISABLED: SYSTEM_RADIO_STATE = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_AnchorSyncSvc: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_CalendarSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_ContactsSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_DeviceMetadataSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_FullEnumSyncSvc: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_HintsSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_MessageSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_NotesSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_RingtonesSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_StatusSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const TYPE_TasksSvc: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_HAS_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 7u32 };
pub const WPDNSE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPDNSE_PROPSHEET_CONTENT_DETAILS: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPDNSE_PROPSHEET_CONTENT_GENERAL: u32 = 4u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPDNSE_PROPSHEET_CONTENT_REFERENCES: u32 = 8u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPDNSE_PROPSHEET_CONTENT_RESOURCES: u32 = 16u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPDNSE_PROPSHEET_DEVICE_GENERAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPDNSE_PROPSHEET_STORAGE_GENERAL: u32 = 2u32;
pub const WPD_API_OPTIONS_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_IOCTL_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_API_OPTION_USE_CLEAR_DATA_STREAM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_ACCEPTED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_DECLINED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 3u32 };
pub const WPD_APPOINTMENT_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_OPTIONAL_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_REQUIRED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TENTATIVE_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_APPOINTMENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BIT_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_BLOCK_ALIGNMENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_CHANNEL_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_AUDIO_FORMAT_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_BITRATE_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_BITRATE_TYPE_UNUSED: WPD_BITRATE_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_BITRATE_TYPE_DISCRETE: WPD_BITRATE_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_BITRATE_TYPE_VARIABLE: WPD_BITRATE_TYPES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_BITRATE_TYPE_FREE: WPD_BITRATE_TYPES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_CAPTURE_MODES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CAPTURE_MODE_UNDEFINED: WPD_CAPTURE_MODES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CAPTURE_MODE_NORMAL: WPD_CAPTURE_MODES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CAPTURE_MODE_BURST: WPD_CAPTURE_MODES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CAPTURE_MODE_TIMELAPSE: WPD_CAPTURE_MODES = 3i32;
pub const WPD_CATEGORY_CAPABILITIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356);
pub const WPD_CATEGORY_COMMON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a);
pub const WPD_CATEGORY_DEVICE_HINTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84);
pub const WPD_CATEGORY_MEDIA_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8);
pub const WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56);
pub const WPD_CATEGORY_NETWORK_CONFIGURATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4);
pub const WPD_CATEGORY_NULL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const WPD_CATEGORY_OBJECT_ENUMERATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec);
pub const WPD_CATEGORY_OBJECT_MANAGEMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089);
pub const WPD_CATEGORY_OBJECT_PROPERTIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804);
pub const WPD_CATEGORY_OBJECT_PROPERTIES_BULK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e);
pub const WPD_CATEGORY_OBJECT_RESOURCES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a);
pub const WPD_CATEGORY_SERVICE_CAPABILITIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89);
pub const WPD_CATEGORY_SERVICE_COMMON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee);
pub const WPD_CATEGORY_SERVICE_METHODS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc);
pub const WPD_CATEGORY_SMS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1);
pub const WPD_CATEGORY_STILL_IMAGE_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32);
pub const WPD_CATEGORY_STORAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DEVICE_IDENTIFICATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_DONT_REGISTER_WPD_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_MULTITRANSPORT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_REGISTER_WPD_PRIVATE_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SILENCE_AUTOPLAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLASS_EXTENSION_OPTIONS_TRANSPORT_BANDWIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 4u32 };
pub const WPD_CLASS_EXTENSION_OPTIONS_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96);
pub const WPD_CLASS_EXTENSION_OPTIONS_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f);
pub const WPD_CLASS_EXTENSION_OPTIONS_V3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30);
pub const WPD_CLASS_EXTENSION_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051);
pub const WPD_CLASS_EXTENSION_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_DESIRED_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_EVENT_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 11u32 };
pub const WPD_CLIENT_INFORMATION_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MAJOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MANUAL_CLOSE_ON_DISCONNECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINIMUM_RESULTS_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_MINOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_REVISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SECURITY_QUALITY_OF_SERVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_SHARE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CLIENT_WMDRM_APPLICATION_PRIVATE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_COLOR_CORRECTED_STATUS_VALUES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COLOR_CORRECTED_STATUS_NOT_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COLOR_CORRECTED_STATUS_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COLOR_CORRECTED_STATUS_SHOULD_NOT_BE_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
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
unsafe impl ::windows::core::Abi for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WPD_COMMAND_ACCESS_LOOKUP_ENTRY>()) == 0 }
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
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_COMMAND_ACCESS_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COMMAND_ACCESS_READ: WPD_COMMAND_ACCESS_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COMMAND_ACCESS_READWRITE: WPD_COMMAND_ACCESS_TYPES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_STGM_ACCESS: WPD_COMMAND_ACCESS_TYPES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_FILE_ACCESS: WPD_COMMAND_ACCESS_TYPES = 8i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_COMMAND_ACCESS_FROM_ATTRIBUTE_WITH_METHOD_ACCESS: WPD_COMMAND_ACCESS_TYPES = 16i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FIXED_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_REGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_UNREGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_CLASS_EXTENSION_WRITE_DEVICE_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMIT_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_GET_OBJECT_IDS_FROM_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_RESET_DEVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_COMMON_SAVE_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_DEVICE_HINTS_GET_CONTENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_GENERATE_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_PAUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MEDIA_CAPTURE_STOP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 17u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 18u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_READ_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_MTP_EXT_WRITE_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 16u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_END_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_FIND_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_ENUMERATION_START_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COMMIT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COPY_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_ONLY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_DELETE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_MOVE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_REVERT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_UPDATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_WRITE_OBJECT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ALL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_PROPERTIES_SET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CLOSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_COMMIT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_CREATE_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_OPEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_REVERT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK_IN_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_OBJECT_RESOURCES_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_PROCESS_WIRELESS_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 16u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS_BY_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_COMMON_GET_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_CANCEL_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_END_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SERVICE_METHODS_START_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_SMS_SEND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STILL_IMAGE_CAPTURE_INITIATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_EJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMAND_STORAGE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_BODY_TEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_END_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_NOTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 7u32 };
pub const WPD_COMMON_INFORMATION_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_PRIORITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_START_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_COMMON_INFORMATION_SUBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ANNIVERSARY_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 62u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ASSISTANT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 61u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BIRTHDATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 57u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 34u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 35u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 45u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 17u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 40u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 41u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 20u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 23u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 18u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 19u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 22u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 21u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_BUSINESS_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 50u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_CHILDREN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 60u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 54u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 51u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 52u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_INSTANT_MESSENGER3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 53u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MIDDLE_NAMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 42u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_MOBILE_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 43u32 };
pub const WPD_CONTACT_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_EMAILS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 36u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 24u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_PHONES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 47u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 27u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 25u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 26u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 29u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 30u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 28u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PAGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 46u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 32u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 33u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 44u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 38u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 39u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 16u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PERSONAL_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 49u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 55u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PHONETIC_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PREFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_EMAIL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 31u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 58u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 37u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_PRIMARY_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 48u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_RINGTONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 63u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_ROLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 56u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SPOUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 59u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_CONTACT_SUFFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 7u32 };
pub const WPD_CONTENT_TYPE_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e170d2_1055_4a3e_b952_82cc4f8a8689);
pub const WPD_CONTENT_TYPE_APPOINTMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fed060e_8793_4b1e_90c9_48ac389ac631);
pub const WPD_CONTENT_TYPE_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ad2c85e_5e2d_45e5_8864_4f229e3c6cf0);
pub const WPD_CONTENT_TYPE_AUDIO_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa18737e_5009_48fa_ae21_85f24383b4e6);
pub const WPD_CONTENT_TYPE_CALENDAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1fd5967_6023_49a0_9df1_f8060be751b0);
pub const WPD_CONTENT_TYPE_CERTIFICATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc3876e8_a948_4060_9050_cbd77e8a3d87);
pub const WPD_CONTENT_TYPE_CONTACT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeaba8313_4525_4707_9f0e_87c6808e9435);
pub const WPD_CONTENT_TYPE_CONTACT_GROUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x346b8932_4c36_40d8_9415_1828291f9de9);
pub const WPD_CONTENT_TYPE_DOCUMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x680adf52_950a_4041_9b41_65e393648155);
pub const WPD_CONTENT_TYPE_EMAIL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8038044a_7e51_4f8f_883d_1d0623d14533);
pub const WPD_CONTENT_TYPE_FOLDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27e2e392_a111_48e0_ab0c_e17705a05f85);
pub const WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99ed0160_17ff_4c44_9d98_1d7a6f941921);
pub const WPD_CONTENT_TYPE_GENERIC_FILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0085e0a6_8d34_45d7_bc5c_447e59c73d48);
pub const WPD_CONTENT_TYPE_GENERIC_MESSAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe80eaaf8_b2db_4133_b67e_1bef4b4a6e5f);
pub const WPD_CONTENT_TYPE_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef2107d5_a52a_4243_a26b_62d4176d7603);
pub const WPD_CONTENT_TYPE_IMAGE_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75793148_15f5_4a30_a813_54ed8a37e226);
pub const WPD_CONTENT_TYPE_MEDIA_CAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e88b3cc_3e65_4e62_bfff_229495253ab0);
pub const WPD_CONTENT_TYPE_MEMO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cd20ecf_3b50_414f_a641_e473ffe45751);
pub const WPD_CONTENT_TYPE_MIXED_CONTENT_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f0c3ac_a593_49ac_9219_24abca5a2563);
pub const WPD_CONTENT_TYPE_NETWORK_ASSOCIATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x031da7ee_18c8_4205_847e_89a11261d0f3);
pub const WPD_CONTENT_TYPE_PLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a33f7e4_af13_48f5_994e_77369dfe04a3);
pub const WPD_CONTENT_TYPE_PROGRAM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd269f96a_247c_4bff_98fb_97f3c49220e6);
pub const WPD_CONTENT_TYPE_SECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x821089f5_1d91_4dc9_be3c_bbb1b35b18ce);
pub const WPD_CONTENT_TYPE_TASK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63252f2c_887f_4cb6_b1ac_d29855dcef6c);
pub const WPD_CONTENT_TYPE_TELEVISION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60a169cf_f2ae_4e21_9375_9677f11c1c6e);
pub const WPD_CONTENT_TYPE_UNSPECIFIED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28d8d31e_249c_454e_aabc_34883168e634);
pub const WPD_CONTENT_TYPE_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9261b03c_3d78_4519_85e3_02c5e1f50bb9);
pub const WPD_CONTENT_TYPE_VIDEO_ALBUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x012b0db7_d4c1_45d6_b081_94b87779614f);
pub const WPD_CONTENT_TYPE_WIRELESS_PROFILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bac070a_9f5f_4da4_a8f6_3de44d68fd6c);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CONTROL_FUNCTION_GENERIC_MESSAGE: u32 = 66u32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_CROPPED_STATUS_VALUES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CROPPED_STATUS_NOT_CROPPED: WPD_CROPPED_STATUS_VALUES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CROPPED_STATUS_CROPPED: WPD_CROPPED_STATUS_VALUES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_CROPPED_STATUS_SHOULD_NOT_BE_CROPPED: WPD_CROPPED_STATUS_VALUES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_EDP_IDENTITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed), pid: 1u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FIRMWARE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_FUNCTIONAL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_MODEL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_NETWORK_IDENTIFIER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 16u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_LEVEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_POWER_SOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 5u32 };
pub const WPD_DEVICE_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc);
pub const WPD_DEVICE_PROPERTIES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799);
pub const WPD_DEVICE_PROPERTIES_V3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_PROTOCOL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_DRM_SCHEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SUPPORTS_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_SYNC_PARTNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TRANSPORT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_DEVICE_TRANSPORTS = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TRANSPORT_UNSPECIFIED: WPD_DEVICE_TRANSPORTS = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TRANSPORT_USB: WPD_DEVICE_TRANSPORTS = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TRANSPORT_IP: WPD_DEVICE_TRANSPORTS = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TRANSPORT_BLUETOOTH: WPD_DEVICE_TRANSPORTS = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_DEVICE_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TYPE_GENERIC: WPD_DEVICE_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TYPE_CAMERA: WPD_DEVICE_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TYPE_MEDIA_PLAYER: WPD_DEVICE_TYPES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TYPE_PHONE: WPD_DEVICE_TYPES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TYPE_VIDEO: WPD_DEVICE_TYPES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER: WPD_DEVICE_TYPES = 5i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_DEVICE_TYPE_AUDIO_RECORDER: WPD_DEVICE_TYPES = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_DEVICE_USE_DEVICE_STAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 5u32 };
pub const WPD_DOCUMENT_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b110203_eb95_4f02_93e0_97c631493ad5);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_EFFECT_MODES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EFFECT_MODE_UNDEFINED: WPD_EFFECT_MODES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EFFECT_MODE_COLOR: WPD_EFFECT_MODES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EFFECT_MODE_BLACK_AND_WHITE: WPD_EFFECT_MODES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EFFECT_MODE_SEPIA: WPD_EFFECT_MODES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_BCC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_CC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_ATTACHMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_HAS_BEEN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 7u32 };
pub const WPD_EMAIL_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_RECEIVED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_SENDER_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EMAIL_TO_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 2u32 };
pub const WPD_EVENT_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 3u32 };
pub const WPD_EVENT_DEVICE_CAPABILITIES_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36885aa1_cd54_4daa_b3d0_afb3e03f5999);
pub const WPD_EVENT_DEVICE_REMOVED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4cbca1b_6918_48b9_85ee_02be7c850af9);
pub const WPD_EVENT_DEVICE_RESET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7755cf53_c1ed_44f3_b5a2_451e2c376b27);
pub const WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_5738_4ff2_8445_be3126691059);
pub const WPD_EVENT_NOTIFICATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ba2e40a_6b4c_4295_bb43_26322b99aeb2);
pub const WPD_EVENT_OBJECT_ADDED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa726da95_e207_4b02_8d44_bef2e86cbffc);
pub const WPD_EVENT_OBJECT_REMOVED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe82ab88_a52c_4823_96e5_d0272671fc38);
pub const WPD_EVENT_OBJECT_TRANSFER_REQUESTED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d16a0a1_f2c6_41da_8f19_5e53721adbf2);
pub const WPD_EVENT_OBJECT_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1445a759_2e01_485d_9f27_ff07dae697ab);
pub const WPD_EVENT_OPTIONS_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_AUTOPLAY_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_OPTION_IS_BROADCAST_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_CHILD_HIERARCHY_CHANGED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_EVENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_CREATION_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OBJECT_PARENT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_PROGRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_OPERATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_PNP_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_EVENT_PARAMETER_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846), pid: 2u32 };
pub const WPD_EVENT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0);
pub const WPD_EVENT_PROPERTIES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846);
pub const WPD_EVENT_SERVICE_METHOD_COMPLETE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a33f5f8_0acc_4d9b_9cc4_112d353b86ca);
pub const WPD_EVENT_STORAGE_FORMAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3782616b_22bc_4474_a251_3070f8d38857);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_EXPOSURE_METERING_MODES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_METERING_MODE_UNDEFINED: WPD_EXPOSURE_METERING_MODES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_METERING_MODE_AVERAGE: WPD_EXPOSURE_METERING_MODES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_METERING_MODE_CENTER_WEIGHTED_AVERAGE: WPD_EXPOSURE_METERING_MODES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_METERING_MODE_MULTI_SPOT: WPD_EXPOSURE_METERING_MODES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_METERING_MODE_CENTER_SPOT: WPD_EXPOSURE_METERING_MODES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_EXPOSURE_PROGRAM_MODES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_UNDEFINED: WPD_EXPOSURE_PROGRAM_MODES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_MANUAL: WPD_EXPOSURE_PROGRAM_MODES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_AUTO: WPD_EXPOSURE_PROGRAM_MODES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_APERTURE_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_SHUTTER_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_CREATIVE: WPD_EXPOSURE_PROGRAM_MODES = 5i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_ACTION: WPD_EXPOSURE_PROGRAM_MODES = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_EXPOSURE_PROGRAM_MODE_PORTRAIT: WPD_EXPOSURE_PROGRAM_MODES = 7i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_FLASH_MODES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FLASH_MODE_UNDEFINED: WPD_FLASH_MODES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FLASH_MODE_AUTO: WPD_FLASH_MODES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FLASH_MODE_OFF: WPD_FLASH_MODES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FLASH_MODE_FILL: WPD_FLASH_MODES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FLASH_MODE_RED_EYE_AUTO: WPD_FLASH_MODES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FLASH_MODE_RED_EYE_FILL: WPD_FLASH_MODES = 5i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FLASH_MODE_EXTERNAL_SYNC: WPD_FLASH_MODES = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_FOCUS_METERING_MODES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FOCUS_METERING_MODE_UNDEFINED: WPD_FOCUS_METERING_MODES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FOCUS_METERING_MODE_CENTER_SPOT: WPD_FOCUS_METERING_MODES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FOCUS_METERING_MODE_MULTI_SPOT: WPD_FOCUS_METERING_MODES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_FOCUS_MODES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FOCUS_UNDEFINED: WPD_FOCUS_MODES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FOCUS_MANUAL: WPD_FOCUS_MODES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FOCUS_AUTOMATIC: WPD_FOCUS_MODES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_FOCUS_AUTOMATIC_MACRO: WPD_FOCUS_MODES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FOLDER_CONTENT_TYPES_ALLOWED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d), pid: 2u32 };
pub const WPD_FOLDER_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d);
pub const WPD_FORMAT_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_MIMETYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FORMAT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 2u32 };
pub const WPD_FUNCTIONAL_CATEGORY_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d8a6512_a74c_448e_ba8a_f4ac07c49399);
pub const WPD_FUNCTIONAL_CATEGORY_AUDIO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f2a1919_c7c2_4a00_855d_f57cf06debbb);
pub const WPD_FUNCTIONAL_CATEGORY_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08ea466b_e3a4_4336_a1f3_a44d2b5c438c);
pub const WPD_FUNCTIONAL_CATEGORY_NETWORK_CONFIGURATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48f4db72_7c6a_4ab0_9e1a_470e3cdbf26a);
pub const WPD_FUNCTIONAL_CATEGORY_RENDERING_INFORMATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08600ba4_a7ba_4a01_ab0e_0065d0a356d3);
pub const WPD_FUNCTIONAL_CATEGORY_SMS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0044a0b1_c1e9_4afd_b358_a62c6117c9cf);
pub const WPD_FUNCTIONAL_CATEGORY_STILL_IMAGE_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x613ca327_ab93_4900_b4fa_895bb5874b79);
pub const WPD_FUNCTIONAL_CATEGORY_STORAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23f05bbc_15de_4c2a_a55b_a9af5ce412ef);
pub const WPD_FUNCTIONAL_CATEGORY_VIDEO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe23e5f6b_7243_43aa_8df1_0eb3d968a918);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_FUNCTIONAL_OBJECT_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598), pid: 2u32 };
pub const WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_BITDEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_COLOR_CORRECTED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_CROPPED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_HORIZONTAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 9u32 };
pub const WPD_IMAGE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_IMAGE_VERTICAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ALBUM_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 25u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 24u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_AUDIO_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 49u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BITRATE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BUY_NOW: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 20u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_BYTE_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 36u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COMPOSER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_COPYRIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 31u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DESTINATION_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 30u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_DURATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 19u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 21u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 32u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 38u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 23u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_ACCESSED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_LAST_BUILD_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 35u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_MANAGING_EDITOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 27u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_META_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OBJECT_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 34u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 26u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_PARENTAL_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 9u32 };
pub const WPD_MEDIA_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_RELEASE_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SAMPLE_RATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SKIP_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SOURCE_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 29u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_STAR_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 16u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUBSCRIPTION_CONTENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 39u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_SUB_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 33u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TIME_TO_LIVE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 37u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 18u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_TOTAL_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USER_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 17u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_USE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WEBMASTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 28u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MEDIA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 22u32 };
pub const WPD_MEMO_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ffbfc7b_7483_41ad_afb9_da3f4e592b8d);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_META_GENRES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_UNUSED: WPD_META_GENRES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_GENERIC_MUSIC_AUDIO_FILE: WPD_META_GENRES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_GENERIC_NON_MUSIC_AUDIO_FILE: WPD_META_GENRES = 17i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_SPOKEN_WORD_AUDIO_BOOK_FILES: WPD_META_GENRES = 18i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_SPOKEN_WORD_FILES_NON_AUDIO_BOOK: WPD_META_GENRES = 19i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_SPOKEN_WORD_NEWS: WPD_META_GENRES = 20i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_SPOKEN_WORD_TALK_SHOWS: WPD_META_GENRES = 21i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_GENERIC_VIDEO_FILE: WPD_META_GENRES = 33i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_NEWS_VIDEO_FILE: WPD_META_GENRES = 34i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_MUSIC_VIDEO_FILE: WPD_META_GENRES = 35i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_HOME_VIDEO_FILE: WPD_META_GENRES = 36i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_FEATURE_FILM_VIDEO_FILE: WPD_META_GENRES = 37i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_TELEVISION_VIDEO_FILE: WPD_META_GENRES = 38i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_TRAINING_EDUCATIONAL_VIDEO_FILE: WPD_META_GENRES = 39i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_PHOTO_MONTAGE_VIDEO_FILE: WPD_META_GENRES = 40i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_GENERIC_NON_AUDIO_NON_VIDEO: WPD_META_GENRES = 48i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_AUDIO_PODCAST: WPD_META_GENRES = 64i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_VIDEO_PODCAST: WPD_META_GENRES = 65i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_META_GENRE_MIXED_PODCAST: WPD_META_GENRES = 66i32;
pub const WPD_METHOD_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_ASSOCIATED_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_METHOD_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_ALBUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_LYRICS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_MOOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 8u32 };
pub const WPD_MUSIC_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_MUSIC_TRACK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_HOST_NETWORK_IDENTIFIERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 2u32 };
pub const WPD_NETWORK_ASSOCIATION_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_NETWORK_ASSOCIATION_X509V3SEQUENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_BACK_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 21u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 26u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTAINER_FUNCTIONAL_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 23u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_AUTHORED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 20u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_CREATED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 18u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_DATE_MODIFIED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 19u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 6u32 };
pub const WPD_OBJECT_FORMAT_3G2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9850000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_3G2A: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a11202d_8759_4e34_ba5e_b1211087eee4);
pub const WPD_OBJECT_FORMAT_3GP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9840000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_3GPA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5172730_f971_41ef_a10b_2271a0019d7a);
pub const WPD_OBJECT_FORMAT_AAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT_GROUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_MEDIA_CAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba0b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ALL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1f62eb2_4bb3_479c_9cfa_05b5f3a57b22);
pub const WPD_OBJECT_FORMAT_AMR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ASF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x300c0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ASXPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba130000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ATSCTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9870000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AUDIBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AVCHD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9860000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AVI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x300a0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_BMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_CIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38050000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_DPOF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_DVBTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9880000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_EXECUTABLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_EXIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_FLAC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_FLASHPIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_GIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_HTML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30050000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ICALENDAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ICON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077232ed_102c_4638_9c22_83f142bfc822);
pub const WPD_OBJECT_FORMAT_JFIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380f0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JPEGXR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JPX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38100000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_M3UPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba110000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_M4A: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30aba7ac_6ffd_4c23_a359_3e9b52f3f1c8);
pub const WPD_OBJECT_FORMAT_MHT_COMPILED_HTML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba840000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_EXCEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba850000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_POWERPOINT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba860000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WFC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WORD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MKV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9900000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30090000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9820000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MPEG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x300b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MPLPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba120000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_NETWORK_ASSOCIATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_OGG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PCD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38090000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PICT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380a0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PLSPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba140000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PNG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PROPERTIES_ONLY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_QCELP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_SCRIPT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380d0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFFEP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFFIT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380e0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_UNSPECIFIED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30000000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCALENDAR1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCARD2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb820000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCARD3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WAVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WBMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WINDOWSIMAGEFORMAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WMA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WMV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WPLPLAYLIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba100000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_X509V3CERTIFICATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_XML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba820000_ae6c_4804_98ba_c57b46965fe7);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_GENERATE_THUMBNAIL_FROM_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 24u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_HINT_LOCATION_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 25u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISHIDDEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ISSYSTEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_IS_DRM_PROTECTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 17u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_KEYWORDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_LANGUAGE_LOCALE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 27u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_ORIGINAL_FILE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 5u32 };
pub const WPD_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c);
pub const WPD_OBJECT_PROPERTIES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SUPPORTED_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OBJECT_SYNC_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 16u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_OPERATION_STATES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_OPERATION_STATE_UNSPECIFIED: WPD_OPERATION_STATES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_OPERATION_STATE_STARTED: WPD_OPERATION_STATES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_OPERATION_STATE_RUNNING: WPD_OPERATION_STATES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_OPERATION_STATE_PAUSED: WPD_OPERATION_STATES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_OPERATION_STATE_CANCELLED: WPD_OPERATION_STATES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_OPERATION_STATE_FINISHED: WPD_OPERATION_STATES = 5i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_OPERATION_STATE_ABORTED: WPD_OPERATION_STATES = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_MANAGEMENT_RECURSIVE_DELETE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_NO_INPUT_BUFFER_ON_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_READ_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_WRITE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_SMS_BINARY_MESSAGE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 5001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_OPTION_VALID_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 5001u32 };
pub const WPD_PARAMETER_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_ORDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PARAMETER_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_PARAMETER_USAGE_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_USAGE_RETURN: WPD_PARAMETER_USAGE_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_USAGE_IN: WPD_PARAMETER_USAGE_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_USAGE_OUT: WPD_PARAMETER_USAGE_TYPES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_USAGE_INOUT: WPD_PARAMETER_USAGE_TYPES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_POWER_SOURCES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_POWER_SOURCE_BATTERY: WPD_POWER_SOURCES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_POWER_SOURCE_EXTERNAL: WPD_POWER_SOURCES = 1i32;
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d545058_8900_40b3_8f1d_dc246e1e8370);
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d545058_4fce_4578_95c8_8698a9bc0f49);
pub const WPD_PROPERTY_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37);
pub const WPD_PROPERTY_ATTRIBUTES_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FAST_PROPERTY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1008u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1007u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1014u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1015u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1010u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1009u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1012u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1011u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1013u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_REGISTRATION_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_ACTIVITY_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1011u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1009u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1010u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_COMMAND_TARGET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1008u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1007u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_LOCATIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_ef88_4e4d_95c3_4f327f728a96), pid: 1011u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1013u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1012u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1009u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1008u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1010u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1011u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1007u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1014u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_NULL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_FILTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_NUM_OBJECTS_REQUESTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_COPY_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1013u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CREATION_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1007u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1010u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DESTINATION_FOLDER_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1011u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_MOVE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1012u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1016u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1009u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1008u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1015u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_UPDATE_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1014u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1007u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PARENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1008u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_ACCESS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1010u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1007u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1008u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1009u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1011u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_POSITION_FROM_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1014u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1012u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_ORIGIN_FLAG: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1013u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_STREAM_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1016u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SUPPORTS_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1015u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_PUBLIC_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1018u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1019u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1012u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1013u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1007u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1008u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITANCE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1014u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1015u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1006u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1010u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1009u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1016u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1017u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1011u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1005u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_PARAMETER_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_METHOD_RESULT_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_BINARY_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1004u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_MESSAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_RECIPIENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1001u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_SMS_TEXT_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1003u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_DESTINATION_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1002u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_PROPERTY_STORAGE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1001u32 };
pub const WPD_RENDERING_INFORMATION_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_CREATABLE_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_OBJECT: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_RESOURCE: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf02aa354_2300_4e2d_a1b9_3b6730f7fa21), pid: 0u32 };
pub const WPD_RESOURCE_ATTRIBUTES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_READ_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_WRITE_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_RESOURCE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ATTRIBUTE_TOTAL_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x3bc13982_85b1_48e0_95a6_8d3ad06be117), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_BRANDING_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb633b1ae_6caf_4a87_9589_22ded6dd5899), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x2c4d6803_80ea_4580_af9a_5be1a23eddcb), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_DEFAULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe81e79be_34f0_41bf_b53f_f1a06ae87842), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_GENERIC: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb9b9f515_ba70_4647_94dc_fa4925e95a07), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf195fed8_aa28_4ee3_b153_e182dd5edc39), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc7c407ba_98fa_46b5_9960_23fec124cfde), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_RESOURCE_VIDEO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb566ee42_6368_4290_8662_70182fb79f20), pid: 0u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_REFERENCED_OBJECT_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SECTION_DATA_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_SECTION_DATA_UNITS_VALUES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_SECTION_DATA_UNITS_BYTES: WPD_SECTION_DATA_UNITS_VALUES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_SECTION_DATA_UNITS_MILLISECONDS: WPD_SECTION_DATA_UNITS_VALUES = 1i32;
pub const WPD_SECTION_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66);
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_SERVICE_INHERITANCE_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_SERVICE_INHERITANCE_IMPLEMENTATION: WPD_SERVICE_INHERITANCE_TYPES = 0i32;
pub const WPD_SERVICE_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SERVICE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_ENCODING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_SMS_ENCODING_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SMS_ENCODING_7_BIT: WPD_SMS_ENCODING_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SMS_ENCODING_8_BIT: WPD_SMS_ENCODING_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const SMS_ENCODING_UTF_16: WPD_SMS_ENCODING_TYPES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_MAX_PAYLOAD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 4u32 };
pub const WPD_SMS_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_PROVIDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_SMS_TIMEOUT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 29u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 24u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_BURST_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 23u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 31u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAMERA_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 30u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_DELAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 17u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 18u32 };
pub const WPD_STILL_IMAGE_CAPTURE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CAPTURE_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_COMPRESSION_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_CONTRAST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 19u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_DIGITAL_ZOOM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 21u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EFFECT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 22u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_BIAS_COMPENSATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 16u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_PROGRAM_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FLASH_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCAL_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 27u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_FOCUS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_RGB_GAIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_SHARPNESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 20u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 26u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_TIMELAPSE_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 25u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_UPLOAD_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 28u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STILL_IMAGE_WHITE_BALANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_ACCESS_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_STORAGE_ACCESS_CAPABILITY_VALUES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READWRITE: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITHOUT_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITH_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_CAPACITY_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FILE_SYSTEM_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 3u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_BYTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_FREE_SPACE_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_MAX_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 9u32 };
pub const WPD_STORAGE_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_STORAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_STORAGE_TYPE_VALUES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_TYPE_UNDEFINED: WPD_STORAGE_TYPE_VALUES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_TYPE_FIXED_ROM: WPD_STORAGE_TYPE_VALUES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_TYPE_REMOVABLE_ROM: WPD_STORAGE_TYPE_VALUES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_TYPE_FIXED_RAM: WPD_STORAGE_TYPE_VALUES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STORAGE_TYPE_REMOVABLE_RAM: WPD_STORAGE_TYPE_VALUES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_STREAM_UNITS = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = 8i32;
pub const WPD_TASK_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_PERCENT_COMPLETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_REMINDER_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 10u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_TASK_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 6u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_AUTHOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 2u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 13u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 8u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_CREDITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 9u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FOURCC_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 14u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_FRAMERATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 15u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_KEY_FRAME_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 10u32 };
pub const WPD_VIDEO_OBJECT_PROPERTIES_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a);
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_QUALITY_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 11u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_CHANNEL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 5u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_REPEAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 7u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_RECORDEDTV_STATION_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 4u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices', 'Win32_UI_Shell_PropertiesSystem'*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const WPD_VIDEO_SCAN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 12u32 };
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_VIDEO_SCAN_TYPES = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_UNUSED: WPD_VIDEO_SCAN_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = 5i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE: WPD_VIDEO_SCAN_TYPES = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE_AND_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = 7i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WPD_WHITE_BALANCE_SETTINGS = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_UNDEFINED: WPD_WHITE_BALANCE_SETTINGS = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_MANUAL: WPD_WHITE_BALANCE_SETTINGS = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_ONE_PUSH_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_DAYLIGHT: WPD_WHITE_BALANCE_SETTINGS = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_FLORESCENT: WPD_WHITE_BALANCE_SETTINGS = 5i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_TUNGSTEN: WPD_WHITE_BALANCE_SETTINGS = 6i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_WHITE_BALANCE_FLASH: WPD_WHITE_BALANCE_SETTINGS = 7i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WpdAttributeForm = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_UNSPECIFIED: WpdAttributeForm = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_RANGE: WpdAttributeForm = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_ENUMERATION: WpdAttributeForm = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdAttributeForm = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdAttributeForm = 4i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub type WpdParameterAttributeForm = i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_UNSPECIFIED: WpdParameterAttributeForm = 0i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_RANGE: WpdParameterAttributeForm = 1i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_ENUMERATION: WpdParameterAttributeForm = 2i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdParameterAttributeForm = 3i32;
#[doc = "*Required features: 'Win32_Devices_PortableDevices'*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdParameterAttributeForm = 4i32;
pub const WpdSerializer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b91a74b_ad7c_4a9d_b563_29eef9167172);
