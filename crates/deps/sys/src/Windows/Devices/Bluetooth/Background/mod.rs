#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BluetoothEventTriggeringMode(i32);
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(i32);
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(i32);
pub struct GattCharacteristicNotificationTriggerDetails(i32);
pub struct GattServiceProviderConnection(i32);
pub struct GattServiceProviderTriggerDetails(i32);
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IGattCharacteristicNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IGattCharacteristicNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
pub struct IGattServiceProviderConnection(pub *mut ::core::ffi::c_void);
pub struct IGattServiceProviderConnectionStatics(pub *mut ::core::ffi::c_void);
pub struct IGattServiceProviderTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IRfcommConnectionTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IRfcommInboundConnectionInformation(pub *mut ::core::ffi::c_void);
pub struct IRfcommOutboundConnectionInformation(pub *mut ::core::ffi::c_void);
pub struct RfcommConnectionTriggerDetails(i32);
pub struct RfcommInboundConnectionInformation(i32);
pub struct RfcommOutboundConnectionInformation(i32);
