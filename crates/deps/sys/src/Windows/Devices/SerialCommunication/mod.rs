#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ErrorReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ErrorReceivedEventArgs {}
impl ::core::clone::Clone for ErrorReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IErrorReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IErrorReceivedEventArgs {}
impl ::core::clone::Clone for IErrorReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPinChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPinChangedEventArgs {}
impl ::core::clone::Clone for IPinChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISerialDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISerialDevice {}
impl ::core::clone::Clone for ISerialDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISerialDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISerialDeviceStatics {}
impl ::core::clone::Clone for ISerialDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PinChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PinChangedEventArgs {}
impl ::core::clone::Clone for PinChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SerialDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SerialDevice {}
impl ::core::clone::Clone for SerialDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SerialError(pub i32);
impl SerialError {
    pub const Frame: Self = Self(0i32);
    pub const BufferOverrun: Self = Self(1i32);
    pub const ReceiveFull: Self = Self(2i32);
    pub const ReceiveParity: Self = Self(3i32);
    pub const TransmitFull: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialError {}
impl ::core::clone::Clone for SerialError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SerialHandshake(pub i32);
impl SerialHandshake {
    pub const None: Self = Self(0i32);
    pub const RequestToSend: Self = Self(1i32);
    pub const XOnXOff: Self = Self(2i32);
    pub const RequestToSendXOnXOff: Self = Self(3i32);
}
impl ::core::marker::Copy for SerialHandshake {}
impl ::core::clone::Clone for SerialHandshake {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SerialParity(pub i32);
impl SerialParity {
    pub const None: Self = Self(0i32);
    pub const Odd: Self = Self(1i32);
    pub const Even: Self = Self(2i32);
    pub const Mark: Self = Self(3i32);
    pub const Space: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialParity {}
impl ::core::clone::Clone for SerialParity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SerialPinChange(pub i32);
impl SerialPinChange {
    pub const BreakSignal: Self = Self(0i32);
    pub const CarrierDetect: Self = Self(1i32);
    pub const ClearToSend: Self = Self(2i32);
    pub const DataSetReady: Self = Self(3i32);
    pub const RingIndicator: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialPinChange {}
impl ::core::clone::Clone for SerialPinChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SerialStopBitCount(pub i32);
impl SerialStopBitCount {
    pub const One: Self = Self(0i32);
    pub const OnePointFive: Self = Self(1i32);
    pub const Two: Self = Self(2i32);
}
impl ::core::marker::Copy for SerialStopBitCount {}
impl ::core::clone::Clone for SerialStopBitCount {
    fn clone(&self) -> Self {
        *self
    }
}
