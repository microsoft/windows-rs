#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBluetoothLEAdvertisementPublisherTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x610eca86_3480_41c9_a918_7ddadf207e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a3d025_c601_42d6_9829_4ccb3f5cd77f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTransmitPowerLevelInDBm: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBluetoothLEAdvertisementWatcherTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7db5ad7_2257_4e69_9784_fee645c1dce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    pub Advertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections")))]
    Advertisements: usize,
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ba03b18_0fec_436a_93b1_f46c697532a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerDetails2 {
    type Vtable = IGattCharacteristicNotificationTriggerDetails2_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicNotificationTriggerDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x727a50dc_949d_454a_b192_983467e3d50f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub EventTriggeringMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothEventTriggeringMode) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    pub ValueChangedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))]
    ValueChangedEvents: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa1b9b9_2f13_40b5_9582_8eb78e98ef13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderConnectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderConnectionStatics {
    type Vtable = IGattServiceProviderConnectionStatics_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderConnectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderConnectionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d509f4b_0b0e_4466_b8cd_6ebdda1fa17d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnectionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllServices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae8c0625_05ff_4afb_b16a_de95f3cf0158);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommConnectionTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IRfcommConnectionTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRfcommConnectionTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf922734d_2e3c_4efc_ab59_fc5cf96f97e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
    pub Incoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RemoteDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommInboundConnectionInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_Vtbl;
}
impl ::core::clone::Clone for IRfcommInboundConnectionInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRfcommInboundConnectionInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d3e75a8_5429_4059_92e3_1e8b65528707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommInboundConnectionInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SdpRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SdpRecord: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSdpRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSdpRecord: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub LocalServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    LocalServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetLocalServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IRfcommOutboundConnectionInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRfcommOutboundConnectionInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb091227b_f434_4cb0_99b1_4ab8cedaedd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommOutboundConnectionInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub RemoteServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    RemoteServiceId: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub SetRemoteServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    SetRemoteServiceId: usize,
}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(::windows::core::IUnknown);
impl BluetoothLEAdvertisementPublisherTriggerDetails {
    #[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Status(&self) -> ::windows::core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<i16>>();
            (::windows::core::Interface::vtable(this).SelectedTransmitPowerLevelInDBm)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails;{610eca86-3480-41c9-a918-7ddadf207e00})");
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BluetoothLEAdvertisementPublisherTriggerDetails {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementPublisherTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails";
}
::windows::imp::interface_hierarchy!(BluetoothLEAdvertisementPublisherTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(::windows::core::IUnknown);
impl BluetoothLEAdvertisementWatcherTriggerDetails {
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    pub fn Advertisements(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>();
            (::windows::core::Interface::vtable(this).Advertisements)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothSignalStrengthFilter>();
            (::windows::core::Interface::vtable(this).SignalStrengthFilter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails;{a7db5ad7-2257-4e69-9784-fee645c1dce0})");
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BluetoothLEAdvertisementWatcherTriggerDetails {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementWatcherTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails";
}
::windows::imp::interface_hierarchy!(BluetoothLEAdvertisementWatcherTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct GattCharacteristicNotificationTriggerDetails(::windows::core::IUnknown);
impl GattCharacteristicNotificationTriggerDetails {
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::GenericAttributeProfile::GattCharacteristic>();
            (::windows::core::Interface::vtable(this).Characteristic)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EventTriggeringMode(&self) -> ::windows::core::Result<BluetoothEventTriggeringMode> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BluetoothEventTriggeringMode>();
            (::windows::core::Interface::vtable(this).EventTriggeringMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    pub fn ValueChangedEvents(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>();
            (::windows::core::Interface::vtable(this).ValueChangedEvents)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for GattCharacteristicNotificationTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails;{9ba03b18-0fec-436a-93b1-f46c697532a2})");
}
impl ::core::clone::Clone for GattCharacteristicNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattCharacteristicNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IGattCharacteristicNotificationTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails";
}
::windows::imp::interface_hierarchy!(GattCharacteristicNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattCharacteristicNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for GattCharacteristicNotificationTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderConnection(::windows::core::IUnknown);
impl GattServiceProviderConnection {
    pub fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TriggerId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::GenericAttributeProfile::GattLocalService>();
            (::windows::core::Interface::vtable(this).Service)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllServices() -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>> {
        Self::IGattServiceProviderConnectionStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>>();
            (::windows::core::Interface::vtable(this).AllServices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderConnectionStatics<R, F: FnOnce(&IGattServiceProviderConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattServiceProviderConnection, IGattServiceProviderConnectionStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for GattServiceProviderConnection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderConnection;{7fa1b9b9-2f13-40b5-9582-8eb78e98ef13})");
}
impl ::core::clone::Clone for GattServiceProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProviderConnection {
    const IID: ::windows::core::GUID = <IGattServiceProviderConnection as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderConnection";
}
::windows::imp::interface_hierarchy!(GattServiceProviderConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProviderConnection {}
unsafe impl ::core::marker::Sync for GattServiceProviderConnection {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderTriggerDetails(::windows::core::IUnknown);
impl GattServiceProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows::core::Result<GattServiceProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattServiceProviderConnection>();
            (::windows::core::Interface::vtable(this).Connection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for GattServiceProviderTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails;{ae8c0625-05ff-4afb-b16a-de95f3cf0158})");
}
impl ::core::clone::Clone for GattServiceProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProviderTriggerDetails {
    const IID: ::windows::core::GUID = <IGattServiceProviderTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails";
}
::windows::imp::interface_hierarchy!(GattServiceProviderTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for GattServiceProviderTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct RfcommConnectionTriggerDetails(::windows::core::IUnknown);
impl RfcommConnectionTriggerDetails {
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn Socket(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Networking::Sockets::StreamSocket>();
            (::windows::core::Interface::vtable(this).Socket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Incoming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Incoming)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteDevice(&self) -> ::windows::core::Result<super::BluetoothDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothDevice>();
            (::windows::core::Interface::vtable(this).RemoteDevice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for RfcommConnectionTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails;{f922734d-2e3c-4efc-ab59-fc5cf96f97e3})");
}
impl ::core::clone::Clone for RfcommConnectionTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RfcommConnectionTriggerDetails {
    const IID: ::windows::core::GUID = <IRfcommConnectionTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails";
}
::windows::imp::interface_hierarchy!(RfcommConnectionTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RfcommConnectionTriggerDetails {}
unsafe impl ::core::marker::Sync for RfcommConnectionTriggerDetails {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct RfcommInboundConnectionInformation(::windows::core::IUnknown);
impl RfcommInboundConnectionInformation {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SdpRecord(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).SdpRecord)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSdpRecord<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSdpRecord)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn LocalServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Rfcomm::RfcommServiceId>();
            (::windows::core::Interface::vtable(this).LocalServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn SetLocalServiceId(&self, value: &super::Rfcomm::RfcommServiceId) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocalServiceId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ServiceCapabilities(&self) -> ::windows::core::Result<super::BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::BluetoothServiceCapabilities>();
            (::windows::core::Interface::vtable(this).ServiceCapabilities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServiceCapabilities)(::windows::core::Interface::as_raw(this), value).ok() }
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
impl ::windows::core::RuntimeType for RfcommInboundConnectionInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation;{6d3e75a8-5429-4059-92e3-1e8b65528707})");
}
impl ::core::clone::Clone for RfcommInboundConnectionInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RfcommInboundConnectionInformation {
    const IID: ::windows::core::GUID = <IRfcommInboundConnectionInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation";
}
::windows::imp::interface_hierarchy!(RfcommInboundConnectionInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RfcommInboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommInboundConnectionInformation {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
pub struct RfcommOutboundConnectionInformation(::windows::core::IUnknown);
impl RfcommOutboundConnectionInformation {
    #[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn RemoteServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Rfcomm::RfcommServiceId>();
            (::windows::core::Interface::vtable(this).RemoteServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn SetRemoteServiceId(&self, value: &super::Rfcomm::RfcommServiceId) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteServiceId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
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
impl ::windows::core::RuntimeType for RfcommOutboundConnectionInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation;{b091227b-f434-4cb0-99b1-4ab8cedaedd7})");
}
impl ::core::clone::Clone for RfcommOutboundConnectionInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RfcommOutboundConnectionInformation {
    const IID: ::windows::core::GUID = <IRfcommOutboundConnectionInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation";
}
::windows::imp::interface_hierarchy!(RfcommOutboundConnectionInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RfcommOutboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommOutboundConnectionInformation {}
#[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for BluetoothEventTriggeringMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BluetoothEventTriggeringMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothEventTriggeringMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BluetoothEventTriggeringMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Background.BluetoothEventTriggeringMode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
