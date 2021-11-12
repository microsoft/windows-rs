#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ConnectionRequestedEventArgs(i32);
pub struct DeviceArrivedEventHandler(pub *mut ::core::ffi::c_void);
pub struct DeviceDepartedEventHandler(pub *mut ::core::ffi::c_void);
pub struct IConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPeerFinderStatics(pub *mut ::core::ffi::c_void);
pub struct IPeerFinderStatics2(pub *mut ::core::ffi::c_void);
pub struct IPeerInformation(pub *mut ::core::ffi::c_void);
pub struct IPeerInformation3(pub *mut ::core::ffi::c_void);
pub struct IPeerInformationWithHostAndService(pub *mut ::core::ffi::c_void);
pub struct IPeerWatcher(pub *mut ::core::ffi::c_void);
pub struct IProximityDevice(pub *mut ::core::ffi::c_void);
pub struct IProximityDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IProximityMessage(pub *mut ::core::ffi::c_void);
pub struct ITriggeredConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MessageReceivedHandler(pub *mut ::core::ffi::c_void);
pub struct MessageTransmittedHandler(pub *mut ::core::ffi::c_void);
pub struct PeerDiscoveryTypes(i32);
pub struct PeerFinder(i32);
pub struct PeerInformation(i32);
pub struct PeerRole(i32);
pub struct PeerWatcher(i32);
pub struct PeerWatcherStatus(i32);
pub struct ProximityDevice(i32);
pub struct ProximityMessage(i32);
pub struct TriggeredConnectState(i32);
pub struct TriggeredConnectionStateChangedEventArgs(i32);
