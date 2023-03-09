#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct INetworkTransportSettings(::windows::core::IUnknown);
impl INetworkTransportSettings {
    #[doc = "*Required features: `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplySetting)(::windows::core::Interface::as_raw(self), settingid, valuein.len() as _, ::core::mem::transmute(valuein.as_ptr()), lengthout, valueout).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QuerySetting)(::windows::core::Interface::as_raw(self), settingid, valuein.len() as _, ::core::mem::transmute(valuein.as_ptr()), lengthout, valueout).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkTransportSettings, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetworkTransportSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkTransportSettings {}
impl ::core::fmt::Debug for INetworkTransportSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkTransportSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkTransportSettings {
    type Vtable = INetworkTransportSettings_Vtbl;
}
impl ::core::clone::Clone for INetworkTransportSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkTransportSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e7abb2c_f2c1_4a61_bd35_deb7a08ab0f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkTransportSettings_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub ApplySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    ApplySetting: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub QuerySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    QuerySetting: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct INotificationTransportSync(::windows::core::IUnknown);
impl INotificationTransportSync {
    pub unsafe fn CompleteDelivery(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteDelivery)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(INotificationTransportSync, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INotificationTransportSync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotificationTransportSync {}
impl ::core::fmt::Debug for INotificationTransportSync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotificationTransportSync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INotificationTransportSync {
    type Vtable = INotificationTransportSync_Vtbl;
}
impl ::core::clone::Clone for INotificationTransportSync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INotificationTransportSync {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eb1402_0ab8_49c0_9e14_a1ae4ba93058);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationTransportSync_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CompleteDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCBuddy(::windows::core::IUnknown);
impl IRTCBuddy {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.PresentityURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPresentityURI)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Data)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.Persistent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPersistent)(::windows::core::Interface::as_raw(self), fpersistent.into_param().abi()).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = ::windows::core::zeroed::<RTC_PRESENCE_STATUS>();
        (::windows::core::Interface::vtable(self).Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Notes(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Notes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCBuddy, ::windows::core::IUnknown, IRTCPresenceContact);
impl ::core::cmp::PartialEq for IRTCBuddy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCBuddy {}
impl ::core::fmt::Debug for IRTCBuddy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddy {
    type Vtable = IRTCBuddy_Vtbl;
}
impl ::core::clone::Clone for IRTCBuddy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCBuddy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcb136c8_7b90_4e0c_befe_56edf0ba6f1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCBuddy2(::windows::core::IUnknown);
impl IRTCBuddy2 {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.PresentityURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPresentityURI)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Data)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetData)(::windows::core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.Persistent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPersistent)(::windows::core::Interface::as_raw(self), fpersistent.into_param().abi()).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = ::windows::core::zeroed::<RTC_PRESENCE_STATUS>();
        (::windows::core::Interface::vtable(self).base__.Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Notes(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Notes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile2>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> ::windows::core::Result<IRTCEnumGroups> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumGroups>();
        (::windows::core::Interface::vtable(self).EnumerateGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Groups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_PresenceProperty)(::windows::core::Interface::as_raw(self), enproperty, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumeratePresenceDevices(&self) -> ::windows::core::Result<IRTCEnumPresenceDevices> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumPresenceDevices>();
        (::windows::core::Interface::vtable(self).EnumeratePresenceDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PresenceDevices(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).PresenceDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SubscriptionType(&self) -> ::windows::core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_BUDDY_SUBSCRIPTION_TYPE>();
        (::windows::core::Interface::vtable(self).SubscriptionType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCBuddy2, ::windows::core::IUnknown, IRTCPresenceContact, IRTCBuddy);
impl ::core::cmp::PartialEq for IRTCBuddy2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCBuddy2 {}
impl ::core::fmt::Debug for IRTCBuddy2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddy2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddy2 {
    type Vtable = IRTCBuddy2_Vtbl;
}
impl ::core::clone::Clone for IRTCBuddy2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCBuddy2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x102f9588_23e7_40e3_954d_cd7a1d5c0361);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy2_Vtbl {
    pub base__: IRTCBuddy_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub EnumeratePresenceDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PresenceDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PresenceDevices: usize,
    pub SubscriptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCBuddyEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEvent {
    pub unsafe fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy>();
        (::windows::core::Interface::vtable(self).Buddy)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCBuddyEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCBuddyEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCBuddyEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCBuddyEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCBuddyEvent {
    type Vtable = IRTCBuddyEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCBuddyEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCBuddyEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf36d755d_17e6_404e_954f_0fc07574c78d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Buddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCBuddyEvent2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEvent2 {
    pub unsafe fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy>();
        (::windows::core::Interface::vtable(self).base__.Buddy)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_BUDDY_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_BUDDY_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCBuddyEvent2, ::windows::core::IUnknown, super::Com::IDispatch, IRTCBuddyEvent);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCBuddyEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCBuddyEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCBuddyEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCBuddyEvent2 {
    type Vtable = IRTCBuddyEvent2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCBuddyEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCBuddyEvent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x484a7f1e_73f0_4990_bfc2_60bc3978a720);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent2_Vtbl {
    pub base__: IRTCBuddyEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCBuddyGroup(::windows::core::IUnknown);
impl IRTCBuddyGroup {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrgroupname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi()).ok()
    }
    pub unsafe fn AddBuddy<P0>(&self, pbuddy: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCBuddy>,
    {
        (::windows::core::Interface::vtable(self).AddBuddy)(::windows::core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCBuddy>,
    {
        (::windows::core::Interface::vtable(self).RemoveBuddy)(::windows::core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumBuddies>();
        (::windows::core::Interface::vtable(self).EnumerateBuddies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Buddies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Data)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile2>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCBuddyGroup, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCBuddyGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCBuddyGroup {}
impl ::core::fmt::Debug for IRTCBuddyGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddyGroup {
    type Vtable = IRTCBuddyGroup_Vtbl;
}
impl ::core::clone::Clone for IRTCBuddyGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCBuddyGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60361e68_9164_4389_a4c6_d0b3925bda5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub AddBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateBuddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCBuddyGroupEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyGroupEvent {
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_GROUP_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_GROUP_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Group(&self) -> ::windows::core::Result<IRTCBuddyGroup> {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddyGroup>();
        (::windows::core::Interface::vtable(self).Group)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy2> {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy2>();
        (::windows::core::Interface::vtable(self).Buddy)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCBuddyGroupEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCBuddyGroupEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCBuddyGroupEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCBuddyGroupEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCBuddyGroupEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCBuddyGroupEvent {
    type Vtable = IRTCBuddyGroupEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCBuddyGroupEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCBuddyGroupEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a79e1d1_b736_4414_96f8_bbc7f08863e4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroupEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Buddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCClient(::windows::core::IUnknown);
impl IRTCClient {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareForShutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventFilter)(::windows::core::Interface::as_raw(self), lfilter).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).EventFilter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredMediaTypes<P0>(&self, lmediatypes: i32, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPreferredMediaTypes)(::windows::core::Interface::as_raw(self), lmediatypes, fpersistent.into_param().abi()).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).PreferredMediaTypes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MediaCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSession<P0, P1>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: P0, pprofile: P1, lflags: i32) -> ::windows::core::Result<IRTCSession>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).CreateSession)(::windows::core::Interface::as_raw(self), entype, bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetListenForIncomingSessions)(::windows::core::Interface::as_raw(self), enlisten).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> ::windows::core::Result<RTC_LISTEN_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_LISTEN_MODE>();
        (::windows::core::Interface::vtable(self).ListenForIncomingSessions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_NetworkAddresses<P0, P1>(&self, ftcp: P0, fexternal: P1) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_NetworkAddresses)(::windows::core::Interface::as_raw(self), ftcp.into_param().abi(), fexternal.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_Volume)(::windows::core::Interface::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).get_Volume)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_AudioMuted<P0>(&self, endevice: RTC_AUDIO_DEVICE, fmuted: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).put_AudioMuted)(::windows::core::Interface::as_raw(self), endevice, fmuted.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).get_AudioMuted)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DirectShow\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub unsafe fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow> {
        let mut result__ = ::windows::core::zeroed::<super::super::Media::DirectShow::IVideoWindow>();
        (::windows::core::Interface::vtable(self).get_IVideoWindow)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_PreferredAudioDevice<P0>(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).put_PreferredAudioDevice)(::windows::core::Interface::as_raw(self), endevice, bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_PreferredAudioDevice)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_PreferredVolume)(::windows::core::Interface::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).get_PreferredVolume)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredAEC<P0>(&self, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPreferredAEC)(::windows::core::Interface::as_raw(self), benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreferredAEC(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).PreferredAEC)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPreferredVideoDevice<P0>(&self, bstrdevicename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPreferredVideoDevice)(::windows::core::Interface::as_raw(self), bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn PreferredVideoDevice(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).PreferredVideoDevice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ActiveMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).ActiveMedia)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxBitrate)(::windows::core::Interface::as_raw(self), lmaxbitrate).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MaxBitrate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTemporalSpatialTradeOff)(::windows::core::Interface::as_raw(self), lvalue).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).TemporalSpatialTradeOff)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NetworkQuality(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).NetworkQuality)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartT120Applet)(::windows::core::Interface::as_raw(self), enapplet).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopT120Applets)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).get_IsT120AppletRunning)(::windows::core::Interface::as_raw(self), enapplet, &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LocalUserURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalUserURI<P0>(&self, bstruseruri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLocalUserURI)(::windows::core::Interface::as_raw(self), bstruseruri.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LocalUserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalUserName<P0>(&self, bstrusername: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLocalUserName)(::windows::core::Interface::as_raw(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlayRing<P0>(&self, entype: RTC_RING_TYPE, bplay: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).PlayRing)(::windows::core::Interface::as_raw(self), entype, bplay.into_param().abi()).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendDTMF)(::windows::core::Interface::as_raw(self), endtmf).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InvokeTuningWizard)(::windows::core::Interface::as_raw(self), hwndparent).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTuned(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsTuned)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClient {}
impl ::core::fmt::Debug for IRTCClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCClient {
    type Vtable = IRTCClient_Vtbl;
}
impl ::core::clone::Clone for IRTCClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07829e45_9a34_408e_a011_bddf13487cd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrepareForShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows::core::HRESULT,
    pub EventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPreferredMediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPreferredMediaTypes: usize,
    pub PreferredMediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub MediaCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetListenForIncomingSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    pub ListenForIncomingSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_NetworkAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL, pvaddresses: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_NetworkAddresses: usize,
    pub put_Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT,
    pub get_Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub put_AudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_AudioMuted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AudioMuted: usize,
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub get_IVideoWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com")))]
    get_IVideoWindow: usize,
    pub put_PreferredAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub get_PreferredAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub put_PreferredVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT,
    pub get_PreferredVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPreferredAEC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPreferredAEC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreferredAEC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreferredAEC: usize,
    pub SetPreferredVideoDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub PreferredVideoDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ActiveMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows::core::HRESULT,
    pub MaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT,
    pub SetTemporalSpatialTradeOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows::core::HRESULT,
    pub TemporalSpatialTradeOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT,
    pub NetworkQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows::core::HRESULT,
    pub StartT120Applet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT,
    pub StopT120Applets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_IsT120AppletRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_IsT120AppletRunning: usize,
    pub LocalUserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetLocalUserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruseruri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LocalUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetLocalUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PlayRing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlayRing: usize,
    pub SendDTMF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows::core::HRESULT,
    pub InvokeTuningWizard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTuned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftuned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTuned: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCClient2(::windows::core::IUnknown);
impl IRTCClient2 {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.PrepareForShutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEventFilter)(::windows::core::Interface::as_raw(self), lfilter).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.EventFilter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredMediaTypes<P0>(&self, lmediatypes: i32, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPreferredMediaTypes)(::windows::core::Interface::as_raw(self), lmediatypes, fpersistent.into_param().abi()).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.PreferredMediaTypes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.MediaCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSession<P0, P1>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: P0, pprofile: P1, lflags: i32) -> ::windows::core::Result<IRTCSession>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).base__.CreateSession)(::windows::core::Interface::as_raw(self), entype, bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetListenForIncomingSessions)(::windows::core::Interface::as_raw(self), enlisten).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> ::windows::core::Result<RTC_LISTEN_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_LISTEN_MODE>();
        (::windows::core::Interface::vtable(self).base__.ListenForIncomingSessions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_NetworkAddresses<P0, P1>(&self, ftcp: P0, fexternal: P1) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.get_NetworkAddresses)(::windows::core::Interface::as_raw(self), ftcp.into_param().abi(), fexternal.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.put_Volume)(::windows::core::Interface::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.get_Volume)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_AudioMuted<P0>(&self, endevice: RTC_AUDIO_DEVICE, fmuted: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.put_AudioMuted)(::windows::core::Interface::as_raw(self), endevice, fmuted.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.get_AudioMuted)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DirectShow\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub unsafe fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow> {
        let mut result__ = ::windows::core::zeroed::<super::super::Media::DirectShow::IVideoWindow>();
        (::windows::core::Interface::vtable(self).base__.get_IVideoWindow)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_PreferredAudioDevice<P0>(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.put_PreferredAudioDevice)(::windows::core::Interface::as_raw(self), endevice, bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.get_PreferredAudioDevice)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.put_PreferredVolume)(::windows::core::Interface::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.get_PreferredVolume)(::windows::core::Interface::as_raw(self), endevice, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredAEC<P0>(&self, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPreferredAEC)(::windows::core::Interface::as_raw(self), benable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreferredAEC(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.PreferredAEC)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPreferredVideoDevice<P0>(&self, bstrdevicename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPreferredVideoDevice)(::windows::core::Interface::as_raw(self), bstrdevicename.into_param().abi()).ok()
    }
    pub unsafe fn PreferredVideoDevice(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.PreferredVideoDevice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ActiveMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.ActiveMedia)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMaxBitrate)(::windows::core::Interface::as_raw(self), lmaxbitrate).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.MaxBitrate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetTemporalSpatialTradeOff)(::windows::core::Interface::as_raw(self), lvalue).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.TemporalSpatialTradeOff)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NetworkQuality(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.NetworkQuality)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartT120Applet)(::windows::core::Interface::as_raw(self), enapplet).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StopT120Applets)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.get_IsT120AppletRunning)(::windows::core::Interface::as_raw(self), enapplet, &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.LocalUserURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalUserURI<P0>(&self, bstruseruri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLocalUserURI)(::windows::core::Interface::as_raw(self), bstruseruri.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.LocalUserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalUserName<P0>(&self, bstrusername: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLocalUserName)(::windows::core::Interface::as_raw(self), bstrusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlayRing<P0>(&self, entype: RTC_RING_TYPE, bplay: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.PlayRing)(::windows::core::Interface::as_raw(self), entype, bplay.into_param().abi()).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SendDTMF)(::windows::core::Interface::as_raw(self), endtmf).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.InvokeTuningWizard)(::windows::core::Interface::as_raw(self), hwndparent).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTuned(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsTuned)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn put_AnswerMode(&self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_AnswerMode)(::windows::core::Interface::as_raw(self), entype, enmode).ok()
    }
    pub unsafe fn get_AnswerMode(&self, entype: RTC_SESSION_TYPE) -> ::windows::core::Result<RTC_ANSWER_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_ANSWER_MODE>();
        (::windows::core::Interface::vtable(self).get_AnswerMode)(::windows::core::Interface::as_raw(self), entype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeTuningWizardEx<P0, P1>(&self, hwndparent: isize, fallowaudio: P0, fallowvideo: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).InvokeTuningWizardEx)(::windows::core::Interface::as_raw(self), hwndparent, fallowaudio.into_param().abi(), fallowvideo.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClientName<P0>(&self, bstrclientname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetClientName)(::windows::core::Interface::as_raw(self), bstrclientname.into_param().abi()).ok()
    }
    pub unsafe fn SetClientCurVer<P0>(&self, bstrclientcurver: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetClientCurVer)(::windows::core::Interface::as_raw(self), bstrclientcurver.into_param().abi()).ok()
    }
    pub unsafe fn InitializeEx(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeEx)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn CreateSessionWithDescription<P0, P1, P2>(&self, bstrcontenttype: P0, bstrsessiondescription: P1, pprofile: P2, lflags: i32) -> ::windows::core::Result<IRTCSession2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCSession2>();
        (::windows::core::Interface::vtable(self).CreateSessionWithDescription)(::windows::core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSessionDescriptionManager<P0>(&self, psessiondescriptionmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCSessionDescriptionManager>,
    {
        (::windows::core::Interface::vtable(self).SetSessionDescriptionManager)(::windows::core::Interface::as_raw(self), psessiondescriptionmanager.into_param().abi()).ok()
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_PreferredSecurityLevel)(::windows::core::Interface::as_raw(self), ensecuritytype, ensecuritylevel).ok()
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::windows::core::zeroed::<RTC_SECURITY_LEVEL>();
        (::windows::core::Interface::vtable(self).get_PreferredSecurityLevel)(::windows::core::Interface::as_raw(self), ensecuritytype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_AllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_AllowedPorts)(::windows::core::Interface::as_raw(self), ltransport, enlistenmode).ok()
    }
    pub unsafe fn get_AllowedPorts(&self, ltransport: i32) -> ::windows::core::Result<RTC_LISTEN_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_LISTEN_MODE>();
        (::windows::core::Interface::vtable(self).get_AllowedPorts)(::windows::core::Interface::as_raw(self), ltransport, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCClient2, ::windows::core::IUnknown, IRTCClient);
impl ::core::cmp::PartialEq for IRTCClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClient2 {}
impl ::core::fmt::Debug for IRTCClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClient2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCClient2 {
    type Vtable = IRTCClient2_Vtbl;
}
impl ::core::clone::Clone for IRTCClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCClient2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c91d71d_1064_42da_bfa5_572beb8eea84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClient2_Vtbl {
    pub base__: IRTCClient_Vtbl,
    pub put_AnswerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::HRESULT,
    pub get_AnswerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InvokeTuningWizardEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvokeTuningWizardEx: usize,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows::core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclientname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetClientCurVer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclientcurver: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub InitializeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub CreateSessionWithDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSessionDescriptionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub put_AllowedPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    pub get_AllowedPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCClientEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientEvent {
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_CLIENT_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_CLIENT_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Client(&self) -> ::windows::core::Result<IRTCClient> {
        let mut result__ = ::windows::core::zeroed::<IRTCClient>();
        (::windows::core::Interface::vtable(self).Client)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCClientEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCClientEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCClientEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCClientEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCClientEvent {
    type Vtable = IRTCClientEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCClientEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCClientEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b493b7a_3cba_4170_9c8b_76a9dacdd644);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub Client: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCClientPortManagement(::windows::core::IUnknown);
impl IRTCClientPortManagement {
    pub unsafe fn StartListenAddressAndPort<P0>(&self, bstrinternallocaladdress: P0, linternallocalport: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).StartListenAddressAndPort)(::windows::core::Interface::as_raw(self), bstrinternallocaladdress.into_param().abi(), linternallocalport).ok()
    }
    pub unsafe fn StopListenAddressAndPort<P0>(&self, bstrinternallocaladdress: P0, linternallocalport: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).StopListenAddressAndPort)(::windows::core::Interface::as_raw(self), bstrinternallocaladdress.into_param().abi(), linternallocalport).ok()
    }
    pub unsafe fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPortRange)(::windows::core::Interface::as_raw(self), enporttype, plminvalue, plmaxvalue).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCClientPortManagement, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCClientPortManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientPortManagement {}
impl ::core::fmt::Debug for IRTCClientPortManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientPortManagement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientPortManagement {
    type Vtable = IRTCClientPortManagement_Vtbl;
}
impl ::core::clone::Clone for IRTCClientPortManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCClientPortManagement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5df3f03_4bde_4417_aefe_71177bdaea66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPortManagement_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartListenAddressAndPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT,
    pub StopListenAddressAndPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT,
    pub GetPortRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCClientPresence(::windows::core::IUnknown);
impl IRTCClientPresence {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnablePresence<P0>(&self, fusestorage: P0, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnablePresence)(::windows::core::Interface::as_raw(self), fusestorage.into_param().abi(), ::core::mem::transmute(varstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Export(&self, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Export)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<P0>(&self, varstorage: super::Com::VARIANT, freplaceall: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).Import)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varstorage), freplaceall.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumBuddies>();
        (::windows::core::Interface::vtable(self).EnumerateBuddies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Buddies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Buddy<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<IRTCBuddy>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy>();
        (::windows::core::Interface::vtable(self).get_Buddy)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddBuddy<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fpersistent: P3, pprofile: P4, lflags: i32) -> ::windows::core::Result<IRTCBuddy>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy>();
        (::windows::core::Interface::vtable(self).AddBuddy)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), fpersistent.into_param().abi(), pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCBuddy>,
    {
        (::windows::core::Interface::vtable(self).RemoveBuddy)(::windows::core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> ::windows::core::Result<IRTCEnumWatchers> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumWatchers>();
        (::windows::core::Interface::vtable(self).EnumerateWatchers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Watchers(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Watchers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Watcher<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<IRTCWatcher>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher>();
        (::windows::core::Interface::vtable(self).get_Watcher)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddWatcher<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fblocked: P3, fpersistent: P4) -> ::windows::core::Result<IRTCWatcher>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher>();
        (::windows::core::Interface::vtable(self).AddWatcher)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), fblocked.into_param().abi(), fpersistent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveWatcher<P0>(&self, pwatcher: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCWatcher>,
    {
        (::windows::core::Interface::vtable(self).RemoveWatcher)(::windows::core::Interface::as_raw(self), pwatcher.into_param().abi()).ok()
    }
    pub unsafe fn SetLocalPresenceInfo<P0>(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLocalPresenceInfo)(::windows::core::Interface::as_raw(self), enstatus, bstrnotes.into_param().abi()).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_OFFER_WATCHER_MODE>();
        (::windows::core::Interface::vtable(self).OfferWatcherMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOfferWatcherMode)(::windows::core::Interface::as_raw(self), enmode).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> ::windows::core::Result<RTC_PRIVACY_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_PRIVACY_MODE>();
        (::windows::core::Interface::vtable(self).PrivacyMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivacyMode)(::windows::core::Interface::as_raw(self), enmode).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCClientPresence, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCClientPresence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientPresence {}
impl ::core::fmt::Debug for IRTCClientPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientPresence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientPresence {
    type Vtable = IRTCClientPresence_Vtbl;
}
impl ::core::clone::Clone for IRTCClientPresence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCClientPresence {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c3cbcc_0744_42d1_968a_51aa1bb274c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnablePresence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnablePresence: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Export: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varstorage: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Export: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varstorage: super::Com::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Import: usize,
    pub EnumerateBuddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    pub get_Buddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddBuddy: usize,
    pub RemoveBuddy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateWatchers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Watchers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Watchers: usize,
    pub get_Watcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddWatcher: usize,
    pub RemoveWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwatcher: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalPresenceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub OfferWatcherMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT,
    pub SetOfferWatcherMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT,
    pub PrivacyMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT,
    pub SetPrivacyMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCClientPresence2(::windows::core::IUnknown);
impl IRTCClientPresence2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnablePresence<P0>(&self, fusestorage: P0, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.EnablePresence)(::windows::core::Interface::as_raw(self), fusestorage.into_param().abi(), ::core::mem::transmute(varstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Export(&self, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Export)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<P0>(&self, varstorage: super::Com::VARIANT, freplaceall: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Import)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varstorage), freplaceall.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumBuddies>();
        (::windows::core::Interface::vtable(self).base__.EnumerateBuddies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).base__.Buddies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Buddy<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<IRTCBuddy>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy>();
        (::windows::core::Interface::vtable(self).base__.get_Buddy)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddBuddy<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fpersistent: P3, pprofile: P4, lflags: i32) -> ::windows::core::Result<IRTCBuddy>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy>();
        (::windows::core::Interface::vtable(self).base__.AddBuddy)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), fpersistent.into_param().abi(), pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCBuddy>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveBuddy)(::windows::core::Interface::as_raw(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> ::windows::core::Result<IRTCEnumWatchers> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumWatchers>();
        (::windows::core::Interface::vtable(self).base__.EnumerateWatchers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Watchers(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).base__.Watchers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Watcher<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<IRTCWatcher>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher>();
        (::windows::core::Interface::vtable(self).base__.get_Watcher)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddWatcher<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fblocked: P3, fpersistent: P4) -> ::windows::core::Result<IRTCWatcher>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher>();
        (::windows::core::Interface::vtable(self).base__.AddWatcher)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), fblocked.into_param().abi(), fpersistent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveWatcher<P0>(&self, pwatcher: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCWatcher>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveWatcher)(::windows::core::Interface::as_raw(self), pwatcher.into_param().abi()).ok()
    }
    pub unsafe fn SetLocalPresenceInfo<P0>(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLocalPresenceInfo)(::windows::core::Interface::as_raw(self), enstatus, bstrnotes.into_param().abi()).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_OFFER_WATCHER_MODE>();
        (::windows::core::Interface::vtable(self).base__.OfferWatcherMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetOfferWatcherMode)(::windows::core::Interface::as_raw(self), enmode).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> ::windows::core::Result<RTC_PRIVACY_MODE> {
        let mut result__ = ::windows::core::zeroed::<RTC_PRIVACY_MODE>();
        (::windows::core::Interface::vtable(self).base__.PrivacyMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivacyMode)(::windows::core::Interface::as_raw(self), enmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnablePresenceEx<P0>(&self, pprofile: P0, varstorage: super::Com::VARIANT, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).EnablePresenceEx)(::windows::core::Interface::as_raw(self), pprofile.into_param().abi(), ::core::mem::transmute(varstorage), lflags).ok()
    }
    pub unsafe fn DisablePresence(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisablePresence)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddGroup<P0, P1, P2>(&self, bstrgroupname: P0, bstrdata: P1, pprofile: P2, lflags: i32) -> ::windows::core::Result<IRTCBuddyGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddyGroup>();
        (::windows::core::Interface::vtable(self).AddGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), bstrdata.into_param().abi(), pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveGroup<P0>(&self, pgroup: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCBuddyGroup>,
    {
        (::windows::core::Interface::vtable(self).RemoveGroup)(::windows::core::Interface::as_raw(self), pgroup.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> ::windows::core::Result<IRTCEnumGroups> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumGroups>();
        (::windows::core::Interface::vtable(self).EnumerateGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Groups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Group<P0>(&self, bstrgroupname: P0) -> ::windows::core::Result<IRTCBuddyGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddyGroup>();
        (::windows::core::Interface::vtable(self).get_Group)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddWatcherEx<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, enstate: RTC_WATCHER_STATE, fpersistent: P3, enscope: RTC_ACE_SCOPE, pprofile: P4, lflags: i32) -> ::windows::core::Result<IRTCWatcher2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher2>();
        (::windows::core::Interface::vtable(self).AddWatcherEx)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), enstate, fpersistent.into_param().abi(), enscope, pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn get_WatcherEx<P0>(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: P0) -> ::windows::core::Result<IRTCWatcher2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher2>();
        (::windows::core::Interface::vtable(self).get_WatcherEx)(::windows::core::Interface::as_raw(self), enmode, bstrpresentityuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn put_PresenceProperty<P0>(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).put_PresenceProperty)(::windows::core::Interface::as_raw(self), enproperty, bstrproperty.into_param().abi()).ok()
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_PresenceProperty)(::windows::core::Interface::as_raw(self), enproperty, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPresenceData<P0, P1>(&self, bstrnamespace: P0, bstrdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPresenceData)(::windows::core::Interface::as_raw(self), bstrnamespace.into_param().abi(), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut ::windows::core::BSTR, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPresenceData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocalPresenceInfo)(::windows::core::Interface::as_raw(self), penstatus, ::core::mem::transmute(pbstrnotes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddBuddyEx<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fpersistent: P3, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: P4, lflags: i32) -> ::windows::core::Result<IRTCBuddy2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<IRTCProfile>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCBuddy2>();
        (::windows::core::Interface::vtable(self).AddBuddyEx)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), fpersistent.into_param().abi(), ensubscriptiontype, pprofile.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCClientPresence2, ::windows::core::IUnknown, IRTCClientPresence);
impl ::core::cmp::PartialEq for IRTCClientPresence2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientPresence2 {}
impl ::core::fmt::Debug for IRTCClientPresence2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientPresence2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientPresence2 {
    type Vtable = IRTCClientPresence2_Vtbl;
}
impl ::core::clone::Clone for IRTCClientPresence2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCClientPresence2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad1809e8_62f7_4783_909a_29c9d2cb1d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence2_Vtbl {
    pub base__: IRTCClientPresence_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnablePresenceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, varstorage: super::Com::VARIANT, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnablePresenceEx: usize,
    pub DisablePresence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub get_Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddWatcherEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddWatcherEx: usize,
    pub get_WatcherEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub put_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnamespace: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddBuddyEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddBuddyEx: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCClientProvisioning(::windows::core::IUnknown);
impl IRTCClientProvisioning {
    pub unsafe fn CreateProfile<P0>(&self, bstrprofilexml: P0) -> ::windows::core::Result<IRTCProfile>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile>();
        (::windows::core::Interface::vtable(self).CreateProfile)(::windows::core::Interface::as_raw(self), bstrprofilexml.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnableProfile<P0>(&self, pprofile: P0, lregisterflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).EnableProfile)(::windows::core::Interface::as_raw(self), pprofile.into_param().abi(), lregisterflags).ok()
    }
    pub unsafe fn DisableProfile<P0>(&self, pprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).DisableProfile)(::windows::core::Interface::as_raw(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> ::windows::core::Result<IRTCEnumProfiles> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumProfiles>();
        (::windows::core::Interface::vtable(self).EnumerateProfiles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Profiles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProfile<P0, P1, P2, P3>(&self, bstruseraccount: P0, bstruserpassword: P1, bstruseruri: P2, bstrserver: P3, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetProfile)(::windows::core::Interface::as_raw(self), bstruseraccount.into_param().abi(), bstruserpassword.into_param().abi(), bstruseruri.into_param().abi(), bstrserver.into_param().abi(), ltransport, lcookie).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SessionCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCClientProvisioning, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCClientProvisioning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientProvisioning {}
impl ::core::fmt::Debug for IRTCClientProvisioning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientProvisioning").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientProvisioning {
    type Vtable = IRTCClientProvisioning_Vtbl;
}
impl ::core::clone::Clone for IRTCClientProvisioning {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCClientProvisioning {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f5cf06_65b9_4a80_a0e6_73cae3ef3822);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprofilexml: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32) -> ::windows::core::HRESULT,
    pub DisableProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Profiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Profiles: usize,
    pub GetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruseraccount: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstruserpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstruseruri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrserver: ::std::mem::MaybeUninit<::windows::core::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCClientProvisioning2(::windows::core::IUnknown);
impl IRTCClientProvisioning2 {
    pub unsafe fn CreateProfile<P0>(&self, bstrprofilexml: P0) -> ::windows::core::Result<IRTCProfile>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile>();
        (::windows::core::Interface::vtable(self).base__.CreateProfile)(::windows::core::Interface::as_raw(self), bstrprofilexml.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnableProfile<P0>(&self, pprofile: P0, lregisterflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).base__.EnableProfile)(::windows::core::Interface::as_raw(self), pprofile.into_param().abi(), lregisterflags).ok()
    }
    pub unsafe fn DisableProfile<P0>(&self, pprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).base__.DisableProfile)(::windows::core::Interface::as_raw(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> ::windows::core::Result<IRTCEnumProfiles> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumProfiles>();
        (::windows::core::Interface::vtable(self).base__.EnumerateProfiles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).base__.Profiles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProfile<P0, P1, P2, P3>(&self, bstruseraccount: P0, bstruserpassword: P1, bstruseruri: P2, bstrserver: P3, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetProfile)(::windows::core::Interface::as_raw(self), bstruseraccount.into_param().abi(), bstruserpassword.into_param().abi(), bstruseruri.into_param().abi(), bstrserver.into_param().abi(), ltransport, lcookie).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.SessionCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnableProfileEx<P0>(&self, pprofile: P0, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).EnableProfileEx)(::windows::core::Interface::as_raw(self), pprofile.into_param().abi(), lregisterflags, lroamingflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCClientProvisioning2, ::windows::core::IUnknown, IRTCClientProvisioning);
impl ::core::cmp::PartialEq for IRTCClientProvisioning2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCClientProvisioning2 {}
impl ::core::fmt::Debug for IRTCClientProvisioning2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCClientProvisioning2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientProvisioning2 {
    type Vtable = IRTCClientProvisioning2_Vtbl;
}
impl ::core::clone::Clone for IRTCClientProvisioning2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCClientProvisioning2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa70909b5_f40e_4587_bb75_e6bc0845023e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning2_Vtbl {
    pub base__: IRTCClientProvisioning_Vtbl,
    pub EnableProfileEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCCollection, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCCollection {
    type Vtable = IRTCCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec7c8096_b918_4044_94f1_e4fba0361d5c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCDispatchEventNotification(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCDispatchEventNotification, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCDispatchEventNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCDispatchEventNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCDispatchEventNotification").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCDispatchEventNotification {
    type Vtable = IRTCDispatchEventNotification_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCDispatchEventNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCDispatchEventNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x176ddfbe_fec0_4d55_bc87_84cff1ef7f91);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCDispatchEventNotification_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEnumBuddies(::windows::core::IUnknown);
impl IRTCEnumBuddies {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCBuddy>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumBuddies>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCEnumBuddies, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEnumBuddies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumBuddies {}
impl ::core::fmt::Debug for IRTCEnumBuddies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumBuddies").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumBuddies {
    type Vtable = IRTCEnumBuddies_Vtbl;
}
impl ::core::clone::Clone for IRTCEnumBuddies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEnumBuddies {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7296917_5569_4b3b_b3af_98d1144b2b87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumBuddies_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEnumGroups(::windows::core::IUnknown);
impl IRTCEnumGroups {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCBuddyGroup>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumGroups> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumGroups>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCEnumGroups, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEnumGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumGroups {}
impl ::core::fmt::Debug for IRTCEnumGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumGroups").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumGroups {
    type Vtable = IRTCEnumGroups_Vtbl;
}
impl ::core::clone::Clone for IRTCEnumGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEnumGroups {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x742378d6_a141_4415_8f27_35d99076cf5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumGroups_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEnumParticipants(::windows::core::IUnknown);
impl IRTCEnumParticipants {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCParticipant>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumParticipants> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumParticipants>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCEnumParticipants, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEnumParticipants {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumParticipants {}
impl ::core::fmt::Debug for IRTCEnumParticipants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumParticipants").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumParticipants {
    type Vtable = IRTCEnumParticipants_Vtbl;
}
impl ::core::clone::Clone for IRTCEnumParticipants {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEnumParticipants {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd56f29_4a4f_41b2_ba5c_f5bccc060bf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumParticipants_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEnumPresenceDevices(::windows::core::IUnknown);
impl IRTCEnumPresenceDevices {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCPresenceDevice>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumPresenceDevices> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumPresenceDevices>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCEnumPresenceDevices, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEnumPresenceDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumPresenceDevices {}
impl ::core::fmt::Debug for IRTCEnumPresenceDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumPresenceDevices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumPresenceDevices {
    type Vtable = IRTCEnumPresenceDevices_Vtbl;
}
impl ::core::clone::Clone for IRTCEnumPresenceDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEnumPresenceDevices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x708c2ab7_8bf8_42f8_8c7d_635197ad5539);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumPresenceDevices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEnumProfiles(::windows::core::IUnknown);
impl IRTCEnumProfiles {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCProfile>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumProfiles> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumProfiles>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCEnumProfiles, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEnumProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumProfiles {}
impl ::core::fmt::Debug for IRTCEnumProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumProfiles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumProfiles {
    type Vtable = IRTCEnumProfiles_Vtbl;
}
impl ::core::clone::Clone for IRTCEnumProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEnumProfiles {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29b7c41c_ed82_4bca_84ad_39d5101b58e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumProfiles_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEnumUserSearchResults(::windows::core::IUnknown);
impl IRTCEnumUserSearchResults {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCUserSearchResult>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumUserSearchResults> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumUserSearchResults>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCEnumUserSearchResults, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEnumUserSearchResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumUserSearchResults {}
impl ::core::fmt::Debug for IRTCEnumUserSearchResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumUserSearchResults").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumUserSearchResults {
    type Vtable = IRTCEnumUserSearchResults_Vtbl;
}
impl ::core::clone::Clone for IRTCEnumUserSearchResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEnumUserSearchResults {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d4d877_aa5d_4a5b_8d0e_002a8067e0e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumUserSearchResults_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEnumWatchers(::windows::core::IUnknown);
impl IRTCEnumWatchers {
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<IRTCWatcher>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumWatchers> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumWatchers>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCEnumWatchers, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEnumWatchers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEnumWatchers {}
impl ::core::fmt::Debug for IRTCEnumWatchers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEnumWatchers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumWatchers {
    type Vtable = IRTCEnumWatchers_Vtbl;
}
impl ::core::clone::Clone for IRTCEnumWatchers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEnumWatchers {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87d55d7_db74_4ed1_9ca4_77a0e41b413e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumWatchers_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCEventNotification(::windows::core::IUnknown);
impl IRTCEventNotification {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Event<P0>(&self, rtcevent: RTC_EVENT, pevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).Event)(::windows::core::Interface::as_raw(self), rtcevent, pevent.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCEventNotification, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCEventNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCEventNotification {}
impl ::core::fmt::Debug for IRTCEventNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCEventNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCEventNotification {
    type Vtable = IRTCEventNotification_Vtbl;
}
impl ::core::clone::Clone for IRTCEventNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCEventNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13fa24c7_5748_4b21_91f5_7397609ce747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEventNotification_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Event: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCInfoEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCInfoEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession2>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__ = ::windows::core::zeroed::<IRTCParticipant>();
        (::windows::core::Interface::vtable(self).Participant)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Info(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Info)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InfoHeader(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).InfoHeader)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCInfoEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCInfoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCInfoEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCInfoEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCInfoEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCInfoEvent {
    type Vtable = IRTCInfoEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCInfoEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCInfoEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e1d68ae_1912_4f49_b2c3_594fadfd425f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCInfoEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinfo: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub InfoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCIntensityEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCIntensityEvent {
    pub unsafe fn Level(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Level)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Min(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Min)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Max(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Max)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<RTC_AUDIO_DEVICE> {
        let mut result__ = ::windows::core::zeroed::<RTC_AUDIO_DEVICE>();
        (::windows::core::Interface::vtable(self).Direction)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCIntensityEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCIntensityEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCIntensityEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCIntensityEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCIntensityEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCIntensityEvent {
    type Vtable = IRTCIntensityEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCIntensityEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCIntensityEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c23bf51_390c_4992_a41d_41eec05b2a4b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCIntensityEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCMediaEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaEvent {
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MediaType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_MEDIA_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_MEDIA_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventReason(&self) -> ::windows::core::Result<RTC_MEDIA_EVENT_REASON> {
        let mut result__ = ::windows::core::zeroed::<RTC_MEDIA_EVENT_REASON>();
        (::windows::core::Interface::vtable(self).EventReason)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCMediaEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCMediaEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCMediaEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCMediaEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCMediaEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCMediaEvent {
    type Vtable = IRTCMediaEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCMediaEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCMediaEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x099944fb_bcda_453e_8c41_e13da2adf7f3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMediaEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows::core::HRESULT,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub EventReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCMediaRequestEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaRequestEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession2>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProposedMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).ProposedMedia)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).CurrentMedia)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Accept(&self, lmediatypes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Accept)(::windows::core::Interface::as_raw(self), lmediatypes).ok()
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::windows::core::zeroed::<RTC_SECURITY_LEVEL>();
        (::windows::core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(::windows::core::Interface::as_raw(self), ensecuritytype, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reject)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REINVITE_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_REINVITE_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCMediaRequestEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCMediaRequestEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCMediaRequestEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCMediaRequestEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCMediaRequestEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCMediaRequestEvent {
    type Vtable = IRTCMediaRequestEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCMediaRequestEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCMediaRequestEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52572d15_148c_4d97_a36c_2da55c289d63);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMediaRequestEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProposedMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows::core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCMessagingEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCMessagingEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__ = ::windows::core::zeroed::<IRTCParticipant>();
        (::windows::core::Interface::vtable(self).Participant)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_MESSAGING_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_MESSAGING_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Message(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Message)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MessageHeader(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).MessageHeader)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserStatus(&self) -> ::windows::core::Result<RTC_MESSAGING_USER_STATUS> {
        let mut result__ = ::windows::core::zeroed::<RTC_MESSAGING_USER_STATUS>();
        (::windows::core::Interface::vtable(self).UserStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCMessagingEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCMessagingEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCMessagingEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCMessagingEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCMessagingEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCMessagingEvent {
    type Vtable = IRTCMessagingEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCMessagingEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCMessagingEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3609541_1b29_4de5_a4ad_5aebaf319512);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMessagingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub MessageHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCParticipant(::windows::core::IUnknown);
impl IRTCParticipant {
    pub unsafe fn UserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Removable(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).Removable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_PARTICIPANT_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCParticipant, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCParticipant {}
impl ::core::fmt::Debug for IRTCParticipant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCParticipant").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCParticipant {
    type Vtable = IRTCParticipant_Vtbl;
}
impl ::core::clone::Clone for IRTCParticipant {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCParticipant {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae86add5_26b1_4414_af1d_b94cd938d739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipant_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub UserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Removable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfremovable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Removable: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCParticipantStateChangeEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCParticipantStateChangeEvent {
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__ = ::windows::core::zeroed::<IRTCParticipant>();
        (::windows::core::Interface::vtable(self).Participant)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_PARTICIPANT_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCParticipantStateChangeEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCParticipantStateChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCParticipantStateChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCParticipantStateChangeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCParticipantStateChangeEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCParticipantStateChangeEvent {
    type Vtable = IRTCParticipantStateChangeEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCParticipantStateChangeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCParticipantStateChangeEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09bcb597_f0fa_48f9_b420_468cea7fde04);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipantStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCPortManager(::windows::core::IUnknown);
impl IRTCPortManager {
    pub unsafe fn GetMapping<P0>(&self, bstrremoteaddress: P0, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::windows::core::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::windows::core::BSTR, plexternallocalport: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMapping)(::windows::core::Interface::as_raw(self), bstrremoteaddress.into_param().abi(), enporttype, ::core::mem::transmute(pbstrinternallocaladdress), plinternallocalport, ::core::mem::transmute(pbstrexternallocaladdress), plexternallocalport).ok()
    }
    pub unsafe fn UpdateRemoteAddress<P0, P1, P2>(&self, bstrremoteaddress: P0, bstrinternallocaladdress: P1, linternallocalport: i32, bstrexternallocaladdress: P2, lexternallocalport: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).UpdateRemoteAddress)(::windows::core::Interface::as_raw(self), bstrremoteaddress.into_param().abi(), bstrinternallocaladdress.into_param().abi(), linternallocalport, bstrexternallocaladdress.into_param().abi(), lexternallocalport).ok()
    }
    pub unsafe fn ReleaseMapping<P0, P1>(&self, bstrinternallocaladdress: P0, linternallocalport: i32, bstrexternallocaladdress: P1, lexternallocaladdress: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ReleaseMapping)(::windows::core::Interface::as_raw(self), bstrinternallocaladdress.into_param().abi(), linternallocalport, bstrexternallocaladdress.into_param().abi(), lexternallocaladdress).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCPortManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCPortManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCPortManager {}
impl ::core::fmt::Debug for IRTCPortManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPortManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCPortManager {
    type Vtable = IRTCPortManager_Vtbl;
}
impl ::core::clone::Clone for IRTCPortManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCPortManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda77c14b_6208_43ca_8ddf_5b60a0a69fac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPortManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, plexternallocalport: *mut i32) -> ::windows::core::HRESULT,
    pub UpdateRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, lexternallocalport: i32) -> ::windows::core::HRESULT,
    pub ReleaseMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, lexternallocaladdress: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCPresenceContact(::windows::core::IUnknown);
impl IRTCPresenceContact {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).PresentityURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPresentityURI)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Data)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).Persistent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPersistent)(::windows::core::Interface::as_raw(self), fpersistent.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCPresenceContact, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCPresenceContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCPresenceContact {}
impl ::core::fmt::Debug for IRTCPresenceContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceContact").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCPresenceContact {
    type Vtable = IRTCPresenceContact_Vtbl;
}
impl ::core::clone::Clone for IRTCPresenceContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCPresenceContact {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b22f92c_cd90_42db_a733_212205c3e3df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceContact_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub PresentityURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPresentityURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Persistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfpersistent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Persistent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPersistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPersistent: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCPresenceDataEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceDataEvent {
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut ::windows::core::BSTR, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPresenceData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCPresenceDataEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCPresenceDataEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCPresenceDataEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCPresenceDataEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceDataEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCPresenceDataEvent {
    type Vtable = IRTCPresenceDataEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCPresenceDataEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCPresenceDataEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38f0e78c_8b87_4c04_a82d_aedd83c909bb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDataEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCPresenceDevice(::windows::core::IUnknown);
impl IRTCPresenceDevice {
    pub unsafe fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = ::windows::core::zeroed::<RTC_PRESENCE_STATUS>();
        (::windows::core::Interface::vtable(self).Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Notes(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Notes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_PresenceProperty)(::windows::core::Interface::as_raw(self), enproperty, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut ::windows::core::BSTR, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPresenceData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCPresenceDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCPresenceDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCPresenceDevice {}
impl ::core::fmt::Debug for IRTCPresenceDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCPresenceDevice {
    type Vtable = IRTCPresenceDevice_Vtbl;
}
impl ::core::clone::Clone for IRTCPresenceDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCPresenceDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc6a90dd_ad9a_48da_9b0c_2515e38521ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCPresencePropertyEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresencePropertyEvent {
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PresenceProperty(&self) -> ::windows::core::Result<RTC_PRESENCE_PROPERTY> {
        let mut result__ = ::windows::core::zeroed::<RTC_PRESENCE_PROPERTY>();
        (::windows::core::Interface::vtable(self).PresenceProperty)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCPresencePropertyEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCPresencePropertyEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCPresencePropertyEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCPresencePropertyEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresencePropertyEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCPresencePropertyEvent {
    type Vtable = IRTCPresencePropertyEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCPresencePropertyEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCPresencePropertyEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf777f570_a820_49d5_86bd_e099493f1518);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresencePropertyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub PresenceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCPresenceStatusEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceStatusEvent {
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocalPresenceInfo)(::windows::core::Interface::as_raw(self), penstatus, ::core::mem::transmute(pbstrnotes)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCPresenceStatusEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCPresenceStatusEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCPresenceStatusEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCPresenceStatusEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCPresenceStatusEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCPresenceStatusEvent {
    type Vtable = IRTCPresenceStatusEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCPresenceStatusEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCPresenceStatusEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78673f32_4a0f_462c_89aa_ee7706707678);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCProfile(::windows::core::IUnknown);
impl IRTCProfile {
    pub unsafe fn Key(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Key)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn XML(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).XML)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ProviderName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_ProviderURI)(::windows::core::Interface::as_raw(self), enuri, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProviderData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ProviderData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ClientName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientBanner(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).ClientBanner)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientMinVer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ClientMinVer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientCurVer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ClientCurVer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientUpdateURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ClientUpdateURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ClientData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserAccount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCredentials<P0, P1, P2>(&self, bstruseruri: P0, bstruseraccount: P1, bstrpassword: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCredentials)(::windows::core::Interface::as_raw(self), bstruseruri.into_param().abi(), bstruseraccount.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SessionCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_REGISTRATION_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCProfile, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCProfile {}
impl ::core::fmt::Debug for IRTCProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCProfile {
    type Vtable = IRTCProfile_Vtbl;
}
impl ::core::clone::Clone for IRTCProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCProfile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd07eca9e_4062_4dd4_9e7d_722a49ba7303);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrkey: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub XML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub get_ProviderURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ProviderData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientBanner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfbanner: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientBanner: usize,
    pub ClientMinVer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrminver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ClientCurVer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcurver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ClientUpdateURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ClientData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruseruri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstruseraccount: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCProfile2(::windows::core::IUnknown);
impl IRTCProfile2 {
    pub unsafe fn Key(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Key)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn XML(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.XML)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ProviderName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.get_ProviderURI)(::windows::core::Interface::as_raw(self), enuri, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProviderData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ProviderData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ClientName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientBanner(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.ClientBanner)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientMinVer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ClientMinVer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientCurVer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ClientCurVer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientUpdateURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ClientUpdateURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClientData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ClientData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserAccount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCredentials<P0, P1, P2>(&self, bstruseruri: P0, bstruseraccount: P1, bstrpassword: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetCredentials)(::windows::core::Interface::as_raw(self), bstruseruri.into_param().abi(), bstruseraccount.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.SessionCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_REGISTRATION_STATE>();
        (::windows::core::Interface::vtable(self).base__.State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Realm(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Realm)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRealm<P0>(&self, bstrrealm: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRealm)(::windows::core::Interface::as_raw(self), bstrrealm.into_param().abi()).ok()
    }
    pub unsafe fn AllowedAuth(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AllowedAuth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllowedAuth(&self, lallowedauth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowedAuth)(::windows::core::Interface::as_raw(self), lallowedauth).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCProfile2, ::windows::core::IUnknown, IRTCProfile);
impl ::core::cmp::PartialEq for IRTCProfile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCProfile2 {}
impl ::core::fmt::Debug for IRTCProfile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfile2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCProfile2 {
    type Vtable = IRTCProfile2_Vtbl;
}
impl ::core::clone::Clone for IRTCProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCProfile2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b81f84e_bdc7_4184_9154_3cb2dd7917fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile2_Vtbl {
    pub base__: IRTCProfile_Vtbl,
    pub Realm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrealm: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetRealm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub AllowedAuth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows::core::HRESULT,
    pub SetAllowedAuth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCProfileEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEvent {
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__ = ::windows::core::zeroed::<isize>();
        (::windows::core::Interface::vtable(self).Cookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCProfileEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCProfileEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCProfileEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCProfileEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfileEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCProfileEvent {
    type Vtable = IRTCProfileEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCProfileEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCProfileEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6d5ab3b_770e_43e8_800a_79b062395fca);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCProfileEvent2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEvent2 {
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile>();
        (::windows::core::Interface::vtable(self).base__.Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__ = ::windows::core::zeroed::<isize>();
        (::windows::core::Interface::vtable(self).base__.Cookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_PROFILE_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_PROFILE_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCProfileEvent2, ::windows::core::IUnknown, super::Com::IDispatch, IRTCProfileEvent);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCProfileEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCProfileEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCProfileEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCProfileEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCProfileEvent2 {
    type Vtable = IRTCProfileEvent2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCProfileEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCProfileEvent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62e56edc_03fa_4121_94fb_23493fd0ae64);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent2_Vtbl {
    pub base__: IRTCProfileEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCReInviteEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCReInviteEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession2>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Accept<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Accept)(::windows::core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi()).ok()
    }
    pub unsafe fn Reject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reject)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REINVITE_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_REINVITE_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::windows::core::BSTR, pbstrsessiondescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRemoteSessionDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCReInviteEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCReInviteEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCReInviteEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCReInviteEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCReInviteEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCReInviteEvent {
    type Vtable = IRTCReInviteEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCReInviteEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCReInviteEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11558d84_204c_43e7_99b0_2034e9417f7d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCReInviteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCRegistrationStateChangeEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCRegistrationStateChangeEvent {
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_REGISTRATION_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCRegistrationStateChangeEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCRegistrationStateChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCRegistrationStateChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCRegistrationStateChangeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCRegistrationStateChangeEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCRegistrationStateChangeEvent {
    type Vtable = IRTCRegistrationStateChangeEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCRegistrationStateChangeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCRegistrationStateChangeEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62d0991b_50ab_4f02_b948_ca94f26f8f95);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRegistrationStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCRoamingEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCRoamingEvent {
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_ROAMING_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_ROAMING_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile2>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCRoamingEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCRoamingEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCRoamingEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCRoamingEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCRoamingEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCRoamingEvent {
    type Vtable = IRTCRoamingEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCRoamingEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCRoamingEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79960a6b_0cb1_4dc8_a805_7318e99902e8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRoamingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCSession(::windows::core::IUnknown);
impl IRTCSession {
    pub unsafe fn Client(&self) -> ::windows::core::Result<IRTCClient> {
        let mut result__ = ::windows::core::zeroed::<IRTCClient>();
        (::windows::core::Interface::vtable(self).Client)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_SESSION_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<RTC_SESSION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_SESSION_TYPE>();
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Participants(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Participants)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Answer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Answer)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self), enreason).ok()
    }
    pub unsafe fn Redirect<P0, P1>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: P0, pprofile: P1, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).Redirect)(::windows::core::Interface::as_raw(self), entype, bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), lflags).ok()
    }
    pub unsafe fn AddParticipant<P0, P1>(&self, bstraddress: P0, bstrname: P1) -> ::windows::core::Result<IRTCParticipant>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCParticipant>();
        (::windows::core::Interface::vtable(self).AddParticipant)(::windows::core::Interface::as_raw(self), bstraddress.into_param().abi(), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveParticipant<P0>(&self, pparticipant: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCParticipant>,
    {
        (::windows::core::Interface::vtable(self).RemoveParticipant)(::windows::core::Interface::as_raw(self), pparticipant.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> ::windows::core::Result<IRTCEnumParticipants> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumParticipants>();
        (::windows::core::Interface::vtable(self).EnumerateParticipants)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanAddParticipants(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).CanAddParticipants)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectedUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).RedirectedUserURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectedUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).RedirectedUserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NextRedirectedUser(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NextRedirectedUser)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage<P0, P1>(&self, bstrmessageheader: P0, bstrmessage: P1, lcookie: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SendMessage)(::windows::core::Interface::as_raw(self), bstrmessageheader.into_param().abi(), bstrmessage.into_param().abi(), lcookie).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendMessageStatus)(::windows::core::Interface::as_raw(self), enuserstatus, lcookie).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStream)(::windows::core::Interface::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStream)(::windows::core::Interface::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn put_EncryptionKey<P0>(&self, lmediatype: i32, encryptionkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).put_EncryptionKey)(::windows::core::Interface::as_raw(self), lmediatype, encryptionkey.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCSession, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSession {}
impl ::core::fmt::Debug for IRTCSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCSession {
    type Vtable = IRTCSession_Vtbl;
}
impl ::core::clone::Clone for IRTCSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x387c8086_99be_42fb_9973_7c0fc0ca9fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSession_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Client: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Participants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Participants: usize,
    pub Answer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT,
    pub Redirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub AddParticipant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveParticipant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparticipant: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanAddParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcanadd: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanAddParticipants: usize,
    pub RedirectedUserURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub RedirectedUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub NextRedirectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrmessage: ::std::mem::MaybeUninit<::windows::core::BSTR>, lcookie: isize) -> ::windows::core::HRESULT,
    pub SendMessageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT,
    pub put_EncryptionKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCSession2(::windows::core::IUnknown);
impl IRTCSession2 {
    pub unsafe fn Client(&self) -> ::windows::core::Result<IRTCClient> {
        let mut result__ = ::windows::core::zeroed::<IRTCClient>();
        (::windows::core::Interface::vtable(self).base__.Client)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_SESSION_STATE>();
        (::windows::core::Interface::vtable(self).base__.State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<RTC_SESSION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_SESSION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile>();
        (::windows::core::Interface::vtable(self).base__.Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Participants(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).base__.Participants)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Answer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Answer)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Terminate)(::windows::core::Interface::as_raw(self), enreason).ok()
    }
    pub unsafe fn Redirect<P0, P1>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: P0, pprofile: P1, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).base__.Redirect)(::windows::core::Interface::as_raw(self), entype, bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), lflags).ok()
    }
    pub unsafe fn AddParticipant<P0, P1>(&self, bstraddress: P0, bstrname: P1) -> ::windows::core::Result<IRTCParticipant>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IRTCParticipant>();
        (::windows::core::Interface::vtable(self).base__.AddParticipant)(::windows::core::Interface::as_raw(self), bstraddress.into_param().abi(), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveParticipant<P0>(&self, pparticipant: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCParticipant>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveParticipant)(::windows::core::Interface::as_raw(self), pparticipant.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> ::windows::core::Result<IRTCEnumParticipants> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumParticipants>();
        (::windows::core::Interface::vtable(self).base__.EnumerateParticipants)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanAddParticipants(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.CanAddParticipants)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectedUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.RedirectedUserURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectedUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.RedirectedUserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NextRedirectedUser(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.NextRedirectedUser)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage<P0, P1>(&self, bstrmessageheader: P0, bstrmessage: P1, lcookie: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SendMessage)(::windows::core::Interface::as_raw(self), bstrmessageheader.into_param().abi(), bstrmessage.into_param().abi(), lcookie).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SendMessageStatus)(::windows::core::Interface::as_raw(self), enuserstatus, lcookie).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddStream)(::windows::core::Interface::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveStream)(::windows::core::Interface::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn put_EncryptionKey<P0>(&self, lmediatype: i32, encryptionkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.put_EncryptionKey)(::windows::core::Interface::as_raw(self), lmediatype, encryptionkey.into_param().abi()).ok()
    }
    pub unsafe fn SendInfo<P0, P1>(&self, bstrinfoheader: P0, bstrinfo: P1, lcookie: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SendInfo)(::windows::core::Interface::as_raw(self), bstrinfoheader.into_param().abi(), bstrinfo.into_param().abi(), lcookie).ok()
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_PreferredSecurityLevel)(::windows::core::Interface::as_raw(self), ensecuritytype, ensecuritylevel).ok()
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::windows::core::zeroed::<RTC_SECURITY_LEVEL>();
        (::windows::core::Interface::vtable(self).get_PreferredSecurityLevel)(::windows::core::Interface::as_raw(self), ensecuritytype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsSecurityEnabled)(::windows::core::Interface::as_raw(self), ensecuritytype, &mut result__).from_abi(result__)
    }
    pub unsafe fn AnswerWithSessionDescription<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AnswerWithSessionDescription)(::windows::core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi()).ok()
    }
    pub unsafe fn ReInviteWithSessionDescription<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1, lcookie: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ReInviteWithSessionDescription)(::windows::core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), lcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCSession2, ::windows::core::IUnknown, IRTCSession);
impl ::core::cmp::PartialEq for IRTCSession2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSession2 {}
impl ::core::fmt::Debug for IRTCSession2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSession2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCSession2 {
    type Vtable = IRTCSession2_Vtbl;
}
impl ::core::clone::Clone for IRTCSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCSession2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17d7cdfc_b007_484c_99d2_86a8a820991d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSession2_Vtbl {
    pub base__: IRTCSession_Vtbl,
    pub SendInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinfoheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrinfo: ::std::mem::MaybeUninit<::windows::core::BSTR>, lcookie: isize) -> ::windows::core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSecurityEnabled: usize,
    pub AnswerWithSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ReInviteWithSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>, lcookie: isize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCSessionCallControl(::windows::core::IUnknown);
impl IRTCSessionCallControl {
    pub unsafe fn Hold(&self, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Hold)(::windows::core::Interface::as_raw(self), lcookie).ok()
    }
    pub unsafe fn UnHold(&self, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnHold)(::windows::core::Interface::as_raw(self), lcookie).ok()
    }
    pub unsafe fn Forward<P0>(&self, bstrforwardtouri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Forward)(::windows::core::Interface::as_raw(self), bstrforwardtouri.into_param().abi()).ok()
    }
    pub unsafe fn Refer<P0, P1>(&self, bstrrefertouri: P0, bstrrefercookie: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Refer)(::windows::core::Interface::as_raw(self), bstrrefertouri.into_param().abi(), bstrrefercookie.into_param().abi()).ok()
    }
    pub unsafe fn SetReferredByURI<P0>(&self, bstrreferredbyuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetReferredByURI)(::windows::core::Interface::as_raw(self), bstrreferredbyuri.into_param().abi()).ok()
    }
    pub unsafe fn ReferredByURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ReferredByURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReferCookie<P0>(&self, bstrrefercookie: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetReferCookie)(::windows::core::Interface::as_raw(self), bstrrefercookie.into_param().abi()).ok()
    }
    pub unsafe fn ReferCookie(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ReferCookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsReferred(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsReferred)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCSessionCallControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCSessionCallControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSessionCallControl {}
impl ::core::fmt::Debug for IRTCSessionCallControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionCallControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionCallControl {
    type Vtable = IRTCSessionCallControl_Vtbl;
}
impl ::core::clone::Clone for IRTCSessionCallControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCSessionCallControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a50d94_190b_4f82_9530_3b8ebf60758a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionCallControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Hold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT,
    pub UnHold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT,
    pub Forward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Refer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrefertouri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrrefercookie: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetReferredByURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetReferCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrefercookie: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsReferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisreferred: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsReferred: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCSessionDescriptionManager(::windows::core::IUnknown);
impl IRTCSessionDescriptionManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EvaluateSessionDescription<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).EvaluateSessionDescription)(::windows::core::Interface::as_raw(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), pfapplicationsession).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCSessionDescriptionManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCSessionDescriptionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSessionDescriptionManager {}
impl ::core::fmt::Debug for IRTCSessionDescriptionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionDescriptionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionDescriptionManager {
    type Vtable = IRTCSessionDescriptionManager_Vtbl;
}
impl ::core::clone::Clone for IRTCSessionDescriptionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCSessionDescriptionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba7f518e_d336_4070_93a6_865395c843f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionDescriptionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EvaluateSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EvaluateSessionDescription: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCSessionOperationCompleteEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__ = ::windows::core::zeroed::<isize>();
        (::windows::core::Interface::vtable(self).Cookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCSessionOperationCompleteEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCSessionOperationCompleteEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCSessionOperationCompleteEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCSessionOperationCompleteEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionOperationCompleteEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCSessionOperationCompleteEvent {
    type Vtable = IRTCSessionOperationCompleteEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCSessionOperationCompleteEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCSessionOperationCompleteEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6bff4c0_f7c8_4d3c_9a41_3550f78a95b0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCSessionOperationCompleteEvent2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEvent2 {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).base__.Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__ = ::windows::core::zeroed::<isize>();
        (::windows::core::Interface::vtable(self).base__.Cookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__ = ::windows::core::zeroed::<IRTCParticipant>();
        (::windows::core::Interface::vtable(self).Participant)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::windows::core::BSTR, pbstrsessiondescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRemoteSessionDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCSessionOperationCompleteEvent2, ::windows::core::IUnknown, super::Com::IDispatch, IRTCSessionOperationCompleteEvent);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCSessionOperationCompleteEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCSessionOperationCompleteEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCSessionOperationCompleteEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionOperationCompleteEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCSessionOperationCompleteEvent2 {
    type Vtable = IRTCSessionOperationCompleteEvent2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCSessionOperationCompleteEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCSessionOperationCompleteEvent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6fc2a9b_d5bc_4241_b436_1b8460c13832);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent2_Vtbl {
    pub base__: IRTCSessionOperationCompleteEvent_Vtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCSessionPortManagement(::windows::core::IUnknown);
impl IRTCSessionPortManagement {
    pub unsafe fn SetPortManager<P0>(&self, pportmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCPortManager>,
    {
        (::windows::core::Interface::vtable(self).SetPortManager)(::windows::core::Interface::as_raw(self), pportmanager.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCSessionPortManagement, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCSessionPortManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCSessionPortManagement {}
impl ::core::fmt::Debug for IRTCSessionPortManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionPortManagement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionPortManagement {
    type Vtable = IRTCSessionPortManagement_Vtbl;
}
impl ::core::clone::Clone for IRTCSessionPortManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCSessionPortManagement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa072f1d6_0286_4e1f_85f2_17a2948456ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionPortManagement_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetPortManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCSessionReferStatusEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferStatusEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession2>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReferStatus(&self) -> ::windows::core::Result<RTC_SESSION_REFER_STATUS> {
        let mut result__ = ::windows::core::zeroed::<RTC_SESSION_REFER_STATUS>();
        (::windows::core::Interface::vtable(self).ReferStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCSessionReferStatusEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCSessionReferStatusEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCSessionReferStatusEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCSessionReferStatusEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionReferStatusEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCSessionReferStatusEvent {
    type Vtable = IRTCSessionReferStatusEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCSessionReferStatusEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCSessionReferStatusEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d8fc2cd_5d76_44ab_bb68_2a80353b34a2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReferStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCSessionReferredEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferredEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession2>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReferredByURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ReferredByURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReferToURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ReferToURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReferCookie(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ReferCookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Accept(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Accept)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reject)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReferredSessionState)(::windows::core::Interface::as_raw(self), enstate).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCSessionReferredEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCSessionReferredEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCSessionReferredEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCSessionReferredEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionReferredEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCSessionReferredEvent {
    type Vtable = IRTCSessionReferredEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCSessionReferredEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCSessionReferredEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x176a6828_4fcc_4f28_a862_04597a6cf1c4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferredEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ReferToURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetReferredSessionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCSessionStateChangeEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_SESSION_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCSessionStateChangeEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCSessionStateChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCSessionStateChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCSessionStateChangeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionStateChangeEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCSessionStateChangeEvent {
    type Vtable = IRTCSessionStateChangeEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCSessionStateChangeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCSessionStateChangeEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bad703_5952_48b3_9321_7f4500521506);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub StatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCSessionStateChangeEvent2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEvent2 {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::windows::core::zeroed::<IRTCSession>();
        (::windows::core::Interface::vtable(self).base__.Session)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_SESSION_STATE>();
        (::windows::core::Interface::vtable(self).base__.State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.StatusText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MediaTypes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = ::windows::core::zeroed::<RTC_SECURITY_LEVEL>();
        (::windows::core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(::windows::core::Interface::as_raw(self), ensecuritytype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsForked(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsForked)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::windows::core::BSTR, pbstrsessiondescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRemoteSessionDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCSessionStateChangeEvent2, ::windows::core::IUnknown, super::Com::IDispatch, IRTCSessionStateChangeEvent);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCSessionStateChangeEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCSessionStateChangeEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCSessionStateChangeEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCSessionStateChangeEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCSessionStateChangeEvent2 {
    type Vtable = IRTCSessionStateChangeEvent2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCSessionStateChangeEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCSessionStateChangeEvent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f933171_6f95_4880_80d9_2ec8d495d261);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent2_Vtbl {
    pub base__: IRTCSessionStateChangeEvent_Vtbl,
    pub MediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsForked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisforked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsForked: usize,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCUserSearch(::windows::core::IUnknown);
impl IRTCUserSearch {
    pub unsafe fn CreateQuery(&self) -> ::windows::core::Result<IRTCUserSearchQuery> {
        let mut result__ = ::windows::core::zeroed::<IRTCUserSearchQuery>();
        (::windows::core::Interface::vtable(self).CreateQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExecuteSearch<P0, P1>(&self, pquery: P0, pprofile: P1, lcookie: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRTCUserSearchQuery>,
        P1: ::windows::core::IntoParam<IRTCProfile>,
    {
        (::windows::core::Interface::vtable(self).ExecuteSearch)(::windows::core::Interface::as_raw(self), pquery.into_param().abi(), pprofile.into_param().abi(), lcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCUserSearch, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCUserSearch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCUserSearch {}
impl ::core::fmt::Debug for IRTCUserSearch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCUserSearch {
    type Vtable = IRTCUserSearch_Vtbl;
}
impl ::core::clone::Clone for IRTCUserSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCUserSearch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb619882b_860c_4db4_be1b_693b6505bbe5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearch_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecuteSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquery: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCUserSearchQuery(::windows::core::IUnknown);
impl IRTCUserSearchQuery {
    pub unsafe fn put_SearchTerm<P0, P1>(&self, bstrname: P0, bstrvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).put_SearchTerm)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    pub unsafe fn get_SearchTerm<P0>(&self, bstrname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_SearchTerm)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchTerms(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).SearchTerms)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn put_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_SearchPreference)(::windows::core::Interface::as_raw(self), enpreference, lvalue).ok()
    }
    pub unsafe fn get_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).get_SearchPreference)(::windows::core::Interface::as_raw(self), enpreference, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSearchDomain<P0>(&self, bstrdomain: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetSearchDomain)(::windows::core::Interface::as_raw(self), bstrdomain.into_param().abi()).ok()
    }
    pub unsafe fn SearchDomain(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).SearchDomain)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCUserSearchQuery, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCUserSearchQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCUserSearchQuery {}
impl ::core::fmt::Debug for IRTCUserSearchQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearchQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCUserSearchQuery {
    type Vtable = IRTCUserSearchQuery_Vtbl;
}
impl ::core::clone::Clone for IRTCUserSearchQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCUserSearchQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x288300f5_d23a_4365_9a73_9985c98c2881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchQuery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub put_SearchTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub get_SearchTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SearchTerms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnames: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub put_SearchPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::HRESULT,
    pub get_SearchPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetSearchDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdomain: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SearchDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdomain: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCUserSearchResult(::windows::core::IUnknown);
impl IRTCUserSearchResult {
    pub unsafe fn get_Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_Value)(::windows::core::Interface::as_raw(self), encolumn, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCUserSearchResult, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRTCUserSearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCUserSearchResult {}
impl ::core::fmt::Debug for IRTCUserSearchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCUserSearchResult {
    type Vtable = IRTCUserSearchResult_Vtbl;
}
impl ::core::clone::Clone for IRTCUserSearchResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCUserSearchResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851278b2_9592_480f_8db5_2de86b26b54d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCUserSearchResultsEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCUserSearchResultsEvent {
    pub unsafe fn EnumerateResults(&self) -> ::windows::core::Result<IRTCEnumUserSearchResults> {
        let mut result__ = ::windows::core::zeroed::<IRTCEnumUserSearchResults>();
        (::windows::core::Interface::vtable(self).EnumerateResults)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Results(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::windows::core::zeroed::<IRTCCollection>();
        (::windows::core::Interface::vtable(self).Results)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile2>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Query(&self) -> ::windows::core::Result<IRTCUserSearchQuery> {
        let mut result__ = ::windows::core::zeroed::<IRTCUserSearchQuery>();
        (::windows::core::Interface::vtable(self).Query)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__ = ::windows::core::zeroed::<isize>();
        (::windows::core::Interface::vtable(self).Cookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoreAvailable(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).MoreAvailable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCUserSearchResultsEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCUserSearchResultsEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCUserSearchResultsEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCUserSearchResultsEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCUserSearchResultsEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCUserSearchResultsEvent {
    type Vtable = IRTCUserSearchResultsEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCUserSearchResultsEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCUserSearchResultsEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c8c3cd_7fac_4088_81c5_c24cbc0938e3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResultsEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EnumerateResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MoreAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoreAvailable: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCWatcher(::windows::core::IUnknown);
impl IRTCWatcher {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.PresentityURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPresentityURI)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Data)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.Persistent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPersistent)(::windows::core::Interface::as_raw(self), fpersistent.into_param().abi()).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_WATCHER_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_WATCHER_STATE>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::windows::core::Interface::as_raw(self), enstate).ok()
    }
}
::windows::imp::interface_hierarchy!(IRTCWatcher, ::windows::core::IUnknown, IRTCPresenceContact);
impl ::core::cmp::PartialEq for IRTCWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCWatcher {}
impl ::core::fmt::Debug for IRTCWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCWatcher {
    type Vtable = IRTCWatcher_Vtbl;
}
impl ::core::clone::Clone for IRTCWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7cedad8_346b_4d1b_ac02_a2088df9be4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct IRTCWatcher2(::windows::core::IUnknown);
impl IRTCWatcher2 {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.PresentityURI)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI<P0>(&self, bstrpresentityuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPresentityURI)(::windows::core::Interface::as_raw(self), bstrpresentityuri.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Data)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetData)(::windows::core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.Persistent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPersistent)(::windows::core::Interface::as_raw(self), fpersistent.into_param().abi()).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_WATCHER_STATE> {
        let mut result__ = ::windows::core::zeroed::<RTC_WATCHER_STATE>();
        (::windows::core::Interface::vtable(self).base__.State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetState)(::windows::core::Interface::as_raw(self), enstate).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__ = ::windows::core::zeroed::<IRTCProfile2>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Scope(&self) -> ::windows::core::Result<RTC_ACE_SCOPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_ACE_SCOPE>();
        (::windows::core::Interface::vtable(self).Scope)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRTCWatcher2, ::windows::core::IUnknown, IRTCPresenceContact, IRTCWatcher);
impl ::core::cmp::PartialEq for IRTCWatcher2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRTCWatcher2 {}
impl ::core::fmt::Debug for IRTCWatcher2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcher2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRTCWatcher2 {
    type Vtable = IRTCWatcher2_Vtbl;
}
impl ::core::clone::Clone for IRTCWatcher2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRTCWatcher2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4d9967f_d011_4b1d_91e3_aba78f96393d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher2_Vtbl {
    pub base__: IRTCWatcher_Vtbl,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCWatcherEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEvent {
    pub unsafe fn Watcher(&self) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher>();
        (::windows::core::Interface::vtable(self).Watcher)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCWatcherEvent, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCWatcherEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCWatcherEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCWatcherEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcherEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCWatcherEvent {
    type Vtable = IRTCWatcherEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCWatcherEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCWatcherEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf30d7261_587a_424f_822c_312788f43548);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Watcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRTCWatcherEvent2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEvent2 {
    pub unsafe fn Watcher(&self) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__ = ::windows::core::zeroed::<IRTCWatcher>();
        (::windows::core::Interface::vtable(self).base__.Watcher)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_WATCHER_EVENT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<RTC_WATCHER_EVENT_TYPE>();
        (::windows::core::Interface::vtable(self).EventType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StatusCode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IRTCWatcherEvent2, ::windows::core::IUnknown, super::Com::IDispatch, IRTCWatcherEvent);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRTCWatcherEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRTCWatcherEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRTCWatcherEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRTCWatcherEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRTCWatcherEvent2 {
    type Vtable = IRTCWatcherEvent2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRTCWatcherEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IRTCWatcherEvent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe52891e8_188c_49af_b005_98ed13f83f9c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent2_Vtbl {
    pub base__: IRTCWatcherEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
pub struct ITransportSettingsInternal(::windows::core::IUnknown);
impl ITransportSettingsInternal {
    #[doc = "*Required features: `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplySetting)(::windows::core::Interface::as_raw(self), setting).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QuerySetting)(::windows::core::Interface::as_raw(self), setting).ok()
    }
}
::windows::imp::interface_hierarchy!(ITransportSettingsInternal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITransportSettingsInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransportSettingsInternal {}
impl ::core::fmt::Debug for ITransportSettingsInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransportSettingsInternal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITransportSettingsInternal {
    type Vtable = ITransportSettingsInternal_Vtbl;
}
impl ::core::clone::Clone for ITransportSettingsInternal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITransportSettingsInternal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5123e076_29e3_4bfd_84fe_0192d411e3e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransportSettingsInternal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub ApplySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    ApplySetting: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub QuerySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    QuerySetting: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_DIGEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_KERBEROS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_NTLM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_USE_LOGON_CRED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCS_FAIL_ON_REDIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCS_FORCE_PROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCClient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a42ea29_a2b7_40c4_b091_f6f024aa89be);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_ALL: u32 = 33554431u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_BUDDY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_BUDDY2: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_GROUP: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_INFO: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_INTENSITY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_MEDIA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_MEDIA_REQUEST: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_MESSAGING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PARTICIPANT_STATE_CHANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PRESENCE_DATA: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PRESENCE_PROPERTY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PRESENCE_STATUS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PROFILE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_REGISTRATION_STATE_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_REINVITE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_ROAMING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_OPERATION_COMPLETE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_REFERRED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_REFER_STATUS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_STATE_CHANGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_USERSEARCH: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_WATCHER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_WATCHER2: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_DISABLE_MEDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_DISABLE_STRICT_DNS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_DISABLE_UPNP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_ENABLE_SERVER_CLASS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_AUDIO_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_AUDIO_SEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_T120_SENDRECV: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_VIDEO_RECEIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_VIDEO_SEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_INVITE_SESSIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_MESSAGE_SESSIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_NOTIFY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_PRESENCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_ALL_ROAMING: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_BUDDY_ROAMING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_PRESENCE_ROAMING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_PROFILE_ROAMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_WATCHER_ROAMING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_APPLICATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_IM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_MULTIPARTY_IM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_PC_TO_PC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_PC_TO_PHONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_PHONE_TO_PHONE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_TCP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_TLS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_UDP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ANOTHER_MEDIA_SESSION_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885961i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_BASIC_AUTH_SET_TLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886017i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_CLIENT_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886042i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_CLIENT_ALREADY_SHUT_DOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886041i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_CLIENT_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886043i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DESTINATION_ADDRESS_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886061i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DESTINATION_ADDRESS_MULTICAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886059i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_BUDDY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886006i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_GROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885998i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_REALM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886013i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_WATCHER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886005i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_ACL_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886000i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_ADDRESS_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886060i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_BUDDY_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886001i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_LISTEN_SOCKET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885957i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_OBJECT_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885983i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PORTRANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885988i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PREFERENCE_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885991i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886034i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PROXY_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886058i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_REGISTRATION_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885971i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_SESSION_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886038i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_SESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886039i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_SIP_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886062i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_LISTENING_SOCKET_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885958i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_LOCAL_PHONE_NEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886036i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MALFORMED_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886004i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MAX_PENDING_OPERATIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885990i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MAX_REDIRECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885960i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_AEC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886044i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_AUDIO_DEVICE_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886047i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_CONTROLLER_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886049i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885970i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885969i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_NEED_TERMINAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886048i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_SESSION_IN_HOLD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885962i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_SESSION_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885963i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_VIDEO_DEVICE_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886046i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885950i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885992i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NOT_PRESENCE_PROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885974i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_BUDDY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885996i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886035i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_GROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885999i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_PROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886037i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_REALM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885994i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_TRANSPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885993i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_WATCHER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885995i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_OPERATION_WITH_TOO_MANY_PARTICIPANTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886018i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_ALL_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755001i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_BADNUMBER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131754997i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755003i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131754998i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_NO_ANSWER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755002i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_PL_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755000i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_SW_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131754999i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PLATFORM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885952i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_POLICY_NOT_ALLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886012i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PORT_MANAGER_ALREADY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885956i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PORT_MAPPING_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886010i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PORT_MAPPING_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886011i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PRESENCE_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885982i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PRESENCE_NOT_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886040i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_AUTHMETHOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886024i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886025i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_ROLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886023i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886021i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SESSION_PARTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886020i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886019i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_MULTIPLE_REGISTRARS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886022i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886032i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886031i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_PROVISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886033i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886028i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_SERVER_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886027i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_SERVER_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886026i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_USER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886030i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_USER_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886029i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_SERVER_UNAUTHORIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886014i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REDIRECT_PROCESSING_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885959i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REFER_NOT_ACCEPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885968i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REFER_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885967i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REFER_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885966i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REGISTRATION_DEACTIVATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885949i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REGISTRATION_REJECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885948i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REGISTRATION_UNREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885947i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ROAMING_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885981i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ROAMING_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886002i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ROAMING_OPERATION_INTERRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886003i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_CONNECTION_ADDR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886070i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_FAILED_TO_BUILD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886067i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_MULTICAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886071i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886074i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886069i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_PARSE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886073i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_UPDATE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886072i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_ALREADY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885955i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_COMPATIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886009i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_DEFINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886008i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_SUPPORTED_BY_PARTICIPANT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886007i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_ADDITIONAL_PARTY_IN_TWO_PARTY_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885986i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886063i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_HEADER_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886065i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_TIME_SKEW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885972i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886064i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_CALL_CONNECTION_NOT_ESTABLISHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885987i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_CALL_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886055i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_CODECS_DO_NOT_MATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886080i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_DNS_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885978i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_HEADER_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886075i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_HIGH_SECURITY_SET_TLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886016i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_HOLD_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885965i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_INVALID_CERTIFICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885979i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_INVITEE_PARTY_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885973i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_INVITE_TRANSACTION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886066i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_NEED_MORE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886056i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_NO_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886077i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_OTHER_PARTY_JOIN_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885984i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_PARSE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886076i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_PARTY_ALREADY_IN_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885985i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_PEER_PARTICIPANT_IN_MULTIPARTY_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885951i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_REFER_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885953i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_REQUEST_DESTINATION_ADDR_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886054i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_SSL_NEGOTIATION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886051i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_SSL_TUNNEL_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886052i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_STACK_SHUTDOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886050i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_STREAM_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886078i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_STREAM_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886079i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TCP_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885977i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886068i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TLS_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885975i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TLS_INCOMPATIBLE_ENCRYPTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885980i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TRANSPORT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886057i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_UDP_SIZE_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886053i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_UNHOLD_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885964i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_START_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886045i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_ADDRESS_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820060i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_AMBIGUOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820059i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_BAD_EXTENSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820124i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_BAD_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820144i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_BUSY_HERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820058i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_CONFLICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820135i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_FORBIDDEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820141i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_GONE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820134i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_LENGTH_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820133i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_LOOP_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820062i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_METHOD_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820139i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_NOT_ACCEPTABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820138i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820140i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_PAYMENT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820142i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_PROXY_AUTHENTICATION_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820137i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_ENTITY_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820131i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820136i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_URI_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820130i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_TEMPORARILY_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820064i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_TOO_MANY_HOPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820061i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_TRANSACTION_DOES_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820063i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_UNAUTHORIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820143i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_UNSUPPORTED_MEDIA_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820129i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_BUSY_EVERYWHERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819944i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_DECLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819941i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819940i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_NOT_ACCEPTABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819938i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_CALL_FORWARDING: ::windows::core::HRESULT = ::windows::core::HRESULT(15663285i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_QUEUED: ::windows::core::HRESULT = ::windows::core::HRESULT(15663286i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_RINGING: ::windows::core::HRESULT = ::windows::core::HRESULT(15663284i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_TRYING: ::windows::core::HRESULT = ::windows::core::HRESULT(15663204i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_NOT_ACCEPTABLE_HERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820056i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_ALTERNATIVE_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820164i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_MOVED_PERMANENTLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820243i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_MOVED_TEMPORARILY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820242i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_MULTIPLE_CHOICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820244i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_SEE_OTHER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820241i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_USE_PROXY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820239i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REQUEST_TERMINATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820057i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_BAD_GATEWAY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820042i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820044i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_NOT_IMPLEMENTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820043i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_SERVER_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820040i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_SERVICE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820041i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_VERSION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820039i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SESSION_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(15663287i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SUCCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(15663304i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_TOO_MANY_GROUPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885997i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_TOO_MANY_RETRIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885989i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_TOO_SMALL_EXPIRES_VALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885976i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_UDP_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885954i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_S_ROAMING_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(15597633i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_ACE_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAS_SCOPE_USER: RTC_ACE_SCOPE = RTC_ACE_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAS_SCOPE_DOMAIN: RTC_ACE_SCOPE = RTC_ACE_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAS_SCOPE_ALL: RTC_ACE_SCOPE = RTC_ACE_SCOPE(2i32);
impl ::core::marker::Copy for RTC_ACE_SCOPE {}
impl ::core::clone::Clone for RTC_ACE_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_ACE_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_ACE_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_ACE_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ACE_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_ANSWER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_OFFER_SESSION_EVENT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_AUTOMATICALLY_ACCEPT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_AUTOMATICALLY_REJECT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_NOT_SUPPORTED: RTC_ANSWER_MODE = RTC_ANSWER_MODE(3i32);
impl ::core::marker::Copy for RTC_ANSWER_MODE {}
impl ::core::clone::Clone for RTC_ANSWER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_ANSWER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_ANSWER_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_ANSWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ANSWER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_AUDIO_DEVICE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAD_SPEAKER: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAD_MICROPHONE: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(1i32);
impl ::core::marker::Copy for RTC_AUDIO_DEVICE {}
impl ::core::clone::Clone for RTC_AUDIO_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_AUDIO_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_AUDIO_DEVICE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_AUDIO_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_AUDIO_DEVICE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_BUDDY_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_ADD: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_REMOVE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_UPDATE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_STATE_CHANGE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_ROAMED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_SUBSCRIBED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(5i32);
impl ::core::marker::Copy for RTC_BUDDY_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_BUDDY_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_BUDDY_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_BUDDY_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_BUDDY_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_BUDDY_SUBSCRIPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_SUBSCRIBED: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_ALWAYS_OFFLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_ALWAYS_ONLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_POLL: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(3i32);
impl ::core::marker::Copy for RTC_BUDDY_SUBSCRIPTION_TYPE {}
impl ::core::clone::Clone for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_BUDDY_SUBSCRIPTION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_SUBSCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_CLIENT_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_VOLUME_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_DEVICE_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_NETWORK_QUALITY_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_ASYNC_CLEANUP_DONE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(3i32);
impl ::core::marker::Copy for RTC_CLIENT_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_CLIENT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_CLIENT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_CLIENT_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_CLIENT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_CLIENT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_DTMF(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_0: RTC_DTMF = RTC_DTMF(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_1: RTC_DTMF = RTC_DTMF(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_2: RTC_DTMF = RTC_DTMF(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_3: RTC_DTMF = RTC_DTMF(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_4: RTC_DTMF = RTC_DTMF(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_5: RTC_DTMF = RTC_DTMF(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_6: RTC_DTMF = RTC_DTMF(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_7: RTC_DTMF = RTC_DTMF(7i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_8: RTC_DTMF = RTC_DTMF(8i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_9: RTC_DTMF = RTC_DTMF(9i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_STAR: RTC_DTMF = RTC_DTMF(10i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_POUND: RTC_DTMF = RTC_DTMF(11i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_A: RTC_DTMF = RTC_DTMF(12i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_B: RTC_DTMF = RTC_DTMF(13i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_C: RTC_DTMF = RTC_DTMF(14i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_D: RTC_DTMF = RTC_DTMF(15i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_FLASH: RTC_DTMF = RTC_DTMF(16i32);
impl ::core::marker::Copy for RTC_DTMF {}
impl ::core::clone::Clone for RTC_DTMF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_DTMF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_DTMF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_DTMF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_DTMF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_CLIENT: RTC_EVENT = RTC_EVENT(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_REGISTRATION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_OPERATION_COMPLETE: RTC_EVENT = RTC_EVENT(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PARTICIPANT_STATE_CHANGE: RTC_EVENT = RTC_EVENT(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_MEDIA: RTC_EVENT = RTC_EVENT(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_INTENSITY: RTC_EVENT = RTC_EVENT(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_MESSAGING: RTC_EVENT = RTC_EVENT(7i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_BUDDY: RTC_EVENT = RTC_EVENT(8i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_WATCHER: RTC_EVENT = RTC_EVENT(9i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PROFILE: RTC_EVENT = RTC_EVENT(10i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_USERSEARCH: RTC_EVENT = RTC_EVENT(11i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_INFO: RTC_EVENT = RTC_EVENT(12i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_GROUP: RTC_EVENT = RTC_EVENT(13i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_MEDIA_REQUEST: RTC_EVENT = RTC_EVENT(14i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_ROAMING: RTC_EVENT = RTC_EVENT(15i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PRESENCE_PROPERTY: RTC_EVENT = RTC_EVENT(16i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PRESENCE_DATA: RTC_EVENT = RTC_EVENT(17i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PRESENCE_STATUS: RTC_EVENT = RTC_EVENT(18i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_REFER_STATUS: RTC_EVENT = RTC_EVENT(19i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_REFERRED: RTC_EVENT = RTC_EVENT(20i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_REINVITE: RTC_EVENT = RTC_EVENT(21i32);
impl ::core::marker::Copy for RTC_EVENT {}
impl ::core::clone::Clone for RTC_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_EVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_GROUP_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_UPDATE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_BUDDY_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_BUDDY_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_ROAMED: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(5i32);
impl ::core::marker::Copy for RTC_GROUP_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_GROUP_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_GROUP_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_GROUP_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_GROUP_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_GROUP_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_LISTEN_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCLM_NONE: RTC_LISTEN_MODE = RTC_LISTEN_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCLM_DYNAMIC: RTC_LISTEN_MODE = RTC_LISTEN_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCLM_BOTH: RTC_LISTEN_MODE = RTC_LISTEN_MODE(2i32);
impl ::core::marker::Copy for RTC_LISTEN_MODE {}
impl ::core::clone::Clone for RTC_LISTEN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_LISTEN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_LISTEN_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_LISTEN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_LISTEN_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_MEDIA_EVENT_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_NORMAL: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_HOLD: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_TIMEOUT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_BAD_DEVICE: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_NO_PORT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_PORT_MAPPING_FAILED: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_REMOTE_REQUEST: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(6i32);
impl ::core::marker::Copy for RTC_MEDIA_EVENT_REASON {}
impl ::core::clone::Clone for RTC_MEDIA_EVENT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MEDIA_EVENT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_MEDIA_EVENT_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_MEDIA_EVENT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_MEDIA_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMET_STOPPED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMET_STARTED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMET_FAILED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(2i32);
impl ::core::marker::Copy for RTC_MEDIA_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_MEDIA_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MEDIA_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_MEDIA_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_MEDIA_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_MESSAGING_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMSET_MESSAGE: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMSET_STATUS: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(1i32);
impl ::core::marker::Copy for RTC_MESSAGING_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_MESSAGING_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MESSAGING_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_MESSAGING_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_MESSAGING_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_MESSAGING_USER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMUS_IDLE: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMUS_TYPING: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(1i32);
impl ::core::marker::Copy for RTC_MESSAGING_USER_STATUS {}
impl ::core::clone::Clone for RTC_MESSAGING_USER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_MESSAGING_USER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_MESSAGING_USER_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_MESSAGING_USER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_USER_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_OFFER_WATCHER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCOWM_OFFER_WATCHER_EVENT: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCOWM_AUTOMATICALLY_ADD_WATCHER: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(1i32);
impl ::core::marker::Copy for RTC_OFFER_WATCHER_MODE {}
impl ::core::clone::Clone for RTC_OFFER_WATCHER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_OFFER_WATCHER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_OFFER_WATCHER_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_OFFER_WATCHER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_OFFER_WATCHER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_PARTICIPANT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_IDLE: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_PENDING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_INCOMING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_ANSWERING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_INPROGRESS: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_ALERTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_CONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_DISCONNECTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(7i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_DISCONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(8i32);
impl ::core::marker::Copy for RTC_PARTICIPANT_STATE {}
impl ::core::clone::Clone for RTC_PARTICIPANT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PARTICIPANT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_PARTICIPANT_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_PARTICIPANT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PARTICIPANT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_PORT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_AUDIO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_AUDIO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_VIDEO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_VIDEO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_SIP: RTC_PORT_TYPE = RTC_PORT_TYPE(4i32);
impl ::core::marker::Copy for RTC_PORT_TYPE {}
impl ::core::clone::Clone for RTC_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_PORT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PORT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_PRESENCE_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_PHONENUMBER: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_DISPLAYNAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_EMAIL: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_DEVICE_NAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_MULTIPLE: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(4i32);
impl ::core::marker::Copy for RTC_PRESENCE_PROPERTY {}
impl ::core::clone::Clone for RTC_PRESENCE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PRESENCE_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_PRESENCE_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_PRESENCE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_PROPERTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_PRESENCE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_OFFLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_ONLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_AWAY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_IDLE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_BUSY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_BE_RIGHT_BACK: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_ON_THE_PHONE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_OUT_TO_LUNCH: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(7i32);
impl ::core::marker::Copy for RTC_PRESENCE_STATUS {}
impl ::core::clone::Clone for RTC_PRESENCE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PRESENCE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_PRESENCE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_PRESENCE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_PRIVACY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPM_BLOCK_LIST_EXCLUDED: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPM_ALLOW_LIST_ONLY: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(1i32);
impl ::core::marker::Copy for RTC_PRIVACY_MODE {}
impl ::core::clone::Clone for RTC_PRIVACY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PRIVACY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_PRIVACY_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_PRIVACY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRIVACY_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_PROFILE_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPFET_PROFILE_GET: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPFET_PROFILE_UPDATE: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(1i32);
impl ::core::marker::Copy for RTC_PROFILE_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_PROFILE_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PROFILE_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_PROFILE_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_PROFILE_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PROFILE_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_PROVIDER_URI(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIHOMEPAGE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIHELPDESK: RTC_PROVIDER_URI = RTC_PROVIDER_URI(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIPERSONALACCOUNT: RTC_PROVIDER_URI = RTC_PROVIDER_URI(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIDISPLAYDURINGCALL: RTC_PROVIDER_URI = RTC_PROVIDER_URI(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIDISPLAYDURINGIDLE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(4i32);
impl ::core::marker::Copy for RTC_PROVIDER_URI {}
impl ::core::clone::Clone for RTC_PROVIDER_URI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_PROVIDER_URI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_PROVIDER_URI {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_PROVIDER_URI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PROVIDER_URI").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_REGISTRATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_NOT_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REJECTED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_UNREGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_ERROR: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_LOCAL_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(7i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REMOTE_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(8i32);
impl ::core::marker::Copy for RTC_REGISTRATION_STATE {}
impl ::core::clone::Clone for RTC_REGISTRATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_REGISTRATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_REGISTRATION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_REGISTRATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_REGISTRATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_REINVITE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRIN_INCOMING: RTC_REINVITE_STATE = RTC_REINVITE_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRIN_SUCCEEDED: RTC_REINVITE_STATE = RTC_REINVITE_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRIN_FAIL: RTC_REINVITE_STATE = RTC_REINVITE_STATE(2i32);
impl ::core::marker::Copy for RTC_REINVITE_STATE {}
impl ::core::clone::Clone for RTC_REINVITE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_REINVITE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_REINVITE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_REINVITE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_REINVITE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_RING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRT_PHONE: RTC_RING_TYPE = RTC_RING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRT_MESSAGE: RTC_RING_TYPE = RTC_RING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRT_RINGBACK: RTC_RING_TYPE = RTC_RING_TYPE(2i32);
impl ::core::marker::Copy for RTC_RING_TYPE {}
impl ::core::clone::Clone for RTC_RING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_RING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_RING_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_RING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_RING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_ROAMING_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_BUDDY_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_WATCHER_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_PRESENCE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_PROFILE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_WPENDING_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(4i32);
impl ::core::marker::Copy for RTC_ROAMING_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_ROAMING_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_ROAMING_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_ROAMING_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_ROAMING_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ROAMING_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_SECURITY_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECL_UNSUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECL_SUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECL_REQUIRED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(3i32);
impl ::core::marker::Copy for RTC_SECURITY_LEVEL {}
impl ::core::clone::Clone for RTC_SECURITY_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SECURITY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_SECURITY_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_SECURITY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_SECURITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECT_T120_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(1i32);
impl ::core::marker::Copy for RTC_SECURITY_TYPE {}
impl ::core::clone::Clone for RTC_SECURITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SECURITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_SECURITY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_SECURITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_SESSION_REFER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_REFERRING: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_ACCEPTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_ERROR: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_REJECTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_DROPPED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_DONE: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(5i32);
impl ::core::marker::Copy for RTC_SESSION_REFER_STATUS {}
impl ::core::clone::Clone for RTC_SESSION_REFER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SESSION_REFER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_SESSION_REFER_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_SESSION_REFER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_REFER_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_SESSION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_IDLE: RTC_SESSION_STATE = RTC_SESSION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_INCOMING: RTC_SESSION_STATE = RTC_SESSION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_ANSWERING: RTC_SESSION_STATE = RTC_SESSION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_INPROGRESS: RTC_SESSION_STATE = RTC_SESSION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_CONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_DISCONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_HOLD: RTC_SESSION_STATE = RTC_SESSION_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_REFER: RTC_SESSION_STATE = RTC_SESSION_STATE(7i32);
impl ::core::marker::Copy for RTC_SESSION_STATE {}
impl ::core::clone::Clone for RTC_SESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_SESSION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_SESSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_PC_TO_PC: RTC_SESSION_TYPE = RTC_SESSION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_PC_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_PHONE_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_MULTIPARTY_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_APPLICATION: RTC_SESSION_TYPE = RTC_SESSION_TYPE(5i32);
impl ::core::marker::Copy for RTC_SESSION_TYPE {}
impl ::core::clone::Clone for RTC_SESSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_SESSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_SESSION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_SESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_T120_APPLET(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTA_WHITEBOARD: RTC_T120_APPLET = RTC_T120_APPLET(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTA_APPSHARING: RTC_T120_APPLET = RTC_T120_APPLET(1i32);
impl ::core::marker::Copy for RTC_T120_APPLET {}
impl ::core::clone::Clone for RTC_T120_APPLET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_T120_APPLET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_T120_APPLET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_T120_APPLET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_T120_APPLET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_TERMINATE_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_NORMAL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_DND: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_BUSY: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_REJECT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_TIMEOUT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_SHUTDOWN: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_INSUFFICIENT_SECURITY_LEVEL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_NOT_SUPPORTED: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(7i32);
impl ::core::marker::Copy for RTC_TERMINATE_REASON {}
impl ::core::clone::Clone for RTC_TERMINATE_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_TERMINATE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_TERMINATE_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_TERMINATE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_TERMINATE_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_USER_SEARCH_COLUMN(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_URI: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_DISPLAYNAME: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_TITLE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_OFFICE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_PHONE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_COMPANY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(5i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_CITY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(6i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_STATE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(7i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_COUNTRY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(8i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_EMAIL: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(9i32);
impl ::core::marker::Copy for RTC_USER_SEARCH_COLUMN {}
impl ::core::clone::Clone for RTC_USER_SEARCH_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_USER_SEARCH_COLUMN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_USER_SEARCH_COLUMN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_USER_SEARCH_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_COLUMN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_USER_SEARCH_PREFERENCE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSP_MAX_MATCHES: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSP_TIME_LIMIT: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(1i32);
impl ::core::marker::Copy for RTC_USER_SEARCH_PREFERENCE {}
impl ::core::clone::Clone for RTC_USER_SEARCH_PREFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_USER_SEARCH_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_USER_SEARCH_PREFERENCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_USER_SEARCH_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_PREFERENCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_VIDEO_DEVICE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCVD_RECEIVE: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCVD_PREVIEW: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(1i32);
impl ::core::marker::Copy for RTC_VIDEO_DEVICE {}
impl ::core::clone::Clone for RTC_VIDEO_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_VIDEO_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_VIDEO_DEVICE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_VIDEO_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_VIDEO_DEVICE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_WATCHER_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_ADD: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_REMOVE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_UPDATE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_OFFERING: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_ROAMED: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(4i32);
impl ::core::marker::Copy for RTC_WATCHER_EVENT_TYPE {}
impl ::core::clone::Clone for RTC_WATCHER_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_WATCHER_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_WATCHER_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_WATCHER_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_WATCHER_MATCH_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWMM_EXACT_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWMM_BEST_ACE_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(1i32);
impl ::core::marker::Copy for RTC_WATCHER_MATCH_MODE {}
impl ::core::clone::Clone for RTC_WATCHER_MATCH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_WATCHER_MATCH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_WATCHER_MATCH_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_WATCHER_MATCH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_MATCH_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTC_WATCHER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_UNKNOWN: RTC_WATCHER_STATE = RTC_WATCHER_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_OFFERING: RTC_WATCHER_STATE = RTC_WATCHER_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_ALLOWED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_BLOCKED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_DENIED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_PROMPT: RTC_WATCHER_STATE = RTC_WATCHER_STATE(5i32);
impl ::core::marker::Copy for RTC_WATCHER_STATE {}
impl ::core::clone::Clone for RTC_WATCHER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTC_WATCHER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTC_WATCHER_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTC_WATCHER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct TRANSPORT_SETTING {
    pub SettingId: super::super::Networking::WinSock::TRANSPORT_SETTING_ID,
    pub Length: *mut u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for TRANSPORT_SETTING {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for TRANSPORT_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for TRANSPORT_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_SETTING").field("SettingId", &self.SettingId).field("Length", &self.Length).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for TRANSPORT_SETTING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for TRANSPORT_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.SettingId == other.SettingId && self.Length == other.Length && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for TRANSPORT_SETTING {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
