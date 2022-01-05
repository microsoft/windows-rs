#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapterImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn IsClassicSupported(&self) -> ::windows::core::Result<bool>;
    fn IsLowEnergySupported(&self) -> ::windows::core::Result<bool>;
    fn IsPeripheralRoleSupported(&self) -> ::windows::core::Result<bool>;
    fn IsCentralRoleSupported(&self) -> ::windows::core::Result<bool>;
    fn IsAdvertisementOffloadSupported(&self) -> ::windows::core::Result<bool>;
    fn GetRadioAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Radios::Radio>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapter2Impl: Sized {
    fn AreClassicSecureConnectionsSupported(&self) -> ::windows::core::Result<bool>;
    fn AreLowEnergySecureConnectionsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapter3Impl: Sized {
    fn IsExtendedAdvertisingSupported(&self) -> ::windows::core::Result<bool>;
    fn MaxAdvertisementDataLength(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapterStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothClassOfDeviceImpl: Sized {
    fn RawValue(&self) -> ::windows::core::Result<u32>;
    fn MajorClass(&self) -> ::windows::core::Result<BluetoothMajorClass>;
    fn MinorClass(&self) -> ::windows::core::Result<BluetoothMinorClass>;
    fn ServiceCapabilities(&self) -> ::windows::core::Result<BluetoothServiceCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothClassOfDeviceStaticsImpl: Sized {
    fn FromRawValue(&self, rawvalue: u32) -> ::windows::core::Result<BluetoothClassOfDevice>;
    fn FromParts(&self, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities) -> ::windows::core::Result<BluetoothClassOfDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HostName(&self) -> ::windows::core::Result<super::super::Networking::HostName>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClassOfDevice(&self) -> ::windows::core::Result<BluetoothClassOfDevice>;
    fn SdpRecords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>>;
    fn RfcommServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn NameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SdpRecordsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSdpRecordsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice2Impl: Sized {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice3Impl: Sized {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>;
    fn GetRfcommServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesForIdAsync(&self, serviceid: &::core::option::Option<Rfcomm::RfcommServiceId>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesForIdWithCacheModeAsync(&self, serviceid: &::core::option::Option<Rfcomm::RfcommServiceId>, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice4Impl: Sized {
    fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice5Impl: Sized {
    fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceIdImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsClassicDevice(&self) -> ::windows::core::Result<bool>;
    fn IsLowEnergyDevice(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceIdStaticsImpl: Sized {
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn FromHostNameAsync(&self, hostname: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn FromBluetoothAddressAsync(&self, address: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceStatics2Impl: Sized {
    fn GetDeviceSelectorFromPairingState(&self, pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromConnectionStatus(&self, connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromDeviceName(&self, devicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddress(&self, bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromClassOfDevice(&self, classofdevice: &::core::option::Option<BluetoothClassOfDevice>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceImpl: Sized {
    fn RawValue(&self) -> ::windows::core::Result<u16>;
    fn Category(&self) -> ::windows::core::Result<u16>;
    fn SubCategory(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceCategoriesStaticsImpl: Sized {
    fn Uncategorized(&self) -> ::windows::core::Result<u16>;
    fn Phone(&self) -> ::windows::core::Result<u16>;
    fn Computer(&self) -> ::windows::core::Result<u16>;
    fn Watch(&self) -> ::windows::core::Result<u16>;
    fn Clock(&self) -> ::windows::core::Result<u16>;
    fn Display(&self) -> ::windows::core::Result<u16>;
    fn RemoteControl(&self) -> ::windows::core::Result<u16>;
    fn EyeGlasses(&self) -> ::windows::core::Result<u16>;
    fn Tag(&self) -> ::windows::core::Result<u16>;
    fn Keyring(&self) -> ::windows::core::Result<u16>;
    fn MediaPlayer(&self) -> ::windows::core::Result<u16>;
    fn BarcodeScanner(&self) -> ::windows::core::Result<u16>;
    fn Thermometer(&self) -> ::windows::core::Result<u16>;
    fn HeartRate(&self) -> ::windows::core::Result<u16>;
    fn BloodPressure(&self) -> ::windows::core::Result<u16>;
    fn HumanInterfaceDevice(&self) -> ::windows::core::Result<u16>;
    fn GlucoseMeter(&self) -> ::windows::core::Result<u16>;
    fn RunningWalking(&self) -> ::windows::core::Result<u16>;
    fn Cycling(&self) -> ::windows::core::Result<u16>;
    fn PulseOximeter(&self) -> ::windows::core::Result<u16>;
    fn WeightScale(&self) -> ::windows::core::Result<u16>;
    fn OutdoorSportActivity(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceStaticsImpl: Sized {
    fn FromRawValue(&self, rawvalue: u16) -> ::windows::core::Result<BluetoothLEAppearance>;
    fn FromParts(&self, appearancecategory: u16, appearancesubcategory: u16) -> ::windows::core::Result<BluetoothLEAppearance>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceSubcategoriesStaticsImpl: Sized {
    fn Generic(&self) -> ::windows::core::Result<u16>;
    fn SportsWatch(&self) -> ::windows::core::Result<u16>;
    fn ThermometerEar(&self) -> ::windows::core::Result<u16>;
    fn HeartRateBelt(&self) -> ::windows::core::Result<u16>;
    fn BloodPressureArm(&self) -> ::windows::core::Result<u16>;
    fn BloodPressureWrist(&self) -> ::windows::core::Result<u16>;
    fn Keyboard(&self) -> ::windows::core::Result<u16>;
    fn Mouse(&self) -> ::windows::core::Result<u16>;
    fn Joystick(&self) -> ::windows::core::Result<u16>;
    fn Gamepad(&self) -> ::windows::core::Result<u16>;
    fn DigitizerTablet(&self) -> ::windows::core::Result<u16>;
    fn CardReader(&self) -> ::windows::core::Result<u16>;
    fn DigitalPen(&self) -> ::windows::core::Result<u16>;
    fn BarcodeScanner(&self) -> ::windows::core::Result<u16>;
    fn RunningWalkingInShoe(&self) -> ::windows::core::Result<u16>;
    fn RunningWalkingOnShoe(&self) -> ::windows::core::Result<u16>;
    fn RunningWalkingOnHip(&self) -> ::windows::core::Result<u16>;
    fn CyclingComputer(&self) -> ::windows::core::Result<u16>;
    fn CyclingSpeedSensor(&self) -> ::windows::core::Result<u16>;
    fn CyclingCadenceSensor(&self) -> ::windows::core::Result<u16>;
    fn CyclingPowerSensor(&self) -> ::windows::core::Result<u16>;
    fn CyclingSpeedCadenceSensor(&self) -> ::windows::core::Result<u16>;
    fn OximeterFingertip(&self) -> ::windows::core::Result<u16>;
    fn OximeterWristWorn(&self) -> ::windows::core::Result<u16>;
    fn LocationDisplay(&self) -> ::windows::core::Result<u16>;
    fn LocationNavigationDisplay(&self) -> ::windows::core::Result<u16>;
    fn LocationPod(&self) -> ::windows::core::Result<u16>;
    fn LocationNavigationPod(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionParametersImpl: Sized {
    fn LinkTimeout(&self) -> ::windows::core::Result<u16>;
    fn ConnectionLatency(&self) -> ::windows::core::Result<u16>;
    fn ConnectionInterval(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionPhyImpl: Sized {
    fn TransmitInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo>;
    fn ReceiveInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionPhyInfoImpl: Sized {
    fn IsUncoded1MPhy(&self) -> ::windows::core::Result<bool>;
    fn IsUncoded2MPhy(&self) -> ::windows::core::Result<bool>;
    fn IsCodedPhy(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDeviceImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GattServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn GetGattService(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<GenericAttributeProfile::GattDeviceService>;
    fn NameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GattServicesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGattServicesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice2Impl: Sized {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
    fn Appearance(&self) -> ::windows::core::Result<BluetoothLEAppearance>;
    fn BluetoothAddressType(&self) -> ::windows::core::Result<BluetoothAddressType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice3Impl: Sized {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>;
    fn GetGattServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesForUuidAsync(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesForUuidWithCacheModeAsync(&self, serviceuuid: &::windows::core::GUID, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice4Impl: Sized {
    fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice5Impl: Sized {
    fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice6Impl: Sized {
    fn GetConnectionParameters(&self) -> ::windows::core::Result<BluetoothLEConnectionParameters>;
    fn GetConnectionPhy(&self) -> ::windows::core::Result<BluetoothLEConnectionPhy>;
    fn RequestPreferredConnectionParameters(&self, preferredconnectionparameters: &::core::option::Option<BluetoothLEPreferredConnectionParameters>) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequest>;
    fn ConnectionParametersChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionParametersChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionPhyChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionPhyChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDeviceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
    fn FromBluetoothAddressAsync(&self, bluetoothaddress: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDeviceStatics2Impl: Sized {
    fn GetDeviceSelectorFromPairingState(&self, pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromConnectionStatus(&self, connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromDeviceName(&self, devicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddress(&self, bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(&self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromAppearance(&self, appearance: &::core::option::Option<BluetoothLEAppearance>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromBluetoothAddressWithBluetoothAddressTypeAsync(&self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersImpl: Sized {
    fn LinkTimeout(&self) -> ::windows::core::Result<u16>;
    fn ConnectionLatency(&self) -> ::windows::core::Result<u16>;
    fn MinConnectionInterval(&self) -> ::windows::core::Result<u16>;
    fn MaxConnectionInterval(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersRequestImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequestStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersStaticsImpl: Sized {
    fn Balanced(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
    fn ThroughputOptimized(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
    fn PowerOptimized(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothSignalStrengthFilterImpl: Sized {
    fn InRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetInRangeThresholdInDBm(&self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn OutOfRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetOutOfRangeThresholdInDBm(&self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn OutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetOutOfRangeTimeout(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetSamplingInterval(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothUuidHelperStaticsImpl: Sized {
    fn FromShortId(&self, shortid: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn TryGetShortId(&self, uuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
