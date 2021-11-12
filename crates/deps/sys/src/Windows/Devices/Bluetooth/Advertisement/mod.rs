#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BluetoothLEAdvertisement(i32);
pub struct BluetoothLEAdvertisementBytePattern(i32);
pub struct BluetoothLEAdvertisementDataSection(i32);
pub struct BluetoothLEAdvertisementDataTypes(i32);
pub struct BluetoothLEAdvertisementFilter(i32);
pub struct BluetoothLEAdvertisementFlags(i32);
pub struct BluetoothLEAdvertisementPublisher(i32);
pub struct BluetoothLEAdvertisementPublisherStatus(i32);
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(i32);
pub struct BluetoothLEAdvertisementReceivedEventArgs(i32);
pub struct BluetoothLEAdvertisementType(i32);
pub struct BluetoothLEAdvertisementWatcher(i32);
pub struct BluetoothLEAdvertisementWatcherStatus(i32);
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(i32);
pub struct BluetoothLEManufacturerData(i32);
pub struct BluetoothLEScanningMode(i32);
pub struct IBluetoothLEAdvertisement(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementBytePattern(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementBytePatternFactory(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementDataSection(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementDataSectionFactory(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementDataTypesStatics(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementFilter(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementPublisher(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementPublisher2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementPublisherFactory(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementReceivedEventArgs2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementWatcher(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementWatcher2(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementWatcherFactory(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEManufacturerData(pub *mut ::core::ffi::c_void);
pub struct IBluetoothLEManufacturerDataFactory(pub *mut ::core::ffi::c_void);
