#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BluetoothLEAdvertisement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementBytePattern(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementDataSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFilter(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BluetoothLEAdvertisementFlags(i32);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BluetoothLEAdvertisementPublisherStatus(i32);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BluetoothLEAdvertisementType(i32);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BluetoothLEAdvertisementWatcherStatus(i32);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEManufacturerData(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BluetoothLEScanningMode(i32);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePattern(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePatternFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataTypesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEManufacturerData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEManufacturerDataFactory(pub *mut ::core::ffi::c_void);
