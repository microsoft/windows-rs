#[cfg(feature = "Devices_Bluetooth_Advertisement")]
pub mod Advertisement;
#[cfg(feature = "Devices_Bluetooth_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
pub mod GenericAttributeProfile;
#[cfg(feature = "Devices_Bluetooth_Rfcomm")]
pub mod Rfcomm;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothAdapter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothAdapter, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothAdapter {
    pub fn DeviceId(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn BluetoothAddress(&self) -> Result<u64, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsClassicSupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsClassicSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLowEnergySupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLowEnergySupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPeripheralRoleSupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPeripheralRoleSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCentralRoleSupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCentralRoleSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAdvertisementOffloadSupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAdvertisementOffloadSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Radios")]
    pub fn GetRadioAsync(&self) -> Result<windows_future::IAsyncOperation<super::Radios::Radio>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRadioAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AreClassicSecureConnectionsSupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreClassicSecureConnectionsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AreLowEnergySecureConnectionsSupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreLowEnergySecureConnectionsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsExtendedAdvertisingSupported(&self) -> Result<bool, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExtendedAdvertisingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxAdvertisementDataLength(&self) -> Result<u32, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAdvertisementDataLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeviceSelector() -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> Result<windows_future::IAsyncOperation<BluetoothAdapter>, windows_result::HRESULT> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> Result<windows_future::IAsyncOperation<BluetoothAdapter>, windows_result::HRESULT> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBluetoothAdapterStatics<R, F: FnOnce(&IBluetoothAdapterStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothAdapter, IBluetoothAdapterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothAdapter>();
}
unsafe impl windows_core::Interface for BluetoothAdapter {
    type Vtable = <IBluetoothAdapter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothAdapter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothAdapter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothAdapter";
}
unsafe impl Send for BluetoothAdapter {}
unsafe impl Sync for BluetoothAdapter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BluetoothAddressType(pub i32);
impl BluetoothAddressType {
    pub const Public: Self = Self(0i32);
    pub const Random: Self = Self(1i32);
    pub const Unspecified: Self = Self(2i32);
}
impl windows_core::TypeKind for BluetoothAddressType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothAddressType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothAddressType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BluetoothCacheMode(pub i32);
impl BluetoothCacheMode {
    pub const Cached: Self = Self(0i32);
    pub const Uncached: Self = Self(1i32);
}
impl windows_core::TypeKind for BluetoothCacheMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothCacheMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothCacheMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothClassOfDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothClassOfDevice, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothClassOfDevice {
    pub fn RawValue(&self) -> Result<u32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MajorClass(&self) -> Result<BluetoothMajorClass, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MajorClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinorClass(&self) -> Result<BluetoothMinorClass, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinorClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceCapabilities(&self) -> Result<BluetoothServiceCapabilities, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FromRawValue(rawvalue: u32) -> Result<BluetoothClassOfDevice, windows_result::HRESULT> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromRawValue)(windows_core::Interface::as_raw(this), rawvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromParts(majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities) -> Result<BluetoothClassOfDevice, windows_result::HRESULT> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromParts)(windows_core::Interface::as_raw(this), majorclass, minorclass, servicecapabilities, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBluetoothClassOfDeviceStatics<R, F: FnOnce(&IBluetoothClassOfDeviceStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothClassOfDevice, IBluetoothClassOfDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothClassOfDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothClassOfDevice>();
}
unsafe impl windows_core::Interface for BluetoothClassOfDevice {
    type Vtable = <IBluetoothClassOfDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothClassOfDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothClassOfDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothClassOfDevice";
}
unsafe impl Send for BluetoothClassOfDevice {}
unsafe impl Sync for BluetoothClassOfDevice {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BluetoothConnectionStatus(pub i32);
impl BluetoothConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl windows_core::TypeKind for BluetoothConnectionStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothConnectionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothConnectionStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BluetoothDevice, super::super::Foundation::IClosable);
impl BluetoothDevice {
    pub fn DeviceId(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Networking")]
    pub fn HostName(&self) -> Result<super::super::Networking::HostName, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ClassOfDevice(&self) -> Result<BluetoothClassOfDevice, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClassOfDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SdpRecords(&self) -> Result<windows_collections::IVectorView<super::super::Storage::Streams::IBuffer>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SdpRecords)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn RfcommServices(&self) -> Result<windows_collections::IVectorView<Rfcomm::RfcommDeviceService>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RfcommServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionStatus(&self) -> Result<BluetoothConnectionStatus, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BluetoothAddress(&self) -> Result<u64, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NameChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NameChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNameChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNameChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SdpRecordsChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SdpRecordsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSdpRecordsChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSdpRecordsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ConnectionStatusChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionStatusChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> Result<super::Enumeration::DeviceInformation, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> Result<super::Enumeration::DeviceAccessInformation, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAccessInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RequestAccessAsync(&self) -> Result<windows_future::IAsyncOperation<super::Enumeration::DeviceAccessStatus>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn GetRfcommServicesAsync(&self) -> Result<windows_future::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRfcommServicesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn GetRfcommServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> Result<windows_future::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRfcommServicesWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn GetRfcommServicesForIdAsync<P0>(&self, serviceid: P0) -> Result<windows_future::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>, windows_result::HRESULT>
    where
        P0: windows_core::Param<Rfcomm::RfcommServiceId>,
    {
        let this = &windows_core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRfcommServicesForIdAsync)(windows_core::Interface::as_raw(this), serviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub fn GetRfcommServicesForIdWithCacheModeAsync<P0>(&self, serviceid: P0, cachemode: BluetoothCacheMode) -> Result<windows_future::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>, windows_result::HRESULT>
    where
        P0: windows_core::Param<Rfcomm::RfcommServiceId>,
    {
        let this = &windows_core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRfcommServicesForIdWithCacheModeAsync)(windows_core::Interface::as_raw(this), serviceid.param().abi(), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BluetoothDeviceId(&self) -> Result<BluetoothDeviceId, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothDevice4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WasSecureConnectionUsedForPairing(&self) -> Result<bool, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothDevice5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WasSecureConnectionUsedForPairing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> Result<windows_future::IAsyncOperation<BluetoothDevice>, windows_result::HRESULT> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Networking")]
    pub fn FromHostNameAsync<P0>(hostname: P0) -> Result<windows_future::IAsyncOperation<BluetoothDevice>, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Networking::HostName>,
    {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromHostNameAsync)(windows_core::Interface::as_raw(this), hostname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromBluetoothAddressAsync(address: u64) -> Result<windows_future::IAsyncOperation<BluetoothDevice>, windows_result::HRESULT> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBluetoothAddressAsync)(windows_core::Interface::as_raw(this), address, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromPairingState)(windows_core::Interface::as_raw(this), pairingstate, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromConnectionStatus)(windows_core::Interface::as_raw(this), connectionstatus, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromDeviceName(devicename: &windows_core::HSTRING) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromDeviceName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(devicename), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromBluetoothAddress)(windows_core::Interface::as_raw(this), bluetoothaddress, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromClassOfDevice<P0>(classofdevice: P0) -> Result<windows_core::HSTRING, windows_result::HRESULT>
    where
        P0: windows_core::Param<BluetoothClassOfDevice>,
    {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromClassOfDevice)(windows_core::Interface::as_raw(this), classofdevice.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn Close(&self) -> Result<(), windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    fn IBluetoothDeviceStatics<R, F: FnOnce(&IBluetoothDeviceStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBluetoothDeviceStatics2<R, F: FnOnce(&IBluetoothDeviceStatics2) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothDevice>();
}
unsafe impl windows_core::Interface for BluetoothDevice {
    type Vtable = <IBluetoothDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDevice";
}
unsafe impl Send for BluetoothDevice {}
unsafe impl Sync for BluetoothDevice {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothDeviceId(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothDeviceId, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothDeviceId {
    pub fn Id(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsClassicDevice(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsClassicDevice)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLowEnergyDevice(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLowEnergyDevice)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FromId(deviceid: &windows_core::HSTRING) -> Result<BluetoothDeviceId, windows_result::HRESULT> {
        Self::IBluetoothDeviceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBluetoothDeviceIdStatics<R, F: FnOnce(&IBluetoothDeviceIdStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothDeviceId, IBluetoothDeviceIdStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothDeviceId>();
}
unsafe impl windows_core::Interface for BluetoothDeviceId {
    type Vtable = <IBluetoothDeviceId as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothDeviceId as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothDeviceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDeviceId";
}
unsafe impl Send for BluetoothDeviceId {}
unsafe impl Sync for BluetoothDeviceId {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::TypeKind for BluetoothError {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothError;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothLEAppearance(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAppearance, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEAppearance {
    pub fn RawValue(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Category(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Category)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubCategory(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FromRawValue(rawvalue: u16) -> Result<BluetoothLEAppearance, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromRawValue)(windows_core::Interface::as_raw(this), rawvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromParts(appearancecategory: u16, appearancesubcategory: u16) -> Result<BluetoothLEAppearance, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromParts)(windows_core::Interface::as_raw(this), appearancecategory, appearancesubcategory, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBluetoothLEAppearanceStatics<R, F: FnOnce(&IBluetoothLEAppearanceStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAppearance, IBluetoothLEAppearanceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEAppearance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAppearance>();
}
unsafe impl windows_core::Interface for BluetoothLEAppearance {
    type Vtable = <IBluetoothLEAppearance as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothLEAppearance as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAppearance {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearance";
}
unsafe impl Send for BluetoothLEAppearance {}
unsafe impl Sync for BluetoothLEAppearance {}
pub struct BluetoothLEAppearanceCategories;
impl BluetoothLEAppearanceCategories {
    pub fn Uncategorized() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uncategorized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Phone() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Phone)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Computer() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Computer)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Watch() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Watch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Clock() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clock)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Display() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Display)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RemoteControl() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteControl)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EyeGlasses() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EyeGlasses)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Tag() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Keyring() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Keyring)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediaPlayer() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlayer)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BarcodeScanner() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BarcodeScanner)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Thermometer() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thermometer)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HeartRate() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeartRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BloodPressure() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BloodPressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HumanInterfaceDevice() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HumanInterfaceDevice)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GlucoseMeter() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GlucoseMeter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RunningWalking() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunningWalking)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Cycling() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cycling)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PulseOximeter() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PulseOximeter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn WeightScale() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WeightScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OutdoorSportActivity() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutdoorSportActivity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IBluetoothLEAppearanceCategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceCategoriesStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAppearanceCategories, IBluetoothLEAppearanceCategoriesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BluetoothLEAppearanceCategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories";
}
pub struct BluetoothLEAppearanceSubcategories;
impl BluetoothLEAppearanceSubcategories {
    pub fn Generic() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Generic)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SportsWatch() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SportsWatch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ThermometerEar() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThermometerEar)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HeartRateBelt() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeartRateBelt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BloodPressureArm() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BloodPressureArm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BloodPressureWrist() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BloodPressureWrist)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Keyboard() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Keyboard)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Mouse() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mouse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Joystick() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Joystick)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gamepad() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gamepad)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DigitizerTablet() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DigitizerTablet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CardReader() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardReader)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DigitalPen() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DigitalPen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BarcodeScanner() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BarcodeScanner)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RunningWalkingInShoe() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunningWalkingInShoe)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RunningWalkingOnShoe() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunningWalkingOnShoe)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RunningWalkingOnHip() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunningWalkingOnHip)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingComputer() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingComputer)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingSpeedSensor() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingSpeedSensor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingCadenceSensor() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingCadenceSensor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingPowerSensor() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingPowerSensor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CyclingSpeedCadenceSensor() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CyclingSpeedCadenceSensor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OximeterFingertip() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OximeterFingertip)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OximeterWristWorn() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OximeterWristWorn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LocationDisplay() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationDisplay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LocationNavigationDisplay() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationNavigationDisplay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LocationPod() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationPod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LocationNavigationPod() -> Result<u16, windows_result::HRESULT> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationNavigationPod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IBluetoothLEAppearanceSubcategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceSubcategoriesStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAppearanceSubcategories, IBluetoothLEAppearanceSubcategoriesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BluetoothLEAppearanceSubcategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothLEConnectionParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEConnectionParameters, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEConnectionParameters {
    pub fn LinkTimeout(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinkTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConnectionLatency(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConnectionInterval(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEConnectionParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEConnectionParameters>();
}
unsafe impl windows_core::Interface for BluetoothLEConnectionParameters {
    type Vtable = <IBluetoothLEConnectionParameters as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothLEConnectionParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionParameters";
}
unsafe impl Send for BluetoothLEConnectionParameters {}
unsafe impl Sync for BluetoothLEConnectionParameters {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothLEConnectionPhy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEConnectionPhy, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEConnectionPhy {
    pub fn TransmitInfo(&self) -> Result<BluetoothLEConnectionPhyInfo, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReceiveInfo(&self) -> Result<BluetoothLEConnectionPhyInfo, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceiveInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEConnectionPhy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEConnectionPhy>();
}
unsafe impl windows_core::Interface for BluetoothLEConnectionPhy {
    type Vtable = <IBluetoothLEConnectionPhy as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothLEConnectionPhy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEConnectionPhy {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhy";
}
unsafe impl Send for BluetoothLEConnectionPhy {}
unsafe impl Sync for BluetoothLEConnectionPhy {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothLEConnectionPhyInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEConnectionPhyInfo, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEConnectionPhyInfo {
    pub fn IsUncoded1MPhy(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUncoded1MPhy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUncoded2MPhy(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUncoded2MPhy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCodedPhy(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCodedPhy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BluetoothLEConnectionPhyInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEConnectionPhyInfo>();
}
unsafe impl windows_core::Interface for BluetoothLEConnectionPhyInfo {
    type Vtable = <IBluetoothLEConnectionPhyInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothLEConnectionPhyInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEConnectionPhyInfo {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo";
}
unsafe impl Send for BluetoothLEConnectionPhyInfo {}
unsafe impl Sync for BluetoothLEConnectionPhyInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothLEDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BluetoothLEDevice, super::super::Foundation::IClosable);
impl BluetoothLEDevice {
    pub fn DeviceId(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Name(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn GattServices(&self) -> Result<windows_collections::IVectorView<GenericAttributeProfile::GattDeviceService>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GattServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionStatus(&self) -> Result<BluetoothConnectionStatus, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BluetoothAddress(&self) -> Result<u64, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn GetGattService(&self, serviceuuid: windows_core::GUID) -> Result<GenericAttributeProfile::GattDeviceService, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGattService)(windows_core::Interface::as_raw(this), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NameChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NameChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNameChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNameChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GattServicesChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GattServicesChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGattServicesChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGattServicesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ConnectionStatusChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionStatusChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> Result<super::Enumeration::DeviceInformation, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Appearance(&self) -> Result<BluetoothLEAppearance, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appearance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BluetoothAddressType(&self) -> Result<BluetoothAddressType, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothAddressType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> Result<super::Enumeration::DeviceAccessInformation, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAccessInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RequestAccessAsync(&self) -> Result<windows_future::IAsyncOperation<super::Enumeration::DeviceAccessStatus>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn GetGattServicesAsync(&self) -> Result<windows_future::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGattServicesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn GetGattServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> Result<windows_future::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGattServicesWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn GetGattServicesForUuidAsync(&self, serviceuuid: windows_core::GUID) -> Result<windows_future::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGattServicesForUuidAsync)(windows_core::Interface::as_raw(this), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn GetGattServicesForUuidWithCacheModeAsync(&self, serviceuuid: windows_core::GUID, cachemode: BluetoothCacheMode) -> Result<windows_future::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGattServicesForUuidWithCacheModeAsync)(windows_core::Interface::as_raw(this), serviceuuid, cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BluetoothDeviceId(&self) -> Result<BluetoothDeviceId, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluetoothDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WasSecureConnectionUsedForPairing(&self) -> Result<bool, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WasSecureConnectionUsedForPairing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetConnectionParameters(&self) -> Result<BluetoothLEConnectionParameters, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionParameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetConnectionPhy(&self) -> Result<BluetoothLEConnectionPhy, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionPhy)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestPreferredConnectionParameters<P0>(&self, preferredconnectionparameters: P0) -> Result<BluetoothLEPreferredConnectionParametersRequest, windows_result::HRESULT>
    where
        P0: windows_core::Param<BluetoothLEPreferredConnectionParameters>,
    {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPreferredConnectionParameters)(windows_core::Interface::as_raw(this), preferredconnectionparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionParametersChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionParametersChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionParametersChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionParametersChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ConnectionPhyChanged<P0>(&self, handler: P0) -> Result<i64, windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionPhyChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionPhyChanged(&self, token: i64) -> Result<(), windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionPhyChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> Result<windows_future::IAsyncOperation<BluetoothLEDevice>, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromBluetoothAddressAsync(bluetoothaddress: u64) -> Result<windows_future::IAsyncOperation<BluetoothLEDevice>, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBluetoothAddressAsync)(windows_core::Interface::as_raw(this), bluetoothaddress, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromPairingState)(windows_core::Interface::as_raw(this), pairingstate, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromConnectionStatus)(windows_core::Interface::as_raw(this), connectionstatus, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromDeviceName(devicename: &windows_core::HSTRING) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromDeviceName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(devicename), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromBluetoothAddress)(windows_core::Interface::as_raw(this), bluetoothaddress, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType)(windows_core::Interface::as_raw(this), bluetoothaddress, bluetoothaddresstype, &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromAppearance<P0>(appearance: P0) -> Result<windows_core::HSTRING, windows_result::HRESULT>
    where
        P0: windows_core::Param<BluetoothLEAppearance>,
    {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromAppearance)(windows_core::Interface::as_raw(this), appearance.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromBluetoothAddressWithBluetoothAddressTypeAsync(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> Result<windows_future::IAsyncOperation<BluetoothLEDevice>, windows_result::HRESULT> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBluetoothAddressWithBluetoothAddressTypeAsync)(windows_core::Interface::as_raw(this), bluetoothaddress, bluetoothaddresstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> Result<(), windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    fn IBluetoothLEDeviceStatics<R, F: FnOnce(&IBluetoothLEDeviceStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBluetoothLEDeviceStatics2<R, F: FnOnce(&IBluetoothLEDeviceStatics2) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEDevice>();
}
unsafe impl windows_core::Interface for BluetoothLEDevice {
    type Vtable = <IBluetoothLEDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothLEDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEDevice";
}
unsafe impl Send for BluetoothLEDevice {}
unsafe impl Sync for BluetoothLEDevice {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothLEPreferredConnectionParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEPreferredConnectionParameters, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothLEPreferredConnectionParameters {
    pub fn LinkTimeout(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinkTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConnectionLatency(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinConnectionInterval(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinConnectionInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxConnectionInterval(&self) -> Result<u16, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxConnectionInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Balanced() -> Result<BluetoothLEPreferredConnectionParameters, windows_result::HRESULT> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Balanced)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ThroughputOptimized() -> Result<BluetoothLEPreferredConnectionParameters, windows_result::HRESULT> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThroughputOptimized)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PowerOptimized() -> Result<BluetoothLEPreferredConnectionParameters, windows_result::HRESULT> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerOptimized)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBluetoothLEPreferredConnectionParametersStatics<R, F: FnOnce(&IBluetoothLEPreferredConnectionParametersStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEPreferredConnectionParameters, IBluetoothLEPreferredConnectionParametersStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BluetoothLEPreferredConnectionParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEPreferredConnectionParameters>();
}
unsafe impl windows_core::Interface for BluetoothLEPreferredConnectionParameters {
    type Vtable = <IBluetoothLEPreferredConnectionParameters as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothLEPreferredConnectionParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEPreferredConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters";
}
unsafe impl Send for BluetoothLEPreferredConnectionParameters {}
unsafe impl Sync for BluetoothLEPreferredConnectionParameters {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothLEPreferredConnectionParametersRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEPreferredConnectionParametersRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BluetoothLEPreferredConnectionParametersRequest, super::super::Foundation::IClosable);
impl BluetoothLEPreferredConnectionParametersRequest {
    pub fn Status(&self) -> Result<BluetoothLEPreferredConnectionParametersRequestStatus, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> Result<(), windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BluetoothLEPreferredConnectionParametersRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEPreferredConnectionParametersRequest>();
}
unsafe impl windows_core::Interface for BluetoothLEPreferredConnectionParametersRequest {
    type Vtable = <IBluetoothLEPreferredConnectionParametersRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothLEPreferredConnectionParametersRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEPreferredConnectionParametersRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest";
}
unsafe impl Send for BluetoothLEPreferredConnectionParametersRequest {}
unsafe impl Sync for BluetoothLEPreferredConnectionParametersRequest {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(pub i32);
impl BluetoothLEPreferredConnectionParametersRequestStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl windows_core::TypeKind for BluetoothLEPreferredConnectionParametersRequestStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothLEPreferredConnectionParametersRequestStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequestStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::TypeKind for BluetoothMajorClass {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothMajorClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMajorClass;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::TypeKind for BluetoothMinorClass {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothMinorClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMinorClass;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::TypeKind for BluetoothServiceCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BluetoothServiceCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothServiceCapabilities;u4)");
}
impl BluetoothServiceCapabilities {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BluetoothServiceCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BluetoothServiceCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BluetoothServiceCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothSignalStrengthFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothSignalStrengthFilter, windows_core::IUnknown, windows_core::IInspectable);
impl BluetoothSignalStrengthFilter {
    pub fn new() -> Result<Self, windows_result::HRESULT> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothSignalStrengthFilter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn InRangeThresholdInDBm(&self) -> Result<super::super::Foundation::IReference<i16>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InRangeThresholdInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInRangeThresholdInDBm<P0>(&self, value: P0) -> Result<(), windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i16>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInRangeThresholdInDBm)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OutOfRangeThresholdInDBm(&self) -> Result<super::super::Foundation::IReference<i16>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutOfRangeThresholdInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutOfRangeThresholdInDBm<P0>(&self, value: P0) -> Result<(), windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i16>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutOfRangeThresholdInDBm)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OutOfRangeTimeout(&self) -> Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutOfRangeTimeout)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutOfRangeTimeout<P0>(&self, value: P0) -> Result<(), windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutOfRangeTimeout)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SamplingInterval(&self) -> Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SamplingInterval)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSamplingInterval<P0>(&self, value: P0) -> Result<(), windows_result::HRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSamplingInterval)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for BluetoothSignalStrengthFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothSignalStrengthFilter>();
}
unsafe impl windows_core::Interface for BluetoothSignalStrengthFilter {
    type Vtable = <IBluetoothSignalStrengthFilter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBluetoothSignalStrengthFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothSignalStrengthFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter";
}
unsafe impl Send for BluetoothSignalStrengthFilter {}
unsafe impl Sync for BluetoothSignalStrengthFilter {}
pub struct BluetoothUuidHelper;
impl BluetoothUuidHelper {
    pub fn FromShortId(shortid: u32) -> Result<windows_core::GUID, windows_result::HRESULT> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromShortId)(windows_core::Interface::as_raw(this), shortid, &mut result__).map(|| result__)
        })
    }
    pub fn TryGetShortId(uuid: windows_core::GUID) -> Result<super::super::Foundation::IReference<u32>, windows_result::HRESULT> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetShortId)(windows_core::Interface::as_raw(this), uuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBluetoothUuidHelperStatics<R, F: FnOnce(&IBluetoothUuidHelperStatics) -> Result<R, windows_result::HRESULT>>(callback: F) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothUuidHelper, IBluetoothUuidHelperStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BluetoothUuidHelper {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothUuidHelper";
}
windows_core::imp::define_interface!(IBluetoothAdapter, IBluetoothAdapter_Vtbl, 0x7974f04c_5f7a_4a34_9225_a855f84b1a8b);
impl windows_core::RuntimeType for IBluetoothAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IsClassicSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLowEnergySupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPeripheralRoleSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCentralRoleSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAdvertisementOffloadSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Radios")]
    pub GetRadioAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Radios"))]
    GetRadioAsync: usize,
}
windows_core::imp::define_interface!(IBluetoothAdapter2, IBluetoothAdapter2_Vtbl, 0xac94cecc_24d5_41b3_916d_1097c50b102b);
impl windows_core::RuntimeType for IBluetoothAdapter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AreClassicSecureConnectionsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AreLowEnergySecureConnectionsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothAdapter3, IBluetoothAdapter3_Vtbl, 0x8f8624e0_cba9_5211_9f89_3aac62b4c6b8);
impl windows_core::RuntimeType for IBluetoothAdapter3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsExtendedAdvertisingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxAdvertisementDataLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothAdapterStatics, IBluetoothAdapterStatics_Vtbl, 0x8b02fb6a_ac4c_4741_8661_8eab7d17ea9f);
impl windows_core::RuntimeType for IBluetoothAdapterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothClassOfDevice, IBluetoothClassOfDevice_Vtbl, 0xd640227e_d7d7_4661_9454_65039ca17a2b);
impl windows_core::RuntimeType for IBluetoothClassOfDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MajorClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothMajorClass) -> windows_core::HRESULT,
    pub MinorClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothMinorClass) -> windows_core::HRESULT,
    pub ServiceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothServiceCapabilities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothClassOfDeviceStatics, IBluetoothClassOfDeviceStatics_Vtbl, 0xe46135bd_0fa2_416c_91b4_c1e48ca061c1);
impl windows_core::RuntimeType for IBluetoothClassOfDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromParts: unsafe extern "system" fn(*mut core::ffi::c_void, BluetoothMajorClass, BluetoothMinorClass, BluetoothServiceCapabilities, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothDevice, IBluetoothDevice_Vtbl, 0x2335b156_90d2_4a04_aef5_0e20b9e6b707);
impl windows_core::RuntimeType for IBluetoothDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Networking")]
    pub HostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    HostName: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassOfDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SdpRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SdpRecords: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub RfcommServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    RfcommServices: usize,
    pub ConnectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothConnectionStatus) -> windows_core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub NameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SdpRecordsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSdpRecordsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ConnectionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothDevice2, IBluetoothDevice2_Vtbl, 0x0133f954_b156_4dd0_b1f5_c11bc31a5163);
impl windows_core::RuntimeType for IBluetoothDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
windows_core::imp::define_interface!(IBluetoothDevice3, IBluetoothDevice3_Vtbl, 0x57fff78b_651a_4454_b90f_eb21ef0b0d71);
impl windows_core::RuntimeType for IBluetoothDevice3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub GetRfcommServicesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    GetRfcommServicesAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub GetRfcommServicesWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    GetRfcommServicesWithCacheModeAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub GetRfcommServicesForIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    GetRfcommServicesForIdAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    pub GetRfcommServicesForIdWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))]
    GetRfcommServicesForIdWithCacheModeAsync: usize,
}
windows_core::imp::define_interface!(IBluetoothDevice4, IBluetoothDevice4_Vtbl, 0x817c34ad_0e9c_42b2_a8dc_3e8094940d12);
impl windows_core::RuntimeType for IBluetoothDevice4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BluetoothDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothDevice5, IBluetoothDevice5_Vtbl, 0xb5e0b385_5e85_4559_a10d_1c7281379f96);
impl windows_core::RuntimeType for IBluetoothDevice5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WasSecureConnectionUsedForPairing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothDeviceId, IBluetoothDeviceId_Vtbl, 0xc17949af_57c1_4642_bcce_e6c06b20ae76);
impl windows_core::RuntimeType for IBluetoothDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClassicDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLowEnergyDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothDeviceIdStatics, IBluetoothDeviceIdStatics_Vtbl, 0xa7884e67_3efb_4f31_bbc2_810e09977404);
impl windows_core::RuntimeType for IBluetoothDeviceIdStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothDeviceStatics, IBluetoothDeviceStatics_Vtbl, 0x0991df51_57db_4725_bbd7_84f64327ec2c);
impl windows_core::RuntimeType for IBluetoothDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Networking")]
    pub FromHostNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    FromHostNameAsync: usize,
    pub FromBluetoothAddressAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothDeviceStatics2, IBluetoothDeviceStatics2_Vtbl, 0xc29e8e2f_4e14_4477_aa1b_b8b47e5b7ece);
impl windows_core::RuntimeType for IBluetoothDeviceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelectorFromPairingState: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromConnectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, BluetoothConnectionStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddress: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromClassOfDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAppearance, IBluetoothLEAppearance_Vtbl, 0x5d2079f2_66a8_4258_985e_02b4d9509f18);
impl windows_core::RuntimeType for IBluetoothLEAppearance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearance_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SubCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAppearanceCategoriesStatics, IBluetoothLEAppearanceCategoriesStatics_Vtbl, 0x6d4d54fe_046a_4185_aab6_824cf0610861);
impl windows_core::RuntimeType for IBluetoothLEAppearanceCategoriesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceCategoriesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uncategorized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Phone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Computer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Watch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Clock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Display: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub RemoteControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub EyeGlasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Keyring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub BarcodeScanner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Thermometer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub HeartRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub BloodPressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub HumanInterfaceDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GlucoseMeter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub RunningWalking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Cycling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub PulseOximeter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub WeightScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub OutdoorSportActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAppearanceStatics, IBluetoothLEAppearanceStatics_Vtbl, 0xa193c0c7_4504_4f4a_9ba5_cd1054e5e065);
impl windows_core::RuntimeType for IBluetoothLEAppearanceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromParts: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAppearanceSubcategoriesStatics, IBluetoothLEAppearanceSubcategoriesStatics_Vtbl, 0xe57ba606_2144_415a_8312_71ccf291f8d1);
impl windows_core::RuntimeType for IBluetoothLEAppearanceSubcategoriesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Generic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SportsWatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ThermometerEar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub HeartRateBelt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub BloodPressureArm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub BloodPressureWrist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Keyboard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Mouse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Joystick: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Gamepad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub DigitizerTablet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub CardReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub DigitalPen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub BarcodeScanner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub RunningWalkingInShoe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub RunningWalkingOnShoe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub RunningWalkingOnHip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub CyclingComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub CyclingSpeedSensor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub CyclingCadenceSensor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub CyclingPowerSensor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub CyclingSpeedCadenceSensor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub OximeterFingertip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub OximeterWristWorn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub LocationDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub LocationNavigationDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub LocationPod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub LocationNavigationPod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEConnectionParameters, IBluetoothLEConnectionParameters_Vtbl, 0x33cb0771_8da9_508f_a366_1ca388c929ab);
impl windows_core::RuntimeType for IBluetoothLEConnectionParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LinkTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ConnectionLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ConnectionInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEConnectionPhy, IBluetoothLEConnectionPhy_Vtbl, 0x781e5e48_621e_5a7e_8be6_1b9561ff63c9);
impl windows_core::RuntimeType for IBluetoothLEConnectionPhy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransmitInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReceiveInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEConnectionPhyInfo, IBluetoothLEConnectionPhyInfo_Vtbl, 0x9a100bdd_602e_5c27_a1ae_b230015a6394);
impl windows_core::RuntimeType for IBluetoothLEConnectionPhyInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsUncoded1MPhy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUncoded2MPhy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCodedPhy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEDevice, IBluetoothLEDevice_Vtbl, 0xb5ee2f7b_4ad8_4642_ac48_80a0b500e887);
impl windows_core::RuntimeType for IBluetoothLEDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub GattServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    GattServices: usize,
    pub ConnectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothConnectionStatus) -> windows_core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub GetGattService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    GetGattService: usize,
    pub NameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GattServicesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGattServicesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ConnectionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEDevice2, IBluetoothLEDevice2_Vtbl, 0x26f062b3_7aee_4d31_baba_b1b9775f5916);
impl windows_core::RuntimeType for IBluetoothLEDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    pub Appearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BluetoothAddressType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothAddressType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEDevice3, IBluetoothLEDevice3_Vtbl, 0xaee9e493_44ac_40dc_af33_b2c13c01ca46);
impl windows_core::RuntimeType for IBluetoothLEDevice3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub GetGattServicesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    GetGattServicesAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub GetGattServicesWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    GetGattServicesWithCacheModeAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub GetGattServicesForUuidAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    GetGattServicesForUuidAsync: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub GetGattServicesForUuidWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    GetGattServicesForUuidWithCacheModeAsync: usize,
}
windows_core::imp::define_interface!(IBluetoothLEDevice4, IBluetoothLEDevice4_Vtbl, 0x2b605031_2248_4b2f_acf0_7cee36fc5870);
impl windows_core::RuntimeType for IBluetoothLEDevice4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BluetoothDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEDevice5, IBluetoothLEDevice5_Vtbl, 0x9d6a1260_5287_458e_95ba_17c8b7bb326e);
impl windows_core::RuntimeType for IBluetoothLEDevice5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WasSecureConnectionUsedForPairing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEDevice6, IBluetoothLEDevice6_Vtbl, 0xca7190ef_0cae_573c_a1ca_e1fc5bfc39e2);
impl windows_core::RuntimeType for IBluetoothLEDevice6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetConnectionParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConnectionPhy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestPreferredConnectionParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionParametersChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveConnectionParametersChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ConnectionPhyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveConnectionPhyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEDeviceStatics, IBluetoothLEDeviceStatics_Vtbl, 0xc8cf1a19_f0b6_4bf0_8689_41303de2d9f4);
impl windows_core::RuntimeType for IBluetoothLEDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromBluetoothAddressAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEDeviceStatics2, IBluetoothLEDeviceStatics2_Vtbl, 0x5f12c06b_3bac_43e8_ad16_563271bd41c2);
impl windows_core::RuntimeType for IBluetoothLEDeviceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelectorFromPairingState: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromConnectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, BluetoothConnectionStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddress: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType: unsafe extern "system" fn(*mut core::ffi::c_void, u64, BluetoothAddressType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromAppearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromBluetoothAddressWithBluetoothAddressTypeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u64, BluetoothAddressType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEPreferredConnectionParameters, IBluetoothLEPreferredConnectionParameters_Vtbl, 0xf2f44344_7372_5f7b_9b34_29c944f5a715);
impl windows_core::RuntimeType for IBluetoothLEPreferredConnectionParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LinkTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ConnectionLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub MinConnectionInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub MaxConnectionInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEPreferredConnectionParametersRequest, IBluetoothLEPreferredConnectionParametersRequest_Vtbl, 0x8a375276_a528_5266_b661_cce6a5ff9739);
impl windows_core::RuntimeType for IBluetoothLEPreferredConnectionParametersRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEPreferredConnectionParametersStatics, IBluetoothLEPreferredConnectionParametersStatics_Vtbl, 0x0e3e8edc_2751_55aa_a838_8faeee818d72);
impl windows_core::RuntimeType for IBluetoothLEPreferredConnectionParametersStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Balanced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThroughputOptimized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PowerOptimized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothSignalStrengthFilter, IBluetoothSignalStrengthFilter_Vtbl, 0xdf7b7391_6bb5_4cfe_90b1_5d7324edcf7f);
impl windows_core::RuntimeType for IBluetoothSignalStrengthFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothSignalStrengthFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InRangeThresholdInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInRangeThresholdInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutOfRangeThresholdInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutOfRangeThresholdInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutOfRangeTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutOfRangeTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SamplingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSamplingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothUuidHelperStatics, IBluetoothUuidHelperStatics_Vtbl, 0x17df0cd8_cf74_4b21_afe6_f57a11bcdea0);
impl windows_core::RuntimeType for IBluetoothUuidHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothUuidHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromShortId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TryGetShortId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
