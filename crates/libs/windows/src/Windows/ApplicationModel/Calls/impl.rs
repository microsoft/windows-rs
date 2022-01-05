#[cfg(feature = "implement_exclusive")]
pub trait ICallAnswerEventArgsImpl: Sized {
    fn AcceptedMedia(&self) -> ::windows::core::Result<VoipPhoneCallMedia>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallRejectEventArgsImpl: Sized {
    fn RejectReason(&self) -> ::windows::core::Result<VoipPhoneCallRejectReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallStateChangeEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<VoipPhoneCallState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenCallEndCallDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenCallEndRequestedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<LockScreenCallEndCallDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenCallUIImpl: Sized {
    fn Dismiss(&self) -> ::windows::core::Result<()>;
    fn EndRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEndRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CallTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCallTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMuteChangeEventArgsImpl: Sized {
    fn Muted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallImpl: Sized {
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioDeviceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioDeviceChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsMutedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsMutedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CallId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsMuted(&self) -> ::windows::core::Result<bool>;
    fn Status(&self) -> ::windows::core::Result<PhoneCallStatus>;
    fn AudioDevice(&self) -> ::windows::core::Result<PhoneCallAudioDevice>;
    fn GetPhoneCallInfo(&self) -> ::windows::core::Result<PhoneCallInfo>;
    fn GetPhoneCallInfoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallInfo>>;
    fn End(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn EndAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn SendDtmfKey(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn SendDtmfKeyAsync(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn AcceptIncoming(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn AcceptIncomingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn Hold(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn HoldAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn ResumeFromHold(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn ResumeFromHoldAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn Mute(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn MuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn Unmute(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn UnmuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn RejectIncoming(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn RejectIncomingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn ChangeAudioDevice(&self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn ChangeAudioDeviceAsync(&self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallBlockingStaticsImpl: Sized {
    fn BlockUnknownNumbers(&self) -> ::windows::core::Result<bool>;
    fn SetBlockUnknownNumbers(&self, value: bool) -> ::windows::core::Result<()>;
    fn BlockPrivateNumbers(&self) -> ::windows::core::Result<bool>;
    fn SetBlockPrivateNumbers(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCallBlockingListAsync(&self, phonenumberlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryEntryImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&self) -> ::windows::core::Result<PhoneCallHistoryEntryAddress>;
    fn SetAddress(&self, value: &::core::option::Option<PhoneCallHistoryEntryAddress>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn IsCallerIdBlocked(&self) -> ::windows::core::Result<bool>;
    fn SetIsCallerIdBlocked(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEmergency(&self) -> ::windows::core::Result<bool>;
    fn SetIsEmergency(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsIncoming(&self) -> ::windows::core::Result<bool>;
    fn SetIsIncoming(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMissed(&self) -> ::windows::core::Result<bool>;
    fn SetIsMissed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRinging(&self) -> ::windows::core::Result<bool>;
    fn SetIsRinging(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeen(&self) -> ::windows::core::Result<bool>;
    fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSuppressed(&self) -> ::windows::core::Result<bool>;
    fn SetIsSuppressed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVoicemail(&self) -> ::windows::core::Result<bool>;
    fn SetIsVoicemail(&self, value: bool) -> ::windows::core::Result<()>;
    fn Media(&self) -> ::windows::core::Result<PhoneCallHistoryEntryMedia>;
    fn SetMedia(&self, value: PhoneCallHistoryEntryMedia) -> ::windows::core::Result<()>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<PhoneCallHistoryEntryOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSourceId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceIdKind(&self) -> ::windows::core::Result<PhoneCallHistorySourceIdKind>;
    fn SetSourceIdKind(&self, value: PhoneCallHistorySourceIdKind) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryEntryAddressImpl: Sized {
    fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RawAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRawAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RawAddressKind(&self) -> ::windows::core::Result<PhoneCallHistoryEntryRawAddressKind>;
    fn SetRawAddressKind(&self, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryEntryAddressFactoryImpl: Sized {
    fn Create(&self, rawaddress: &::windows::core::HSTRING, rawaddresskind: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<PhoneCallHistoryEntryAddress>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryEntryQueryOptionsImpl: Sized {
    fn DesiredMedia(&self) -> ::windows::core::Result<PhoneCallHistoryEntryQueryDesiredMedia>;
    fn SetDesiredMedia(&self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::Result<()>;
    fn SourceIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryEntryReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryManagerForUserImpl: Sized {
    fn RequestStoreAsync(&self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryManagerStaticsImpl: Sized {
    fn RequestStoreAsync(&self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<PhoneCallHistoryManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryStoreImpl: Sized {
    fn GetEntryAsync(&self, callhistoryentryid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>>;
    fn GetEntryReader(&self) -> ::windows::core::Result<PhoneCallHistoryEntryReader>;
    fn GetEntryReaderWithOptions(&self, queryoptions: &::core::option::Option<PhoneCallHistoryEntryQueryOptions>) -> ::windows::core::Result<PhoneCallHistoryEntryReader>;
    fn SaveEntryAsync(&self, callhistoryentry: &::core::option::Option<PhoneCallHistoryEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteEntryAsync(&self, callhistoryentry: &::core::option::Option<PhoneCallHistoryEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteEntriesAsync(&self, callhistoryentries: &::core::option::Option<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkEntryAsSeenAsync(&self, callhistoryentry: &::core::option::Option<PhoneCallHistoryEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkEntriesAsSeenAsync(&self, callhistoryentries: &::core::option::Option<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetUnseenCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn MarkAllAsSeenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetSourcesUnseenCountAsync(&self, sourceids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn MarkSourcesAsSeenAsync(&self, sourceids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallInfoImpl: Sized {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsHoldSupported(&self) -> ::windows::core::Result<bool>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CallDirection(&self) -> ::windows::core::Result<PhoneCallDirection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallManagerStaticsImpl: Sized {
    fn ShowPhoneCallUI(&self, phonenumber: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallManagerStatics2Impl: Sized {
    fn CallStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCallStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsCallActive(&self) -> ::windows::core::Result<bool>;
    fn IsCallIncoming(&self) -> ::windows::core::Result<bool>;
    fn ShowPhoneCallSettingsUI(&self) -> ::windows::core::Result<()>;
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallStaticsImpl: Sized {
    fn GetFromId(&self, callid: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneCall>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallStoreImpl: Sized {
    fn IsEmergencyPhoneNumberAsync(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetDefaultLineAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::GUID>>;
    fn RequestLineWatcher(&self) -> ::windows::core::Result<PhoneLineWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallVideoCapabilitiesImpl: Sized {
    fn IsVideoCallingCapable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallVideoCapabilitiesManagerStaticsImpl: Sized {
    fn GetCapabilitiesAsync(&self, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallsResultImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<PhoneLineOperationStatus>;
    fn AllActivePhoneCalls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhoneCall>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneDialOptionsImpl: Sized {
    fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact>;
    fn SetContact(&self, value: &::core::option::Option<super::Contacts::Contact>) -> ::windows::core::Result<()>;
    fn ContactPhone(&self) -> ::windows::core::Result<super::Contacts::ContactPhone>;
    fn SetContactPhone(&self, value: &::core::option::Option<super::Contacts::ContactPhone>) -> ::windows::core::Result<()>;
    fn Media(&self) -> ::windows::core::Result<PhoneCallMedia>;
    fn SetMedia(&self, value: PhoneCallMedia) -> ::windows::core::Result<()>;
    fn AudioEndpoint(&self) -> ::windows::core::Result<PhoneAudioRoutingEndpoint>;
    fn SetAudioEndpoint(&self, value: PhoneAudioRoutingEndpoint) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineImpl: Sized {
    fn LineChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLine, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DisplayColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn NetworkState(&self) -> ::windows::core::Result<PhoneNetworkState>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Voicemail(&self) -> ::windows::core::Result<PhoneVoicemail>;
    fn NetworkName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularDetails(&self) -> ::windows::core::Result<PhoneLineCellularDetails>;
    fn Transport(&self) -> ::windows::core::Result<PhoneLineTransport>;
    fn CanDial(&self) -> ::windows::core::Result<bool>;
    fn SupportsTile(&self) -> ::windows::core::Result<bool>;
    fn VideoCallingCapabilities(&self) -> ::windows::core::Result<PhoneCallVideoCapabilities>;
    fn LineConfiguration(&self) -> ::windows::core::Result<PhoneLineConfiguration>;
    fn IsImmediateDialNumberAsync(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Dial(&self, number: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DialWithOptions(&self, options: &::core::option::Option<PhoneDialOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLine2Impl: Sized {
    fn EnableTextReply(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransportDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLine3Impl: Sized {
    fn DialWithResult(&self, number: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneLineDialResult>;
    fn DialWithResultAsync(&self, number: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>>;
    fn GetAllActivePhoneCalls(&self) -> ::windows::core::Result<PhoneCallsResult>;
    fn GetAllActivePhoneCallsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallsResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineCellularDetailsImpl: Sized {
    fn SimState(&self) -> ::windows::core::Result<PhoneSimState>;
    fn SimSlotIndex(&self) -> ::windows::core::Result<i32>;
    fn IsModemOn(&self) -> ::windows::core::Result<bool>;
    fn RegistrationRejectCode(&self) -> ::windows::core::Result<i32>;
    fn GetNetworkOperatorDisplayText(&self, location: PhoneLineNetworkOperatorDisplayTextLocation) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineConfigurationImpl: Sized {
    fn IsVideoCallingEnabled(&self) -> ::windows::core::Result<bool>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineDialResultImpl: Sized {
    fn DialCallStatus(&self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn DialedCall(&self) -> ::windows::core::Result<PhoneCall>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineStaticsImpl: Sized {
    fn FromIdAsync(&self, lineid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLine>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineTransportDeviceImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transport(&self) -> ::windows::core::Result<PhoneLineTransport>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>>;
    fn RegisterApp(&self) -> ::windows::core::Result<()>;
    fn RegisterAppForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<()>;
    fn UnregisterApp(&self) -> ::windows::core::Result<()>;
    fn UnregisterAppForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<()>;
    fn IsRegistered(&self) -> ::windows::core::Result<bool>;
    fn Connect(&self) -> ::windows::core::Result<bool>;
    fn ConnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineTransportDevice2Impl: Sized {
    fn AudioRoutingStatus(&self) -> ::windows::core::Result<TransportDeviceAudioRoutingStatus>;
    fn AudioRoutingStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioRoutingStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InBandRingingEnabled(&self) -> ::windows::core::Result<bool>;
    fn InBandRingingEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInBandRingingEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineTransportDeviceStaticsImpl: Sized {
    fn FromId(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneLineTransportDevice>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForPhoneLineTransport(&self, transport: PhoneLineTransport) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineWatcherImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn LineAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LineRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LineUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<PhoneLineWatcherStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineWatcherEventArgsImpl: Sized {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneVoicemailImpl: Sized {
    fn Number(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MessageCount(&self) -> ::windows::core::Result<i32>;
    fn Type(&self) -> ::windows::core::Result<PhoneVoicemailType>;
    fn DialVoicemailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipCallCoordinatorImpl: Sized {
    fn ReserveCallResourcesAsync(&self, taskentrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>;
    fn MuteStateChanged(&self, mutechangehandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMuteStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestNewIncomingCall(&self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, contactimage: &::core::option::Option<super::super::Foundation::Uri>, servicename: &::windows::core::HSTRING, brandingimage: &::core::option::Option<super::super::Foundation::Uri>, calldetails: &::windows::core::HSTRING, ringtone: &::core::option::Option<super::super::Foundation::Uri>, media: VoipPhoneCallMedia, ringtimeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<VoipPhoneCall>;
    fn RequestNewOutgoingCall(&self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall>;
    fn NotifyMuted(&self) -> ::windows::core::Result<()>;
    fn NotifyUnmuted(&self) -> ::windows::core::Result<()>;
    fn RequestOutgoingUpgradeToVideoCall(&self, callupgradeguid: &::windows::core::GUID, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<VoipPhoneCall>;
    fn RequestIncomingUpgradeToVideoCall(&self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, contactimage: &::core::option::Option<super::super::Foundation::Uri>, servicename: &::windows::core::HSTRING, brandingimage: &::core::option::Option<super::super::Foundation::Uri>, calldetails: &::windows::core::HSTRING, ringtone: &::core::option::Option<super::super::Foundation::Uri>, ringtimeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<VoipPhoneCall>;
    fn TerminateCellularCall(&self, callupgradeguid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CancelUpgrade(&self, callupgradeguid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipCallCoordinator2Impl: Sized + IVoipCallCoordinatorImpl {
    fn SetupNewAcceptedCall(&self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipCallCoordinator3Impl: Sized + IVoipCallCoordinatorImpl {
    fn RequestNewAppInitiatedCall(&self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall>;
    fn RequestNewIncomingCallWithContactRemoteId(&self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, contactimage: &::core::option::Option<super::super::Foundation::Uri>, servicename: &::windows::core::HSTRING, brandingimage: &::core::option::Option<super::super::Foundation::Uri>, calldetails: &::windows::core::HSTRING, ringtone: &::core::option::Option<super::super::Foundation::Uri>, media: VoipPhoneCallMedia, ringtimeout: &super::super::Foundation::TimeSpan, contactremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<VoipPhoneCall>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipCallCoordinator4Impl: Sized + IVoipCallCoordinatorImpl {
    fn ReserveOneProcessCallResourcesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipCallCoordinatorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<VoipCallCoordinator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipPhoneCallImpl: Sized {
    fn EndRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEndRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResumeRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResumeRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AnswerRequested(&self, accepthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnswerRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RejectRequested(&self, rejecthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRejectRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyCallHeld(&self) -> ::windows::core::Result<()>;
    fn NotifyCallActive(&self) -> ::windows::core::Result<()>;
    fn NotifyCallEnded(&self) -> ::windows::core::Result<()>;
    fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn CallMedia(&self) -> ::windows::core::Result<VoipPhoneCallMedia>;
    fn SetCallMedia(&self, value: VoipPhoneCallMedia) -> ::windows::core::Result<()>;
    fn NotifyCallReady(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipPhoneCall2Impl: Sized + IVoipPhoneCallImpl {
    fn TryShowAppUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipPhoneCall3Impl: Sized + IVoipPhoneCallImpl + IVoipPhoneCall2Impl {
    fn NotifyCallAccepted(&self, media: VoipPhoneCallMedia) -> ::windows::core::Result<()>;
}
