#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INDClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDClient {}
impl ::core::clone::Clone for INDClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDClientFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDClientFactory {}
impl ::core::clone::Clone for INDClientFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDClosedCaptionDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDClosedCaptionDataReceivedEventArgs {}
impl ::core::clone::Clone for INDClosedCaptionDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDCustomData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDCustomData {}
impl ::core::clone::Clone for INDCustomData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDCustomDataFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDCustomDataFactory {}
impl ::core::clone::Clone for INDCustomDataFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDDownloadEngine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDDownloadEngine {}
impl ::core::clone::Clone for INDDownloadEngine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDDownloadEngineNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDDownloadEngineNotifier {}
impl ::core::clone::Clone for INDDownloadEngineNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDLicenseFetchCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDLicenseFetchCompletedEventArgs {}
impl ::core::clone::Clone for INDLicenseFetchCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDLicenseFetchDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDLicenseFetchDescriptor {}
impl ::core::clone::Clone for INDLicenseFetchDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDLicenseFetchDescriptorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDLicenseFetchDescriptorFactory {}
impl ::core::clone::Clone for INDLicenseFetchDescriptorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDLicenseFetchResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDLicenseFetchResult {}
impl ::core::clone::Clone for INDLicenseFetchResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDMessenger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDMessenger {}
impl ::core::clone::Clone for INDMessenger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDProximityDetectionCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDProximityDetectionCompletedEventArgs {}
impl ::core::clone::Clone for INDProximityDetectionCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDRegistrationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDRegistrationCompletedEventArgs {}
impl ::core::clone::Clone for INDRegistrationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDSendResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDSendResult {}
impl ::core::clone::Clone for INDSendResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDStartResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDStartResult {}
impl ::core::clone::Clone for INDStartResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDStorageFileHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDStorageFileHelper {}
impl ::core::clone::Clone for INDStorageFileHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDStreamParser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDStreamParser {}
impl ::core::clone::Clone for INDStreamParser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDStreamParserNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDStreamParserNotifier {}
impl ::core::clone::Clone for INDStreamParserNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDTCPMessengerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDTCPMessengerFactory {}
impl ::core::clone::Clone for INDTCPMessengerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INDTransmitterProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INDTransmitterProperties {}
impl ::core::clone::Clone for INDTransmitterProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyContentHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyContentHeader {}
impl ::core::clone::Clone for IPlayReadyContentHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyContentHeader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyContentHeader2 {}
impl ::core::clone::Clone for IPlayReadyContentHeader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyContentHeaderFactory {}
impl ::core::clone::Clone for IPlayReadyContentHeaderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyContentHeaderFactory2 {}
impl ::core::clone::Clone for IPlayReadyContentHeaderFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyContentResolver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyContentResolver {}
impl ::core::clone::Clone for IPlayReadyContentResolver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyDomain(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyDomain {}
impl ::core::clone::Clone for IPlayReadyDomain {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyDomainIterableFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyDomainIterableFactory {}
impl ::core::clone::Clone for IPlayReadyDomainIterableFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyDomainJoinServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyDomainJoinServiceRequest {}
impl ::core::clone::Clone for IPlayReadyDomainJoinServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyDomainLeaveServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyDomainLeaveServiceRequest {}
impl ::core::clone::Clone for IPlayReadyDomainLeaveServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyITADataGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyITADataGenerator {}
impl ::core::clone::Clone for IPlayReadyITADataGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyIndividualizationServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyIndividualizationServiceRequest {}
impl ::core::clone::Clone for IPlayReadyIndividualizationServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicense(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicense {}
impl ::core::clone::Clone for IPlayReadyLicense {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicense2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicense2 {}
impl ::core::clone::Clone for IPlayReadyLicense2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseAcquisitionServiceRequest {}
impl ::core::clone::Clone for IPlayReadyLicenseAcquisitionServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseAcquisitionServiceRequest2 {}
impl ::core::clone::Clone for IPlayReadyLicenseAcquisitionServiceRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseAcquisitionServiceRequest3 {}
impl ::core::clone::Clone for IPlayReadyLicenseAcquisitionServiceRequest3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseIterableFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseIterableFactory {}
impl ::core::clone::Clone for IPlayReadyLicenseIterableFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseManagement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseManagement {}
impl ::core::clone::Clone for IPlayReadyLicenseManagement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseSession {}
impl ::core::clone::Clone for IPlayReadyLicenseSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseSession2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseSession2 {}
impl ::core::clone::Clone for IPlayReadyLicenseSession2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyLicenseSessionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyLicenseSessionFactory {}
impl ::core::clone::Clone for IPlayReadyLicenseSessionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyMeteringReportServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyMeteringReportServiceRequest {}
impl ::core::clone::Clone for IPlayReadyMeteringReportServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyRevocationServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyRevocationServiceRequest {}
impl ::core::clone::Clone for IPlayReadyRevocationServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadySecureStopIterableFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadySecureStopIterableFactory {}
impl ::core::clone::Clone for IPlayReadySecureStopIterableFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadySecureStopServiceRequest {}
impl ::core::clone::Clone for IPlayReadySecureStopServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequestFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadySecureStopServiceRequestFactory {}
impl ::core::clone::Clone for IPlayReadySecureStopServiceRequestFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyServiceRequest {}
impl ::core::clone::Clone for IPlayReadyServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadySoapMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadySoapMessage {}
impl ::core::clone::Clone for IPlayReadySoapMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyStatics {}
impl ::core::clone::Clone for IPlayReadyStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyStatics2 {}
impl ::core::clone::Clone for IPlayReadyStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyStatics3 {}
impl ::core::clone::Clone for IPlayReadyStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyStatics4 {}
impl ::core::clone::Clone for IPlayReadyStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayReadyStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayReadyStatics5 {}
impl ::core::clone::Clone for IPlayReadyStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDCertificateFeature(pub i32);
impl NDCertificateFeature {
    pub const Transmitter: Self = Self(1i32);
    pub const Receiver: Self = Self(2i32);
    pub const SharedCertificate: Self = Self(3i32);
    pub const SecureClock: Self = Self(4i32);
    pub const AntiRollBackClock: Self = Self(5i32);
    pub const CRLS: Self = Self(9i32);
    pub const PlayReady3Features: Self = Self(13i32);
}
impl ::core::marker::Copy for NDCertificateFeature {}
impl ::core::clone::Clone for NDCertificateFeature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDCertificatePlatformID(pub i32);
impl NDCertificatePlatformID {
    pub const Windows: Self = Self(0i32);
    pub const OSX: Self = Self(1i32);
    pub const WindowsOnARM: Self = Self(2i32);
    pub const WindowsMobile7: Self = Self(5i32);
    pub const iOSOnARM: Self = Self(6i32);
    pub const XBoxOnPPC: Self = Self(7i32);
    pub const WindowsPhone8OnARM: Self = Self(8i32);
    pub const WindowsPhone8OnX86: Self = Self(9i32);
    pub const XboxOne: Self = Self(10i32);
    pub const AndroidOnARM: Self = Self(11i32);
    pub const WindowsPhone81OnARM: Self = Self(12i32);
    pub const WindowsPhone81OnX86: Self = Self(13i32);
}
impl ::core::marker::Copy for NDCertificatePlatformID {}
impl ::core::clone::Clone for NDCertificatePlatformID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDCertificateType(pub i32);
impl NDCertificateType {
    pub const Unknown: Self = Self(0i32);
    pub const PC: Self = Self(1i32);
    pub const Device: Self = Self(2i32);
    pub const Domain: Self = Self(3i32);
    pub const Issuer: Self = Self(4i32);
    pub const CrlSigner: Self = Self(5i32);
    pub const Service: Self = Self(6i32);
    pub const Silverlight: Self = Self(7i32);
    pub const Application: Self = Self(8i32);
    pub const Metering: Self = Self(9i32);
    pub const KeyFileSigner: Self = Self(10i32);
    pub const Server: Self = Self(11i32);
    pub const LicenseSigner: Self = Self(12i32);
}
impl ::core::marker::Copy for NDCertificateType {}
impl ::core::clone::Clone for NDCertificateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NDClient {}
impl ::core::clone::Clone for NDClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDClosedCaptionFormat(pub i32);
impl NDClosedCaptionFormat {
    pub const ATSC: Self = Self(0i32);
    pub const SCTE20: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl ::core::marker::Copy for NDClosedCaptionFormat {}
impl ::core::clone::Clone for NDClosedCaptionFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDContentIDType(pub i32);
impl NDContentIDType {
    pub const KeyID: Self = Self(1i32);
    pub const PlayReadyObject: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for NDContentIDType {}
impl ::core::clone::Clone for NDContentIDType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDCustomData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NDCustomData {}
impl ::core::clone::Clone for NDCustomData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDDownloadEngineNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NDDownloadEngineNotifier {}
impl ::core::clone::Clone for NDDownloadEngineNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDLicenseFetchDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NDLicenseFetchDescriptor {}
impl ::core::clone::Clone for NDLicenseFetchDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDMediaStreamType(pub i32);
impl NDMediaStreamType {
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for NDMediaStreamType {}
impl ::core::clone::Clone for NDMediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDProximityDetectionType(pub i32);
impl NDProximityDetectionType {
    pub const UDP: Self = Self(1i32);
    pub const TCP: Self = Self(2i32);
    pub const TransportAgnostic: Self = Self(4i32);
}
impl ::core::marker::Copy for NDProximityDetectionType {}
impl ::core::clone::Clone for NDProximityDetectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDStartAsyncOptions(pub i32);
impl NDStartAsyncOptions {
    pub const MutualAuthentication: Self = Self(1i32);
    pub const WaitForLicenseDescriptor: Self = Self(2i32);
}
impl ::core::marker::Copy for NDStartAsyncOptions {}
impl ::core::clone::Clone for NDStartAsyncOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDStorageFileHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NDStorageFileHelper {}
impl ::core::clone::Clone for NDStorageFileHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDStreamParserNotifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NDStreamParserNotifier {}
impl ::core::clone::Clone for NDStreamParserNotifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NDTCPMessenger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NDTCPMessenger {}
impl ::core::clone::Clone for NDTCPMessenger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyContentHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyContentHeader {}
impl ::core::clone::Clone for PlayReadyContentHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: Self = Self(0i32);
    pub const OnDemand: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyDecryptorSetup {}
impl ::core::clone::Clone for PlayReadyDecryptorSetup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyDomain(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyDomain {}
impl ::core::clone::Clone for PlayReadyDomain {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyDomainIterable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyDomainIterable {}
impl ::core::clone::Clone for PlayReadyDomainIterable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyDomainIterator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyDomainIterator {}
impl ::core::clone::Clone for PlayReadyDomainIterator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyDomainJoinServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyDomainJoinServiceRequest {}
impl ::core::clone::Clone for PlayReadyDomainJoinServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyDomainLeaveServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyDomainLeaveServiceRequest {}
impl ::core::clone::Clone for PlayReadyDomainLeaveServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: Self = Self(0i32);
    pub const Aes128Ctr: Self = Self(1i32);
    pub const Cocktail: Self = Self(4i32);
    pub const Aes128Cbc: Self = Self(5i32);
    pub const Unspecified: Self = Self(65535i32);
    pub const Uninitialized: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for PlayReadyEncryptionAlgorithm {}
impl ::core::clone::Clone for PlayReadyEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: Self = Self(1i32);
    pub const HEVC: Self = Self(2i32);
    pub const Aes128Cbc: Self = Self(3i32);
}
impl ::core::marker::Copy for PlayReadyHardwareDRMFeatures {}
impl ::core::clone::Clone for PlayReadyHardwareDRMFeatures {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: Self = Self(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyITADataFormat {}
impl ::core::clone::Clone for PlayReadyITADataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyITADataGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyITADataGenerator {}
impl ::core::clone::Clone for PlayReadyITADataGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyIndividualizationServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyIndividualizationServiceRequest {}
impl ::core::clone::Clone for PlayReadyIndividualizationServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyLicense(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyLicense {}
impl ::core::clone::Clone for PlayReadyLicense {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyLicenseAcquisitionServiceRequest {}
impl ::core::clone::Clone for PlayReadyLicenseAcquisitionServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyLicenseIterable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyLicenseIterable {}
impl ::core::clone::Clone for PlayReadyLicenseIterable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyLicenseIterator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyLicenseIterator {}
impl ::core::clone::Clone for PlayReadyLicenseIterator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyLicenseSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyLicenseSession {}
impl ::core::clone::Clone for PlayReadyLicenseSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyMeteringReportServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyMeteringReportServiceRequest {}
impl ::core::clone::Clone for PlayReadyMeteringReportServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadyRevocationServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadyRevocationServiceRequest {}
impl ::core::clone::Clone for PlayReadyRevocationServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadySecureStopIterable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadySecureStopIterable {}
impl ::core::clone::Clone for PlayReadySecureStopIterable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadySecureStopIterator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadySecureStopIterator {}
impl ::core::clone::Clone for PlayReadySecureStopIterator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadySecureStopServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadySecureStopServiceRequest {}
impl ::core::clone::Clone for PlayReadySecureStopServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayReadySoapMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayReadySoapMessage {}
impl ::core::clone::Clone for PlayReadySoapMessage {
    fn clone(&self) -> Self {
        *self
    }
}
