#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct NDCertificateFeature(pub i32);
impl NDCertificateFeature {
    pub const Transmitter: NDCertificateFeature = NDCertificateFeature(1i32);
    pub const Receiver: NDCertificateFeature = NDCertificateFeature(2i32);
    pub const SharedCertificate: NDCertificateFeature = NDCertificateFeature(3i32);
    pub const SecureClock: NDCertificateFeature = NDCertificateFeature(4i32);
    pub const AntiRollBackClock: NDCertificateFeature = NDCertificateFeature(5i32);
    pub const CRLS: NDCertificateFeature = NDCertificateFeature(9i32);
    pub const PlayReady3Features: NDCertificateFeature = NDCertificateFeature(13i32);
}
#[repr(transparent)]
pub struct NDCertificatePlatformID(pub i32);
impl NDCertificatePlatformID {
    pub const Windows: NDCertificatePlatformID = NDCertificatePlatformID(0i32);
    pub const OSX: NDCertificatePlatformID = NDCertificatePlatformID(1i32);
    pub const WindowsOnARM: NDCertificatePlatformID = NDCertificatePlatformID(2i32);
    pub const WindowsMobile7: NDCertificatePlatformID = NDCertificatePlatformID(5i32);
    pub const iOSOnARM: NDCertificatePlatformID = NDCertificatePlatformID(6i32);
    pub const XBoxOnPPC: NDCertificatePlatformID = NDCertificatePlatformID(7i32);
    pub const WindowsPhone8OnARM: NDCertificatePlatformID = NDCertificatePlatformID(8i32);
    pub const WindowsPhone8OnX86: NDCertificatePlatformID = NDCertificatePlatformID(9i32);
    pub const XboxOne: NDCertificatePlatformID = NDCertificatePlatformID(10i32);
    pub const AndroidOnARM: NDCertificatePlatformID = NDCertificatePlatformID(11i32);
    pub const WindowsPhone81OnARM: NDCertificatePlatformID = NDCertificatePlatformID(12i32);
    pub const WindowsPhone81OnX86: NDCertificatePlatformID = NDCertificatePlatformID(13i32);
}
#[repr(transparent)]
pub struct NDCertificateType(pub i32);
impl NDCertificateType {
    pub const Unknown: NDCertificateType = NDCertificateType(0i32);
    pub const PC: NDCertificateType = NDCertificateType(1i32);
    pub const Device: NDCertificateType = NDCertificateType(2i32);
    pub const Domain: NDCertificateType = NDCertificateType(3i32);
    pub const Issuer: NDCertificateType = NDCertificateType(4i32);
    pub const CrlSigner: NDCertificateType = NDCertificateType(5i32);
    pub const Service: NDCertificateType = NDCertificateType(6i32);
    pub const Silverlight: NDCertificateType = NDCertificateType(7i32);
    pub const Application: NDCertificateType = NDCertificateType(8i32);
    pub const Metering: NDCertificateType = NDCertificateType(9i32);
    pub const KeyFileSigner: NDCertificateType = NDCertificateType(10i32);
    pub const Server: NDCertificateType = NDCertificateType(11i32);
    pub const LicenseSigner: NDCertificateType = NDCertificateType(12i32);
}
#[repr(transparent)]
pub struct NDClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDClosedCaptionFormat(pub i32);
impl NDClosedCaptionFormat {
    pub const ATSC: NDClosedCaptionFormat = NDClosedCaptionFormat(0i32);
    pub const SCTE20: NDClosedCaptionFormat = NDClosedCaptionFormat(1i32);
    pub const Unknown: NDClosedCaptionFormat = NDClosedCaptionFormat(2i32);
}
#[repr(transparent)]
pub struct NDContentIDType(pub i32);
impl NDContentIDType {
    pub const KeyID: NDContentIDType = NDContentIDType(1i32);
    pub const PlayReadyObject: NDContentIDType = NDContentIDType(2i32);
    pub const Custom: NDContentIDType = NDContentIDType(3i32);
}
#[repr(transparent)]
pub struct NDCustomData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDDownloadEngineNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDLicenseFetchDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDMediaStreamType(pub i32);
impl NDMediaStreamType {
    pub const Audio: NDMediaStreamType = NDMediaStreamType(1i32);
    pub const Video: NDMediaStreamType = NDMediaStreamType(2i32);
}
#[repr(transparent)]
pub struct NDProximityDetectionType(pub i32);
impl NDProximityDetectionType {
    pub const UDP: NDProximityDetectionType = NDProximityDetectionType(1i32);
    pub const TCP: NDProximityDetectionType = NDProximityDetectionType(2i32);
    pub const TransportAgnostic: NDProximityDetectionType = NDProximityDetectionType(4i32);
}
#[repr(transparent)]
pub struct NDStartAsyncOptions(pub i32);
impl NDStartAsyncOptions {
    pub const MutualAuthentication: NDStartAsyncOptions = NDStartAsyncOptions(1i32);
    pub const WaitForLicenseDescriptor: NDStartAsyncOptions = NDStartAsyncOptions(2i32);
}
#[repr(transparent)]
pub struct NDStorageFileHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDStreamParserNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDTCPMessenger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyContentHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: PlayReadyDecryptorSetup = PlayReadyDecryptorSetup(0i32);
    pub const OnDemand: PlayReadyDecryptorSetup = PlayReadyDecryptorSetup(1i32);
}
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
#[repr(transparent)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(0i32);
    pub const Aes128Ctr: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(1i32);
    pub const Cocktail: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(4i32);
    pub const Aes128Cbc: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(5i32);
    pub const Unspecified: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(65535i32);
    pub const Uninitialized: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(2147483647i32);
}
#[repr(transparent)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(1i32);
    pub const HEVC: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(2i32);
    pub const Aes128Cbc: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(3i32);
}
#[repr(transparent)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: PlayReadyITADataFormat = PlayReadyITADataFormat(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: PlayReadyITADataFormat = PlayReadyITADataFormat(1i32);
}
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
