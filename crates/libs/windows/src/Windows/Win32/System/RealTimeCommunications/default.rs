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
impl IRTCBuddy {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PresentityURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPresentityURI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Data)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetData(&self, bstrdata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Persistent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPersistent)(::windows::core::Vtable::as_raw(self), fpersistent.into()).ok()
    }
}
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
impl IRTCBuddy2 {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PresentityURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPresentityURI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Data)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetData(&self, bstrdata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Persistent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPersistent)(::windows::core::Vtable::as_raw(self), fpersistent.into()).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Notes(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Notes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCBuddyEvent2 {
    pub unsafe fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Buddy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCClient2 {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PrepareForShutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEventFilter)(::windows::core::Vtable::as_raw(self), lfilter).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EventFilter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredMediaTypes<P0>(&self, lmediatypes: i32, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPreferredMediaTypes)(::windows::core::Vtable::as_raw(self), lmediatypes, fpersistent.into()).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PreferredMediaTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MediaCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSession<P0>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &::windows::core::BSTR, pprofile: P0, lflags: i32) -> ::windows::core::Result<IRTCSession>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRTCProfile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSession)(::windows::core::Vtable::as_raw(self), entype, ::core::mem::transmute_copy(bstrlocalphoneuri), pprofile.into().abi(), lflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetListenForIncomingSessions)(::windows::core::Vtable::as_raw(self), enlisten).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> ::windows::core::Result<RTC_LISTEN_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ListenForIncomingSessions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_NetworkAddresses<P0, P1>(&self, ftcp: P0, fexternal: P1) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_NetworkAddresses)(::windows::core::Vtable::as_raw(self), ftcp.into(), fexternal.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_Volume)(::windows::core::Vtable::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Volume)(::windows::core::Vtable::as_raw(self), endevice, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_AudioMuted<P0>(&self, endevice: RTC_AUDIO_DEVICE, fmuted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.put_AudioMuted)(::windows::core::Vtable::as_raw(self), endevice, fmuted.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_AudioMuted)(::windows::core::Vtable::as_raw(self), endevice, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DirectShow\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub unsafe fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_IVideoWindow)(::windows::core::Vtable::as_raw(self), endevice, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_PreferredAudioDevice)(::windows::core::Vtable::as_raw(self), endevice, ::core::mem::transmute_copy(bstrdevicename)).ok()
    }
    pub unsafe fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_PreferredAudioDevice)(::windows::core::Vtable::as_raw(self), endevice, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_PreferredVolume)(::windows::core::Vtable::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_PreferredVolume)(::windows::core::Vtable::as_raw(self), endevice, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredAEC<P0>(&self, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPreferredAEC)(::windows::core::Vtable::as_raw(self), benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreferredAEC(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PreferredAEC)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPreferredVideoDevice(&self, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPreferredVideoDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdevicename)).ok()
    }
    pub unsafe fn PreferredVideoDevice(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PreferredVideoDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ActiveMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActiveMedia)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxBitrate)(::windows::core::Vtable::as_raw(self), lmaxbitrate).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MaxBitrate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTemporalSpatialTradeOff)(::windows::core::Vtable::as_raw(self), lvalue).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TemporalSpatialTradeOff)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NetworkQuality(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NetworkQuality)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StartT120Applet)(::windows::core::Vtable::as_raw(self), enapplet).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StopT120Applets)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_IsT120AppletRunning)(::windows::core::Vtable::as_raw(self), enapplet, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocalUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalUserURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalUserURI(&self, bstruseruri: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLocalUserURI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstruseruri)).ok()
    }
    pub unsafe fn LocalUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalUserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalUserName(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLocalUserName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PlayRing<P0>(&self, entype: RTC_RING_TYPE, bplay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.PlayRing)(::windows::core::Vtable::as_raw(self), entype, bplay.into()).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendDTMF)(::windows::core::Vtable::as_raw(self), endtmf).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InvokeTuningWizard)(::windows::core::Vtable::as_raw(self), hwndparent).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTuned(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsTuned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCClientPresence2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnablePresence<P0>(&self, fusestorage: P0, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnablePresence)(::windows::core::Vtable::as_raw(self), fusestorage.into(), ::core::mem::transmute(varstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Export(&self, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Export)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<P0>(&self, varstorage: super::Com::VARIANT, freplaceall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Import)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varstorage), freplaceall.into()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateBuddies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Buddies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Buddy(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Buddy)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddBuddy<P0, P1>(&self, bstrpresentityuri: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR, fpersistent: P0, pprofile: P1, lflags: i32) -> ::windows::core::Result<IRTCBuddy>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IRTCProfile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddBuddy)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri), ::core::mem::transmute_copy(bstrusername), ::core::mem::transmute_copy(bstrdata), fpersistent.into(), pprofile.into().abi(), lflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRTCBuddy>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveBuddy)(::windows::core::Vtable::as_raw(self), pbuddy.into().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> ::windows::core::Result<IRTCEnumWatchers> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateWatchers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Watchers(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Watchers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Watcher(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Watcher)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddWatcher<P0, P1>(&self, bstrpresentityuri: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR, fblocked: P0, fpersistent: P1) -> ::windows::core::Result<IRTCWatcher>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddWatcher)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri), ::core::mem::transmute_copy(bstrusername), ::core::mem::transmute_copy(bstrdata), fblocked.into(), fpersistent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveWatcher<P0>(&self, pwatcher: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRTCWatcher>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveWatcher)(::windows::core::Vtable::as_raw(self), pwatcher.into().abi()).ok()
    }
    pub unsafe fn SetLocalPresenceInfo(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLocalPresenceInfo)(::windows::core::Vtable::as_raw(self), enstatus, ::core::mem::transmute_copy(bstrnotes)).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OfferWatcherMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOfferWatcherMode)(::windows::core::Vtable::as_raw(self), enmode).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> ::windows::core::Result<RTC_PRIVACY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PrivacyMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivacyMode)(::windows::core::Vtable::as_raw(self), enmode).ok()
    }
}
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
impl IRTCClientProvisioning2 {
    pub unsafe fn CreateProfile(&self, bstrprofilexml: &::windows::core::BSTR) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateProfile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprofilexml), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnableProfile<P0>(&self, pprofile: P0, lregisterflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRTCProfile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnableProfile)(::windows::core::Vtable::as_raw(self), pprofile.into().abi(), lregisterflags).ok()
    }
    pub unsafe fn DisableProfile<P0>(&self, pprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRTCProfile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DisableProfile)(::windows::core::Vtable::as_raw(self), pprofile.into().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> ::windows::core::Result<IRTCEnumProfiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateProfiles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Profiles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProfile(&self, bstruseraccount: &::windows::core::BSTR, bstruserpassword: &::windows::core::BSTR, bstruseruri: &::windows::core::BSTR, bstrserver: &::windows::core::BSTR, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProfile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstruseraccount), ::core::mem::transmute_copy(bstruserpassword), ::core::mem::transmute_copy(bstruseruri), ::core::mem::transmute_copy(bstrserver), ltransport, lcookie).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCProfile2 {
    pub unsafe fn Key(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Key)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn XML(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.XML)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_ProviderURI)(::windows::core::Vtable::as_raw(self), enuri, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProviderData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientBanner(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientBanner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClientMinVer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientMinVer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClientCurVer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientCurVer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClientUpdateURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientUpdateURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClientData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserAccount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCredentials(&self, bstruseruri: &::windows::core::BSTR, bstruseraccount: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCredentials)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstruseruri), ::core::mem::transmute_copy(bstruseraccount), ::core::mem::transmute_copy(bstrpassword)).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCProfileEvent2 {
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Profile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Cookie)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StatusCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCSession2 {
    pub unsafe fn Client(&self) -> ::windows::core::Result<IRTCClient> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Client)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<RTC_SESSION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Profile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Participants(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Participants)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Answer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Answer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), enreason).ok()
    }
    pub unsafe fn Redirect<P0>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &::windows::core::BSTR, pprofile: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRTCProfile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Redirect)(::windows::core::Vtable::as_raw(self), entype, ::core::mem::transmute_copy(bstrlocalphoneuri), pprofile.into().abi(), lflags).ok()
    }
    pub unsafe fn AddParticipant(&self, bstraddress: &::windows::core::BSTR, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddParticipant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstraddress), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveParticipant<P0>(&self, pparticipant: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRTCParticipant>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveParticipant)(::windows::core::Vtable::as_raw(self), pparticipant.into().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> ::windows::core::Result<IRTCEnumParticipants> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateParticipants)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanAddParticipants(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanAddParticipants)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RedirectedUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RedirectedUserURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RedirectedUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RedirectedUserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NextRedirectedUser(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NextRedirectedUser)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage(&self, bstrmessageheader: &::windows::core::BSTR, bstrmessage: &::windows::core::BSTR, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmessageheader), ::core::mem::transmute_copy(bstrmessage), lcookie).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendMessageStatus)(::windows::core::Vtable::as_raw(self), enuserstatus, lcookie).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveStream)(::windows::core::Vtable::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn put_EncryptionKey(&self, lmediatype: i32, encryptionkey: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_EncryptionKey)(::windows::core::Vtable::as_raw(self), lmediatype, ::core::mem::transmute_copy(encryptionkey)).ok()
    }
}
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
impl IRTCSessionOperationCompleteEvent2 {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Session)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Cookie)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StatusCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StatusText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCSessionStateChangeEvent2 {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Session)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StatusCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StatusText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl IRTCWatcher {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PresentityURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPresentityURI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Data)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetData(&self, bstrdata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Persistent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPersistent)(::windows::core::Vtable::as_raw(self), fpersistent.into()).ok()
    }
}
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
impl IRTCWatcher2 {
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PresentityURI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPresentityURI(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPresentityURI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpresentityuri)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Data)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetData(&self, bstrdata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Persistent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPersistent)(::windows::core::Vtable::as_raw(self), fpersistent.into()).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_WATCHER_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetState)(::windows::core::Vtable::as_raw(self), enstate).ok()
    }
}
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
impl IRTCWatcherEvent2 {
    pub unsafe fn Watcher(&self) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Watcher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
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
impl ::core::default::Default for RTC_ACE_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_ACE_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ACE_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_ANSWER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_ANSWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ANSWER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_AUDIO_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_AUDIO_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_AUDIO_DEVICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_BUDDY_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_BUDDY_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_SUBSCRIPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_CLIENT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_CLIENT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_CLIENT_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_DTMF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_DTMF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_DTMF").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_GROUP_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_GROUP_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_GROUP_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_LISTEN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_LISTEN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_LISTEN_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_MEDIA_EVENT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_MEDIA_EVENT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_MEDIA_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_MEDIA_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_MESSAGING_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_MESSAGING_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_MESSAGING_USER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_MESSAGING_USER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_USER_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_OFFER_WATCHER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_OFFER_WATCHER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_OFFER_WATCHER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_PARTICIPANT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_PARTICIPANT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PARTICIPANT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_PRESENCE_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_PRESENCE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_PRESENCE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_PRESENCE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_PRIVACY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_PRIVACY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PRIVACY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_PROFILE_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_PROFILE_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PROFILE_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_PROVIDER_URI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_PROVIDER_URI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_PROVIDER_URI").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_REGISTRATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_REGISTRATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_REGISTRATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_REINVITE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_REINVITE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_REINVITE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_RING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_RING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_RING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_ROAMING_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_ROAMING_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_ROAMING_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_SECURITY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_SECURITY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_SECURITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_SECURITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_SESSION_REFER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_SESSION_REFER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_REFER_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_SESSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_SESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_SESSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_T120_APPLET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_T120_APPLET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_T120_APPLET").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_TERMINATE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_TERMINATE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_TERMINATE_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_USER_SEARCH_COLUMN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_USER_SEARCH_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_COLUMN").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_USER_SEARCH_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_USER_SEARCH_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_PREFERENCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_VIDEO_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_VIDEO_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_VIDEO_DEVICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_WATCHER_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_WATCHER_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_WATCHER_MATCH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_WATCHER_MATCH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_MATCH_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTC_WATCHER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTC_WATCHER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for TRANSPORT_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_SETTING").field("SettingId", &self.SettingId).field("Length", &self.Length).field("Value", &self.Value).finish()
    }
}
