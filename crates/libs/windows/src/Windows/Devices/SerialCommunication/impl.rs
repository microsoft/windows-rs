#[cfg(feature = "implement_exclusive")]
pub trait IErrorReceivedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<SerialError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IErrorReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.IErrorReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IErrorReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorReceivedEventArgsImpl, const OFFSET: isize>() -> IErrorReceivedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IErrorReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IErrorReceivedEventArgs>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinChangedEventArgsImpl: Sized {
    fn PinChange(&self) -> ::windows::core::Result<SerialPinChange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPinChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.IPinChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPinChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPinChangedEventArgsImpl, const OFFSET: isize>() -> IPinChangedEventArgsVtbl {
        unsafe extern "system" fn PinChange<Impl: IPinChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialPinChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPinChangedEventArgs>, ::windows::core::GetTrustLevel, PinChange::<Impl, OFFSET>)
    }
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISerialDevice {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.ISerialDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISerialDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISerialDeviceImpl, const OFFSET: isize>() -> ISerialDeviceVtbl {
        unsafe extern "system" fn BaudRate<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaudRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaudRate<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaudRate(value).into()
        }
        unsafe extern "system" fn BreakSignalState<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakSignalState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakSignalState<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakSignalState(value).into()
        }
        unsafe extern "system" fn BytesReceived<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CarrierDetectState<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CarrierDetectState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearToSendState<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearToSendState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataBits<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataBits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataBits<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataBits(value).into()
        }
        unsafe extern "system" fn DataSetReadyState<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSetReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handshake<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialHandshake) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handshake() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandshake<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SerialHandshake) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandshake(value).into()
        }
        unsafe extern "system" fn IsDataTerminalReadyEnabled<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataTerminalReadyEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDataTerminalReadyEnabled<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDataTerminalReadyEnabled(value).into()
        }
        unsafe extern "system" fn IsRequestToSendEnabled<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequestToSendEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRequestToSendEnabled<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRequestToSendEnabled(value).into()
        }
        unsafe extern "system" fn Parity<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialParity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParity<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SerialParity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParity(value).into()
        }
        unsafe extern "system" fn PortName<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PortName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTimeout<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadTimeout<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopBits<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialStopBitCount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopBits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopBits<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SerialStopBitCount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopBits(value).into()
        }
        unsafe extern "system" fn UsbVendorId<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsbVendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsbProductId<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsbProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteTimeout<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteTimeout<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InputStream<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorReceived<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorReceived(&*(&reporthandler as *const <super::super::Foundation::TypedEventHandler<SerialDevice, ErrorReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SerialDevice, ErrorReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorReceived<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveErrorReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PinChanged<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinChanged(&*(&reporthandler as *const <super::super::Foundation::TypedEventHandler<SerialDevice, PinChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SerialDevice, PinChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePinChanged<Impl: ISerialDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePinChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISerialDevice>,
            ::windows::core::GetTrustLevel,
            BaudRate::<Impl, OFFSET>,
            SetBaudRate::<Impl, OFFSET>,
            BreakSignalState::<Impl, OFFSET>,
            SetBreakSignalState::<Impl, OFFSET>,
            BytesReceived::<Impl, OFFSET>,
            CarrierDetectState::<Impl, OFFSET>,
            ClearToSendState::<Impl, OFFSET>,
            DataBits::<Impl, OFFSET>,
            SetDataBits::<Impl, OFFSET>,
            DataSetReadyState::<Impl, OFFSET>,
            Handshake::<Impl, OFFSET>,
            SetHandshake::<Impl, OFFSET>,
            IsDataTerminalReadyEnabled::<Impl, OFFSET>,
            SetIsDataTerminalReadyEnabled::<Impl, OFFSET>,
            IsRequestToSendEnabled::<Impl, OFFSET>,
            SetIsRequestToSendEnabled::<Impl, OFFSET>,
            Parity::<Impl, OFFSET>,
            SetParity::<Impl, OFFSET>,
            PortName::<Impl, OFFSET>,
            ReadTimeout::<Impl, OFFSET>,
            SetReadTimeout::<Impl, OFFSET>,
            StopBits::<Impl, OFFSET>,
            SetStopBits::<Impl, OFFSET>,
            UsbVendorId::<Impl, OFFSET>,
            UsbProductId::<Impl, OFFSET>,
            WriteTimeout::<Impl, OFFSET>,
            SetWriteTimeout::<Impl, OFFSET>,
            InputStream::<Impl, OFFSET>,
            OutputStream::<Impl, OFFSET>,
            ErrorReceived::<Impl, OFFSET>,
            RemoveErrorReceived::<Impl, OFFSET>,
            PinChanged::<Impl, OFFSET>,
            RemovePinChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISerialDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromPortName(&self, portname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromUsbVidPid(&self, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SerialDevice>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISerialDeviceStatics {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.ISerialDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISerialDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISerialDeviceStaticsImpl, const OFFSET: isize>() -> ISerialDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ISerialDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromPortName<Impl: ISerialDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromPortName(&*(&portname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromUsbVidPid<Impl: ISerialDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorid: u16, productid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromUsbVidPid(vendorid, productid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ISerialDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISerialDeviceStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, OFFSET>, GetDeviceSelectorFromPortName::<Impl, OFFSET>, GetDeviceSelectorFromUsbVidPid::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
}
