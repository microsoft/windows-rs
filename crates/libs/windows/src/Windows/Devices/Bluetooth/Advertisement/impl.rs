#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementImpl: Sized {
    fn Flags(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>;
    fn SetFlags(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>) -> ::windows::core::Result<()>;
    fn LocalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServiceUuids(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::GUID>>;
    fn ManufacturerData(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>>;
    fn DataSections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>>;
    fn GetManufacturerDataByCompanyId(&self, companyid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>>;
    fn GetSectionsByType(&self, r#type: u8) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementBytePatternImpl: Sized {
    fn DataType(&self) -> ::windows::core::Result<u8>;
    fn SetDataType(&self, value: u8) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<i16>;
    fn SetOffset(&self, value: i16) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementBytePatternFactoryImpl: Sized {
    fn Create(&self, datatype: u8, offset: i16, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BluetoothLEAdvertisementBytePattern>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementDataSectionImpl: Sized {
    fn DataType(&self) -> ::windows::core::Result<u8>;
    fn SetDataType(&self, value: u8) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementDataSectionFactoryImpl: Sized {
    fn Create(&self, datatype: u8, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BluetoothLEAdvertisementDataSection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementDataTypesStaticsImpl: Sized {
    fn Flags(&self) -> ::windows::core::Result<u8>;
    fn IncompleteService16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn CompleteService16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn IncompleteService32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn CompleteService32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn IncompleteService128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn CompleteService128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ShortenedLocalName(&self) -> ::windows::core::Result<u8>;
    fn CompleteLocalName(&self) -> ::windows::core::Result<u8>;
    fn TxPowerLevel(&self) -> ::windows::core::Result<u8>;
    fn SlaveConnectionIntervalRange(&self) -> ::windows::core::Result<u8>;
    fn ServiceSolicitation16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceSolicitation32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceSolicitation128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceData16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceData32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceData128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn PublicTargetAddress(&self) -> ::windows::core::Result<u8>;
    fn RandomTargetAddress(&self) -> ::windows::core::Result<u8>;
    fn Appearance(&self) -> ::windows::core::Result<u8>;
    fn AdvertisingInterval(&self) -> ::windows::core::Result<u8>;
    fn ManufacturerSpecificData(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementFilterImpl: Sized {
    fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement>;
    fn SetAdvertisement(&self, value: &::core::option::Option<BluetoothLEAdvertisement>) -> ::windows::core::Result<()>;
    fn BytePatterns(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus>;
    fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisher2Impl: Sized {
    fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
    fn SetPreferredTransmitPowerLevelInDBm(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn UseExtendedAdvertisement(&self) -> ::windows::core::Result<bool>;
    fn SetUseExtendedAdvertisement(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAnonymous(&self) -> ::windows::core::Result<bool>;
    fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherFactoryImpl: Sized {
    fn Create(&self, advertisement: &::core::option::Option<BluetoothLEAdvertisement>) -> ::windows::core::Result<BluetoothLEAdvertisementPublisher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherStatusChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Impl: Sized {
    fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementReceivedEventArgsImpl: Sized {
    fn RawSignalStrengthInDBm(&self) -> ::windows::core::Result<i16>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn AdvertisementType(&self) -> ::windows::core::Result<BluetoothLEAdvertisementType>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementReceivedEventArgs2Impl: Sized {
    fn BluetoothAddressType(&self) -> ::windows::core::Result<super::BluetoothAddressType>;
    fn TransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
    fn IsAnonymous(&self) -> ::windows::core::Result<bool>;
    fn IsConnectable(&self) -> ::windows::core::Result<bool>;
    fn IsScannable(&self) -> ::windows::core::Result<bool>;
    fn IsDirected(&self) -> ::windows::core::Result<bool>;
    fn IsScanResponse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherImpl: Sized {
    fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementWatcherStatus>;
    fn ScanningMode(&self) -> ::windows::core::Result<BluetoothLEScanningMode>;
    fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> ::windows::core::Result<()>;
    fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter>;
    fn SetSignalStrengthFilter(&self, value: &::core::option::Option<super::BluetoothSignalStrengthFilter>) -> ::windows::core::Result<()>;
    fn AdvertisementFilter(&self) -> ::windows::core::Result<BluetoothLEAdvertisementFilter>;
    fn SetAdvertisementFilter(&self, value: &::core::option::Option<BluetoothLEAdvertisementFilter>) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Received(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcher2Impl: Sized {
    fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool>;
    fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherFactoryImpl: Sized {
    fn Create(&self, advertisementfilter: &::core::option::Option<BluetoothLEAdvertisementFilter>) -> ::windows::core::Result<BluetoothLEAdvertisementWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherStoppedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEManufacturerDataImpl: Sized {
    fn CompanyId(&self) -> ::windows::core::Result<u16>;
    fn SetCompanyId(&self, value: u16) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEManufacturerDataFactoryImpl: Sized {
    fn Create(&self, companyid: u16, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BluetoothLEManufacturerData>;
}
