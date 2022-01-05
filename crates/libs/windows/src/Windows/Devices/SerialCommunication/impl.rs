#[cfg(feature = "implement_exclusive")]
pub trait IErrorReceivedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<SerialError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinChangedEventArgsImpl: Sized {
    fn PinChange(&self) -> ::windows::core::Result<SerialPinChange>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISerialDeviceImpl: Sized + IClosableImpl {
    fn BaudRate(&self) -> ::windows::core::Result<u32>;
    fn SetBaudRate(&self, value: u32) -> ::windows::core::Result<()>;
    fn BreakSignalState(&self) -> ::windows::core::Result<bool>;
    fn SetBreakSignalState(&self, value: bool) -> ::windows::core::Result<()>;
    fn BytesReceived(&self) -> ::windows::core::Result<u32>;
    fn CarrierDetectState(&self) -> ::windows::core::Result<bool>;
    fn ClearToSendState(&self) -> ::windows::core::Result<bool>;
    fn DataBits(&self) -> ::windows::core::Result<u16>;
    fn SetDataBits(&self, value: u16) -> ::windows::core::Result<()>;
    fn DataSetReadyState(&self) -> ::windows::core::Result<bool>;
    fn Handshake(&self) -> ::windows::core::Result<SerialHandshake>;
    fn SetHandshake(&self, value: SerialHandshake) -> ::windows::core::Result<()>;
    fn IsDataTerminalReadyEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDataTerminalReadyEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestToSendEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRequestToSendEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Parity(&self) -> ::windows::core::Result<SerialParity>;
    fn SetParity(&self, value: SerialParity) -> ::windows::core::Result<()>;
    fn PortName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetReadTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StopBits(&self) -> ::windows::core::Result<SerialStopBitCount>;
    fn SetStopBits(&self, value: SerialStopBitCount) -> ::windows::core::Result<()>;
    fn UsbVendorId(&self) -> ::windows::core::Result<u16>;
    fn UsbProductId(&self) -> ::windows::core::Result<u16>;
    fn WriteTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetWriteTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ErrorReceived(&self, reporthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SerialDevice, ErrorReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PinChanged(&self, reporthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SerialDevice, PinChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePinChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISerialDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromPortName(&self, portname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromUsbVidPid(&self, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SerialDevice>>;
}
