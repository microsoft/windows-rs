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
#[repr(transparent)]
pub struct NDDownloadEngineNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDLicenseFetchDescriptor(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct NDStreamParserNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NDTCPMessenger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayReadyContentHeader(pub *mut ::core::ffi::c_void);
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
