#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BluetoothEventTriggeringMode();
    fn BluetoothLEAdvertisementPublisherTriggerDetails();
    fn BluetoothLEAdvertisementWatcherTriggerDetails();
    fn GattCharacteristicNotificationTriggerDetails();
    fn GattServiceProviderConnection();
    fn GattServiceProviderTriggerDetails();
    fn IBluetoothLEAdvertisementPublisherTriggerDetails();
    fn IBluetoothLEAdvertisementPublisherTriggerDetails2();
    fn IBluetoothLEAdvertisementWatcherTriggerDetails();
    fn IGattCharacteristicNotificationTriggerDetails();
    fn IGattCharacteristicNotificationTriggerDetails2();
    fn IGattServiceProviderConnection();
    fn IGattServiceProviderConnectionStatics();
    fn IGattServiceProviderTriggerDetails();
    fn IRfcommConnectionTriggerDetails();
    fn IRfcommInboundConnectionInformation();
    fn IRfcommOutboundConnectionInformation();
    fn RfcommConnectionTriggerDetails();
    fn RfcommInboundConnectionInformation();
    fn RfcommOutboundConnectionInformation();
}
