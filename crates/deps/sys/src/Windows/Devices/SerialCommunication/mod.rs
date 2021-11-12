#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ErrorReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IErrorReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISerialDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISerialDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PinChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SerialDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SerialError(pub i32);
impl SerialError {
    pub const Frame: SerialError = SerialError(0i32);
    pub const BufferOverrun: SerialError = SerialError(1i32);
    pub const ReceiveFull: SerialError = SerialError(2i32);
    pub const ReceiveParity: SerialError = SerialError(3i32);
    pub const TransmitFull: SerialError = SerialError(4i32);
}
#[repr(transparent)]
pub struct SerialHandshake(pub i32);
impl SerialHandshake {
    pub const None: SerialHandshake = SerialHandshake(0i32);
    pub const RequestToSend: SerialHandshake = SerialHandshake(1i32);
    pub const XOnXOff: SerialHandshake = SerialHandshake(2i32);
    pub const RequestToSendXOnXOff: SerialHandshake = SerialHandshake(3i32);
}
#[repr(transparent)]
pub struct SerialParity(pub i32);
impl SerialParity {
    pub const None: SerialParity = SerialParity(0i32);
    pub const Odd: SerialParity = SerialParity(1i32);
    pub const Even: SerialParity = SerialParity(2i32);
    pub const Mark: SerialParity = SerialParity(3i32);
    pub const Space: SerialParity = SerialParity(4i32);
}
#[repr(transparent)]
pub struct SerialPinChange(pub i32);
impl SerialPinChange {
    pub const BreakSignal: SerialPinChange = SerialPinChange(0i32);
    pub const CarrierDetect: SerialPinChange = SerialPinChange(1i32);
    pub const ClearToSend: SerialPinChange = SerialPinChange(2i32);
    pub const DataSetReady: SerialPinChange = SerialPinChange(3i32);
    pub const RingIndicator: SerialPinChange = SerialPinChange(4i32);
}
#[repr(transparent)]
pub struct SerialStopBitCount(pub i32);
impl SerialStopBitCount {
    pub const One: SerialStopBitCount = SerialStopBitCount(0i32);
    pub const OnePointFive: SerialStopBitCount = SerialStopBitCount(1i32);
    pub const Two: SerialStopBitCount = SerialStopBitCount(2i32);
}
