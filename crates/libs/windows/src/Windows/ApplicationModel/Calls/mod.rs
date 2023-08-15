#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallAnswerEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICallAnswerEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICallAnswerEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd789617_2dd7_4c8c_b2bd_95d17a5bb733);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallAnswerEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AcceptedMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallMedia) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallRejectEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICallRejectEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICallRejectEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda47fad7_13d4_4d92_a1c2_b77811ee37ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallRejectEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RejectReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallRejectReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallStateChangeEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICallStateChangeEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICallStateChangeEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeab2349e_66f5_47f9_9fb5_459c5198c720);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallStateChangeEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenCallEndCallDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_Vtbl;
}
impl ::core::clone::Clone for ILockScreenCallEndCallDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILockScreenCallEndCallDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dd7ed0d_98ed_4041_9632_50ff812b773f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndCallDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenCallEndRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILockScreenCallEndRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILockScreenCallEndRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8190a363_6f27_46e9_aeb6_c0ae83e47dc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallEndRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenCallUI(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILockScreenCallUI {
    type Vtable = ILockScreenCallUI_Vtbl;
}
impl ::core::clone::Clone for ILockScreenCallUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILockScreenCallUI {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc596fd8d_73c9_4a14_b021_ec1c50a3b727);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallUI_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Dismiss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CallTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCallTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMuteChangeEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMuteChangeEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMuteChangeEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8585e159_0c41_432c_814d_c5f1fdf530be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Muted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCall(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCall {
    type Vtable = IPhoneCall_Vtbl;
}
impl ::core::clone::Clone for IPhoneCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCall {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc14ed0f8_c17d_59d2_9628_66e545b6cd21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCall_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AudioDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsMutedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsMutedChanged: usize,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallStatus) -> ::windows_core::HRESULT,
    pub AudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallAudioDevice) -> ::windows_core::HRESULT,
    pub GetPhoneCallInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPhoneCallInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPhoneCallInfoAsync: usize,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndAsync: usize,
    pub SendDtmfKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendDtmfKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendDtmfKeyAsync: usize,
    pub AcceptIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AcceptIncomingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptIncomingAsync: usize,
    pub Hold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HoldAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldAsync: usize,
    pub ResumeFromHold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResumeFromHoldAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeFromHoldAsync: usize,
    pub Mute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MuteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteAsync: usize,
    pub Unmute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnmuteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnmuteAsync: usize,
    pub RejectIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RejectIncomingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RejectIncomingAsync: usize,
    pub ChangeAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioDevice, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ChangeAudioDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioDevice, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeAudioDeviceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallBlockingStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallBlockingStatics {
    type Vtable = IPhoneCallBlockingStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallBlockingStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallBlockingStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19646f84_2b79_26f1_a46f_694be043f313);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockingStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BlockUnknownNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBlockUnknownNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub BlockPrivateNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBlockPrivateNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetCallBlockingListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumberlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetCallBlockingListAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryEntry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfab0e129_32a4_4b85_83d1_f90d8c23a857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsCallerIdBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCallerIdBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsEmergency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEmergency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsMissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRinging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRinging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsVoicemail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsVoicemail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Media: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryMedia) -> ::windows_core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryMedia) -> ::windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SourceIdKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistorySourceIdKind) -> ::windows_core::HRESULT,
    pub SetSourceIdKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistorySourceIdKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryEntryAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryEntryAddress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30f159da_3955_4042_84e6_66eebf82e67f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddress_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RawAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRawAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RawAddressKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryRawAddressKind) -> ::windows_core::HRESULT,
    pub SetRawAddressKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddressFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryEntryAddressFactory {
    type Vtable = IPhoneCallHistoryEntryAddressFactory_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryEntryAddressFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryEntryAddressFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb0fadba_c7f0_4bb6_9f6b_ba5d73209aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryAddressFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, rawaddresskind: PhoneCallHistoryEntryRawAddressKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryQueryOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryEntryQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryEntryQueryOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c5fe15c_8bed_40ca_b06e_c4ca8eae5c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryQueryOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DesiredMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows_core::HRESULT,
    pub SetDesiredMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryEntryReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryEntryReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61ece4be_8d86_479f_8404_a9846920fee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryEntryReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryManagerForUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd925c523_f55f_4353_9db4_0205a5265a55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryManagerStatics {
    type Vtable = IPhoneCallHistoryManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5a6da39_b31f_4f45_ac8e_1b08893c1b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryManagerStatics2 {
    type Vtable = IPhoneCallHistoryManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefd474f0_a2db_4188_9e92_bc3cfa6813cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallHistoryStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallHistoryStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallHistoryStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f907db8_b40e_422b_8545_cb1910a61c52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallHistoryStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentryid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetEntryAsync: usize,
    pub GetEntryReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEntryReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveEntryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteEntryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteEntriesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentries: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteEntriesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkEntryAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkEntryAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkEntriesAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callhistoryentries: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkEntriesAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetUnseenCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUnseenCountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkAllAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAllAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSourcesUnseenCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSourcesUnseenCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkSourcesAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkSourcesAsSeenAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallInfo {
    type Vtable = IPhoneCallInfo_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22b42577_3e4d_5dc6_89c2_469fe5ffc189);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IsHoldSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallDirection) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallManagerStatics {
    type Vtable = IPhoneCallManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60edac78_78a6_4872_a3ef_98325ec8b843);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowPhoneCallUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallManagerStatics2 {
    type Vtable = IPhoneCallManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7e3c8bc_2370_431c_98fd_43be5f03086d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CallStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CallStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCallStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCallStateChanged: usize,
    pub IsCallActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCallIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShowPhoneCallSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallStatics {
    type Vtable = IPhoneCallStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2218eeab_f60b_53e7_ba13_5aeafbc22957);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallStore {
    type Vtable = IPhoneCallStore_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f610748_18a6_4173_86d1_28be9dc62dba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IsEmergencyPhoneNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsEmergencyPhoneNumberAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultLineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultLineAsync: usize,
    pub RequestLineWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallVideoCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallVideoCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02382786_b16a_4fdb_be3b_c4240e13ad0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsVideoCallingCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallVideoCapabilitiesManagerStatics {
    type Vtable = IPhoneCallVideoCapabilitiesManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallVideoCapabilitiesManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallVideoCapabilitiesManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3c64b56_f00b_4a1c_a0c6_ee1910749ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallsResult {
    type Vtable = IPhoneCallsResult_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bfad365_57cf_57dd_986d_b057c91eac33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllActivePhoneCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllActivePhoneCalls: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneDialOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneDialOptions {
    type Vtable = IPhoneDialOptions_Vtbl;
}
impl ::core::clone::Clone for IPhoneDialOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneDialOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb639c4b8_f06f_36cb_a863_823742b5f2d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneDialOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPhone: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContactPhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContactPhone: usize,
    pub Media: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallMedia) -> ::windows_core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallMedia) -> ::windows_core::HRESULT,
    pub AudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneAudioRoutingEndpoint) -> ::windows_core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneAudioRoutingEndpoint) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLine {
    type Vtable = IPhoneLine_Vtbl;
}
impl ::core::clone::Clone for IPhoneLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27c66f30_6a69_34ca_a2ba_65302530c311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LineChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineChanged: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub NetworkState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneNetworkState) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Voicemail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NetworkName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CellularDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Transport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineTransport) -> ::windows_core::HRESULT,
    pub CanDial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SupportsTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub VideoCallingCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsImmediateDialNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsImmediateDialNumberAsync: usize,
    pub Dial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DialWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLine2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLine2 {
    type Vtable = IPhoneLine2_Vtbl;
}
impl ::core::clone::Clone for IPhoneLine2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLine2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0167f56a_5344_5d64_8af3_a31a950e916a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub EnableTextReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EnableTextReply: usize,
    pub TransportDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLine3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLine3 {
    type Vtable = IPhoneLine3_Vtbl;
}
impl ::core::clone::Clone for IPhoneLine3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLine3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2e33cf7_2406_57f3_826a_e5a5f40d6fb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLine3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DialWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DialWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialWithResultAsync: usize,
    pub GetAllActivePhoneCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAllActivePhoneCallsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllActivePhoneCallsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineCellularDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineCellularDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineCellularDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x192601d5_147c_4769_b673_98a5ec8426cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineCellularDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SimState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneSimState) -> ::windows_core::HRESULT,
    pub SimSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub IsModemOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RegistrationRejectCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetNetworkOperatorDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: PhoneLineNetworkOperatorDisplayTextLocation, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe265862_f64f_4312_b2a8_4e257721aa95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsVideoCallingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDialResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineDialResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineDialResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe825a30a_5c7f_546f_b918_3ad2fe70fb34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDialResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DialCallStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows_core::HRESULT,
    pub DialedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineStatics {
    type Vtable = IPhoneLineStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf38b5f23_ceb0_404f_bcf2_ba9f697d8adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineTransportDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineTransportDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineTransportDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefa8f889_cffa_59f4_97e4_74705b7dc490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Transport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineTransport) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    pub RegisterApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub RegisterAppForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    RegisterAppForUser: usize,
    pub UnregisterApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub UnregisterAppForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    UnregisterAppForUser: usize,
    pub IsRegistered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineTransportDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineTransportDevice2 {
    type Vtable = IPhoneLineTransportDevice2_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineTransportDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineTransportDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64c885f2_ecf4_5761_8c04_3c248ce61690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDevice2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AudioRoutingStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TransportDeviceAudioRoutingStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioRoutingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioRoutingStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioRoutingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioRoutingStatusChanged: usize,
    pub InBandRingingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InBandRingingEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InBandRingingEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInBandRingingEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInBandRingingEnabledChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineTransportDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineTransportDeviceStatics {
    type Vtable = IPhoneLineTransportDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineTransportDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineTransportDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f3121ac_d609_51a1_96f3_fb00d1819252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineTransportDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorForPhoneLineTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transport: PhoneLineTransport, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a45cd0a_6323_44e0_a6f6_9f21f64dc90a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LineAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineAdded: usize,
    #[cfg(feature = "Foundation")]
    pub LineRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub LineUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineWatcherStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineWatcherEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineWatcherEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd07c753e_9e12_4a37_82b7_ad535dad6a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineWatcherEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneVoicemail(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneVoicemail {
    type Vtable = IPhoneVoicemail_Vtbl;
}
impl ::core::clone::Clone for IPhoneVoicemail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneVoicemail {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9ce77f6_6e9f_3a8b_b727_6e0cf6998224);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneVoicemail_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Number: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneVoicemailType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DialVoicemailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialVoicemailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_Vtbl;
}
impl ::core::clone::Clone for IVoipCallCoordinator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipCallCoordinator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f118bcf_e8ef_4434_9c5f_a8d893fafe79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReserveCallResourcesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskentrypoint: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReserveCallResourcesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MuteStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mutechangehandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMuteStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMuteStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestNewIncomingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactnumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactimage: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, brandingimage: *mut ::core::ffi::c_void, calldetails: ::std::mem::MaybeUninit<::windows_core::HSTRING>, ringtone: *mut ::core::ffi::c_void, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewIncomingCall: usize,
    pub RequestNewOutgoingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, servicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyUnmuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestOutgoingUpgradeToVideoCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows_core::GUID, context: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, servicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestIncomingUpgradeToVideoCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactnumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactimage: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, brandingimage: *mut ::core::ffi::c_void, calldetails: ::std::mem::MaybeUninit<::windows_core::HSTRING>, ringtone: *mut ::core::ffi::c_void, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestIncomingUpgradeToVideoCall: usize,
    pub TerminateCellularCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CancelUpgrade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipCallCoordinator2 {
    type Vtable = IVoipCallCoordinator2_Vtbl;
}
impl ::core::clone::Clone for IVoipCallCoordinator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipCallCoordinator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbeb4a9f3_c704_4234_89ce_e88cc0d28fbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetupNewAcceptedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactnumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, servicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipCallCoordinator3 {
    type Vtable = IVoipCallCoordinator3_Vtbl;
}
impl ::core::clone::Clone for IVoipCallCoordinator3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipCallCoordinator3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x338d0cbf_9b55_4021_87ca_e64b9bd666c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestNewAppInitiatedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactnumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, servicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestNewIncomingCallWithContactRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactnumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contactimage: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, brandingimage: *mut ::core::ffi::c_void, calldetails: ::std::mem::MaybeUninit<::windows_core::HSTRING>, ringtone: *mut ::core::ffi::c_void, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, contactremoteid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewIncomingCallWithContactRemoteId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinator4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipCallCoordinator4 {
    type Vtable = IVoipCallCoordinator4_Vtbl;
}
impl ::core::clone::Clone for IVoipCallCoordinator4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipCallCoordinator4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83737239_9311_468f_bb49_47e0dfb5d93e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinator4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReserveOneProcessCallResourcesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReserveOneProcessCallResourcesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipCallCoordinatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipCallCoordinatorStatics {
    type Vtable = IVoipCallCoordinatorStatics_Vtbl;
}
impl ::core::clone::Clone for IVoipCallCoordinatorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipCallCoordinatorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f5d1f2b_e04a_4d10_b31a_a55c922cc2fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipCallCoordinatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipPhoneCall(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipPhoneCall {
    type Vtable = IVoipPhoneCall_Vtbl;
}
impl ::core::clone::Clone for IVoipPhoneCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipPhoneCall {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cf1f19a_7794_4a5a_8c68_ae87947a6990);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub EndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEndRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HoldRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResumeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResumeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accepthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RejectRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rejecthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RejectRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRejectRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRejectRequested: usize,
    pub NotifyCallHeld: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyCallActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyCallEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    pub CallMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallMedia) -> ::windows_core::HRESULT,
    pub SetCallMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VoipPhoneCallMedia) -> ::windows_core::HRESULT,
    pub NotifyCallReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipPhoneCall2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipPhoneCall2 {
    type Vtable = IVoipPhoneCall2_Vtbl;
}
impl ::core::clone::Clone for IVoipPhoneCall2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipPhoneCall2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x741b46e1_245f_41f3_9399_3141d25b52e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryShowAppUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoipPhoneCall3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoipPhoneCall3 {
    type Vtable = IVoipPhoneCall3_Vtbl;
}
impl ::core::clone::Clone for IVoipPhoneCall3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoipPhoneCall3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d891522_e258_4aa9_907a_1aa413c25523);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoipPhoneCall3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NotifyCallAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, media: VoipPhoneCallMedia) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct CallAnswerEventArgs(::windows_core::IUnknown);
impl CallAnswerEventArgs {
    pub fn AcceptedMedia(&self) -> ::windows_core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcceptedMedia)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CallAnswerEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallAnswerEventArgs {}
impl ::core::fmt::Debug for CallAnswerEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallAnswerEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CallAnswerEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallAnswerEventArgs;{fd789617-2dd7-4c8c-b2bd-95d17a5bb733})");
}
impl ::core::clone::Clone for CallAnswerEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CallAnswerEventArgs {
    const IID: ::windows_core::GUID = <ICallAnswerEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CallAnswerEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallAnswerEventArgs";
}
::windows_core::imp::interface_hierarchy!(CallAnswerEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CallAnswerEventArgs {}
unsafe impl ::core::marker::Sync for CallAnswerEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct CallRejectEventArgs(::windows_core::IUnknown);
impl CallRejectEventArgs {
    pub fn RejectReason(&self) -> ::windows_core::Result<VoipPhoneCallRejectReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RejectReason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CallRejectEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallRejectEventArgs {}
impl ::core::fmt::Debug for CallRejectEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallRejectEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CallRejectEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallRejectEventArgs;{da47fad7-13d4-4d92-a1c2-b77811ee37ec})");
}
impl ::core::clone::Clone for CallRejectEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CallRejectEventArgs {
    const IID: ::windows_core::GUID = <ICallRejectEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CallRejectEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallRejectEventArgs";
}
::windows_core::imp::interface_hierarchy!(CallRejectEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CallRejectEventArgs {}
unsafe impl ::core::marker::Sync for CallRejectEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct CallStateChangeEventArgs(::windows_core::IUnknown);
impl CallStateChangeEventArgs {
    pub fn State(&self) -> ::windows_core::Result<VoipPhoneCallState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CallStateChangeEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallStateChangeEventArgs {}
impl ::core::fmt::Debug for CallStateChangeEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallStateChangeEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CallStateChangeEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.CallStateChangeEventArgs;{eab2349e-66f5-47f9-9fb5-459c5198c720})");
}
impl ::core::clone::Clone for CallStateChangeEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CallStateChangeEventArgs {
    const IID: ::windows_core::GUID = <ICallStateChangeEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CallStateChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallStateChangeEventArgs";
}
::windows_core::imp::interface_hierarchy!(CallStateChangeEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CallStateChangeEventArgs {}
unsafe impl ::core::marker::Sync for CallStateChangeEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct LockScreenCallEndCallDeferral(::windows_core::IUnknown);
impl LockScreenCallEndCallDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for LockScreenCallEndCallDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallEndCallDeferral {}
impl ::core::fmt::Debug for LockScreenCallEndCallDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallEndCallDeferral").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LockScreenCallEndCallDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral;{2dd7ed0d-98ed-4041-9632-50ff812b773f})");
}
impl ::core::clone::Clone for LockScreenCallEndCallDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LockScreenCallEndCallDeferral {
    const IID: ::windows_core::GUID = <ILockScreenCallEndCallDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenCallEndCallDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral";
}
::windows_core::imp::interface_hierarchy!(LockScreenCallEndCallDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LockScreenCallEndCallDeferral {}
unsafe impl ::core::marker::Sync for LockScreenCallEndCallDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct LockScreenCallEndRequestedEventArgs(::windows_core::IUnknown);
impl LockScreenCallEndRequestedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<LockScreenCallEndCallDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LockScreenCallEndRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallEndRequestedEventArgs {}
impl ::core::fmt::Debug for LockScreenCallEndRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallEndRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LockScreenCallEndRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs;{8190a363-6f27-46e9-aeb6-c0ae83e47dc7})");
}
impl ::core::clone::Clone for LockScreenCallEndRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LockScreenCallEndRequestedEventArgs {
    const IID: ::windows_core::GUID = <ILockScreenCallEndRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenCallEndRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LockScreenCallEndRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LockScreenCallEndRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallEndRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct LockScreenCallUI(::windows_core::IUnknown);
impl LockScreenCallUI {
    pub fn Dismiss(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Dismiss)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEndRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEndRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CallTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCallTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCallTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for LockScreenCallUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallUI {}
impl ::core::fmt::Debug for LockScreenCallUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallUI").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LockScreenCallUI {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.LockScreenCallUI;{c596fd8d-73c9-4a14-b021-ec1c50a3b727})");
}
impl ::core::clone::Clone for LockScreenCallUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LockScreenCallUI {
    type Vtable = ILockScreenCallUI_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LockScreenCallUI {
    const IID: ::windows_core::GUID = <ILockScreenCallUI as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenCallUI {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallUI";
}
::windows_core::imp::interface_hierarchy!(LockScreenCallUI, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LockScreenCallUI {}
unsafe impl ::core::marker::Sync for LockScreenCallUI {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct MuteChangeEventArgs(::windows_core::IUnknown);
impl MuteChangeEventArgs {
    pub fn Muted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Muted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MuteChangeEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MuteChangeEventArgs {}
impl ::core::fmt::Debug for MuteChangeEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MuteChangeEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MuteChangeEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.MuteChangeEventArgs;{8585e159-0c41-432c-814d-c5f1fdf530be})");
}
impl ::core::clone::Clone for MuteChangeEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MuteChangeEventArgs {
    const IID: ::windows_core::GUID = <IMuteChangeEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MuteChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.MuteChangeEventArgs";
}
::windows_core::imp::interface_hierarchy!(MuteChangeEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MuteChangeEventArgs {}
unsafe impl ::core::marker::Sync for MuteChangeEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCall(::windows_core::IUnknown);
impl PhoneCall {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioDeviceChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioDeviceChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioDeviceChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsMutedChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMutedChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsMutedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsMutedChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CallId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMuted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<PhoneCallStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AudioDevice(&self) -> ::windows_core::Result<PhoneCallAudioDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioDevice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPhoneCallInfo(&self) -> ::windows_core::Result<PhoneCallInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPhoneCallInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPhoneCallInfoAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPhoneCallInfoAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn End(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).End)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SendDtmfKey(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendDtmfKey)(::windows_core::Interface::as_raw(this), key, dtmftoneaudioplayback, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendDtmfKeyAsync(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendDtmfKeyAsync)(::windows_core::Interface::as_raw(this), key, dtmftoneaudioplayback, &mut result__).from_abi(result__)
        }
    }
    pub fn AcceptIncoming(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcceptIncoming)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceptIncomingAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcceptIncomingAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Hold(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hold)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HoldAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HoldAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResumeFromHold(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResumeFromHold)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeFromHoldAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResumeFromHoldAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mute(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MuteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MuteAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Unmute(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Unmute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnmuteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnmuteAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RejectIncoming(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RejectIncoming)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RejectIncomingAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RejectIncomingAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeAudioDevice(&self, endpoint: PhoneCallAudioDevice) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeAudioDevice)(::windows_core::Interface::as_raw(this), endpoint, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChangeAudioDeviceAsync(&self, endpoint: PhoneCallAudioDevice) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeAudioDeviceAsync)(::windows_core::Interface::as_raw(this), endpoint, &mut result__).from_abi(result__)
        }
    }
    pub fn GetFromId(callid: &::windows_core::HSTRING) -> ::windows_core::Result<PhoneCall> {
        Self::IPhoneCallStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(callid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallStatics<R, F: FnOnce(&IPhoneCallStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCall, IPhoneCallStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PhoneCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCall {}
impl ::core::fmt::Debug for PhoneCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCall").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCall {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCall;{c14ed0f8-c17d-59d2-9628-66e545b6cd21})");
}
impl ::core::clone::Clone for PhoneCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCall {
    type Vtable = IPhoneCall_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCall {
    const IID: ::windows_core::GUID = <IPhoneCall as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCall";
}
::windows_core::imp::interface_hierarchy!(PhoneCall, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCall {}
unsafe impl ::core::marker::Sync for PhoneCall {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallBlocking;
impl PhoneCallBlocking {
    pub fn BlockUnknownNumbers() -> ::windows_core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BlockUnknownNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetBlockUnknownNumbers(value: bool) -> ::windows_core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetBlockUnknownNumbers)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn BlockPrivateNumbers() -> ::windows_core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BlockPrivateNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetBlockPrivateNumbers(value: bool) -> ::windows_core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetBlockPrivateNumbers)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetCallBlockingListAsync<P0>(phonenumberlist: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetCallBlockingListAsync)(::windows_core::Interface::as_raw(this), phonenumberlist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallBlockingStatics<R, F: FnOnce(&IPhoneCallBlockingStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallBlocking, IPhoneCallBlockingStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PhoneCallBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallBlocking";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntry(::windows_core::IUnknown);
impl PhoneCallHistoryEntry {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallHistoryEntry, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Address(&self) -> ::windows_core::Result<PhoneCallHistoryEntryAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAddress<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PhoneCallHistoryEntryAddress>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsCallerIdBlocked(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCallerIdBlocked)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCallerIdBlocked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCallerIdBlocked)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEmergency(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEmergency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEmergency(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEmergency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsIncoming(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsIncoming)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsIncoming(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsIncoming)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMissed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMissed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsMissed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMissed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRinging(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRinging)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsRinging(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRinging)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSeen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSeen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSeen)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSuppressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSuppressed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsSuppressed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSuppressed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVoicemail(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVoicemail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsVoicemail(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsVoicemail)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Media(&self) -> ::windows_core::Result<PhoneCallHistoryEntryMedia> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Media)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMedia(&self, value: PhoneCallHistoryEntryMedia) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMedia)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows_core::Result<PhoneCallHistoryEntryOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppReadAccess)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppReadAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSourceId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceIdKind(&self) -> ::windows_core::Result<PhoneCallHistorySourceIdKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceIdKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSourceIdKind(&self, value: PhoneCallHistorySourceIdKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceIdKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntry {}
impl ::core::fmt::Debug for PhoneCallHistoryEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntry").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntry;{fab0e129-32a4-4b85-83d1-f90d8c23a857})");
}
impl ::core::clone::Clone for PhoneCallHistoryEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallHistoryEntry {
    const IID: ::windows_core::GUID = <IPhoneCallHistoryEntry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallHistoryEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntry";
}
::windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntry, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallHistoryEntry {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntry {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryAddress(::windows_core::IUnknown);
impl PhoneCallHistoryEntryAddress {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallHistoryEntryAddress, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ContactId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContactId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContactId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RawAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRawAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRawAddress)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RawAddressKind(&self) -> ::windows_core::Result<PhoneCallHistoryEntryRawAddressKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawAddressKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRawAddressKind(&self, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRawAddressKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(rawaddress: &::windows_core::HSTRING, rawaddresskind: PhoneCallHistoryEntryRawAddressKind) -> ::windows_core::Result<PhoneCallHistoryEntryAddress> {
        Self::IPhoneCallHistoryEntryAddressFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(rawaddress), rawaddresskind, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryEntryAddressFactory<R, F: FnOnce(&IPhoneCallHistoryEntryAddressFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallHistoryEntryAddress, IPhoneCallHistoryEntryAddressFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntryAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntryAddress {}
impl ::core::fmt::Debug for PhoneCallHistoryEntryAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryAddress").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntryAddress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress;{30f159da-3955-4042-84e6-66eebf82e67f})");
}
impl ::core::clone::Clone for PhoneCallHistoryEntryAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallHistoryEntryAddress {
    const IID: ::windows_core::GUID = <IPhoneCallHistoryEntryAddress as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallHistoryEntryAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress";
}
::windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntryAddress, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryAddress {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryAddress {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryOptions(::windows_core::IUnknown);
impl PhoneCallHistoryEntryQueryOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallHistoryEntryQueryOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DesiredMedia(&self) -> ::windows_core::Result<PhoneCallHistoryEntryQueryDesiredMedia> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredMedia)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredMedia(&self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredMedia)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntryQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntryQueryOptions {}
impl ::core::fmt::Debug for PhoneCallHistoryEntryQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryQueryOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntryQueryOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions;{9c5fe15c-8bed-40ca-b06e-c4ca8eae5c87})");
}
impl ::core::clone::Clone for PhoneCallHistoryEntryQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallHistoryEntryQueryOptions {
    const IID: ::windows_core::GUID = <IPhoneCallHistoryEntryQueryOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallHistoryEntryQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions";
}
::windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntryQueryOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryQueryOptions {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryReader(::windows_core::IUnknown);
impl PhoneCallHistoryEntryReader {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryEntryReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryEntryReader {}
impl ::core::fmt::Debug for PhoneCallHistoryEntryReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntryReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader;{61ece4be-8d86-479f-8404-a9846920fee6})");
}
impl ::core::clone::Clone for PhoneCallHistoryEntryReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallHistoryEntryReader {
    const IID: ::windows_core::GUID = <IPhoneCallHistoryEntryReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallHistoryEntryReader {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader";
}
::windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntryReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallHistoryEntryReader {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryEntryReader {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallHistoryManager;
impl PhoneCallHistoryManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(accesstype: PhoneCallHistoryStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        Self::IPhoneCallHistoryManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<PhoneCallHistoryManagerForUser>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IPhoneCallHistoryManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryManagerStatics<R, F: FnOnce(&IPhoneCallHistoryManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryManagerStatics2<R, F: FnOnce(&IPhoneCallHistoryManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PhoneCallHistoryManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManager";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryManagerForUser(::windows_core::IUnknown);
impl PhoneCallHistoryManagerForUser {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryManagerForUser {}
impl ::core::fmt::Debug for PhoneCallHistoryManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryManagerForUser").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryManagerForUser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser;{d925c523-f55f-4353-9db4-0205a5265a55})");
}
impl ::core::clone::Clone for PhoneCallHistoryManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallHistoryManagerForUser {
    const IID: ::windows_core::GUID = <IPhoneCallHistoryManagerForUser as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallHistoryManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser";
}
::windows_core::imp::interface_hierarchy!(PhoneCallHistoryManagerForUser, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallHistoryManagerForUser {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryManagerForUser {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryStore(::windows_core::IUnknown);
impl PhoneCallHistoryStore {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetEntryAsync(&self, callhistoryentryid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEntryAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(callhistoryentryid), &mut result__).from_abi(result__)
        }
    }
    pub fn GetEntryReader(&self) -> ::windows_core::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEntryReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetEntryReaderWithOptions<P0>(&self, queryoptions: P0) -> ::windows_core::Result<PhoneCallHistoryEntryReader>
    where
        P0: ::windows_core::IntoParam<PhoneCallHistoryEntryQueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEntryReaderWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveEntryAsync<P0>(&self, callhistoryentry: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<PhoneCallHistoryEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveEntryAsync)(::windows_core::Interface::as_raw(this), callhistoryentry.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteEntryAsync<P0>(&self, callhistoryentry: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<PhoneCallHistoryEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteEntryAsync)(::windows_core::Interface::as_raw(this), callhistoryentry.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeleteEntriesAsync<P0>(&self, callhistoryentries: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteEntriesAsync)(::windows_core::Interface::as_raw(this), callhistoryentries.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkEntryAsSeenAsync<P0>(&self, callhistoryentry: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<PhoneCallHistoryEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarkEntryAsSeenAsync)(::windows_core::Interface::as_raw(this), callhistoryentry.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkEntriesAsSeenAsync<P0>(&self, callhistoryentries: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarkEntriesAsSeenAsync)(::windows_core::Interface::as_raw(this), callhistoryentries.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetUnseenCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUnseenCountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAllAsSeenAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarkAllAsSeenAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSourcesUnseenCountAsync<P0>(&self, sourceids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSourcesUnseenCountAsync)(::windows_core::Interface::as_raw(this), sourceids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkSourcesAsSeenAsync<P0>(&self, sourceids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarkSourcesAsSeenAsync)(::windows_core::Interface::as_raw(this), sourceids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallHistoryStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallHistoryStore {}
impl ::core::fmt::Debug for PhoneCallHistoryStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryStore").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallHistoryStore;{2f907db8-b40e-422b-8545-cb1910a61c52})");
}
impl ::core::clone::Clone for PhoneCallHistoryStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallHistoryStore {
    const IID: ::windows_core::GUID = <IPhoneCallHistoryStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallHistoryStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryStore";
}
::windows_core::imp::interface_hierarchy!(PhoneCallHistoryStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallHistoryStore {}
unsafe impl ::core::marker::Sync for PhoneCallHistoryStore {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallInfo(::windows_core::IUnknown);
impl PhoneCallInfo {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsHoldSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHoldSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallDirection(&self) -> ::windows_core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallDirection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallInfo {}
impl ::core::fmt::Debug for PhoneCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallInfo;{22b42577-3e4d-5dc6-89c2-469fe5ffc189})");
}
impl ::core::clone::Clone for PhoneCallInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallInfo {
    type Vtable = IPhoneCallInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallInfo {
    const IID: ::windows_core::GUID = <IPhoneCallInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallInfo";
}
::windows_core::imp::interface_hierarchy!(PhoneCallInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallInfo {}
unsafe impl ::core::marker::Sync for PhoneCallInfo {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallManager;
impl PhoneCallManager {
    pub fn ShowPhoneCallUI(phonenumber: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IPhoneCallManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowPhoneCallUI)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(phonenumber), ::core::mem::transmute_copy(displayname)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CallStateChanged<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCallStateChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveCallStateChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn IsCallActive() -> ::windows_core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCallActive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsCallIncoming() -> ::windows_core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCallIncoming)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowPhoneCallSettingsUI() -> ::windows_core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).ShowPhoneCallSettingsUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallStore>> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallManagerStatics<R, F: FnOnce(&IPhoneCallManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPhoneCallManagerStatics2<R, F: FnOnce(&IPhoneCallManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PhoneCallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallManager";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallStore(::windows_core::IUnknown);
impl PhoneCallStore {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsEmergencyPhoneNumberAsync(&self, number: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEmergencyPhoneNumberAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultLineAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultLineAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestLineWatcher(&self) -> ::windows_core::Result<PhoneLineWatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestLineWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallStore {}
impl ::core::fmt::Debug for PhoneCallStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallStore").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallStore;{5f610748-18a6-4173-86d1-28be9dc62dba})");
}
impl ::core::clone::Clone for PhoneCallStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallStore {
    type Vtable = IPhoneCallStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallStore {
    const IID: ::windows_core::GUID = <IPhoneCallStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallStore";
}
::windows_core::imp::interface_hierarchy!(PhoneCallStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallStore {}
unsafe impl ::core::marker::Sync for PhoneCallStore {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallVideoCapabilities(::windows_core::IUnknown);
impl PhoneCallVideoCapabilities {
    pub fn IsVideoCallingCapable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoCallingCapable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallVideoCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallVideoCapabilities {}
impl ::core::fmt::Debug for PhoneCallVideoCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallVideoCapabilities").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallVideoCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities;{02382786-b16a-4fdb-be3b-c4240e13ad0d})");
}
impl ::core::clone::Clone for PhoneCallVideoCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallVideoCapabilities {
    const IID: ::windows_core::GUID = <IPhoneCallVideoCapabilities as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallVideoCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities";
}
::windows_core::imp::interface_hierarchy!(PhoneCallVideoCapabilities, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallVideoCapabilities {}
unsafe impl ::core::marker::Sync for PhoneCallVideoCapabilities {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
pub struct PhoneCallVideoCapabilitiesManager;
impl PhoneCallVideoCapabilitiesManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesAsync(phonenumber: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>> {
        Self::IPhoneCallVideoCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCapabilitiesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(phonenumber), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallVideoCapabilitiesManagerStatics<R, F: FnOnce(&IPhoneCallVideoCapabilitiesManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneCallVideoCapabilitiesManager, IPhoneCallVideoCapabilitiesManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PhoneCallVideoCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager";
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallsResult(::windows_core::IUnknown);
impl PhoneCallsResult {
    pub fn OperationStatus(&self) -> ::windows_core::Result<PhoneLineOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllActivePhoneCalls(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PhoneCall>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllActivePhoneCalls)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallsResult {}
impl ::core::fmt::Debug for PhoneCallsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallsResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneCallsResult;{1bfad365-57cf-57dd-986d-b057c91eac33})");
}
impl ::core::clone::Clone for PhoneCallsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallsResult {
    type Vtable = IPhoneCallsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallsResult {
    const IID: ::windows_core::GUID = <IPhoneCallsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallsResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallsResult";
}
::windows_core::imp::interface_hierarchy!(PhoneCallsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneCallsResult {}
unsafe impl ::core::marker::Sync for PhoneCallsResult {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneDialOptions(::windows_core::IUnknown);
impl PhoneDialOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneDialOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Number(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Number)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumber(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumber)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContact<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Contacts::Contact>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContact)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPhone(&self) -> ::windows_core::Result<super::Contacts::ContactPhone> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactPhone)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContactPhone<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Contacts::ContactPhone>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContactPhone)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Media(&self) -> ::windows_core::Result<PhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Media)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMedia(&self, value: PhoneCallMedia) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMedia)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEndpoint(&self) -> ::windows_core::Result<PhoneAudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioEndpoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioEndpoint(&self, value: PhoneAudioRoutingEndpoint) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioEndpoint)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for PhoneDialOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneDialOptions {}
impl ::core::fmt::Debug for PhoneDialOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneDialOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneDialOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneDialOptions;{b639c4b8-f06f-36cb-a863-823742b5f2d4})");
}
impl ::core::clone::Clone for PhoneDialOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneDialOptions {
    type Vtable = IPhoneDialOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneDialOptions {
    const IID: ::windows_core::GUID = <IPhoneDialOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneDialOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneDialOptions";
}
::windows_core::imp::interface_hierarchy!(PhoneDialOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneDialOptions {}
unsafe impl ::core::marker::Sync for PhoneDialOptions {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLine(::windows_core::IUnknown);
impl PhoneLine {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLine, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLineChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn DisplayColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkState(&self) -> ::windows_core::Result<PhoneNetworkState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Voicemail(&self) -> ::windows_core::Result<PhoneVoicemail> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Voicemail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularDetails(&self) -> ::windows_core::Result<PhoneLineCellularDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellularDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Transport(&self) -> ::windows_core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanDial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanDial)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportsTile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsTile)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoCallingCapabilities(&self) -> ::windows_core::Result<PhoneCallVideoCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoCallingCapabilities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LineConfiguration(&self) -> ::windows_core::Result<PhoneLineConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineConfiguration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsImmediateDialNumberAsync(&self, number: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsImmediateDialNumberAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn Dial(&self, number: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Dial)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), ::core::mem::transmute_copy(displayname)).ok() }
    }
    pub fn DialWithOptions<P0>(&self, options: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PhoneDialOptions>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DialWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn EnableTextReply(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLine2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableTextReply)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransportDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLine2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransportDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DialWithResult(&self, number: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<PhoneLineDialResult> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DialWithResult)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DialWithResultAsync(&self, number: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DialWithResultAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    pub fn GetAllActivePhoneCalls(&self) -> ::windows_core::Result<PhoneCallsResult> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllActivePhoneCalls)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAllActivePhoneCallsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallsResult>> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllActivePhoneCallsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(lineid: ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneLine>> {
        Self::IPhoneLineStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), lineid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneLineStatics<R, F: FnOnce(&IPhoneLineStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneLine, IPhoneLineStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PhoneLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLine {}
impl ::core::fmt::Debug for PhoneLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLine").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLine {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLine;{27c66f30-6a69-34ca-a2ba-65302530c311})");
}
impl ::core::clone::Clone for PhoneLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLine {
    type Vtable = IPhoneLine_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLine {
    const IID: ::windows_core::GUID = <IPhoneLine as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLine {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLine";
}
::windows_core::imp::interface_hierarchy!(PhoneLine, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLine {}
unsafe impl ::core::marker::Sync for PhoneLine {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineCellularDetails(::windows_core::IUnknown);
impl PhoneLineCellularDetails {
    pub fn SimState(&self) -> ::windows_core::Result<PhoneSimState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimSlotIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimSlotIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsModemOn(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsModemOn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegistrationRejectCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegistrationRejectCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNetworkOperatorDisplayText(&self, location: PhoneLineNetworkOperatorDisplayTextLocation) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNetworkOperatorDisplayText)(::windows_core::Interface::as_raw(this), location, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneLineCellularDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineCellularDetails {}
impl ::core::fmt::Debug for PhoneLineCellularDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineCellularDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineCellularDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineCellularDetails;{192601d5-147c-4769-b673-98a5ec8426cb})");
}
impl ::core::clone::Clone for PhoneLineCellularDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineCellularDetails {
    const IID: ::windows_core::GUID = <IPhoneLineCellularDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineCellularDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineCellularDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneLineCellularDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineCellularDetails {}
unsafe impl ::core::marker::Sync for PhoneLineCellularDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineConfiguration(::windows_core::IUnknown);
impl PhoneLineConfiguration {
    pub fn IsVideoCallingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoCallingEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneLineConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineConfiguration {}
impl ::core::fmt::Debug for PhoneLineConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineConfiguration;{fe265862-f64f-4312-b2a8-4e257721aa95})");
}
impl ::core::clone::Clone for PhoneLineConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineConfiguration {
    const IID: ::windows_core::GUID = <IPhoneLineConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineConfiguration";
}
::windows_core::imp::interface_hierarchy!(PhoneLineConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineConfiguration {}
unsafe impl ::core::marker::Sync for PhoneLineConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineDialResult(::windows_core::IUnknown);
impl PhoneLineDialResult {
    pub fn DialCallStatus(&self) -> ::windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DialCallStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DialedCall(&self) -> ::windows_core::Result<PhoneCall> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DialedCall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneLineDialResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineDialResult {}
impl ::core::fmt::Debug for PhoneLineDialResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineDialResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineDialResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineDialResult;{e825a30a-5c7f-546f-b918-3ad2fe70fb34})");
}
impl ::core::clone::Clone for PhoneLineDialResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineDialResult {
    const IID: ::windows_core::GUID = <IPhoneLineDialResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineDialResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineDialResult";
}
::windows_core::imp::interface_hierarchy!(PhoneLineDialResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineDialResult {}
unsafe impl ::core::marker::Sync for PhoneLineDialResult {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineTransportDevice(::windows_core::IUnknown);
impl PhoneLineTransportDevice {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Transport(&self) -> ::windows_core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegisterApp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterApp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn RegisterAppForUser<P0>(&self, user: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAppForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi()).ok() }
    }
    pub fn UnregisterApp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnregisterApp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn UnregisterAppForUser<P0>(&self, user: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnregisterAppForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi()).ok() }
    }
    pub fn IsRegistered(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRegistered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Connect(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connect)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AudioRoutingStatus(&self) -> ::windows_core::Result<TransportDeviceAudioRoutingStatus> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioRoutingStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioRoutingStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioRoutingStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioRoutingStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioRoutingStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn InBandRingingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InBandRingingEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InBandRingingEnabledChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InBandRingingEnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInBandRingingEnabledChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInBandRingingEnabledChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FromId(id: &::windows_core::HSTRING) -> ::windows_core::Result<PhoneLineTransportDevice> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForPhoneLineTransport(transport: PhoneLineTransport) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorForPhoneLineTransport)(::windows_core::Interface::as_raw(this), transport, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneLineTransportDeviceStatics<R, F: FnOnce(&IPhoneLineTransportDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneLineTransportDevice, IPhoneLineTransportDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PhoneLineTransportDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineTransportDevice {}
impl ::core::fmt::Debug for PhoneLineTransportDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineTransportDevice").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineTransportDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineTransportDevice;{efa8f889-cffa-59f4-97e4-74705b7dc490})");
}
impl ::core::clone::Clone for PhoneLineTransportDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineTransportDevice {
    const IID: ::windows_core::GUID = <IPhoneLineTransportDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineTransportDevice {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineTransportDevice";
}
::windows_core::imp::interface_hierarchy!(PhoneLineTransportDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineTransportDevice {}
unsafe impl ::core::marker::Sync for PhoneLineTransportDevice {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineWatcher(::windows_core::IUnknown);
impl PhoneLineWatcher {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineAdded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLineAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineRemoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLineRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LineUpdated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLineUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLineUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<PhoneLineWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneLineWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineWatcher {}
impl ::core::fmt::Debug for PhoneLineWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineWatcher").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcher;{8a45cd0a-6323-44e0-a6f6-9f21f64dc90a})");
}
impl ::core::clone::Clone for PhoneLineWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineWatcher {
    const IID: ::windows_core::GUID = <IPhoneLineWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineWatcher {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcher";
}
::windows_core::imp::interface_hierarchy!(PhoneLineWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineWatcher {}
unsafe impl ::core::marker::Sync for PhoneLineWatcher {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineWatcherEventArgs(::windows_core::IUnknown);
impl PhoneLineWatcherEventArgs {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneLineWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineWatcherEventArgs {}
impl ::core::fmt::Debug for PhoneLineWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineWatcherEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineWatcherEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs;{d07c753e-9e12-4a37-82b7-ad535dad6a67})");
}
impl ::core::clone::Clone for PhoneLineWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineWatcherEventArgs {
    const IID: ::windows_core::GUID = <IPhoneLineWatcherEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineWatcherEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs";
}
::windows_core::imp::interface_hierarchy!(PhoneLineWatcherEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneLineWatcherEventArgs {}
unsafe impl ::core::marker::Sync for PhoneLineWatcherEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneVoicemail(::windows_core::IUnknown);
impl PhoneVoicemail {
    pub fn Number(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Number)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MessageCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<PhoneVoicemailType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DialVoicemailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DialVoicemailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneVoicemail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneVoicemail {}
impl ::core::fmt::Debug for PhoneVoicemail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneVoicemail").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneVoicemail {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.PhoneVoicemail;{c9ce77f6-6e9f-3a8b-b727-6e0cf6998224})");
}
impl ::core::clone::Clone for PhoneVoicemail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneVoicemail {
    type Vtable = IPhoneVoicemail_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneVoicemail {
    const IID: ::windows_core::GUID = <IPhoneVoicemail as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneVoicemail {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneVoicemail";
}
::windows_core::imp::interface_hierarchy!(PhoneVoicemail, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneVoicemail {}
unsafe impl ::core::marker::Sync for PhoneVoicemail {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipCallCoordinator(::windows_core::IUnknown);
impl VoipCallCoordinator {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReserveCallResourcesAsync(&self, taskentrypoint: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReserveCallResourcesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(taskentrypoint), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MuteStateChanged<P0>(&self, mutechangehandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MuteStateChanged)(::windows_core::Interface::as_raw(this), mutechangehandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMuteStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMuteStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestNewIncomingCall<P0, P1, P2>(&self, context: &::windows_core::HSTRING, contactname: &::windows_core::HSTRING, contactnumber: &::windows_core::HSTRING, contactimage: P0, servicename: &::windows_core::HSTRING, brandingimage: P1, calldetails: &::windows_core::HSTRING, ringtone: P2, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan) -> ::windows_core::Result<VoipPhoneCall>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestNewIncomingCall)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(context), ::core::mem::transmute_copy(contactname), ::core::mem::transmute_copy(contactnumber), contactimage.into_param().abi(), ::core::mem::transmute_copy(servicename), brandingimage.into_param().abi(), ::core::mem::transmute_copy(calldetails), ringtone.into_param().abi(), media, ringtimeout, &mut result__).from_abi(result__)
        }
    }
    pub fn RequestNewOutgoingCall(&self, context: &::windows_core::HSTRING, contactname: &::windows_core::HSTRING, servicename: &::windows_core::HSTRING, media: VoipPhoneCallMedia) -> ::windows_core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestNewOutgoingCall)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(context), ::core::mem::transmute_copy(contactname), ::core::mem::transmute_copy(servicename), media, &mut result__).from_abi(result__)
        }
    }
    pub fn NotifyMuted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyMuted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyUnmuted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyUnmuted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestOutgoingUpgradeToVideoCall(&self, callupgradeguid: ::windows_core::GUID, context: &::windows_core::HSTRING, contactname: &::windows_core::HSTRING, servicename: &::windows_core::HSTRING) -> ::windows_core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestOutgoingUpgradeToVideoCall)(::windows_core::Interface::as_raw(this), callupgradeguid, ::core::mem::transmute_copy(context), ::core::mem::transmute_copy(contactname), ::core::mem::transmute_copy(servicename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestIncomingUpgradeToVideoCall<P0, P1, P2>(&self, context: &::windows_core::HSTRING, contactname: &::windows_core::HSTRING, contactnumber: &::windows_core::HSTRING, contactimage: P0, servicename: &::windows_core::HSTRING, brandingimage: P1, calldetails: &::windows_core::HSTRING, ringtone: P2, ringtimeout: super::super::Foundation::TimeSpan) -> ::windows_core::Result<VoipPhoneCall>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestIncomingUpgradeToVideoCall)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(context), ::core::mem::transmute_copy(contactname), ::core::mem::transmute_copy(contactnumber), contactimage.into_param().abi(), ::core::mem::transmute_copy(servicename), brandingimage.into_param().abi(), ::core::mem::transmute_copy(calldetails), ringtone.into_param().abi(), ringtimeout, &mut result__).from_abi(result__)
        }
    }
    pub fn TerminateCellularCall(&self, callupgradeguid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).TerminateCellularCall)(::windows_core::Interface::as_raw(this), callupgradeguid).ok() }
    }
    pub fn CancelUpgrade(&self, callupgradeguid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CancelUpgrade)(::windows_core::Interface::as_raw(this), callupgradeguid).ok() }
    }
    pub fn SetupNewAcceptedCall(&self, context: &::windows_core::HSTRING, contactname: &::windows_core::HSTRING, contactnumber: &::windows_core::HSTRING, servicename: &::windows_core::HSTRING, media: VoipPhoneCallMedia) -> ::windows_core::Result<VoipPhoneCall> {
        let this = &::windows_core::ComInterface::cast::<IVoipCallCoordinator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetupNewAcceptedCall)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(context), ::core::mem::transmute_copy(contactname), ::core::mem::transmute_copy(contactnumber), ::core::mem::transmute_copy(servicename), media, &mut result__).from_abi(result__)
        }
    }
    pub fn RequestNewAppInitiatedCall(&self, context: &::windows_core::HSTRING, contactname: &::windows_core::HSTRING, contactnumber: &::windows_core::HSTRING, servicename: &::windows_core::HSTRING, media: VoipPhoneCallMedia) -> ::windows_core::Result<VoipPhoneCall> {
        let this = &::windows_core::ComInterface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestNewAppInitiatedCall)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(context), ::core::mem::transmute_copy(contactname), ::core::mem::transmute_copy(contactnumber), ::core::mem::transmute_copy(servicename), media, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestNewIncomingCallWithContactRemoteId<P0, P1, P2>(&self, context: &::windows_core::HSTRING, contactname: &::windows_core::HSTRING, contactnumber: &::windows_core::HSTRING, contactimage: P0, servicename: &::windows_core::HSTRING, brandingimage: P1, calldetails: &::windows_core::HSTRING, ringtone: P2, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, contactremoteid: &::windows_core::HSTRING) -> ::windows_core::Result<VoipPhoneCall>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestNewIncomingCallWithContactRemoteId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(context), ::core::mem::transmute_copy(contactname), ::core::mem::transmute_copy(contactnumber), contactimage.into_param().abi(), ::core::mem::transmute_copy(servicename), brandingimage.into_param().abi(), ::core::mem::transmute_copy(calldetails), ringtone.into_param().abi(), media, ringtimeout, ::core::mem::transmute_copy(contactremoteid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReserveOneProcessCallResourcesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = &::windows_core::ComInterface::cast::<IVoipCallCoordinator4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReserveOneProcessCallResourcesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<VoipCallCoordinator> {
        Self::IVoipCallCoordinatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoipCallCoordinatorStatics<R, F: FnOnce(&IVoipCallCoordinatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VoipCallCoordinator, IVoipCallCoordinatorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VoipCallCoordinator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoipCallCoordinator {}
impl ::core::fmt::Debug for VoipCallCoordinator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipCallCoordinator").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoipCallCoordinator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipCallCoordinator;{4f118bcf-e8ef-4434-9c5f-a8d893fafe79})");
}
impl ::core::clone::Clone for VoipCallCoordinator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoipCallCoordinator {
    const IID: ::windows_core::GUID = <IVoipCallCoordinator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoipCallCoordinator {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipCallCoordinator";
}
::windows_core::imp::interface_hierarchy!(VoipCallCoordinator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoipCallCoordinator {}
unsafe impl ::core::marker::Sync for VoipCallCoordinator {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipPhoneCall(::windows_core::IUnknown);
impl VoipPhoneCall {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEndRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEndRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HoldRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HoldRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHoldRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHoldRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResumeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResumeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResumeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnswerRequested<P0>(&self, accepthandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnswerRequested)(::windows_core::Interface::as_raw(this), accepthandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAnswerRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAnswerRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RejectRequested<P0>(&self, rejecthandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RejectRequested)(::windows_core::Interface::as_raw(this), rejecthandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRejectRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRejectRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NotifyCallHeld(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyCallHeld)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyCallActive(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyCallActive)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyCallEnded(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyCallEnded)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ContactName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContactName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContactName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CallMedia(&self) -> ::windows_core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallMedia)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCallMedia(&self, value: VoipPhoneCallMedia) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCallMedia)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NotifyCallReady(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyCallReady)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryShowAppUI(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVoipPhoneCall2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).TryShowAppUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyCallAccepted(&self, media: VoipPhoneCallMedia) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVoipPhoneCall3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).NotifyCallAccepted)(::windows_core::Interface::as_raw(this), media).ok() }
    }
}
impl ::core::cmp::PartialEq for VoipPhoneCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoipPhoneCall {}
impl ::core::fmt::Debug for VoipPhoneCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCall").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoipPhoneCall {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.VoipPhoneCall;{6cf1f19a-7794-4a5a-8c68-ae87947a6990})");
}
impl ::core::clone::Clone for VoipPhoneCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoipPhoneCall {
    type Vtable = IVoipPhoneCall_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoipPhoneCall {
    const IID: ::windows_core::GUID = <IVoipPhoneCall as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoipPhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipPhoneCall";
}
::windows_core::imp::interface_hierarchy!(VoipPhoneCall, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoipPhoneCall {}
unsafe impl ::core::marker::Sync for VoipPhoneCall {}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: Self = Self(0i32);
    pub const Burst: Self = Self(1i32);
}
impl ::core::marker::Copy for CellularDtmfMode {}
impl ::core::clone::Clone for CellularDtmfMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CellularDtmfMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CellularDtmfMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CellularDtmfMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CellularDtmfMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CellularDtmfMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.CellularDtmfMode;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DtmfKey(pub i32);
impl DtmfKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
}
impl ::core::marker::Copy for DtmfKey {}
impl ::core::clone::Clone for DtmfKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DtmfKey {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DtmfKey {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DtmfKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtmfKey").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DtmfKey {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfKey;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: Self = Self(0i32);
    pub const DoNotPlay: Self = Self(1i32);
}
impl ::core::marker::Copy for DtmfToneAudioPlayback {}
impl ::core::clone::Clone for DtmfToneAudioPlayback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DtmfToneAudioPlayback {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DtmfToneAudioPlayback {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DtmfToneAudioPlayback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtmfToneAudioPlayback").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DtmfToneAudioPlayback {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfToneAudioPlayback;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Bluetooth: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneAudioRoutingEndpoint {}
impl ::core::clone::Clone for PhoneAudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneAudioRoutingEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneAudioRoutingEndpoint {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneAudioRoutingEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneAudioRoutingEndpoint").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneAudioRoutingEndpoint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneAudioRoutingEndpoint;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: Self = Self(0i32);
    pub const LocalDevice: Self = Self(1i32);
    pub const RemoteDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallAudioDevice {}
impl ::core::clone::Clone for PhoneCallAudioDevice {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallAudioDevice {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallAudioDevice {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallAudioDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallAudioDevice").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallAudioDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallAudioDevice;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Outgoing: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallDirection {}
impl ::core::clone::Clone for PhoneCallDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallDirection;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryMedia {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallHistoryEntryMedia {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryMedia").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntryMedia {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryMedia;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryOtherAppReadAccess {}
impl ::core::clone::Clone for PhoneCallHistoryEntryOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallHistoryEntryOtherAppReadAccess {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryOtherAppReadAccess").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntryOtherAppReadAccess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryOtherAppReadAccess;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryQueryDesiredMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryQueryDesiredMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryQueryDesiredMedia {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallHistoryEntryQueryDesiredMedia {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryQueryDesiredMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryQueryDesiredMedia").field(&self.0).finish()
    }
}
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntryQueryDesiredMedia {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryDesiredMedia;u4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryRawAddressKind {}
impl ::core::clone::Clone for PhoneCallHistoryEntryRawAddressKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryEntryRawAddressKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallHistoryEntryRawAddressKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallHistoryEntryRawAddressKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryRawAddressKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryEntryRawAddressKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryRawAddressKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: Self = Self(0i32);
    pub const PackageFamilyName: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistorySourceIdKind {}
impl ::core::clone::Clone for PhoneCallHistorySourceIdKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistorySourceIdKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallHistorySourceIdKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallHistorySourceIdKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistorySourceIdKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistorySourceIdKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistorySourceIdKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: Self = Self(0i32);
    pub const AllEntriesLimitedReadWrite: Self = Self(1i32);
    pub const AllEntriesReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallHistoryStoreAccessType {}
impl ::core::clone::Clone for PhoneCallHistoryStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallHistoryStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallHistoryStoreAccessType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallHistoryStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryStoreAccessType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallHistoryStoreAccessType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryStoreAccessType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: Self = Self(0i32);
    pub const AudioAndVideo: Self = Self(1i32);
    pub const AudioAndRealTimeText: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallMedia {}
impl ::core::clone::Clone for PhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallMedia {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallMedia {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallMedia").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallMedia {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallMedia;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneCallOperationStatus {}
impl ::core::clone::Clone for PhoneCallOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallOperationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallOperationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallOperationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallOperationStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneCallStatus(pub i32);
impl PhoneCallStatus {
    pub const Lost: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Dialing: Self = Self(2i32);
    pub const Talking: Self = Self(3i32);
    pub const Held: Self = Self(4i32);
    pub const Ended: Self = Self(5i32);
}
impl ::core::marker::Copy for PhoneCallStatus {}
impl ::core::clone::Clone for PhoneCallStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneCallStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Dialer: Self = Self(2i32);
    pub const InCallUI: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineNetworkOperatorDisplayTextLocation {}
impl ::core::clone::Clone for PhoneLineNetworkOperatorDisplayTextLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineNetworkOperatorDisplayTextLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneLineNetworkOperatorDisplayTextLocation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineNetworkOperatorDisplayTextLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineNetworkOperatorDisplayTextLocation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineNetworkOperatorDisplayTextLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineNetworkOperatorDisplayTextLocation;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneLineOperationStatus {}
impl ::core::clone::Clone for PhoneLineOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneLineOperationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineOperationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineOperationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineOperationStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: Self = Self(0i32);
    pub const VoipApp: Self = Self(1i32);
    pub const Bluetooth: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineTransport {}
impl ::core::clone::Clone for PhoneLineTransport {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineTransport {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneLineTransport {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineTransport").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineTransport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineTransport;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineWatcherStatus {}
impl ::core::clone::Clone for PhoneLineWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneLineWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineWatcherStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneNetworkState(pub i32);
impl PhoneNetworkState {
    pub const Unknown: Self = Self(0i32);
    pub const NoSignal: Self = Self(1i32);
    pub const Deregistered: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const Searching: Self = Self(4i32);
    pub const Home: Self = Self(5i32);
    pub const RoamingInternational: Self = Self(6i32);
    pub const RoamingDomestic: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneNetworkState {}
impl ::core::clone::Clone for PhoneNetworkState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNetworkState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneNetworkState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneNetworkState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNetworkState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneNetworkState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneNetworkState;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneSimState(pub i32);
impl PhoneSimState {
    pub const Unknown: Self = Self(0i32);
    pub const PinNotRequired: Self = Self(1i32);
    pub const PinUnlocked: Self = Self(2i32);
    pub const PinLocked: Self = Self(3i32);
    pub const PukLocked: Self = Self(4i32);
    pub const NotInserted: Self = Self(5i32);
    pub const Invalid: Self = Self(6i32);
    pub const Disabled: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneSimState {}
impl ::core::clone::Clone for PhoneSimState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneSimState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneSimState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneSimState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneSimState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneSimState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneSimState;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: Self = Self(0i32);
    pub const Traditional: Self = Self(1i32);
    pub const Visual: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneVoicemailType {}
impl ::core::clone::Clone for PhoneVoicemailType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneVoicemailType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneVoicemailType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneVoicemailType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneVoicemailType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneVoicemailType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneVoicemailType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CanRouteToLocalDevice: Self = Self(1i32);
    pub const CannotRouteToLocalDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for TransportDeviceAudioRoutingStatus {}
impl ::core::clone::Clone for TransportDeviceAudioRoutingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TransportDeviceAudioRoutingStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TransportDeviceAudioRoutingStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TransportDeviceAudioRoutingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransportDeviceAudioRoutingStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TransportDeviceAudioRoutingStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.TransportDeviceAudioRoutingStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
}
impl ::core::marker::Copy for VoipPhoneCallMedia {}
impl ::core::clone::Clone for VoipPhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallMedia {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VoipPhoneCallMedia {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VoipPhoneCallMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallMedia").field(&self.0).finish()
    }
}
impl VoipPhoneCallMedia {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VoipPhoneCallMedia {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VoipPhoneCallMedia {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VoipPhoneCallMedia {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VoipPhoneCallMedia {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VoipPhoneCallMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for VoipPhoneCallMedia {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallMedia;u4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const OtherIncomingCall: Self = Self(2i32);
    pub const EmergencyCallExists: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for VoipPhoneCallRejectReason {}
impl ::core::clone::Clone for VoipPhoneCallRejectReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallRejectReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VoipPhoneCallRejectReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VoipPhoneCallRejectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallRejectReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoipPhoneCallRejectReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallRejectReason;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: Self = Self(0i32);
    pub const ResourcesNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for VoipPhoneCallResourceReservationStatus {}
impl ::core::clone::Clone for VoipPhoneCallResourceReservationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallResourceReservationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VoipPhoneCallResourceReservationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VoipPhoneCallResourceReservationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallResourceReservationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoipPhoneCallResourceReservationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: Self = Self(0i32);
    pub const Held: Self = Self(1i32);
    pub const Active: Self = Self(2i32);
    pub const Incoming: Self = Self(3i32);
    pub const Outgoing: Self = Self(4i32);
}
impl ::core::marker::Copy for VoipPhoneCallState {}
impl ::core::clone::Clone for VoipPhoneCallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoipPhoneCallState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VoipPhoneCallState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VoipPhoneCallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoipPhoneCallState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoipPhoneCallState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallState;i4)");
}
