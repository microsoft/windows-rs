#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceAccessInformation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DeviceAccessStatus(i32);
#[repr(C)]
pub struct DeviceClass(i32);
#[repr(transparent)]
pub struct DeviceConnectionChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceDisconnectButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceInformationCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceInformationCustomPairing(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DeviceInformationKind(i32);
#[repr(transparent)]
pub struct DeviceInformationPairing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceInformationUpdate(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DevicePairingKinds(i32);
#[repr(C)]
pub struct DevicePairingProtectionLevel(i32);
#[repr(transparent)]
pub struct DevicePairingRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DevicePairingResultStatus(i32);
#[repr(transparent)]
pub struct DevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePickerAppearance(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DevicePickerDisplayStatusOptions(i32);
#[repr(transparent)]
pub struct DevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceThumbnail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceUnpairingResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DeviceUnpairingResultStatus(i32);
#[repr(transparent)]
pub struct DeviceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceWatcherEvent(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DeviceWatcherEventKind(i32);
#[repr(C)]
pub struct DeviceWatcherStatus(i32);
#[repr(transparent)]
pub struct DeviceWatcherTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnclosureLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceAccessChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceAccessInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceAccessInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceConnectionChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceDisconnectButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationCustomPairing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationPairing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationPairing2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationPairingStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationPairingStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceInformationUpdate2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePairingRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePairingRequestedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePairingResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePairingSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePickerAppearance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceUnpairingResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceWatcher2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceWatcherEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceWatcherTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnclosureLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnclosureLocation2(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Panel(i32);
