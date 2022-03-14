#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothEventTriggeringMode(pub i32);
impl BluetoothEventTriggeringMode {
    pub const Serial: Self = Self(0i32);
    pub const Batch: Self = Self(1i32);
    pub const KeepLatest: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothEventTriggeringMode {}
impl ::core::clone::Clone for BluetoothEventTriggeringMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothEventTriggeringMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothEventTriggeringMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothEventTriggeringMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothEventTriggeringMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothEventTriggeringMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Background.BluetoothEventTriggeringMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(::windows::core::IUnknown);
impl BluetoothLEAdvertisementPublisherTriggerDetails {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_Advertisement\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Status(&self) -> ::windows::core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: super::Advertisement::BluetoothLEAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Error)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectedTransmitPowerLevelInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementPublisherTriggerDetails {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisherTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails;{610eca86-3480-41c9-a918-7ddadf207e00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementPublisherTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(::windows::core::IUnknown);
impl BluetoothLEAdvertisementWatcherTriggerDetails {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Error)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_Advertisement\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    pub fn Advertisements(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Advertisements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalStrengthFilter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothSignalStrengthFilter>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementWatcherTriggerDetails {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcherTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails;{a7db5ad7-2257-4e69-9784-fee645c1dce0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementWatcherTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct GattCharacteristicNotificationTriggerDetails(::windows::core::IUnknown);
impl GattCharacteristicNotificationTriggerDetails {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Characteristic)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::GenericAttributeProfile::GattCharacteristic>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Error)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn EventTriggeringMode(&self) -> ::windows::core::Result<BluetoothEventTriggeringMode> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: BluetoothEventTriggeringMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EventTriggeringMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothEventTriggeringMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    pub fn ValueChangedEvents(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueChangedEvents)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>(result__)
        }
    }
}
impl ::core::clone::Clone for GattCharacteristicNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GattCharacteristicNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattCharacteristicNotificationTriggerDetails {}
impl ::core::fmt::Debug for GattCharacteristicNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristicNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GattCharacteristicNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails;{9ba03b18-0fec-436a-93b1-f46c697532a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IGattCharacteristicNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails";
}
impl ::core::convert::From<GattCharacteristicNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: GattCharacteristicNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &GattCharacteristicNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GattCharacteristicNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: GattCharacteristicNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &GattCharacteristicNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GattCharacteristicNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for GattCharacteristicNotificationTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderConnection(::windows::core::IUnknown);
impl GattServiceProviderConnection {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TriggerId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Service)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::GenericAttributeProfile::GattLocalService>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllServices() -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>> {
        Self::IGattServiceProviderConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllServices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderConnectionStatics<R, F: FnOnce(&IGattServiceProviderConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattServiceProviderConnection, IGattServiceProviderConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GattServiceProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderConnection {}
impl ::core::fmt::Debug for GattServiceProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderConnection;{7fa1b9b9-2f13-40b5-9582-8eb78e98ef13})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_Vtbl;
    const IID: ::windows::core::GUID = <IGattServiceProviderConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderConnection";
}
impl ::core::convert::From<GattServiceProviderConnection> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderConnection> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GattServiceProviderConnection> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderConnection> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderConnection {}
unsafe impl ::core::marker::Sync for GattServiceProviderConnection {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderTriggerDetails(::windows::core::IUnknown);
impl GattServiceProviderTriggerDetails {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn Connection(&self) -> ::windows::core::Result<GattServiceProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Connection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattServiceProviderConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for GattServiceProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderTriggerDetails {}
impl ::core::fmt::Debug for GattServiceProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails;{ae8c0625-05ff-4afb-b16a-de95f3cf0158})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IGattServiceProviderTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails";
}
impl ::core::convert::From<GattServiceProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GattServiceProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for GattServiceProviderTriggerDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x610eca86_3480_41c9_a918_7ddadf207e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Advertisement::BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Status: usize,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a3d025_c601_42d6_9829_4ccb3f5cd77f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTransmitPowerLevelInDBm: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7db5ad7_2257_4e69_9784_fee645c1dce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    pub Advertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections")))]
    Advertisements: usize,
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ba03b18_0fec_436a_93b1_f46c697532a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerDetails2 {
    type Vtable = IGattCharacteristicNotificationTriggerDetails2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x727a50dc_949d_454a_b192_983467e3d50f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub EventTriggeringMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothEventTriggeringMode) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    pub ValueChangedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))]
    ValueChangedEvents: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa1b9b9_2f13_40b5_9582_8eb78e98ef13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TriggerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderConnectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderConnectionStatics {
    type Vtable = IGattServiceProviderConnectionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d509f4b_0b0e_4466_b8cd_6ebdda1fa17d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnectionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllServices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae8c0625_05ff_4afb_b16a_de95f3cf0158);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommConnectionTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf922734d_2e3c_4efc_ab59_fc5cf96f97e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
    pub Incoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RemoteDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommInboundConnectionInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d3e75a8_5429_4059_92e3_1e8b65528707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommInboundConnectionInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SdpRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SdpRecord: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSdpRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSdpRecord: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub LocalServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    LocalServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetLocalServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    SetLocalServiceId: usize,
    pub ServiceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT,
    pub SetServiceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommOutboundConnectionInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb091227b_f434_4cb0_99b1_4ab8cedaedd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommOutboundConnectionInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub RemoteServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    RemoteServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetRemoteServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    SetRemoteServiceId: usize,
}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct RfcommConnectionTriggerDetails(::windows::core::IUnknown);
impl RfcommConnectionTriggerDetails {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn Socket(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Socket)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::Sockets::StreamSocket>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn Incoming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Incoming)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn RemoteDevice(&self) -> ::windows::core::Result<super::BluetoothDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothDevice>(result__)
        }
    }
}
impl ::core::clone::Clone for RfcommConnectionTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommConnectionTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommConnectionTriggerDetails {}
impl ::core::fmt::Debug for RfcommConnectionTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommConnectionTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommConnectionTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails;{f922734d-2e3c-4efc-ab59-fc5cf96f97e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IRfcommConnectionTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails";
}
impl ::core::convert::From<RfcommConnectionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: RfcommConnectionTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommConnectionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &RfcommConnectionTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommConnectionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: RfcommConnectionTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommConnectionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &RfcommConnectionTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RfcommConnectionTriggerDetails {}
unsafe impl ::core::marker::Sync for RfcommConnectionTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct RfcommInboundConnectionInformation(::windows::core::IUnknown);
impl RfcommInboundConnectionInformation {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SdpRecord(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SdpRecord)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSdpRecord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSdpRecord)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn LocalServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalServiceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Rfcomm::RfcommServiceId>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn SetLocalServiceId<'a, Param0: ::windows::core::IntoParam<'a, super::Rfcomm::RfcommServiceId>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocalServiceId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn ServiceCapabilities(&self) -> ::windows::core::Result<super::BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothServiceCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothServiceCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    pub fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServiceCapabilities)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for RfcommInboundConnectionInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommInboundConnectionInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommInboundConnectionInformation {}
impl ::core::fmt::Debug for RfcommInboundConnectionInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommInboundConnectionInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommInboundConnectionInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation;{6d3e75a8-5429-4059-92e3-1e8b65528707})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_Vtbl;
    const IID: ::windows::core::GUID = <IRfcommInboundConnectionInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation";
}
impl ::core::convert::From<RfcommInboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: RfcommInboundConnectionInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommInboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: &RfcommInboundConnectionInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommInboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: RfcommInboundConnectionInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommInboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: &RfcommInboundConnectionInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RfcommInboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommInboundConnectionInformation {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct RfcommOutboundConnectionInformation(::windows::core::IUnknown);
impl RfcommOutboundConnectionInformation {
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn RemoteServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteServiceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Rfcomm::RfcommServiceId>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn SetRemoteServiceId<'a, Param0: ::windows::core::IntoParam<'a, super::Rfcomm::RfcommServiceId>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteServiceId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for RfcommOutboundConnectionInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommOutboundConnectionInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommOutboundConnectionInformation {}
impl ::core::fmt::Debug for RfcommOutboundConnectionInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommOutboundConnectionInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommOutboundConnectionInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation;{b091227b-f434-4cb0-99b1-4ab8cedaedd7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_Vtbl;
    const IID: ::windows::core::GUID = <IRfcommOutboundConnectionInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation";
}
impl ::core::convert::From<RfcommOutboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: RfcommOutboundConnectionInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommOutboundConnectionInformation> for ::windows::core::IUnknown {
    fn from(value: &RfcommOutboundConnectionInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommOutboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: RfcommOutboundConnectionInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommOutboundConnectionInformation> for ::windows::core::IInspectable {
    fn from(value: &RfcommOutboundConnectionInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RfcommOutboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommOutboundConnectionInformation {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
