pub trait IAMAnalogVideoDecoderImpl: Sized {
    fn AvailableTVFormats();
    fn SetTVFormat();
    fn TVFormat();
    fn HorizontalLocked();
    fn SetVCRHorizontalLocking();
    fn VCRHorizontalLocking();
    fn NumberOfLines();
    fn SetOutputEnable();
    fn OutputEnable();
}
pub trait IAMAnalogVideoEncoderImpl: Sized {
    fn AvailableTVFormats();
    fn SetTVFormat();
    fn TVFormat();
    fn SetCopyProtection();
    fn CopyProtection();
    fn SetCCEnable();
    fn CCEnable();
}
pub trait IAMAsyncReaderTimestampScalingImpl: Sized {
    fn GetTimestampMode();
    fn SetTimestampMode();
}
pub trait IAMAudioInputMixerImpl: Sized {
    fn SetEnable();
    fn Enable();
    fn SetMono();
    fn Mono();
    fn SetMixLevel();
    fn MixLevel();
    fn SetPan();
    fn Pan();
    fn SetLoudness();
    fn Loudness();
    fn SetTreble();
    fn Treble();
    fn TrebleRange();
    fn SetBass();
    fn Bass();
    fn BassRange();
}
pub trait IAMAudioRendererStatsImpl: Sized {
    fn GetStatParam();
}
pub trait IAMBufferNegotiationImpl: Sized {
    fn SuggestAllocatorProperties();
    fn GetAllocatorProperties();
}
pub trait IAMCameraControlImpl: Sized {
    fn GetRange();
    fn Set();
    fn Get();
}
pub trait IAMCertifiedOutputProtectionImpl: Sized {
    fn KeyExchange();
    fn SessionSequenceStart();
    fn ProtectionCommand();
    fn ProtectionStatus();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMChannelInfoImpl: Sized + IDispatchImpl {
    fn ChannelName();
    fn ChannelDescription();
    fn ChannelURL();
    fn ContactAddress();
    fn ContactPhone();
    fn ContactEmail();
}
pub trait IAMClockAdjustImpl: Sized {
    fn SetClockDelta();
}
pub trait IAMClockSlaveImpl: Sized {
    fn SetErrorTolerance();
    fn GetErrorTolerance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
pub trait IAMCopyCaptureFileProgressImpl: Sized {
    fn Progress();
}
pub trait IAMCrossbarImpl: Sized {
    fn PinCounts();
    fn CanRoute();
    fn Route();
    fn IsRoutedTo();
    fn CrossbarPinInfo();
}
pub trait IAMDecoderCapsImpl: Sized {
    fn GetDecoderCaps();
}
pub trait IAMDevMemoryAllocatorImpl: Sized {
    fn GetInfo();
    fn CheckMemory();
    fn Alloc();
    fn Free();
    fn GetDevMemoryObject();
}
pub trait IAMDevMemoryControlImpl: Sized {
    fn QueryWriteSync();
    fn WriteSync();
    fn GetDevId();
}
pub trait IAMDeviceRemovalImpl: Sized {
    fn DeviceInfo();
    fn Reassociate();
    fn Disassociate();
}
pub trait IAMDirectSoundImpl: Sized {
    fn GetDirectSoundInterface();
    fn GetPrimaryBufferInterface();
    fn GetSecondaryBufferInterface();
    fn ReleaseDirectSoundInterface();
    fn ReleasePrimaryBufferInterface();
    fn ReleaseSecondaryBufferInterface();
    fn SetFocusWindow();
    fn GetFocusWindow();
}
pub trait IAMDroppedFramesImpl: Sized {
    fn GetNumDropped();
    fn GetNumNotDropped();
    fn GetDroppedInfo();
    fn GetAverageFrameSize();
}
pub trait IAMExtDeviceImpl: Sized {
    fn GetCapability();
    fn ExternalDeviceID();
    fn ExternalDeviceVersion();
    fn SetDevicePower();
    fn DevicePower();
    fn Calibrate();
    fn SetDevicePort();
    fn DevicePort();
}
pub trait IAMExtTransportImpl: Sized {
    fn GetCapability();
    fn SetMediaState();
    fn MediaState();
    fn SetLocalControl();
    fn LocalControl();
    fn GetStatus();
    fn GetTransportBasicParameters();
    fn SetTransportBasicParameters();
    fn GetTransportVideoParameters();
    fn SetTransportVideoParameters();
    fn GetTransportAudioParameters();
    fn SetTransportAudioParameters();
    fn SetMode();
    fn Mode();
    fn SetRate();
    fn Rate();
    fn GetChase();
    fn SetChase();
    fn GetBump();
    fn SetBump();
    fn AntiClogControl();
    fn SetAntiClogControl();
    fn GetEditPropertySet();
    fn SetEditPropertySet();
    fn GetEditProperty();
    fn SetEditProperty();
    fn EditStart();
    fn SetEditStart();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMExtendedErrorInfoImpl: Sized + IDispatchImpl {
    fn HasError();
    fn ErrorDescription();
    fn ErrorCode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMExtendedSeekingImpl: Sized + IDispatchImpl {
    fn ExSeekCapabilities();
    fn MarkerCount();
    fn CurrentMarker();
    fn GetMarkerTime();
    fn GetMarkerName();
    fn SetPlaybackSpeed();
    fn PlaybackSpeed();
}
pub trait IAMFilterGraphCallbackImpl: Sized {
    fn UnableToRender();
}
pub trait IAMFilterMiscFlagsImpl: Sized {
    fn GetMiscFlags();
}
pub trait IAMGraphBuilderCallbackImpl: Sized {
    fn SelectedFilter();
    fn CreatedFilter();
}
pub trait IAMGraphStreamsImpl: Sized {
    fn FindUpstreamInterface();
    fn SyncUsingStreamOffset();
    fn SetMaxGraphLatency();
}
pub trait IAMLatencyImpl: Sized {
    fn GetLatency();
}
pub trait IAMLine21DecoderImpl: Sized {
    fn GetDecoderLevel();
    fn GetCurrentService();
    fn SetCurrentService();
    fn GetServiceState();
    fn SetServiceState();
    fn GetOutputFormat();
    fn SetOutputFormat();
    fn GetBackgroundColor();
    fn SetBackgroundColor();
    fn GetRedrawAlways();
    fn SetRedrawAlways();
    fn GetDrawBackgroundMode();
    fn SetDrawBackgroundMode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMMediaContentImpl: Sized + IDispatchImpl {
    fn AuthorName();
    fn Title();
    fn Rating();
    fn Description();
    fn Copyright();
    fn BaseURL();
    fn LogoURL();
    fn LogoIconURL();
    fn WatermarkURL();
    fn MoreInfoURL();
    fn MoreInfoBannerImage();
    fn MoreInfoBannerURL();
    fn MoreInfoText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMMediaContent2Impl: Sized + IDispatchImpl {
    fn MediaParameter();
    fn MediaParameterName();
    fn PlaylistCount();
}
pub trait IAMMediaStreamImpl: Sized + IMediaStreamImpl {
    fn Initialize();
    fn SetState();
    fn JoinAMMultiMediaStream();
    fn JoinFilter();
    fn JoinFilterGraph();
}
pub trait IAMMediaTypeSampleImpl: Sized + IStreamSampleImpl {
    fn SetPointer();
    fn GetPointer();
    fn GetSize();
    fn GetTime();
    fn SetTime();
    fn IsSyncPoint();
    fn SetSyncPoint();
    fn IsPreroll();
    fn SetPreroll();
    fn GetActualDataLength();
    fn SetActualDataLength();
    fn GetMediaType();
    fn SetMediaType();
    fn IsDiscontinuity();
    fn SetDiscontinuity();
    fn GetMediaTime();
    fn SetMediaTime();
}
pub trait IAMMediaTypeStreamImpl: Sized + IMediaStreamImpl {
    fn GetFormat();
    fn SetFormat();
    fn CreateSample();
    fn GetStreamAllocatorRequirements();
    fn SetStreamAllocatorRequirements();
}
pub trait IAMMultiMediaStreamImpl: Sized + IMultiMediaStreamImpl {
    fn Initialize();
    fn GetFilterGraph();
    fn GetFilter();
    fn AddMediaStream();
    fn OpenFile();
    fn OpenMoniker();
    fn Render();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMNetShowConfigImpl: Sized + IDispatchImpl {
    fn BufferingTime();
    fn SetBufferingTime();
    fn UseFixedUDPPort();
    fn SetUseFixedUDPPort();
    fn FixedUDPPort();
    fn SetFixedUDPPort();
    fn UseHTTPProxy();
    fn SetUseHTTPProxy();
    fn EnableAutoProxy();
    fn SetEnableAutoProxy();
    fn HTTPProxyHost();
    fn SetHTTPProxyHost();
    fn HTTPProxyPort();
    fn SetHTTPProxyPort();
    fn EnableMulticast();
    fn SetEnableMulticast();
    fn EnableUDP();
    fn SetEnableUDP();
    fn EnableTCP();
    fn SetEnableTCP();
    fn EnableHTTP();
    fn SetEnableHTTP();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMNetShowExPropsImpl: Sized + IDispatchImpl {
    fn SourceProtocol();
    fn Bandwidth();
    fn ErrorCorrection();
    fn CodecCount();
    fn GetCodecInstalled();
    fn GetCodecDescription();
    fn GetCodecURL();
    fn CreationDate();
    fn SourceLink();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMNetShowPrerollImpl: Sized + IDispatchImpl {
    fn SetPreroll();
    fn Preroll();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMNetworkStatusImpl: Sized + IDispatchImpl {
    fn ReceivedPackets();
    fn RecoveredPackets();
    fn LostPackets();
    fn ReceptionQuality();
    fn BufferingCount();
    fn IsBroadcast();
    fn BufferingProgress();
}
pub trait IAMOpenProgressImpl: Sized {
    fn QueryProgress();
    fn AbortOperation();
}
pub trait IAMOverlayFXImpl: Sized {
    fn QueryOverlayFXCaps();
    fn SetOverlayFX();
    fn GetOverlayFX();
}
pub trait IAMParseImpl: Sized {
    fn GetParseTime();
    fn SetParseTime();
    fn Flush();
}
pub trait IAMPhysicalPinInfoImpl: Sized {
    fn GetPhysicalType();
}
pub trait IAMPlayListImpl: Sized {
    fn GetFlags();
    fn GetItemCount();
    fn GetItem();
    fn GetNamedEvent();
    fn GetRepeatInfo();
}
pub trait IAMPlayListItemImpl: Sized {
    fn GetFlags();
    fn GetSourceCount();
    fn GetSourceURL();
    fn GetSourceStart();
    fn GetSourceDuration();
    fn GetSourceStartMarker();
    fn GetSourceEndMarker();
    fn GetSourceStartMarkerName();
    fn GetSourceEndMarkerName();
    fn GetLinkURL();
    fn GetScanDuration();
}
pub trait IAMPluginControlImpl: Sized {
    fn GetPreferredClsid();
    fn GetPreferredClsidByIndex();
    fn SetPreferredClsid();
    fn IsDisabled();
    fn GetDisabledByIndex();
    fn SetDisabled();
    fn IsLegacyDisabled();
}
pub trait IAMPushSourceImpl: Sized + IAMLatencyImpl {
    fn GetPushSourceFlags();
    fn SetPushSourceFlags();
    fn SetStreamOffset();
    fn GetStreamOffset();
    fn GetMaxStreamOffset();
    fn SetMaxStreamOffset();
}
pub trait IAMRebuildImpl: Sized {
    fn RebuildNow();
}
pub trait IAMResourceControlImpl: Sized {
    fn Reserve();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAMStatsImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Count();
    fn GetValueByIndex();
    fn GetValueByName();
    fn GetIndex();
    fn AddValue();
}
pub trait IAMStreamConfigImpl: Sized {
    fn SetFormat();
    fn GetFormat();
    fn GetNumberOfCapabilities();
    fn GetStreamCaps();
}
pub trait IAMStreamControlImpl: Sized {
    fn StartAt();
    fn StopAt();
    fn GetInfo();
}
pub trait IAMStreamSelectImpl: Sized {
    fn Count();
    fn Info();
    fn Enable();
}
pub trait IAMTVAudioImpl: Sized {
    fn GetHardwareSupportedTVAudioModes();
    fn GetAvailableTVAudioModes();
    fn TVAudioMode();
    fn SetTVAudioMode();
    fn RegisterNotificationCallBack();
    fn UnRegisterNotificationCallBack();
}
pub trait IAMTVAudioNotificationImpl: Sized {
    fn OnEvent();
}
pub trait IAMTVTunerImpl: Sized + IAMTunerImpl {
    fn AvailableTVFormats();
    fn TVFormat();
    fn AutoTune();
    fn StoreAutoTune();
    fn NumInputConnections();
    fn SetInputType();
    fn InputType();
    fn SetConnectInput();
    fn ConnectInput();
    fn VideoFrequency();
    fn AudioFrequency();
}
pub trait IAMTimecodeDisplayImpl: Sized {
    fn GetTCDisplayEnable();
    fn SetTCDisplayEnable();
    fn GetTCDisplay();
    fn SetTCDisplay();
}
pub trait IAMTimecodeGeneratorImpl: Sized {
    fn GetTCGMode();
    fn SetTCGMode();
    fn SetVITCLine();
    fn VITCLine();
    fn SetTimecode();
    fn GetTimecode();
}
pub trait IAMTimecodeReaderImpl: Sized {
    fn GetTCRMode();
    fn SetTCRMode();
    fn SetVITCLine();
    fn VITCLine();
    fn GetTimecode();
}
pub trait IAMTunerImpl: Sized {
    fn SetChannel();
    fn Channel();
    fn ChannelMinMax();
    fn SetCountryCode();
    fn CountryCode();
    fn SetTuningSpace();
    fn TuningSpace();
    fn Logon();
    fn Logout();
    fn SignalPresent();
    fn SetMode();
    fn Mode();
    fn GetAvailableModes();
    fn RegisterNotificationCallBack();
    fn UnRegisterNotificationCallBack();
}
pub trait IAMTunerNotificationImpl: Sized {
    fn OnEvent();
}
pub trait IAMVfwCaptureDialogsImpl: Sized {
    fn HasDialog();
    fn ShowDialog();
    fn SendDriverMessage();
}
pub trait IAMVfwCompressDialogsImpl: Sized {
    fn ShowDialog();
    fn GetState();
    fn SetState();
    fn SendDriverMessage();
}
pub trait IAMVideoAcceleratorImpl: Sized {
    fn GetVideoAcceleratorGUIDs();
    fn GetUncompFormatsSupported();
    fn GetInternalMemInfo();
    fn GetCompBufferInfo();
    fn GetInternalCompBufferInfo();
    fn BeginFrame();
    fn EndFrame();
    fn GetBuffer();
    fn ReleaseBuffer();
    fn Execute();
    fn QueryRenderStatus();
    fn DisplayFrame();
}
pub trait IAMVideoAcceleratorNotifyImpl: Sized {
    fn GetUncompSurfacesInfo();
    fn SetUncompSurfacesInfo();
    fn GetCreateVideoAcceleratorData();
}
pub trait IAMVideoCompressionImpl: Sized {
    fn SetKeyFrameRate();
    fn KeyFrameRate();
    fn SetPFramesPerKeyFrame();
    fn PFramesPerKeyFrame();
    fn SetQuality();
    fn Quality();
    fn SetWindowSize();
    fn WindowSize();
    fn GetInfo();
    fn OverrideKeyFrame();
    fn OverrideFrameSize();
}
pub trait IAMVideoControlImpl: Sized {
    fn GetCaps();
    fn SetMode();
    fn GetMode();
    fn GetCurrentActualFrameRate();
    fn GetMaxAvailableFrameRate();
    fn GetFrameRateList();
}
pub trait IAMVideoDecimationPropertiesImpl: Sized {
    fn QueryDecimationUsage();
    fn SetDecimationUsage();
}
pub trait IAMVideoProcAmpImpl: Sized {
    fn GetRange();
    fn Set();
    fn Get();
}
pub trait IAMWstDecoderImpl: Sized {
    fn GetDecoderLevel();
    fn GetCurrentService();
    fn GetServiceState();
    fn SetServiceState();
    fn GetOutputFormat();
    fn SetOutputFormat();
    fn GetBackgroundColor();
    fn SetBackgroundColor();
    fn GetRedrawAlways();
    fn SetRedrawAlways();
    fn GetDrawBackgroundMode();
    fn SetDrawBackgroundMode();
    fn SetAnswerMode();
    fn GetAnswerMode();
    fn SetHoldPage();
    fn GetHoldPage();
    fn GetCurrentPage();
    fn SetCurrentPage();
}
pub trait IAMovieSetupImpl: Sized {
    fn Register();
    fn Unregister();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IATSCChannelTuneRequestImpl: Sized + IChannelTuneRequestImpl + ITuneRequestImpl + IDispatchImpl {
    fn MinorChannel();
    fn SetMinorChannel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IATSCComponentTypeImpl: Sized + IMPEG2ComponentTypeImpl + ILanguageComponentTypeImpl + IComponentTypeImpl + IDispatchImpl {
    fn Flags();
    fn SetFlags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IATSCLocatorImpl: Sized + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {
    fn PhysicalChannel();
    fn SetPhysicalChannel();
    fn TSID();
    fn SetTSID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IATSCLocator2Impl: Sized + IATSCLocatorImpl + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {
    fn ProgramNumber();
    fn SetProgramNumber();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IATSCTuningSpaceImpl: Sized + IAnalogTVTuningSpaceImpl + ITuningSpaceImpl + IDispatchImpl {
    fn MinMinorChannel();
    fn SetMinMinorChannel();
    fn MaxMinorChannel();
    fn SetMaxMinorChannel();
    fn MinPhysicalChannel();
    fn SetMinPhysicalChannel();
    fn MaxPhysicalChannel();
    fn SetMaxPhysicalChannel();
}
pub trait IATSC_EITImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetSourceId();
    fn GetProtocolVersion();
    fn GetCountOfRecords();
    fn GetRecordEventId();
    fn GetRecordStartTime();
    fn GetRecordEtmLocation();
    fn GetRecordDuration();
    fn GetRecordTitleText();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
}
pub trait IATSC_ETTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetProtocolVersion();
    fn GetEtmId();
    fn GetExtendedMessageText();
}
pub trait IATSC_MGTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetProtocolVersion();
    fn GetCountOfRecords();
    fn GetRecordType();
    fn GetRecordTypePid();
    fn GetRecordVersionNumber();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
}
pub trait IATSC_STTImpl: Sized {
    fn Initialize();
    fn GetProtocolVersion();
    fn GetSystemTime();
    fn GetGpsUtcOffset();
    fn GetDaylightSavings();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
}
pub trait IATSC_VCTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetTransportStreamId();
    fn GetProtocolVersion();
    fn GetCountOfRecords();
    fn GetRecordName();
    fn GetRecordMajorChannelNumber();
    fn GetRecordMinorChannelNumber();
    fn GetRecordModulationMode();
    fn GetRecordCarrierFrequency();
    fn GetRecordTransportStreamId();
    fn GetRecordProgramNumber();
    fn GetRecordEtmLocation();
    fn GetRecordIsAccessControlledBitSet();
    fn GetRecordIsHiddenBitSet();
    fn GetRecordIsPathSelectBitSet();
    fn GetRecordIsOutOfBandBitSet();
    fn GetRecordIsHideGuideBitSet();
    fn GetRecordServiceType();
    fn GetRecordSourceId();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAnalogAudioComponentTypeImpl: Sized + IComponentTypeImpl + IDispatchImpl {
    fn AnalogAudioMode();
    fn SetAnalogAudioMode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAnalogLocatorImpl: Sized + ILocatorImpl + IDispatchImpl {
    fn VideoStandard();
    fn SetVideoStandard();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAnalogRadioTuningSpaceImpl: Sized + ITuningSpaceImpl + IDispatchImpl {
    fn MinFrequency();
    fn SetMinFrequency();
    fn MaxFrequency();
    fn SetMaxFrequency();
    fn Step();
    fn SetStep();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAnalogRadioTuningSpace2Impl: Sized + IAnalogRadioTuningSpaceImpl + ITuningSpaceImpl + IDispatchImpl {
    fn CountryCode();
    fn SetCountryCode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAnalogTVTuningSpaceImpl: Sized + ITuningSpaceImpl + IDispatchImpl {
    fn MinChannel();
    fn SetMinChannel();
    fn MaxChannel();
    fn SetMaxChannel();
    fn InputType();
    fn SetInputType();
    fn CountryCode();
    fn SetCountryCode();
}
pub trait IAsyncReaderImpl: Sized {
    fn RequestAllocator();
    fn Request();
    fn WaitForNext();
    fn SyncReadAligned();
    fn SyncRead();
    fn Length();
    fn BeginFlush();
    fn EndFlush();
}
pub trait IAtscContentAdvisoryDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetRatingRegionCount();
    fn GetRecordRatingRegion();
    fn GetRecordRatedDimensions();
    fn GetRecordRatingDimension();
    fn GetRecordRatingValue();
    fn GetRecordRatingDescriptionText();
}
pub trait IAtscPsipParserImpl: Sized {
    fn Initialize();
    fn GetPAT();
    fn GetCAT();
    fn GetPMT();
    fn GetTSDT();
    fn GetMGT();
    fn GetVCT();
    fn GetEIT();
    fn GetETT();
    fn GetSTT();
    fn GetEAS();
}
pub trait IAttributeGetImpl: Sized {
    fn GetCount();
    fn GetAttribIndexed();
    fn GetAttrib();
}
pub trait IAttributeSetImpl: Sized {
    fn SetAttrib();
}
pub trait IAudioDataImpl: Sized + IMemoryDataImpl {
    fn GetFormat();
    fn SetFormat();
}
pub trait IAudioMediaStreamImpl: Sized + IMediaStreamImpl {
    fn GetFormat();
    fn SetFormat();
    fn CreateSample();
}
pub trait IAudioStreamSampleImpl: Sized + IStreamSampleImpl {
    fn GetAudioData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAuxInTuningSpaceImpl: Sized + ITuningSpaceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IAuxInTuningSpace2Impl: Sized + IAuxInTuningSpaceImpl + ITuningSpaceImpl + IDispatchImpl {
    fn CountryCode();
    fn SetCountryCode();
}
pub trait IBDAComparableImpl: Sized {
    fn CompareExact();
    fn CompareEquivalent();
    fn HashExact();
    fn HashExactIncremental();
    fn HashEquivalent();
    fn HashEquivalentIncremental();
}
pub trait IBDACreateTuneRequestExImpl: Sized {
    fn CreateTuneRequestEx();
}
pub trait IBDA_AUXImpl: Sized {
    fn QueryCapabilities();
    fn EnumCapability();
}
pub trait IBDA_AutoDemodulateImpl: Sized {
    fn SetAutoDemodulate();
}
pub trait IBDA_AutoDemodulateExImpl: Sized + IBDA_AutoDemodulateImpl {
    fn SupportedDeviceNodeTypes();
    fn SupportedVideoFormats();
    fn AuxInputCount();
}
pub trait IBDA_ConditionalAccessImpl: Sized {
    fn SmartCardStatus();
    fn SmartCardInfo();
    fn SmartCardApplications();
    fn Entitlement();
    fn TuneByChannel();
    fn SetProgram();
    fn AddProgram();
    fn RemoveProgram();
    fn GetModuleUI();
    fn InformUIClosed();
}
pub trait IBDA_ConditionalAccessExImpl: Sized {
    fn CheckEntitlementToken();
    fn SetCaptureToken();
    fn OpenBroadcastMmi();
    fn CloseMmiDialog();
    fn CreateDialogRequestNumber();
}
pub trait IBDA_DRIDRMServiceImpl: Sized {
    fn SetDRM();
    fn GetDRMStatus();
    fn GetPairingStatus();
}
pub trait IBDA_DRIWMDRMSessionImpl: Sized {
    fn AcknowledgeLicense();
    fn ProcessLicenseChallenge();
    fn ProcessRegistrationChallenge();
    fn SetRevInfo();
    fn SetCrl();
    fn GetHMSAssociationData();
    fn GetLastCardeaError();
}
pub trait IBDA_DRMImpl: Sized {
    fn GetDRMPairingStatus();
    fn PerformDRMPairing();
}
pub trait IBDA_DRMServiceImpl: Sized {
    fn SetDRM();
    fn GetDRMStatus();
}
pub trait IBDA_DeviceControlImpl: Sized {
    fn StartChanges();
    fn CheckChanges();
    fn CommitChanges();
    fn GetChangeState();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IBDA_DiagnosticPropertiesImpl: Sized + IPropertyBagImpl {}
pub trait IBDA_DigitalDemodulatorImpl: Sized {
    fn SetModulationType();
    fn ModulationType();
    fn SetInnerFECMethod();
    fn InnerFECMethod();
    fn SetInnerFECRate();
    fn InnerFECRate();
    fn SetOuterFECMethod();
    fn OuterFECMethod();
    fn SetOuterFECRate();
    fn OuterFECRate();
    fn SetSymbolRate();
    fn SymbolRate();
    fn SetSpectralInversion();
    fn SpectralInversion();
}
pub trait IBDA_DigitalDemodulator2Impl: Sized + IBDA_DigitalDemodulatorImpl {
    fn SetGuardInterval();
    fn GuardInterval();
    fn SetTransmissionMode();
    fn TransmissionMode();
    fn SetRollOff();
    fn RollOff();
    fn SetPilot();
    fn Pilot();
}
pub trait IBDA_DigitalDemodulator3Impl: Sized + IBDA_DigitalDemodulator2Impl + IBDA_DigitalDemodulatorImpl {
    fn SetSignalTimeouts();
    fn SignalTimeouts();
    fn SetPLPNumber();
    fn PLPNumber();
}
pub trait IBDA_DiseqCommandImpl: Sized {
    fn SetEnableDiseqCommands();
    fn SetDiseqLNBSource();
    fn SetDiseqUseToneBurst();
    fn SetDiseqRepeats();
    fn SetDiseqSendCommand();
    fn DiseqResponse();
}
pub trait IBDA_EasMessageImpl: Sized {
    fn EasMessage();
}
pub trait IBDA_EncoderImpl: Sized {
    fn QueryCapabilities();
    fn EnumAudioCapability();
    fn EnumVideoCapability();
    fn SetParameters();
    fn GetState();
}
pub trait IBDA_EthernetFilterImpl: Sized {
    fn GetMulticastListSize();
    fn PutMulticastList();
    fn GetMulticastList();
    fn PutMulticastMode();
    fn GetMulticastMode();
}
pub trait IBDA_EventingServiceImpl: Sized {
    fn CompleteEvent();
}
pub trait IBDA_FDCImpl: Sized {
    fn GetStatus();
    fn RequestTables();
    fn AddPid();
    fn RemovePid();
    fn AddTid();
    fn RemoveTid();
    fn GetTableSection();
}
pub trait IBDA_FrequencyFilterImpl: Sized {
    fn SetAutotune();
    fn Autotune();
    fn SetFrequency();
    fn Frequency();
    fn SetPolarity();
    fn Polarity();
    fn SetRange();
    fn Range();
    fn SetBandwidth();
    fn Bandwidth();
    fn SetFrequencyMultiplier();
    fn FrequencyMultiplier();
}
pub trait IBDA_GuideDataDeliveryServiceImpl: Sized {
    fn GetGuideDataType();
    fn GetGuideData();
    fn RequestGuideDataUpdate();
    fn GetTuneXmlFromServiceIdx();
    fn GetServices();
    fn GetServiceInfoFromTuneXml();
}
pub trait IBDA_IPSinkControlImpl: Sized {
    fn GetMulticastList();
    fn GetAdapterIPAddress();
}
pub trait IBDA_IPSinkInfoImpl: Sized {
    fn MulticastList();
    fn AdapterIPAddress();
    fn AdapterDescription();
}
pub trait IBDA_IPV4FilterImpl: Sized {
    fn GetMulticastListSize();
    fn PutMulticastList();
    fn GetMulticastList();
    fn PutMulticastMode();
    fn GetMulticastMode();
}
pub trait IBDA_IPV6FilterImpl: Sized {
    fn GetMulticastListSize();
    fn PutMulticastList();
    fn GetMulticastList();
    fn PutMulticastMode();
    fn GetMulticastMode();
}
pub trait IBDA_ISDBConditionalAccessImpl: Sized {
    fn SetIsdbCasRequest();
}
pub trait IBDA_LNBInfoImpl: Sized {
    fn SetLocalOscilatorFrequencyLowBand();
    fn LocalOscilatorFrequencyLowBand();
    fn SetLocalOscilatorFrequencyHighBand();
    fn LocalOscilatorFrequencyHighBand();
    fn SetHighLowSwitchFrequency();
    fn HighLowSwitchFrequency();
}
pub trait IBDA_MUXImpl: Sized {
    fn SetPidList();
    fn GetPidList();
}
pub trait IBDA_NameValueServiceImpl: Sized {
    fn GetValueNameByIndex();
    fn GetValue();
    fn SetValue();
}
pub trait IBDA_NetworkProviderImpl: Sized {
    fn PutSignalSource();
    fn GetSignalSource();
    fn GetNetworkType();
    fn PutTuningSpace();
    fn GetTuningSpace();
    fn RegisterDeviceFilter();
    fn UnRegisterDeviceFilter();
}
pub trait IBDA_NullTransformImpl: Sized {
    fn Start();
    fn Stop();
}
pub trait IBDA_PinControlImpl: Sized {
    fn GetPinID();
    fn GetPinType();
    fn RegistrationContext();
}
pub trait IBDA_SignalPropertiesImpl: Sized {
    fn PutNetworkType();
    fn GetNetworkType();
    fn PutSignalSource();
    fn GetSignalSource();
    fn PutTuningSpace();
    fn GetTuningSpace();
}
pub trait IBDA_SignalStatisticsImpl: Sized {
    fn SetSignalStrength();
    fn SignalStrength();
    fn SetSignalQuality();
    fn SignalQuality();
    fn SetSignalPresent();
    fn SignalPresent();
    fn SetSignalLocked();
    fn SignalLocked();
    fn SetSampleTime();
    fn SampleTime();
}
pub trait IBDA_TIF_REGISTRATIONImpl: Sized {
    fn RegisterTIFEx();
    fn UnregisterTIF();
}
pub trait IBDA_TopologyImpl: Sized {
    fn GetNodeTypes();
    fn GetNodeDescriptors();
    fn GetNodeInterfaces();
    fn GetPinTypes();
    fn GetTemplateConnections();
    fn CreatePin();
    fn DeletePin();
    fn SetMediaType();
    fn SetMedium();
    fn CreateTopology();
    fn GetControlNode();
}
pub trait IBDA_TransportStreamInfoImpl: Sized {
    fn PatTableTickCount();
}
pub trait IBDA_TransportStreamSelectorImpl: Sized {
    fn SetTSID();
    fn GetTSInformation();
}
pub trait IBDA_UserActivityServiceImpl: Sized {
    fn SetCurrentTunerUseReason();
    fn GetUserActivityInterval();
    fn UserActivityDetected();
}
pub trait IBDA_VoidTransformImpl: Sized {
    fn Start();
    fn Stop();
}
pub trait IBDA_WMDRMSessionImpl: Sized {
    fn GetStatus();
    fn SetRevInfo();
    fn SetCrl();
    fn TransactMessage();
    fn GetLicense();
    fn ReissueLicense();
    fn RenewLicense();
    fn GetKeyInfo();
}
pub trait IBDA_WMDRMTunerImpl: Sized {
    fn PurchaseEntitlement();
    fn CancelCaptureToken();
    fn SetPidProtection();
    fn GetPidProtection();
    fn SetSyncValue();
    fn GetStartCodeProfile();
}
pub trait IBPCSatelliteTunerImpl: Sized + IAMTunerImpl {
    fn DefaultSubChannelTypes();
    fn SetDefaultSubChannelTypes();
    fn IsTapingPermitted();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBaseFilterImpl: Sized + IMediaFilterImpl + IPersistImpl {
    fn EnumPins();
    fn FindPin();
    fn QueryFilterInfo();
    fn JoinFilterGraph();
    fn QueryVendorInfo();
}
pub trait IBaseVideoMixerImpl: Sized {
    fn SetLeadPin();
    fn GetLeadPin();
    fn GetInputPinCount();
    fn IsUsingClock();
    fn SetUsingClock();
    fn GetClockPeriod();
    fn SetClockPeriod();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBasicAudioImpl: Sized + IDispatchImpl {
    fn SetVolume();
    fn Volume();
    fn SetBalance();
    fn Balance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBasicVideoImpl: Sized + IDispatchImpl {
    fn AvgTimePerFrame();
    fn BitRate();
    fn BitErrorRate();
    fn VideoWidth();
    fn VideoHeight();
    fn SetSourceLeft();
    fn SourceLeft();
    fn SetSourceWidth();
    fn SourceWidth();
    fn SetSourceTop();
    fn SourceTop();
    fn SetSourceHeight();
    fn SourceHeight();
    fn SetDestinationLeft();
    fn DestinationLeft();
    fn SetDestinationWidth();
    fn DestinationWidth();
    fn SetDestinationTop();
    fn DestinationTop();
    fn SetDestinationHeight();
    fn DestinationHeight();
    fn SetSourcePosition();
    fn GetSourcePosition();
    fn SetDefaultSourcePosition();
    fn SetDestinationPosition();
    fn GetDestinationPosition();
    fn SetDefaultDestinationPosition();
    fn GetVideoSize();
    fn GetVideoPaletteEntries();
    fn GetCurrentImage();
    fn IsUsingDefaultSource();
    fn IsUsingDefaultDestination();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBasicVideo2Impl: Sized + IBasicVideoImpl + IDispatchImpl {
    fn GetPreferredAspectRatio();
}
pub trait IBroadcastEventImpl: Sized {
    fn Fire();
}
pub trait IBroadcastEventExImpl: Sized + IBroadcastEventImpl {
    fn FireEx();
}
pub trait IBufferingTimeImpl: Sized {
    fn GetBufferingTime();
    fn SetBufferingTime();
}
pub trait ICATImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
}
pub trait ICCSubStreamFilteringImpl: Sized {
    fn SubstreamTypes();
    fn SetSubstreamTypes();
}
pub trait ICameraControlImpl: Sized {
    fn Exposure();
    fn SetExposure();
    fn getRange_Exposure();
    fn Focus();
    fn SetFocus();
    fn getRange_Focus();
    fn Iris();
    fn SetIris();
    fn getRange_Iris();
    fn Zoom();
    fn SetZoom();
    fn getRange_Zoom();
    fn FocalLengths();
    fn Pan();
    fn SetPan();
    fn getRange_Pan();
    fn Tilt();
    fn SetTilt();
    fn getRange_Tilt();
    fn PanTilt();
    fn SetPanTilt();
    fn Roll();
    fn SetRoll();
    fn getRange_Roll();
    fn ExposureRelative();
    fn SetExposureRelative();
    fn getRange_ExposureRelative();
    fn FocusRelative();
    fn SetFocusRelative();
    fn getRange_FocusRelative();
    fn IrisRelative();
    fn SetIrisRelative();
    fn getRange_IrisRelative();
    fn ZoomRelative();
    fn SetZoomRelative();
    fn getRange_ZoomRelative();
    fn PanRelative();
    fn SetPanRelative();
    fn TiltRelative();
    fn SetTiltRelative();
    fn getRange_TiltRelative();
    fn PanTiltRelative();
    fn SetPanTiltRelative();
    fn getRange_PanRelative();
    fn RollRelative();
    fn SetRollRelative();
    fn getRange_RollRelative();
    fn ScanMode();
    fn SetScanMode();
    fn PrivacyMode();
    fn SetPrivacyMode();
}
pub trait ICaptionServiceDescriptorImpl: Sized {
    fn GetNumberOfServices();
    fn GetLanguageCode();
    fn GetCaptionServiceNumber();
    fn GetCCType();
    fn GetEasyReader();
    fn GetWideAspectRatio();
}
pub trait ICaptureGraphBuilderImpl: Sized {
    fn SetFiltergraph();
    fn GetFiltergraph();
    fn SetOutputFileName();
    fn FindInterface();
    fn RenderStream();
    fn ControlStream();
    fn AllocCapFile();
    fn CopyCaptureFile();
}
pub trait ICaptureGraphBuilder2Impl: Sized {
    fn SetFiltergraph();
    fn GetFiltergraph();
    fn SetOutputFileName();
    fn FindInterface();
    fn RenderStream();
    fn ControlStream();
    fn AllocCapFile();
    fn CopyCaptureFile();
    fn FindPin();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IChannelIDTuneRequestImpl: Sized + ITuneRequestImpl + IDispatchImpl {
    fn ChannelID();
    fn SetChannelID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IChannelTuneRequestImpl: Sized + ITuneRequestImpl + IDispatchImpl {
    fn Channel();
    fn SetChannel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentImpl: Sized + IDispatchImpl {
    fn Type();
    fn SetType();
    fn DescLangID();
    fn SetDescLangID();
    fn Status();
    fn SetStatus();
    fn Description();
    fn SetDescription();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentTypeImpl: Sized + IDispatchImpl {
    fn Category();
    fn SetCategory();
    fn MediaMajorType();
    fn SetMediaMajorType();
    fn _MediaMajorType();
    fn Set_MediaMajorType();
    fn MediaSubType();
    fn SetMediaSubType();
    fn _MediaSubType();
    fn Set_MediaSubType();
    fn MediaFormatType();
    fn SetMediaFormatType();
    fn _MediaFormatType();
    fn Set_MediaFormatType();
    fn MediaType();
    fn SetMediaType();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentTypesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn EnumComponentTypes();
    fn Item();
    fn SetItem();
    fn Add();
    fn Remove();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn EnumComponents();
    fn Item();
    fn Add();
    fn Remove();
    fn Clone();
    fn SetItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentsOldImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn EnumComponents();
    fn Item();
    fn Add();
    fn Remove();
    fn Clone();
}
pub trait IConfigAsfWriterImpl: Sized {
    fn ConfigureFilterUsingProfileId();
    fn GetCurrentProfileId();
    fn ConfigureFilterUsingProfileGuid();
    fn GetCurrentProfileGuid();
    fn ConfigureFilterUsingProfile();
    fn GetCurrentProfile();
    fn SetIndexMode();
    fn GetIndexMode();
}
pub trait IConfigAsfWriter2Impl: Sized + IConfigAsfWriterImpl {
    fn StreamNumFromPin();
    fn SetParam();
    fn GetParam();
    fn ResetMultiPassState();
}
pub trait IConfigAviMuxImpl: Sized {
    fn SetMasterStream();
    fn GetMasterStream();
    fn SetOutputCompatibilityIndex();
    fn GetOutputCompatibilityIndex();
}
pub trait IConfigInterleavingImpl: Sized {
    fn SetMode();
    fn Mode();
    fn SetInterleaving();
    fn Interleaving();
}
pub trait ICreateDevEnumImpl: Sized {
    fn CreateClassEnumerator();
}
pub trait ICreatePropBagOnRegKeyImpl: Sized {
    fn Create();
}
pub trait IDDrawExclModeVideoImpl: Sized {
    fn SetDDrawObject();
    fn GetDDrawObject();
    fn SetDDrawSurface();
    fn GetDDrawSurface();
    fn SetDrawParameters();
    fn GetNativeVideoProps();
    fn SetCallbackInterface();
}
pub trait IDDrawExclModeVideoCallbackImpl: Sized {
    fn OnUpdateOverlay();
    fn OnUpdateColorKey();
    fn OnUpdateSize();
}
pub trait IDMOWrapperFilterImpl: Sized {
    fn Init();
}
pub trait IDShowPluginImpl: Sized {
    fn URL();
    fn UserAgent();
}
pub trait IDTFilterImpl: Sized {
    fn EvalRatObjOK();
    fn GetCurrRating();
    fn BlockedRatingAttributes();
    fn SetBlockedRatingAttributes();
    fn BlockUnRated();
    fn SetBlockUnRated();
    fn BlockUnRatedDelay();
    fn SetBlockUnRatedDelay();
}
pub trait IDTFilter2Impl: Sized + IDTFilterImpl {
    fn ChallengeUrl();
    fn GetCurrLicenseExpDate();
    fn GetLastErrorCode();
}
pub trait IDTFilter3Impl: Sized + IDTFilter2Impl + IDTFilterImpl {
    fn GetProtectionType();
    fn LicenseHasExpirationDate();
    fn SetRights();
}
pub trait IDTFilterConfigImpl: Sized {
    fn GetSecureChannelObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDTFilterEventsImpl: Sized + IDispatchImpl {}
pub trait IDTFilterLicenseRenewalImpl: Sized {
    fn GetLicenseRenewalData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBCLocatorImpl: Sized + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBSLocatorImpl: Sized + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {
    fn SignalPolarisation();
    fn SetSignalPolarisation();
    fn WestPosition();
    fn SetWestPosition();
    fn OrbitalPosition();
    fn SetOrbitalPosition();
    fn Azimuth();
    fn SetAzimuth();
    fn Elevation();
    fn SetElevation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBSLocator2Impl: Sized + IDVBSLocatorImpl + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {
    fn DiseqLNBSource();
    fn SetDiseqLNBSource();
    fn LocalOscillatorOverrideLow();
    fn SetLocalOscillatorOverrideLow();
    fn LocalOscillatorOverrideHigh();
    fn SetLocalOscillatorOverrideHigh();
    fn LocalLNBSwitchOverride();
    fn SetLocalLNBSwitchOverride();
    fn LocalSpectralInversionOverride();
    fn SetLocalSpectralInversionOverride();
    fn SignalRollOff();
    fn SetSignalRollOff();
    fn SignalPilot();
    fn SetSignalPilot();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBSTuningSpaceImpl: Sized + IDVBTuningSpace2Impl + IDVBTuningSpaceImpl + ITuningSpaceImpl + IDispatchImpl {
    fn LowOscillator();
    fn SetLowOscillator();
    fn HighOscillator();
    fn SetHighOscillator();
    fn LNBSwitch();
    fn SetLNBSwitch();
    fn InputRange();
    fn SetInputRange();
    fn SpectralInversion();
    fn SetSpectralInversion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBTLocatorImpl: Sized + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {
    fn Bandwidth();
    fn SetBandwidth();
    fn LPInnerFEC();
    fn SetLPInnerFEC();
    fn LPInnerFECRate();
    fn SetLPInnerFECRate();
    fn HAlpha();
    fn SetHAlpha();
    fn Guard();
    fn SetGuard();
    fn Mode();
    fn SetMode();
    fn OtherFrequencyInUse();
    fn SetOtherFrequencyInUse();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBTLocator2Impl: Sized + IDVBTLocatorImpl + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {
    fn PhysicalLayerPipeId();
    fn SetPhysicalLayerPipeId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBTuneRequestImpl: Sized + ITuneRequestImpl + IDispatchImpl {
    fn ONID();
    fn SetONID();
    fn TSID();
    fn SetTSID();
    fn SID();
    fn SetSID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBTuningSpaceImpl: Sized + ITuningSpaceImpl + IDispatchImpl {
    fn SystemType();
    fn SetSystemType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDVBTuningSpace2Impl: Sized + IDVBTuningSpaceImpl + ITuningSpaceImpl + IDispatchImpl {
    fn NetworkID();
    fn SetNetworkID();
}
pub trait IDVB_BATImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetBouquetId();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn GetCountOfRecords();
    fn GetRecordTransportStreamId();
    fn GetRecordOriginalNetworkId();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
}
pub trait IDVB_DITImpl: Sized {
    fn Initialize();
    fn GetTransitionFlag();
}
pub trait IDVB_EITImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetServiceId();
    fn GetTransportStreamId();
    fn GetOriginalNetworkId();
    fn GetSegmentLastSectionNumber();
    fn GetLastTableId();
    fn GetCountOfRecords();
    fn GetRecordEventId();
    fn GetRecordStartTime();
    fn GetRecordDuration();
    fn GetRecordRunningStatus();
    fn GetRecordFreeCAMode();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
    fn GetVersionHash();
}
pub trait IDVB_EIT2Impl: Sized + IDVB_EITImpl {
    fn GetSegmentInfo();
    fn GetRecordSection();
}
pub trait IDVB_NITImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetNetworkId();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn GetCountOfRecords();
    fn GetRecordTransportStreamId();
    fn GetRecordOriginalNetworkId();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
    fn GetVersionHash();
}
pub trait IDVB_RSTImpl: Sized {
    fn Initialize();
    fn GetCountOfRecords();
    fn GetRecordTransportStreamId();
    fn GetRecordOriginalNetworkId();
    fn GetRecordServiceId();
    fn GetRecordEventId();
    fn GetRecordRunningStatus();
}
pub trait IDVB_SDTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetTransportStreamId();
    fn GetOriginalNetworkId();
    fn GetCountOfRecords();
    fn GetRecordServiceId();
    fn GetRecordEITScheduleFlag();
    fn GetRecordEITPresentFollowingFlag();
    fn GetRecordRunningStatus();
    fn GetRecordFreeCAMode();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
    fn GetVersionHash();
}
pub trait IDVB_SITImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn GetCountOfRecords();
    fn GetRecordServiceId();
    fn GetRecordRunningStatus();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
}
pub trait IDVB_STImpl: Sized {
    fn Initialize();
    fn GetDataLength();
    fn GetData();
}
pub trait IDVB_TDTImpl: Sized {
    fn Initialize();
    fn GetUTCTime();
}
pub trait IDVB_TOTImpl: Sized {
    fn Initialize();
    fn GetUTCTime();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
}
pub trait IDVEncImpl: Sized {
    fn IFormatResolution();
    fn SetIFormatResolution();
}
pub trait IDVRGB219Impl: Sized {
    fn SetRGB219();
}
pub trait IDVSplitterImpl: Sized {
    fn DiscardAlternateVideoFrames();
}
pub trait IDecimateVideoImageImpl: Sized {
    fn SetDecimationImageSize();
    fn ResetDecimationImageSize();
}
pub trait IDeferredCommandImpl: Sized {
    fn Cancel();
    fn Confidence();
    fn Postpone();
    fn GetHResult();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDigitalCableLocatorImpl: Sized + IATSCLocator2Impl + IATSCLocatorImpl + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IDigitalCableTuneRequestImpl: Sized + IATSCChannelTuneRequestImpl + IChannelTuneRequestImpl + ITuneRequestImpl + IDispatchImpl {
    fn MajorChannel();
    fn SetMajorChannel();
    fn SourceID();
    fn SetSourceID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDigitalCableTuningSpaceImpl: Sized + IATSCTuningSpaceImpl + IAnalogTVTuningSpaceImpl + ITuningSpaceImpl + IDispatchImpl {
    fn MinMajorChannel();
    fn SetMinMajorChannel();
    fn MaxMajorChannel();
    fn SetMaxMajorChannel();
    fn MinSourceID();
    fn SetMinSourceID();
    fn MaxSourceID();
    fn SetMaxSourceID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDigitalLocatorImpl: Sized + ILocatorImpl + IDispatchImpl {}
pub trait IDirectDrawMediaSampleImpl: Sized {
    fn GetSurfaceAndReleaseLock();
    fn LockMediaSamplePointer();
}
pub trait IDirectDrawMediaSampleAllocatorImpl: Sized {
    fn GetDirectDraw();
}
pub trait IDirectDrawMediaStreamImpl: Sized + IMediaStreamImpl {
    fn GetFormat();
    fn SetFormat();
    fn GetDirectDraw();
    fn SetDirectDraw();
    fn CreateSample();
    fn GetTimePerFrame();
}
pub trait IDirectDrawStreamSampleImpl: Sized + IStreamSampleImpl {
    fn GetSurface();
    fn SetRect();
}
pub trait IDirectDrawVideoImpl: Sized {
    fn GetSwitches();
    fn SetSwitches();
    fn GetCaps();
    fn GetEmulatedCaps();
    fn GetSurfaceDesc();
    fn GetFourCCCodes();
    fn SetDirectDraw();
    fn GetDirectDraw();
    fn GetSurfaceType();
    fn SetDefault();
    fn UseScanLine();
    fn CanUseScanLine();
    fn UseOverlayStretch();
    fn CanUseOverlayStretch();
    fn UseWhenFullScreen();
    fn WillUseFullScreen();
}
pub trait IDistributorNotifyImpl: Sized {
    fn Stop();
    fn Pause();
    fn Run();
    fn SetSyncSource();
    fn NotifyGraphChange();
}
pub trait IDrawVideoImageImpl: Sized {
    fn DrawVideoImageBegin();
    fn DrawVideoImageEnd();
    fn DrawVideoImageDraw();
}
pub trait IDvbCableDeliverySystemDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetFrequency();
    fn GetFECOuter();
    fn GetModulation();
    fn GetSymbolRate();
    fn GetFECInner();
}
pub trait IDvbComponentDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetStreamContent();
    fn GetComponentType();
    fn GetComponentTag();
    fn GetLanguageCode();
    fn GetTextW();
}
pub trait IDvbContentDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordContentNibbles();
    fn GetRecordUserNibbles();
}
pub trait IDvbContentIdentifierDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordCrid();
}
pub trait IDvbDataBroadcastDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetDataBroadcastID();
    fn GetComponentTag();
    fn GetSelectorLength();
    fn GetSelectorBytes();
    fn GetLangID();
    fn GetTextLength();
    fn GetText();
}
pub trait IDvbDataBroadcastIDDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetDataBroadcastID();
    fn GetIDSelectorBytes();
}
pub trait IDvbDefaultAuthorityDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetDefaultAuthority();
}
pub trait IDvbExtendedEventDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetDescriptorNumber();
    fn GetLastDescriptorNumber();
    fn GetLanguageCode();
    fn GetCountOfRecords();
    fn GetRecordItemW();
    fn GetConcatenatedItemW();
    fn GetTextW();
    fn GetConcatenatedTextW();
    fn GetRecordItemRawBytes();
}
pub trait IDvbFrequencyListDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCodingType();
    fn GetCountOfRecords();
    fn GetRecordCentreFrequency();
}
pub trait IDvbHDSimulcastLogicalChannelDescriptorImpl: Sized + IDvbLogicalChannelDescriptor2Impl + IDvbLogicalChannelDescriptorImpl {}
pub trait IDvbLinkageDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetTSId();
    fn GetONId();
    fn GetServiceId();
    fn GetLinkageType();
    fn GetPrivateDataLength();
    fn GetPrivateData();
}
pub trait IDvbLogicalChannel2DescriptorImpl: Sized + IDvbLogicalChannelDescriptor2Impl + IDvbLogicalChannelDescriptorImpl {
    fn GetCountOfLists();
    fn GetListId();
    fn GetListNameW();
    fn GetListCountryCode();
    fn GetListCountOfRecords();
    fn GetListRecordServiceId();
    fn GetListRecordLogicalChannelNumber();
    fn GetListRecordLogicalChannelAndVisibility();
}
pub trait IDvbLogicalChannelDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordServiceId();
    fn GetRecordLogicalChannelNumber();
}
pub trait IDvbLogicalChannelDescriptor2Impl: Sized + IDvbLogicalChannelDescriptorImpl {
    fn GetRecordLogicalChannelAndVisibility();
}
pub trait IDvbMultilingualServiceNameDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordLangId();
    fn GetRecordServiceProviderNameW();
    fn GetRecordServiceNameW();
}
pub trait IDvbNetworkNameDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetNetworkName();
    fn GetNetworkNameW();
}
pub trait IDvbParentalRatingDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordRating();
}
pub trait IDvbPrivateDataSpecifierDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetPrivateDataSpecifier();
}
pub trait IDvbSatelliteDeliverySystemDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetFrequency();
    fn GetOrbitalPosition();
    fn GetWestEastFlag();
    fn GetPolarization();
    fn GetModulation();
    fn GetSymbolRate();
    fn GetFECInner();
}
pub trait IDvbServiceAttributeDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordServiceId();
    fn GetRecordNumericSelectionFlag();
    fn GetRecordVisibleServiceFlag();
}
pub trait IDvbServiceDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetServiceType();
    fn GetServiceProviderName();
    fn GetServiceProviderNameW();
    fn GetServiceName();
    fn GetProcessedServiceName();
    fn GetServiceNameEmphasized();
}
pub trait IDvbServiceDescriptor2Impl: Sized + IDvbServiceDescriptorImpl {
    fn GetServiceProviderNameW();
    fn GetServiceNameW();
}
pub trait IDvbServiceListDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordServiceId();
    fn GetRecordServiceType();
}
pub trait IDvbShortEventDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetLanguageCode();
    fn GetEventNameW();
    fn GetTextW();
}
pub trait IDvbSiParserImpl: Sized {
    fn Initialize();
    fn GetPAT();
    fn GetCAT();
    fn GetPMT();
    fn GetTSDT();
    fn GetNIT();
    fn GetSDT();
    fn GetEIT();
    fn GetBAT();
    fn GetRST();
    fn GetST();
    fn GetTDT();
    fn GetTOT();
    fn GetDIT();
    fn GetSIT();
}
pub trait IDvbSiParser2Impl: Sized + IDvbSiParserImpl {
    fn GetEIT2();
}
pub trait IDvbSubtitlingDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordLangId();
    fn GetRecordSubtitlingType();
    fn GetRecordCompositionPageID();
    fn GetRecordAncillaryPageID();
}
pub trait IDvbTeletextDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetRecordLangId();
    fn GetRecordTeletextType();
    fn GetRecordMagazineNumber();
    fn GetRecordPageNumber();
}
pub trait IDvbTerrestrial2DeliverySystemDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetTagExtension();
    fn GetCentreFrequency();
    fn GetPLPId();
    fn GetT2SystemId();
    fn GetMultipleInputMode();
    fn GetBandwidth();
    fn GetGuardInterval();
    fn GetTransmissionMode();
    fn GetCellId();
    fn GetOtherFrequencyFlag();
    fn GetTFSFlag();
}
pub trait IDvbTerrestrialDeliverySystemDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCentreFrequency();
    fn GetBandwidth();
    fn GetConstellation();
    fn GetHierarchyInformation();
    fn GetCodeRateHPStream();
    fn GetCodeRateLPStream();
    fn GetGuardInterval();
    fn GetTransmissionMode();
    fn GetOtherFrequencyFlag();
}
pub trait IDvdCmdImpl: Sized {
    fn WaitForStart();
    fn WaitForEnd();
}
pub trait IDvdControlImpl: Sized {
    fn TitlePlay();
    fn ChapterPlay();
    fn TimePlay();
    fn StopForResume();
    fn GoUp();
    fn TimeSearch();
    fn ChapterSearch();
    fn PrevPGSearch();
    fn TopPGSearch();
    fn NextPGSearch();
    fn ForwardScan();
    fn BackwardScan();
    fn MenuCall();
    fn Resume();
    fn UpperButtonSelect();
    fn LowerButtonSelect();
    fn LeftButtonSelect();
    fn RightButtonSelect();
    fn ButtonActivate();
    fn ButtonSelectAndActivate();
    fn StillOff();
    fn PauseOn();
    fn PauseOff();
    fn MenuLanguageSelect();
    fn AudioStreamChange();
    fn SubpictureStreamChange();
    fn AngleChange();
    fn ParentalLevelSelect();
    fn ParentalCountrySelect();
    fn KaraokeAudioPresentationModeChange();
    fn VideoModePreferrence();
    fn SetRoot();
    fn MouseActivate();
    fn MouseSelect();
    fn ChapterPlayAutoStop();
}
pub trait IDvdControl2Impl: Sized {
    fn PlayTitle();
    fn PlayChapterInTitle();
    fn PlayAtTimeInTitle();
    fn Stop();
    fn ReturnFromSubmenu();
    fn PlayAtTime();
    fn PlayChapter();
    fn PlayPrevChapter();
    fn ReplayChapter();
    fn PlayNextChapter();
    fn PlayForwards();
    fn PlayBackwards();
    fn ShowMenu();
    fn Resume();
    fn SelectRelativeButton();
    fn ActivateButton();
    fn SelectButton();
    fn SelectAndActivateButton();
    fn StillOff();
    fn Pause();
    fn SelectAudioStream();
    fn SelectSubpictureStream();
    fn SetSubpictureState();
    fn SelectAngle();
    fn SelectParentalLevel();
    fn SelectParentalCountry();
    fn SelectKaraokeAudioPresentationMode();
    fn SelectVideoModePreference();
    fn SetDVDDirectory();
    fn ActivateAtPosition();
    fn SelectAtPosition();
    fn PlayChaptersAutoStop();
    fn AcceptParentalLevelChange();
    fn SetOption();
    fn SetState();
    fn PlayPeriodInTitleAutoStop();
    fn SetGPRM();
    fn SelectDefaultMenuLanguage();
    fn SelectDefaultAudioLanguage();
    fn SelectDefaultSubpictureLanguage();
}
pub trait IDvdGraphBuilderImpl: Sized {
    fn GetFiltergraph();
    fn GetDvdInterface();
    fn RenderDvdVideoVolume();
}
pub trait IDvdInfoImpl: Sized {
    fn GetCurrentDomain();
    fn GetCurrentLocation();
    fn GetTotalTitleTime();
    fn GetCurrentButton();
    fn GetCurrentAngle();
    fn GetCurrentAudio();
    fn GetCurrentSubpicture();
    fn GetCurrentUOPS();
    fn GetAllSPRMs();
    fn GetAllGPRMs();
    fn GetAudioLanguage();
    fn GetSubpictureLanguage();
    fn GetTitleAttributes();
    fn GetVMGAttributes();
    fn GetCurrentVideoAttributes();
    fn GetCurrentAudioAttributes();
    fn GetCurrentSubpictureAttributes();
    fn GetCurrentVolumeInfo();
    fn GetDVDTextInfo();
    fn GetPlayerParentalLevel();
    fn GetNumberOfChapters();
    fn GetTitleParentalLevels();
    fn GetRoot();
}
pub trait IDvdInfo2Impl: Sized {
    fn GetCurrentDomain();
    fn GetCurrentLocation();
    fn GetTotalTitleTime();
    fn GetCurrentButton();
    fn GetCurrentAngle();
    fn GetCurrentAudio();
    fn GetCurrentSubpicture();
    fn GetCurrentUOPS();
    fn GetAllSPRMs();
    fn GetAllGPRMs();
    fn GetAudioLanguage();
    fn GetSubpictureLanguage();
    fn GetTitleAttributes();
    fn GetVMGAttributes();
    fn GetCurrentVideoAttributes();
    fn GetAudioAttributes();
    fn GetKaraokeAttributes();
    fn GetSubpictureAttributes();
    fn GetDVDVolumeInfo();
    fn GetDVDTextNumberOfLanguages();
    fn GetDVDTextLanguageInfo();
    fn GetDVDTextStringAsNative();
    fn GetDVDTextStringAsUnicode();
    fn GetPlayerParentalLevel();
    fn GetNumberOfChapters();
    fn GetTitleParentalLevels();
    fn GetDVDDirectory();
    fn IsAudioStreamEnabled();
    fn GetDiscID();
    fn GetState();
    fn GetMenuLanguages();
    fn GetButtonAtPosition();
    fn GetCmdFromEvent();
    fn GetDefaultMenuLanguage();
    fn GetDefaultAudioLanguage();
    fn GetDefaultSubpictureLanguage();
    fn GetDecoderCaps();
    fn GetButtonRect();
    fn IsSubpictureStreamEnabled();
}
pub trait IDvdStateImpl: Sized {
    fn GetDiscID();
    fn GetParentalLevel();
}
pub trait IESCloseMmiEventImpl: Sized + IESEventImpl {
    fn GetDialogNumber();
}
pub trait IESEventImpl: Sized {
    fn GetEventId();
    fn GetEventType();
    fn SetCompletionStatus();
    fn GetData();
    fn GetStringData();
}
pub trait IESEventFactoryImpl: Sized {
    fn CreateESEvent();
}
pub trait IESEventServiceImpl: Sized {
    fn FireESEvent();
}
pub trait IESEventServiceConfigurationImpl: Sized {
    fn SetParent();
    fn RemoveParent();
    fn SetOwner();
    fn RemoveOwner();
    fn SetGraph();
    fn RemoveGraph();
}
pub trait IESEventsImpl: Sized {
    fn OnESEventReceived();
}
pub trait IESFileExpiryDateEventImpl: Sized + IESEventImpl {
    fn GetTunerId();
    fn GetExpiryDate();
    fn GetFinalExpiryDate();
    fn GetMaxRenewalCount();
    fn IsEntitlementTokenPresent();
    fn DoesExpireAfterFirstUse();
}
pub trait IESIsdbCasResponseEventImpl: Sized + IESEventImpl {
    fn GetRequestId();
    fn GetStatus();
    fn GetDataLength();
    fn GetResponseData();
}
pub trait IESLicenseRenewalResultEventImpl: Sized + IESEventImpl {
    fn GetCallersId();
    fn GetFileName();
    fn IsRenewalSuccessful();
    fn IsCheckEntitlementCallRequired();
    fn GetDescrambledStatus();
    fn GetRenewalResultCode();
    fn GetCASFailureCode();
    fn GetRenewalHResult();
    fn GetEntitlementTokenLength();
    fn GetEntitlementToken();
    fn GetExpiryDate();
}
pub trait IESOpenMmiEventImpl: Sized + IESEventImpl {
    fn GetDialogNumber();
    fn GetDialogType();
    fn GetDialogData();
    fn GetDialogStringData();
}
pub trait IESRequestTunerEventImpl: Sized + IESEventImpl {
    fn GetPriority();
    fn GetReason();
    fn GetConsequences();
    fn GetEstimatedTime();
}
pub trait IESValueUpdatedEventImpl: Sized + IESEventImpl {
    fn GetValueNames();
}
pub trait IETFilterImpl: Sized {
    fn EvalRatObjOK();
    fn GetCurrRating();
    fn GetCurrLicenseExpDate();
    fn GetLastErrorCode();
    fn SetRecordingOn();
}
pub trait IETFilterConfigImpl: Sized {
    fn InitLicense();
    fn GetSecureChannelObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IETFilterEventsImpl: Sized + IDispatchImpl {}
pub trait IEncoderAPIImpl: Sized {
    fn IsSupported();
    fn IsAvailable();
    fn GetParameterRange();
    fn GetParameterValues();
    fn GetDefaultValue();
    fn GetValue();
    fn SetValue();
}
pub trait IEnumComponentTypesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumComponentsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumFiltersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumGuideDataPropertiesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumMSVidGraphSegmentImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumMediaTypesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumPIDMapImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumPinsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumRegFiltersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumStreamBufferRecordingAttribImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumStreamIdMapImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumTuneRequestsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumTuningSpacesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEvalRatImpl: Sized + IDispatchImpl {
    fn BlockedRatingAttributes();
    fn SetBlockedRatingAttributes();
    fn BlockUnRated();
    fn SetBlockUnRated();
    fn MostRestrictiveRating();
    fn TestRating();
}
pub trait IFileSinkFilterImpl: Sized {
    fn SetFileName();
    fn GetCurFile();
}
pub trait IFileSinkFilter2Impl: Sized + IFileSinkFilterImpl {
    fn SetMode();
    fn GetMode();
}
pub trait IFileSourceFilterImpl: Sized {
    fn Load();
    fn GetCurFile();
}
pub trait IFilterChainImpl: Sized {
    fn StartChain();
    fn PauseChain();
    fn StopChain();
    fn RemoveChain();
}
pub trait IFilterGraphImpl: Sized {
    fn AddFilter();
    fn RemoveFilter();
    fn EnumFilters();
    fn FindFilterByName();
    fn ConnectDirect();
    fn Reconnect();
    fn Disconnect();
    fn SetDefaultSyncSource();
}
pub trait IFilterGraph2Impl: Sized + IGraphBuilderImpl + IFilterGraphImpl {
    fn AddSourceFilterForMoniker();
    fn ReconnectEx();
    fn RenderEx();
}
pub trait IFilterGraph3Impl: Sized + IFilterGraph2Impl + IGraphBuilderImpl + IFilterGraphImpl {
    fn SetSyncSourceEx();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFilterInfoImpl: Sized + IDispatchImpl {
    fn FindPin();
    fn Name();
    fn VendorInfo();
    fn Filter();
    fn Pins();
    fn IsFileSource();
    fn Filename();
    fn SetFilename();
}
pub trait IFilterMapperImpl: Sized {
    fn RegisterFilter();
    fn RegisterFilterInstance();
    fn RegisterPin();
    fn RegisterPinType();
    fn UnregisterFilter();
    fn UnregisterFilterInstance();
    fn UnregisterPin();
    fn EnumMatchingFilters();
}
pub trait IFilterMapper2Impl: Sized {
    fn CreateCategory();
    fn UnregisterFilter();
    fn RegisterFilter();
    fn EnumMatchingFilters();
}
pub trait IFilterMapper3Impl: Sized + IFilterMapper2Impl {
    fn GetICreateDevEnum();
}
pub trait IFrequencyMapImpl: Sized {
    fn FrequencyMapping();
    fn SetFrequencyMapping();
    fn CountryCode();
    fn SetCountryCode();
    fn DefaultFrequencyMapping();
    fn CountryCodeList();
}
pub trait IFullScreenVideoImpl: Sized {
    fn CountModes();
    fn GetModeInfo();
    fn GetCurrentMode();
    fn IsModeAvailable();
    fn IsModeEnabled();
    fn SetEnabled();
    fn GetClipFactor();
    fn SetClipFactor();
    fn SetMessageDrain();
    fn GetMessageDrain();
    fn SetMonitor();
    fn GetMonitor();
    fn HideOnDeactivate();
    fn IsHideOnDeactivate();
    fn SetCaption();
    fn GetCaption();
    fn SetDefault();
}
pub trait IFullScreenVideoExImpl: Sized + IFullScreenVideoImpl {
    fn SetAcceleratorTable();
    fn GetAcceleratorTable();
    fn KeepPixelAspectRatio();
    fn IsKeepPixelAspectRatio();
}
pub trait IGenericDescriptorImpl: Sized {
    fn Initialize();
    fn GetTag();
    fn GetLength();
    fn GetBody();
}
pub trait IGenericDescriptor2Impl: Sized + IGenericDescriptorImpl {
    fn Initialize();
    fn GetLength();
}
pub trait IGetCapabilitiesKeyImpl: Sized {
    fn GetCapabilitiesKey();
}
pub trait IGpnvsCommonBaseImpl: Sized {
    fn GetValueUpdateName();
}
pub trait IGraphBuilderImpl: Sized + IFilterGraphImpl {
    fn Connect();
    fn Render();
    fn RenderFile();
    fn AddSourceFilter();
    fn SetLogFile();
    fn Abort();
    fn ShouldOperationContinue();
}
pub trait IGraphConfigImpl: Sized {
    fn Reconnect();
    fn Reconfigure();
    fn AddFilterToCache();
    fn EnumCacheFilter();
    fn RemoveFilterFromCache();
    fn GetStartTime();
    fn PushThroughData();
    fn SetFilterFlags();
    fn GetFilterFlags();
    fn RemoveFilterEx();
}
pub trait IGraphConfigCallbackImpl: Sized {
    fn Reconfigure();
}
pub trait IGraphVersionImpl: Sized {
    fn QueryVersion();
}
pub trait IGuideDataImpl: Sized {
    fn GetServices();
    fn GetServiceProperties();
    fn GetGuideProgramIDs();
    fn GetProgramProperties();
    fn GetScheduleEntryIDs();
    fn GetScheduleEntryProperties();
}
pub trait IGuideDataEventImpl: Sized {
    fn GuideDataAcquired();
    fn ProgramChanged();
    fn ServiceChanged();
    fn ScheduleEntryChanged();
    fn ProgramDeleted();
    fn ServiceDeleted();
    fn ScheduleDeleted();
}
pub trait IGuideDataLoaderImpl: Sized {
    fn Init();
    fn Terminate();
}
pub trait IGuideDataPropertyImpl: Sized {
    fn Name();
    fn Language();
    fn Value();
}
pub trait IIPDVDecImpl: Sized {
    fn IPDisplay();
    fn SetIPDisplay();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IISDBSLocatorImpl: Sized + IDVBSLocatorImpl + IDigitalLocatorImpl + ILocatorImpl + IDispatchImpl {}
pub trait IISDB_BITImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetOriginalNetworkId();
    fn GetBroadcastViewPropriety();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn GetCountOfRecords();
    fn GetRecordBroadcasterId();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn GetVersionHash();
}
pub trait IISDB_CDTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetDownloadDataId();
    fn GetSectionNumber();
    fn GetOriginalNetworkId();
    fn GetDataType();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn GetSizeOfDataModule();
    fn GetDataModule();
    fn GetVersionHash();
}
pub trait IISDB_EMMImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetTableIdExtension();
    fn GetDataBytes();
    fn GetSharedEmmMessage();
    fn GetIndividualEmmMessage();
    fn GetVersionHash();
}
pub trait IISDB_LDTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetOriginalServiceId();
    fn GetTransportStreamId();
    fn GetOriginalNetworkId();
    fn GetCountOfRecords();
    fn GetRecordDescriptionId();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn GetVersionHash();
}
pub trait IISDB_NBITImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetOriginalNetworkId();
    fn GetCountOfRecords();
    fn GetRecordInformationId();
    fn GetRecordInformationType();
    fn GetRecordDescriptionBodyLocation();
    fn GetRecordMessageSectionNumber();
    fn GetRecordUserDefined();
    fn GetRecordNumberOfKeys();
    fn GetRecordKeys();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn GetVersionHash();
}
pub trait IISDB_SDTImpl: Sized + IDVB_SDTImpl {
    fn GetRecordEITUserDefinedFlags();
}
pub trait IISDB_SDTTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetTableIdExt();
    fn GetTransportStreamId();
    fn GetOriginalNetworkId();
    fn GetServiceId();
    fn GetCountOfRecords();
    fn GetRecordGroup();
    fn GetRecordTargetVersion();
    fn GetRecordNewVersion();
    fn GetRecordDownloadLevel();
    fn GetRecordVersionIndicator();
    fn GetRecordScheduleTimeShiftInformation();
    fn GetRecordCountOfSchedules();
    fn GetRecordStartTimeByIndex();
    fn GetRecordDurationByIndex();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn GetVersionHash();
}
pub trait IIsdbAudioComponentDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetStreamContent();
    fn GetComponentType();
    fn GetComponentTag();
    fn GetStreamType();
    fn GetSimulcastGroupTag();
    fn GetESMultiLingualFlag();
    fn GetMainComponentFlag();
    fn GetQualityIndicator();
    fn GetSamplingRate();
    fn GetLanguageCode();
    fn GetLanguageCode2();
    fn GetTextW();
}
pub trait IIsdbCAContractInformationDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCASystemId();
    fn GetCAUnitId();
    fn GetCountOfRecords();
    fn GetRecordComponentTag();
    fn GetContractVerificationInfoLength();
    fn GetContractVerificationInfo();
    fn GetFeeNameW();
}
pub trait IIsdbCADescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCASystemId();
    fn GetReservedBits();
    fn GetCAPID();
    fn GetPrivateDataBytes();
}
pub trait IIsdbCAServiceDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCASystemId();
    fn GetCABroadcasterGroupId();
    fn GetMessageControl();
    fn GetServiceIds();
}
pub trait IIsdbComponentGroupDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetComponentGroupType();
    fn GetCountOfRecords();
    fn GetRecordGroupId();
    fn GetRecordNumberOfCAUnit();
    fn GetRecordCAUnitCAUnitId();
    fn GetRecordCAUnitNumberOfComponents();
    fn GetRecordCAUnitComponentTag();
    fn GetRecordTotalBitRate();
    fn GetRecordTextW();
}
pub trait IIsdbDataContentDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetDataComponentId();
    fn GetEntryComponent();
    fn GetSelectorLength();
    fn GetSelectorBytes();
    fn GetCountOfRecords();
    fn GetRecordComponentRef();
    fn GetLanguageCode();
    fn GetTextW();
}
pub trait IIsdbDigitalCopyControlDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCopyControl();
    fn GetCountOfRecords();
    fn GetRecordCopyControl();
}
pub trait IIsdbDownloadContentDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetFlags();
    fn GetComponentSize();
    fn GetDownloadId();
    fn GetTimeOutValueDII();
    fn GetLeakRate();
    fn GetComponentTag();
    fn GetCompatiblityDescriptorLength();
    fn GetCompatiblityDescriptor();
    fn GetCountOfRecords();
    fn GetRecordModuleId();
    fn GetRecordModuleSize();
    fn GetRecordModuleInfoLength();
    fn GetRecordModuleInfo();
    fn GetTextLanguageCode();
    fn GetTextW();
}
pub trait IIsdbEmergencyInformationDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetCountOfRecords();
    fn GetServiceId();
    fn GetStartEndFlag();
    fn GetSignalLevel();
    fn GetAreaCode();
}
pub trait IIsdbEventGroupDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetGroupType();
    fn GetCountOfRecords();
    fn GetRecordEvent();
    fn GetCountOfRefRecords();
    fn GetRefRecordEvent();
}
pub trait IIsdbHierarchicalTransmissionDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetFutureUse1();
    fn GetQualityLevel();
    fn GetFutureUse2();
    fn GetReferencePid();
}
pub trait IIsdbLogoTransmissionDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetLogoTransmissionType();
    fn GetLogoId();
    fn GetLogoVersion();
    fn GetDownloadDataId();
    fn GetLogoCharW();
}
pub trait IIsdbSIParameterDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetParameterVersion();
    fn GetUpdateTime();
    fn GetRecordNumberOfTable();
    fn GetTableId();
    fn GetTableDescriptionLength();
    fn GetTableDescriptionBytes();
}
pub trait IIsdbSeriesDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetSeriesId();
    fn GetRepeatLabel();
    fn GetProgramPattern();
    fn GetExpireDate();
    fn GetEpisodeNumber();
    fn GetLastEpisodeNumber();
    fn GetSeriesNameW();
}
pub trait IIsdbSiParser2Impl: Sized + IDvbSiParser2Impl + IDvbSiParserImpl {
    fn GetSDT();
    fn GetBIT();
    fn GetNBIT();
    fn GetLDT();
    fn GetSDTT();
    fn GetCDT();
    fn GetEMM();
}
pub trait IIsdbTSInformationDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetRemoteControlKeyId();
    fn GetTSNameW();
    fn GetCountOfRecords();
    fn GetRecordTransmissionTypeInfo();
    fn GetRecordNumberOfServices();
    fn GetRecordServiceIdByIndex();
}
pub trait IIsdbTerrestrialDeliverySystemDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetAreaCode();
    fn GetGuardInterval();
    fn GetTransmissionMode();
    fn GetCountOfRecords();
    fn GetRecordFrequency();
}
pub trait IKsNodeControlImpl: Sized {
    fn SetNodeId();
    fn SetKsControl();
}
pub trait IKsTopologyInfoImpl: Sized {
    fn NumCategories();
    fn Category();
    fn NumConnections();
    fn ConnectionInfo();
    fn NodeName();
    fn NumNodes();
    fn NodeType();
    fn CreateNodeInstance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILanguageComponentTypeImpl: Sized + IComponentTypeImpl + IDispatchImpl {
    fn LangID();
    fn SetLangID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILocatorImpl: Sized + IDispatchImpl {
    fn CarrierFrequency();
    fn SetCarrierFrequency();
    fn InnerFEC();
    fn SetInnerFEC();
    fn InnerFECRate();
    fn SetInnerFECRate();
    fn OuterFEC();
    fn SetOuterFEC();
    fn OuterFECRate();
    fn SetOuterFECRate();
    fn Modulation();
    fn SetModulation();
    fn SymbolRate();
    fn SetSymbolRate();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMPEG2ComponentImpl: Sized + IComponentImpl + IDispatchImpl {
    fn PID();
    fn SetPID();
    fn PCRPID();
    fn SetPCRPID();
    fn ProgramNumber();
    fn SetProgramNumber();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMPEG2ComponentTypeImpl: Sized + ILanguageComponentTypeImpl + IComponentTypeImpl + IDispatchImpl {
    fn StreamType();
    fn SetStreamType();
}
pub trait IMPEG2PIDMapImpl: Sized {
    fn MapPID();
    fn UnmapPID();
    fn EnumPIDMap();
}
pub trait IMPEG2StreamIdMapImpl: Sized {
    fn MapStreamId();
    fn UnmapStreamId();
    fn EnumStreamIdMap();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMPEG2TuneRequestImpl: Sized + ITuneRequestImpl + IDispatchImpl {
    fn TSID();
    fn SetTSID();
    fn ProgNo();
    fn SetProgNo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMPEG2TuneRequestFactoryImpl: Sized + IDispatchImpl {
    fn CreateTuneRequest();
}
pub trait IMPEG2TuneRequestSupportImpl: Sized {}
pub trait IMPEG2_TIF_CONTROLImpl: Sized {
    fn RegisterTIF();
    fn UnregisterTIF();
    fn AddPIDs();
    fn DeletePIDs();
    fn GetPIDCount();
    fn GetPIDs();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSEventBinderImpl: Sized + IDispatchImpl {
    fn Bind();
    fn Unbind();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidAnalogTunerImpl: Sized + IMSVidTunerImpl + IMSVidVideoInputDeviceImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Channel();
    fn SetChannel();
    fn VideoFrequency();
    fn AudioFrequency();
    fn CountryCode();
    fn SetCountryCode();
    fn SAP();
    fn SetSAP();
    fn ChannelAvailable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidAnalogTuner2Impl: Sized + IMSVidAnalogTunerImpl + IMSVidTunerImpl + IMSVidVideoInputDeviceImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn TVFormats();
    fn TunerModes();
    fn NumAuxInputs();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidAnalogTunerEventImpl: Sized + IMSVidTunerEventImpl + IMSVidInputDeviceEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidAudioRendererImpl: Sized + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn SetVolume();
    fn Volume();
    fn SetBalance();
    fn Balance();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidAudioRendererDevicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidAudioRendererEventImpl: Sized + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidAudioRendererEvent2Impl: Sized + IMSVidAudioRendererEventImpl + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn AVDecAudioDualMono();
    fn AVAudioSampleRate();
    fn AVAudioChannelConfig();
    fn AVAudioChannelCount();
    fn AVDecCommonMeanBitRate();
    fn AVDDSurroundMode();
    fn AVDecCommonInputFormat();
    fn AVDecCommonOutputFormat();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidClosedCaptioningImpl: Sized + IMSVidFeatureImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Enable();
    fn SetEnable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidClosedCaptioning2Impl: Sized + IMSVidClosedCaptioningImpl + IMSVidFeatureImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Service();
    fn SetService();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidClosedCaptioning3Impl: Sized + IMSVidClosedCaptioning2Impl + IMSVidClosedCaptioningImpl + IMSVidFeatureImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn TeleTextFilter();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidCompositionSegmentImpl: Sized + IMSVidGraphSegmentImpl + IPersistImpl {
    fn Compose();
    fn Up();
    fn Down();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidCtlImpl: Sized + IDispatchImpl {
    fn AutoSize();
    fn SetAutoSize();
    fn BackColor();
    fn SetBackColor();
    fn Enabled();
    fn SetEnabled();
    fn TabStop();
    fn SetTabStop();
    fn Window();
    fn Refresh();
    fn DisplaySize();
    fn SetDisplaySize();
    fn MaintainAspectRatio();
    fn SetMaintainAspectRatio();
    fn ColorKey();
    fn SetColorKey();
    fn InputsAvailable();
    fn OutputsAvailable();
    fn _InputsAvailable();
    fn _OutputsAvailable();
    fn VideoRenderersAvailable();
    fn AudioRenderersAvailable();
    fn FeaturesAvailable();
    fn InputActive();
    fn SetInputActive();
    fn OutputsActive();
    fn SetOutputsActive();
    fn VideoRendererActive();
    fn SetVideoRendererActive();
    fn AudioRendererActive();
    fn SetAudioRendererActive();
    fn FeaturesActive();
    fn SetFeaturesActive();
    fn State();
    fn View();
    fn Build();
    fn Pause();
    fn Run();
    fn Stop();
    fn Decompose();
    fn DisableVideo();
    fn DisableAudio();
    fn ViewNext();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidDataServicesImpl: Sized + IMSVidFeatureImpl + IMSVidDeviceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidDataServicesEventImpl: Sized + IMSVidDeviceEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidDeviceImpl: Sized + IDispatchImpl {
    fn Name();
    fn Status();
    fn SetPower();
    fn Power();
    fn Category();
    fn ClassID();
    fn _Category();
    fn _ClassID();
    fn IsEqualDevice();
}
pub trait IMSVidDevice2Impl: Sized {
    fn DevicePath();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidDeviceEventImpl: Sized + IDispatchImpl {
    fn StateChange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidEVRImpl: Sized + IMSVidVideoRendererImpl + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Presenter();
    fn SetPresenter();
    fn SetSuppressEffects();
    fn SuppressEffects();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidEVREventImpl: Sized + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn OnUserEvent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidEncoderImpl: Sized + IMSVidFeatureImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn VideoEncoderInterface();
    fn AudioEncoderInterface();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidFeatureImpl: Sized + IMSVidDeviceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidFeatureEventImpl: Sized + IMSVidDeviceEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidFeaturesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidFilePlaybackImpl: Sized + IMSVidPlaybackImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn FileName();
    fn SetFileName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidFilePlayback2Impl: Sized + IMSVidFilePlaybackImpl + IMSVidPlaybackImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Set_SourceFilter();
    fn Set__SourceFilter();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidFilePlaybackEventImpl: Sized + IMSVidPlaybackEventImpl + IMSVidInputDeviceEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidGenericSinkImpl: Sized + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn SetSinkFilter();
    fn SinkStreams();
    fn SetSinkStreams();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidGenericSink2Impl: Sized + IMSVidGenericSinkImpl + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn AddFilter();
    fn ResetFilterList();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidGraphSegmentImpl: Sized + IPersistImpl {
    fn Init();
    fn SetInit();
    fn EnumFilters();
    fn Container();
    fn SetContainer();
    fn Type();
    fn Category();
    fn Build();
    fn PostBuild();
    fn PreRun();
    fn PostRun();
    fn PreStop();
    fn PostStop();
    fn OnEventNotify();
    fn Decompose();
}
pub trait IMSVidGraphSegmentContainerImpl: Sized {
    fn Graph();
    fn Input();
    fn Outputs();
    fn VideoRenderer();
    fn AudioRenderer();
    fn Features();
    fn Composites();
    fn ParentContainer();
    fn Decompose();
    fn IsWindowless();
    fn GetFocus();
}
pub trait IMSVidGraphSegmentUserInputImpl: Sized {
    fn Click();
    fn DblClick();
    fn KeyDown();
    fn KeyPress();
    fn KeyUp();
    fn MouseDown();
    fn MouseMove();
    fn MouseUp();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidInputDeviceImpl: Sized + IMSVidDeviceImpl + IDispatchImpl {
    fn IsViewable();
    fn View();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidInputDeviceEventImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidInputDevicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidOutputDeviceImpl: Sized + IMSVidDeviceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidOutputDeviceEventImpl: Sized + IMSVidDeviceEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidOutputDevicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidPlaybackImpl: Sized + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn EnableResetOnStop();
    fn SetEnableResetOnStop();
    fn Run();
    fn Pause();
    fn Stop();
    fn CanStep();
    fn Step();
    fn SetRate();
    fn Rate();
    fn SetCurrentPosition();
    fn CurrentPosition();
    fn SetPositionMode();
    fn PositionMode();
    fn Length();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidPlaybackEventImpl: Sized + IMSVidInputDeviceEventImpl + IDispatchImpl {
    fn EndOfMedia();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidRectImpl: Sized + IDispatchImpl {
    fn Top();
    fn SetTop();
    fn Left();
    fn SetLeft();
    fn Width();
    fn SetWidth();
    fn Height();
    fn SetHeight();
    fn HWnd();
    fn SetHWnd();
    fn SetRect();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferRecordingControlImpl: Sized + IDispatchImpl {
    fn StartTime();
    fn SetStartTime();
    fn StopTime();
    fn SetStopTime();
    fn RecordingStopped();
    fn RecordingStarted();
    fn RecordingType();
    fn RecordingAttribute();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSinkImpl: Sized + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn ContentRecorder();
    fn ReferenceRecorder();
    fn SinkName();
    fn SetSinkName();
    fn NameSetLock();
    fn SBESink();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSink2Impl: Sized + IMSVidStreamBufferSinkImpl + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn UnlockProfile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSink3Impl: Sized + IMSVidStreamBufferSink2Impl + IMSVidStreamBufferSinkImpl + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn SetMinSeek();
    fn AudioCounter();
    fn VideoCounter();
    fn CCCounter();
    fn WSTCounter();
    fn SetAudioAnalysisFilter();
    fn AudioAnalysisFilter();
    fn Set_AudioAnalysisFilter();
    fn _AudioAnalysisFilter();
    fn SetVideoAnalysisFilter();
    fn VideoAnalysisFilter();
    fn Set_VideoAnalysisFilter();
    fn _VideoAnalysisFilter();
    fn SetDataAnalysisFilter();
    fn DataAnalysisFilter();
    fn Set_DataAnalysisFilter();
    fn _DataAnalysisFilter();
    fn LicenseErrorCode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSinkEventImpl: Sized + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn CertificateFailure();
    fn CertificateSuccess();
    fn WriteFailure();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSinkEvent2Impl: Sized + IMSVidStreamBufferSinkEventImpl + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn EncryptionOn();
    fn EncryptionOff();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSinkEvent3Impl: Sized + IMSVidStreamBufferSinkEvent2Impl + IMSVidStreamBufferSinkEventImpl + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn LicenseChange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSinkEvent4Impl: Sized + IMSVidStreamBufferSinkEvent3Impl + IMSVidStreamBufferSinkEvent2Impl + IMSVidStreamBufferSinkEventImpl + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn WriteFailureClear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSourceImpl: Sized + IMSVidFilePlaybackImpl + IMSVidPlaybackImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Start();
    fn RecordingAttribute();
    fn CurrentRatings();
    fn MaxRatingsLevel();
    fn SetBlockUnrated();
    fn SetUnratedDelay();
    fn SBESource();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSource2Impl: Sized + IMSVidStreamBufferSourceImpl + IMSVidFilePlaybackImpl + IMSVidPlaybackImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn SetRateEx();
    fn AudioCounter();
    fn VideoCounter();
    fn CCCounter();
    fn WSTCounter();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSourceEventImpl: Sized + IMSVidFilePlaybackEventImpl + IMSVidPlaybackEventImpl + IMSVidInputDeviceEventImpl + IDispatchImpl {
    fn CertificateFailure();
    fn CertificateSuccess();
    fn RatingsBlocked();
    fn RatingsUnblocked();
    fn RatingsChanged();
    fn TimeHole();
    fn StaleDataRead();
    fn ContentBecomingStale();
    fn StaleFileDeleted();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSourceEvent2Impl: Sized + IMSVidStreamBufferSourceEventImpl + IMSVidFilePlaybackEventImpl + IMSVidPlaybackEventImpl + IMSVidInputDeviceEventImpl + IDispatchImpl {
    fn RateChange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferSourceEvent3Impl: Sized + IMSVidStreamBufferSourceEvent2Impl + IMSVidStreamBufferSourceEventImpl + IMSVidFilePlaybackEventImpl + IMSVidPlaybackEventImpl + IMSVidInputDeviceEventImpl + IDispatchImpl {
    fn BroadcastEvent();
    fn BroadcastEventEx();
    fn COPPBlocked();
    fn COPPUnblocked();
    fn ContentPrimarilyAudio();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidStreamBufferV2SourceEventImpl: Sized + IMSVidFilePlaybackEventImpl + IMSVidPlaybackEventImpl + IMSVidInputDeviceEventImpl + IDispatchImpl {
    fn RatingsChanged();
    fn TimeHole();
    fn StaleDataRead();
    fn ContentBecomingStale();
    fn StaleFileDeleted();
    fn RateChange();
    fn BroadcastEvent();
    fn BroadcastEventEx();
    fn ContentPrimarilyAudio();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidTunerImpl: Sized + IMSVidVideoInputDeviceImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Tune();
    fn SetTune();
    fn TuningSpace();
    fn SetTuningSpace();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidTunerEventImpl: Sized + IMSVidInputDeviceEventImpl + IDispatchImpl {
    fn TuneChanged();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVMR9Impl: Sized + IMSVidVideoRendererImpl + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Allocator_ID();
    fn SetAllocator();
    fn SetSuppressEffects();
    fn SuppressEffects();
    fn Allocator();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVRGraphSegmentImpl: Sized + IMSVidGraphSegmentImpl + IPersistImpl {
    fn Set_VMRendererMode();
    fn SetOwner();
    fn Owner();
    fn UseOverlay();
    fn SetUseOverlay();
    fn Visible();
    fn SetVisible();
    fn ColorKey();
    fn SetColorKey();
    fn Source();
    fn SetSource();
    fn Destination();
    fn SetDestination();
    fn NativeSize();
    fn BorderColor();
    fn SetBorderColor();
    fn MaintainAspectRatio();
    fn SetMaintainAspectRatio();
    fn Refresh();
    fn DisplayChange();
    fn RePaint();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVideoInputDeviceImpl: Sized + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVideoRendererImpl: Sized + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn CustomCompositorClass();
    fn SetCustomCompositorClass();
    fn _CustomCompositorClass();
    fn Set_CustomCompositorClass();
    fn _CustomCompositor();
    fn Set_CustomCompositor();
    fn MixerBitmap();
    fn _MixerBitmap();
    fn SetMixerBitmap();
    fn Set_MixerBitmap();
    fn MixerBitmapPositionRect();
    fn SetMixerBitmapPositionRect();
    fn MixerBitmapOpacity();
    fn SetMixerBitmapOpacity();
    fn SetupMixerBitmap();
    fn SourceSize();
    fn SetSourceSize();
    fn OverScan();
    fn SetOverScan();
    fn AvailableSourceRect();
    fn MaxVidRect();
    fn MinVidRect();
    fn ClippedSourceRect();
    fn SetClippedSourceRect();
    fn UsingOverlay();
    fn SetUsingOverlay();
    fn Capture();
    fn FramesPerSecond();
    fn DecimateInput();
    fn SetDecimateInput();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVideoRenderer2Impl: Sized + IMSVidVideoRendererImpl + IMSVidOutputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Allocator();
    fn _Allocator();
    fn Allocator_ID();
    fn SetAllocator();
    fn _SetAllocator2();
    fn SetSuppressEffects();
    fn SuppressEffects();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVideoRendererDevicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVideoRendererEventImpl: Sized + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn OverlayUnavailable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidVideoRendererEvent2Impl: Sized + IMSVidOutputDeviceEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn OverlayUnavailable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidWebDVDImpl: Sized + IMSVidPlaybackImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn OnDVDEvent();
    fn PlayTitle();
    fn PlayChapterInTitle();
    fn PlayChapter();
    fn PlayChaptersAutoStop();
    fn PlayAtTime();
    fn PlayAtTimeInTitle();
    fn PlayPeriodInTitleAutoStop();
    fn ReplayChapter();
    fn PlayPrevChapter();
    fn PlayNextChapter();
    fn StillOff();
    fn AudioLanguage();
    fn ShowMenu();
    fn Resume();
    fn ReturnFromSubmenu();
    fn ButtonsAvailable();
    fn CurrentButton();
    fn SelectAndActivateButton();
    fn ActivateButton();
    fn SelectRightButton();
    fn SelectLeftButton();
    fn SelectLowerButton();
    fn SelectUpperButton();
    fn ActivateAtPosition();
    fn SelectAtPosition();
    fn ButtonAtPosition();
    fn NumberOfChapters();
    fn TotalTitleTime();
    fn TitlesAvailable();
    fn VolumesAvailable();
    fn CurrentVolume();
    fn CurrentDiscSide();
    fn CurrentDomain();
    fn CurrentChapter();
    fn CurrentTitle();
    fn CurrentTime();
    fn DVDTimeCode2bstr();
    fn DVDDirectory();
    fn SetDVDDirectory();
    fn IsSubpictureStreamEnabled();
    fn IsAudioStreamEnabled();
    fn CurrentSubpictureStream();
    fn SetCurrentSubpictureStream();
    fn SubpictureLanguage();
    fn CurrentAudioStream();
    fn SetCurrentAudioStream();
    fn AudioStreamsAvailable();
    fn AnglesAvailable();
    fn CurrentAngle();
    fn SetCurrentAngle();
    fn SubpictureStreamsAvailable();
    fn SubpictureOn();
    fn SetSubpictureOn();
    fn DVDUniqueID();
    fn AcceptParentalLevelChange();
    fn NotifyParentalLevelChange();
    fn SelectParentalCountry();
    fn SelectParentalLevel();
    fn TitleParentalLevels();
    fn PlayerParentalCountry();
    fn PlayerParentalLevel();
    fn Eject();
    fn UOPValid();
    fn SPRM();
    fn GPRM();
    fn SetGPRM();
    fn DVDTextStringType();
    fn DVDTextString();
    fn DVDTextNumberOfStrings();
    fn DVDTextNumberOfLanguages();
    fn DVDTextLanguageLCID();
    fn RegionChange();
    fn DVDAdm();
    fn DeleteBookmark();
    fn RestoreBookmark();
    fn SaveBookmark();
    fn SelectDefaultAudioLanguage();
    fn SelectDefaultSubpictureLanguage();
    fn PreferredSubpictureStream();
    fn DefaultMenuLanguage();
    fn SetDefaultMenuLanguage();
    fn DefaultSubpictureLanguage();
    fn DefaultAudioLanguage();
    fn DefaultSubpictureLanguageExt();
    fn DefaultAudioLanguageExt();
    fn LanguageFromLCID();
    fn KaraokeAudioPresentationMode();
    fn SetKaraokeAudioPresentationMode();
    fn KaraokeChannelContent();
    fn KaraokeChannelAssignment();
    fn RestorePreferredSettings();
    fn ButtonRect();
    fn DVDScreenInMouseCoordinates();
    fn SetDVDScreenInMouseCoordinates();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidWebDVD2Impl: Sized + IMSVidWebDVDImpl + IMSVidPlaybackImpl + IMSVidInputDeviceImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn Bookmark();
    fn SetBookmark();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidWebDVDAdmImpl: Sized + IDispatchImpl {
    fn ChangePassword();
    fn SaveParentalLevel();
    fn SaveParentalCountry();
    fn ConfirmPassword();
    fn GetParentalLevel();
    fn GetParentalCountry();
    fn DefaultAudioLCID();
    fn SetDefaultAudioLCID();
    fn DefaultSubpictureLCID();
    fn SetDefaultSubpictureLCID();
    fn DefaultMenuLCID();
    fn SetDefaultMenuLCID();
    fn BookmarkOnStop();
    fn SetBookmarkOnStop();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidWebDVDEventImpl: Sized + IMSVidPlaybackEventImpl + IMSVidInputDeviceEventImpl + IDispatchImpl {
    fn DVDNotify();
    fn PlayForwards();
    fn PlayBackwards();
    fn ShowMenu();
    fn Resume();
    fn SelectOrActivateButton();
    fn StillOff();
    fn PauseOn();
    fn ChangeCurrentAudioStream();
    fn ChangeCurrentSubpictureStream();
    fn ChangeCurrentAngle();
    fn PlayAtTimeInTitle();
    fn PlayAtTime();
    fn PlayChapterInTitle();
    fn PlayChapter();
    fn ReplayChapter();
    fn PlayNextChapter();
    fn Stop();
    fn ReturnFromSubmenu();
    fn PlayTitle();
    fn PlayPrevChapter();
    fn ChangeKaraokePresMode();
    fn ChangeVideoPresMode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidXDSImpl: Sized + IMSVidFeatureImpl + IMSVidDeviceImpl + IDispatchImpl {
    fn ChannelChangeInterface();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSVidXDSEventImpl: Sized + IMSVidFeatureEventImpl + IMSVidDeviceEventImpl + IDispatchImpl {
    fn RatingChange();
}
pub trait IMceBurnerControlImpl: Sized {
    fn GetBurnerNoDecryption();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMediaControlImpl: Sized + IDispatchImpl {
    fn Run();
    fn Pause();
    fn Stop();
    fn GetState();
    fn RenderFile();
    fn AddSourceFilter();
    fn FilterCollection();
    fn RegFilterCollection();
    fn StopWhenReady();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMediaEventImpl: Sized + IDispatchImpl {
    fn GetEventHandle();
    fn GetEvent();
    fn WaitForCompletion();
    fn CancelDefaultHandling();
    fn RestoreDefaultHandling();
    fn FreeEventParams();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMediaEventExImpl: Sized + IMediaEventImpl + IDispatchImpl {
    fn SetNotifyWindow();
    fn SetNotifyFlags();
    fn GetNotifyFlags();
}
pub trait IMediaEventSinkImpl: Sized {
    fn Notify();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMediaFilterImpl: Sized + IPersistImpl {
    fn Stop();
    fn Pause();
    fn Run();
    fn GetState();
    fn SetSyncSource();
    fn GetSyncSource();
}
pub trait IMediaParamInfoImpl: Sized {
    fn GetParamCount();
    fn GetParamInfo();
    fn GetParamText();
    fn GetNumTimeFormats();
    fn GetSupportedTimeFormat();
    fn GetCurrentTimeFormat();
}
pub trait IMediaParamsImpl: Sized {
    fn GetParam();
    fn SetParam();
    fn AddEnvelope();
    fn FlushEnvelope();
    fn SetTimeFormat();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMediaPositionImpl: Sized + IDispatchImpl {
    fn Duration();
    fn SetCurrentPosition();
    fn CurrentPosition();
    fn StopTime();
    fn SetStopTime();
    fn PrerollTime();
    fn SetPrerollTime();
    fn SetRate();
    fn Rate();
    fn CanSeekForward();
    fn CanSeekBackward();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IMediaPropertyBagImpl: Sized + IPropertyBagImpl {
    fn EnumProperty();
}
pub trait IMediaSampleImpl: Sized {
    fn GetPointer();
    fn GetSize();
    fn GetTime();
    fn SetTime();
    fn IsSyncPoint();
    fn SetSyncPoint();
    fn IsPreroll();
    fn SetPreroll();
    fn GetActualDataLength();
    fn SetActualDataLength();
    fn GetMediaType();
    fn SetMediaType();
    fn IsDiscontinuity();
    fn SetDiscontinuity();
    fn GetMediaTime();
    fn SetMediaTime();
}
pub trait IMediaSample2Impl: Sized + IMediaSampleImpl {
    fn GetProperties();
    fn SetProperties();
}
pub trait IMediaSample2ConfigImpl: Sized {
    fn GetSurface();
}
pub trait IMediaSeekingImpl: Sized {
    fn GetCapabilities();
    fn CheckCapabilities();
    fn IsFormatSupported();
    fn QueryPreferredFormat();
    fn GetTimeFormat();
    fn IsUsingTimeFormat();
    fn SetTimeFormat();
    fn GetDuration();
    fn GetStopPosition();
    fn GetCurrentPosition();
    fn ConvertTimeFormat();
    fn SetPositions();
    fn GetPositions();
    fn GetAvailable();
    fn SetRate();
    fn GetRate();
    fn GetPreroll();
}
pub trait IMediaStreamImpl: Sized {
    fn GetMultiMediaStream();
    fn GetInformation();
    fn SetSameFormat();
    fn AllocateSample();
    fn CreateSharedSample();
    fn SendEndOfStream();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMediaStreamFilterImpl: Sized + IBaseFilterImpl + IMediaFilterImpl + IPersistImpl {
    fn AddMediaStream();
    fn GetMediaStream();
    fn EnumMediaStreams();
    fn SupportSeeking();
    fn ReferenceTimeToStreamTime();
    fn GetCurrentStreamTime();
    fn WaitUntil();
    fn Flush();
    fn EndOfStream();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMediaTypeInfoImpl: Sized + IDispatchImpl {
    fn Type();
    fn Subtype();
}
pub trait IMemAllocatorImpl: Sized {
    fn SetProperties();
    fn GetProperties();
    fn Commit();
    fn Decommit();
    fn GetBuffer();
    fn ReleaseBuffer();
}
pub trait IMemAllocatorCallbackTempImpl: Sized + IMemAllocatorImpl {
    fn SetNotify();
    fn GetFreeCount();
}
pub trait IMemAllocatorNotifyCallbackTempImpl: Sized {
    fn NotifyRelease();
}
pub trait IMemInputPinImpl: Sized {
    fn GetAllocator();
    fn NotifyAllocator();
    fn GetAllocatorRequirements();
    fn Receive();
    fn ReceiveMultiple();
    fn ReceiveCanBlock();
}
pub trait IMemoryDataImpl: Sized {
    fn SetBuffer();
    fn GetInfo();
    fn SetActual();
}
pub trait IMixerOCXImpl: Sized {
    fn OnDisplayChange();
    fn GetAspectRatio();
    fn GetVideoSize();
    fn GetStatus();
    fn OnDraw();
    fn SetDrawRegion();
    fn Advise();
    fn UnAdvise();
}
pub trait IMixerOCXNotifyImpl: Sized {
    fn OnInvalidateRect();
    fn OnStatusChange();
    fn OnDataChange();
}
pub trait IMixerPinConfigImpl: Sized {
    fn SetRelativePosition();
    fn GetRelativePosition();
    fn SetZOrder();
    fn GetZOrder();
    fn SetColorKey();
    fn GetColorKey();
    fn SetBlendingParameter();
    fn GetBlendingParameter();
    fn SetAspectRatioMode();
    fn GetAspectRatioMode();
    fn SetStreamTransparent();
    fn GetStreamTransparent();
}
pub trait IMixerPinConfig2Impl: Sized + IMixerPinConfigImpl {
    fn SetOverlaySurfaceColorControls();
    fn GetOverlaySurfaceColorControls();
}
pub trait IMpeg2DataImpl: Sized {
    fn GetSection();
    fn GetTable();
    fn GetStreamOfSections();
}
pub trait IMpeg2DemultiplexerImpl: Sized {
    fn CreateOutputPin();
    fn SetOutputPinMediaType();
    fn DeleteOutputPin();
}
pub trait IMpeg2StreamImpl: Sized {
    fn Initialize();
    fn SupplyDataBuffer();
}
pub trait IMpeg2TableFilterImpl: Sized {
    fn AddPID();
    fn AddTable();
    fn AddExtension();
    fn RemovePID();
    fn RemoveTable();
    fn RemoveExtension();
}
pub trait IMpegAudioDecoderImpl: Sized {
    fn FrequencyDivider();
    fn SetFrequencyDivider();
    fn DecoderAccuracy();
    fn SetDecoderAccuracy();
    fn Stereo();
    fn SetStereo();
    fn DecoderWordSize();
    fn SetDecoderWordSize();
    fn IntegerDecode();
    fn SetIntegerDecode();
    fn DualMode();
    fn SetDualMode();
    fn AudioFormat();
}
pub trait IMultiMediaStreamImpl: Sized {
    fn GetInformation();
    fn GetMediaStream();
    fn EnumMediaStreams();
    fn GetState();
    fn SetState();
    fn GetTime();
    fn GetDuration();
    fn Seek();
    fn GetEndOfStreamEventHandle();
}
pub trait IOverlayImpl: Sized {
    fn GetPalette();
    fn SetPalette();
    fn GetDefaultColorKey();
    fn GetColorKey();
    fn SetColorKey();
    fn GetWindowHandle();
    fn GetClipList();
    fn GetVideoPosition();
    fn Advise();
    fn Unadvise();
}
pub trait IOverlayNotifyImpl: Sized {
    fn OnPaletteChange();
    fn OnClipChange();
    fn OnColorKeyChange();
    fn OnPositionChange();
}
pub trait IOverlayNotify2Impl: Sized + IOverlayNotifyImpl {
    fn OnDisplayChange();
}
pub trait IPATImpl: Sized {
    fn Initialize();
    fn GetTransportStreamId();
    fn GetVersionNumber();
    fn GetCountOfRecords();
    fn GetRecordProgramNumber();
    fn GetRecordProgramMapPid();
    fn FindRecordProgramMapPid();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
}
pub trait IPBDAAttributesDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetAttributePayload();
}
pub trait IPBDAEntitlementDescriptorImpl: Sized {
    fn GetTag();
    fn GetLength();
    fn GetToken();
}
pub trait IPBDASiParserImpl: Sized {
    fn Initialize();
    fn GetEIT();
    fn GetServices();
}
pub trait IPBDA_EITImpl: Sized {
    fn Initialize();
    fn GetTableId();
    fn GetVersionNumber();
    fn GetServiceIdx();
    fn GetCountOfRecords();
    fn GetRecordEventId();
    fn GetRecordStartTime();
    fn GetRecordDuration();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
}
pub trait IPBDA_ServicesImpl: Sized {
    fn Initialize();
    fn GetCountOfRecords();
    fn GetRecordByIndex();
}
pub trait IPMTImpl: Sized {
    fn Initialize();
    fn GetProgramNumber();
    fn GetVersionNumber();
    fn GetPcrPid();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn GetCountOfRecords();
    fn GetRecordStreamType();
    fn GetRecordElementaryPid();
    fn GetRecordCountOfDescriptors();
    fn GetRecordDescriptorByIndex();
    fn GetRecordDescriptorByTag();
    fn QueryServiceGatewayInfo();
    fn QueryMPEInfo();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
}
pub trait IPSITablesImpl: Sized {
    fn GetTable();
}
pub trait IPTFilterLicenseRenewalImpl: Sized {
    fn RenewLicenses();
    fn CancelLicenseRenewal();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistMediaPropertyBagImpl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistTuneXmlImpl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
}
pub trait IPersistTuneXmlUtilityImpl: Sized {
    fn Deserialize();
}
pub trait IPersistTuneXmlUtility2Impl: Sized + IPersistTuneXmlUtilityImpl {
    fn Serialize();
}
pub trait IPinImpl: Sized {
    fn Connect();
    fn ReceiveConnection();
    fn Disconnect();
    fn ConnectedTo();
    fn ConnectionMediaType();
    fn QueryPinInfo();
    fn QueryDirection();
    fn QueryId();
    fn QueryAccept();
    fn EnumMediaTypes();
    fn QueryInternalConnections();
    fn EndOfStream();
    fn BeginFlush();
    fn EndFlush();
    fn NewSegment();
}
pub trait IPinConnectionImpl: Sized {
    fn DynamicQueryAccept();
    fn NotifyEndOfStream();
    fn IsEndPin();
    fn DynamicDisconnect();
}
pub trait IPinFlowControlImpl: Sized {
    fn Block();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPinInfoImpl: Sized + IDispatchImpl {
    fn Pin();
    fn ConnectedTo();
    fn ConnectionMediaType();
    fn FilterInfo();
    fn Name();
    fn Direction();
    fn PinID();
    fn MediaTypes();
    fn Connect();
    fn ConnectDirect();
    fn ConnectWithType();
    fn Disconnect();
    fn Render();
}
pub trait IQualPropImpl: Sized {
    fn FramesDroppedInRenderer();
    fn FramesDrawn();
    fn AvgFrameRate();
    fn Jitter();
    fn AvgSyncOffset();
    fn DevSyncOffset();
}
pub trait IQualityControlImpl: Sized {
    fn Notify();
    fn SetSink();
}
pub trait IQueueCommandImpl: Sized {
    fn InvokeAtStreamTime();
    fn InvokeAtPresentationTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRegFilterInfoImpl: Sized + IDispatchImpl {
    fn Name();
    fn Filter();
}
pub trait IRegisterServiceProviderImpl: Sized {
    fn RegisterService();
}
pub trait IRegisterTunerImpl: Sized {
    fn Register();
    fn Unregister();
}
pub trait IResourceConsumerImpl: Sized {
    fn AcquireResource();
    fn ReleaseResource();
}
pub trait IResourceManagerImpl: Sized {
    fn Register();
    fn RegisterGroup();
    fn RequestResource();
    fn NotifyAcquire();
    fn NotifyRelease();
    fn CancelRequest();
    fn SetFocus();
    fn ReleaseFocus();
}
pub trait ISBE2CrossbarImpl: Sized {
    fn EnableDefaultMode();
    fn GetInitialProfile();
    fn SetOutputProfile();
    fn EnumStreams();
}
pub trait ISBE2EnumStreamImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait ISBE2FileScanImpl: Sized {
    fn RepairFile();
}
pub trait ISBE2GlobalEventImpl: Sized {
    fn GetEvent();
}
pub trait ISBE2GlobalEvent2Impl: Sized + ISBE2GlobalEventImpl {
    fn GetEventEx();
}
pub trait ISBE2MediaTypeProfileImpl: Sized {
    fn GetStreamCount();
    fn GetStream();
    fn AddStream();
    fn DeleteStream();
}
pub trait ISBE2SpanningEventImpl: Sized {
    fn GetEvent();
}
pub trait ISBE2StreamMapImpl: Sized {
    fn MapStream();
    fn UnmapStream();
    fn EnumMappedStreams();
}
pub trait ISCTE_EASImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetSequencyNumber();
    fn GetProtocolVersion();
    fn GetEASEventID();
    fn GetOriginatorCode();
    fn GetEASEventCodeLen();
    fn GetEASEventCode();
    fn GetRawNatureOfActivationTextLen();
    fn GetRawNatureOfActivationText();
    fn GetNatureOfActivationText();
    fn GetTimeRemaining();
    fn GetStartTime();
    fn GetDuration();
    fn GetAlertPriority();
    fn GetDetailsOOBSourceID();
    fn GetDetailsMajor();
    fn GetDetailsMinor();
    fn GetDetailsAudioOOBSourceID();
    fn GetAlertText();
    fn GetRawAlertTextLen();
    fn GetRawAlertText();
    fn GetLocationCount();
    fn GetLocationCodes();
    fn GetExceptionCount();
    fn GetExceptionService();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
}
pub trait ISIInbandEPGImpl: Sized {
    fn StartSIEPGScan();
    fn StopSIEPGScan();
    fn IsSIEPGScanRunning();
}
pub trait ISIInbandEPGEventImpl: Sized {
    fn SIObjectEvent();
}
pub trait IScanningTunerImpl: Sized + ITunerImpl {
    fn SeekUp();
    fn SeekDown();
    fn ScanUp();
    fn ScanDown();
    fn AutoProgram();
}
pub trait IScanningTunerExImpl: Sized + IScanningTunerImpl + ITunerImpl {
    fn GetCurrentLocator();
    fn PerformExhaustiveScan();
    fn TerminateCurrentScan();
    fn ResumeCurrentScan();
    fn GetTunerScanningCapability();
    fn GetTunerStatus();
    fn GetCurrentTunerStandardCapability();
    fn SetScanSignalTypeFilter();
}
pub trait ISectionListImpl: Sized {
    fn Initialize();
    fn InitializeWithRawSections();
    fn CancelPendingRequest();
    fn GetNumberOfSections();
    fn GetSectionData();
    fn GetProgramIdentifier();
    fn GetTableIdentifier();
}
pub trait ISeekingPassThruImpl: Sized {
    fn Init();
}
pub trait ISelectorImpl: Sized {
    fn NumSources();
    fn SourceNodeId();
    fn SetSourceNodeId();
}
pub trait IServiceLocationDescriptorImpl: Sized {
    fn GetPCR_PID();
    fn GetNumberOfElements();
    fn GetElementStreamType();
    fn GetElementPID();
    fn GetElementLanguageCode();
}
pub trait ISpecifyParticularPagesImpl: Sized {
    fn GetPages();
}
pub trait IStreamBufferConfigureImpl: Sized {
    fn SetDirectory();
    fn GetDirectory();
    fn SetBackingFileCount();
    fn GetBackingFileCount();
    fn SetBackingFileDuration();
    fn GetBackingFileDuration();
}
pub trait IStreamBufferConfigure2Impl: Sized + IStreamBufferConfigureImpl {
    fn SetMultiplexedPacketSize();
    fn GetMultiplexedPacketSize();
    fn SetFFTransitionRates();
    fn GetFFTransitionRates();
}
pub trait IStreamBufferConfigure3Impl: Sized + IStreamBufferConfigure2Impl + IStreamBufferConfigureImpl {
    fn SetStartRecConfig();
    fn GetStartRecConfig();
    fn SetNamespace();
    fn GetNamespace();
}
pub trait IStreamBufferDataCountersImpl: Sized {
    fn GetData();
    fn ResetData();
}
pub trait IStreamBufferInitializeImpl: Sized {
    fn SetHKEY();
    fn SetSIDs();
}
pub trait IStreamBufferMediaSeekingImpl: Sized + IMediaSeekingImpl {}
pub trait IStreamBufferMediaSeeking2Impl: Sized + IStreamBufferMediaSeekingImpl + IMediaSeekingImpl {
    fn SetRateEx();
}
pub trait IStreamBufferRecCompImpl: Sized {
    fn Initialize();
    fn Append();
    fn AppendEx();
    fn GetCurrentLength();
    fn Close();
    fn Cancel();
}
pub trait IStreamBufferRecordControlImpl: Sized {
    fn Start();
    fn Stop();
    fn GetRecordingStatus();
}
pub trait IStreamBufferRecordingAttributeImpl: Sized {
    fn SetAttribute();
    fn GetAttributeCount();
    fn GetAttributeByName();
    fn GetAttributeByIndex();
    fn EnumAttributes();
}
pub trait IStreamBufferSinkImpl: Sized {
    fn LockProfile();
    fn CreateRecorder();
    fn IsProfileLocked();
}
pub trait IStreamBufferSink2Impl: Sized + IStreamBufferSinkImpl {
    fn UnlockProfile();
}
pub trait IStreamBufferSink3Impl: Sized + IStreamBufferSink2Impl + IStreamBufferSinkImpl {
    fn SetAvailableFilter();
}
pub trait IStreamBufferSourceImpl: Sized {
    fn SetStreamSink();
}
pub trait IStreamBuilderImpl: Sized {
    fn Render();
    fn Backout();
}
pub trait IStreamSampleImpl: Sized {
    fn GetMediaStream();
    fn GetSampleTimes();
    fn SetSampleTimes();
    fn Update();
    fn CompletionStatus();
}
pub trait ITSDTImpl: Sized {
    fn Initialize();
    fn GetVersionNumber();
    fn GetCountOfTableDescriptors();
    fn GetTableDescriptorByIndex();
    fn GetTableDescriptorByTag();
    fn RegisterForNextTable();
    fn GetNextTable();
    fn RegisterForWhenCurrent();
    fn ConvertNextToCurrent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITuneRequestImpl: Sized + IDispatchImpl {
    fn TuningSpace();
    fn Components();
    fn Clone();
    fn Locator();
    fn SetLocator();
}
pub trait ITuneRequestInfoImpl: Sized {
    fn GetLocatorData();
    fn GetComponentData();
    fn CreateComponentList();
    fn GetNextProgram();
    fn GetPreviousProgram();
    fn GetNextLocator();
    fn GetPreviousLocator();
}
pub trait ITuneRequestInfoExImpl: Sized + ITuneRequestInfoImpl {
    fn CreateComponentListEx();
}
pub trait ITunerImpl: Sized {
    fn TuningSpace();
    fn SetTuningSpace();
    fn EnumTuningSpaces();
    fn TuneRequest();
    fn SetTuneRequest();
    fn Validate();
    fn PreferredComponentTypes();
    fn SetPreferredComponentTypes();
    fn SignalStrength();
    fn TriggerSignalEvents();
}
pub trait ITunerCapImpl: Sized {
    fn SupportedNetworkTypes();
    fn SupportedVideoFormats();
    fn AuxInputCount();
}
pub trait ITunerCapExImpl: Sized {
    fn Has608_708Caption();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITuningSpaceImpl: Sized + IDispatchImpl {
    fn UniqueName();
    fn SetUniqueName();
    fn FriendlyName();
    fn SetFriendlyName();
    fn CLSID();
    fn NetworkType();
    fn SetNetworkType();
    fn _NetworkType();
    fn Set_NetworkType();
    fn CreateTuneRequest();
    fn EnumCategoryGUIDs();
    fn EnumDeviceMonikers();
    fn DefaultPreferredComponentTypes();
    fn SetDefaultPreferredComponentTypes();
    fn FrequencyMapping();
    fn SetFrequencyMapping();
    fn DefaultLocator();
    fn SetDefaultLocator();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITuningSpaceContainerImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn SetItem();
    fn TuningSpacesForCLSID();
    fn _TuningSpacesForCLSID2();
    fn TuningSpacesForName();
    fn FindID();
    fn Add();
    fn EnumTuningSpaces();
    fn Remove();
    fn MaxCount();
    fn SetMaxCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITuningSpacesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn EnumTuningSpaces();
}
pub trait IVMRAspectRatioControlImpl: Sized {
    fn GetAspectRatioMode();
    fn SetAspectRatioMode();
}
pub trait IVMRAspectRatioControl9Impl: Sized {
    fn GetAspectRatioMode();
    fn SetAspectRatioMode();
}
pub trait IVMRDeinterlaceControlImpl: Sized {
    fn GetNumberOfDeinterlaceModes();
    fn GetDeinterlaceModeCaps();
    fn GetDeinterlaceMode();
    fn SetDeinterlaceMode();
    fn GetDeinterlacePrefs();
    fn SetDeinterlacePrefs();
    fn GetActualDeinterlaceMode();
}
pub trait IVMRDeinterlaceControl9Impl: Sized {
    fn GetNumberOfDeinterlaceModes();
    fn GetDeinterlaceModeCaps();
    fn GetDeinterlaceMode();
    fn SetDeinterlaceMode();
    fn GetDeinterlacePrefs();
    fn SetDeinterlacePrefs();
    fn GetActualDeinterlaceMode();
}
pub trait IVMRFilterConfigImpl: Sized {
    fn SetImageCompositor();
    fn SetNumberOfStreams();
    fn GetNumberOfStreams();
    fn SetRenderingPrefs();
    fn GetRenderingPrefs();
    fn SetRenderingMode();
    fn GetRenderingMode();
}
pub trait IVMRFilterConfig9Impl: Sized {
    fn SetImageCompositor();
    fn SetNumberOfStreams();
    fn GetNumberOfStreams();
    fn SetRenderingPrefs();
    fn GetRenderingPrefs();
    fn SetRenderingMode();
    fn GetRenderingMode();
}
pub trait IVMRImageCompositorImpl: Sized {
    fn InitCompositionTarget();
    fn TermCompositionTarget();
    fn SetStreamMediaType();
    fn CompositeImage();
}
pub trait IVMRImageCompositor9Impl: Sized {
    fn InitCompositionDevice();
    fn TermCompositionDevice();
    fn SetStreamMediaType();
    fn CompositeImage();
}
pub trait IVMRImagePresenterImpl: Sized {
    fn StartPresenting();
    fn StopPresenting();
    fn PresentImage();
}
pub trait IVMRImagePresenter9Impl: Sized {
    fn StartPresenting();
    fn StopPresenting();
    fn PresentImage();
}
pub trait IVMRImagePresenterConfigImpl: Sized {
    fn SetRenderingPrefs();
    fn GetRenderingPrefs();
}
pub trait IVMRImagePresenterConfig9Impl: Sized {
    fn SetRenderingPrefs();
    fn GetRenderingPrefs();
}
pub trait IVMRImagePresenterExclModeConfigImpl: Sized + IVMRImagePresenterConfigImpl {
    fn SetXlcModeDDObjAndPrimarySurface();
    fn GetXlcModeDDObjAndPrimarySurface();
}
pub trait IVMRMixerBitmapImpl: Sized {
    fn SetAlphaBitmap();
    fn UpdateAlphaBitmapParameters();
    fn GetAlphaBitmapParameters();
}
pub trait IVMRMixerBitmap9Impl: Sized {
    fn SetAlphaBitmap();
    fn UpdateAlphaBitmapParameters();
    fn GetAlphaBitmapParameters();
}
pub trait IVMRMixerControlImpl: Sized {
    fn SetAlpha();
    fn GetAlpha();
    fn SetZOrder();
    fn GetZOrder();
    fn SetOutputRect();
    fn GetOutputRect();
    fn SetBackgroundClr();
    fn GetBackgroundClr();
    fn SetMixingPrefs();
    fn GetMixingPrefs();
}
pub trait IVMRMixerControl9Impl: Sized {
    fn SetAlpha();
    fn GetAlpha();
    fn SetZOrder();
    fn GetZOrder();
    fn SetOutputRect();
    fn GetOutputRect();
    fn SetBackgroundClr();
    fn GetBackgroundClr();
    fn SetMixingPrefs();
    fn GetMixingPrefs();
    fn SetProcAmpControl();
    fn GetProcAmpControl();
    fn GetProcAmpControlRange();
}
pub trait IVMRMonitorConfigImpl: Sized {
    fn SetMonitor();
    fn GetMonitor();
    fn SetDefaultMonitor();
    fn GetDefaultMonitor();
    fn GetAvailableMonitors();
}
pub trait IVMRMonitorConfig9Impl: Sized {
    fn SetMonitor();
    fn GetMonitor();
    fn SetDefaultMonitor();
    fn GetDefaultMonitor();
    fn GetAvailableMonitors();
}
pub trait IVMRSurfaceImpl: Sized {
    fn IsSurfaceLocked();
    fn LockSurface();
    fn UnlockSurface();
    fn GetSurface();
}
pub trait IVMRSurface9Impl: Sized {
    fn IsSurfaceLocked();
    fn LockSurface();
    fn UnlockSurface();
    fn GetSurface();
}
pub trait IVMRSurfaceAllocatorImpl: Sized {
    fn AllocateSurface();
    fn FreeSurface();
    fn PrepareSurface();
    fn AdviseNotify();
}
pub trait IVMRSurfaceAllocator9Impl: Sized {
    fn InitializeDevice();
    fn TerminateDevice();
    fn GetSurface();
    fn AdviseNotify();
}
pub trait IVMRSurfaceAllocatorEx9Impl: Sized + IVMRSurfaceAllocator9Impl {
    fn GetSurfaceEx();
}
pub trait IVMRSurfaceAllocatorNotifyImpl: Sized {
    fn AdviseSurfaceAllocator();
    fn SetDDrawDevice();
    fn ChangeDDrawDevice();
    fn RestoreDDrawSurfaces();
    fn NotifyEvent();
    fn SetBorderColor();
}
pub trait IVMRSurfaceAllocatorNotify9Impl: Sized {
    fn AdviseSurfaceAllocator();
    fn SetD3DDevice();
    fn ChangeD3DDevice();
    fn AllocateSurfaceHelper();
    fn NotifyEvent();
}
pub trait IVMRVideoStreamControlImpl: Sized {
    fn SetColorKey();
    fn GetColorKey();
    fn SetStreamActiveState();
    fn GetStreamActiveState();
}
pub trait IVMRVideoStreamControl9Impl: Sized {
    fn SetStreamActiveState();
    fn GetStreamActiveState();
}
pub trait IVMRWindowlessControlImpl: Sized {
    fn GetNativeVideoSize();
    fn GetMinIdealVideoSize();
    fn GetMaxIdealVideoSize();
    fn SetVideoPosition();
    fn GetVideoPosition();
    fn GetAspectRatioMode();
    fn SetAspectRatioMode();
    fn SetVideoClippingWindow();
    fn RepaintVideo();
    fn DisplayModeChanged();
    fn GetCurrentImage();
    fn SetBorderColor();
    fn GetBorderColor();
    fn SetColorKey();
    fn GetColorKey();
}
pub trait IVMRWindowlessControl9Impl: Sized {
    fn GetNativeVideoSize();
    fn GetMinIdealVideoSize();
    fn GetMaxIdealVideoSize();
    fn SetVideoPosition();
    fn GetVideoPosition();
    fn GetAspectRatioMode();
    fn SetAspectRatioMode();
    fn SetVideoClippingWindow();
    fn RepaintVideo();
    fn DisplayModeChanged();
    fn GetCurrentImage();
    fn SetBorderColor();
    fn GetBorderColor();
}
pub trait IVPBaseConfigImpl: Sized {
    fn GetConnectInfo();
    fn SetConnectInfo();
    fn GetVPDataInfo();
    fn GetMaxPixelRate();
    fn InformVPInputFormats();
    fn GetVideoFormats();
    fn SetVideoFormat();
    fn SetInvertPolarity();
    fn GetOverlaySurface();
    fn SetDirectDrawKernelHandle();
    fn SetVideoPortID();
    fn SetDDSurfaceKernelHandles();
    fn SetSurfaceParameters();
}
pub trait IVPBaseNotifyImpl: Sized {
    fn RenegotiateVPParameters();
}
pub trait IVPConfigImpl: Sized + IVPBaseConfigImpl {
    fn IsVPDecimationAllowed();
    fn SetScalingFactors();
}
pub trait IVPManagerImpl: Sized {
    fn SetVideoPortIndex();
    fn GetVideoPortIndex();
}
pub trait IVPNotifyImpl: Sized + IVPBaseNotifyImpl {
    fn SetDeinterlaceMode();
    fn GetDeinterlaceMode();
}
pub trait IVPNotify2Impl: Sized + IVPNotifyImpl + IVPBaseNotifyImpl {
    fn SetVPSyncMaster();
    fn GetVPSyncMaster();
}
pub trait IVPVBIConfigImpl: Sized + IVPBaseConfigImpl {}
pub trait IVPVBINotifyImpl: Sized + IVPBaseNotifyImpl {}
pub trait IVideoEncoderImpl: Sized + IEncoderAPIImpl {}
pub trait IVideoFrameStepImpl: Sized {
    fn Step();
    fn CanStep();
    fn CancelStep();
}
pub trait IVideoProcAmpImpl: Sized {
    fn BacklightCompensation();
    fn SetBacklightCompensation();
    fn getRange_BacklightCompensation();
    fn Brightness();
    fn SetBrightness();
    fn getRange_Brightness();
    fn ColorEnable();
    fn SetColorEnable();
    fn getRange_ColorEnable();
    fn Contrast();
    fn SetContrast();
    fn getRange_Contrast();
    fn Gamma();
    fn SetGamma();
    fn getRange_Gamma();
    fn Saturation();
    fn SetSaturation();
    fn getRange_Saturation();
    fn Sharpness();
    fn SetSharpness();
    fn getRange_Sharpness();
    fn WhiteBalance();
    fn SetWhiteBalance();
    fn getRange_WhiteBalance();
    fn Gain();
    fn SetGain();
    fn getRange_Gain();
    fn Hue();
    fn SetHue();
    fn getRange_Hue();
    fn DigitalMultiplier();
    fn SetDigitalMultiplier();
    fn getRange_DigitalMultiplier();
    fn PowerlineFrequency();
    fn SetPowerlineFrequency();
    fn getRange_PowerlineFrequency();
    fn WhiteBalanceComponent();
    fn SetWhiteBalanceComponent();
    fn getRange_WhiteBalanceComponent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVideoWindowImpl: Sized + IDispatchImpl {
    fn SetCaption();
    fn Caption();
    fn SetWindowStyle();
    fn WindowStyle();
    fn SetWindowStyleEx();
    fn WindowStyleEx();
    fn SetAutoShow();
    fn AutoShow();
    fn SetWindowState();
    fn WindowState();
    fn SetBackgroundPalette();
    fn BackgroundPalette();
    fn SetVisible();
    fn Visible();
    fn SetLeft();
    fn Left();
    fn SetWidth();
    fn Width();
    fn SetTop();
    fn Top();
    fn SetHeight();
    fn Height();
    fn SetOwner();
    fn Owner();
    fn SetMessageDrain();
    fn MessageDrain();
    fn BorderColor();
    fn SetBorderColor();
    fn FullScreenMode();
    fn SetFullScreenMode();
    fn SetWindowForeground();
    fn NotifyOwnerMessage();
    fn SetWindowPosition();
    fn GetWindowPosition();
    fn GetMinIdealImageSize();
    fn GetMaxIdealImageSize();
    fn GetRestorePosition();
    fn HideCursor();
    fn IsCursorHidden();
}
pub trait IXDSCodecImpl: Sized {
    fn XDSToRatObjOK();
    fn SetCCSubstreamService();
    fn CCSubstreamService();
    fn GetContentAdvisoryRating();
    fn GetXDSPacket();
    fn GetCurrLicenseExpDate();
    fn GetLastErrorCode();
}
pub trait IXDSCodecConfigImpl: Sized {
    fn GetSecureChannelObject();
    fn SetPauseBufferTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXDSCodecEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IXDSToRatImpl: Sized + IDispatchImpl {
    fn Init();
    fn ParseXDSBytePair();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IMSVidCtlEventsImpl: Sized + IDispatchImpl {}
