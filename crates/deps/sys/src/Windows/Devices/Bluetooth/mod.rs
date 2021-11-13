#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Bluetooth_Advertisement")]
pub mod Advertisement;
#[cfg(feature = "Devices_Bluetooth_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
pub mod GenericAttributeProfile;
#[cfg(feature = "Devices_Bluetooth_Rfcomm")]
pub mod Rfcomm;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BluetoothAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothAdapter {}
impl ::core::clone::Clone for BluetoothAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothAddressType(pub i32);
impl BluetoothAddressType {
    pub const Public: Self = Self(0i32);
    pub const Random: Self = Self(1i32);
    pub const Unspecified: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothAddressType {}
impl ::core::clone::Clone for BluetoothAddressType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothCacheMode(pub i32);
impl BluetoothCacheMode {
    pub const Cached: Self = Self(0i32);
    pub const Uncached: Self = Self(1i32);
}
impl ::core::marker::Copy for BluetoothCacheMode {}
impl ::core::clone::Clone for BluetoothCacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothClassOfDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothClassOfDevice {}
impl ::core::clone::Clone for BluetoothClassOfDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothConnectionStatus(pub i32);
impl BluetoothConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl ::core::marker::Copy for BluetoothConnectionStatus {}
impl ::core::clone::Clone for BluetoothConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothDevice {}
impl ::core::clone::Clone for BluetoothDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothDeviceId {}
impl ::core::clone::Clone for BluetoothDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothError(pub i32);
impl BluetoothError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const DeviceNotConnected: Self = Self(3i32);
    pub const OtherError: Self = Self(4i32);
    pub const DisabledByPolicy: Self = Self(5i32);
    pub const NotSupported: Self = Self(6i32);
    pub const DisabledByUser: Self = Self(7i32);
    pub const ConsentRequired: Self = Self(8i32);
    pub const TransportNotSupported: Self = Self(9i32);
}
impl ::core::marker::Copy for BluetoothError {}
impl ::core::clone::Clone for BluetoothError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAppearance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAppearance {}
impl ::core::clone::Clone for BluetoothLEAppearance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEConnectionParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEConnectionParameters {}
impl ::core::clone::Clone for BluetoothLEConnectionParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEConnectionPhy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEConnectionPhy {}
impl ::core::clone::Clone for BluetoothLEConnectionPhy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEConnectionPhyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEConnectionPhyInfo {}
impl ::core::clone::Clone for BluetoothLEConnectionPhyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEDevice {}
impl ::core::clone::Clone for BluetoothLEDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEPreferredConnectionParameters {}
impl ::core::clone::Clone for BluetoothLEPreferredConnectionParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEPreferredConnectionParametersRequest {}
impl ::core::clone::Clone for BluetoothLEPreferredConnectionParametersRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(pub i32);
impl BluetoothLEPreferredConnectionParametersRequestStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for BluetoothLEPreferredConnectionParametersRequestStatus {}
impl ::core::clone::Clone for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothMajorClass(pub i32);
impl BluetoothMajorClass {
    pub const Miscellaneous: Self = Self(0i32);
    pub const Computer: Self = Self(1i32);
    pub const Phone: Self = Self(2i32);
    pub const NetworkAccessPoint: Self = Self(3i32);
    pub const AudioVideo: Self = Self(4i32);
    pub const Peripheral: Self = Self(5i32);
    pub const Imaging: Self = Self(6i32);
    pub const Wearable: Self = Self(7i32);
    pub const Toy: Self = Self(8i32);
    pub const Health: Self = Self(9i32);
}
impl ::core::marker::Copy for BluetoothMajorClass {}
impl ::core::clone::Clone for BluetoothMajorClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothMinorClass(pub i32);
impl BluetoothMinorClass {
    pub const Uncategorized: Self = Self(0i32);
    pub const ComputerDesktop: Self = Self(1i32);
    pub const ComputerServer: Self = Self(2i32);
    pub const ComputerLaptop: Self = Self(3i32);
    pub const ComputerHandheld: Self = Self(4i32);
    pub const ComputerPalmSize: Self = Self(5i32);
    pub const ComputerWearable: Self = Self(6i32);
    pub const ComputerTablet: Self = Self(7i32);
    pub const PhoneCellular: Self = Self(1i32);
    pub const PhoneCordless: Self = Self(2i32);
    pub const PhoneSmartPhone: Self = Self(3i32);
    pub const PhoneWired: Self = Self(4i32);
    pub const PhoneIsdn: Self = Self(5i32);
    pub const NetworkFullyAvailable: Self = Self(0i32);
    pub const NetworkUsed01To17Percent: Self = Self(8i32);
    pub const NetworkUsed17To33Percent: Self = Self(16i32);
    pub const NetworkUsed33To50Percent: Self = Self(24i32);
    pub const NetworkUsed50To67Percent: Self = Self(32i32);
    pub const NetworkUsed67To83Percent: Self = Self(40i32);
    pub const NetworkUsed83To99Percent: Self = Self(48i32);
    pub const NetworkNoServiceAvailable: Self = Self(56i32);
    pub const AudioVideoWearableHeadset: Self = Self(1i32);
    pub const AudioVideoHandsFree: Self = Self(2i32);
    pub const AudioVideoMicrophone: Self = Self(4i32);
    pub const AudioVideoLoudspeaker: Self = Self(5i32);
    pub const AudioVideoHeadphones: Self = Self(6i32);
    pub const AudioVideoPortableAudio: Self = Self(7i32);
    pub const AudioVideoCarAudio: Self = Self(8i32);
    pub const AudioVideoSetTopBox: Self = Self(9i32);
    pub const AudioVideoHifiAudioDevice: Self = Self(10i32);
    pub const AudioVideoVcr: Self = Self(11i32);
    pub const AudioVideoVideoCamera: Self = Self(12i32);
    pub const AudioVideoCamcorder: Self = Self(13i32);
    pub const AudioVideoVideoMonitor: Self = Self(14i32);
    pub const AudioVideoVideoDisplayAndLoudspeaker: Self = Self(15i32);
    pub const AudioVideoVideoConferencing: Self = Self(16i32);
    pub const AudioVideoGamingOrToy: Self = Self(18i32);
    pub const PeripheralJoystick: Self = Self(1i32);
    pub const PeripheralGamepad: Self = Self(2i32);
    pub const PeripheralRemoteControl: Self = Self(3i32);
    pub const PeripheralSensing: Self = Self(4i32);
    pub const PeripheralDigitizerTablet: Self = Self(5i32);
    pub const PeripheralCardReader: Self = Self(6i32);
    pub const PeripheralDigitalPen: Self = Self(7i32);
    pub const PeripheralHandheldScanner: Self = Self(8i32);
    pub const PeripheralHandheldGesture: Self = Self(9i32);
    pub const WearableWristwatch: Self = Self(1i32);
    pub const WearablePager: Self = Self(2i32);
    pub const WearableJacket: Self = Self(3i32);
    pub const WearableHelmet: Self = Self(4i32);
    pub const WearableGlasses: Self = Self(5i32);
    pub const ToyRobot: Self = Self(1i32);
    pub const ToyVehicle: Self = Self(2i32);
    pub const ToyDoll: Self = Self(3i32);
    pub const ToyController: Self = Self(4i32);
    pub const ToyGame: Self = Self(5i32);
    pub const HealthBloodPressureMonitor: Self = Self(1i32);
    pub const HealthThermometer: Self = Self(2i32);
    pub const HealthWeighingScale: Self = Self(3i32);
    pub const HealthGlucoseMeter: Self = Self(4i32);
    pub const HealthPulseOximeter: Self = Self(5i32);
    pub const HealthHeartRateMonitor: Self = Self(6i32);
    pub const HealthHealthDataDisplay: Self = Self(7i32);
    pub const HealthStepCounter: Self = Self(8i32);
    pub const HealthBodyCompositionAnalyzer: Self = Self(9i32);
    pub const HealthPeakFlowMonitor: Self = Self(10i32);
    pub const HealthMedicationMonitor: Self = Self(11i32);
    pub const HealthKneeProsthesis: Self = Self(12i32);
    pub const HealthAnkleProsthesis: Self = Self(13i32);
    pub const HealthGenericHealthManager: Self = Self(14i32);
    pub const HealthPersonalMobilityDevice: Self = Self(15i32);
}
impl ::core::marker::Copy for BluetoothMinorClass {}
impl ::core::clone::Clone for BluetoothMinorClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothServiceCapabilities(pub u32);
impl BluetoothServiceCapabilities {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const PositioningService: Self = Self(8u32);
    pub const NetworkingService: Self = Self(16u32);
    pub const RenderingService: Self = Self(32u32);
    pub const CapturingService: Self = Self(64u32);
    pub const ObjectTransferService: Self = Self(128u32);
    pub const AudioService: Self = Self(256u32);
    pub const TelephoneService: Self = Self(512u32);
    pub const InformationService: Self = Self(1024u32);
}
impl ::core::marker::Copy for BluetoothServiceCapabilities {}
impl ::core::clone::Clone for BluetoothServiceCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothSignalStrengthFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothSignalStrengthFilter {}
impl ::core::clone::Clone for BluetoothSignalStrengthFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothAdapter {}
impl ::core::clone::Clone for IBluetoothAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothAdapter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothAdapter2 {}
impl ::core::clone::Clone for IBluetoothAdapter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothAdapter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothAdapter3 {}
impl ::core::clone::Clone for IBluetoothAdapter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothAdapterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothAdapterStatics {}
impl ::core::clone::Clone for IBluetoothAdapterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothClassOfDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothClassOfDevice {}
impl ::core::clone::Clone for IBluetoothClassOfDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothClassOfDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothClassOfDeviceStatics {}
impl ::core::clone::Clone for IBluetoothClassOfDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDevice {}
impl ::core::clone::Clone for IBluetoothDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDevice2 {}
impl ::core::clone::Clone for IBluetoothDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDevice3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDevice3 {}
impl ::core::clone::Clone for IBluetoothDevice3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDevice4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDevice4 {}
impl ::core::clone::Clone for IBluetoothDevice4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDevice5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDevice5 {}
impl ::core::clone::Clone for IBluetoothDevice5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDeviceId {}
impl ::core::clone::Clone for IBluetoothDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDeviceIdStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDeviceIdStatics {}
impl ::core::clone::Clone for IBluetoothDeviceIdStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDeviceStatics {}
impl ::core::clone::Clone for IBluetoothDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothDeviceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothDeviceStatics2 {}
impl ::core::clone::Clone for IBluetoothDeviceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAppearance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAppearance {}
impl ::core::clone::Clone for IBluetoothLEAppearance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAppearanceCategoriesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAppearanceCategoriesStatics {}
impl ::core::clone::Clone for IBluetoothLEAppearanceCategoriesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAppearanceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAppearanceStatics {}
impl ::core::clone::Clone for IBluetoothLEAppearanceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAppearanceSubcategoriesStatics {}
impl ::core::clone::Clone for IBluetoothLEAppearanceSubcategoriesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEConnectionParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEConnectionParameters {}
impl ::core::clone::Clone for IBluetoothLEConnectionParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEConnectionPhy {}
impl ::core::clone::Clone for IBluetoothLEConnectionPhy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEConnectionPhyInfo {}
impl ::core::clone::Clone for IBluetoothLEConnectionPhyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDevice {}
impl ::core::clone::Clone for IBluetoothLEDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDevice2 {}
impl ::core::clone::Clone for IBluetoothLEDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDevice3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDevice3 {}
impl ::core::clone::Clone for IBluetoothLEDevice3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDevice4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDevice4 {}
impl ::core::clone::Clone for IBluetoothLEDevice4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDevice5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDevice5 {}
impl ::core::clone::Clone for IBluetoothLEDevice5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDevice6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDevice6 {}
impl ::core::clone::Clone for IBluetoothLEDevice6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDeviceStatics {}
impl ::core::clone::Clone for IBluetoothLEDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEDeviceStatics2 {}
impl ::core::clone::Clone for IBluetoothLEDeviceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEPreferredConnectionParameters {}
impl ::core::clone::Clone for IBluetoothLEPreferredConnectionParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEPreferredConnectionParametersRequest {}
impl ::core::clone::Clone for IBluetoothLEPreferredConnectionParametersRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEPreferredConnectionParametersStatics {}
impl ::core::clone::Clone for IBluetoothLEPreferredConnectionParametersStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothSignalStrengthFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothSignalStrengthFilter {}
impl ::core::clone::Clone for IBluetoothSignalStrengthFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothUuidHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothUuidHelperStatics {}
impl ::core::clone::Clone for IBluetoothUuidHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
