#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ComponentLoadFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComponentLoadFailedEventArgs {}
impl ::core::clone::Clone for ComponentLoadFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComponentLoadFailedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComponentLoadFailedEventHandler {}
impl ::core::clone::Clone for ComponentLoadFailedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: Self = Self(0i32);
    pub const TrustEstablished: Self = Self(1i32);
    pub const EnvironmentNotSupported: Self = Self(2i32);
    pub const DriverNotSupported: Self = Self(3i32);
    pub const DriverSigningFailure: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for GraphicsTrustStatus {}
impl ::core::clone::Clone for GraphicsTrustStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const OnWithTypeEnforcement: Self = Self(2i32);
}
impl ::core::marker::Copy for HdcpProtection {}
impl ::core::clone::Clone for HdcpProtection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HdcpSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HdcpSession {}
impl ::core::clone::Clone for HdcpSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for HdcpSetProtectionResult {}
impl ::core::clone::Clone for HdcpSetProtectionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponentLoadFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentLoadFailedEventArgs {}
impl ::core::clone::Clone for IComponentLoadFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponentRenewalStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentRenewalStatics {}
impl ::core::clone::Clone for IComponentRenewalStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHdcpSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHdcpSession {}
impl ::core::clone::Clone for IHdcpSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaProtectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaProtectionManager {}
impl ::core::clone::Clone for IMediaProtectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaProtectionPMPServer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaProtectionPMPServer {}
impl ::core::clone::Clone for IMediaProtectionPMPServer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaProtectionPMPServerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaProtectionPMPServerFactory {}
impl ::core::clone::Clone for IMediaProtectionPMPServerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaProtectionServiceCompletion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaProtectionServiceCompletion {}
impl ::core::clone::Clone for IMediaProtectionServiceCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaProtectionServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaProtectionServiceRequest {}
impl ::core::clone::Clone for IMediaProtectionServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionCapabilities {}
impl ::core::clone::Clone for IProtectionCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevocationAndRenewalInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevocationAndRenewalInformation {}
impl ::core::clone::Clone for IRevocationAndRenewalInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevocationAndRenewalItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevocationAndRenewalItem {}
impl ::core::clone::Clone for IRevocationAndRenewalItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceRequestedEventArgs {}
impl ::core::clone::Clone for IServiceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceRequestedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceRequestedEventArgs2 {}
impl ::core::clone::Clone for IServiceRequestedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaProtectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaProtectionManager {}
impl ::core::clone::Clone for MediaProtectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaProtectionPMPServer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaProtectionPMPServer {}
impl ::core::clone::Clone for MediaProtectionPMPServer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaProtectionServiceCompletion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaProtectionServiceCompletion {}
impl ::core::clone::Clone for MediaProtectionServiceCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectionCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProtectionCapabilities {}
impl ::core::clone::Clone for ProtectionCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionCapabilityResult {}
impl ::core::clone::Clone for ProtectionCapabilityResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RebootNeededEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RebootNeededEventHandler {}
impl ::core::clone::Clone for RebootNeededEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const UpdatesInProgress: Self = Self(1i32);
    pub const UserCancelled: Self = Self(2i32);
    pub const AppComponentsMayNeedUpdating: Self = Self(3i32);
    pub const NoComponentsFound: Self = Self(4i32);
}
impl ::core::marker::Copy for RenewalStatus {}
impl ::core::clone::Clone for RenewalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RevocationAndRenewalInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RevocationAndRenewalInformation {}
impl ::core::clone::Clone for RevocationAndRenewalInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RevocationAndRenewalItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RevocationAndRenewalItem {}
impl ::core::clone::Clone for RevocationAndRenewalItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: Self = Self(1u32);
    pub const KernelModeComponentLoad: Self = Self(2u32);
    pub const AppComponent: Self = Self(4u32);
    pub const GlobalRevocationListLoadFailed: Self = Self(16u32);
    pub const InvalidGlobalRevocationListSignature: Self = Self(32u32);
    pub const GlobalRevocationListAbsent: Self = Self(4096u32);
    pub const ComponentRevoked: Self = Self(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: Self = Self(16384u32);
    pub const ComponentCertificateRevoked: Self = Self(32768u32);
    pub const InvalidComponentCertificateRoot: Self = Self(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: Self = Self(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: Self = Self(262144u32);
    pub const BootDriverVerificationFailed: Self = Self(1048576u32);
    pub const ComponentSignedWithTestCertificate: Self = Self(16777216u32);
    pub const EncryptionFailure: Self = Self(268435456u32);
}
impl ::core::marker::Copy for RevocationAndRenewalReasons {}
impl ::core::clone::Clone for RevocationAndRenewalReasons {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServiceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServiceRequestedEventArgs {}
impl ::core::clone::Clone for ServiceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServiceRequestedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServiceRequestedEventHandler {}
impl ::core::clone::Clone for ServiceRequestedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
