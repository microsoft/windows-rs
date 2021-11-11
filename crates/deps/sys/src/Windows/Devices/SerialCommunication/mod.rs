#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ErrorReceivedEventArgs();
    fn IErrorReceivedEventArgs();
    fn IPinChangedEventArgs();
    fn ISerialDevice();
    fn ISerialDeviceStatics();
    fn PinChangedEventArgs();
    fn SerialDevice();
    fn SerialError();
    fn SerialHandshake();
    fn SerialParity();
    fn SerialPinChange();
    fn SerialStopBitCount();
}
