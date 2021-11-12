#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceArrivedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceDepartedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerFinderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerFinderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerInformation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerInformationWithHostAndService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximityDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximityDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximityMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITriggeredConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageReceivedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageTransmittedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeerDiscoveryTypes(pub u32);
impl PeerDiscoveryTypes {
    pub const None: PeerDiscoveryTypes = PeerDiscoveryTypes(0u32);
    pub const Browse: PeerDiscoveryTypes = PeerDiscoveryTypes(1u32);
    pub const Triggered: PeerDiscoveryTypes = PeerDiscoveryTypes(2u32);
}
#[repr(transparent)]
pub struct PeerInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeerRole(pub i32);
impl PeerRole {
    pub const Peer: PeerRole = PeerRole(0i32);
    pub const Host: PeerRole = PeerRole(1i32);
    pub const Client: PeerRole = PeerRole(2i32);
}
#[repr(transparent)]
pub struct PeerWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeerWatcherStatus(pub i32);
impl PeerWatcherStatus {
    pub const Created: PeerWatcherStatus = PeerWatcherStatus(0i32);
    pub const Started: PeerWatcherStatus = PeerWatcherStatus(1i32);
    pub const EnumerationCompleted: PeerWatcherStatus = PeerWatcherStatus(2i32);
    pub const Stopping: PeerWatcherStatus = PeerWatcherStatus(3i32);
    pub const Stopped: PeerWatcherStatus = PeerWatcherStatus(4i32);
    pub const Aborted: PeerWatcherStatus = PeerWatcherStatus(5i32);
}
#[repr(transparent)]
pub struct ProximityDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProximityMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriggeredConnectState(pub i32);
impl TriggeredConnectState {
    pub const PeerFound: TriggeredConnectState = TriggeredConnectState(0i32);
    pub const Listening: TriggeredConnectState = TriggeredConnectState(1i32);
    pub const Connecting: TriggeredConnectState = TriggeredConnectState(2i32);
    pub const Completed: TriggeredConnectState = TriggeredConnectState(3i32);
    pub const Canceled: TriggeredConnectState = TriggeredConnectState(4i32);
    pub const Failed: TriggeredConnectState = TriggeredConnectState(5i32);
}
#[repr(transparent)]
pub struct TriggeredConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
