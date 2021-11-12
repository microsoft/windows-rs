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
impl ::core::marker::Copy for BluetoothEventTriggeringMode {}
impl ::core::clone::Clone for BluetoothEventTriggeringMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisherTriggerDetails {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcherTriggerDetails {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattCharacteristicNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattCharacteristicNotificationTriggerDetails {}
impl ::core::clone::Clone for GattCharacteristicNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProviderConnection {}
impl ::core::clone::Clone for GattServiceProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProviderTriggerDetails {}
impl ::core::clone::Clone for GattServiceProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisherTriggerDetails {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisherTriggerDetails2 {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementWatcherTriggerDetails {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicNotificationTriggerDetails {}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicNotificationTriggerDetails2 {}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderConnection {}
impl ::core::clone::Clone for IGattServiceProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderConnectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderConnectionStatics {}
impl ::core::clone::Clone for IGattServiceProviderConnectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderTriggerDetails {}
impl ::core::clone::Clone for IGattServiceProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRfcommConnectionTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRfcommConnectionTriggerDetails {}
impl ::core::clone::Clone for IRfcommConnectionTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRfcommInboundConnectionInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRfcommInboundConnectionInformation {}
impl ::core::clone::Clone for IRfcommInboundConnectionInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRfcommOutboundConnectionInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRfcommOutboundConnectionInformation {}
impl ::core::clone::Clone for IRfcommOutboundConnectionInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RfcommConnectionTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RfcommConnectionTriggerDetails {}
impl ::core::clone::Clone for RfcommConnectionTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RfcommInboundConnectionInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RfcommInboundConnectionInformation {}
impl ::core::clone::Clone for RfcommInboundConnectionInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RfcommOutboundConnectionInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RfcommOutboundConnectionInformation {}
impl ::core::clone::Clone for RfcommOutboundConnectionInformation {
    fn clone(&self) -> Self {
        *self
    }
}
