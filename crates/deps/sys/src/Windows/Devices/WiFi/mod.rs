#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWiFiAdapter();
    fn IWiFiAdapter2();
    fn IWiFiAdapterStatics();
    fn IWiFiAvailableNetwork();
    fn IWiFiConnectionResult();
    fn IWiFiNetworkReport();
    fn IWiFiWpsConfigurationResult();
    fn WiFiAccessStatus();
    fn WiFiAdapter();
    fn WiFiAvailableNetwork();
    fn WiFiConnectionMethod();
    fn WiFiConnectionResult();
    fn WiFiConnectionStatus();
    fn WiFiNetworkKind();
    fn WiFiNetworkReport();
    fn WiFiPhyKind();
    fn WiFiReconnectionKind();
    fn WiFiWpsConfigurationResult();
    fn WiFiWpsConfigurationStatus();
    fn WiFiWpsKind();
}
