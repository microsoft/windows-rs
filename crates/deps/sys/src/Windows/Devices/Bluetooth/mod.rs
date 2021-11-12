#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct BluetoothAddressType(pub i32);
impl BluetoothAddressType {
    pub const Public: BluetoothAddressType = BluetoothAddressType(0i32);
    pub const Random: BluetoothAddressType = BluetoothAddressType(1i32);
    pub const Unspecified: BluetoothAddressType = BluetoothAddressType(2i32);
}
#[repr(transparent)]
pub struct BluetoothCacheMode(pub i32);
impl BluetoothCacheMode {
    pub const Cached: BluetoothCacheMode = BluetoothCacheMode(0i32);
    pub const Uncached: BluetoothCacheMode = BluetoothCacheMode(1i32);
}
#[repr(transparent)]
pub struct BluetoothClassOfDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothConnectionStatus(pub i32);
impl BluetoothConnectionStatus {
    pub const Disconnected: BluetoothConnectionStatus = BluetoothConnectionStatus(0i32);
    pub const Connected: BluetoothConnectionStatus = BluetoothConnectionStatus(1i32);
}
#[repr(transparent)]
pub struct BluetoothDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothError(pub i32);
impl BluetoothError {
    pub const Success: BluetoothError = BluetoothError(0i32);
    pub const RadioNotAvailable: BluetoothError = BluetoothError(1i32);
    pub const ResourceInUse: BluetoothError = BluetoothError(2i32);
    pub const DeviceNotConnected: BluetoothError = BluetoothError(3i32);
    pub const OtherError: BluetoothError = BluetoothError(4i32);
    pub const DisabledByPolicy: BluetoothError = BluetoothError(5i32);
    pub const NotSupported: BluetoothError = BluetoothError(6i32);
    pub const DisabledByUser: BluetoothError = BluetoothError(7i32);
    pub const ConsentRequired: BluetoothError = BluetoothError(8i32);
    pub const TransportNotSupported: BluetoothError = BluetoothError(9i32);
}
#[repr(transparent)]
pub struct BluetoothLEAppearance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEConnectionPhy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEConnectionPhyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(pub i32);
impl BluetoothLEPreferredConnectionParametersRequestStatus {
    pub const Unspecified: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(0i32);
    pub const Success: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(1i32);
    pub const DeviceNotAvailable: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(2i32);
    pub const AccessDenied: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(3i32);
}
#[repr(transparent)]
pub struct BluetoothMajorClass(pub i32);
impl BluetoothMajorClass {
    pub const Miscellaneous: BluetoothMajorClass = BluetoothMajorClass(0i32);
    pub const Computer: BluetoothMajorClass = BluetoothMajorClass(1i32);
    pub const Phone: BluetoothMajorClass = BluetoothMajorClass(2i32);
    pub const NetworkAccessPoint: BluetoothMajorClass = BluetoothMajorClass(3i32);
    pub const AudioVideo: BluetoothMajorClass = BluetoothMajorClass(4i32);
    pub const Peripheral: BluetoothMajorClass = BluetoothMajorClass(5i32);
    pub const Imaging: BluetoothMajorClass = BluetoothMajorClass(6i32);
    pub const Wearable: BluetoothMajorClass = BluetoothMajorClass(7i32);
    pub const Toy: BluetoothMajorClass = BluetoothMajorClass(8i32);
    pub const Health: BluetoothMajorClass = BluetoothMajorClass(9i32);
}
#[repr(transparent)]
pub struct BluetoothMinorClass(pub i32);
impl BluetoothMinorClass {
    pub const Uncategorized: BluetoothMinorClass = BluetoothMinorClass(0i32);
    pub const ComputerDesktop: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const ComputerServer: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const ComputerLaptop: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const ComputerHandheld: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const ComputerPalmSize: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const ComputerWearable: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const ComputerTablet: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const PhoneCellular: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const PhoneCordless: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const PhoneSmartPhone: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const PhoneWired: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const PhoneIsdn: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const NetworkFullyAvailable: BluetoothMinorClass = BluetoothMinorClass(0i32);
    pub const NetworkUsed01To17Percent: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const NetworkUsed17To33Percent: BluetoothMinorClass = BluetoothMinorClass(16i32);
    pub const NetworkUsed33To50Percent: BluetoothMinorClass = BluetoothMinorClass(24i32);
    pub const NetworkUsed50To67Percent: BluetoothMinorClass = BluetoothMinorClass(32i32);
    pub const NetworkUsed67To83Percent: BluetoothMinorClass = BluetoothMinorClass(40i32);
    pub const NetworkUsed83To99Percent: BluetoothMinorClass = BluetoothMinorClass(48i32);
    pub const NetworkNoServiceAvailable: BluetoothMinorClass = BluetoothMinorClass(56i32);
    pub const AudioVideoWearableHeadset: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const AudioVideoHandsFree: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const AudioVideoMicrophone: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const AudioVideoLoudspeaker: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const AudioVideoHeadphones: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const AudioVideoPortableAudio: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const AudioVideoCarAudio: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const AudioVideoSetTopBox: BluetoothMinorClass = BluetoothMinorClass(9i32);
    pub const AudioVideoHifiAudioDevice: BluetoothMinorClass = BluetoothMinorClass(10i32);
    pub const AudioVideoVcr: BluetoothMinorClass = BluetoothMinorClass(11i32);
    pub const AudioVideoVideoCamera: BluetoothMinorClass = BluetoothMinorClass(12i32);
    pub const AudioVideoCamcorder: BluetoothMinorClass = BluetoothMinorClass(13i32);
    pub const AudioVideoVideoMonitor: BluetoothMinorClass = BluetoothMinorClass(14i32);
    pub const AudioVideoVideoDisplayAndLoudspeaker: BluetoothMinorClass = BluetoothMinorClass(15i32);
    pub const AudioVideoVideoConferencing: BluetoothMinorClass = BluetoothMinorClass(16i32);
    pub const AudioVideoGamingOrToy: BluetoothMinorClass = BluetoothMinorClass(18i32);
    pub const PeripheralJoystick: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const PeripheralGamepad: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const PeripheralRemoteControl: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const PeripheralSensing: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const PeripheralDigitizerTablet: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const PeripheralCardReader: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const PeripheralDigitalPen: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const PeripheralHandheldScanner: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const PeripheralHandheldGesture: BluetoothMinorClass = BluetoothMinorClass(9i32);
    pub const WearableWristwatch: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const WearablePager: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const WearableJacket: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const WearableHelmet: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const WearableGlasses: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const ToyRobot: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const ToyVehicle: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const ToyDoll: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const ToyController: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const ToyGame: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const HealthBloodPressureMonitor: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const HealthThermometer: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const HealthWeighingScale: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const HealthGlucoseMeter: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const HealthPulseOximeter: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const HealthHeartRateMonitor: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const HealthHealthDataDisplay: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const HealthStepCounter: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const HealthBodyCompositionAnalyzer: BluetoothMinorClass = BluetoothMinorClass(9i32);
    pub const HealthPeakFlowMonitor: BluetoothMinorClass = BluetoothMinorClass(10i32);
    pub const HealthMedicationMonitor: BluetoothMinorClass = BluetoothMinorClass(11i32);
    pub const HealthKneeProsthesis: BluetoothMinorClass = BluetoothMinorClass(12i32);
    pub const HealthAnkleProsthesis: BluetoothMinorClass = BluetoothMinorClass(13i32);
    pub const HealthGenericHealthManager: BluetoothMinorClass = BluetoothMinorClass(14i32);
    pub const HealthPersonalMobilityDevice: BluetoothMinorClass = BluetoothMinorClass(15i32);
}
#[repr(transparent)]
pub struct BluetoothServiceCapabilities(pub u32);
impl BluetoothServiceCapabilities {
    pub const None: BluetoothServiceCapabilities = BluetoothServiceCapabilities(0u32);
    pub const LimitedDiscoverableMode: BluetoothServiceCapabilities = BluetoothServiceCapabilities(1u32);
    pub const PositioningService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(8u32);
    pub const NetworkingService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(16u32);
    pub const RenderingService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(32u32);
    pub const CapturingService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(64u32);
    pub const ObjectTransferService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(128u32);
    pub const AudioService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(256u32);
    pub const TelephoneService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(512u32);
    pub const InformationService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(1024u32);
}
#[repr(transparent)]
pub struct BluetoothSignalStrengthFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothAdapterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothClassOfDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothClassOfDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDevice5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceIdStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothDeviceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearanceCategoriesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearanceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDevice6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothSignalStrengthFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothUuidHelperStatics(pub *mut ::core::ffi::c_void);
