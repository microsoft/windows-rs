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
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceAccessStatus {}
impl ::core::clone::Clone for DeviceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceClass(pub i32);
impl DeviceClass {
    pub const All: Self = Self(0i32);
    pub const AudioCapture: Self = Self(1i32);
    pub const AudioRender: Self = Self(2i32);
    pub const PortableStorageDevice: Self = Self(3i32);
    pub const VideoCapture: Self = Self(4i32);
    pub const ImageScanner: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceClass {}
impl ::core::clone::Clone for DeviceClass {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
}
impl ::core::marker::Copy for DeviceInformationKind {}
impl ::core::clone::Clone for DeviceInformationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceInformationPairing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceInformationUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingKinds(pub u32);
impl DevicePairingKinds {
    pub const None: Self = Self(0u32);
    pub const ConfirmOnly: Self = Self(1u32);
    pub const DisplayPin: Self = Self(2u32);
    pub const ProvidePin: Self = Self(4u32);
    pub const ConfirmPinMatch: Self = Self(8u32);
    pub const ProvidePasswordCredential: Self = Self(16u32);
}
impl ::core::marker::Copy for DevicePairingKinds {}
impl ::core::clone::Clone for DevicePairingKinds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DevicePairingProtectionLevel(pub i32);
impl DevicePairingProtectionLevel {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Encryption: Self = Self(2i32);
    pub const EncryptionAndAuthentication: Self = Self(3i32);
}
impl ::core::marker::Copy for DevicePairingProtectionLevel {}
impl ::core::clone::Clone for DevicePairingProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DevicePairingRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePairingResultStatus(pub i32);
impl DevicePairingResultStatus {
    pub const Paired: Self = Self(0i32);
    pub const NotReadyToPair: Self = Self(1i32);
    pub const NotPaired: Self = Self(2i32);
    pub const AlreadyPaired: Self = Self(3i32);
    pub const ConnectionRejected: Self = Self(4i32);
    pub const TooManyConnections: Self = Self(5i32);
    pub const HardwareFailure: Self = Self(6i32);
    pub const AuthenticationTimeout: Self = Self(7i32);
    pub const AuthenticationNotAllowed: Self = Self(8i32);
    pub const AuthenticationFailure: Self = Self(9i32);
    pub const NoSupportedProfiles: Self = Self(10i32);
    pub const ProtectionLevelCouldNotBeMet: Self = Self(11i32);
    pub const AccessDenied: Self = Self(12i32);
    pub const InvalidCeremonyData: Self = Self(13i32);
    pub const PairingCanceled: Self = Self(14i32);
    pub const OperationAlreadyInProgress: Self = Self(15i32);
    pub const RequiredHandlerNotRegistered: Self = Self(16i32);
    pub const RejectedByHandler: Self = Self(17i32);
    pub const RemoteDeviceHasAssociation: Self = Self(18i32);
    pub const Failed: Self = Self(19i32);
}
impl ::core::marker::Copy for DevicePairingResultStatus {}
impl ::core::clone::Clone for DevicePairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePickerAppearance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePickerDisplayStatusOptions(pub u32);
impl DevicePickerDisplayStatusOptions {
    pub const None: Self = Self(0u32);
    pub const ShowProgress: Self = Self(1u32);
    pub const ShowDisconnectButton: Self = Self(2u32);
    pub const ShowRetryButton: Self = Self(4u32);
}
impl ::core::marker::Copy for DevicePickerDisplayStatusOptions {}
impl ::core::clone::Clone for DevicePickerDisplayStatusOptions {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Unpaired: Self = Self(0i32);
    pub const AlreadyUnpaired: Self = Self(1i32);
    pub const OperationAlreadyInProgress: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl ::core::marker::Copy for DeviceUnpairingResultStatus {}
impl ::core::clone::Clone for DeviceUnpairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceWatcherEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceWatcherEventKind(pub i32);
impl DeviceWatcherEventKind {
    pub const Add: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Remove: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceWatcherEventKind {}
impl ::core::clone::Clone for DeviceWatcherEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceWatcherStatus(pub i32);
impl DeviceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DeviceWatcherStatus {}
impl ::core::clone::Clone for DeviceWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Unknown: Self = Self(0i32);
    pub const Front: Self = Self(1i32);
    pub const Back: Self = Self(2i32);
    pub const Top: Self = Self(3i32);
    pub const Bottom: Self = Self(4i32);
    pub const Left: Self = Self(5i32);
    pub const Right: Self = Self(6i32);
}
impl ::core::marker::Copy for Panel {}
impl ::core::clone::Clone for Panel {
    fn clone(&self) -> Self {
        *self
    }
}
