#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDClientImpl: Sized {
    fn RegistrationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRegistrationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProximityDetectionCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveProximityDetectionCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LicenseFetchCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLicenseFetchCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReRegistrationNeeded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReRegistrationNeeded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ClosedCaptionDataReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosedCaptionDataReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAsync(&self, contenturl: &::core::option::Option<super::super::super::Foundation::Uri>, startasyncoptions: u32, registrationcustomdata: &::core::option::Option<INDCustomData>, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDStartResult>>;
    fn LicenseFetchAsync(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>>;
    fn ReRegistrationAsync(&self, registrationcustomdata: &::core::option::Option<INDCustomData>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDClientFactoryImpl: Sized {
    fn CreateInstance(&self, downloadengine: &::core::option::Option<INDDownloadEngine>, streamparser: &::core::option::Option<INDStreamParser>, pmessenger: &::core::option::Option<INDMessenger>) -> ::windows::core::Result<NDClient>;
}
#[cfg(feature = "deprecated")]
pub trait INDClosedCaptionDataReceivedEventArgsImpl: Sized {
    fn ClosedCaptionDataFormat(&self) -> ::windows::core::Result<NDClosedCaptionFormat>;
    fn PresentationTimestamp(&self) -> ::windows::core::Result<i64>;
    fn ClosedCaptionData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
pub trait INDCustomDataImpl: Sized {
    fn CustomDataTypeID(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn CustomData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDCustomDataFactoryImpl: Sized {
    fn CreateInstance(&self, customdatatypeidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], customdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<NDCustomData>;
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineImpl: Sized {
    fn Open(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Seek(&self, startposition: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn BufferFullMinThresholdInSamples(&self) -> ::windows::core::Result<u32>;
    fn BufferFullMaxThresholdInSamples(&self) -> ::windows::core::Result<u32>;
    fn Notifier(&self) -> ::windows::core::Result<NDDownloadEngineNotifier>;
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineNotifierImpl: Sized {
    fn OnStreamOpened(&self) -> ::windows::core::Result<()>;
    fn OnPlayReadyObjectReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn OnContentIDReceived(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnDataReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], bytesreceived: u32) -> ::windows::core::Result<()>;
    fn OnEndOfStream(&self) -> ::windows::core::Result<()>;
    fn OnNetworkError(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchCompletedEventArgsImpl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchDescriptorImpl: Sized {
    fn ContentIDType(&self) -> ::windows::core::Result<NDContentIDType>;
    fn ContentID(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn LicenseFetchChallengeCustomData(&self) -> ::windows::core::Result<INDCustomData>;
    fn SetLicenseFetchChallengeCustomData(&self, licensefetchchallengecustomdata: &::core::option::Option<INDCustomData>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDLicenseFetchDescriptorFactoryImpl: Sized {
    fn CreateInstance(&self, contentidtype: NDContentIDType, contentidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], licensefetchchallengecustomdata: &::core::option::Option<INDCustomData>) -> ::windows::core::Result<NDLicenseFetchDescriptor>;
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchResultImpl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
pub trait INDMessengerImpl: Sized {
    fn SendRegistrationRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
}
#[cfg(feature = "deprecated")]
pub trait INDProximityDetectionCompletedEventArgsImpl: Sized {
    fn ProximityDetectionRetryCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "deprecated")]
pub trait INDRegistrationCompletedEventArgsImpl: Sized {
    fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData>;
    fn TransmitterProperties(&self) -> ::windows::core::Result<INDTransmitterProperties>;
    fn TransmitterCertificateAccepted(&self) -> ::windows::core::Result<bool>;
    fn SetTransmitterCertificateAccepted(&self, accept: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
pub trait INDSendResultImpl: Sized {
    fn Response(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
pub trait INDStartResultImpl: Sized {
    fn MediaStreamSource(&self) -> ::windows::core::Result<super::super::Core::MediaStreamSource>;
}
#[cfg(feature = "deprecated")]
pub trait INDStorageFileHelperImpl: Sized {
    fn GetFileURLs(&self, file: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "deprecated")]
pub trait INDStreamParserImpl: Sized {
    fn ParseData(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetStreamInformation(&self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, streamtype: &mut NDMediaStreamType) -> ::windows::core::Result<u32>;
    fn BeginOfStream(&self) -> ::windows::core::Result<()>;
    fn EndOfStream(&self) -> ::windows::core::Result<()>;
    fn Notifier(&self) -> ::windows::core::Result<NDStreamParserNotifier>;
}
#[cfg(feature = "deprecated")]
pub trait INDStreamParserNotifierImpl: Sized {
    fn OnContentIDReceived(&self, licensefetchdescriptor: &::core::option::Option<INDLicenseFetchDescriptor>) -> ::windows::core::Result<()>;
    fn OnMediaStreamDescriptorCreated(&self, audiostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, videostreamdescriptors: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn OnSampleParsed(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: &::core::option::Option<super::super::Core::MediaStreamSample>, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn OnBeginSetupDecryptor(&self, descriptor: &::core::option::Option<super::super::Core::IMediaStreamDescriptor>, keyid: &::windows::core::GUID, probytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait INDTCPMessengerFactoryImpl: Sized {
    fn CreateInstance(&self, remotehostname: &::windows::core::HSTRING, remotehostport: u32) -> ::windows::core::Result<NDTCPMessenger>;
}
#[cfg(feature = "deprecated")]
pub trait INDTransmitterPropertiesImpl: Sized {
    fn CertificateType(&self) -> ::windows::core::Result<NDCertificateType>;
    fn PlatformIdentifier(&self) -> ::windows::core::Result<NDCertificatePlatformID>;
    fn SupportedFeatures(&self) -> ::windows::core::Result<::windows::core::Array<NDCertificateFeature>>;
    fn SecurityLevel(&self) -> ::windows::core::Result<u32>;
    fn SecurityVersion(&self) -> ::windows::core::Result<u32>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ClientID(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn ModelDigest(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn ModelManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeaderImpl: Sized {
    fn KeyId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn KeyIdString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LicenseAcquisitionUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn LicenseAcquisitionUserInterfaceUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn EncryptionType(&self) -> ::windows::core::Result<PlayReadyEncryptionAlgorithm>;
    fn CustomAttributes(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DecryptorSetup(&self) -> ::windows::core::Result<PlayReadyDecryptorSetup>;
    fn GetSerializedHeader(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn HeaderWithEmbeddedUpdates(&self) -> ::windows::core::Result<PlayReadyContentHeader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeader2Impl: Sized + IPlayReadyContentHeaderImpl {
    fn KeyIds(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::GUID>>;
    fn KeyIdStrings(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeaderFactoryImpl: Sized {
    fn CreateInstanceFromWindowsMediaDrmHeader(&self, headerbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], licenseacquisitionurl: &::core::option::Option<super::super::super::Foundation::Uri>, licenseacquisitionuserinterfaceurl: &::core::option::Option<super::super::super::Foundation::Uri>, customattributes: &::windows::core::HSTRING, domainserviceid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn CreateInstanceFromComponents(&self, contentkeyid: &::windows::core::GUID, contentkeyidstring: &::windows::core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: &::core::option::Option<super::super::super::Foundation::Uri>, licenseacquisitionuserinterfaceurl: &::core::option::Option<super::super::super::Foundation::Uri>, customattributes: &::windows::core::HSTRING, domainserviceid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn CreateInstanceFromPlayReadyHeader(&self, headerbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadyContentHeader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentHeaderFactory2Impl: Sized {
    fn CreateInstanceFromComponents2(&self, dwflags: u32, contentkeyids: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], contentkeyidstrings: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: &::core::option::Option<super::super::super::Foundation::Uri>, licenseacquisitionuserinterfaceurl: &::core::option::Option<super::super::super::Foundation::Uri>, customattributes: &::windows::core::HSTRING, domainserviceid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyContentResolverImpl: Sized {
    fn ServiceRequest(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<IPlayReadyServiceRequest>;
}
pub trait IPlayReadyDomainImpl: Sized {
    fn AccountId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Revision(&self) -> ::windows::core::Result<u32>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DomainJoinUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyDomainIterableFactoryImpl: Sized {
    fn CreateInstance(&self, domainaccountid: &::windows::core::GUID) -> ::windows::core::Result<PlayReadyDomainIterable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyDomainJoinServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainAccountId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DomainFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDomainFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyDomainLeaveServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainAccountId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyITADataGeneratorImpl: Sized {
    fn GenerateData(&self, guidcpsystemid: &::windows::core::GUID, countofstreams: u32, configuration: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>, format: PlayReadyITADataFormat) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyIndividualizationServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {}
pub trait IPlayReadyLicenseImpl: Sized {
    fn FullyEvaluated(&self) -> ::windows::core::Result<bool>;
    fn UsableForPlay(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ExpireAfterFirstPlay(&self) -> ::windows::core::Result<u32>;
    fn DomainAccountID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ChainDepth(&self) -> ::windows::core::Result<u32>;
    fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicense2Impl: Sized + IPlayReadyLicenseImpl {
    fn SecureStopId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SecurityLevel(&self) -> ::windows::core::Result<u32>;
    fn InMemoryOnly(&self) -> ::windows::core::Result<bool>;
    fn ExpiresInRealTime(&self) -> ::windows::core::Result<bool>;
}
pub trait IPlayReadyLicenseAcquisitionServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn ContentHeader(&self) -> ::windows::core::Result<PlayReadyContentHeader>;
    fn SetContentHeader(&self, value: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<()>;
    fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetDomainServiceId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseAcquisitionServiceRequest2Impl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyLicenseAcquisitionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseAcquisitionServiceRequest3Impl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyLicenseAcquisitionServiceRequestImpl + IPlayReadyLicenseAcquisitionServiceRequest2Impl + IPlayReadyServiceRequestImpl {
    fn CreateLicenseIterable(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseIterableFactoryImpl: Sized {
    fn CreateInstance(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseManagementImpl: Sized {
    fn DeleteLicenses(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
pub trait IPlayReadyLicenseSessionImpl: Sized {
    fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest>;
    fn ConfigureMediaProtectionManager(&self, mpm: &::core::option::Option<super::MediaProtectionManager>) -> ::windows::core::Result<()>;
}
pub trait IPlayReadyLicenseSession2Impl: Sized + IPlayReadyLicenseSessionImpl {
    fn CreateLicenseIterable(&self, contentheader: &::core::option::Option<PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyLicenseSessionFactoryImpl: Sized {
    fn CreateInstance(&self, configuration: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<PlayReadyLicenseSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyMeteringReportServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn MeteringCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetMeteringCertificate(&self, meteringcertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyRevocationServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadySecureStopIterableFactoryImpl: Sized {
    fn CreateInstance(&self, publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopIterable>;
}
pub trait IPlayReadySecureStopServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl + IPlayReadyServiceRequestImpl {
    fn SessionID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Stopped(&self) -> ::windows::core::Result<bool>;
    fn PublisherCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadySecureStopServiceRequestFactoryImpl: Sized {
    fn CreateInstance(&self, publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest>;
    fn CreateInstanceFromSessionID(&self, sessionid: &::windows::core::GUID, publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest>;
}
pub trait IPlayReadyServiceRequestImpl: Sized + IMediaProtectionServiceRequestImpl {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest>;
    fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage>;
    fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadySoapMessageImpl: Sized {
    fn GetMessageBody(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn MessageHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStaticsImpl: Sized {
    fn DomainJoinServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DomainLeaveServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IndividualizationServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LicenseAcquirerServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn MeteringReportServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RevocationServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn MediaProtectionSystemId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PlayReadySecurityVersion(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics2Impl: Sized + IPlayReadyStaticsImpl {
    fn PlayReadyCertificateSecurityLevel(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics3Impl: Sized + IPlayReadyStaticsImpl + IPlayReadyStatics2Impl {
    fn SecureStopServiceRequestType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CheckSupportedHardware(&self, hwdrmfeature: PlayReadyHardwareDRMFeatures) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics4Impl: Sized + IPlayReadyStaticsImpl + IPlayReadyStatics2Impl + IPlayReadyStatics3Impl {
    fn InputTrustAuthorityToCreate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProtectionSystemId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayReadyStatics5Impl: Sized + IPlayReadyStaticsImpl + IPlayReadyStatics2Impl + IPlayReadyStatics3Impl + IPlayReadyStatics4Impl {
    fn HardwareDRMDisabledAtTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn HardwareDRMDisabledUntilTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ResetHardwareDRMDisabled(&self) -> ::windows::core::Result<()>;
}
