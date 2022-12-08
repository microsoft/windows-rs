#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IErrorReceivedEventArgs {
    type Vtable = IErrorReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IErrorReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcc6bf59_1283_4d8a_bfdf_566b33ddb28f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPinChangedEventArgs {
    type Vtable = IPinChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPinChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2bf1db0_fc9c_4607_93d0_fa5e8343ee22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PinChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialPinChange) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISerialDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISerialDevice {
    type Vtable = ISerialDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for ISerialDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe187ccc6_2210_414f_b65a_f5553a03372a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISerialDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BaudRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetBaudRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub BreakSignalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBreakSignalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CarrierDetectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ClearToSendState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DataBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetDataBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    pub DataSetReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Handshake: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialHandshake) -> ::windows::core::HRESULT,
    pub SetHandshake: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SerialHandshake) -> ::windows::core::HRESULT,
    pub IsDataTerminalReadyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDataTerminalReadyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRequestToSendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRequestToSendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Parity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialParity) -> ::windows::core::HRESULT,
    pub SetParity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SerialParity) -> ::windows::core::HRESULT,
    pub PortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetReadTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReadTimeout: usize,
    pub StopBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialStopBitCount) -> ::windows::core::HRESULT,
    pub SetStopBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SerialStopBitCount) -> ::windows::core::HRESULT,
    pub UsbVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsbProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WriteTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetWriteTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWriteTimeout: usize,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PinChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PinChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePinChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePinChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISerialDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISerialDeviceStatics {
    type Vtable = ISerialDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISerialDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x058c4a70_0836_4993_ae1a_b61ae3be056b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISerialDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromPortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromUsbVidPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorid: u16, productid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct ErrorReceivedEventArgs(::windows::core::IUnknown);
impl ErrorReceivedEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<SerialError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ErrorReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ErrorReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ErrorReceivedEventArgs {}
impl ::core::fmt::Debug for ErrorReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ErrorReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.SerialCommunication.ErrorReceivedEventArgs;{fcc6bf59-1283-4d8a-bfdf-566b33ddb28f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ErrorReceivedEventArgs {
    type Vtable = IErrorReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ErrorReceivedEventArgs {
    const IID: ::windows::core::GUID = <IErrorReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ErrorReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.ErrorReceivedEventArgs";
}
::windows::core::interface_hierarchy!(ErrorReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ErrorReceivedEventArgs {}
unsafe impl ::core::marker::Sync for ErrorReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct PinChangedEventArgs(::windows::core::IUnknown);
impl PinChangedEventArgs {
    pub fn PinChange(&self) -> ::windows::core::Result<SerialPinChange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinChange)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PinChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PinChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PinChangedEventArgs {}
impl ::core::fmt::Debug for PinChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PinChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PinChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.SerialCommunication.PinChangedEventArgs;{a2bf1db0-fc9c-4607-93d0-fa5e8343ee22})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PinChangedEventArgs {
    type Vtable = IPinChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PinChangedEventArgs {
    const IID: ::windows::core::GUID = <IPinChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PinChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.PinChangedEventArgs";
}
::windows::core::interface_hierarchy!(PinChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PinChangedEventArgs {}
unsafe impl ::core::marker::Sync for PinChangedEventArgs {}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct SerialDevice(::windows::core::IUnknown);
impl SerialDevice {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn BaudRate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaudRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBaudRate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBaudRate)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BreakSignalState(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BreakSignalState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBreakSignalState(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBreakSignalState)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BytesReceived(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BytesReceived)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CarrierDetectState(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CarrierDetectState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ClearToSendState(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClearToSendState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DataBits(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataBits)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDataBits(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDataBits)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DataSetReadyState(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataSetReadyState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Handshake(&self) -> ::windows::core::Result<SerialHandshake> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handshake)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandshake(&self, value: SerialHandshake) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandshake)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsDataTerminalReadyEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDataTerminalReadyEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsDataTerminalReadyEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsDataTerminalReadyEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsRequestToSendEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRequestToSendEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsRequestToSendEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsRequestToSendEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Parity(&self) -> ::windows::core::Result<SerialParity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetParity(&self, value: SerialParity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetParity)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PortName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PortName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadTimeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReadTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetReadTimeout)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn StopBits(&self) -> ::windows::core::Result<SerialStopBitCount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StopBits)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStopBits(&self, value: SerialStopBitCount) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStopBits)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn UsbVendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsbVendorId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsbProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsbProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteTimeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetWriteTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetWriteTimeout)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OutputStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorReceived(&self, reporthandler: &super::super::Foundation::TypedEventHandler<SerialDevice, ErrorReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(reporthandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveErrorReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PinChanged(&self, reporthandler: &super::super::Foundation::TypedEventHandler<SerialDevice, PinChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(reporthandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePinChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePinChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorFromPortName(portname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorFromPortName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(portname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorFromUsbVidPid(vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorFromUsbVidPid)(::windows::core::Vtable::as_raw(this), vendorid, productid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SerialDevice>> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISerialDeviceStatics<R, F: FnOnce(&ISerialDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SerialDevice, ISerialDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SerialDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SerialDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SerialDevice {}
impl ::core::fmt::Debug for SerialDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SerialDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.SerialCommunication.SerialDevice;{e187ccc6-2210-414f-b65a-f5553a03372a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SerialDevice {
    type Vtable = ISerialDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for SerialDevice {
    const IID: ::windows::core::GUID = <ISerialDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SerialDevice {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.SerialDevice";
}
::windows::core::interface_hierarchy!(SerialDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SerialDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SerialDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SerialDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SerialDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SerialDevice> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SerialDevice) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SerialDevice {}
unsafe impl ::core::marker::Sync for SerialDevice {}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SerialError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SerialError {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SerialError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SerialHandshake {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SerialHandshake {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialHandshake {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialHandshake").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SerialHandshake {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialHandshake;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SerialParity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SerialParity {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialParity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialParity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SerialParity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialParity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SerialPinChange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SerialPinChange {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialPinChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialPinChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SerialPinChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialPinChange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SerialStopBitCount {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SerialStopBitCount {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialStopBitCount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialStopBitCount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SerialStopBitCount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialStopBitCount;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
