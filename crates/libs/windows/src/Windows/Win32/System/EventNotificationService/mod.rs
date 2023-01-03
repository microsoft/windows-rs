#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDestinationReachableA<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "sensapi.dll""system" fn IsDestinationReachableA ( lpszdestination : :: windows::core::PCSTR , lpqocinfo : *mut QOCINFO ) -> super::super::Foundation:: BOOL );
    IsDestinationReachableA(lpszdestination.into().abi(), lpqocinfo)
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDestinationReachableW<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "sensapi.dll""system" fn IsDestinationReachableW ( lpszdestination : :: windows::core::PCWSTR , lpqocinfo : *mut QOCINFO ) -> super::super::Foundation:: BOOL );
    IsDestinationReachableW(lpszdestination.into().abi(), lpqocinfo)
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNetworkAlive(lpdwflags: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "sensapi.dll""system" fn IsNetworkAlive ( lpdwflags : *mut u32 ) -> super::super::Foundation:: BOOL );
    IsNetworkAlive(lpdwflags)
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensLogon(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon {
    pub unsafe fn Logon(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Logon)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn Logoff(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Logoff)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn StartShell(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartShell)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn DisplayLock(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisplayLock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn DisplayUnlock(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisplayUnlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn StartScreenSaver(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartScreenSaver)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn StopScreenSaver(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StopScreenSaver)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISensLogon, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensLogon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensLogon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensLogon {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensLogon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensLogon").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISensLogon {
    type Vtable = ISensLogon_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensLogon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab3_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensLogon_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Logon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Logoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartShell: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartScreenSaver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopScreenSaver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensLogon2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon2 {
    pub unsafe fn Logon(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Logon)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername), dwsessionid).ok()
    }
    pub unsafe fn Logoff(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Logoff)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername), dwsessionid).ok()
    }
    pub unsafe fn SessionDisconnect(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SessionDisconnect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername), dwsessionid).ok()
    }
    pub unsafe fn SessionReconnect(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SessionReconnect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername), dwsessionid).ok()
    }
    pub unsafe fn PostShell(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PostShell)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername), dwsessionid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISensLogon2, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensLogon2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensLogon2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensLogon2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensLogon2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensLogon2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISensLogon2 {
    type Vtable = ISensLogon2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensLogon2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab4_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensLogon2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Logon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void, dwsessionid: u32) -> ::windows::core::HRESULT,
    pub Logoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void, dwsessionid: u32) -> ::windows::core::HRESULT,
    pub SessionDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void, dwsessionid: u32) -> ::windows::core::HRESULT,
    pub SessionReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void, dwsessionid: u32) -> ::windows::core::HRESULT,
    pub PostShell: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::core::ffi::c_void, dwsessionid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensNetwork(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensNetwork {
    pub unsafe fn ConnectionMade(&self, bstrconnection: &::windows::core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConnectionMade)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrconnection), ultype, lpqocinfo).ok()
    }
    pub unsafe fn ConnectionMadeNoQOCInfo(&self, bstrconnection: &::windows::core::BSTR, ultype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConnectionMadeNoQOCInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrconnection), ultype).ok()
    }
    pub unsafe fn ConnectionLost(&self, bstrconnection: &::windows::core::BSTR, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConnectionLost)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrconnection), ultype).ok()
    }
    pub unsafe fn DestinationReachable(&self, bstrdestination: &::windows::core::BSTR, bstrconnection: &::windows::core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DestinationReachable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdestination), ::core::mem::transmute_copy(bstrconnection), ultype, lpqocinfo).ok()
    }
    pub unsafe fn DestinationReachableNoQOCInfo(&self, bstrdestination: &::windows::core::BSTR, bstrconnection: &::windows::core::BSTR, ultype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DestinationReachableNoQOCInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdestination), ::core::mem::transmute_copy(bstrconnection), ultype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISensNetwork, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensNetwork").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISensNetwork {
    type Vtable = ISensNetwork_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensNetwork {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab1_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensNetwork_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConnectionMade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnection: *mut ::core::ffi::c_void, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT,
    pub ConnectionMadeNoQOCInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnection: *mut ::core::ffi::c_void, ultype: u32) -> ::windows::core::HRESULT,
    pub ConnectionLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnection: *mut ::core::ffi::c_void, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::HRESULT,
    pub DestinationReachable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdestination: *mut ::core::ffi::c_void, bstrconnection: *mut ::core::ffi::c_void, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT,
    pub DestinationReachableNoQOCInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdestination: *mut ::core::ffi::c_void, bstrconnection: *mut ::core::ffi::c_void, ultype: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISensOnNow(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISensOnNow {
    pub unsafe fn OnACPower(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnACPower)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnBatteryPower)(::windows::core::Vtable::as_raw(self), dwbatterylifepercent).ok()
    }
    pub unsafe fn BatteryLow(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BatteryLow)(::windows::core::Vtable::as_raw(self), dwbatterylifepercent).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ISensOnNow, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISensOnNow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISensOnNow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISensOnNow {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISensOnNow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensOnNow").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ISensOnNow {
    type Vtable = ISensOnNow_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISensOnNow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597bab2_5b9f_11d1_8dd2_00aa004abd5e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensOnNow_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub OnACPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnBatteryPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT,
    pub BatteryLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_AOL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd597cafe_5b9f_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENSGUID_EVENTCLASS_LOGON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978630_5b9f_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENSGUID_EVENTCLASS_LOGON2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978650_5b9f_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENSGUID_EVENTCLASS_NETWORK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978620_5b9f_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENSGUID_EVENTCLASS_ONNOW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5978640_5b9f_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENSGUID_PUBLISHER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fee1bd6_5b9b_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENSGUID_SUBSCRIBER_LCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3938ab0_5b9d_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const SENSGUID_SUBSCRIBER_WININET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3938ab5_5b9d_11d1_8dd2_00aa004abd5e);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SENS_CONNECTION_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_LAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_WAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(1u32);
impl ::core::marker::Copy for SENS_CONNECTION_TYPE {}
impl ::core::clone::Clone for SENS_CONNECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SENS_CONNECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SENS_CONNECTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SENS_CONNECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SENS_CONNECTION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
impl ::core::marker::Copy for QOCINFO {}
impl ::core::clone::Clone for QOCINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOCINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOCINFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwInSpeed", &self.dwInSpeed).field("dwOutSpeed", &self.dwOutSpeed).finish()
    }
}
unsafe impl ::windows::core::Abi for QOCINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOCINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwInSpeed == other.dwInSpeed && self.dwOutSpeed == other.dwOutSpeed
    }
}
impl ::core::cmp::Eq for QOCINFO {}
impl ::core::default::Default for QOCINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub struct SENS_QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOutSpeed: u32,
    pub dwInSpeed: u32,
}
impl ::core::marker::Copy for SENS_QOCINFO {}
impl ::core::clone::Clone for SENS_QOCINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SENS_QOCINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SENS_QOCINFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwOutSpeed", &self.dwOutSpeed).field("dwInSpeed", &self.dwInSpeed).finish()
    }
}
unsafe impl ::windows::core::Abi for SENS_QOCINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SENS_QOCINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwOutSpeed == other.dwOutSpeed && self.dwInSpeed == other.dwInSpeed
    }
}
impl ::core::cmp::Eq for SENS_QOCINFO {}
impl ::core::default::Default for SENS_QOCINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
