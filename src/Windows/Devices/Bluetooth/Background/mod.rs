#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothEventTriggeringMode(pub i32);
impl BluetoothEventTriggeringMode {
    pub const Serial: BluetoothEventTriggeringMode = BluetoothEventTriggeringMode(0i32);
    pub const Batch: BluetoothEventTriggeringMode = BluetoothEventTriggeringMode(1i32);
    pub const KeepLatest: BluetoothEventTriggeringMode = BluetoothEventTriggeringMode(2i32);
}
impl ::core::convert::From<i32> for BluetoothEventTriggeringMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothEventTriggeringMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothEventTriggeringMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Background.BluetoothEventTriggeringMode;i4)");
}
impl ::windows::core::DefaultType for BluetoothEventTriggeringMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementPublisherTriggerDetails {
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Advertisement`*"]
    pub fn Status(&self) -> ::windows::core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: super::Advertisement::BluetoothLEAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Foundation`*"]
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails;{610eca86-3480-41c9-a918-7ddadf207e00})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x610eca86_3480_41c9_a918_7ddadf207e00);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementWatcherTriggerDetails {
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn Advertisements(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothSignalStrengthFilter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails;{a7db5ad7-2257-4e69-9784-fee645c1dce0})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7db5ad7_2257_4e69_9784_fee645c1dce0);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattCharacteristicNotificationTriggerDetails(pub ::windows::core::IInspectable);
impl GattCharacteristicNotificationTriggerDetails {
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Characteristic(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::GenericAttributeProfile::GattCharacteristic>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Storage_Streams`*"]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn EventTriggeringMode(&self) -> ::windows::core::Result<BluetoothEventTriggeringMode> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: BluetoothEventTriggeringMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothEventTriggeringMode>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn ValueChangedEvents(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattCharacteristicNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails;{9ba03b18-0fec-436a-93b1-f46c697532a2})");
}
unsafe impl ::windows::core::Interface for GattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ba03b18_0fec_436a_93b1_f46c697532a2);
}
impl ::windows::core::RuntimeName for GattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails";
}
impl ::core::convert::From<GattCharacteristicNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattCharacteristicNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattCharacteristicNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for GattCharacteristicNotificationTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProviderConnection(pub ::windows::core::IInspectable);
impl GattServiceProviderConnection {
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Service(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::GenericAttributeProfile::GattLocalService>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Foundation_Collections`*"]
    pub fn AllServices() -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>> {
        Self::IGattServiceProviderConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>>(result__)
        })
    }
    pub fn IGattServiceProviderConnectionStatics<R, F: FnOnce(&IGattServiceProviderConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattServiceProviderConnection, IGattServiceProviderConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderConnection;{7fa1b9b9-2f13-40b5-9582-8eb78e98ef13})");
}
unsafe impl ::windows::core::Interface for GattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa1b9b9_2f13_40b5_9582_8eb78e98ef13);
}
impl ::windows::core::RuntimeName for GattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderConnection";
}
impl ::core::convert::From<GattServiceProviderConnection> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProviderConnection> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProviderConnection> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProviderConnection> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderConnection {}
unsafe impl ::core::marker::Sync for GattServiceProviderConnection {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProviderTriggerDetails(pub ::windows::core::IInspectable);
impl GattServiceProviderTriggerDetails {
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Connection(&self) -> ::windows::core::Result<GattServiceProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattServiceProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails;{ae8c0625-05ff-4afb-b16a-de95f3cf0158})");
}
unsafe impl ::windows::core::Interface for GattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae8c0625_05ff_4afb_b16a_de95f3cf0158);
}
impl ::windows::core::RuntimeName for GattServiceProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails";
}
impl ::core::convert::From<GattServiceProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for GattServiceProviderTriggerDetails {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x610eca86_3480_41c9_a918_7ddadf207e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Advertisement::BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a3d025_c601_42d6_9829_4ccb3f5cd77f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7db5ad7_2257_4e69_9784_fee645c1dce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ba03b18_0fec_436a_93b1_f46c697532a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerDetails2 {
    type Vtable = IGattCharacteristicNotificationTriggerDetails2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x727a50dc_949d_454a_b192_983467e3d50f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothEventTriggeringMode) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa1b9b9_2f13_40b5_9582_8eb78e98ef13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderConnectionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderConnectionStatics {
    type Vtable = IGattServiceProviderConnectionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d509f4b_0b0e_4466_b8cd_6ebdda1fa17d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnectionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae8c0625_05ff_4afb_b16a_de95f3cf0158);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRfcommConnectionTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf922734d_2e3c_4efc_ab59_fc5cf96f97e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRfcommInboundConnectionInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d3e75a8_5429_4059_92e3_1e8b65528707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommInboundConnectionInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRfcommOutboundConnectionInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb091227b_f434_4cb0_99b1_4ab8cedaedd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommOutboundConnectionInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
);
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RfcommConnectionTriggerDetails(pub ::windows::core::IInspectable);
impl RfcommConnectionTriggerDetails {
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Networking_Sockets`*"]
    pub fn Socket(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::Sockets::StreamSocket>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Incoming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn RemoteDevice(&self) -> ::windows::core::Result<super::BluetoothDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothDevice>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommConnectionTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails;{f922734d-2e3c-4efc-ab59-fc5cf96f97e3})");
}
unsafe impl ::windows::core::Interface for RfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf922734d_2e3c_4efc_ab59_fc5cf96f97e3);
}
impl ::windows::core::RuntimeName for RfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails";
}
impl ::core::convert::From<RfcommConnectionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: RfcommConnectionTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RfcommConnectionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &RfcommConnectionTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RfcommConnectionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: RfcommConnectionTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RfcommConnectionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &RfcommConnectionTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RfcommConnectionTriggerDetails {}
unsafe impl ::core::marker::Sync for RfcommConnectionTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RfcommInboundConnectionInformation(pub ::windows::core::IInspectable);
impl RfcommInboundConnectionInformation {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Storage_Streams`*"]
    pub fn SdpRecord(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Storage_Streams`*"]
    pub fn SetSdpRecord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn LocalServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Rfcomm::RfcommServiceId>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn SetLocalServiceId<'a, Param0: ::windows::core::IntoParam<'a, super::Rfcomm::RfcommServiceId>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn ServiceCapabilities(&self) -> ::windows::core::Result<super::BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothServiceCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothServiceCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommInboundConnectionInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation;{6d3e75a8-5429-4059-92e3-1e8b65528707})");
}
unsafe impl ::windows::core::Interface for RfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d3e75a8_5429_4059_92e3_1e8b65528707);
}
impl ::windows::core::RuntimeName for RfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation";
}
impl ::core::convert::From<RfcommInboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: RfcommInboundConnectionInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RfcommInboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: &RfcommInboundConnectionInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RfcommInboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: RfcommInboundConnectionInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RfcommInboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: &RfcommInboundConnectionInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RfcommInboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommInboundConnectionInformation {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RfcommOutboundConnectionInformation(pub ::windows::core::IInspectable);
impl RfcommOutboundConnectionInformation {
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn RemoteServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Rfcomm::RfcommServiceId>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn SetRemoteServiceId<'a, Param0: ::windows::core::IntoParam<'a, super::Rfcomm::RfcommServiceId>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommOutboundConnectionInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation;{b091227b-f434-4cb0-99b1-4ab8cedaedd7})");
}
unsafe impl ::windows::core::Interface for RfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb091227b_f434_4cb0_99b1_4ab8cedaedd7);
}
impl ::windows::core::RuntimeName for RfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation";
}
impl ::core::convert::From<RfcommOutboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: RfcommOutboundConnectionInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RfcommOutboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: &RfcommOutboundConnectionInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RfcommOutboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: RfcommOutboundConnectionInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RfcommOutboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: &RfcommOutboundConnectionInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RfcommOutboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommOutboundConnectionInformation {}
