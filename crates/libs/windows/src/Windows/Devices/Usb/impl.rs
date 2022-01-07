#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkInEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Pipe(&self) -> ::windows::core::Result<UsbBulkInPipe>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbBulkInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkInEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbBulkInEndpointDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkInEndpointDescriptorImpl, const OFFSET: isize>() -> IUsbBulkInEndpointDescriptorVtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbBulkInEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointNumber<Impl: IUsbBulkInEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pipe<Impl: IUsbBulkInEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pipe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbBulkInEndpointDescriptor>, ::windows::core::GetTrustLevel, MaxPacketSize::<Impl, OFFSET>, EndpointNumber::<Impl, OFFSET>, Pipe::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbBulkInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkInPipe";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbBulkInPipeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkInPipeImpl, const OFFSET: isize>() -> IUsbBulkInPipeVtbl {
        unsafe extern "system" fn MaxTransferSizeBytes<Impl: IUsbBulkInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTransferSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbBulkInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbBulkInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearStallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadOptions<Impl: IUsbBulkInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbReadOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadOptions(value).into()
        }
        unsafe extern "system" fn ReadOptions<Impl: IUsbBulkInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbReadOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushBuffer<Impl: IUsbBulkInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushBuffer().into()
        }
        unsafe extern "system" fn InputStream<Impl: IUsbBulkInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUsbBulkInPipe>,
            ::windows::core::GetTrustLevel,
            MaxTransferSizeBytes::<Impl, OFFSET>,
            EndpointDescriptor::<Impl, OFFSET>,
            ClearStallAsync::<Impl, OFFSET>,
            SetReadOptions::<Impl, OFFSET>,
            ReadOptions::<Impl, OFFSET>,
            FlushBuffer::<Impl, OFFSET>,
            InputStream::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkOutEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Pipe(&self) -> ::windows::core::Result<UsbBulkOutPipe>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbBulkOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbBulkOutEndpointDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkOutEndpointDescriptorImpl, const OFFSET: isize>() -> IUsbBulkOutEndpointDescriptorVtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbBulkOutEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointNumber<Impl: IUsbBulkOutEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pipe<Impl: IUsbBulkOutEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pipe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbBulkOutEndpointDescriptor>, ::windows::core::GetTrustLevel, MaxPacketSize::<Impl, OFFSET>, EndpointNumber::<Impl, OFFSET>, Pipe::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbBulkOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkOutPipe";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbBulkOutPipeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkOutPipeImpl, const OFFSET: isize>() -> IUsbBulkOutPipeVtbl {
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbBulkOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbBulkOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearStallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteOptions<Impl: IUsbBulkOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbWriteOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteOptions(value).into()
        }
        unsafe extern "system" fn WriteOptions<Impl: IUsbBulkOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbWriteOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: IUsbBulkOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbBulkOutPipe>, ::windows::core::GetTrustLevel, EndpointDescriptor::<Impl, OFFSET>, ClearStallAsync::<Impl, OFFSET>, SetWriteOptions::<Impl, OFFSET>, WriteOptions::<Impl, OFFSET>, OutputStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationImpl: Sized {
    fn UsbInterfaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterface>>;
    fn ConfigurationDescriptor(&self) -> ::windows::core::Result<UsbConfigurationDescriptor>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbConfiguration {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbConfigurationImpl, const OFFSET: isize>() -> IUsbConfigurationVtbl {
        unsafe extern "system" fn UsbInterfaces<Impl: IUsbConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsbInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigurationDescriptor<Impl: IUsbConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigurationDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descriptors<Impl: IUsbConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbConfiguration>, ::windows::core::GetTrustLevel, UsbInterfaces::<Impl, OFFSET>, ConfigurationDescriptor::<Impl, OFFSET>, Descriptors::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationDescriptorImpl: Sized {
    fn ConfigurationValue(&self) -> ::windows::core::Result<u8>;
    fn MaxPowerMilliamps(&self) -> ::windows::core::Result<u32>;
    fn SelfPowered(&self) -> ::windows::core::Result<bool>;
    fn RemoteWakeup(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbConfigurationDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbConfigurationDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbConfigurationDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbConfigurationDescriptorImpl, const OFFSET: isize>() -> IUsbConfigurationDescriptorVtbl {
        unsafe extern "system" fn ConfigurationValue<Impl: IUsbConfigurationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigurationValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPowerMilliamps<Impl: IUsbConfigurationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPowerMilliamps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelfPowered<Impl: IUsbConfigurationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelfPowered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteWakeup<Impl: IUsbConfigurationDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteWakeup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbConfigurationDescriptor>, ::windows::core::GetTrustLevel, ConfigurationValue::<Impl, OFFSET>, MaxPowerMilliamps::<Impl, OFFSET>, SelfPowered::<Impl, OFFSET>, RemoteWakeup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationDescriptorStaticsImpl: Sized {
    fn TryParse(&self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbConfigurationDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbConfigurationDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbConfigurationDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbConfigurationDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbConfigurationDescriptorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbConfigurationDescriptorStaticsImpl, const OFFSET: isize>() -> IUsbConfigurationDescriptorStaticsVtbl {
        unsafe extern "system" fn TryParse<Impl: IUsbConfigurationDescriptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&descriptor as *const <UsbDescriptor as ::windows::core::Abi>::Abi as *const <UsbDescriptor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parsed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parse<Impl: IUsbConfigurationDescriptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&descriptor as *const <UsbDescriptor as ::windows::core::Abi>::Abi as *const <UsbDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbConfigurationDescriptorStatics>, ::windows::core::GetTrustLevel, TryParse::<Impl, OFFSET>, Parse::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbControlRequestType {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbControlRequestType";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbControlRequestTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>() -> IUsbControlRequestTypeVtbl {
        unsafe extern "system" fn Direction<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbTransferDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbTransferDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(value).into()
        }
        unsafe extern "system" fn ControlTransferType<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbControlTransferType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlTransferType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlTransferType<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbControlTransferType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlTransferType(value).into()
        }
        unsafe extern "system" fn Recipient<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbControlRecipient) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipient() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecipient<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbControlRecipient) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecipient(value).into()
        }
        unsafe extern "system" fn AsByte<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsByte() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsByte<Impl: IUsbControlRequestTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAsByte(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUsbControlRequestType>,
            ::windows::core::GetTrustLevel,
            Direction::<Impl, OFFSET>,
            SetDirection::<Impl, OFFSET>,
            ControlTransferType::<Impl, OFFSET>,
            SetControlTransferType::<Impl, OFFSET>,
            Recipient::<Impl, OFFSET>,
            SetRecipient::<Impl, OFFSET>,
            AsByte::<Impl, OFFSET>,
            SetAsByte::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDescriptorImpl: Sized {
    fn Length(&self) -> ::windows::core::Result<u8>;
    fn DescriptorType(&self) -> ::windows::core::Result<u8>;
    fn ReadDescriptorBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDescriptorImpl, const OFFSET: isize>() -> IUsbDescriptorVtbl {
        unsafe extern "system" fn Length<Impl: IUsbDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DescriptorType<Impl: IUsbDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DescriptorType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDescriptorBuffer<Impl: IUsbDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadDescriptorBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbDescriptor>, ::windows::core::GetTrustLevel, Length::<Impl, OFFSET>, DescriptorType::<Impl, OFFSET>, ReadDescriptorBuffer::<Impl, OFFSET>)
    }
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbDevice {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUsbDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceImpl, const OFFSET: isize>() -> IUsbDeviceVtbl {
        unsafe extern "system" fn SendControlOutTransferAsync<Impl: IUsbDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendControlOutTransferAsync(&*(&setuppacket as *const <UsbSetupPacket as ::windows::core::Abi>::Abi as *const <UsbSetupPacket as ::windows::core::DefaultType>::DefaultType), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlOutTransferAsyncNoBuffer<Impl: IUsbDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendControlOutTransferAsyncNoBuffer(&*(&setuppacket as *const <UsbSetupPacket as ::windows::core::Abi>::Abi as *const <UsbSetupPacket as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlInTransferAsync<Impl: IUsbDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendControlInTransferAsync(&*(&setuppacket as *const <UsbSetupPacket as ::windows::core::Abi>::Abi as *const <UsbSetupPacket as ::windows::core::DefaultType>::DefaultType), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlInTransferAsyncNoBuffer<Impl: IUsbDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendControlInTransferAsyncNoBuffer(&*(&setuppacket as *const <UsbSetupPacket as ::windows::core::Abi>::Abi as *const <UsbSetupPacket as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterface<Impl: IUsbDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceDescriptor<Impl: IUsbDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configuration<Impl: IUsbDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUsbDevice>,
            ::windows::core::GetTrustLevel,
            SendControlOutTransferAsync::<Impl, OFFSET>,
            SendControlOutTransferAsyncNoBuffer::<Impl, OFFSET>,
            SendControlInTransferAsync::<Impl, OFFSET>,
            SendControlInTransferAsyncNoBuffer::<Impl, OFFSET>,
            DefaultInterface::<Impl, OFFSET>,
            DeviceDescriptor::<Impl, OFFSET>,
            Configuration::<Impl, OFFSET>,
        )
    }
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
impl ::windows::core::RuntimeName for IUsbDeviceClass {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceClass";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceClassImpl, const OFFSET: isize>() -> IUsbDeviceClassVtbl {
        unsafe extern "system" fn ClassCode<Impl: IUsbDeviceClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassCode<Impl: IUsbDeviceClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassCode(value).into()
        }
        unsafe extern "system" fn SubclassCode<Impl: IUsbDeviceClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubclassCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubclassCode<Impl: IUsbDeviceClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubclassCode(&*(&value as *const <super::super::Foundation::IReference<u8> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u8> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProtocolCode<Impl: IUsbDeviceClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocolCode<Impl: IUsbDeviceClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtocolCode(&*(&value as *const <super::super::Foundation::IReference<u8> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u8> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbDeviceClass>, ::windows::core::GetTrustLevel, ClassCode::<Impl, OFFSET>, SetClassCode::<Impl, OFFSET>, SubclassCode::<Impl, OFFSET>, SetSubclassCode::<Impl, OFFSET>, ProtocolCode::<Impl, OFFSET>, SetProtocolCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceClassesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbDeviceClasses {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceClasses";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceClassesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceClassesImpl, const OFFSET: isize>() -> IUsbDeviceClassesVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbDeviceClasses>, ::windows::core::GetTrustLevel)
    }
}
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
impl ::windows::core::RuntimeName for IUsbDeviceClassesStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceClassesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceClassesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>() -> IUsbDeviceClassesStaticsVtbl {
        unsafe extern "system" fn CdcControl<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CdcControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Physical<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Physical() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PersonalHealthcare<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PersonalHealthcare() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveSync<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveSync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PalmSync<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PalmSync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceFirmwareUpdate<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceFirmwareUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Irda<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Irda() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Measurement<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Measurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorSpecific<Impl: IUsbDeviceClassesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUsbDeviceClassesStatics>,
            ::windows::core::GetTrustLevel,
            CdcControl::<Impl, OFFSET>,
            Physical::<Impl, OFFSET>,
            PersonalHealthcare::<Impl, OFFSET>,
            ActiveSync::<Impl, OFFSET>,
            PalmSync::<Impl, OFFSET>,
            DeviceFirmwareUpdate::<Impl, OFFSET>,
            Irda::<Impl, OFFSET>,
            Measurement::<Impl, OFFSET>,
            VendorSpecific::<Impl, OFFSET>,
        )
    }
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
impl ::windows::core::RuntimeName for IUsbDeviceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceDescriptorImpl, const OFFSET: isize>() -> IUsbDeviceDescriptorVtbl {
        unsafe extern "system" fn BcdUsb<Impl: IUsbDeviceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BcdUsb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPacketSize0<Impl: IUsbDeviceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPacketSize0() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Impl: IUsbDeviceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Impl: IUsbDeviceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BcdDeviceRevision<Impl: IUsbDeviceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BcdDeviceRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfConfigurations<Impl: IUsbDeviceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfConfigurations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbDeviceDescriptor>, ::windows::core::GetTrustLevel, BcdUsb::<Impl, OFFSET>, MaxPacketSize0::<Impl, OFFSET>, VendorId::<Impl, OFFSET>, ProductId::<Impl, OFFSET>, BcdDeviceRevision::<Impl, OFFSET>, NumberOfConfigurations::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceStaticsImpl, const OFFSET: isize>() -> IUsbDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IUsbDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorid: u32, productid: u32, winusbinterfaceclass: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(vendorid, productid, &*(&winusbinterfaceclass as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorGuidOnly<Impl: IUsbDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, winusbinterfaceclass: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorGuidOnly(&*(&winusbinterfaceclass as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorVidPidOnly<Impl: IUsbDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorid: u32, productid: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorVidPidOnly(vendorid, productid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceClassSelector<Impl: IUsbDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usbclass: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceClassSelector(&*(&usbclass as *const <UsbDeviceClass as ::windows::core::Abi>::Abi as *const <UsbDeviceClass as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IUsbDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbDeviceStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, OFFSET>, GetDeviceSelectorGuidOnly::<Impl, OFFSET>, GetDeviceSelectorVidPidOnly::<Impl, OFFSET>, GetDeviceClassSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbEndpointDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>() -> IUsbEndpointDescriptorVtbl {
        unsafe extern "system" fn EndpointNumber<Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbTransferDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointType<Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbEndpointType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsBulkInEndpointDescriptor<Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsBulkInEndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsInterruptInEndpointDescriptor<Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsInterruptInEndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsBulkOutEndpointDescriptor<Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsBulkOutEndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsInterruptOutEndpointDescriptor<Impl: IUsbEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsInterruptOutEndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUsbEndpointDescriptor>,
            ::windows::core::GetTrustLevel,
            EndpointNumber::<Impl, OFFSET>,
            Direction::<Impl, OFFSET>,
            EndpointType::<Impl, OFFSET>,
            AsBulkInEndpointDescriptor::<Impl, OFFSET>,
            AsInterruptInEndpointDescriptor::<Impl, OFFSET>,
            AsBulkOutEndpointDescriptor::<Impl, OFFSET>,
            AsInterruptOutEndpointDescriptor::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbEndpointDescriptorStaticsImpl: Sized {
    fn TryParse(&self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbEndpointDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbEndpointDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbEndpointDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbEndpointDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbEndpointDescriptorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbEndpointDescriptorStaticsImpl, const OFFSET: isize>() -> IUsbEndpointDescriptorStaticsVtbl {
        unsafe extern "system" fn TryParse<Impl: IUsbEndpointDescriptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&descriptor as *const <UsbDescriptor as ::windows::core::Abi>::Abi as *const <UsbDescriptor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parsed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parse<Impl: IUsbEndpointDescriptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&descriptor as *const <UsbDescriptor as ::windows::core::Abi>::Abi as *const <UsbDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbEndpointDescriptorStatics>, ::windows::core::GetTrustLevel, TryParse::<Impl, OFFSET>, Parse::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbInterface {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterface";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterfaceImpl, const OFFSET: isize>() -> IUsbInterfaceVtbl {
        unsafe extern "system" fn BulkInPipes<Impl: IUsbInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BulkInPipes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterruptInPipes<Impl: IUsbInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterruptInPipes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BulkOutPipes<Impl: IUsbInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BulkOutPipes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterruptOutPipes<Impl: IUsbInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterruptOutPipes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceSettings<Impl: IUsbInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceNumber<Impl: IUsbInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descriptors<Impl: IUsbInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterface>, ::windows::core::GetTrustLevel, BulkInPipes::<Impl, OFFSET>, InterruptInPipes::<Impl, OFFSET>, BulkOutPipes::<Impl, OFFSET>, InterruptOutPipes::<Impl, OFFSET>, InterfaceSettings::<Impl, OFFSET>, InterfaceNumber::<Impl, OFFSET>, Descriptors::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbInterfaceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterfaceDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterfaceDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterfaceDescriptorImpl, const OFFSET: isize>() -> IUsbInterfaceDescriptorVtbl {
        unsafe extern "system" fn ClassCode<Impl: IUsbInterfaceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassCode<Impl: IUsbInterfaceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubclassCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtocolCode<Impl: IUsbInterfaceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateSettingNumber<Impl: IUsbInterfaceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateSettingNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceNumber<Impl: IUsbInterfaceDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterfaceDescriptor>, ::windows::core::GetTrustLevel, ClassCode::<Impl, OFFSET>, SubclassCode::<Impl, OFFSET>, ProtocolCode::<Impl, OFFSET>, AlternateSettingNumber::<Impl, OFFSET>, InterfaceNumber::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterfaceDescriptorStaticsImpl: Sized {
    fn TryParse(&self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbInterfaceDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbInterfaceDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbInterfaceDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterfaceDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterfaceDescriptorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterfaceDescriptorStaticsImpl, const OFFSET: isize>() -> IUsbInterfaceDescriptorStaticsVtbl {
        unsafe extern "system" fn TryParse<Impl: IUsbInterfaceDescriptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&descriptor as *const <UsbDescriptor as ::windows::core::Abi>::Abi as *const <UsbDescriptor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parsed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parse<Impl: IUsbInterfaceDescriptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&descriptor as *const <UsbDescriptor as ::windows::core::Abi>::Abi as *const <UsbDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterfaceDescriptorStatics>, ::windows::core::GetTrustLevel, TryParse::<Impl, OFFSET>, Parse::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbInterfaceSetting {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterfaceSetting";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterfaceSettingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>() -> IUsbInterfaceSettingVtbl {
        unsafe extern "system" fn BulkInEndpoints<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BulkInEndpoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterruptInEndpoints<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterruptInEndpoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BulkOutEndpoints<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BulkOutEndpoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterruptOutEndpoints<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterruptOutEndpoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selected<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectSettingAsync<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectSettingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceDescriptor<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descriptors<Impl: IUsbInterfaceSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUsbInterfaceSetting>,
            ::windows::core::GetTrustLevel,
            BulkInEndpoints::<Impl, OFFSET>,
            InterruptInEndpoints::<Impl, OFFSET>,
            BulkOutEndpoints::<Impl, OFFSET>,
            InterruptOutEndpoints::<Impl, OFFSET>,
            Selected::<Impl, OFFSET>,
            SelectSettingAsync::<Impl, OFFSET>,
            InterfaceDescriptor::<Impl, OFFSET>,
            Descriptors::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptInEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Pipe(&self) -> ::windows::core::Result<UsbInterruptInPipe>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbInterruptInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterruptInEndpointDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptInEndpointDescriptorImpl, const OFFSET: isize>() -> IUsbInterruptInEndpointDescriptorVtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbInterruptInEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointNumber<Impl: IUsbInterruptInEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Interval<Impl: IUsbInterruptInEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pipe<Impl: IUsbInterruptInEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pipe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterruptInEndpointDescriptor>, ::windows::core::GetTrustLevel, MaxPacketSize::<Impl, OFFSET>, EndpointNumber::<Impl, OFFSET>, Interval::<Impl, OFFSET>, Pipe::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptInEventArgsImpl: Sized {
    fn InterruptData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbInterruptInEventArgs {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptInEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterruptInEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptInEventArgsImpl, const OFFSET: isize>() -> IUsbInterruptInEventArgsVtbl {
        unsafe extern "system" fn InterruptData<Impl: IUsbInterruptInEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterruptData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterruptInEventArgs>, ::windows::core::GetTrustLevel, InterruptData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptInPipeImpl: Sized {
    fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptInEndpointDescriptor>;
    fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UsbInterruptInPipe, UsbInterruptInEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbInterruptInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptInPipe";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterruptInPipeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptInPipeImpl, const OFFSET: isize>() -> IUsbInterruptInPipeVtbl {
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbInterruptInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbInterruptInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearStallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataReceived<Impl: IUsbInterruptInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UsbInterruptInPipe, UsbInterruptInEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UsbInterruptInPipe, UsbInterruptInEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataReceived<Impl: IUsbInterruptInPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterruptInPipe>, ::windows::core::GetTrustLevel, EndpointDescriptor::<Impl, OFFSET>, ClearStallAsync::<Impl, OFFSET>, DataReceived::<Impl, OFFSET>, RemoveDataReceived::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterruptOutEndpointDescriptorImpl: Sized {
    fn MaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&self) -> ::windows::core::Result<u8>;
    fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Pipe(&self) -> ::windows::core::Result<UsbInterruptOutPipe>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbInterruptOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterruptOutEndpointDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptOutEndpointDescriptorImpl, const OFFSET: isize>() -> IUsbInterruptOutEndpointDescriptorVtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbInterruptOutEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointNumber<Impl: IUsbInterruptOutEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Interval<Impl: IUsbInterruptOutEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pipe<Impl: IUsbInterruptOutEndpointDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pipe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterruptOutEndpointDescriptor>, ::windows::core::GetTrustLevel, MaxPacketSize::<Impl, OFFSET>, EndpointNumber::<Impl, OFFSET>, Interval::<Impl, OFFSET>, Pipe::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbInterruptOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptOutPipe";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterruptOutPipeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptOutPipeImpl, const OFFSET: isize>() -> IUsbInterruptOutPipeVtbl {
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbInterruptOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbInterruptOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearStallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteOptions<Impl: IUsbInterruptOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbWriteOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteOptions(value).into()
        }
        unsafe extern "system" fn WriteOptions<Impl: IUsbInterruptOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbWriteOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: IUsbInterruptOutPipeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbInterruptOutPipe>, ::windows::core::GetTrustLevel, EndpointDescriptor::<Impl, OFFSET>, ClearStallAsync::<Impl, OFFSET>, SetWriteOptions::<Impl, OFFSET>, WriteOptions::<Impl, OFFSET>, OutputStream::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IUsbSetupPacket {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbSetupPacket";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbSetupPacketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbSetupPacketImpl, const OFFSET: isize>() -> IUsbSetupPacketVtbl {
        unsafe extern "system" fn RequestType<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestType<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestType(&*(&value as *const <UsbControlRequestType as ::windows::core::Abi>::Abi as *const <UsbControlRequestType as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Request<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequest<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequest(value).into()
        }
        unsafe extern "system" fn Value<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn Index<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndex(value).into()
        }
        unsafe extern "system" fn Length<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: IUsbSetupPacketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUsbSetupPacket>,
            ::windows::core::GetTrustLevel,
            RequestType::<Impl, OFFSET>,
            SetRequestType::<Impl, OFFSET>,
            Request::<Impl, OFFSET>,
            SetRequest::<Impl, OFFSET>,
            Value::<Impl, OFFSET>,
            SetValue::<Impl, OFFSET>,
            Index::<Impl, OFFSET>,
            SetIndex::<Impl, OFFSET>,
            Length::<Impl, OFFSET>,
            SetLength::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbSetupPacketFactoryImpl: Sized {
    fn CreateWithEightByteBuffer(&self, eightbytebuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<UsbSetupPacket>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbSetupPacketFactory {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbSetupPacketFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbSetupPacketFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbSetupPacketFactoryImpl, const OFFSET: isize>() -> IUsbSetupPacketFactoryVtbl {
        unsafe extern "system" fn CreateWithEightByteBuffer<Impl: IUsbSetupPacketFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eightbytebuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithEightByteBuffer(&*(&eightbytebuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUsbSetupPacketFactory>, ::windows::core::GetTrustLevel, CreateWithEightByteBuffer::<Impl, OFFSET>)
    }
}
