#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[link(name = "windows")]
extern "system" {
    fn IWiFiDirectAdvertisement();
    fn IWiFiDirectAdvertisement2();
    fn IWiFiDirectAdvertisementPublisher();
    fn IWiFiDirectAdvertisementPublisherStatusChangedEventArgs();
    fn IWiFiDirectConnectionListener();
    fn IWiFiDirectConnectionParameters();
    fn IWiFiDirectConnectionParameters2();
    fn IWiFiDirectConnectionParametersStatics();
    fn IWiFiDirectConnectionRequest();
    fn IWiFiDirectConnectionRequestedEventArgs();
    fn IWiFiDirectDevice();
    fn IWiFiDirectDeviceStatics();
    fn IWiFiDirectDeviceStatics2();
    fn IWiFiDirectInformationElement();
    fn IWiFiDirectInformationElementStatics();
    fn IWiFiDirectLegacySettings();
    fn WiFiDirectAdvertisement();
    fn WiFiDirectAdvertisementListenStateDiscoverability();
    fn WiFiDirectAdvertisementPublisher();
    fn WiFiDirectAdvertisementPublisherStatus();
    fn WiFiDirectAdvertisementPublisherStatusChangedEventArgs();
    fn WiFiDirectConfigurationMethod();
    fn WiFiDirectConnectionListener();
    fn WiFiDirectConnectionParameters();
    fn WiFiDirectConnectionRequest();
    fn WiFiDirectConnectionRequestedEventArgs();
    fn WiFiDirectConnectionStatus();
    fn WiFiDirectDevice();
    fn WiFiDirectDeviceSelectorType();
    fn WiFiDirectError();
    fn WiFiDirectInformationElement();
    fn WiFiDirectLegacySettings();
    fn WiFiDirectPairingProcedure();
}
