#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceAccessInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceAccessStatus(pub i32);
impl DeviceAccessStatus {
    pub const Unspecified: DeviceAccessStatus = DeviceAccessStatus(0i32);
    pub const Allowed: DeviceAccessStatus = DeviceAccessStatus(1i32);
    pub const DeniedByUser: DeviceAccessStatus = DeviceAccessStatus(2i32);
    pub const DeniedBySystem: DeviceAccessStatus = DeviceAccessStatus(3i32);
}
#[repr(transparent)]
pub struct DeviceClass(pub i32);
impl DeviceClass {
    pub const All: DeviceClass = DeviceClass(0i32);
    pub const AudioCapture: DeviceClass = DeviceClass(1i32);
    pub const AudioRender: DeviceClass = DeviceClass(2i32);
    pub const PortableStorageDevice: DeviceClass = DeviceClass(3i32);
    pub const VideoCapture: DeviceClass = DeviceClass(4i32);
    pub const ImageScanner: DeviceClass = DeviceClass(5i32);
    pub const Location: DeviceClass = DeviceClass(6i32);
}
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
#[repr(transparent)]
pub struct DeviceInformationKind(pub i32);
impl DeviceInformationKind {
    pub const Unknown: DeviceInformationKind = DeviceInformationKind(0i32);
    pub const DeviceInterface: DeviceInformationKind = DeviceInformationKind(1i32);
    pub const DeviceContainer: DeviceInformationKind = DeviceInformationKind(2i32);
    pub const Device: DeviceInformationKind = DeviceInformationKind(3i32);
    pub const DeviceInterfaceClass: DeviceInformationKind = DeviceInformationKind(4i32);
    pub const AssociationEndpoint: DeviceInformationKind = DeviceInformationKind(5i32);
    pub const AssociationEndpointContainer: DeviceInformationKind = DeviceInformationKind(6i32);
    pub const AssociationEndpointService: DeviceInformationKind = DeviceInformationKind(7i32);
    pub const DevicePanel: DeviceInformationKind = DeviceInformationKind(8i32);
}
#[repr(transparent)]
pub struct DeviceInformationPairing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceInformationUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingKinds(pub u32);
impl DevicePairingKinds {
    pub const None: DevicePairingKinds = DevicePairingKinds(0u32);
    pub const ConfirmOnly: DevicePairingKinds = DevicePairingKinds(1u32);
    pub const DisplayPin: DevicePairingKinds = DevicePairingKinds(2u32);
    pub const ProvidePin: DevicePairingKinds = DevicePairingKinds(4u32);
    pub const ConfirmPinMatch: DevicePairingKinds = DevicePairingKinds(8u32);
    pub const ProvidePasswordCredential: DevicePairingKinds = DevicePairingKinds(16u32);
}
#[repr(transparent)]
pub struct DevicePairingProtectionLevel(pub i32);
impl DevicePairingProtectionLevel {
    pub const Default: DevicePairingProtectionLevel = DevicePairingProtectionLevel(0i32);
    pub const None: DevicePairingProtectionLevel = DevicePairingProtectionLevel(1i32);
    pub const Encryption: DevicePairingProtectionLevel = DevicePairingProtectionLevel(2i32);
    pub const EncryptionAndAuthentication: DevicePairingProtectionLevel = DevicePairingProtectionLevel(3i32);
}
#[repr(transparent)]
pub struct DevicePairingRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingResultStatus(pub i32);
impl DevicePairingResultStatus {
    pub const Paired: DevicePairingResultStatus = DevicePairingResultStatus(0i32);
    pub const NotReadyToPair: DevicePairingResultStatus = DevicePairingResultStatus(1i32);
    pub const NotPaired: DevicePairingResultStatus = DevicePairingResultStatus(2i32);
    pub const AlreadyPaired: DevicePairingResultStatus = DevicePairingResultStatus(3i32);
    pub const ConnectionRejected: DevicePairingResultStatus = DevicePairingResultStatus(4i32);
    pub const TooManyConnections: DevicePairingResultStatus = DevicePairingResultStatus(5i32);
    pub const HardwareFailure: DevicePairingResultStatus = DevicePairingResultStatus(6i32);
    pub const AuthenticationTimeout: DevicePairingResultStatus = DevicePairingResultStatus(7i32);
    pub const AuthenticationNotAllowed: DevicePairingResultStatus = DevicePairingResultStatus(8i32);
    pub const AuthenticationFailure: DevicePairingResultStatus = DevicePairingResultStatus(9i32);
    pub const NoSupportedProfiles: DevicePairingResultStatus = DevicePairingResultStatus(10i32);
    pub const ProtectionLevelCouldNotBeMet: DevicePairingResultStatus = DevicePairingResultStatus(11i32);
    pub const AccessDenied: DevicePairingResultStatus = DevicePairingResultStatus(12i32);
    pub const InvalidCeremonyData: DevicePairingResultStatus = DevicePairingResultStatus(13i32);
    pub const PairingCanceled: DevicePairingResultStatus = DevicePairingResultStatus(14i32);
    pub const OperationAlreadyInProgress: DevicePairingResultStatus = DevicePairingResultStatus(15i32);
    pub const RequiredHandlerNotRegistered: DevicePairingResultStatus = DevicePairingResultStatus(16i32);
    pub const RejectedByHandler: DevicePairingResultStatus = DevicePairingResultStatus(17i32);
    pub const RemoteDeviceHasAssociation: DevicePairingResultStatus = DevicePairingResultStatus(18i32);
    pub const Failed: DevicePairingResultStatus = DevicePairingResultStatus(19i32);
}
#[repr(transparent)]
pub struct DevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePickerAppearance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePickerDisplayStatusOptions(pub u32);
impl DevicePickerDisplayStatusOptions {
    pub const None: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(0u32);
    pub const ShowProgress: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(1u32);
    pub const ShowDisconnectButton: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(2u32);
    pub const ShowRetryButton: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(4u32);
}
#[repr(transparent)]
pub struct DevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceThumbnail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceUnpairingResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceUnpairingResultStatus(pub i32);
impl DeviceUnpairingResultStatus {
    pub const Unpaired: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(0i32);
    pub const AlreadyUnpaired: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(1i32);
    pub const OperationAlreadyInProgress: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(2i32);
    pub const AccessDenied: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(3i32);
    pub const Failed: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(4i32);
}
#[repr(transparent)]
pub struct DeviceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceWatcherEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceWatcherEventKind(pub i32);
impl DeviceWatcherEventKind {
    pub const Add: DeviceWatcherEventKind = DeviceWatcherEventKind(0i32);
    pub const Update: DeviceWatcherEventKind = DeviceWatcherEventKind(1i32);
    pub const Remove: DeviceWatcherEventKind = DeviceWatcherEventKind(2i32);
}
#[repr(transparent)]
pub struct DeviceWatcherStatus(pub i32);
impl DeviceWatcherStatus {
    pub const Created: DeviceWatcherStatus = DeviceWatcherStatus(0i32);
    pub const Started: DeviceWatcherStatus = DeviceWatcherStatus(1i32);
    pub const EnumerationCompleted: DeviceWatcherStatus = DeviceWatcherStatus(2i32);
    pub const Stopping: DeviceWatcherStatus = DeviceWatcherStatus(3i32);
    pub const Stopped: DeviceWatcherStatus = DeviceWatcherStatus(4i32);
    pub const Aborted: DeviceWatcherStatus = DeviceWatcherStatus(5i32);
}
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
#[repr(transparent)]
pub struct Panel(pub i32);
impl Panel {
    pub const Unknown: Panel = Panel(0i32);
    pub const Front: Panel = Panel(1i32);
    pub const Back: Panel = Panel(2i32);
    pub const Top: Panel = Panel(3i32);
    pub const Bottom: Panel = Panel(4i32);
    pub const Left: Panel = Panel(5i32);
    pub const Right: Panel = Panel(6i32);
}
