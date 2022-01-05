#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTriggerDetailsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTriggerDetails2Impl: Sized {
    fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherTriggerDetailsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Advertisements(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>;
    fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerDetailsImpl: Sized {
    fn Characteristic(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattCharacteristic>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerDetails2Impl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn EventTriggeringMode(&self) -> ::windows::core::Result<BluetoothEventTriggeringMode>;
    fn ValueChangedEvents(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderConnectionImpl: Sized {
    fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattLocalService>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderConnectionStaticsImpl: Sized {
    fn AllServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<GattServiceProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommConnectionTriggerDetailsImpl: Sized {
    fn Socket(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::StreamSocket>;
    fn Incoming(&self) -> ::windows::core::Result<bool>;
    fn RemoteDevice(&self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommInboundConnectionInformationImpl: Sized {
    fn SdpRecord(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetSdpRecord(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn LocalServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetLocalServiceId(&self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
    fn ServiceCapabilities(&self) -> ::windows::core::Result<super::BluetoothServiceCapabilities>;
    fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommOutboundConnectionInformationImpl: Sized {
    fn RemoteServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetRemoteServiceId(&self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
}
