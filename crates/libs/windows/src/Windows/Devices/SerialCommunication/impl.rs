#[cfg(feature = "implement_exclusive")]
pub trait IErrorReceivedEventArgs_Impl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<SerialError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IErrorReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.IErrorReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IErrorReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Error<Impl: IErrorReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialError) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IErrorReceivedEventArgs, BASE_OFFSET>(), Error: Error::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPinChangedEventArgs_Impl: Sized {
    fn PinChange(&mut self) -> ::windows::core::Result<SerialPinChange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPinChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.IPinChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPinChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPinChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPinChangedEventArgs_Vtbl {
        unsafe extern "system" fn PinChange<Impl: IPinChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialPinChange) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPinChangedEventArgs, BASE_OFFSET>(), PinChange: PinChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPinChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISerialDevice_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn BaudRate(&mut self) -> ::windows::core::Result<u32>;
    fn SetBaudRate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn BreakSignalState(&mut self) -> ::windows::core::Result<bool>;
    fn SetBreakSignalState(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BytesReceived(&mut self) -> ::windows::core::Result<u32>;
    fn CarrierDetectState(&mut self) -> ::windows::core::Result<bool>;
    fn ClearToSendState(&mut self) -> ::windows::core::Result<bool>;
    fn DataBits(&mut self) -> ::windows::core::Result<u16>;
    fn SetDataBits(&mut self, value: u16) -> ::windows::core::Result<()>;
    fn DataSetReadyState(&mut self) -> ::windows::core::Result<bool>;
    fn Handshake(&mut self) -> ::windows::core::Result<SerialHandshake>;
    fn SetHandshake(&mut self, value: SerialHandshake) -> ::windows::core::Result<()>;
    fn IsDataTerminalReadyEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDataTerminalReadyEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestToSendEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRequestToSendEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Parity(&mut self) -> ::windows::core::Result<SerialParity>;
    fn SetParity(&mut self, value: SerialParity) -> ::windows::core::Result<()>;
    fn PortName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadTimeout(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetReadTimeout(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StopBits(&mut self) -> ::windows::core::Result<SerialStopBitCount>;
    fn SetStopBits(&mut self, value: SerialStopBitCount) -> ::windows::core::Result<()>;
    fn UsbVendorId(&mut self) -> ::windows::core::Result<u16>;
    fn UsbProductId(&mut self) -> ::windows::core::Result<u16>;
    fn WriteTimeout(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetWriteTimeout(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn InputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ErrorReceived(&mut self, reporthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SerialDevice, ErrorReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PinChanged(&mut self, reporthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SerialDevice, PinChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePinChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISerialDevice {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.ISerialDevice";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISerialDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISerialDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISerialDevice_Vtbl {
        unsafe extern "system" fn BaudRate<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBaudRate<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaudRate(value).into()
        }
        unsafe extern "system" fn BreakSignalState<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBreakSignalState<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakSignalState(value).into()
        }
        unsafe extern "system" fn BytesReceived<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CarrierDetectState<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearToSendState<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DataBits<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDataBits<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataBits(value).into()
        }
        unsafe extern "system" fn DataSetReadyState<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handshake<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialHandshake) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandshake<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SerialHandshake) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandshake(value).into()
        }
        unsafe extern "system" fn IsDataTerminalReadyEnabled<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsDataTerminalReadyEnabled<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDataTerminalReadyEnabled(value).into()
        }
        unsafe extern "system" fn IsRequestToSendEnabled<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRequestToSendEnabled<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRequestToSendEnabled(value).into()
        }
        unsafe extern "system" fn Parity<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialParity) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetParity<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SerialParity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParity(value).into()
        }
        unsafe extern "system" fn PortName<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadTimeout<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReadTimeout<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopBits<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SerialStopBitCount) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStopBits<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SerialStopBitCount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopBits(value).into()
        }
        unsafe extern "system" fn UsbVendorId<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsbProductId<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WriteTimeout<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWriteTimeout<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InputStream<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputStream<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ErrorReceived<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveErrorReceived<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveErrorReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PinChanged<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePinChanged<Impl: ISerialDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePinChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISerialDevice, BASE_OFFSET>(),
            BaudRate: BaudRate::<Impl, IMPL_OFFSET>,
            SetBaudRate: SetBaudRate::<Impl, IMPL_OFFSET>,
            BreakSignalState: BreakSignalState::<Impl, IMPL_OFFSET>,
            SetBreakSignalState: SetBreakSignalState::<Impl, IMPL_OFFSET>,
            BytesReceived: BytesReceived::<Impl, IMPL_OFFSET>,
            CarrierDetectState: CarrierDetectState::<Impl, IMPL_OFFSET>,
            ClearToSendState: ClearToSendState::<Impl, IMPL_OFFSET>,
            DataBits: DataBits::<Impl, IMPL_OFFSET>,
            SetDataBits: SetDataBits::<Impl, IMPL_OFFSET>,
            DataSetReadyState: DataSetReadyState::<Impl, IMPL_OFFSET>,
            Handshake: Handshake::<Impl, IMPL_OFFSET>,
            SetHandshake: SetHandshake::<Impl, IMPL_OFFSET>,
            IsDataTerminalReadyEnabled: IsDataTerminalReadyEnabled::<Impl, IMPL_OFFSET>,
            SetIsDataTerminalReadyEnabled: SetIsDataTerminalReadyEnabled::<Impl, IMPL_OFFSET>,
            IsRequestToSendEnabled: IsRequestToSendEnabled::<Impl, IMPL_OFFSET>,
            SetIsRequestToSendEnabled: SetIsRequestToSendEnabled::<Impl, IMPL_OFFSET>,
            Parity: Parity::<Impl, IMPL_OFFSET>,
            SetParity: SetParity::<Impl, IMPL_OFFSET>,
            PortName: PortName::<Impl, IMPL_OFFSET>,
            ReadTimeout: ReadTimeout::<Impl, IMPL_OFFSET>,
            SetReadTimeout: SetReadTimeout::<Impl, IMPL_OFFSET>,
            StopBits: StopBits::<Impl, IMPL_OFFSET>,
            SetStopBits: SetStopBits::<Impl, IMPL_OFFSET>,
            UsbVendorId: UsbVendorId::<Impl, IMPL_OFFSET>,
            UsbProductId: UsbProductId::<Impl, IMPL_OFFSET>,
            WriteTimeout: WriteTimeout::<Impl, IMPL_OFFSET>,
            SetWriteTimeout: SetWriteTimeout::<Impl, IMPL_OFFSET>,
            InputStream: InputStream::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
            ErrorReceived: ErrorReceived::<Impl, IMPL_OFFSET>,
            RemoveErrorReceived: RemoveErrorReceived::<Impl, IMPL_OFFSET>,
            PinChanged: PinChanged::<Impl, IMPL_OFFSET>,
            RemovePinChanged: RemovePinChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISerialDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISerialDeviceStatics_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromPortName(&mut self, portname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromUsbVidPid(&mut self, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SerialDevice>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISerialDeviceStatics {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.ISerialDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISerialDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISerialDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISerialDeviceStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ISerialDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorFromPortName<Impl: ISerialDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorFromUsbVidPid<Impl: ISerialDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorid: u16, productid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: ISerialDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISerialDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromPortName: GetDeviceSelectorFromPortName::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromUsbVidPid: GetDeviceSelectorFromUsbVidPid::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISerialDeviceStatics as ::windows::core::Interface>::IID
    }
}
