#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkInEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Pipe(&self) -> ::windows::core::Result<UsbBulkInPipe>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkInPipeImpl: Sized {
    fn MaxTransferSizeBytes(&self) -> ::windows::core::Result<u32>;
    fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkInEndpointDescriptor>;
    fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetReadOptions(&self, value: UsbReadOptions) -> ::windows::core::Result<()>;
    fn ReadOptions(&self) -> ::windows::core::Result<UsbReadOptions>;
    fn FlushBuffer(&self) -> ::windows::core::Result<()>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkOutEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Pipe(&self) -> ::windows::core::Result<UsbBulkOutPipe>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkOutPipeImpl: Sized {
    fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkOutEndpointDescriptor>;
    fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetWriteOptions(&self, value: UsbWriteOptions) -> ::windows::core::Result<()>;
    fn WriteOptions(&self) -> ::windows::core::Result<UsbWriteOptions>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationImpl: Sized {
    fn UsbInterfaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterface>>;
    fn ConfigurationDescriptor(&self) -> ::windows::core::Result<UsbConfigurationDescriptor>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationDescriptorImpl: Sized {
    fn ConfigurationValue(&self) -> ::windows::core::Result<u8>;
    fn MaxPowerMilliamps(&self) -> ::windows::core::Result<u32>;
    fn SelfPowered(&self) -> ::windows::core::Result<bool>;
    fn RemoteWakeup(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationDescriptorStaticsImpl: Sized {
    fn TryParse(&self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbConfigurationDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbConfigurationDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbControlRequestTypeImpl: Sized {
    fn Direction(&self) -> ::windows::core::Result<UsbTransferDirection>;
    fn SetDirection(&self, value: UsbTransferDirection) -> ::windows::core::Result<()>;
    fn ControlTransferType(&self) -> ::windows::core::Result<UsbControlTransferType>;
    fn SetControlTransferType(&self, value: UsbControlTransferType) -> ::windows::core::Result<()>;
    fn Recipient(&self) -> ::windows::core::Result<UsbControlRecipient>;
    fn SetRecipient(&self, value: UsbControlRecipient) -> ::windows::core::Result<()>;
    fn AsByte(&self) -> ::windows::core::Result<u8>;
    fn SetAsByte(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDescriptorImpl: Sized {
    fn Length(&self) -> ::windows::core::Result<u8>;
    fn DescriptorType(&self) -> ::windows::core::Result<u8>;
    fn ReadDescriptorBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUsbDeviceImpl: Sized + IClosableImpl {
    fn SendControlOutTransferAsync(&self, setuppacket: &::core::option::Option<UsbSetupPacket>, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn SendControlOutTransferAsyncNoBuffer(&self, setuppacket: &::core::option::Option<UsbSetupPacket>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn SendControlInTransferAsync(&self, setuppacket: &::core::option::Option<UsbSetupPacket>, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn SendControlInTransferAsyncNoBuffer(&self, setuppacket: &::core::option::Option<UsbSetupPacket>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn DefaultInterface(&self) -> ::windows::core::Result<UsbInterface>;
    fn DeviceDescriptor(&self) -> ::windows::core::Result<UsbDeviceDescriptor>;
    fn Configuration(&self) -> ::windows::core::Result<UsbConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceClassImpl: Sized {
    fn ClassCode(&self) -> ::windows::core::Result<u8>;
    fn SetClassCode(&self, value: u8) -> ::windows::core::Result<()>;
    fn SubclassCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
    fn SetSubclassCode(&self, value: &::core::option::Option<super::super::Foundation::IReference<u8>>) -> ::windows::core::Result<()>;
    fn ProtocolCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
    fn SetProtocolCode(&self, value: &::core::option::Option<super::super::Foundation::IReference<u8>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceClassesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceClassesStaticsImpl: Sized {
    fn CdcControl(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn Physical(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn PersonalHealthcare(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn ActiveSync(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn PalmSync(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn DeviceFirmwareUpdate(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn Irda(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn Measurement(&self) -> ::windows::core::Result<UsbDeviceClass>;
    fn VendorSpecific(&self) -> ::windows::core::Result<UsbDeviceClass>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceDescriptorImpl: Sized {
    fn BcdUsb(&self) -> ::windows::core::Result<u32>;
    fn MaxPacketSize0(&self) -> ::windows::core::Result<u8>;
    fn VendorId(&self) -> ::windows::core::Result<u32>;
    fn ProductId(&self) -> ::windows::core::Result<u32>;
    fn BcdDeviceRevision(&self) -> ::windows::core::Result<u32>;
    fn NumberOfConfigurations(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, vendorid: u32, productid: u32, winusbinterfaceclass: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorGuidOnly(&self, winusbinterfaceclass: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorVidPidOnly(&self, vendorid: u32, productid: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceClassSelector(&self, usbclass: &::core::option::Option<UsbDeviceClass>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UsbDevice>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbEndpointDescriptorImpl: Sized {
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Direction(&self) -> ::windows::core::Result<UsbTransferDirection>;
    fn EndpointType(&self) -> ::windows::core::Result<UsbEndpointType>;
    fn AsBulkInEndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkInEndpointDescriptor>;
    fn AsInterruptInEndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptInEndpointDescriptor>;
    fn AsBulkOutEndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkOutEndpointDescriptor>;
    fn AsInterruptOutEndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptOutEndpointDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbEndpointDescriptorStaticsImpl: Sized {
    fn TryParse(&self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbEndpointDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbEndpointDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterfaceImpl: Sized {
    fn BulkInPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInPipe>>;
    fn InterruptInPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInPipe>>;
    fn BulkOutPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutPipe>>;
    fn InterruptOutPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutPipe>>;
    fn InterfaceSettings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterfaceSetting>>;
    fn InterfaceNumber(&self) -> ::windows::core::Result<u8>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterfaceDescriptorImpl: Sized {
    fn ClassCode(&self) -> ::windows::core::Result<u8>;
    fn SubclassCode(&self) -> ::windows::core::Result<u8>;
    fn ProtocolCode(&self) -> ::windows::core::Result<u8>;
    fn AlternateSettingNumber(&self) -> ::windows::core::Result<u8>;
    fn InterfaceNumber(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterfaceDescriptorStaticsImpl: Sized {
    fn TryParse(&self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbInterfaceDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbInterfaceDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterfaceSettingImpl: Sized {
    fn BulkInEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInEndpointDescriptor>>;
    fn InterruptInEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInEndpointDescriptor>>;
    fn BulkOutEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutEndpointDescriptor>>;
    fn InterruptOutEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutEndpointDescriptor>>;
    fn Selected(&self) -> ::windows::core::Result<bool>;
    fn SelectSettingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InterfaceDescriptor(&self) -> ::windows::core::Result<UsbInterfaceDescriptor>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptInEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Pipe(&self) -> ::windows::core::Result<UsbInterruptInPipe>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptInEventArgsImpl: Sized {
    fn InterruptData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptInPipeImpl: Sized {
    fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptInEndpointDescriptor>;
    fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UsbInterruptInPipe, UsbInterruptInEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptOutEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Pipe(&self) -> ::windows::core::Result<UsbInterruptOutPipe>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptOutPipeImpl: Sized {
    fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptOutEndpointDescriptor>;
    fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetWriteOptions(&self, value: UsbWriteOptions) -> ::windows::core::Result<()>;
    fn WriteOptions(&self) -> ::windows::core::Result<UsbWriteOptions>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbSetupPacketImpl: Sized {
    fn RequestType(&self) -> ::windows::core::Result<UsbControlRequestType>;
    fn SetRequestType(&self, value: &::core::option::Option<UsbControlRequestType>) -> ::windows::core::Result<()>;
    fn Request(&self) -> ::windows::core::Result<u8>;
    fn SetRequest(&self, value: u8) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValue(&self, value: u32) -> ::windows::core::Result<()>;
    fn Index(&self) -> ::windows::core::Result<u32>;
    fn SetIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbSetupPacketFactoryImpl: Sized {
    fn CreateWithEightByteBuffer(&self, eightbytebuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<UsbSetupPacket>;
}
