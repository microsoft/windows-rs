#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ComponentLoadFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComponentLoadFailedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: GraphicsTrustStatus = GraphicsTrustStatus(0i32);
    pub const TrustEstablished: GraphicsTrustStatus = GraphicsTrustStatus(1i32);
    pub const EnvironmentNotSupported: GraphicsTrustStatus = GraphicsTrustStatus(2i32);
    pub const DriverNotSupported: GraphicsTrustStatus = GraphicsTrustStatus(3i32);
    pub const DriverSigningFailure: GraphicsTrustStatus = GraphicsTrustStatus(4i32);
    pub const UnknownFailure: GraphicsTrustStatus = GraphicsTrustStatus(5i32);
}
#[repr(transparent)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: HdcpProtection = HdcpProtection(0i32);
    pub const On: HdcpProtection = HdcpProtection(1i32);
    pub const OnWithTypeEnforcement: HdcpProtection = HdcpProtection(2i32);
}
#[repr(transparent)]
pub struct HdcpSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: HdcpSetProtectionResult = HdcpSetProtectionResult(0i32);
    pub const TimedOut: HdcpSetProtectionResult = HdcpSetProtectionResult(1i32);
    pub const NotSupported: HdcpSetProtectionResult = HdcpSetProtectionResult(2i32);
    pub const UnknownFailure: HdcpSetProtectionResult = HdcpSetProtectionResult(3i32);
}
#[repr(transparent)]
pub struct IComponentLoadFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentRenewalStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdcpSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaProtectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaProtectionPMPServer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaProtectionPMPServerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaProtectionServiceCompletion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaProtectionServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectionCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevocationAndRenewalInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevocationAndRenewalItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceRequestedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaProtectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaProtectionPMPServer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaProtectionServiceCompletion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectionCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: ProtectionCapabilityResult = ProtectionCapabilityResult(0i32);
    pub const Maybe: ProtectionCapabilityResult = ProtectionCapabilityResult(1i32);
    pub const Probably: ProtectionCapabilityResult = ProtectionCapabilityResult(2i32);
}
#[repr(C)]
pub struct ProtectionRenewalContract(i32);
#[repr(transparent)]
pub struct RebootNeededEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: RenewalStatus = RenewalStatus(0i32);
    pub const UpdatesInProgress: RenewalStatus = RenewalStatus(1i32);
    pub const UserCancelled: RenewalStatus = RenewalStatus(2i32);
    pub const AppComponentsMayNeedUpdating: RenewalStatus = RenewalStatus(3i32);
    pub const NoComponentsFound: RenewalStatus = RenewalStatus(4i32);
}
#[repr(transparent)]
pub struct RevocationAndRenewalInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RevocationAndRenewalItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: RevocationAndRenewalReasons = RevocationAndRenewalReasons(1u32);
    pub const KernelModeComponentLoad: RevocationAndRenewalReasons = RevocationAndRenewalReasons(2u32);
    pub const AppComponent: RevocationAndRenewalReasons = RevocationAndRenewalReasons(4u32);
    pub const GlobalRevocationListLoadFailed: RevocationAndRenewalReasons = RevocationAndRenewalReasons(16u32);
    pub const InvalidGlobalRevocationListSignature: RevocationAndRenewalReasons = RevocationAndRenewalReasons(32u32);
    pub const GlobalRevocationListAbsent: RevocationAndRenewalReasons = RevocationAndRenewalReasons(4096u32);
    pub const ComponentRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: RevocationAndRenewalReasons = RevocationAndRenewalReasons(16384u32);
    pub const ComponentCertificateRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(32768u32);
    pub const InvalidComponentCertificateRoot: RevocationAndRenewalReasons = RevocationAndRenewalReasons(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(262144u32);
    pub const BootDriverVerificationFailed: RevocationAndRenewalReasons = RevocationAndRenewalReasons(1048576u32);
    pub const ComponentSignedWithTestCertificate: RevocationAndRenewalReasons = RevocationAndRenewalReasons(16777216u32);
    pub const EncryptionFailure: RevocationAndRenewalReasons = RevocationAndRenewalReasons(268435456u32);
}
#[repr(transparent)]
pub struct ServiceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ServiceRequestedEventHandler(pub *mut ::core::ffi::c_void);
