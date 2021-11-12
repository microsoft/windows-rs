#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BluetoothLEAdvertisement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisement {}
impl ::core::clone::Clone for BluetoothLEAdvertisement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementBytePattern(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementBytePattern {}
impl ::core::clone::Clone for BluetoothLEAdvertisementBytePattern {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementDataSection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementDataSection {}
impl ::core::clone::Clone for BluetoothLEAdvertisementDataSection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementFilter {}
impl ::core::clone::Clone for BluetoothLEAdvertisementFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFlags(pub u32);
impl BluetoothLEAdvertisementFlags {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const GeneralDiscoverableMode: Self = Self(2u32);
    pub const ClassicNotSupported: Self = Self(4u32);
    pub const DualModeControllerCapable: Self = Self(8u32);
    pub const DualModeHostCapable: Self = Self(16u32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementFlags {}
impl ::core::clone::Clone for BluetoothLEAdvertisementFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisher {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatus(pub i32);
impl BluetoothLEAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Waiting: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementReceivedEventArgs {}
impl ::core::clone::Clone for BluetoothLEAdvertisementReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementType(pub i32);
impl BluetoothLEAdvertisementType {
    pub const ConnectableUndirected: Self = Self(0i32);
    pub const ConnectableDirected: Self = Self(1i32);
    pub const ScannableUndirected: Self = Self(2i32);
    pub const NonConnectableUndirected: Self = Self(3i32);
    pub const ScanResponse: Self = Self(4i32);
    pub const Extended: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementType {}
impl ::core::clone::Clone for BluetoothLEAdvertisementType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcher {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStatus(pub i32);
impl BluetoothLEAdvertisementWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopping: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEManufacturerData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEManufacturerData {}
impl ::core::clone::Clone for BluetoothLEManufacturerData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEScanningMode(pub i32);
impl BluetoothLEScanningMode {
    pub const Passive: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothLEScanningMode {}
impl ::core::clone::Clone for BluetoothLEScanningMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisement {}
impl ::core::clone::Clone for IBluetoothLEAdvertisement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePattern(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementBytePattern {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementBytePattern {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePatternFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementBytePatternFactory {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementBytePatternFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementDataSection {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementDataSection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementDataSectionFactory {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementDataSectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataTypesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementDataTypesStatics {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementDataTypesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementFilter {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisher {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisher2 {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisher2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisherFactory {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementReceivedEventArgs {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementReceivedEventArgs2 {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementReceivedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementWatcher {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementWatcher2 {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcher2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementWatcherFactory {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementWatcherStoppedEventArgs {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEManufacturerData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEManufacturerData {}
impl ::core::clone::Clone for IBluetoothLEManufacturerData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEManufacturerDataFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEManufacturerDataFactory {}
impl ::core::clone::Clone for IBluetoothLEManufacturerDataFactory {
    fn clone(&self) -> Self {
        *self
    }
}
