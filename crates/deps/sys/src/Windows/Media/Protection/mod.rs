#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
#[link(name = "windows")]
extern "system" {}
pub struct ComponentLoadFailedEventArgs(i32);
pub struct ComponentLoadFailedEventHandler(pub *mut ::core::ffi::c_void);
pub struct ComponentRenewal(i32);
pub struct GraphicsTrustStatus(i32);
pub struct HdcpProtection(i32);
pub struct HdcpSession(i32);
pub struct HdcpSetProtectionResult(i32);
pub struct IComponentLoadFailedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IComponentRenewalStatics(pub *mut ::core::ffi::c_void);
pub struct IHdcpSession(pub *mut ::core::ffi::c_void);
pub struct IMediaProtectionManager(pub *mut ::core::ffi::c_void);
pub struct IMediaProtectionPMPServer(pub *mut ::core::ffi::c_void);
pub struct IMediaProtectionPMPServerFactory(pub *mut ::core::ffi::c_void);
pub struct IMediaProtectionServiceCompletion(pub *mut ::core::ffi::c_void);
pub struct IMediaProtectionServiceRequest(pub *mut ::core::ffi::c_void);
pub struct IProtectionCapabilities(pub *mut ::core::ffi::c_void);
pub struct IRevocationAndRenewalInformation(pub *mut ::core::ffi::c_void);
pub struct IRevocationAndRenewalItem(pub *mut ::core::ffi::c_void);
pub struct IServiceRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IServiceRequestedEventArgs2(pub *mut ::core::ffi::c_void);
pub struct MediaProtectionManager(i32);
pub struct MediaProtectionPMPServer(i32);
pub struct MediaProtectionServiceCompletion(i32);
pub struct ProtectionCapabilities(i32);
pub struct ProtectionCapabilityResult(i32);
pub struct ProtectionRenewalContract(i32);
pub struct RebootNeededEventHandler(pub *mut ::core::ffi::c_void);
pub struct RenewalStatus(i32);
pub struct RevocationAndRenewalInformation(i32);
pub struct RevocationAndRenewalItem(i32);
pub struct RevocationAndRenewalReasons(i32);
pub struct ServiceRequestedEventArgs(i32);
pub struct ServiceRequestedEventHandler(pub *mut ::core::ffi::c_void);
