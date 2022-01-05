pub trait IEnumACDGroupImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumAddressImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumAgentImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumAgentHandlerImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumAgentSessionImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumBstrImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumCallImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumCallHubImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumCallingCardImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumDialableAddrsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumDirectoryImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumDirectoryObjectImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumLocationImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumMcastScopeImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumPhoneImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumPluggableSuperclassInfoImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumPluggableTerminalClassInfoImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumQueueImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumStreamImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumSubStreamImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumTerminalImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
pub trait IEnumTerminalClassImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastAddressAllocationImpl: Sized + IDispatchImpl {
    fn Scopes();
    fn EnumerateScopes();
    fn RequestAddress();
    fn RenewAddress();
    fn ReleaseAddress();
    fn CreateLeaseInfo();
    fn CreateLeaseInfoFromVariant();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastLeaseInfoImpl: Sized + IDispatchImpl {
    fn RequestID();
    fn LeaseStartTime();
    fn SetLeaseStartTime();
    fn LeaseStopTime();
    fn SetLeaseStopTime();
    fn AddressCount();
    fn ServerAddress();
    fn TTL();
    fn Addresses();
    fn EnumerateAddresses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastScopeImpl: Sized + IDispatchImpl {
    fn ScopeID();
    fn ServerID();
    fn InterfaceID();
    fn ScopeDescription();
    fn TTL();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITACDGroupImpl: Sized + IDispatchImpl {
    fn Name();
    fn EnumerateQueues();
    fn Queues();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITACDGroupEventImpl: Sized + IDispatchImpl {
    fn Group();
    fn Event();
}
pub trait ITAMMediaFormatImpl: Sized {
    fn MediaFormat();
    fn SetMediaFormat();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITASRTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressImpl: Sized + IDispatchImpl {
    fn State();
    fn AddressName();
    fn ServiceProviderName();
    fn TAPIObject();
    fn CreateCall();
    fn Calls();
    fn EnumerateCalls();
    fn DialableAddress();
    fn CreateForwardInfoObject();
    fn Forward();
    fn CurrentForwardInfo();
    fn SetMessageWaiting();
    fn MessageWaiting();
    fn SetDoNotDisturb();
    fn DoNotDisturb();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddress2Impl: Sized + ITAddressImpl + IDispatchImpl {
    fn Phones();
    fn EnumeratePhones();
    fn GetPhoneFromTerminal();
    fn PreferredPhones();
    fn EnumeratePreferredPhones();
    fn EventFilter();
    fn SetEventFilter();
    fn DeviceSpecific();
    fn DeviceSpecificVariant();
    fn NegotiateExtVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressCapabilitiesImpl: Sized + IDispatchImpl {
    fn AddressCapability();
    fn AddressCapabilityString();
    fn CallTreatments();
    fn EnumerateCallTreatments();
    fn CompletionMessages();
    fn EnumerateCompletionMessages();
    fn DeviceClasses();
    fn EnumerateDeviceClasses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Call();
    fn lParam1();
    fn lParam2();
    fn lParam3();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Event();
    fn Terminal();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressTranslationImpl: Sized + IDispatchImpl {
    fn TranslateAddress();
    fn TranslateDialog();
    fn EnumerateLocations();
    fn Locations();
    fn EnumerateCallingCards();
    fn CallingCards();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressTranslationInfoImpl: Sized + IDispatchImpl {
    fn DialableString();
    fn DisplayableString();
    fn CurrentCountryCode();
    fn DestinationCountryCode();
    fn TranslationResults();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentImpl: Sized + IDispatchImpl {
    fn EnumerateAgentSessions();
    fn CreateSession();
    fn CreateSessionWithPIN();
    fn ID();
    fn User();
    fn SetState();
    fn State();
    fn SetMeasurementPeriod();
    fn MeasurementPeriod();
    fn OverallCallRate();
    fn NumberOfACDCalls();
    fn NumberOfIncomingCalls();
    fn NumberOfOutgoingCalls();
    fn TotalACDTalkTime();
    fn TotalACDCallTime();
    fn TotalWrapUpTime();
    fn AgentSessions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentEventImpl: Sized + IDispatchImpl {
    fn Agent();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentHandlerImpl: Sized + IDispatchImpl {
    fn Name();
    fn CreateAgent();
    fn CreateAgentWithID();
    fn EnumerateACDGroups();
    fn EnumerateUsableAddresses();
    fn ACDGroups();
    fn UsableAddresses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentHandlerEventImpl: Sized + IDispatchImpl {
    fn AgentHandler();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentSessionImpl: Sized + IDispatchImpl {
    fn Agent();
    fn Address();
    fn ACDGroup();
    fn SetState();
    fn State();
    fn SessionStartTime();
    fn SessionDuration();
    fn NumberOfCalls();
    fn TotalTalkTime();
    fn AverageTalkTime();
    fn TotalCallTime();
    fn AverageCallTime();
    fn TotalWrapUpTime();
    fn AverageWrapUpTime();
    fn ACDCallRate();
    fn LongestTimeToAnswer();
    fn AverageTimeToAnswer();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentSessionEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Event();
}
pub trait ITAllocatorPropertiesImpl: Sized {
    fn SetAllocatorProperties();
    fn GetAllocatorProperties();
    fn SetAllocateBuffers();
    fn GetAllocateBuffers();
    fn SetBufferSize();
    fn GetBufferSize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAutomatedPhoneControlImpl: Sized + IDispatchImpl {
    fn StartTone();
    fn StopTone();
    fn Tone();
    fn StartRinger();
    fn StopRinger();
    fn Ringer();
    fn SetPhoneHandlingEnabled();
    fn PhoneHandlingEnabled();
    fn SetAutoEndOfNumberTimeout();
    fn AutoEndOfNumberTimeout();
    fn SetAutoDialtone();
    fn AutoDialtone();
    fn SetAutoStopTonesOnOnHook();
    fn AutoStopTonesOnOnHook();
    fn SetAutoStopRingOnOffHook();
    fn AutoStopRingOnOffHook();
    fn SetAutoKeypadTones();
    fn AutoKeypadTones();
    fn SetAutoKeypadTonesMinimumDuration();
    fn AutoKeypadTonesMinimumDuration();
    fn SetAutoVolumeControl();
    fn AutoVolumeControl();
    fn SetAutoVolumeControlStep();
    fn AutoVolumeControlStep();
    fn SetAutoVolumeControlRepeatDelay();
    fn AutoVolumeControlRepeatDelay();
    fn SetAutoVolumeControlRepeatPeriod();
    fn AutoVolumeControlRepeatPeriod();
    fn SelectCall();
    fn UnselectCall();
    fn EnumerateSelectedCalls();
    fn SelectedCalls();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicAudioTerminalImpl: Sized + IDispatchImpl {
    fn SetVolume();
    fn Volume();
    fn SetBalance();
    fn Balance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicCallControlImpl: Sized + IDispatchImpl {
    fn Connect();
    fn Answer();
    fn Disconnect();
    fn Hold();
    fn HandoffDirect();
    fn HandoffIndirect();
    fn Conference();
    fn Transfer();
    fn BlindTransfer();
    fn SwapHold();
    fn ParkDirect();
    fn ParkIndirect();
    fn Unpark();
    fn SetQOS();
    fn Pickup();
    fn Dial();
    fn Finish();
    fn RemoveFromConference();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicCallControl2Impl: Sized + ITBasicCallControlImpl + IDispatchImpl {
    fn RequestTerminal();
    fn SelectTerminalOnCall();
    fn UnselectTerminalOnCall();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallHubImpl: Sized + IDispatchImpl {
    fn Clear();
    fn EnumerateCalls();
    fn Calls();
    fn NumCalls();
    fn State();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallHubEventImpl: Sized + IDispatchImpl {
    fn Event();
    fn CallHub();
    fn Call();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfoImpl: Sized + IDispatchImpl {
    fn Address();
    fn CallState();
    fn Privilege();
    fn CallHub();
    fn CallInfoLong();
    fn SetCallInfoLong();
    fn CallInfoString();
    fn SetCallInfoString();
    fn CallInfoBuffer();
    fn SetCallInfoBuffer();
    fn GetCallInfoBuffer();
    fn SetCallInfoBuffer();
    fn ReleaseUserUserInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfo2Impl: Sized + ITCallInfoImpl + IDispatchImpl {
    fn EventFilter();
    fn SetEventFilter();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfoChangeEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Cause();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallMediaEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn Error();
    fn Terminal();
    fn Stream();
    fn Cause();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallNotificationEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallStateEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn State();
    fn Cause();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallingCardImpl: Sized + IDispatchImpl {
    fn PermanentCardID();
    fn NumberOfDigits();
    fn Options();
    fn CardName();
    fn SameAreaDialingRule();
    fn LongDistanceDialingRule();
    fn InternationalDialingRule();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCollection2Impl: Sized + ITCollectionImpl + IDispatchImpl {
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCustomToneImpl: Sized + IDispatchImpl {
    fn Frequency();
    fn SetFrequency();
    fn CadenceOn();
    fn SetCadenceOn();
    fn CadenceOff();
    fn SetCadenceOff();
    fn Volume();
    fn SetVolume();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDetectToneImpl: Sized + IDispatchImpl {
    fn AppSpecific();
    fn SetAppSpecific();
    fn Duration();
    fn SetDuration();
    fn Frequency();
    fn SetFrequency();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitDetectionEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Digit();
    fn DigitMode();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitGenerationEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn GenerationTermination();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitsGatheredEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Digits();
    fn GatherTermination();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryImpl: Sized + IDispatchImpl {
    fn DirectoryType();
    fn DisplayName();
    fn IsDynamic();
    fn DefaultObjectTTL();
    fn SetDefaultObjectTTL();
    fn EnableAutoRefresh();
    fn Connect();
    fn Bind();
    fn AddDirectoryObject();
    fn ModifyDirectoryObject();
    fn RefreshDirectoryObject();
    fn DeleteDirectoryObject();
    fn DirectoryObjects();
    fn EnumerateDirectoryObjects();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectImpl: Sized + IDispatchImpl {
    fn ObjectType();
    fn Name();
    fn SetName();
    fn DialableAddrs();
    fn EnumerateDialableAddrs();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectConferenceImpl: Sized + IDispatchImpl {
    fn Protocol();
    fn Originator();
    fn SetOriginator();
    fn AdvertisingScope();
    fn SetAdvertisingScope();
    fn Url();
    fn SetUrl();
    fn Description();
    fn SetDescription();
    fn IsEncrypted();
    fn SetIsEncrypted();
    fn StartTime();
    fn SetStartTime();
    fn StopTime();
    fn SetStopTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectUserImpl: Sized + IDispatchImpl {
    fn IPPhonePrimary();
    fn SetIPPhonePrimary();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDispatchMapperImpl: Sized + IDispatchImpl {
    fn QueryDispatchInterface();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITFileTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Track();
    fn Call();
    fn State();
    fn Cause();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITFileTrackImpl: Sized + IDispatchImpl {
    fn Format();
    fn SetFormat();
    fn ControllingTerminal();
    fn AudioFormatForScripting();
    fn SetAudioFormatForScripting();
    fn EmptyAudioFormatForScripting();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITForwardInformationImpl: Sized + IDispatchImpl {
    fn SetNumRingsNoAnswer();
    fn NumRingsNoAnswer();
    fn SetForwardType();
    fn ForwardTypeDestination();
    fn ForwardTypeCaller();
    fn GetForwardType();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITForwardInformation2Impl: Sized + ITForwardInformationImpl + IDispatchImpl {
    fn SetForwardType2();
    fn GetForwardType2();
    fn ForwardTypeDestinationAddressType();
    fn ForwardTypeCallerAddressType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITILSConfigImpl: Sized + IDispatchImpl {
    fn Port();
    fn SetPort();
}
pub trait ITLegacyAddressMediaControlImpl: Sized {
    fn GetID();
    fn GetDevConfig();
    fn SetDevConfig();
}
pub trait ITLegacyAddressMediaControl2Impl: Sized + ITLegacyAddressMediaControlImpl {
    fn ConfigDialog();
    fn ConfigDialogEdit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyCallMediaControlImpl: Sized + IDispatchImpl {
    fn DetectDigits();
    fn GenerateDigits();
    fn GetID();
    fn SetMediaType();
    fn MonitorMedia();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyCallMediaControl2Impl: Sized + ITLegacyCallMediaControlImpl + IDispatchImpl {
    fn GenerateDigits2();
    fn GatherDigits();
    fn DetectTones();
    fn DetectTonesByCollection();
    fn GenerateTone();
    fn GenerateCustomTones();
    fn GenerateCustomTonesByCollection();
    fn CreateDetectToneObject();
    fn CreateCustomToneObject();
    fn GetIDAsVariant();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyWaveSupportImpl: Sized + IDispatchImpl {
    fn IsFullDuplex();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLocationInfoImpl: Sized + IDispatchImpl {
    fn PermanentLocationID();
    fn CountryCode();
    fn CountryID();
    fn Options();
    fn PreferredCardID();
    fn LocationName();
    fn CityCode();
    fn LocalAccessCode();
    fn LongDistanceAccessCode();
    fn TollPrefixList();
    fn CancelCallWaitingCode();
}
pub trait ITMSPAddressImpl: Sized {
    fn Initialize();
    fn Shutdown();
    fn CreateMSPCall();
    fn ShutdownMSPCall();
    fn ReceiveTSPData();
    fn GetEvent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaControlImpl: Sized + IDispatchImpl {
    fn Start();
    fn Stop();
    fn Pause();
    fn MediaState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaPlaybackImpl: Sized + IDispatchImpl {
    fn SetPlayList();
    fn PlayList();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaRecordImpl: Sized + IDispatchImpl {
    fn SetFileName();
    fn FileName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaSupportImpl: Sized + IDispatchImpl {
    fn MediaTypes();
    fn QueryMediaType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMultiTrackTerminalImpl: Sized + IDispatchImpl {
    fn TrackTerminals();
    fn EnumerateTrackTerminals();
    fn CreateTrackTerminal();
    fn MediaTypesInUse();
    fn DirectionsInUse();
    fn RemoveTrackTerminal();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneImpl: Sized + IDispatchImpl {
    fn Open();
    fn Close();
    fn Addresses();
    fn EnumerateAddresses();
    fn PhoneCapsLong();
    fn PhoneCapsString();
    fn Terminals();
    fn EnumerateTerminals();
    fn ButtonMode();
    fn SetButtonMode();
    fn ButtonFunction();
    fn SetButtonFunction();
    fn ButtonText();
    fn SetButtonText();
    fn ButtonState();
    fn HookSwitchState();
    fn SetHookSwitchState();
    fn SetRingMode();
    fn RingMode();
    fn SetRingVolume();
    fn RingVolume();
    fn Privilege();
    fn GetPhoneCapsBuffer();
    fn PhoneCapsBuffer();
    fn LampMode();
    fn SetLampMode();
    fn Display();
    fn SetDisplay();
    fn PreferredAddresses();
    fn EnumeratePreferredAddresses();
    fn DeviceSpecific();
    fn DeviceSpecificVariant();
    fn NegotiateExtVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Phone();
    fn lParam1();
    fn lParam2();
    fn lParam3();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneEventImpl: Sized + IDispatchImpl {
    fn Phone();
    fn Event();
    fn ButtonState();
    fn HookSwitchState();
    fn HookSwitchDevice();
    fn RingMode();
    fn ButtonLampId();
    fn NumberGathered();
    fn Call();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalClassInfoImpl: Sized + IDispatchImpl {
    fn Name();
    fn Company();
    fn Version();
    fn TerminalClass();
    fn CLSID();
    fn Direction();
    fn MediaTypes();
}
pub trait ITPluggableTerminalEventSinkImpl: Sized {
    fn FireEvent();
}
pub trait ITPluggableTerminalEventSinkRegistrationImpl: Sized {
    fn RegisterSink();
    fn UnregisterSink();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalSuperclassInfoImpl: Sized + IDispatchImpl {
    fn Name();
    fn CLSID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPrivateEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Call();
    fn CallHub();
    fn EventCode();
    fn EventInterface();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQOSEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn MediaType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQueueImpl: Sized + IDispatchImpl {
    fn SetMeasurementPeriod();
    fn MeasurementPeriod();
    fn TotalCallsQueued();
    fn CurrentCallsQueued();
    fn TotalCallsAbandoned();
    fn TotalCallsFlowedIn();
    fn TotalCallsFlowedOut();
    fn LongestEverWaitTime();
    fn CurrentLongestWaitTime();
    fn AverageWaitTime();
    fn FinalDisposition();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQueueEventImpl: Sized + IDispatchImpl {
    fn Queue();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRendezvousImpl: Sized + IDispatchImpl {
    fn DefaultDirectories();
    fn EnumerateDefaultDirectories();
    fn CreateDirectory();
    fn CreateDirectoryObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRequestImpl: Sized + IDispatchImpl {
    fn MakeCall();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRequestEventImpl: Sized + IDispatchImpl {
    fn RegistrationInstance();
    fn RequestMode();
    fn DestAddress();
    fn AppName();
    fn CalledParty();
    fn Comment();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITScriptableAudioFormatImpl: Sized + IDispatchImpl {
    fn Channels();
    fn SetChannels();
    fn SamplesPerSec();
    fn SetSamplesPerSec();
    fn AvgBytesPerSec();
    fn SetAvgBytesPerSec();
    fn BlockAlign();
    fn SetBlockAlign();
    fn BitsPerSample();
    fn SetBitsPerSample();
    fn FormatTag();
    fn SetFormatTag();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStaticAudioTerminalImpl: Sized + IDispatchImpl {
    fn WaveId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStreamImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn Direction();
    fn Name();
    fn StartStream();
    fn PauseStream();
    fn StopStream();
    fn SelectTerminal();
    fn UnselectTerminal();
    fn EnumerateTerminals();
    fn Terminals();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStreamControlImpl: Sized + IDispatchImpl {
    fn CreateStream();
    fn RemoveStream();
    fn EnumerateStreams();
    fn Streams();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITSubStreamImpl: Sized + IDispatchImpl {
    fn StartSubStream();
    fn PauseSubStream();
    fn StopSubStream();
    fn SelectTerminal();
    fn UnselectTerminal();
    fn EnumerateTerminals();
    fn Terminals();
    fn Stream();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITSubStreamControlImpl: Sized + IDispatchImpl {
    fn CreateSubStream();
    fn RemoveSubStream();
    fn EnumerateSubStreams();
    fn SubStreams();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Shutdown();
    fn Addresses();
    fn EnumerateAddresses();
    fn RegisterCallNotifications();
    fn UnregisterNotifications();
    fn CallHubs();
    fn EnumerateCallHubs();
    fn SetCallHubTracking();
    fn EnumeratePrivateTAPIObjects();
    fn PrivateTAPIObjects();
    fn RegisterRequestRecipient();
    fn SetAssistedTelephonyPriority();
    fn SetApplicationPriority();
    fn SetEventFilter();
    fn EventFilter();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPI2Impl: Sized + ITTAPIImpl + IDispatchImpl {
    fn Phones();
    fn EnumeratePhones();
    fn CreateEmptyCollectionObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPICallCenterImpl: Sized + IDispatchImpl {
    fn EnumerateAgentHandlers();
    fn AgentHandlers();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIDispatchEventNotificationImpl: Sized + IDispatchImpl {}
pub trait ITTAPIEventNotificationImpl: Sized {
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIObjectEventImpl: Sized + IDispatchImpl {
    fn TAPIObject();
    fn Event();
    fn Address();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIObjectEvent2Impl: Sized + ITTAPIObjectEventImpl + IDispatchImpl {
    fn Phone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTTSTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalImpl: Sized + IDispatchImpl {
    fn Name();
    fn State();
    fn TerminalType();
    fn TerminalClass();
    fn MediaType();
    fn Direction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalSupportImpl: Sized + IDispatchImpl {
    fn StaticTerminals();
    fn EnumerateStaticTerminals();
    fn DynamicTerminalClasses();
    fn EnumerateDynamicTerminalClasses();
    fn CreateTerminal();
    fn GetDefaultStaticTerminal();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalSupport2Impl: Sized + ITTerminalSupportImpl + IDispatchImpl {
    fn PluggableSuperclasses();
    fn EnumeratePluggableSuperclasses();
    fn PluggableTerminalClasses();
    fn EnumeratePluggableTerminalClasses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITToneDetectionEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn AppSpecific();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITToneTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
pub trait ITnefImpl: Sized {
    fn AddProps();
    fn ExtractProps();
    fn Finish();
    fn OpenTaggedBody();
    fn SetProps();
    fn EncodeRecips();
    fn FinishComponent();
}
