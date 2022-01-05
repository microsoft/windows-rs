pub trait IAMWMBufferPassImpl: Sized {
    fn SetNotify();
}
pub trait IAMWMBufferPassCallbackImpl: Sized {
    fn Notify();
}
pub trait INSNetSourceCreatorImpl: Sized {
    fn Initialize();
    fn CreateNetSource();
    fn GetNetSourceProperties();
    fn GetNetSourceSharedNamespace();
    fn GetNetSourceAdminInterface();
    fn GetNumProtocolsSupported();
    fn GetProtocolName();
    fn Shutdown();
}
pub trait INSSBufferImpl: Sized {
    fn GetLength();
    fn SetLength();
    fn GetMaxLength();
    fn GetBuffer();
    fn GetBufferAndLength();
}
pub trait INSSBuffer2Impl: Sized + INSSBufferImpl {
    fn GetSampleProperties();
    fn SetSampleProperties();
}
pub trait INSSBuffer3Impl: Sized + INSSBuffer2Impl + INSSBufferImpl {
    fn SetProperty();
    fn GetProperty();
}
pub trait INSSBuffer4Impl: Sized + INSSBuffer3Impl + INSSBuffer2Impl + INSSBufferImpl {
    fn GetPropertyCount();
    fn GetPropertyByIndex();
}
pub trait IWMAddressAccessImpl: Sized {
    fn GetAccessEntryCount();
    fn GetAccessEntry();
    fn AddAccessEntry();
    fn RemoveAccessEntry();
}
pub trait IWMAddressAccess2Impl: Sized + IWMAddressAccessImpl {
    fn GetAccessEntryEx();
    fn AddAccessEntryEx();
}
pub trait IWMAuthorizerImpl: Sized {
    fn GetCertCount();
    fn GetCert();
    fn GetSharedData();
}
pub trait IWMBackupRestorePropsImpl: Sized {
    fn GetPropCount();
    fn GetPropByIndex();
    fn GetPropByName();
    fn SetProp();
    fn RemoveProp();
    fn RemoveAllProps();
}
pub trait IWMBandwidthSharingImpl: Sized + IWMStreamListImpl {
    fn GetType();
    fn SetType();
    fn GetBandwidth();
    fn SetBandwidth();
}
pub trait IWMClientConnectionsImpl: Sized {
    fn GetClientCount();
    fn GetClientProperties();
}
pub trait IWMClientConnections2Impl: Sized + IWMClientConnectionsImpl {
    fn GetClientInfo();
}
pub trait IWMCodecAMVideoAcceleratorImpl: Sized {
    fn SetAcceleratorInterface();
    fn NegotiateConnection();
    fn SetPlayerNotify();
}
pub trait IWMCodecInfoImpl: Sized {
    fn GetCodecInfoCount();
    fn GetCodecFormatCount();
    fn GetCodecFormat();
}
pub trait IWMCodecInfo2Impl: Sized + IWMCodecInfoImpl {
    fn GetCodecName();
    fn GetCodecFormatDesc();
}
pub trait IWMCodecInfo3Impl: Sized + IWMCodecInfo2Impl + IWMCodecInfoImpl {
    fn GetCodecFormatProp();
    fn GetCodecProp();
    fn SetCodecEnumerationSetting();
    fn GetCodecEnumerationSetting();
}
pub trait IWMCodecVideoAcceleratorImpl: Sized {
    fn NegotiateConnection();
    fn SetPlayerNotify();
}
pub trait IWMCredentialCallbackImpl: Sized {
    fn AcquireCredentials();
}
pub trait IWMDRMEditorImpl: Sized {
    fn GetDRMProperty();
}
pub trait IWMDRMMessageParserImpl: Sized {
    fn ParseRegistrationReqMsg();
    fn ParseLicenseRequestMsg();
}
pub trait IWMDRMReaderImpl: Sized {
    fn AcquireLicense();
    fn CancelLicenseAcquisition();
    fn Individualize();
    fn CancelIndividualization();
    fn MonitorLicenseAcquisition();
    fn CancelMonitorLicenseAcquisition();
    fn SetDRMProperty();
    fn GetDRMProperty();
}
pub trait IWMDRMReader2Impl: Sized + IWMDRMReaderImpl {
    fn SetEvaluateOutputLevelLicenses();
    fn GetPlayOutputLevels();
    fn GetCopyOutputLevels();
    fn TryNextLicense();
}
pub trait IWMDRMReader3Impl: Sized + IWMDRMReader2Impl + IWMDRMReaderImpl {
    fn GetInclusionList();
}
pub trait IWMDRMTranscryptionManagerImpl: Sized {
    fn CreateTranscryptor();
}
pub trait IWMDRMTranscryptorImpl: Sized {
    fn Initialize();
    fn Seek();
    fn Read();
    fn Close();
}
pub trait IWMDRMTranscryptor2Impl: Sized + IWMDRMTranscryptorImpl {
    fn SeekEx();
    fn ZeroAdjustTimestamps();
    fn GetSeekStartTime();
    fn GetDuration();
}
pub trait IWMDRMWriterImpl: Sized {
    fn GenerateKeySeed();
    fn GenerateKeyID();
    fn GenerateSigningKeyPair();
    fn SetDRMAttribute();
}
pub trait IWMDRMWriter2Impl: Sized + IWMDRMWriterImpl {
    fn SetWMDRMNetEncryption();
}
pub trait IWMDRMWriter3Impl: Sized + IWMDRMWriter2Impl + IWMDRMWriterImpl {
    fn SetProtectStreamSamples();
}
pub trait IWMDeviceRegistrationImpl: Sized {
    fn RegisterDevice();
    fn UnregisterDevice();
    fn GetRegistrationStats();
    fn GetFirstRegisteredDevice();
    fn GetNextRegisteredDevice();
    fn GetRegisteredDeviceByID();
}
pub trait IWMGetSecureChannelImpl: Sized {
    fn GetPeerSecureChannelInterface();
}
pub trait IWMHeaderInfoImpl: Sized {
    fn GetAttributeCount();
    fn GetAttributeByIndex();
    fn GetAttributeByName();
    fn SetAttribute();
    fn GetMarkerCount();
    fn GetMarker();
    fn AddMarker();
    fn RemoveMarker();
    fn GetScriptCount();
    fn GetScript();
    fn AddScript();
    fn RemoveScript();
}
pub trait IWMHeaderInfo2Impl: Sized + IWMHeaderInfoImpl {
    fn GetCodecInfoCount();
    fn GetCodecInfo();
}
pub trait IWMHeaderInfo3Impl: Sized + IWMHeaderInfo2Impl + IWMHeaderInfoImpl {
    fn GetAttributeCountEx();
    fn GetAttributeIndices();
    fn GetAttributeByIndexEx();
    fn ModifyAttribute();
    fn AddAttribute();
    fn DeleteAttribute();
    fn AddCodecInfo();
}
pub trait IWMIStreamPropsImpl: Sized {
    fn GetProperty();
}
pub trait IWMImageInfoImpl: Sized {
    fn GetImageCount();
    fn GetImage();
}
pub trait IWMIndexerImpl: Sized {
    fn StartIndexing();
    fn Cancel();
}
pub trait IWMIndexer2Impl: Sized + IWMIndexerImpl {
    fn Configure();
}
pub trait IWMInputMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetConnectionName();
    fn GetGroupName();
}
pub trait IWMLanguageListImpl: Sized {
    fn GetLanguageCount();
    fn GetLanguageDetails();
    fn AddLanguageByRFC1766String();
}
pub trait IWMLicenseBackupImpl: Sized {
    fn BackupLicenses();
    fn CancelLicenseBackup();
}
pub trait IWMLicenseRestoreImpl: Sized {
    fn RestoreLicenses();
    fn CancelLicenseRestore();
}
pub trait IWMLicenseRevocationAgentImpl: Sized {
    fn GetLRBChallenge();
    fn ProcessLRB();
}
pub trait IWMMediaPropsImpl: Sized {
    fn GetType();
    fn GetMediaType();
    fn SetMediaType();
}
pub trait IWMMetadataEditorImpl: Sized {
    fn Open();
    fn Close();
    fn Flush();
}
pub trait IWMMetadataEditor2Impl: Sized + IWMMetadataEditorImpl {
    fn OpenEx();
}
pub trait IWMMutualExclusionImpl: Sized + IWMStreamListImpl {
    fn GetType();
    fn SetType();
}
pub trait IWMMutualExclusion2Impl: Sized + IWMMutualExclusionImpl + IWMStreamListImpl {
    fn GetName();
    fn SetName();
    fn GetRecordCount();
    fn AddRecord();
    fn RemoveRecord();
    fn GetRecordName();
    fn SetRecordName();
    fn GetStreamsForRecord();
    fn AddStreamForRecord();
    fn RemoveStreamForRecord();
}
pub trait IWMOutputMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetStreamGroupName();
    fn GetConnectionName();
}
pub trait IWMPacketSizeImpl: Sized {
    fn GetMaxPacketSize();
    fn SetMaxPacketSize();
}
pub trait IWMPacketSize2Impl: Sized + IWMPacketSizeImpl {
    fn GetMinPacketSize();
    fn SetMinPacketSize();
}
pub trait IWMPlayerHookImpl: Sized {
    fn PreDecode();
}
pub trait IWMPlayerTimestampHookImpl: Sized {
    fn MapTimestamp();
}
pub trait IWMProfileImpl: Sized {
    fn GetVersion();
    fn GetName();
    fn SetName();
    fn GetDescription();
    fn SetDescription();
    fn GetStreamCount();
    fn GetStream();
    fn GetStreamByNumber();
    fn RemoveStream();
    fn RemoveStreamByNumber();
    fn AddStream();
    fn ReconfigStream();
    fn CreateNewStream();
    fn GetMutualExclusionCount();
    fn GetMutualExclusion();
    fn RemoveMutualExclusion();
    fn AddMutualExclusion();
    fn CreateNewMutualExclusion();
}
pub trait IWMProfile2Impl: Sized + IWMProfileImpl {
    fn GetProfileID();
}
pub trait IWMProfile3Impl: Sized + IWMProfile2Impl + IWMProfileImpl {
    fn GetStorageFormat();
    fn SetStorageFormat();
    fn GetBandwidthSharingCount();
    fn GetBandwidthSharing();
    fn RemoveBandwidthSharing();
    fn AddBandwidthSharing();
    fn CreateNewBandwidthSharing();
    fn GetStreamPrioritization();
    fn SetStreamPrioritization();
    fn RemoveStreamPrioritization();
    fn CreateNewStreamPrioritization();
    fn GetExpectedPacketCount();
}
pub trait IWMProfileManagerImpl: Sized {
    fn CreateEmptyProfile();
    fn LoadProfileByID();
    fn LoadProfileByData();
    fn SaveProfile();
    fn GetSystemProfileCount();
    fn LoadSystemProfile();
}
pub trait IWMProfileManager2Impl: Sized + IWMProfileManagerImpl {
    fn GetSystemProfileVersion();
    fn SetSystemProfileVersion();
}
pub trait IWMProfileManagerLanguageImpl: Sized {
    fn GetUserLanguageID();
    fn SetUserLanguageID();
}
pub trait IWMPropertyVaultImpl: Sized {
    fn GetPropertyCount();
    fn GetPropertyByName();
    fn SetProperty();
    fn GetPropertyByIndex();
    fn CopyPropertiesFrom();
    fn Clear();
}
pub trait IWMProximityDetectionImpl: Sized {
    fn StartDetection();
}
pub trait IWMReaderImpl: Sized {
    fn Open();
    fn Close();
    fn GetOutputCount();
    fn GetOutputProps();
    fn SetOutputProps();
    fn GetOutputFormatCount();
    fn GetOutputFormat();
    fn Start();
    fn Stop();
    fn Pause();
    fn Resume();
}
pub trait IWMReaderAcceleratorImpl: Sized {
    fn GetCodecInterface();
    fn Notify();
}
pub trait IWMReaderAdvancedImpl: Sized {
    fn SetUserProvidedClock();
    fn GetUserProvidedClock();
    fn DeliverTime();
    fn SetManualStreamSelection();
    fn GetManualStreamSelection();
    fn SetStreamsSelected();
    fn GetStreamSelected();
    fn SetReceiveSelectionCallbacks();
    fn GetReceiveSelectionCallbacks();
    fn SetReceiveStreamSamples();
    fn GetReceiveStreamSamples();
    fn SetAllocateForOutput();
    fn GetAllocateForOutput();
    fn SetAllocateForStream();
    fn GetAllocateForStream();
    fn GetStatistics();
    fn SetClientInfo();
    fn GetMaxOutputSampleSize();
    fn GetMaxStreamSampleSize();
    fn NotifyLateDelivery();
}
pub trait IWMReaderAdvanced2Impl: Sized + IWMReaderAdvancedImpl {
    fn SetPlayMode();
    fn GetPlayMode();
    fn GetBufferProgress();
    fn GetDownloadProgress();
    fn GetSaveAsProgress();
    fn SaveFileAs();
    fn GetProtocolName();
    fn StartAtMarker();
    fn GetOutputSetting();
    fn SetOutputSetting();
    fn Preroll();
    fn SetLogClientID();
    fn GetLogClientID();
    fn StopBuffering();
    fn OpenStream();
}
pub trait IWMReaderAdvanced3Impl: Sized + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn StopNetStreaming();
    fn StartAtPosition();
}
pub trait IWMReaderAdvanced4Impl: Sized + IWMReaderAdvanced3Impl + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn GetLanguageCount();
    fn GetLanguage();
    fn GetMaxSpeedFactor();
    fn IsUsingFastCache();
    fn AddLogParam();
    fn SendLogParams();
    fn CanSaveFileAs();
    fn CancelSaveFileAs();
    fn GetURL();
}
pub trait IWMReaderAdvanced5Impl: Sized + IWMReaderAdvanced4Impl + IWMReaderAdvanced3Impl + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn SetPlayerHook();
}
pub trait IWMReaderAdvanced6Impl: Sized + IWMReaderAdvanced5Impl + IWMReaderAdvanced4Impl + IWMReaderAdvanced3Impl + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn SetProtectStreamSamples();
}
pub trait IWMReaderAllocatorExImpl: Sized {
    fn AllocateForStreamEx();
    fn AllocateForOutputEx();
}
pub trait IWMReaderCallbackImpl: Sized + IWMStatusCallbackImpl {
    fn OnSample();
}
pub trait IWMReaderCallbackAdvancedImpl: Sized {
    fn OnStreamSample();
    fn OnTime();
    fn OnStreamSelection();
    fn OnOutputPropsChanged();
    fn AllocateForStream();
    fn AllocateForOutput();
}
pub trait IWMReaderNetworkConfigImpl: Sized {
    fn GetBufferingTime();
    fn SetBufferingTime();
    fn GetUDPPortRanges();
    fn SetUDPPortRanges();
    fn GetProxySettings();
    fn SetProxySettings();
    fn GetProxyHostName();
    fn SetProxyHostName();
    fn GetProxyPort();
    fn SetProxyPort();
    fn GetProxyExceptionList();
    fn SetProxyExceptionList();
    fn GetProxyBypassForLocal();
    fn SetProxyBypassForLocal();
    fn GetForceRerunAutoProxyDetection();
    fn SetForceRerunAutoProxyDetection();
    fn GetEnableMulticast();
    fn SetEnableMulticast();
    fn GetEnableHTTP();
    fn SetEnableHTTP();
    fn GetEnableUDP();
    fn SetEnableUDP();
    fn GetEnableTCP();
    fn SetEnableTCP();
    fn ResetProtocolRollover();
    fn GetConnectionBandwidth();
    fn SetConnectionBandwidth();
    fn GetNumProtocolsSupported();
    fn GetSupportedProtocolName();
    fn AddLoggingUrl();
    fn GetLoggingUrl();
    fn GetLoggingUrlCount();
    fn ResetLoggingUrlList();
}
pub trait IWMReaderNetworkConfig2Impl: Sized + IWMReaderNetworkConfigImpl {
    fn GetEnableContentCaching();
    fn SetEnableContentCaching();
    fn GetEnableFastCache();
    fn SetEnableFastCache();
    fn GetAcceleratedStreamingDuration();
    fn SetAcceleratedStreamingDuration();
    fn GetAutoReconnectLimit();
    fn SetAutoReconnectLimit();
    fn GetEnableResends();
    fn SetEnableResends();
    fn GetEnableThinning();
    fn SetEnableThinning();
    fn GetMaxNetPacketSize();
}
pub trait IWMReaderPlaylistBurnImpl: Sized {
    fn InitPlaylistBurn();
    fn GetInitResults();
    fn Cancel();
    fn EndPlaylistBurn();
}
pub trait IWMReaderStreamClockImpl: Sized {
    fn GetTime();
    fn SetTimer();
    fn KillTimer();
}
pub trait IWMReaderTimecodeImpl: Sized {
    fn GetTimecodeRangeCount();
    fn GetTimecodeRangeBounds();
}
pub trait IWMReaderTypeNegotiationImpl: Sized {
    fn TryOutputProps();
}
pub trait IWMRegisterCallbackImpl: Sized {
    fn Advise();
    fn Unadvise();
}
pub trait IWMRegisteredDeviceImpl: Sized {
    fn GetDeviceSerialNumber();
    fn GetDeviceCertificate();
    fn GetDeviceType();
    fn GetAttributeCount();
    fn GetAttributeByIndex();
    fn GetAttributeByName();
    fn SetAttributeByName();
    fn Approve();
    fn IsValid();
    fn IsApproved();
    fn IsWmdrmCompliant();
    fn IsOpened();
    fn Open();
    fn Close();
}
pub trait IWMSBufferAllocatorImpl: Sized {
    fn AllocateBuffer();
    fn AllocatePageSizeBuffer();
}
pub trait IWMSInternalAdminNetSourceImpl: Sized {
    fn Initialize();
    fn GetNetSourceCreator();
    fn SetCredentials();
    fn GetCredentials();
    fn DeleteCredentials();
    fn GetCredentialFlags();
    fn SetCredentialFlags();
    fn FindProxyForURL();
    fn RegisterProxyFailure();
    fn ShutdownProxyContext();
    fn IsUsingIE();
}
pub trait IWMSInternalAdminNetSource2Impl: Sized {
    fn SetCredentialsEx();
    fn GetCredentialsEx();
    fn DeleteCredentialsEx();
    fn FindProxyForURLEx();
}
pub trait IWMSInternalAdminNetSource3Impl: Sized + IWMSInternalAdminNetSource2Impl {
    fn GetNetSourceCreator2();
    fn FindProxyForURLEx2();
    fn RegisterProxyFailure2();
    fn ShutdownProxyContext2();
    fn IsUsingIE2();
    fn SetCredentialsEx2();
    fn GetCredentialsEx2();
}
pub trait IWMSecureChannelImpl: Sized + IWMAuthorizerImpl {
    fn WMSC_AddCertificate();
    fn WMSC_AddSignature();
    fn WMSC_Connect();
    fn WMSC_IsConnected();
    fn WMSC_Disconnect();
    fn WMSC_GetValidCertificate();
    fn WMSC_Encrypt();
    fn WMSC_Decrypt();
    fn WMSC_Lock();
    fn WMSC_Unlock();
    fn WMSC_SetSharedData();
}
pub trait IWMStatusCallbackImpl: Sized {
    fn OnStatus();
}
pub trait IWMStreamConfigImpl: Sized {
    fn GetStreamType();
    fn GetStreamNumber();
    fn SetStreamNumber();
    fn GetStreamName();
    fn SetStreamName();
    fn GetConnectionName();
    fn SetConnectionName();
    fn GetBitrate();
    fn SetBitrate();
    fn GetBufferWindow();
    fn SetBufferWindow();
}
pub trait IWMStreamConfig2Impl: Sized + IWMStreamConfigImpl {
    fn GetTransportType();
    fn SetTransportType();
    fn AddDataUnitExtension();
    fn GetDataUnitExtensionCount();
    fn GetDataUnitExtension();
    fn RemoveAllDataUnitExtensions();
}
pub trait IWMStreamConfig3Impl: Sized + IWMStreamConfig2Impl + IWMStreamConfigImpl {
    fn GetLanguage();
    fn SetLanguage();
}
pub trait IWMStreamListImpl: Sized {
    fn GetStreams();
    fn AddStream();
    fn RemoveStream();
}
pub trait IWMStreamPrioritizationImpl: Sized {
    fn GetPriorityRecords();
    fn SetPriorityRecords();
}
pub trait IWMSyncReaderImpl: Sized {
    fn Open();
    fn Close();
    fn SetRange();
    fn SetRangeByFrame();
    fn GetNextSample();
    fn SetStreamsSelected();
    fn GetStreamSelected();
    fn SetReadStreamSamples();
    fn GetReadStreamSamples();
    fn GetOutputSetting();
    fn SetOutputSetting();
    fn GetOutputCount();
    fn GetOutputProps();
    fn SetOutputProps();
    fn GetOutputFormatCount();
    fn GetOutputFormat();
    fn GetOutputNumberForStream();
    fn GetStreamNumberForOutput();
    fn GetMaxOutputSampleSize();
    fn GetMaxStreamSampleSize();
    fn OpenStream();
}
pub trait IWMSyncReader2Impl: Sized + IWMSyncReaderImpl {
    fn SetRangeByTimecode();
    fn SetRangeByFrameEx();
    fn SetAllocateForOutput();
    fn GetAllocateForOutput();
    fn SetAllocateForStream();
    fn GetAllocateForStream();
}
pub trait IWMVideoMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetMaxKeyFrameSpacing();
    fn SetMaxKeyFrameSpacing();
    fn GetQuality();
    fn SetQuality();
}
pub trait IWMWatermarkInfoImpl: Sized {
    fn GetWatermarkEntryCount();
    fn GetWatermarkEntry();
}
pub trait IWMWriterImpl: Sized {
    fn SetProfileByID();
    fn SetProfile();
    fn SetOutputFilename();
    fn GetInputCount();
    fn GetInputProps();
    fn SetInputProps();
    fn GetInputFormatCount();
    fn GetInputFormat();
    fn BeginWriting();
    fn EndWriting();
    fn AllocateSample();
    fn WriteSample();
    fn Flush();
}
pub trait IWMWriterAdvancedImpl: Sized {
    fn GetSinkCount();
    fn GetSink();
    fn AddSink();
    fn RemoveSink();
    fn WriteStreamSample();
    fn SetLiveSource();
    fn IsRealTime();
    fn GetWriterTime();
    fn GetStatistics();
    fn SetSyncTolerance();
    fn GetSyncTolerance();
}
pub trait IWMWriterAdvanced2Impl: Sized + IWMWriterAdvancedImpl {
    fn GetInputSetting();
    fn SetInputSetting();
}
pub trait IWMWriterAdvanced3Impl: Sized + IWMWriterAdvanced2Impl + IWMWriterAdvancedImpl {
    fn GetStatisticsEx();
    fn SetNonBlocking();
}
pub trait IWMWriterFileSinkImpl: Sized + IWMWriterSinkImpl {
    fn Open();
}
pub trait IWMWriterFileSink2Impl: Sized + IWMWriterFileSinkImpl + IWMWriterSinkImpl {
    fn Start();
    fn Stop();
    fn IsStopped();
    fn GetFileDuration();
    fn GetFileSize();
    fn Close();
    fn IsClosed();
}
pub trait IWMWriterFileSink3Impl: Sized + IWMWriterFileSink2Impl + IWMWriterFileSinkImpl + IWMWriterSinkImpl {
    fn SetAutoIndexing();
    fn GetAutoIndexing();
    fn SetControlStream();
    fn GetMode();
    fn OnDataUnitEx();
    fn SetUnbufferedIO();
    fn GetUnbufferedIO();
    fn CompleteOperations();
}
pub trait IWMWriterNetworkSinkImpl: Sized + IWMWriterSinkImpl {
    fn SetMaximumClients();
    fn GetMaximumClients();
    fn SetNetworkProtocol();
    fn GetNetworkProtocol();
    fn GetHostURL();
    fn Open();
    fn Disconnect();
    fn Close();
}
pub trait IWMWriterPostViewImpl: Sized {
    fn SetPostViewCallback();
    fn SetReceivePostViewSamples();
    fn GetReceivePostViewSamples();
    fn GetPostViewProps();
    fn SetPostViewProps();
    fn GetPostViewFormatCount();
    fn GetPostViewFormat();
    fn SetAllocateForPostView();
    fn GetAllocateForPostView();
}
pub trait IWMWriterPostViewCallbackImpl: Sized + IWMStatusCallbackImpl {
    fn OnPostViewSample();
    fn AllocateForPostView();
}
pub trait IWMWriterPreprocessImpl: Sized {
    fn GetMaxPreprocessingPasses();
    fn SetNumPreprocessingPasses();
    fn BeginPreprocessingPass();
    fn PreprocessSample();
    fn EndPreprocessingPass();
}
pub trait IWMWriterPushSinkImpl: Sized + IWMWriterSinkImpl {
    fn Connect();
    fn Disconnect();
    fn EndSession();
}
pub trait IWMWriterSinkImpl: Sized {
    fn OnHeader();
    fn IsRealTime();
    fn AllocateDataUnit();
    fn OnDataUnit();
    fn OnEndWriting();
}
