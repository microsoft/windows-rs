pub trait INetworkTransportSettingsImpl: Sized {
    fn ApplySetting();
    fn QuerySetting();
}
pub trait INotificationTransportSyncImpl: Sized {
    fn CompleteDelivery();
    fn Flush();
}
pub trait IRTCBuddyImpl: Sized + IRTCPresenceContactImpl {
    fn Status();
    fn Notes();
}
pub trait IRTCBuddy2Impl: Sized + IRTCBuddyImpl + IRTCPresenceContactImpl {
    fn Profile();
    fn Refresh();
    fn EnumerateGroups();
    fn Groups();
    fn PresenceProperty();
    fn EnumeratePresenceDevices();
    fn PresenceDevices();
    fn SubscriptionType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyEventImpl: Sized + IDispatchImpl {
    fn Buddy();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyEvent2Impl: Sized + IRTCBuddyEventImpl + IDispatchImpl {
    fn EventType();
    fn StatusCode();
    fn StatusText();
}
pub trait IRTCBuddyGroupImpl: Sized {
    fn Name();
    fn SetName();
    fn AddBuddy();
    fn RemoveBuddy();
    fn EnumerateBuddies();
    fn Buddies();
    fn Data();
    fn SetData();
    fn Profile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyGroupEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Group();
    fn Buddy();
    fn StatusCode();
}
pub trait IRTCClientImpl: Sized {
    fn Initialize();
    fn Shutdown();
    fn PrepareForShutdown();
    fn SetEventFilter();
    fn EventFilter();
    fn SetPreferredMediaTypes();
    fn PreferredMediaTypes();
    fn MediaCapabilities();
    fn CreateSession();
    fn SetListenForIncomingSessions();
    fn ListenForIncomingSessions();
    fn NetworkAddresses();
    fn SetVolume();
    fn Volume();
    fn SetAudioMuted();
    fn AudioMuted();
    fn IVideoWindow();
    fn SetPreferredAudioDevice();
    fn PreferredAudioDevice();
    fn SetPreferredVolume();
    fn PreferredVolume();
    fn SetPreferredAEC();
    fn PreferredAEC();
    fn SetPreferredVideoDevice();
    fn PreferredVideoDevice();
    fn ActiveMedia();
    fn SetMaxBitrate();
    fn MaxBitrate();
    fn SetTemporalSpatialTradeOff();
    fn TemporalSpatialTradeOff();
    fn NetworkQuality();
    fn StartT120Applet();
    fn StopT120Applets();
    fn IsT120AppletRunning();
    fn LocalUserURI();
    fn SetLocalUserURI();
    fn LocalUserName();
    fn SetLocalUserName();
    fn PlayRing();
    fn SendDTMF();
    fn InvokeTuningWizard();
    fn IsTuned();
}
pub trait IRTCClient2Impl: Sized + IRTCClientImpl {
    fn SetAnswerMode();
    fn AnswerMode();
    fn InvokeTuningWizardEx();
    fn Version();
    fn SetClientName();
    fn SetClientCurVer();
    fn InitializeEx();
    fn CreateSessionWithDescription();
    fn SetSessionDescriptionManager();
    fn SetPreferredSecurityLevel();
    fn PreferredSecurityLevel();
    fn SetAllowedPorts();
    fn AllowedPorts();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Client();
}
pub trait IRTCClientPortManagementImpl: Sized {
    fn StartListenAddressAndPort();
    fn StopListenAddressAndPort();
    fn GetPortRange();
}
pub trait IRTCClientPresenceImpl: Sized {
    fn EnablePresence();
    fn Export();
    fn Import();
    fn EnumerateBuddies();
    fn Buddies();
    fn Buddy();
    fn AddBuddy();
    fn RemoveBuddy();
    fn EnumerateWatchers();
    fn Watchers();
    fn Watcher();
    fn AddWatcher();
    fn RemoveWatcher();
    fn SetLocalPresenceInfo();
    fn OfferWatcherMode();
    fn SetOfferWatcherMode();
    fn PrivacyMode();
    fn SetPrivacyMode();
}
pub trait IRTCClientPresence2Impl: Sized + IRTCClientPresenceImpl {
    fn EnablePresenceEx();
    fn DisablePresence();
    fn AddGroup();
    fn RemoveGroup();
    fn EnumerateGroups();
    fn Groups();
    fn Group();
    fn AddWatcherEx();
    fn WatcherEx();
    fn SetPresenceProperty();
    fn PresenceProperty();
    fn SetPresenceData();
    fn GetPresenceData();
    fn GetLocalPresenceInfo();
    fn AddBuddyEx();
}
pub trait IRTCClientProvisioningImpl: Sized {
    fn CreateProfile();
    fn EnableProfile();
    fn DisableProfile();
    fn EnumerateProfiles();
    fn Profiles();
    fn GetProfile();
    fn SessionCapabilities();
}
pub trait IRTCClientProvisioning2Impl: Sized + IRTCClientProvisioningImpl {
    fn EnableProfileEx();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCDispatchEventNotificationImpl: Sized + IDispatchImpl {}
pub trait IRTCEnumBuddiesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IRTCEnumGroupsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IRTCEnumParticipantsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IRTCEnumPresenceDevicesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IRTCEnumProfilesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IRTCEnumUserSearchResultsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IRTCEnumWatchersImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IRTCEventNotificationImpl: Sized {
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCInfoEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Participant();
    fn Info();
    fn InfoHeader();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCIntensityEventImpl: Sized + IDispatchImpl {
    fn Level();
    fn Min();
    fn Max();
    fn Direction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMediaEventImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn EventType();
    fn EventReason();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMediaRequestEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ProposedMedia();
    fn CurrentMedia();
    fn Accept();
    fn RemotePreferredSecurityLevel();
    fn Reject();
    fn State();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMessagingEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Participant();
    fn EventType();
    fn Message();
    fn MessageHeader();
    fn UserStatus();
}
pub trait IRTCParticipantImpl: Sized {
    fn UserURI();
    fn Name();
    fn Removable();
    fn State();
    fn Session();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCParticipantStateChangeEventImpl: Sized + IDispatchImpl {
    fn Participant();
    fn State();
    fn StatusCode();
}
pub trait IRTCPortManagerImpl: Sized {
    fn GetMapping();
    fn UpdateRemoteAddress();
    fn ReleaseMapping();
}
pub trait IRTCPresenceContactImpl: Sized {
    fn PresentityURI();
    fn SetPresentityURI();
    fn Name();
    fn SetName();
    fn Data();
    fn SetData();
    fn Persistent();
    fn SetPersistent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresenceDataEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn GetPresenceData();
}
pub trait IRTCPresenceDeviceImpl: Sized {
    fn Status();
    fn Notes();
    fn PresenceProperty();
    fn GetPresenceData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresencePropertyEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn PresenceProperty();
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresenceStatusEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn GetLocalPresenceInfo();
}
pub trait IRTCProfileImpl: Sized {
    fn Key();
    fn Name();
    fn XML();
    fn ProviderName();
    fn ProviderURI();
    fn ProviderData();
    fn ClientName();
    fn ClientBanner();
    fn ClientMinVer();
    fn ClientCurVer();
    fn ClientUpdateURI();
    fn ClientData();
    fn UserURI();
    fn UserName();
    fn UserAccount();
    fn SetCredentials();
    fn SessionCapabilities();
    fn State();
}
pub trait IRTCProfile2Impl: Sized + IRTCProfileImpl {
    fn Realm();
    fn SetRealm();
    fn AllowedAuth();
    fn SetAllowedAuth();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCProfileEventImpl: Sized + IDispatchImpl {
    fn Profile();
    fn Cookie();
    fn StatusCode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCProfileEvent2Impl: Sized + IRTCProfileEventImpl + IDispatchImpl {
    fn EventType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCReInviteEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Accept();
    fn Reject();
    fn State();
    fn GetRemoteSessionDescription();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCRegistrationStateChangeEventImpl: Sized + IDispatchImpl {
    fn Profile();
    fn State();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCRoamingEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Profile();
    fn StatusCode();
    fn StatusText();
}
pub trait IRTCSessionImpl: Sized {
    fn Client();
    fn State();
    fn Type();
    fn Profile();
    fn Participants();
    fn Answer();
    fn Terminate();
    fn Redirect();
    fn AddParticipant();
    fn RemoveParticipant();
    fn EnumerateParticipants();
    fn CanAddParticipants();
    fn RedirectedUserURI();
    fn RedirectedUserName();
    fn NextRedirectedUser();
    fn SendMessage();
    fn SendMessageStatus();
    fn AddStream();
    fn RemoveStream();
    fn SetEncryptionKey();
}
pub trait IRTCSession2Impl: Sized + IRTCSessionImpl {
    fn SendInfo();
    fn SetPreferredSecurityLevel();
    fn PreferredSecurityLevel();
    fn IsSecurityEnabled();
    fn AnswerWithSessionDescription();
    fn ReInviteWithSessionDescription();
}
pub trait IRTCSessionCallControlImpl: Sized {
    fn Hold();
    fn UnHold();
    fn Forward();
    fn Refer();
    fn SetReferredByURI();
    fn ReferredByURI();
    fn SetReferCookie();
    fn ReferCookie();
    fn IsReferred();
}
pub trait IRTCSessionDescriptionManagerImpl: Sized {
    fn EvaluateSessionDescription();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionOperationCompleteEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Cookie();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionOperationCompleteEvent2Impl: Sized + IRTCSessionOperationCompleteEventImpl + IDispatchImpl {
    fn Participant();
    fn GetRemoteSessionDescription();
}
pub trait IRTCSessionPortManagementImpl: Sized {
    fn SetPortManager();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionReferStatusEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ReferStatus();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionReferredEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ReferredByURI();
    fn ReferToURI();
    fn ReferCookie();
    fn Accept();
    fn Reject();
    fn SetReferredSessionState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionStateChangeEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn State();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionStateChangeEvent2Impl: Sized + IRTCSessionStateChangeEventImpl + IDispatchImpl {
    fn MediaTypes();
    fn RemotePreferredSecurityLevel();
    fn IsForked();
    fn GetRemoteSessionDescription();
}
pub trait IRTCUserSearchImpl: Sized {
    fn CreateQuery();
    fn ExecuteSearch();
}
pub trait IRTCUserSearchQueryImpl: Sized {
    fn SetSearchTerm();
    fn SearchTerm();
    fn SearchTerms();
    fn SetSearchPreference();
    fn SearchPreference();
    fn SetSearchDomain();
    fn SearchDomain();
}
pub trait IRTCUserSearchResultImpl: Sized {
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCUserSearchResultsEventImpl: Sized + IDispatchImpl {
    fn EnumerateResults();
    fn Results();
    fn Profile();
    fn Query();
    fn Cookie();
    fn StatusCode();
    fn MoreAvailable();
}
pub trait IRTCWatcherImpl: Sized + IRTCPresenceContactImpl {
    fn State();
    fn SetState();
}
pub trait IRTCWatcher2Impl: Sized + IRTCWatcherImpl + IRTCPresenceContactImpl {
    fn Profile();
    fn Scope();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCWatcherEventImpl: Sized + IDispatchImpl {
    fn Watcher();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCWatcherEvent2Impl: Sized + IRTCWatcherEventImpl + IDispatchImpl {
    fn EventType();
    fn StatusCode();
}
pub trait ITransportSettingsInternalImpl: Sized {
    fn ApplySetting();
    fn QuerySetting();
}
