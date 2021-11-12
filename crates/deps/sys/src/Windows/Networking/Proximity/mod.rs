#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct PeerDiscoveryTypes(i32);
#[repr(transparent)]
pub struct PeerFinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeerInformation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PeerRole(i32);
#[repr(transparent)]
pub struct PeerWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PeerWatcherStatus(i32);
#[repr(transparent)]
pub struct ProximityDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProximityMessage(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TriggeredConnectState(i32);
#[repr(transparent)]
pub struct TriggeredConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
