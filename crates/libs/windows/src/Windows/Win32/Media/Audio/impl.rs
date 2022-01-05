pub trait IActivateAudioInterfaceAsyncOperationImpl: Sized {
    fn GetActivateResult();
}
pub trait IActivateAudioInterfaceCompletionHandlerImpl: Sized {
    fn ActivateCompleted();
}
pub trait IAudioAmbisonicsControlImpl: Sized {
    fn SetData();
    fn SetHeadTracking();
    fn GetHeadTracking();
    fn SetRotation();
}
pub trait IAudioAutoGainControlImpl: Sized {
    fn GetEnabled();
    fn SetEnabled();
}
pub trait IAudioBassImpl: Sized + IPerChannelDbLevelImpl {}
pub trait IAudioCaptureClientImpl: Sized {
    fn GetBuffer();
    fn ReleaseBuffer();
    fn GetNextPacketSize();
}
pub trait IAudioChannelConfigImpl: Sized {
    fn SetChannelConfig();
    fn GetChannelConfig();
}
pub trait IAudioClientImpl: Sized {
    fn Initialize();
    fn GetBufferSize();
    fn GetStreamLatency();
    fn GetCurrentPadding();
    fn IsFormatSupported();
    fn GetMixFormat();
    fn GetDevicePeriod();
    fn Start();
    fn Stop();
    fn Reset();
    fn SetEventHandle();
    fn GetService();
}
pub trait IAudioClient2Impl: Sized + IAudioClientImpl {
    fn IsOffloadCapable();
    fn SetClientProperties();
    fn GetBufferSizeLimits();
}
pub trait IAudioClient3Impl: Sized + IAudioClient2Impl + IAudioClientImpl {
    fn GetSharedModeEnginePeriod();
    fn GetCurrentSharedModeEnginePeriod();
    fn InitializeSharedAudioStream();
}
pub trait IAudioClientDuckingControlImpl: Sized {
    fn SetDuckingOptionsForCurrentStream();
}
pub trait IAudioClockImpl: Sized {
    fn GetFrequency();
    fn GetPosition();
    fn GetCharacteristics();
}
pub trait IAudioClock2Impl: Sized {
    fn GetDevicePosition();
}
pub trait IAudioClockAdjustmentImpl: Sized {
    fn SetSampleRate();
}
pub trait IAudioEffectsChangedNotificationClientImpl: Sized {
    fn OnAudioEffectsChanged();
}
pub trait IAudioEffectsManagerImpl: Sized {
    fn RegisterAudioEffectsChangedNotificationCallback();
    fn UnregisterAudioEffectsChangedNotificationCallback();
    fn GetAudioEffects();
    fn SetAudioEffectState();
}
pub trait IAudioFormatEnumeratorImpl: Sized {
    fn GetCount();
    fn GetFormat();
}
pub trait IAudioInputSelectorImpl: Sized {
    fn GetSelection();
    fn SetSelection();
}
pub trait IAudioLoudnessImpl: Sized {
    fn GetEnabled();
    fn SetEnabled();
}
pub trait IAudioMidrangeImpl: Sized + IPerChannelDbLevelImpl {}
pub trait IAudioMuteImpl: Sized {
    fn SetMute();
    fn GetMute();
}
pub trait IAudioOutputSelectorImpl: Sized {
    fn GetSelection();
    fn SetSelection();
}
pub trait IAudioPeakMeterImpl: Sized {
    fn GetChannelCount();
    fn GetLevel();
}
pub trait IAudioRenderClientImpl: Sized {
    fn GetBuffer();
    fn ReleaseBuffer();
}
pub trait IAudioSessionControlImpl: Sized {
    fn GetState();
    fn GetDisplayName();
    fn SetDisplayName();
    fn GetIconPath();
    fn SetIconPath();
    fn GetGroupingParam();
    fn SetGroupingParam();
    fn RegisterAudioSessionNotification();
    fn UnregisterAudioSessionNotification();
}
pub trait IAudioSessionControl2Impl: Sized + IAudioSessionControlImpl {
    fn GetSessionIdentifier();
    fn GetSessionInstanceIdentifier();
    fn GetProcessId();
    fn IsSystemSoundsSession();
    fn SetDuckingPreference();
}
pub trait IAudioSessionEnumeratorImpl: Sized {
    fn GetCount();
    fn GetSession();
}
pub trait IAudioSessionEventsImpl: Sized {
    fn OnDisplayNameChanged();
    fn OnIconPathChanged();
    fn OnSimpleVolumeChanged();
    fn OnChannelVolumeChanged();
    fn OnGroupingParamChanged();
    fn OnStateChanged();
    fn OnSessionDisconnected();
}
pub trait IAudioSessionManagerImpl: Sized {
    fn GetAudioSessionControl();
    fn GetSimpleAudioVolume();
}
pub trait IAudioSessionManager2Impl: Sized + IAudioSessionManagerImpl {
    fn GetSessionEnumerator();
    fn RegisterSessionNotification();
    fn UnregisterSessionNotification();
    fn RegisterDuckNotification();
    fn UnregisterDuckNotification();
}
pub trait IAudioSessionNotificationImpl: Sized {
    fn OnSessionCreated();
}
pub trait IAudioStateMonitorImpl: Sized {
    fn RegisterCallback();
    fn UnregisterCallback();
    fn GetSoundLevel();
}
pub trait IAudioStreamVolumeImpl: Sized {
    fn GetChannelCount();
    fn SetChannelVolume();
    fn GetChannelVolume();
    fn SetAllVolumes();
    fn GetAllVolumes();
}
pub trait IAudioSystemEffectsPropertyChangeNotificationClientImpl: Sized {
    fn OnPropertyChanged();
}
pub trait IAudioSystemEffectsPropertyStoreImpl: Sized {
    fn OpenDefaultPropertyStore();
    fn OpenUserPropertyStore();
    fn OpenVolatilePropertyStore();
    fn ResetUserPropertyStore();
    fn ResetVolatilePropertyStore();
    fn RegisterPropertyChangeNotification();
    fn UnregisterPropertyChangeNotification();
}
pub trait IAudioTrebleImpl: Sized + IPerChannelDbLevelImpl {}
pub trait IAudioVolumeDuckNotificationImpl: Sized {
    fn OnVolumeDuckNotification();
    fn OnVolumeUnduckNotification();
}
pub trait IAudioVolumeLevelImpl: Sized + IPerChannelDbLevelImpl {}
pub trait IChannelAudioVolumeImpl: Sized {
    fn GetChannelCount();
    fn SetChannelVolume();
    fn GetChannelVolume();
    fn SetAllVolumes();
    fn GetAllVolumes();
}
pub trait IConnectorImpl: Sized {
    fn GetType();
    fn GetDataFlow();
    fn ConnectTo();
    fn Disconnect();
    fn IsConnected();
    fn GetConnectedTo();
    fn GetConnectorIdConnectedTo();
    fn GetDeviceIdConnectedTo();
}
pub trait IControlChangeNotifyImpl: Sized {
    fn OnNotify();
}
pub trait IControlInterfaceImpl: Sized {
    fn GetName();
    fn GetIID();
}
pub trait IDeviceSpecificPropertyImpl: Sized {
    fn GetType();
    fn GetValue();
    fn SetValue();
    fn Get4BRange();
}
pub trait IDeviceTopologyImpl: Sized {
    fn GetConnectorCount();
    fn GetConnector();
    fn GetSubunitCount();
    fn GetSubunit();
    fn GetPartById();
    fn GetDeviceId();
    fn GetSignalPath();
}
pub trait IMMDeviceImpl: Sized {
    fn Activate();
    fn OpenPropertyStore();
    fn GetId();
    fn GetState();
}
pub trait IMMDeviceActivatorImpl: Sized {
    fn Activate();
}
pub trait IMMDeviceCollectionImpl: Sized {
    fn GetCount();
    fn Item();
}
pub trait IMMDeviceEnumeratorImpl: Sized {
    fn EnumAudioEndpoints();
    fn GetDefaultAudioEndpoint();
    fn GetDevice();
    fn RegisterEndpointNotificationCallback();
    fn UnregisterEndpointNotificationCallback();
}
pub trait IMMEndpointImpl: Sized {
    fn GetDataFlow();
}
pub trait IMMNotificationClientImpl: Sized {
    fn OnDeviceStateChanged();
    fn OnDeviceAdded();
    fn OnDeviceRemoved();
    fn OnDefaultDeviceChanged();
    fn OnPropertyValueChanged();
}
pub trait IMessageFilterImpl: Sized {
    fn HandleInComingCall();
    fn RetryRejectedCall();
    fn MessagePending();
}
pub trait IPartImpl: Sized {
    fn GetName();
    fn GetLocalId();
    fn GetGlobalId();
    fn GetPartType();
    fn GetSubType();
    fn GetControlInterfaceCount();
    fn GetControlInterface();
    fn EnumPartsIncoming();
    fn EnumPartsOutgoing();
    fn GetTopologyObject();
    fn Activate();
    fn RegisterControlChangeCallback();
    fn UnregisterControlChangeCallback();
}
pub trait IPartsListImpl: Sized {
    fn GetCount();
    fn GetPart();
}
pub trait IPerChannelDbLevelImpl: Sized {
    fn GetChannelCount();
    fn GetLevelRange();
    fn GetLevel();
    fn SetLevel();
    fn SetLevelUniform();
    fn SetLevelAllChannels();
}
pub trait ISimpleAudioVolumeImpl: Sized {
    fn SetMasterVolume();
    fn GetMasterVolume();
    fn SetMute();
    fn GetMute();
}
pub trait ISpatialAudioClientImpl: Sized {
    fn GetStaticObjectPosition();
    fn GetNativeStaticObjectTypeMask();
    fn GetMaxDynamicObjectCount();
    fn GetSupportedAudioObjectFormatEnumerator();
    fn GetMaxFrameCount();
    fn IsAudioObjectFormatSupported();
    fn IsSpatialAudioStreamAvailable();
    fn ActivateSpatialAudioStream();
}
pub trait ISpatialAudioClient2Impl: Sized + ISpatialAudioClientImpl {
    fn IsOffloadCapable();
    fn GetMaxFrameCountForCategory();
}
pub trait ISpatialAudioMetadataClientImpl: Sized {
    fn ActivateSpatialAudioMetadataItems();
    fn GetSpatialAudioMetadataItemsBufferLength();
    fn ActivateSpatialAudioMetadataWriter();
    fn ActivateSpatialAudioMetadataCopier();
    fn ActivateSpatialAudioMetadataReader();
}
pub trait ISpatialAudioMetadataCopierImpl: Sized {
    fn Open();
    fn CopyMetadataForFrames();
    fn Close();
}
pub trait ISpatialAudioMetadataItemsImpl: Sized {
    fn GetFrameCount();
    fn GetItemCount();
    fn GetMaxItemCount();
    fn GetMaxValueBufferLength();
    fn GetInfo();
}
pub trait ISpatialAudioMetadataItemsBufferImpl: Sized {
    fn AttachToBuffer();
    fn AttachToPopulatedBuffer();
    fn DetachBuffer();
}
pub trait ISpatialAudioMetadataReaderImpl: Sized {
    fn Open();
    fn ReadNextItem();
    fn ReadNextItemCommand();
    fn Close();
}
pub trait ISpatialAudioMetadataWriterImpl: Sized {
    fn Open();
    fn WriteNextItem();
    fn WriteNextItemCommand();
    fn Close();
}
pub trait ISpatialAudioObjectImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn SetPosition();
    fn SetVolume();
}
pub trait ISpatialAudioObjectBaseImpl: Sized {
    fn GetBuffer();
    fn SetEndOfStream();
    fn IsActive();
    fn GetAudioObjectType();
}
pub trait ISpatialAudioObjectForHrtfImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn SetPosition();
    fn SetGain();
    fn SetOrientation();
    fn SetEnvironment();
    fn SetDistanceDecay();
    fn SetDirectivity();
}
pub trait ISpatialAudioObjectForMetadataCommandsImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn WriteNextMetadataCommand();
}
pub trait ISpatialAudioObjectForMetadataItemsImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn GetSpatialAudioMetadataItems();
}
pub trait ISpatialAudioObjectRenderStreamImpl: Sized + ISpatialAudioObjectRenderStreamBaseImpl {
    fn ActivateSpatialAudioObject();
}
pub trait ISpatialAudioObjectRenderStreamBaseImpl: Sized {
    fn GetAvailableDynamicObjectCount();
    fn GetService();
    fn Start();
    fn Stop();
    fn Reset();
    fn BeginUpdatingAudioObjects();
    fn EndUpdatingAudioObjects();
}
pub trait ISpatialAudioObjectRenderStreamForHrtfImpl: Sized + ISpatialAudioObjectRenderStreamBaseImpl {
    fn ActivateSpatialAudioObjectForHrtf();
}
pub trait ISpatialAudioObjectRenderStreamForMetadataImpl: Sized + ISpatialAudioObjectRenderStreamBaseImpl {
    fn ActivateSpatialAudioObjectForMetadataCommands();
    fn ActivateSpatialAudioObjectForMetadataItems();
}
pub trait ISpatialAudioObjectRenderStreamNotifyImpl: Sized {
    fn OnAvailableDynamicObjectCountChange();
}
pub trait ISubunitImpl: Sized {}
