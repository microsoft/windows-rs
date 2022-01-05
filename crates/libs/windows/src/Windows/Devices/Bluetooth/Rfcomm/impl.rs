#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServiceImpl: Sized {
    fn ConnectionHostName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn ConnectionServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn MaxProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn GetSdpRawAttributesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
    fn GetSdpRawAttributesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceService2Impl: Sized + IRfcommDeviceServiceImpl {
    fn Device(&self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceService3Impl: Sized + IRfcommDeviceServiceImpl + IRfcommDeviceService2Impl {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServiceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>>;
    fn GetDeviceSelector(&self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServiceStatics2Impl: Sized + IRfcommDeviceServiceStaticsImpl {
    fn GetDeviceSelectorForBluetoothDevice(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceWithCacheMode(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceId(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServicesResultImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceIdImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AsShortId(&self) -> ::windows::core::Result<u32>;
    fn AsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceIdStaticsImpl: Sized {
    fn FromUuid(&self, uuid: &::windows::core::GUID) -> ::windows::core::Result<RfcommServiceId>;
    fn FromShortId(&self, shortid: u32) -> ::windows::core::Result<RfcommServiceId>;
    fn SerialPort(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn ObexObjectPush(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn ObexFileTransfer(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn PhoneBookAccessPce(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn PhoneBookAccessPse(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn GenericFileTransfer(&self) -> ::windows::core::Result<RfcommServiceId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceProviderImpl: Sized {
    fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn SdpRawAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>>;
    fn StartAdvertising(&self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>) -> ::windows::core::Result<()>;
    fn StopAdvertising(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceProvider2Impl: Sized + IRfcommServiceProviderImpl {
    fn StartAdvertisingWithRadioDiscoverability(&self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>, radiodiscoverable: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceProviderStaticsImpl: Sized {
    fn CreateAsync(&self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>>;
}
