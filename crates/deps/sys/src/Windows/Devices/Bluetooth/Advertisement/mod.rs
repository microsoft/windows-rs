#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFlags(pub u32);
impl BluetoothLEAdvertisementFlags {
    pub const None: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(0u32);
    pub const LimitedDiscoverableMode: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(1u32);
    pub const GeneralDiscoverableMode: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(2u32);
    pub const ClassicNotSupported: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(4u32);
    pub const DualModeControllerCapable: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(8u32);
    pub const DualModeHostCapable: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(16u32);
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatus(pub i32);
impl BluetoothLEAdvertisementPublisherStatus {
    pub const Created: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(0i32);
    pub const Waiting: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(1i32);
    pub const Started: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(2i32);
    pub const Stopping: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(3i32);
    pub const Stopped: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(4i32);
    pub const Aborted: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(5i32);
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementType(pub i32);
impl BluetoothLEAdvertisementType {
    pub const ConnectableUndirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(0i32);
    pub const ConnectableDirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(1i32);
    pub const ScannableUndirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(2i32);
    pub const NonConnectableUndirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(3i32);
    pub const ScanResponse: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(4i32);
    pub const Extended: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(5i32);
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStatus(pub i32);
impl BluetoothLEAdvertisementWatcherStatus {
    pub const Created: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(0i32);
    pub const Started: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(1i32);
    pub const Stopping: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(2i32);
    pub const Stopped: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(3i32);
    pub const Aborted: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(4i32);
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEManufacturerData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEScanningMode(pub i32);
impl BluetoothLEScanningMode {
    pub const Passive: BluetoothLEScanningMode = BluetoothLEScanningMode(0i32);
    pub const Active: BluetoothLEScanningMode = BluetoothLEScanningMode(1i32);
    pub const None: BluetoothLEScanningMode = BluetoothLEScanningMode(2i32);
}
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
