#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BluetoothEventTriggeringMode(pub i32);
impl BluetoothEventTriggeringMode {
    pub const Serial: Self = Self(0i32);
    pub const Batch: Self = Self(1i32);
    pub const KeepLatest: Self = Self(2i32);
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattCharacteristicNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderConnectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommConnectionTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommInboundConnectionInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommOutboundConnectionInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommConnectionTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommInboundConnectionInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommOutboundConnectionInformation(pub *mut ::core::ffi::c_void);
