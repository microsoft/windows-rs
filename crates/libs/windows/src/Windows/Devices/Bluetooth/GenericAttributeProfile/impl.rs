#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicImpl: Sized {
    fn GetDescriptors(&self, descriptoruuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
    fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&self) -> ::windows::core::Result<u16>;
    fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>;
    fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn WriteValueAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn WriteValueWithOptionAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn ReadClientCharacteristicConfigurationDescriptorAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>>;
    fn WriteClientCharacteristicConfigurationDescriptorAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn ValueChanged(&self, valuechangedhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, valuechangedeventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristic2Impl: Sized + IGattCharacteristicImpl {
    fn Service(&self) -> ::windows::core::Result<GattDeviceService>;
    fn GetAllDescriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristic3Impl: Sized {
    fn GetDescriptorsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsForUuidAsync(&self, descriptoruuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsForUuidWithCacheModeAsync(&self, descriptoruuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn WriteValueWithResultAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
    fn WriteValueWithResultAndOptionAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
    fn WriteClientCharacteristicConfigurationDescriptorWithResultAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicStaticsImpl: Sized {
    fn ConvertShortIdToUuid(&self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicUuidsStaticsImpl: Sized {
    fn BatteryLevel(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressureFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressureMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BodySensorLocation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CscFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CscMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseMeasurementContext(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRateControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRateMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IntermediateCuffPressure(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IntermediateTemperature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn MeasurementInterval(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RecordAccessControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RscFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RscMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SCControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SensorLocation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TemperatureMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TemperatureType(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicUuidsStatics2Impl: Sized {
    fn AlertCategoryId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertCategoryIdBitMask(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertLevel(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertNotificationControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertStatus(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapAppearance(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootKeyboardInputReport(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootKeyboardOutputReport(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootMouseInputReport(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CurrentTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerVector(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DateTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DayDateTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DayOfWeek(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapDeviceName(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DstOffset(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ExactTime256(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn FirmwareRevisionString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HardwareRevisionString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HidControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HidInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Ieee1107320601RegulatoryCertificationDataList(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LnControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LnFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocalTimeInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocationAndSpeed(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ManufacturerNameString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ModelNumberString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Navigation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn NewAlert(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapPeripheralPreferredConnectionParameters(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapPeripheralPrivacyFlag(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PnpId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PositionQuality(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ProtocolMode(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapReconnectionAddress(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReferenceTimeInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Report(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReportMap(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RingerControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RingerSetting(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanIntervalWindow(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanRefresh(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SerialNumberString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GattServiceChanged(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SoftwareRevisionString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportedNewAlertCategory(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportUnreadAlertCategory(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SystemId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeAccuracy(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeSource(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeUpdateControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeUpdateState(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeWithDst(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeZone(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TxPowerLevel(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn UnreadAlertStatus(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattClientNotificationResultImpl: Sized {
    fn SubscribedClient(&self) -> ::windows::core::Result<GattSubscribedClient>;
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattClientNotificationResult2Impl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorImpl: Sized {
    fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&self) -> ::windows::core::Result<u16>;
    fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn WriteValueAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptor2Impl: Sized {
    fn WriteValueWithResultAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorStaticsImpl: Sized {
    fn ConvertShortIdToUuid(&self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorUuidsStaticsImpl: Sized {
    fn CharacteristicAggregateFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicExtendedProperties(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicPresentationFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicUserDescription(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ClientCharacteristicConfiguration(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServerCharacteristicConfiguration(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattDeviceServiceImpl: Sized + IClosableImpl {
    fn GetCharacteristics(&self, characteristicuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
    fn GetIncludedServices(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattDeviceService2Impl: Sized + IClosableImpl + IGattDeviceServiceImpl {
    fn Device(&self) -> ::windows::core::Result<super::BluetoothLEDevice>;
    fn ParentServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
    fn GetAllCharacteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
    fn GetAllIncludedServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceService3Impl: Sized {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation>;
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn SharingMode(&self) -> ::windows::core::Result<GattSharingMode>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>;
    fn OpenAsync(&self, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>>;
    fn GetCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsForUuidAsync(&self, characteristicuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsForUuidWithCacheModeAsync(&self, characteristicuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetIncludedServicesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesForUuidAsync(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesForUuidWithCacheModeAsync(&self, serviceuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceServiceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>;
    fn GetDeviceSelectorFromUuid(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromShortId(&self, serviceshortid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConvertShortIdToUuid(&self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceServiceStatics2Impl: Sized {
    fn FromIdWithSharingModeAsync(&self, deviceid: &::windows::core::HSTRING, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>;
    fn GetDeviceSelectorForBluetoothDeviceId(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdWithCacheMode(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdAndUuid(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, serviceuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceServicesResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalCharacteristicImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn CreateDescriptorAsync(&self, descriptoruuid: &::windows::core::GUID, parameters: &::core::option::Option<GattLocalDescriptorParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>>;
    fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>;
    fn SubscribedClients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>>;
    fn SubscribedClientsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubscribedClientsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReadRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WriteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWriteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyValueAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>>;
    fn NotifyValueForSubscribedClientAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, subscribedclient: &::core::option::Option<GattSubscribedClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalCharacteristicParametersImpl: Sized {
    fn SetStaticValue(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetCharacteristicProperties(&self, value: GattCharacteristicProperties) -> ::windows::core::Result<()>;
    fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetUserDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalCharacteristicResultImpl: Sized {
    fn Characteristic(&self) -> ::windows::core::Result<GattLocalCharacteristic>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalDescriptorImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn ReadRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WriteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWriteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalDescriptorParametersImpl: Sized {
    fn SetStaticValue(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalDescriptorResultImpl: Sized {
    fn Descriptor(&self) -> ::windows::core::Result<GattLocalDescriptor>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalServiceImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CreateCharacteristicAsync(&self, characteristicuuid: &::windows::core::GUID, parameters: &::core::option::Option<GattLocalCharacteristicParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatImpl: Sized {
    fn FormatType(&self) -> ::windows::core::Result<u8>;
    fn Exponent(&self) -> ::windows::core::Result<i32>;
    fn Unit(&self) -> ::windows::core::Result<u16>;
    fn Namespace(&self) -> ::windows::core::Result<u8>;
    fn Description(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatStaticsImpl: Sized {
    fn BluetoothSigAssignedNumbers(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatStatics2Impl: Sized + IGattPresentationFormatStaticsImpl {
    fn FromParts(&self, formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16) -> ::windows::core::Result<GattPresentationFormat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatTypesStaticsImpl: Sized {
    fn Boolean(&self) -> ::windows::core::Result<u8>;
    fn Bit2(&self) -> ::windows::core::Result<u8>;
    fn Nibble(&self) -> ::windows::core::Result<u8>;
    fn UInt8(&self) -> ::windows::core::Result<u8>;
    fn UInt12(&self) -> ::windows::core::Result<u8>;
    fn UInt16(&self) -> ::windows::core::Result<u8>;
    fn UInt24(&self) -> ::windows::core::Result<u8>;
    fn UInt32(&self) -> ::windows::core::Result<u8>;
    fn UInt48(&self) -> ::windows::core::Result<u8>;
    fn UInt64(&self) -> ::windows::core::Result<u8>;
    fn UInt128(&self) -> ::windows::core::Result<u8>;
    fn SInt8(&self) -> ::windows::core::Result<u8>;
    fn SInt12(&self) -> ::windows::core::Result<u8>;
    fn SInt16(&self) -> ::windows::core::Result<u8>;
    fn SInt24(&self) -> ::windows::core::Result<u8>;
    fn SInt32(&self) -> ::windows::core::Result<u8>;
    fn SInt48(&self) -> ::windows::core::Result<u8>;
    fn SInt64(&self) -> ::windows::core::Result<u8>;
    fn SInt128(&self) -> ::windows::core::Result<u8>;
    fn Float32(&self) -> ::windows::core::Result<u8>;
    fn Float64(&self) -> ::windows::core::Result<u8>;
    fn SFloat(&self) -> ::windows::core::Result<u8>;
    fn Float(&self) -> ::windows::core::Result<u8>;
    fn DUInt16(&self) -> ::windows::core::Result<u8>;
    fn Utf8(&self) -> ::windows::core::Result<u8>;
    fn Utf16(&self) -> ::windows::core::Result<u8>;
    fn Struct(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattProtocolErrorStaticsImpl: Sized {
    fn InvalidHandle(&self) -> ::windows::core::Result<u8>;
    fn ReadNotPermitted(&self) -> ::windows::core::Result<u8>;
    fn WriteNotPermitted(&self) -> ::windows::core::Result<u8>;
    fn InvalidPdu(&self) -> ::windows::core::Result<u8>;
    fn InsufficientAuthentication(&self) -> ::windows::core::Result<u8>;
    fn RequestNotSupported(&self) -> ::windows::core::Result<u8>;
    fn InvalidOffset(&self) -> ::windows::core::Result<u8>;
    fn InsufficientAuthorization(&self) -> ::windows::core::Result<u8>;
    fn PrepareQueueFull(&self) -> ::windows::core::Result<u8>;
    fn AttributeNotFound(&self) -> ::windows::core::Result<u8>;
    fn AttributeNotLong(&self) -> ::windows::core::Result<u8>;
    fn InsufficientEncryptionKeySize(&self) -> ::windows::core::Result<u8>;
    fn InvalidAttributeValueLength(&self) -> ::windows::core::Result<u8>;
    fn UnlikelyError(&self) -> ::windows::core::Result<u8>;
    fn InsufficientEncryption(&self) -> ::windows::core::Result<u8>;
    fn UnsupportedGroupType(&self) -> ::windows::core::Result<u8>;
    fn InsufficientResources(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadClientCharacteristicConfigurationDescriptorResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ClientCharacteristicConfigurationDescriptor(&self) -> ::windows::core::Result<GattClientCharacteristicConfigurationDescriptorValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadClientCharacteristicConfigurationDescriptorResult2Impl: Sized {
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadRequestImpl: Sized {
    fn Offset(&self) -> ::windows::core::Result<u32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn State(&self) -> ::windows::core::Result<GattRequestState>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RespondWithValue(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadRequestedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
    fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadRequest>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadResult2Impl: Sized {
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReliableWriteTransactionImpl: Sized {
    fn WriteValue(&self, characteristic: &::core::option::Option<GattCharacteristic>, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReliableWriteTransaction2Impl: Sized {
    fn CommitWithResultAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattRequestStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<GattRequestState>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderImpl: Sized {
    fn Service(&self) -> ::windows::core::Result<GattLocalService>;
    fn AdvertisementStatus(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus>;
    fn AdvertisementStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvertisementStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAdvertising(&self) -> ::windows::core::Result<()>;
    fn StartAdvertisingWithParameters(&self, parameters: &::core::option::Option<GattServiceProviderAdvertisingParameters>) -> ::windows::core::Result<()>;
    fn StopAdvertising(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisementStatusChangedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Status(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisingParametersImpl: Sized {
    fn SetIsConnectable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConnectable(&self) -> ::windows::core::Result<bool>;
    fn SetIsDiscoverable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDiscoverable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisingParameters2Impl: Sized {
    fn SetServiceData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ServiceData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderResultImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn ServiceProvider(&self) -> ::windows::core::Result<GattServiceProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderStaticsImpl: Sized {
    fn CreateAsync(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceUuidsStaticsImpl: Sized {
    fn Battery(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressure(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingSpeedAndCadence(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GenericAccess(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GenericAttribute(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Glucose(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HealthThermometer(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRate(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RunningSpeedAndCadence(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceUuidsStatics2Impl: Sized {
    fn AlertNotification(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CurrentTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPower(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DeviceInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HumanInterfaceDevice(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ImmediateAlert(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LinkLoss(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocationAndNavigation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn NextDstChange(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PhoneAlertStatus(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReferenceTimeUpdate(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanParameters(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TxPower(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSessionImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<super::BluetoothDeviceId>;
    fn CanMaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn MaxPduSize(&self) -> ::windows::core::Result<u16>;
    fn SessionStatus(&self) -> ::windows::core::Result<GattSessionStatus>;
    fn MaxPduSizeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMaxPduSizeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SessionStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSessionStaticsImpl: Sized {
    fn FromDeviceIdAsync(&self, deviceid: &::core::option::Option<super::BluetoothDeviceId>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattSession>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSessionStatusChangedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Status(&self) -> ::windows::core::Result<GattSessionStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSubscribedClientImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn MaxNotificationSize(&self) -> ::windows::core::Result<u16>;
    fn MaxNotificationSizeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMaxNotificationSizeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattValueChangedEventArgsImpl: Sized {
    fn CharacteristicValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattWriteRequestImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Offset(&self) -> ::windows::core::Result<u32>;
    fn Option(&self) -> ::windows::core::Result<GattWriteOption>;
    fn State(&self) -> ::windows::core::Result<GattRequestState>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Respond(&self) -> ::windows::core::Result<()>;
    fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattWriteRequestedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
    fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattWriteResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
