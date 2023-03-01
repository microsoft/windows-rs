#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristic {
    type Vtable = IGattCharacteristic_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59cb50c1_5934_4f68_a198_eb864fa44e6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetDescriptors: usize,
    pub CharacteristicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub SetProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub UserDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
    #[cfg(feature = "Foundation")]
    pub ReadValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadValueWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueWithCacheModeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, writeoption: GattWriteOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadClientCharacteristicConfigurationDescriptorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadClientCharacteristicConfigurationDescriptorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteClientCharacteristicConfigurationDescriptorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteClientCharacteristicConfigurationDescriptorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuechangedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuechangedeventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristic2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristic2 {
    type Vtable = IGattCharacteristic2_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristic2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristic2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae1ab578_ec06_4764_b780_9835a1d35d6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristic2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllDescriptors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristic3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristic3 {
    type Vtable = IGattCharacteristic3_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristic3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristic3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f3c663e_93d4_406b_b817_db81f8ed53b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristic3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsForUuidAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsForUuidAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDescriptorsForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDescriptorsForUuidWithCacheModeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithResultAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithResultAndOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, writeoption: GattWriteOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithResultAndOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteClientCharacteristicConfigurationDescriptorWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteClientCharacteristicConfigurationDescriptorWithResultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicStatics {
    type Vtable = IGattCharacteristicStatics_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59cb50c3_5934_4f68_a198_eb864fa44e6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicUuidsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicUuidsStatics {
    type Vtable = IGattCharacteristicUuidsStatics_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicUuidsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicUuidsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58fa4586_b1de_470c_b7de_0d11ff44f4b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicUuidsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BatteryLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BloodPressureFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BloodPressureMeasurement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BodySensorLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CscFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CscMeasurement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GlucoseFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GlucoseMeasurement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GlucoseMeasurementContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HeartRateControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HeartRateMeasurement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IntermediateCuffPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IntermediateTemperature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub MeasurementInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RecordAccessControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RscFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RscMeasurement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SCControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SensorLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TemperatureMeasurement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TemperatureType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicUuidsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicUuidsStatics2 {
    type Vtable = IGattCharacteristicUuidsStatics2_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicUuidsStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicUuidsStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1855b425_d46e_4a2c_9c3f_ed6dea29e7be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicUuidsStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AlertCategoryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AlertCategoryIdBitMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AlertLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AlertNotificationControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AlertStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GapAppearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BootKeyboardInputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BootKeyboardOutputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BootMouseInputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CurrentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CyclingPowerControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CyclingPowerFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CyclingPowerMeasurement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CyclingPowerVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DayDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DayOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GapDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DstOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ExactTime256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub FirmwareRevisionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HardwareRevisionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HidControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HidInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Ieee1107320601RegulatoryCertificationDataList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub LnControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub LnFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub LocalTimeInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub LocationAndSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ManufacturerNameString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ModelNumberString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Navigation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub NewAlert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GapPeripheralPreferredConnectionParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GapPeripheralPrivacyFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PnpId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PositionQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ProtocolMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GapReconnectionAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ReferenceTimeInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ReportMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RingerControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RingerSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ScanIntervalWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ScanRefresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SerialNumberString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GattServiceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SoftwareRevisionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SupportedNewAlertCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SupportUnreadAlertCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TimeAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TimeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TimeUpdateControlPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TimeUpdateState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TimeWithDst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TxPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnreadAlertStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicsResult {
    type Vtable = IGattCharacteristicsResult_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1194945c_b257_4f3e_9db7_f68bc9a9aef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicsResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattClientNotificationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattClientNotificationResult {
    type Vtable = IGattClientNotificationResult_Vtbl;
}
impl ::core::clone::Clone for IGattClientNotificationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattClientNotificationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x506d5599_0112_419a_8e3b_ae21afabd2c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattClientNotificationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SubscribedClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattClientNotificationResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattClientNotificationResult2 {
    type Vtable = IGattClientNotificationResult2_Vtbl;
}
impl ::core::clone::Clone for IGattClientNotificationResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattClientNotificationResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8faec497_45e0_497e_9582_29a1fe281ad5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattClientNotificationResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDescriptor {
    type Vtable = IGattDescriptor_Vtbl;
}
impl ::core::clone::Clone for IGattDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92055f2b_8084_4344_b4c2_284de19a8506);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub SetProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadValueWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadValueWithCacheModeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDescriptor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDescriptor2 {
    type Vtable = IGattDescriptor2_Vtbl;
}
impl ::core::clone::Clone for IGattDescriptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDescriptor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f563d39_d630_406c_ba11_10cdd16b0e5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteValueWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteValueWithResultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDescriptorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDescriptorStatics {
    type Vtable = IGattDescriptorStatics_Vtbl;
}
impl ::core::clone::Clone for IGattDescriptorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDescriptorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92055f2d_8084_4344_b4c2_284de19a8506);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDescriptorUuidsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDescriptorUuidsStatics {
    type Vtable = IGattDescriptorUuidsStatics_Vtbl;
}
impl ::core::clone::Clone for IGattDescriptorUuidsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDescriptorUuidsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6f862ce_9cfc_42f1_9185_ff37b75181d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptorUuidsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CharacteristicAggregateFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CharacteristicExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CharacteristicPresentationFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CharacteristicUserDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ClientCharacteristicConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ServerCharacteristicConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDescriptorsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDescriptorsResult {
    type Vtable = IGattDescriptorsResult_Vtbl;
}
impl ::core::clone::Clone for IGattDescriptorsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDescriptorsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bc091f3_95e7_4489_8d25_ff81955a57b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptorsResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDeviceService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDeviceService {
    type Vtable = IGattDeviceService_Vtbl;
}
impl ::core::clone::Clone for IGattDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDeviceService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac7b7c05_b33c_47cf_990f_6b8f5577df71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceService_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCharacteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCharacteristics: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetIncludedServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetIncludedServices: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AttributeHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDeviceService2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDeviceService2 {
    type Vtable = IGattDeviceService2_Vtbl;
}
impl ::core::clone::Clone for IGattDeviceService2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDeviceService2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc54520b_0b0d_4708_bae0_9ffd9489bc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceService2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Device: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ParentServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ParentServices: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllCharacteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllCharacteristics: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetAllIncludedServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetAllIncludedServices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDeviceService3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDeviceService3 {
    type Vtable = IGattDeviceService3_Vtbl;
}
impl ::core::clone::Clone for IGattDeviceService3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDeviceService3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb293a950_0c53_437c_a9b3_5c3210c6e569);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceService3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattSharingMode) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingmode: GattSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsForUuidAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsForUuidAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacteristicsForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacteristicsForUuidWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesWithCacheModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesForUuidAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesForUuidAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIncludedServicesForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIncludedServicesForUuidWithCacheModeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDeviceServiceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDeviceServiceStatics {
    type Vtable = IGattDeviceServiceStatics_Vtbl;
}
impl ::core::clone::Clone for IGattDeviceServiceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDeviceServiceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196d0022_faad_45dc_ae5b_2ac3184e84db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceServiceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelectorFromUuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GetDeviceSelectorFromShortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceshortid: u16, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeviceSelectorFromShortId: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertShortIdToUuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertShortIdToUuid: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDeviceServiceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDeviceServiceStatics2 {
    type Vtable = IGattDeviceServiceStatics2_Vtbl;
}
impl ::core::clone::Clone for IGattDeviceServiceStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDeviceServiceStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0604186e_24a6_4b0d_a2f2_30cc01545d25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceServiceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdWithSharingModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, sharingmode: GattSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdWithSharingModeAsync: usize,
    pub GetDeviceSelectorForBluetoothDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdeviceid: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdWithCacheMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdeviceid: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdAndUuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdeviceid: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdeviceid: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattDeviceServicesResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattDeviceServicesResult {
    type Vtable = IGattDeviceServicesResult_Vtbl;
}
impl ::core::clone::Clone for IGattDeviceServicesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattDeviceServicesResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x171dd3ee_016d_419d_838a_576cf475a3d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceServicesResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Services: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattLocalCharacteristic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattLocalCharacteristic {
    type Vtable = IGattLocalCharacteristic_Vtbl;
}
impl ::core::clone::Clone for IGattLocalCharacteristic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattLocalCharacteristic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaede376d_5412_4d74_92a8_8deb8526829c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalCharacteristic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub CharacteristicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateDescriptorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, parameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDescriptorAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Descriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Descriptors: usize,
    pub UserDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubscribedClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubscribedClients: usize,
    #[cfg(feature = "Foundation")]
    pub SubscribedClientsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubscribedClientsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubscribedClientsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubscribedClientsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ReadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub WriteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWriteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWriteRequested: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub NotifyValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    NotifyValueAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub NotifyValueForSubscribedClientAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, subscribedclient: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    NotifyValueForSubscribedClientAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattLocalCharacteristicParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattLocalCharacteristicParameters {
    type Vtable = IGattLocalCharacteristicParameters_Vtbl;
}
impl ::core::clone::Clone for IGattLocalCharacteristicParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattLocalCharacteristicParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf73db4_4cff_44c7_8445_040e6ead0063);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalCharacteristicParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetStaticValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStaticValue: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub SetCharacteristicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub CharacteristicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub SetReadProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub SetWriteProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub SetUserDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationFormats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattLocalCharacteristicResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattLocalCharacteristicResult {
    type Vtable = IGattLocalCharacteristicResult_Vtbl;
}
impl ::core::clone::Clone for IGattLocalCharacteristicResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattLocalCharacteristicResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7975de9b_0170_4397_9666_92f863f12ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalCharacteristicResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Characteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattLocalDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattLocalDescriptor {
    type Vtable = IGattLocalDescriptor_Vtbl;
}
impl ::core::clone::Clone for IGattLocalDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattLocalDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf48ebe06_789d_4a4b_8652_bd017b5d2fc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub WriteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWriteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWriteRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattLocalDescriptorParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattLocalDescriptorParameters {
    type Vtable = IGattLocalDescriptorParameters_Vtbl;
}
impl ::core::clone::Clone for IGattLocalDescriptorParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattLocalDescriptorParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fdede6a_f3c1_4b66_8c4b_e3d2293b40e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalDescriptorParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetStaticValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStaticValue: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StaticValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StaticValue: usize,
    pub SetReadProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub ReadProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub SetWriteProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub WriteProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattLocalDescriptorResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattLocalDescriptorResult {
    type Vtable = IGattLocalDescriptorResult_Vtbl;
}
impl ::core::clone::Clone for IGattLocalDescriptorResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattLocalDescriptorResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x375791be_321f_4366_bfc1_3bc6b82c79f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalDescriptorResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Descriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattLocalService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattLocalService {
    type Vtable = IGattLocalService_Vtbl;
}
impl ::core::clone::Clone for IGattLocalService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattLocalService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf513e258_f7f7_4902_b803_57fcc7d6fe83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalService_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateCharacteristicAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, parameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateCharacteristicAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattPresentationFormat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattPresentationFormat {
    type Vtable = IGattPresentationFormat_Vtbl;
}
impl ::core::clone::Clone for IGattPresentationFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattPresentationFormat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196d0021_faad_45dc_ae5b_2ac3184e84db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormat_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FormatType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Exponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattPresentationFormatStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattPresentationFormatStatics {
    type Vtable = IGattPresentationFormatStatics_Vtbl;
}
impl ::core::clone::Clone for IGattPresentationFormatStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattPresentationFormatStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196d0020_faad_45dc_ae5b_2ac3184e84db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormatStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BluetoothSigAssignedNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattPresentationFormatStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattPresentationFormatStatics2 {
    type Vtable = IGattPresentationFormatStatics2_Vtbl;
}
impl ::core::clone::Clone for IGattPresentationFormatStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattPresentationFormatStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9c21713_b82f_435e_b634_21fd85a43c07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormatStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FromParts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattPresentationFormatTypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattPresentationFormatTypesStatics {
    type Vtable = IGattPresentationFormatTypesStatics_Vtbl;
}
impl ::core::clone::Clone for IGattPresentationFormatTypesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattPresentationFormatTypesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf1ba0a_30ba_409c_bef7_cffb6d03b8fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormatTypesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Boolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Bit2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Nibble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt24: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt48: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UInt128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt24: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt48: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SInt128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Float32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Float64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SFloat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Float: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub DUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Utf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Utf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Struct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattProtocolErrorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattProtocolErrorStatics {
    type Vtable = IGattProtocolErrorStatics_Vtbl;
}
impl ::core::clone::Clone for IGattProtocolErrorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattProtocolErrorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca46c5c5_0ecc_4809_bea3_cf79bc991e37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattProtocolErrorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InvalidHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ReadNotPermitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub WriteNotPermitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InvalidPdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InsufficientAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub RequestNotSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InvalidOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InsufficientAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub PrepareQueueFull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub AttributeNotFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub AttributeNotLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InsufficientEncryptionKeySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InvalidAttributeValueLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UnlikelyError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InsufficientEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub UnsupportedGroupType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub InsufficientResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReadClientCharacteristicConfigurationDescriptorResult {
    type Vtable = IGattReadClientCharacteristicConfigurationDescriptorResult_Vtbl;
}
impl ::core::clone::Clone for IGattReadClientCharacteristicConfigurationDescriptorResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReadClientCharacteristicConfigurationDescriptorResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f09_1aea_4c4c_a50f_97bae474b348);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    pub ClientCharacteristicConfigurationDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    type Vtable = IGattReadClientCharacteristicConfigurationDescriptorResult2_Vtbl;
}
impl ::core::clone::Clone for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bf1a59d_ba4d_4622_8651_f4ee150d0a5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReadRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReadRequest {
    type Vtable = IGattReadRequest_Vtbl;
}
impl ::core::clone::Clone for IGattReadRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReadRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1dd6535_6acd_42a6_a4bb_d789dae0043e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattRequestState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RespondWithValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RespondWithValue: usize,
    pub RespondWithProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocolerror: u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReadRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReadRequestedEventArgs {
    type Vtable = IGattReadRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGattReadRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReadRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93497243_f39c_484b_8ab6_996ba486cfa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Foundation")]
    pub GetRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReadResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReadResult {
    type Vtable = IGattReadResult_Vtbl;
}
impl ::core::clone::Clone for IGattReadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReadResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f08_1aea_4c4c_a50f_97bae474b348);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReadResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReadResult2 {
    type Vtable = IGattReadResult2_Vtbl;
}
impl ::core::clone::Clone for IGattReadResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReadResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa10f50a0_fb43_48af_baaa_638a5c6329fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReliableWriteTransaction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReliableWriteTransaction {
    type Vtable = IGattReliableWriteTransaction_Vtbl;
}
impl ::core::clone::Clone for IGattReliableWriteTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReliableWriteTransaction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f07_1aea_4c4c_a50f_97bae474b348);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReliableWriteTransaction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub WriteValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristic: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteValue: usize,
    #[cfg(feature = "Foundation")]
    pub CommitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommitAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattReliableWriteTransaction2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattReliableWriteTransaction2 {
    type Vtable = IGattReliableWriteTransaction2_Vtbl;
}
impl ::core::clone::Clone for IGattReliableWriteTransaction2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattReliableWriteTransaction2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51113987_ef12_462f_9fb2_a1a43a679416);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReliableWriteTransaction2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CommitWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommitWithResultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattRequestStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattRequestStateChangedEventArgs {
    type Vtable = IGattRequestStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGattRequestStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattRequestStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe834d92c_27be_44b3_9d0d_4fc6e808dd3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattRequestStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattRequestState) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProvider {
    type Vtable = IGattServiceProvider_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7822b3cd_2889_4f86_a051_3f0aed1c2760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AdvertisementStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AdvertisementStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvertisementStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvertisementStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvertisementStatusChanged: usize,
    pub StartAdvertising: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartAdvertisingWithParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopAdvertising: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    type Vtable = IGattServiceProviderAdvertisementStatusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59a5aa65_fa21_4ffc_b155_04d928012686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisingParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderAdvertisingParameters {
    type Vtable = IGattServiceProviderAdvertisingParameters_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderAdvertisingParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderAdvertisingParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ce31ab_6315_4c22_9bd7_781dbc3d8d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisingParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetIsConnectable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsConnectable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDiscoverable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDiscoverable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisingParameters2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderAdvertisingParameters2 {
    type Vtable = IGattServiceProviderAdvertisingParameters2_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderAdvertisingParameters2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderAdvertisingParameters2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff68468d_ca92_4434_9743_0e90988ad879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisingParameters2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetServiceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetServiceData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ServiceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ServiceData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderResult {
    type Vtable = IGattServiceProviderResult_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x764696d8_c53e_428c_8a48_67afe02c3ae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub ServiceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderStatics {
    type Vtable = IGattServiceProviderStatics_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31794063_5256_4054_a4f4_7bbe7755a57e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceUuidsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceUuidsStatics {
    type Vtable = IGattServiceUuidsStatics_Vtbl;
}
impl ::core::clone::Clone for IGattServiceUuidsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceUuidsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc57058_9aba_4417_b8f2_dce016d34ee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceUuidsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Battery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BloodPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CyclingSpeedAndCadence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GenericAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GenericAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Glucose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HealthThermometer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HeartRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RunningSpeedAndCadence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceUuidsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceUuidsStatics2 {
    type Vtable = IGattServiceUuidsStatics2_Vtbl;
}
impl ::core::clone::Clone for IGattServiceUuidsStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceUuidsStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ae94f5_3d15_4f79_9c0c_eaafa675155c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceUuidsStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AlertNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CurrentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CyclingPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub HumanInterfaceDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ImmediateAlert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub LinkLoss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub LocationAndNavigation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub NextDstChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PhoneAlertStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ReferenceTimeUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ScanParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TxPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattSession {
    type Vtable = IGattSession_Vtbl;
}
impl ::core::clone::Clone for IGattSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd23b5143_e04e_4c24_999c_9c256f9856b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxPduSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SessionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattSessionStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxPduSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPduSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMaxPduSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMaxPduSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SessionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattSessionStatics {
    type Vtable = IGattSessionStatics_Vtbl;
}
impl ::core::clone::Clone for IGattSessionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattSessionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e65b95c_539f_4db7_82a8_73bdbbf73ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSessionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromDeviceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromDeviceIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattSessionStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattSessionStatusChangedEventArgs {
    type Vtable = IGattSessionStatusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGattSessionStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattSessionStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7605b72e_837f_404c_ab34_3163f39ddf32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSessionStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattSessionStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattSubscribedClient(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattSubscribedClient {
    type Vtable = IGattSubscribedClient_Vtbl;
}
impl ::core::clone::Clone for IGattSubscribedClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattSubscribedClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x736e9001_15a4_4ec2_9248_e3f20d463be9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSubscribedClient_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxNotificationSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxNotificationSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxNotificationSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMaxNotificationSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMaxNotificationSizeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattValueChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattValueChangedEventArgs {
    type Vtable = IGattValueChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGattValueChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattValueChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21bdb54_06e3_4ed8_a263_acfac8ba7313);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattValueChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CharacteristicValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CharacteristicValue: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattWriteRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattWriteRequest {
    type Vtable = IGattWriteRequest_Vtbl;
}
impl ::core::clone::Clone for IGattWriteRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattWriteRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaeb6a9ed_de2f_4fc2_a9a8_94ea7844f13d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattWriteRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Option: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattWriteOption) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattRequestState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub Respond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RespondWithProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocolerror: u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattWriteRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattWriteRequestedEventArgs {
    type Vtable = IGattWriteRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGattWriteRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattWriteRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dec8bbe_a73a_471a_94d5_037deadd0806);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattWriteRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Foundation")]
    pub GetRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattWriteResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattWriteResult {
    type Vtable = IGattWriteResult_Vtbl;
}
impl ::core::clone::Clone for IGattWriteResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattWriteResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4991ddb1_cb2b_44f7_99fc_d29a2871dc9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattWriteResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtocolError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtocolError: usize,
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattCharacteristic(::windows::core::IUnknown);
impl GattCharacteristic {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetDescriptors(&self, descriptoruuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>();
            (::windows::core::Interface::vtable(this).GetDescriptors)(::windows::core::Interface::as_raw(this), descriptoruuid, &mut result__).from_abi(result__)
        }
    }
    pub fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCharacteristicProperties>();
            (::windows::core::Interface::vtable(this).CharacteristicProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).ProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UserDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Uuid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttributeHandle(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).AttributeHandle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>();
            (::windows::core::Interface::vtable(this).PresentationFormats)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>();
            (::windows::core::Interface::vtable(this).ReadValueAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>();
            (::windows::core::Interface::vtable(this).ReadValueWithCacheModeAsync)(::windows::core::Interface::as_raw(this), cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteValueAsync<P0>(&self, value: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>();
            (::windows::core::Interface::vtable(this).WriteValueAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteValueWithOptionAsync<P0>(&self, value: P0, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>();
            (::windows::core::Interface::vtable(this).WriteValueWithOptionAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), writeoption, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadClientCharacteristicConfigurationDescriptorAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>>();
            (::windows::core::Interface::vtable(this).ReadClientCharacteristicConfigurationDescriptorAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteClientCharacteristicConfigurationDescriptorAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>();
            (::windows::core::Interface::vtable(this).WriteClientCharacteristicConfigurationDescriptorAsync)(::windows::core::Interface::as_raw(this), clientcharacteristicconfigurationdescriptorvalue, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValueChanged(&self, valuechangedhandler: &super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ValueChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(valuechangedhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueChanged(&self, valuechangedeventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveValueChanged)(::windows::core::Interface::as_raw(this), valuechangedeventcookie).ok() }
    }
    pub fn Service(&self) -> ::windows::core::Result<GattDeviceService> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattDeviceService>();
            (::windows::core::Interface::vtable(this).Service)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetAllDescriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>();
            (::windows::core::Interface::vtable(this).GetAllDescriptors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDescriptorsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>();
            (::windows::core::Interface::vtable(this).GetDescriptorsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDescriptorsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>();
            (::windows::core::Interface::vtable(this).GetDescriptorsWithCacheModeAsync)(::windows::core::Interface::as_raw(this), cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDescriptorsForUuidAsync(&self, descriptoruuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>();
            (::windows::core::Interface::vtable(this).GetDescriptorsForUuidAsync)(::windows::core::Interface::as_raw(this), descriptoruuid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDescriptorsForUuidWithCacheModeAsync(&self, descriptoruuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>();
            (::windows::core::Interface::vtable(this).GetDescriptorsForUuidWithCacheModeAsync)(::windows::core::Interface::as_raw(this), descriptoruuid, cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteValueWithResultAsync<P0>(&self, value: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>();
            (::windows::core::Interface::vtable(this).WriteValueWithResultAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteValueWithResultAndOptionAsync<P0>(&self, value: P0, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>();
            (::windows::core::Interface::vtable(this).WriteValueWithResultAndOptionAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), writeoption, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteClientCharacteristicConfigurationDescriptorWithResultAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>();
            (::windows::core::Interface::vtable(this).WriteClientCharacteristicConfigurationDescriptorWithResultAsync)(::windows::core::Interface::as_raw(this), clientcharacteristicconfigurationdescriptorvalue, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ConvertShortIdToUuid(shortid: u16) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ConvertShortIdToUuid)(::windows::core::Interface::as_raw(this), shortid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicStatics<R, F: FnOnce(&IGattCharacteristicStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattCharacteristic, IGattCharacteristicStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattCharacteristic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattCharacteristic {}
impl ::core::fmt::Debug for GattCharacteristic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristic").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattCharacteristic {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristic;{59cb50c1-5934-4f68-a198-eb864fa44e6b})");
}
impl ::core::clone::Clone for GattCharacteristic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattCharacteristic {
    type Vtable = IGattCharacteristic_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattCharacteristic {
    const IID: ::windows::core::GUID = <IGattCharacteristic as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristic";
}
::windows::imp::interface_hierarchy!(GattCharacteristic, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattCharacteristic {}
unsafe impl ::core::marker::Sync for GattCharacteristic {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
pub struct GattCharacteristicUuids;
impl GattCharacteristicUuids {
    pub fn BatteryLevel() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BatteryLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BloodPressureFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BloodPressureFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BloodPressureMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BloodPressureMeasurement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BodySensorLocation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BodySensorLocation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CscFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CscFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CscMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CscMeasurement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GlucoseFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GlucoseFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GlucoseMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GlucoseMeasurement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GlucoseMeasurementContext() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GlucoseMeasurementContext)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HeartRateControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HeartRateControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HeartRateMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HeartRateMeasurement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IntermediateCuffPressure() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).IntermediateCuffPressure)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IntermediateTemperature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).IntermediateTemperature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MeasurementInterval() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).MeasurementInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RecordAccessControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RecordAccessControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RscFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RscFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RscMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RscMeasurement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SCControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SCControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SensorLocation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SensorLocation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TemperatureMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TemperatureMeasurement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TemperatureType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TemperatureType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AlertCategoryId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).AlertCategoryId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AlertCategoryIdBitMask() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).AlertCategoryIdBitMask)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AlertLevel() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).AlertLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AlertNotificationControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).AlertNotificationControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AlertStatus() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).AlertStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GapAppearance() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GapAppearance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BootKeyboardInputReport() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BootKeyboardInputReport)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BootKeyboardOutputReport() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BootKeyboardOutputReport)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BootMouseInputReport() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BootMouseInputReport)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CurrentTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CurrentTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CyclingPowerControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CyclingPowerControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CyclingPowerFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CyclingPowerFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CyclingPowerMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CyclingPowerMeasurement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CyclingPowerVector() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CyclingPowerVector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DateTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).DateTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DayDateTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).DayDateTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DayOfWeek() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).DayOfWeek)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GapDeviceName() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GapDeviceName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DstOffset() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).DstOffset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ExactTime256() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ExactTime256)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FirmwareRevisionString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).FirmwareRevisionString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HardwareRevisionString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HardwareRevisionString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HidControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HidControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HidInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HidInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Ieee1107320601RegulatoryCertificationDataList() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Ieee1107320601RegulatoryCertificationDataList)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LnControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LnControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LnFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LnFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LocalTimeInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LocalTimeInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LocationAndSpeed() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LocationAndSpeed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ManufacturerNameString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ManufacturerNameString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ModelNumberString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ModelNumberString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Navigation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Navigation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NewAlert() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).NewAlert)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GapPeripheralPreferredConnectionParameters() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GapPeripheralPreferredConnectionParameters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GapPeripheralPrivacyFlag() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GapPeripheralPrivacyFlag)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PnpId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).PnpId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PositionQuality() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).PositionQuality)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProtocolMode() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ProtocolMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GapReconnectionAddress() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GapReconnectionAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ReferenceTimeInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ReferenceTimeInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Report() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Report)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ReportMap() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ReportMap)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RingerControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RingerControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RingerSetting() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RingerSetting)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ScanIntervalWindow() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ScanIntervalWindow)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ScanRefresh() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ScanRefresh)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SerialNumberString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SerialNumberString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GattServiceChanged() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GattServiceChanged)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SoftwareRevisionString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SoftwareRevisionString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SupportedNewAlertCategory() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SupportedNewAlertCategory)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SupportUnreadAlertCategory() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SupportUnreadAlertCategory)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SystemId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).SystemId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TimeAccuracy() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TimeAccuracy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TimeSource() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TimeSource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TimeUpdateControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TimeUpdateControlPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TimeUpdateState() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TimeUpdateState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TimeWithDst() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TimeWithDst)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TimeZone() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TimeZone)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TxPowerLevel() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TxPowerLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UnreadAlertStatus() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).UnreadAlertStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicUuidsStatics<R, F: FnOnce(&IGattCharacteristicUuidsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattCharacteristicUuids, IGattCharacteristicUuidsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicUuidsStatics2<R, F: FnOnce(&IGattCharacteristicUuidsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattCharacteristicUuids, IGattCharacteristicUuidsStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GattCharacteristicUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicUuids";
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattCharacteristicsResult(::windows::core::IUnknown);
impl GattCharacteristicsResult {
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCommunicationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<u8>>();
            (::windows::core::Interface::vtable(this).ProtocolError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>();
            (::windows::core::Interface::vtable(this).Characteristics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattCharacteristicsResult {}
impl ::core::fmt::Debug for GattCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristicsResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattCharacteristicsResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicsResult;{1194945c-b257-4f3e-9db7-f68bc9a9aef2})");
}
impl ::core::clone::Clone for GattCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattCharacteristicsResult {
    type Vtable = IGattCharacteristicsResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattCharacteristicsResult {
    const IID: ::windows::core::GUID = <IGattCharacteristicsResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicsResult";
}
::windows::imp::interface_hierarchy!(GattCharacteristicsResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattCharacteristicsResult {}
unsafe impl ::core::marker::Sync for GattCharacteristicsResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattClientNotificationResult(::windows::core::IUnknown);
impl GattClientNotificationResult {
    pub fn SubscribedClient(&self) -> ::windows::core::Result<GattSubscribedClient> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSubscribedClient>();
            (::windows::core::Interface::vtable(this).SubscribedClient)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCommunicationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<u8>>();
            (::windows::core::Interface::vtable(this).ProtocolError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BytesSent(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::ComInterface::cast::<IGattClientNotificationResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).BytesSent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattClientNotificationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattClientNotificationResult {}
impl ::core::fmt::Debug for GattClientNotificationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattClientNotificationResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattClientNotificationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientNotificationResult;{506d5599-0112-419a-8e3b-ae21afabd2c2})");
}
impl ::core::clone::Clone for GattClientNotificationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattClientNotificationResult {
    type Vtable = IGattClientNotificationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattClientNotificationResult {
    const IID: ::windows::core::GUID = <IGattClientNotificationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattClientNotificationResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientNotificationResult";
}
::windows::imp::interface_hierarchy!(GattClientNotificationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattClientNotificationResult {}
unsafe impl ::core::marker::Sync for GattClientNotificationResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattDescriptor(::windows::core::IUnknown);
impl GattDescriptor {
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).ProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Uuid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttributeHandle(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).AttributeHandle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>();
            (::windows::core::Interface::vtable(this).ReadValueAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>();
            (::windows::core::Interface::vtable(this).ReadValueWithCacheModeAsync)(::windows::core::Interface::as_raw(this), cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteValueAsync<P0>(&self, value: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>();
            (::windows::core::Interface::vtable(this).WriteValueAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteValueWithResultAsync<P0>(&self, value: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IGattDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>();
            (::windows::core::Interface::vtable(this).WriteValueWithResultAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ConvertShortIdToUuid(shortid: u16) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ConvertShortIdToUuid)(::windows::core::Interface::as_raw(this), shortid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattDescriptorStatics<R, F: FnOnce(&IGattDescriptorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattDescriptor, IGattDescriptorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDescriptor {}
impl ::core::fmt::Debug for GattDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDescriptor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattDescriptor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptor;{92055f2b-8084-4344-b4c2-284de19a8506})");
}
impl ::core::clone::Clone for GattDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattDescriptor {
    type Vtable = IGattDescriptor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattDescriptor {
    const IID: ::windows::core::GUID = <IGattDescriptor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptor";
}
::windows::imp::interface_hierarchy!(GattDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattDescriptor {}
unsafe impl ::core::marker::Sync for GattDescriptor {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
pub struct GattDescriptorUuids;
impl GattDescriptorUuids {
    pub fn CharacteristicAggregateFormat() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CharacteristicAggregateFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CharacteristicExtendedProperties() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CharacteristicExtendedProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CharacteristicPresentationFormat() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CharacteristicPresentationFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CharacteristicUserDescription() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CharacteristicUserDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ClientCharacteristicConfiguration() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ClientCharacteristicConfiguration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ServerCharacteristicConfiguration() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ServerCharacteristicConfiguration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattDescriptorUuidsStatics<R, F: FnOnce(&IGattDescriptorUuidsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattDescriptorUuids, IGattDescriptorUuidsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GattDescriptorUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorUuids";
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattDescriptorsResult(::windows::core::IUnknown);
impl GattDescriptorsResult {
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCommunicationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<u8>>();
            (::windows::core::Interface::vtable(this).ProtocolError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>();
            (::windows::core::Interface::vtable(this).Descriptors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattDescriptorsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDescriptorsResult {}
impl ::core::fmt::Debug for GattDescriptorsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDescriptorsResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattDescriptorsResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorsResult;{9bc091f3-95e7-4489-8d25-ff81955a57b9})");
}
impl ::core::clone::Clone for GattDescriptorsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattDescriptorsResult {
    type Vtable = IGattDescriptorsResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattDescriptorsResult {
    const IID: ::windows::core::GUID = <IGattDescriptorsResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattDescriptorsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorsResult";
}
::windows::imp::interface_hierarchy!(GattDescriptorsResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattDescriptorsResult {}
unsafe impl ::core::marker::Sync for GattDescriptorsResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattDeviceService(::windows::core::IUnknown);
impl GattDeviceService {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCharacteristics(&self, characteristicuuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>();
            (::windows::core::Interface::vtable(this).GetCharacteristics)(::windows::core::Interface::as_raw(this), characteristicuuid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetIncludedServices(&self, serviceuuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>();
            (::windows::core::Interface::vtable(this).GetIncludedServices)(::windows::core::Interface::as_raw(this), serviceuuid, &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Uuid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttributeHandle(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).AttributeHandle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Device(&self) -> ::windows::core::Result<super::BluetoothLEDevice> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothLEDevice>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ParentServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>();
            (::windows::core::Interface::vtable(this).ParentServices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetAllCharacteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>();
            (::windows::core::Interface::vtable(this).GetAllCharacteristics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetAllIncludedServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>();
            (::windows::core::Interface::vtable(this).GetAllIncludedServices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Enumeration::DeviceAccessInformation>();
            (::windows::core::Interface::vtable(this).DeviceAccessInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSession>();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<GattSharingMode> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSharingMode>();
            (::windows::core::Interface::vtable(this).SharingMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>>();
            (::windows::core::Interface::vtable(this).OpenAsync)(::windows::core::Interface::as_raw(this), sharingmode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>();
            (::windows::core::Interface::vtable(this).GetCharacteristicsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCharacteristicsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>();
            (::windows::core::Interface::vtable(this).GetCharacteristicsWithCacheModeAsync)(::windows::core::Interface::as_raw(this), cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCharacteristicsForUuidAsync(&self, characteristicuuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>();
            (::windows::core::Interface::vtable(this).GetCharacteristicsForUuidAsync)(::windows::core::Interface::as_raw(this), characteristicuuid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCharacteristicsForUuidWithCacheModeAsync(&self, characteristicuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>();
            (::windows::core::Interface::vtable(this).GetCharacteristicsForUuidWithCacheModeAsync)(::windows::core::Interface::as_raw(this), characteristicuuid, cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIncludedServicesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>();
            (::windows::core::Interface::vtable(this).GetIncludedServicesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIncludedServicesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>();
            (::windows::core::Interface::vtable(this).GetIncludedServicesWithCacheModeAsync)(::windows::core::Interface::as_raw(this), cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIncludedServicesForUuidAsync(&self, serviceuuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>();
            (::windows::core::Interface::vtable(this).GetIncludedServicesForUuidAsync)(::windows::core::Interface::as_raw(this), serviceuuid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIncludedServicesForUuidWithCacheModeAsync(&self, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>();
            (::windows::core::Interface::vtable(this).GetIncludedServicesForUuidWithCacheModeAsync)(::windows::core::Interface::as_raw(this), serviceuuid, cachemode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorFromUuid(serviceuuid: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromUuid)(::windows::core::Interface::as_raw(this), serviceuuid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDeviceSelectorFromShortId(serviceshortid: u16) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromShortId)(::windows::core::Interface::as_raw(this), serviceshortid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ConvertShortIdToUuid(shortid: u16) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ConvertShortIdToUuid)(::windows::core::Interface::as_raw(this), shortid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdWithSharingModeAsync(deviceid: &::windows::core::HSTRING, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>();
            (::windows::core::Interface::vtable(this).FromIdWithSharingModeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), sharingmode, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceId(bluetoothdeviceid: &super::BluetoothDeviceId) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(bluetoothdeviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceIdWithCacheMode(bluetoothdeviceid: &super::BluetoothDeviceId, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceIdWithCacheMode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(bluetoothdeviceid), cachemode, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceIdAndUuid(bluetoothdeviceid: &super::BluetoothDeviceId, serviceuuid: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceIdAndUuid)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(bluetoothdeviceid), serviceuuid, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode(bluetoothdeviceid: &super::BluetoothDeviceId, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(bluetoothdeviceid), serviceuuid, cachemode, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattDeviceServiceStatics<R, F: FnOnce(&IGattDeviceServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattDeviceService, IGattDeviceServiceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattDeviceServiceStatics2<R, F: FnOnce(&IGattDeviceServiceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattDeviceService, IGattDeviceServiceStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDeviceService {}
impl ::core::fmt::Debug for GattDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDeviceService").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattDeviceService {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService;{ac7b7c05-b33c-47cf-990f-6b8f5577df71})");
}
impl ::core::clone::Clone for GattDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattDeviceService {
    type Vtable = IGattDeviceService_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattDeviceService {
    const IID: ::windows::core::GUID = <IGattDeviceService as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService";
}
::windows::imp::interface_hierarchy!(GattDeviceService, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::IClosable> for GattDeviceService {}
unsafe impl ::core::marker::Send for GattDeviceService {}
unsafe impl ::core::marker::Sync for GattDeviceService {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattDeviceServicesResult(::windows::core::IUnknown);
impl GattDeviceServicesResult {
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCommunicationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<u8>>();
            (::windows::core::Interface::vtable(this).ProtocolError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>();
            (::windows::core::Interface::vtable(this).Services)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattDeviceServicesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDeviceServicesResult {}
impl ::core::fmt::Debug for GattDeviceServicesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDeviceServicesResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattDeviceServicesResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceServicesResult;{171dd3ee-016d-419d-838a-576cf475a3d8})");
}
impl ::core::clone::Clone for GattDeviceServicesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattDeviceServicesResult {
    type Vtable = IGattDeviceServicesResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattDeviceServicesResult {
    const IID: ::windows::core::GUID = <IGattDeviceServicesResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceServicesResult";
}
::windows::imp::interface_hierarchy!(GattDeviceServicesResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattDeviceServicesResult {}
unsafe impl ::core::marker::Sync for GattDeviceServicesResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattLocalCharacteristic(::windows::core::IUnknown);
impl GattLocalCharacteristic {
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Uuid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).StaticValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCharacteristicProperties>();
            (::windows::core::Interface::vtable(this).CharacteristicProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).ReadProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).WriteProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateDescriptorAsync(&self, descriptoruuid: ::windows::core::GUID, parameters: &GattLocalDescriptorParameters) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>>();
            (::windows::core::Interface::vtable(this).CreateDescriptorAsync)(::windows::core::Interface::as_raw(this), descriptoruuid, ::core::mem::transmute_copy(parameters), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>>();
            (::windows::core::Interface::vtable(this).Descriptors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UserDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>();
            (::windows::core::Interface::vtable(this).PresentationFormats)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubscribedClients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>>();
            (::windows::core::Interface::vtable(this).SubscribedClients)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SubscribedClientsChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SubscribedClientsChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubscribedClientsChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSubscribedClientsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).WriteRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWriteRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveWriteRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn NotifyValueAsync<P0>(&self, value: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>>();
            (::windows::core::Interface::vtable(this).NotifyValueAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn NotifyValueForSubscribedClientAsync<P0>(&self, value: P0, subscribedclient: &GattSubscribedClient) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>>();
            (::windows::core::Interface::vtable(this).NotifyValueForSubscribedClientAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), ::core::mem::transmute_copy(subscribedclient), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattLocalCharacteristic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalCharacteristic {}
impl ::core::fmt::Debug for GattLocalCharacteristic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalCharacteristic").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattLocalCharacteristic {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristic;{aede376d-5412-4d74-92a8-8deb8526829c})");
}
impl ::core::clone::Clone for GattLocalCharacteristic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattLocalCharacteristic {
    type Vtable = IGattLocalCharacteristic_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattLocalCharacteristic {
    const IID: ::windows::core::GUID = <IGattLocalCharacteristic as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattLocalCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristic";
}
::windows::imp::interface_hierarchy!(GattLocalCharacteristic, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattLocalCharacteristic {}
unsafe impl ::core::marker::Sync for GattLocalCharacteristic {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattLocalCharacteristicParameters(::windows::core::IUnknown);
impl GattLocalCharacteristicParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattLocalCharacteristicParameters, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStaticValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStaticValue)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).StaticValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCharacteristicProperties(&self, value: GattCharacteristicProperties) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacteristicProperties)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCharacteristicProperties>();
            (::windows::core::Interface::vtable(this).CharacteristicProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReadProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).ReadProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWriteProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).WriteProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUserDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUserDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UserDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>>();
            (::windows::core::Interface::vtable(this).PresentationFormats)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattLocalCharacteristicParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalCharacteristicParameters {}
impl ::core::fmt::Debug for GattLocalCharacteristicParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalCharacteristicParameters").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattLocalCharacteristicParameters {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicParameters;{faf73db4-4cff-44c7-8445-040e6ead0063})");
}
impl ::core::clone::Clone for GattLocalCharacteristicParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattLocalCharacteristicParameters {
    type Vtable = IGattLocalCharacteristicParameters_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattLocalCharacteristicParameters {
    const IID: ::windows::core::GUID = <IGattLocalCharacteristicParameters as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattLocalCharacteristicParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicParameters";
}
::windows::imp::interface_hierarchy!(GattLocalCharacteristicParameters, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattLocalCharacteristicParameters {}
unsafe impl ::core::marker::Sync for GattLocalCharacteristicParameters {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattLocalCharacteristicResult(::windows::core::IUnknown);
impl GattLocalCharacteristicResult {
    pub fn Characteristic(&self) -> ::windows::core::Result<GattLocalCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattLocalCharacteristic>();
            (::windows::core::Interface::vtable(this).Characteristic)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattLocalCharacteristicResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalCharacteristicResult {}
impl ::core::fmt::Debug for GattLocalCharacteristicResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalCharacteristicResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattLocalCharacteristicResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicResult;{7975de9b-0170-4397-9666-92f863f12ee6})");
}
impl ::core::clone::Clone for GattLocalCharacteristicResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattLocalCharacteristicResult {
    type Vtable = IGattLocalCharacteristicResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattLocalCharacteristicResult {
    const IID: ::windows::core::GUID = <IGattLocalCharacteristicResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattLocalCharacteristicResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicResult";
}
::windows::imp::interface_hierarchy!(GattLocalCharacteristicResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattLocalCharacteristicResult {}
unsafe impl ::core::marker::Sync for GattLocalCharacteristicResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattLocalDescriptor(::windows::core::IUnknown);
impl GattLocalDescriptor {
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Uuid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).StaticValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).ReadProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).WriteProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ReadRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReadRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).WriteRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWriteRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveWriteRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for GattLocalDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalDescriptor {}
impl ::core::fmt::Debug for GattLocalDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalDescriptor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattLocalDescriptor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptor;{f48ebe06-789d-4a4b-8652-bd017b5d2fc6})");
}
impl ::core::clone::Clone for GattLocalDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattLocalDescriptor {
    type Vtable = IGattLocalDescriptor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattLocalDescriptor {
    const IID: ::windows::core::GUID = <IGattLocalDescriptor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattLocalDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptor";
}
::windows::imp::interface_hierarchy!(GattLocalDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattLocalDescriptor {}
unsafe impl ::core::marker::Sync for GattLocalDescriptor {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattLocalDescriptorParameters(::windows::core::IUnknown);
impl GattLocalDescriptorParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattLocalDescriptorParameters, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStaticValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStaticValue)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).StaticValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReadProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).ReadProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWriteProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattProtectionLevel>();
            (::windows::core::Interface::vtable(this).WriteProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattLocalDescriptorParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalDescriptorParameters {}
impl ::core::fmt::Debug for GattLocalDescriptorParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalDescriptorParameters").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattLocalDescriptorParameters {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorParameters;{5fdede6a-f3c1-4b66-8c4b-e3d2293b40e9})");
}
impl ::core::clone::Clone for GattLocalDescriptorParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattLocalDescriptorParameters {
    type Vtable = IGattLocalDescriptorParameters_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattLocalDescriptorParameters {
    const IID: ::windows::core::GUID = <IGattLocalDescriptorParameters as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattLocalDescriptorParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorParameters";
}
::windows::imp::interface_hierarchy!(GattLocalDescriptorParameters, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattLocalDescriptorParameters {}
unsafe impl ::core::marker::Sync for GattLocalDescriptorParameters {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattLocalDescriptorResult(::windows::core::IUnknown);
impl GattLocalDescriptorResult {
    pub fn Descriptor(&self) -> ::windows::core::Result<GattLocalDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattLocalDescriptor>();
            (::windows::core::Interface::vtable(this).Descriptor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattLocalDescriptorResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalDescriptorResult {}
impl ::core::fmt::Debug for GattLocalDescriptorResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalDescriptorResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattLocalDescriptorResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorResult;{375791be-321f-4366-bfc1-3bc6b82c79f8})");
}
impl ::core::clone::Clone for GattLocalDescriptorResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattLocalDescriptorResult {
    type Vtable = IGattLocalDescriptorResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattLocalDescriptorResult {
    const IID: ::windows::core::GUID = <IGattLocalDescriptorResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattLocalDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorResult";
}
::windows::imp::interface_hierarchy!(GattLocalDescriptorResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattLocalDescriptorResult {}
unsafe impl ::core::marker::Sync for GattLocalDescriptorResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattLocalService(::windows::core::IUnknown);
impl GattLocalService {
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Uuid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateCharacteristicAsync(&self, characteristicuuid: ::windows::core::GUID, parameters: &GattLocalCharacteristicParameters) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>>();
            (::windows::core::Interface::vtable(this).CreateCharacteristicAsync)(::windows::core::Interface::as_raw(this), characteristicuuid, ::core::mem::transmute_copy(parameters), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>>();
            (::windows::core::Interface::vtable(this).Characteristics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattLocalService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalService {}
impl ::core::fmt::Debug for GattLocalService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalService").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattLocalService {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalService;{f513e258-f7f7-4902-b803-57fcc7d6fe83})");
}
impl ::core::clone::Clone for GattLocalService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattLocalService {
    type Vtable = IGattLocalService_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattLocalService {
    const IID: ::windows::core::GUID = <IGattLocalService as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattLocalService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalService";
}
::windows::imp::interface_hierarchy!(GattLocalService, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattLocalService {}
unsafe impl ::core::marker::Sync for GattLocalService {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattPresentationFormat(::windows::core::IUnknown);
impl GattPresentationFormat {
    pub fn FormatType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).FormatType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Exponent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Exponent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Unit(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).Unit)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Namespace(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Namespace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BluetoothSigAssignedNumbers() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).BluetoothSigAssignedNumbers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FromParts(formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16) -> ::windows::core::Result<GattPresentationFormat> {
        Self::IGattPresentationFormatStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GattPresentationFormat>();
            (::windows::core::Interface::vtable(this).FromParts)(::windows::core::Interface::as_raw(this), formattype, exponent, unit, namespaceid, description, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattPresentationFormatStatics<R, F: FnOnce(&IGattPresentationFormatStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattPresentationFormat, IGattPresentationFormatStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattPresentationFormatStatics2<R, F: FnOnce(&IGattPresentationFormatStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattPresentationFormat, IGattPresentationFormatStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattPresentationFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattPresentationFormat {}
impl ::core::fmt::Debug for GattPresentationFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattPresentationFormat").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattPresentationFormat {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormat;{196d0021-faad-45dc-ae5b-2ac3184e84db})");
}
impl ::core::clone::Clone for GattPresentationFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattPresentationFormat {
    type Vtable = IGattPresentationFormat_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattPresentationFormat {
    const IID: ::windows::core::GUID = <IGattPresentationFormat as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattPresentationFormat {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormat";
}
::windows::imp::interface_hierarchy!(GattPresentationFormat, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattPresentationFormat {}
unsafe impl ::core::marker::Sync for GattPresentationFormat {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
pub struct GattPresentationFormatTypes;
impl GattPresentationFormatTypes {
    pub fn Boolean() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Boolean)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Bit2() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Bit2)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Nibble() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Nibble)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt8() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt8)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt12() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt12)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt16)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt24() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt24)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt32() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt32)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt48() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt48)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt64() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt64)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UInt128() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UInt128)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt8() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt8)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt12() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt12)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt16)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt24() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt24)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt32() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt32)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt48() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt48)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt64() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt64)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SInt128() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SInt128)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Float32() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Float32)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Float64() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Float64)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SFloat() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SFloat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Float() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Float)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DUInt16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).DUInt16)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Utf8() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Utf8)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Utf16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Utf16)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Struct() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).Struct)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattPresentationFormatTypesStatics<R, F: FnOnce(&IGattPresentationFormatTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattPresentationFormatTypes, IGattPresentationFormatTypesStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GattPresentationFormatTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormatTypes";
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
pub struct GattProtocolError;
impl GattProtocolError {
    pub fn InvalidHandle() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InvalidHandle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ReadNotPermitted() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).ReadNotPermitted)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn WriteNotPermitted() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).WriteNotPermitted)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InvalidPdu() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InvalidPdu)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InsufficientAuthentication() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InsufficientAuthentication)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RequestNotSupported() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).RequestNotSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InvalidOffset() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InvalidOffset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InsufficientAuthorization() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InsufficientAuthorization)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PrepareQueueFull() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).PrepareQueueFull)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AttributeNotFound() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).AttributeNotFound)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AttributeNotLong() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).AttributeNotLong)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InsufficientEncryptionKeySize() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InsufficientEncryptionKeySize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InvalidAttributeValueLength() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InvalidAttributeValueLength)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UnlikelyError() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UnlikelyError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InsufficientEncryption() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InsufficientEncryption)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UnsupportedGroupType() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).UnsupportedGroupType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn InsufficientResources() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).InsufficientResources)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattProtocolErrorStatics<R, F: FnOnce(&IGattProtocolErrorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattProtocolError, IGattProtocolErrorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GattProtocolError {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattProtocolError";
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattReadClientCharacteristicConfigurationDescriptorResult(::windows::core::IUnknown);
impl GattReadClientCharacteristicConfigurationDescriptorResult {
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCommunicationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ClientCharacteristicConfigurationDescriptor(&self) -> ::windows::core::Result<GattClientCharacteristicConfigurationDescriptorValue> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattClientCharacteristicConfigurationDescriptorValue>();
            (::windows::core::Interface::vtable(this).ClientCharacteristicConfigurationDescriptor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = &::windows::core::ComInterface::cast::<IGattReadClientCharacteristicConfigurationDescriptorResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<u8>>();
            (::windows::core::Interface::vtable(this).ProtocolError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadClientCharacteristicConfigurationDescriptorResult {}
impl ::core::fmt::Debug for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadClientCharacteristicConfigurationDescriptorResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattReadClientCharacteristicConfigurationDescriptorResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadClientCharacteristicConfigurationDescriptorResult;{63a66f09-1aea-4c4c-a50f-97bae474b348})");
}
impl ::core::clone::Clone for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattReadClientCharacteristicConfigurationDescriptorResult {
    type Vtable = IGattReadClientCharacteristicConfigurationDescriptorResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattReadClientCharacteristicConfigurationDescriptorResult {
    const IID: ::windows::core::GUID = <IGattReadClientCharacteristicConfigurationDescriptorResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattReadClientCharacteristicConfigurationDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadClientCharacteristicConfigurationDescriptorResult";
}
::windows::imp::interface_hierarchy!(GattReadClientCharacteristicConfigurationDescriptorResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattReadClientCharacteristicConfigurationDescriptorResult {}
unsafe impl ::core::marker::Sync for GattReadClientCharacteristicConfigurationDescriptorResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattReadRequest(::windows::core::IUnknown);
impl GattReadRequest {
    pub fn Offset(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Offset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattRequestState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).StateChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RespondWithValue<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RespondWithValue)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RespondWithProtocolError)(::windows::core::Interface::as_raw(this), protocolerror).ok() }
    }
}
impl ::core::cmp::PartialEq for GattReadRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadRequest {}
impl ::core::fmt::Debug for GattReadRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadRequest").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattReadRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequest;{f1dd6535-6acd-42a6-a4bb-d789dae0043e})");
}
impl ::core::clone::Clone for GattReadRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattReadRequest {
    type Vtable = IGattReadRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattReadRequest {
    const IID: ::windows::core::GUID = <IGattReadRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattReadRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequest";
}
::windows::imp::interface_hierarchy!(GattReadRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattReadRequest {}
unsafe impl ::core::marker::Sync for GattReadRequest {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattReadRequestedEventArgs(::windows::core::IUnknown);
impl GattReadRequestedEventArgs {
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSession>();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattReadRequest>>();
            (::windows::core::Interface::vtable(this).GetRequestAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattReadRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadRequestedEventArgs {}
impl ::core::fmt::Debug for GattReadRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattReadRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequestedEventArgs;{93497243-f39c-484b-8ab6-996ba486cfa3})");
}
impl ::core::clone::Clone for GattReadRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattReadRequestedEventArgs {
    type Vtable = IGattReadRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattReadRequestedEventArgs {
    const IID: ::windows::core::GUID = <IGattReadRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattReadRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(GattReadRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattReadRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GattReadRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattReadResult(::windows::core::IUnknown);
impl GattReadResult {
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCommunicationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = &::windows::core::ComInterface::cast::<IGattReadResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<u8>>();
            (::windows::core::Interface::vtable(this).ProtocolError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattReadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadResult {}
impl ::core::fmt::Debug for GattReadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattReadResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadResult;{63a66f08-1aea-4c4c-a50f-97bae474b348})");
}
impl ::core::clone::Clone for GattReadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattReadResult {
    type Vtable = IGattReadResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattReadResult {
    const IID: ::windows::core::GUID = <IGattReadResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattReadResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadResult";
}
::windows::imp::interface_hierarchy!(GattReadResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattReadResult {}
unsafe impl ::core::marker::Sync for GattReadResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattReliableWriteTransaction(::windows::core::IUnknown);
impl GattReliableWriteTransaction {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattReliableWriteTransaction, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteValue<P0>(&self, characteristic: &GattCharacteristic, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).WriteValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(characteristic), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>();
            (::windows::core::Interface::vtable(this).CommitAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommitWithResultAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &::windows::core::ComInterface::cast::<IGattReliableWriteTransaction2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>();
            (::windows::core::Interface::vtable(this).CommitWithResultAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattReliableWriteTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReliableWriteTransaction {}
impl ::core::fmt::Debug for GattReliableWriteTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReliableWriteTransaction").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattReliableWriteTransaction {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReliableWriteTransaction;{63a66f07-1aea-4c4c-a50f-97bae474b348})");
}
impl ::core::clone::Clone for GattReliableWriteTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattReliableWriteTransaction {
    type Vtable = IGattReliableWriteTransaction_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattReliableWriteTransaction {
    const IID: ::windows::core::GUID = <IGattReliableWriteTransaction as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattReliableWriteTransaction {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReliableWriteTransaction";
}
::windows::imp::interface_hierarchy!(GattReliableWriteTransaction, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattReliableWriteTransaction {}
unsafe impl ::core::marker::Sync for GattReliableWriteTransaction {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattRequestStateChangedEventArgs(::windows::core::IUnknown);
impl GattRequestStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattRequestState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattRequestStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattRequestStateChangedEventArgs {}
impl ::core::fmt::Debug for GattRequestStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattRequestStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattRequestStateChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestStateChangedEventArgs;{e834d92c-27be-44b3-9d0d-4fc6e808dd3f})");
}
impl ::core::clone::Clone for GattRequestStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattRequestStateChangedEventArgs {
    type Vtable = IGattRequestStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattRequestStateChangedEventArgs {
    const IID: ::windows::core::GUID = <IGattRequestStateChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattRequestStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestStateChangedEventArgs";
}
::windows::imp::interface_hierarchy!(GattRequestStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattRequestStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattRequestStateChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattServiceProvider(::windows::core::IUnknown);
impl GattServiceProvider {
    pub fn Service(&self) -> ::windows::core::Result<GattLocalService> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattLocalService>();
            (::windows::core::Interface::vtable(this).Service)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AdvertisementStatus(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattServiceProviderAdvertisementStatus>();
            (::windows::core::Interface::vtable(this).AdvertisementStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AdvertisementStatusChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AdvertisementStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdvertisementStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdvertisementStatusChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn StartAdvertising(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartAdvertising)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn StartAdvertisingWithParameters(&self, parameters: &GattServiceProviderAdvertisingParameters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartAdvertisingWithParameters)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(parameters)).ok() }
    }
    pub fn StopAdvertising(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopAdvertising)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(serviceuuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>> {
        Self::IGattServiceProviderStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>>();
            (::windows::core::Interface::vtable(this).CreateAsync)(::windows::core::Interface::as_raw(this), serviceuuid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderStatics<R, F: FnOnce(&IGattServiceProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattServiceProvider, IGattServiceProviderStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProvider {}
impl ::core::fmt::Debug for GattServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProvider").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattServiceProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProvider;{7822b3cd-2889-4f86-a051-3f0aed1c2760})");
}
impl ::core::clone::Clone for GattServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProvider {
    type Vtable = IGattServiceProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProvider {
    const IID: ::windows::core::GUID = <IGattServiceProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProvider";
}
::windows::imp::interface_hierarchy!(GattServiceProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProvider {}
unsafe impl ::core::marker::Sync for GattServiceProvider {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatusChangedEventArgs(::windows::core::IUnknown);
impl GattServiceProviderAdvertisementStatusChangedEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattServiceProviderAdvertisementStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderAdvertisementStatusChangedEventArgs {}
impl ::core::fmt::Debug for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderAdvertisementStatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattServiceProviderAdvertisementStatusChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatusChangedEventArgs;{59a5aa65-fa21-4ffc-b155-04d928012686})");
}
impl ::core::clone::Clone for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderAdvertisementStatusChangedEventArgs {
    type Vtable = IGattServiceProviderAdvertisementStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProviderAdvertisementStatusChangedEventArgs {
    const IID: ::windows::core::GUID = <IGattServiceProviderAdvertisementStatusChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderAdvertisementStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatusChangedEventArgs";
}
::windows::imp::interface_hierarchy!(GattServiceProviderAdvertisementStatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProviderAdvertisementStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattServiceProviderAdvertisementStatusChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderAdvertisingParameters(::windows::core::IUnknown);
impl GattServiceProviderAdvertisingParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattServiceProviderAdvertisingParameters, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetIsConnectable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsConnectable)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsConnectable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsConnectable)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDiscoverable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDiscoverable)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscoverable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDiscoverable)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetServiceData<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IGattServiceProviderAdvertisingParameters2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetServiceData)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ServiceData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::ComInterface::cast::<IGattServiceProviderAdvertisingParameters2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).ServiceData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderAdvertisingParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderAdvertisingParameters {}
impl ::core::fmt::Debug for GattServiceProviderAdvertisingParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderAdvertisingParameters").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattServiceProviderAdvertisingParameters {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisingParameters;{e2ce31ab-6315-4c22-9bd7-781dbc3d8d82})");
}
impl ::core::clone::Clone for GattServiceProviderAdvertisingParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderAdvertisingParameters {
    type Vtable = IGattServiceProviderAdvertisingParameters_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProviderAdvertisingParameters {
    const IID: ::windows::core::GUID = <IGattServiceProviderAdvertisingParameters as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderAdvertisingParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisingParameters";
}
::windows::imp::interface_hierarchy!(GattServiceProviderAdvertisingParameters, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProviderAdvertisingParameters {}
unsafe impl ::core::marker::Sync for GattServiceProviderAdvertisingParameters {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderResult(::windows::core::IUnknown);
impl GattServiceProviderResult {
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceProvider(&self) -> ::windows::core::Result<GattServiceProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattServiceProvider>();
            (::windows::core::Interface::vtable(this).ServiceProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderResult {}
impl ::core::fmt::Debug for GattServiceProviderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattServiceProviderResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderResult;{764696d8-c53e-428c-8a48-67afe02c3ae6})");
}
impl ::core::clone::Clone for GattServiceProviderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderResult {
    type Vtable = IGattServiceProviderResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProviderResult {
    const IID: ::windows::core::GUID = <IGattServiceProviderResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderResult";
}
::windows::imp::interface_hierarchy!(GattServiceProviderResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProviderResult {}
unsafe impl ::core::marker::Sync for GattServiceProviderResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
pub struct GattServiceUuids;
impl GattServiceUuids {
    pub fn Battery() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Battery)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BloodPressure() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).BloodPressure)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CyclingSpeedAndCadence() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CyclingSpeedAndCadence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GenericAccess() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GenericAccess)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GenericAttribute() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).GenericAttribute)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Glucose() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Glucose)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HealthThermometer() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HealthThermometer)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HeartRate() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HeartRate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RunningSpeedAndCadence() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RunningSpeedAndCadence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AlertNotification() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).AlertNotification)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CurrentTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CurrentTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CyclingPower() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).CyclingPower)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DeviceInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HumanInterfaceDevice() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).HumanInterfaceDevice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ImmediateAlert() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ImmediateAlert)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LinkLoss() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LinkLoss)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LocationAndNavigation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LocationAndNavigation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn NextDstChange() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).NextDstChange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PhoneAlertStatus() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).PhoneAlertStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ReferenceTimeUpdate() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ReferenceTimeUpdate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ScanParameters() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ScanParameters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TxPower() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TxPower)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceUuidsStatics<R, F: FnOnce(&IGattServiceUuidsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattServiceUuids, IGattServiceUuidsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattServiceUuidsStatics2<R, F: FnOnce(&IGattServiceUuidsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattServiceUuids, IGattServiceUuidsStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GattServiceUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceUuids";
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattSession(::windows::core::IUnknown);
impl GattSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<super::BluetoothDeviceId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothDeviceId>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanMaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanMaintainConnection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaintainConnection)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).MaintainConnection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxPduSize(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).MaxPduSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SessionStatus(&self) -> ::windows::core::Result<GattSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSessionStatus>();
            (::windows::core::Interface::vtable(this).SessionStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPduSizeChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MaxPduSizeChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMaxPduSizeChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMaxPduSizeChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionStatusChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SessionStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSessionStatusChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromDeviceIdAsync(deviceid: &super::BluetoothDeviceId) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattSession>> {
        Self::IGattSessionStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattSession>>();
            (::windows::core::Interface::vtable(this).FromDeviceIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattSessionStatics<R, F: FnOnce(&IGattSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattSession, IGattSessionStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattSession {}
impl ::core::fmt::Debug for GattSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSession;{d23b5143-e04e-4c24-999c-9c256f9856b1})");
}
impl ::core::clone::Clone for GattSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattSession {
    type Vtable = IGattSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattSession {
    const IID: ::windows::core::GUID = <IGattSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattSession {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSession";
}
::windows::imp::interface_hierarchy!(GattSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::IClosable> for GattSession {}
unsafe impl ::core::marker::Send for GattSession {}
unsafe impl ::core::marker::Sync for GattSession {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattSessionStatusChangedEventArgs(::windows::core::IUnknown);
impl GattSessionStatusChangedEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<GattSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSessionStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattSessionStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattSessionStatusChangedEventArgs {}
impl ::core::fmt::Debug for GattSessionStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSessionStatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattSessionStatusChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatusChangedEventArgs;{7605b72e-837f-404c-ab34-3163f39ddf32})");
}
impl ::core::clone::Clone for GattSessionStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattSessionStatusChangedEventArgs {
    type Vtable = IGattSessionStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattSessionStatusChangedEventArgs {
    const IID: ::windows::core::GUID = <IGattSessionStatusChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattSessionStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatusChangedEventArgs";
}
::windows::imp::interface_hierarchy!(GattSessionStatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattSessionStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattSessionStatusChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattSubscribedClient(::windows::core::IUnknown);
impl GattSubscribedClient {
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSession>();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxNotificationSize(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).MaxNotificationSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxNotificationSizeChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MaxNotificationSizeChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMaxNotificationSizeChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMaxNotificationSizeChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for GattSubscribedClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattSubscribedClient {}
impl ::core::fmt::Debug for GattSubscribedClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSubscribedClient").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattSubscribedClient {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSubscribedClient;{736e9001-15a4-4ec2-9248-e3f20d463be9})");
}
impl ::core::clone::Clone for GattSubscribedClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattSubscribedClient {
    type Vtable = IGattSubscribedClient_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattSubscribedClient {
    const IID: ::windows::core::GUID = <IGattSubscribedClient as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattSubscribedClient {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSubscribedClient";
}
::windows::imp::interface_hierarchy!(GattSubscribedClient, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattSubscribedClient {}
unsafe impl ::core::marker::Sync for GattSubscribedClient {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattValueChangedEventArgs(::windows::core::IUnknown);
impl GattValueChangedEventArgs {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CharacteristicValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).CharacteristicValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattValueChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattValueChangedEventArgs {}
impl ::core::fmt::Debug for GattValueChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattValueChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattValueChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs;{d21bdb54-06e3-4ed8-a263-acfac8ba7313})");
}
impl ::core::clone::Clone for GattValueChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattValueChangedEventArgs {
    type Vtable = IGattValueChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattValueChangedEventArgs {
    const IID: ::windows::core::GUID = <IGattValueChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs";
}
::windows::imp::interface_hierarchy!(GattValueChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattValueChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattWriteRequest(::windows::core::IUnknown);
impl GattWriteRequest {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Offset(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Offset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Option(&self) -> ::windows::core::Result<GattWriteOption> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattWriteOption>();
            (::windows::core::Interface::vtable(this).Option)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattRequestState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).StateChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Respond(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Respond)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RespondWithProtocolError)(::windows::core::Interface::as_raw(this), protocolerror).ok() }
    }
}
impl ::core::cmp::PartialEq for GattWriteRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattWriteRequest {}
impl ::core::fmt::Debug for GattWriteRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteRequest").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattWriteRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequest;{aeb6a9ed-de2f-4fc2-a9a8-94ea7844f13d})");
}
impl ::core::clone::Clone for GattWriteRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattWriteRequest {
    type Vtable = IGattWriteRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattWriteRequest {
    const IID: ::windows::core::GUID = <IGattWriteRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattWriteRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequest";
}
::windows::imp::interface_hierarchy!(GattWriteRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattWriteRequest {}
unsafe impl ::core::marker::Sync for GattWriteRequest {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattWriteRequestedEventArgs(::windows::core::IUnknown);
impl GattWriteRequestedEventArgs {
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattSession>();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>>();
            (::windows::core::Interface::vtable(this).GetRequestAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattWriteRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattWriteRequestedEventArgs {}
impl ::core::fmt::Debug for GattWriteRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattWriteRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequestedEventArgs;{2dec8bbe-a73a-471a-94d5-037deadd0806})");
}
impl ::core::clone::Clone for GattWriteRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattWriteRequestedEventArgs {
    type Vtable = IGattWriteRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattWriteRequestedEventArgs {
    const IID: ::windows::core::GUID = <IGattWriteRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattWriteRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(GattWriteRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattWriteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GattWriteRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
pub struct GattWriteResult(::windows::core::IUnknown);
impl GattWriteResult {
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCommunicationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<u8>>();
            (::windows::core::Interface::vtable(this).ProtocolError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattWriteResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattWriteResult {}
impl ::core::fmt::Debug for GattWriteResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattWriteResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteResult;{4991ddb1-cb2b-44f7-99fc-d29a2871dc9b})");
}
impl ::core::clone::Clone for GattWriteResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattWriteResult {
    type Vtable = IGattWriteResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattWriteResult {
    const IID: ::windows::core::GUID = <IGattWriteResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattWriteResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteResult";
}
::windows::imp::interface_hierarchy!(GattWriteResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattWriteResult {}
unsafe impl ::core::marker::Sync for GattWriteResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattCharacteristicProperties(pub u32);
impl GattCharacteristicProperties {
    pub const None: Self = Self(0u32);
    pub const Broadcast: Self = Self(1u32);
    pub const Read: Self = Self(2u32);
    pub const WriteWithoutResponse: Self = Self(4u32);
    pub const Write: Self = Self(8u32);
    pub const Notify: Self = Self(16u32);
    pub const Indicate: Self = Self(32u32);
    pub const AuthenticatedSignedWrites: Self = Self(64u32);
    pub const ExtendedProperties: Self = Self(128u32);
    pub const ReliableWrites: Self = Self(256u32);
    pub const WritableAuxiliaries: Self = Self(512u32);
}
impl ::core::marker::Copy for GattCharacteristicProperties {}
impl ::core::clone::Clone for GattCharacteristicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattCharacteristicProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattCharacteristicProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattCharacteristicProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristicProperties").field(&self.0).finish()
    }
}
impl GattCharacteristicProperties {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GattCharacteristicProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GattCharacteristicProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GattCharacteristicProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GattCharacteristicProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GattCharacteristicProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for GattCharacteristicProperties {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicProperties;u4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(pub i32);
impl GattClientCharacteristicConfigurationDescriptorValue {
    pub const None: Self = Self(0i32);
    pub const Notify: Self = Self(1i32);
    pub const Indicate: Self = Self(2i32);
}
impl ::core::marker::Copy for GattClientCharacteristicConfigurationDescriptorValue {}
impl ::core::clone::Clone for GattClientCharacteristicConfigurationDescriptorValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattClientCharacteristicConfigurationDescriptorValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattClientCharacteristicConfigurationDescriptorValue {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattClientCharacteristicConfigurationDescriptorValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattClientCharacteristicConfigurationDescriptorValue").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattClientCharacteristicConfigurationDescriptorValue {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientCharacteristicConfigurationDescriptorValue;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattCommunicationStatus(pub i32);
impl GattCommunicationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unreachable: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for GattCommunicationStatus {}
impl ::core::clone::Clone for GattCommunicationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattCommunicationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattCommunicationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattCommunicationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCommunicationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattCommunicationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCommunicationStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattOpenStatus(pub i32);
impl GattOpenStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AlreadyOpened: Self = Self(2i32);
    pub const NotFound: Self = Self(3i32);
    pub const SharingViolation: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
impl ::core::marker::Copy for GattOpenStatus {}
impl ::core::clone::Clone for GattOpenStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattOpenStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattOpenStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattOpenStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattOpenStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattOpenStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattOpenStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattProtectionLevel(pub i32);
impl GattProtectionLevel {
    pub const Plain: Self = Self(0i32);
    pub const AuthenticationRequired: Self = Self(1i32);
    pub const EncryptionRequired: Self = Self(2i32);
    pub const EncryptionAndAuthenticationRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for GattProtectionLevel {}
impl ::core::clone::Clone for GattProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattProtectionLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattProtectionLevel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattProtectionLevel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattProtectionLevel;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattRequestState(pub i32);
impl GattRequestState {
    pub const Pending: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for GattRequestState {}
impl ::core::clone::Clone for GattRequestState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattRequestState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattRequestState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattRequestState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattRequestState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattRequestState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestState;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattServiceProviderAdvertisementStatus(pub i32);
impl GattServiceProviderAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
    pub const StartedWithoutAllAdvertisementData: Self = Self(4i32);
}
impl ::core::marker::Copy for GattServiceProviderAdvertisementStatus {}
impl ::core::clone::Clone for GattServiceProviderAdvertisementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattServiceProviderAdvertisementStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattServiceProviderAdvertisementStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattServiceProviderAdvertisementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderAdvertisementStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattServiceProviderAdvertisementStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattSessionStatus(pub i32);
impl GattSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
impl ::core::marker::Copy for GattSessionStatus {}
impl ::core::clone::Clone for GattSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattSessionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattSessionStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattSessionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSessionStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattSessionStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattSharingMode(pub i32);
impl GattSharingMode {
    pub const Unspecified: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
    pub const SharedReadOnly: Self = Self(2i32);
    pub const SharedReadAndWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for GattSharingMode {}
impl ::core::clone::Clone for GattSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattSharingMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSharingMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattSharingMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSharingMode;i4)");
}
#[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GattWriteOption(pub i32);
impl GattWriteOption {
    pub const WriteWithResponse: Self = Self(0i32);
    pub const WriteWithoutResponse: Self = Self(1i32);
}
impl ::core::marker::Copy for GattWriteOption {}
impl ::core::clone::Clone for GattWriteOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GattWriteOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GattWriteOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GattWriteOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteOption").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattWriteOption {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteOption;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
