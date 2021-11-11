#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
#[link(name = "windows")]
extern "system" {
    fn ComponentLoadFailedEventArgs();
    fn ComponentLoadFailedEventHandler();
    fn ComponentRenewal();
    fn GraphicsTrustStatus();
    fn HdcpProtection();
    fn HdcpSession();
    fn HdcpSetProtectionResult();
    fn IComponentLoadFailedEventArgs();
    fn IComponentRenewalStatics();
    fn IHdcpSession();
    fn IMediaProtectionManager();
    fn IMediaProtectionPMPServer();
    fn IMediaProtectionPMPServerFactory();
    fn IMediaProtectionServiceCompletion();
    fn IMediaProtectionServiceRequest();
    fn IProtectionCapabilities();
    fn IRevocationAndRenewalInformation();
    fn IRevocationAndRenewalItem();
    fn IServiceRequestedEventArgs();
    fn IServiceRequestedEventArgs2();
    fn MediaProtectionManager();
    fn MediaProtectionPMPServer();
    fn MediaProtectionServiceCompletion();
    fn ProtectionCapabilities();
    fn ProtectionCapabilityResult();
    fn ProtectionRenewalContract();
    fn RebootNeededEventHandler();
    fn RenewalStatus();
    fn RevocationAndRenewalInformation();
    fn RevocationAndRenewalItem();
    fn RevocationAndRenewalReasons();
    fn ServiceRequestedEventArgs();
    fn ServiceRequestedEventHandler();
}
