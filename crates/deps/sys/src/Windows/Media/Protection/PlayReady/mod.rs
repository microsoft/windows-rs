#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INDClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDClientFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDClosedCaptionDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDCustomData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDCustomDataFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDDownloadEngine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDDownloadEngineNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDLicenseFetchCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDLicenseFetchDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDLicenseFetchDescriptorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDLicenseFetchResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDMessenger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDProximityDetectionCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDRegistrationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDSendResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDStartResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDStorageFileHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDStreamParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDStreamParserNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDTCPMessengerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INDTransmitterProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyContentHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyContentHeader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyContentResolver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyDomain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyDomainIterableFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyDomainJoinServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyDomainLeaveServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyITADataGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyIndividualizationServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicense2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseIterableFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseManagement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyLicenseSessionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyMeteringReportServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyRevocationServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadySecureStopIterableFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadySoapMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayReadyStatics5(pub *mut ::core::ffi::c_void);
pub struct NDCertificateFeature(i32);
pub struct NDCertificatePlatformID(i32);
pub struct NDCertificateType(i32);
#[repr(transparent)]
pub struct NDClient(pub *mut ::core::ffi::c_void);
pub struct NDClosedCaptionFormat(i32);
pub struct NDContentIDType(i32);
#[repr(transparent)]
pub struct NDCustomData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDDownloadEngineNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDLicenseFetchDescriptor(pub *mut ::core::ffi::c_void);
pub struct NDMediaStreamType(i32);
pub struct NDProximityDetectionType(i32);
pub struct NDStartAsyncOptions(i32);
#[repr(transparent)]
pub struct NDStorageFileHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDStreamParserNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDTCPMessenger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyContentHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyContentResolver(pub *mut ::core::ffi::c_void);
pub struct PlayReadyDecryptorSetup(i32);
#[repr(transparent)]
pub struct PlayReadyDomain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyDomainIterable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyDomainIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyDomainJoinServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyDomainLeaveServiceRequest(pub *mut ::core::ffi::c_void);
pub struct PlayReadyEncryptionAlgorithm(i32);
pub struct PlayReadyHardwareDRMFeatures(i32);
pub struct PlayReadyITADataFormat(i32);
#[repr(transparent)]
pub struct PlayReadyITADataGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyIndividualizationServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyLicense(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyLicenseIterable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyLicenseIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyLicenseManagement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyLicenseSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyMeteringReportServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyRevocationServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadySecureStopIterable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadySecureStopIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadySecureStopServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadySoapMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyStatics(pub *mut ::core::ffi::c_void);
