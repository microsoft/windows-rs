#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWiFiDirectService();
    fn IWiFiDirectServiceAdvertiser();
    fn IWiFiDirectServiceAdvertiserFactory();
    fn IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs();
    fn IWiFiDirectServiceProvisioningInfo();
    fn IWiFiDirectServiceRemotePortAddedEventArgs();
    fn IWiFiDirectServiceSession();
    fn IWiFiDirectServiceSessionDeferredEventArgs();
    fn IWiFiDirectServiceSessionRequest();
    fn IWiFiDirectServiceSessionRequestedEventArgs();
    fn IWiFiDirectServiceStatics();
    fn WiFiDirectService();
    fn WiFiDirectServiceAdvertisementStatus();
    fn WiFiDirectServiceAdvertiser();
    fn WiFiDirectServiceAutoAcceptSessionConnectedEventArgs();
    fn WiFiDirectServiceConfigurationMethod();
    fn WiFiDirectServiceError();
    fn WiFiDirectServiceIPProtocol();
    fn WiFiDirectServiceProvisioningInfo();
    fn WiFiDirectServiceRemotePortAddedEventArgs();
    fn WiFiDirectServiceSession();
    fn WiFiDirectServiceSessionDeferredEventArgs();
    fn WiFiDirectServiceSessionErrorStatus();
    fn WiFiDirectServiceSessionRequest();
    fn WiFiDirectServiceSessionRequestedEventArgs();
    fn WiFiDirectServiceSessionStatus();
    fn WiFiDirectServiceStatus();
}
