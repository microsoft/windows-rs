#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ComponentLoadFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComponentLoadFailedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComponentRenewal(pub *mut ::core::ffi::c_void);
pub struct GraphicsTrustStatus(i32);
pub struct HdcpProtection(i32);
#[repr(transparent)]
pub struct HdcpSession(pub *mut ::core::ffi::c_void);
pub struct HdcpSetProtectionResult(i32);
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
pub struct ProtectionCapabilityResult(i32);
pub struct ProtectionRenewalContract(i32);
#[repr(transparent)]
pub struct RebootNeededEventHandler(pub *mut ::core::ffi::c_void);
pub struct RenewalStatus(i32);
#[repr(transparent)]
pub struct RevocationAndRenewalInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RevocationAndRenewalItem(pub *mut ::core::ffi::c_void);
pub struct RevocationAndRenewalReasons(i32);
#[repr(transparent)]
pub struct ServiceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ServiceRequestedEventHandler(pub *mut ::core::ffi::c_void);
