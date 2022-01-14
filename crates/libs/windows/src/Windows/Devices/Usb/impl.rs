#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkInEndpointDescriptor_Impl: Sized {
    fn MaxPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&mut self) -> ::windows::core::Result<u8>;
    fn Pipe(&mut self) -> ::windows::core::Result<UsbBulkInPipe>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbBulkInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkInEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbBulkInEndpointDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkInEndpointDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbBulkInEndpointDescriptor_Vtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbBulkInEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointNumber<Impl: IUsbBulkInEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pipe<Impl: IUsbBulkInEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbBulkInEndpointDescriptor, BASE_OFFSET>(),
            MaxPacketSize: MaxPacketSize::<Impl, IMPL_OFFSET>,
            EndpointNumber: EndpointNumber::<Impl, IMPL_OFFSET>,
            Pipe: Pipe::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbBulkInEndpointDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUsbBulkInPipe_Impl: Sized {
    fn MaxTransferSizeBytes(&mut self) -> ::windows::core::Result<u32>;
    fn EndpointDescriptor(&mut self) -> ::windows::core::Result<UsbBulkInEndpointDescriptor>;
    fn ClearStallAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetReadOptions(&mut self, value: UsbReadOptions) -> ::windows::core::Result<()>;
    fn ReadOptions(&mut self) -> ::windows::core::Result<UsbReadOptions>;
    fn FlushBuffer(&mut self) -> ::windows::core::Result<()>;
    fn InputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbBulkInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkInPipe";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUsbBulkInPipe_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkInPipe_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbBulkInPipe_Vtbl {
        unsafe extern "system" fn MaxTransferSizeBytes<Impl: IUsbBulkInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbBulkInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbBulkInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReadOptions<Impl: IUsbBulkInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbReadOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadOptions(value).into()
        }
        unsafe extern "system" fn ReadOptions<Impl: IUsbBulkInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbReadOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FlushBuffer<Impl: IUsbBulkInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushBuffer().into()
        }
        unsafe extern "system" fn InputStream<Impl: IUsbBulkInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbBulkInPipe, BASE_OFFSET>(),
            MaxTransferSizeBytes: MaxTransferSizeBytes::<Impl, IMPL_OFFSET>,
            EndpointDescriptor: EndpointDescriptor::<Impl, IMPL_OFFSET>,
            ClearStallAsync: ClearStallAsync::<Impl, IMPL_OFFSET>,
            SetReadOptions: SetReadOptions::<Impl, IMPL_OFFSET>,
            ReadOptions: ReadOptions::<Impl, IMPL_OFFSET>,
            FlushBuffer: FlushBuffer::<Impl, IMPL_OFFSET>,
            InputStream: InputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbBulkInPipe as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbBulkOutEndpointDescriptor_Impl: Sized {
    fn MaxPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&mut self) -> ::windows::core::Result<u8>;
    fn Pipe(&mut self) -> ::windows::core::Result<UsbBulkOutPipe>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbBulkOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkOutEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbBulkOutEndpointDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkOutEndpointDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbBulkOutEndpointDescriptor_Vtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbBulkOutEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointNumber<Impl: IUsbBulkOutEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pipe<Impl: IUsbBulkOutEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbBulkOutEndpointDescriptor, BASE_OFFSET>(),
            MaxPacketSize: MaxPacketSize::<Impl, IMPL_OFFSET>,
            EndpointNumber: EndpointNumber::<Impl, IMPL_OFFSET>,
            Pipe: Pipe::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbBulkOutEndpointDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUsbBulkOutPipe_Impl: Sized {
    fn EndpointDescriptor(&mut self) -> ::windows::core::Result<UsbBulkOutEndpointDescriptor>;
    fn ClearStallAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetWriteOptions(&mut self, value: UsbWriteOptions) -> ::windows::core::Result<()>;
    fn WriteOptions(&mut self) -> ::windows::core::Result<UsbWriteOptions>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbBulkOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbBulkOutPipe";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUsbBulkOutPipe_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbBulkOutPipe_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbBulkOutPipe_Vtbl {
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbBulkOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbBulkOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWriteOptions<Impl: IUsbBulkOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbWriteOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteOptions(value).into()
        }
        unsafe extern "system" fn WriteOptions<Impl: IUsbBulkOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbWriteOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputStream<Impl: IUsbBulkOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbBulkOutPipe, BASE_OFFSET>(),
            EndpointDescriptor: EndpointDescriptor::<Impl, IMPL_OFFSET>,
            ClearStallAsync: ClearStallAsync::<Impl, IMPL_OFFSET>,
            SetWriteOptions: SetWriteOptions::<Impl, IMPL_OFFSET>,
            WriteOptions: WriteOptions::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbBulkOutPipe as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUsbConfiguration_Impl: Sized {
    fn UsbInterfaces(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterface>>;
    fn ConfigurationDescriptor(&mut self) -> ::windows::core::Result<UsbConfigurationDescriptor>;
    fn Descriptors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbConfiguration {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbConfiguration";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUsbConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbConfiguration_Vtbl {
        unsafe extern "system" fn UsbInterfaces<Impl: IUsbConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConfigurationDescriptor<Impl: IUsbConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Descriptors<Impl: IUsbConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbConfiguration, BASE_OFFSET>(),
            UsbInterfaces: UsbInterfaces::<Impl, IMPL_OFFSET>,
            ConfigurationDescriptor: ConfigurationDescriptor::<Impl, IMPL_OFFSET>,
            Descriptors: Descriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationDescriptor_Impl: Sized {
    fn ConfigurationValue(&mut self) -> ::windows::core::Result<u8>;
    fn MaxPowerMilliamps(&mut self) -> ::windows::core::Result<u32>;
    fn SelfPowered(&mut self) -> ::windows::core::Result<bool>;
    fn RemoteWakeup(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbConfigurationDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbConfigurationDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbConfigurationDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbConfigurationDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbConfigurationDescriptor_Vtbl {
        unsafe extern "system" fn ConfigurationValue<Impl: IUsbConfigurationDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxPowerMilliamps<Impl: IUsbConfigurationDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelfPowered<Impl: IUsbConfigurationDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteWakeup<Impl: IUsbConfigurationDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbConfigurationDescriptor, BASE_OFFSET>(),
            ConfigurationValue: ConfigurationValue::<Impl, IMPL_OFFSET>,
            MaxPowerMilliamps: MaxPowerMilliamps::<Impl, IMPL_OFFSET>,
            SelfPowered: SelfPowered::<Impl, IMPL_OFFSET>,
            RemoteWakeup: RemoteWakeup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbConfigurationDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbConfigurationDescriptorStatics_Impl: Sized {
    fn TryParse(&mut self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbConfigurationDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&mut self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbConfigurationDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbConfigurationDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbConfigurationDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbConfigurationDescriptorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbConfigurationDescriptorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbConfigurationDescriptorStatics_Vtbl {
        unsafe extern "system" fn TryParse<Impl: IUsbConfigurationDescriptorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Parse<Impl: IUsbConfigurationDescriptorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbConfigurationDescriptorStatics, BASE_OFFSET>(),
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
            Parse: Parse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbConfigurationDescriptorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbControlRequestType_Impl: Sized {
    fn Direction(&mut self) -> ::windows::core::Result<UsbTransferDirection>;
    fn SetDirection(&mut self, value: UsbTransferDirection) -> ::windows::core::Result<()>;
    fn ControlTransferType(&mut self) -> ::windows::core::Result<UsbControlTransferType>;
    fn SetControlTransferType(&mut self, value: UsbControlTransferType) -> ::windows::core::Result<()>;
    fn Recipient(&mut self) -> ::windows::core::Result<UsbControlRecipient>;
    fn SetRecipient(&mut self, value: UsbControlRecipient) -> ::windows::core::Result<()>;
    fn AsByte(&mut self) -> ::windows::core::Result<u8>;
    fn SetAsByte(&mut self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbControlRequestType {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbControlRequestType";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbControlRequestType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbControlRequestType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbControlRequestType_Vtbl {
        unsafe extern "system" fn Direction<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbTransferDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbTransferDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(value).into()
        }
        unsafe extern "system" fn ControlTransferType<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbControlTransferType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetControlTransferType<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbControlTransferType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlTransferType(value).into()
        }
        unsafe extern "system" fn Recipient<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbControlRecipient) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRecipient<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbControlRecipient) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecipient(value).into()
        }
        unsafe extern "system" fn AsByte<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAsByte<Impl: IUsbControlRequestType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAsByte(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbControlRequestType, BASE_OFFSET>(),
            Direction: Direction::<Impl, IMPL_OFFSET>,
            SetDirection: SetDirection::<Impl, IMPL_OFFSET>,
            ControlTransferType: ControlTransferType::<Impl, IMPL_OFFSET>,
            SetControlTransferType: SetControlTransferType::<Impl, IMPL_OFFSET>,
            Recipient: Recipient::<Impl, IMPL_OFFSET>,
            SetRecipient: SetRecipient::<Impl, IMPL_OFFSET>,
            AsByte: AsByte::<Impl, IMPL_OFFSET>,
            SetAsByte: SetAsByte::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbControlRequestType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUsbDescriptor_Impl: Sized {
    fn Length(&mut self) -> ::windows::core::Result<u8>;
    fn DescriptorType(&mut self) -> ::windows::core::Result<u8>;
    fn ReadDescriptorBuffer(&mut self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDescriptor";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUsbDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbDescriptor_Vtbl {
        unsafe extern "system" fn Length<Impl: IUsbDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DescriptorType<Impl: IUsbDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadDescriptorBuffer<Impl: IUsbDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadDescriptorBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbDescriptor, BASE_OFFSET>(),
            Length: Length::<Impl, IMPL_OFFSET>,
            DescriptorType: DescriptorType::<Impl, IMPL_OFFSET>,
            ReadDescriptorBuffer: ReadDescriptorBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUsbDevice_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn SendControlOutTransferAsync(&mut self, setuppacket: &::core::option::Option<UsbSetupPacket>, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn SendControlOutTransferAsyncNoBuffer(&mut self, setuppacket: &::core::option::Option<UsbSetupPacket>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn SendControlInTransferAsync(&mut self, setuppacket: &::core::option::Option<UsbSetupPacket>, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn SendControlInTransferAsyncNoBuffer(&mut self, setuppacket: &::core::option::Option<UsbSetupPacket>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn DefaultInterface(&mut self) -> ::windows::core::Result<UsbInterface>;
    fn DeviceDescriptor(&mut self) -> ::windows::core::Result<UsbDeviceDescriptor>;
    fn Configuration(&mut self) -> ::windows::core::Result<UsbConfiguration>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbDevice {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDevice";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUsbDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbDevice_Vtbl {
        unsafe extern "system" fn SendControlOutTransferAsync<Impl: IUsbDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendControlOutTransferAsyncNoBuffer<Impl: IUsbDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendControlInTransferAsync<Impl: IUsbDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendControlInTransferAsyncNoBuffer<Impl: IUsbDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DefaultInterface<Impl: IUsbDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceDescriptor<Impl: IUsbDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Configuration<Impl: IUsbDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbDevice, BASE_OFFSET>(),
            SendControlOutTransferAsync: SendControlOutTransferAsync::<Impl, IMPL_OFFSET>,
            SendControlOutTransferAsyncNoBuffer: SendControlOutTransferAsyncNoBuffer::<Impl, IMPL_OFFSET>,
            SendControlInTransferAsync: SendControlInTransferAsync::<Impl, IMPL_OFFSET>,
            SendControlInTransferAsyncNoBuffer: SendControlInTransferAsyncNoBuffer::<Impl, IMPL_OFFSET>,
            DefaultInterface: DefaultInterface::<Impl, IMPL_OFFSET>,
            DeviceDescriptor: DeviceDescriptor::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUsbDeviceClass_Impl: Sized {
    fn ClassCode(&mut self) -> ::windows::core::Result<u8>;
    fn SetClassCode(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn SubclassCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
    fn SetSubclassCode(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u8>>) -> ::windows::core::Result<()>;
    fn ProtocolCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
    fn SetProtocolCode(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u8>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbDeviceClass {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceClass";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUsbDeviceClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceClass_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbDeviceClass_Vtbl {
        unsafe extern "system" fn ClassCode<Impl: IUsbDeviceClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClassCode<Impl: IUsbDeviceClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassCode(value).into()
        }
        unsafe extern "system" fn SubclassCode<Impl: IUsbDeviceClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSubclassCode<Impl: IUsbDeviceClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubclassCode(&*(&value as *const <super::super::Foundation::IReference<u8> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u8> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProtocolCode<Impl: IUsbDeviceClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProtocolCode<Impl: IUsbDeviceClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtocolCode(&*(&value as *const <super::super::Foundation::IReference<u8> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u8> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbDeviceClass, BASE_OFFSET>(),
            ClassCode: ClassCode::<Impl, IMPL_OFFSET>,
            SetClassCode: SetClassCode::<Impl, IMPL_OFFSET>,
            SubclassCode: SubclassCode::<Impl, IMPL_OFFSET>,
            SetSubclassCode: SetSubclassCode::<Impl, IMPL_OFFSET>,
            ProtocolCode: ProtocolCode::<Impl, IMPL_OFFSET>,
            SetProtocolCode: SetProtocolCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbDeviceClass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceClasses_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbDeviceClasses {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceClasses";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceClasses_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceClasses_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbDeviceClasses_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbDeviceClasses, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbDeviceClasses as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceClassesStatics_Impl: Sized {
    fn CdcControl(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn Physical(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn PersonalHealthcare(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn ActiveSync(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn PalmSync(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn DeviceFirmwareUpdate(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn Irda(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn Measurement(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
    fn VendorSpecific(&mut self) -> ::windows::core::Result<UsbDeviceClass>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbDeviceClassesStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceClassesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceClassesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceClassesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbDeviceClassesStatics_Vtbl {
        unsafe extern "system" fn CdcControl<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Physical<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PersonalHealthcare<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActiveSync<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PalmSync<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceFirmwareUpdate<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Irda<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Measurement<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VendorSpecific<Impl: IUsbDeviceClassesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbDeviceClassesStatics, BASE_OFFSET>(),
            CdcControl: CdcControl::<Impl, IMPL_OFFSET>,
            Physical: Physical::<Impl, IMPL_OFFSET>,
            PersonalHealthcare: PersonalHealthcare::<Impl, IMPL_OFFSET>,
            ActiveSync: ActiveSync::<Impl, IMPL_OFFSET>,
            PalmSync: PalmSync::<Impl, IMPL_OFFSET>,
            DeviceFirmwareUpdate: DeviceFirmwareUpdate::<Impl, IMPL_OFFSET>,
            Irda: Irda::<Impl, IMPL_OFFSET>,
            Measurement: Measurement::<Impl, IMPL_OFFSET>,
            VendorSpecific: VendorSpecific::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbDeviceClassesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbDeviceDescriptor_Impl: Sized {
    fn BcdUsb(&mut self) -> ::windows::core::Result<u32>;
    fn MaxPacketSize0(&mut self) -> ::windows::core::Result<u8>;
    fn VendorId(&mut self) -> ::windows::core::Result<u32>;
    fn ProductId(&mut self) -> ::windows::core::Result<u32>;
    fn BcdDeviceRevision(&mut self) -> ::windows::core::Result<u32>;
    fn NumberOfConfigurations(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbDeviceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbDeviceDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbDeviceDescriptor_Vtbl {
        unsafe extern "system" fn BcdUsb<Impl: IUsbDeviceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxPacketSize0<Impl: IUsbDeviceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VendorId<Impl: IUsbDeviceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProductId<Impl: IUsbDeviceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BcdDeviceRevision<Impl: IUsbDeviceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NumberOfConfigurations<Impl: IUsbDeviceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbDeviceDescriptor, BASE_OFFSET>(),
            BcdUsb: BcdUsb::<Impl, IMPL_OFFSET>,
            MaxPacketSize0: MaxPacketSize0::<Impl, IMPL_OFFSET>,
            VendorId: VendorId::<Impl, IMPL_OFFSET>,
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            BcdDeviceRevision: BcdDeviceRevision::<Impl, IMPL_OFFSET>,
            NumberOfConfigurations: NumberOfConfigurations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbDeviceDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUsbDeviceStatics_Impl: Sized {
    fn GetDeviceSelector(&mut self, vendorid: u32, productid: u32, winusbinterfaceclass: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorGuidOnly(&mut self, winusbinterfaceclass: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorVidPidOnly(&mut self, vendorid: u32, productid: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceClassSelector(&mut self, usbclass: &::core::option::Option<UsbDeviceClass>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UsbDevice>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUsbDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbDeviceStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IUsbDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorid: u32, productid: u32, winusbinterfaceclass: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorGuidOnly<Impl: IUsbDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, winusbinterfaceclass: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorVidPidOnly<Impl: IUsbDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorid: u32, productid: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceClassSelector<Impl: IUsbDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usbclass: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IUsbDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorGuidOnly: GetDeviceSelectorGuidOnly::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorVidPidOnly: GetDeviceSelectorVidPidOnly::<Impl, IMPL_OFFSET>,
            GetDeviceClassSelector: GetDeviceClassSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbEndpointDescriptor_Impl: Sized {
    fn EndpointNumber(&mut self) -> ::windows::core::Result<u8>;
    fn Direction(&mut self) -> ::windows::core::Result<UsbTransferDirection>;
    fn EndpointType(&mut self) -> ::windows::core::Result<UsbEndpointType>;
    fn AsBulkInEndpointDescriptor(&mut self) -> ::windows::core::Result<UsbBulkInEndpointDescriptor>;
    fn AsInterruptInEndpointDescriptor(&mut self) -> ::windows::core::Result<UsbInterruptInEndpointDescriptor>;
    fn AsBulkOutEndpointDescriptor(&mut self) -> ::windows::core::Result<UsbBulkOutEndpointDescriptor>;
    fn AsInterruptOutEndpointDescriptor(&mut self) -> ::windows::core::Result<UsbInterruptOutEndpointDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbEndpointDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbEndpointDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbEndpointDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbEndpointDescriptor_Vtbl {
        unsafe extern "system" fn EndpointNumber<Impl: IUsbEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Direction<Impl: IUsbEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbTransferDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointType<Impl: IUsbEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbEndpointType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AsBulkInEndpointDescriptor<Impl: IUsbEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AsInterruptInEndpointDescriptor<Impl: IUsbEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AsBulkOutEndpointDescriptor<Impl: IUsbEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AsInterruptOutEndpointDescriptor<Impl: IUsbEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbEndpointDescriptor, BASE_OFFSET>(),
            EndpointNumber: EndpointNumber::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            EndpointType: EndpointType::<Impl, IMPL_OFFSET>,
            AsBulkInEndpointDescriptor: AsBulkInEndpointDescriptor::<Impl, IMPL_OFFSET>,
            AsInterruptInEndpointDescriptor: AsInterruptInEndpointDescriptor::<Impl, IMPL_OFFSET>,
            AsBulkOutEndpointDescriptor: AsBulkOutEndpointDescriptor::<Impl, IMPL_OFFSET>,
            AsInterruptOutEndpointDescriptor: AsInterruptOutEndpointDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbEndpointDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbEndpointDescriptorStatics_Impl: Sized {
    fn TryParse(&mut self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbEndpointDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&mut self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbEndpointDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbEndpointDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbEndpointDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbEndpointDescriptorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbEndpointDescriptorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbEndpointDescriptorStatics_Vtbl {
        unsafe extern "system" fn TryParse<Impl: IUsbEndpointDescriptorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Parse<Impl: IUsbEndpointDescriptorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbEndpointDescriptorStatics, BASE_OFFSET>(),
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
            Parse: Parse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbEndpointDescriptorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUsbInterface_Impl: Sized {
    fn BulkInPipes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInPipe>>;
    fn InterruptInPipes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInPipe>>;
    fn BulkOutPipes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutPipe>>;
    fn InterruptOutPipes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutPipe>>;
    fn InterfaceSettings(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterfaceSetting>>;
    fn InterfaceNumber(&mut self) -> ::windows::core::Result<u8>;
    fn Descriptors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbInterface {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterface";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUsbInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterface_Vtbl {
        unsafe extern "system" fn BulkInPipes<Impl: IUsbInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterruptInPipes<Impl: IUsbInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BulkOutPipes<Impl: IUsbInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterruptOutPipes<Impl: IUsbInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterfaceSettings<Impl: IUsbInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterfaceNumber<Impl: IUsbInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Descriptors<Impl: IUsbInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterface, BASE_OFFSET>(),
            BulkInPipes: BulkInPipes::<Impl, IMPL_OFFSET>,
            InterruptInPipes: InterruptInPipes::<Impl, IMPL_OFFSET>,
            BulkOutPipes: BulkOutPipes::<Impl, IMPL_OFFSET>,
            InterruptOutPipes: InterruptOutPipes::<Impl, IMPL_OFFSET>,
            InterfaceSettings: InterfaceSettings::<Impl, IMPL_OFFSET>,
            InterfaceNumber: InterfaceNumber::<Impl, IMPL_OFFSET>,
            Descriptors: Descriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterfaceDescriptor_Impl: Sized {
    fn ClassCode(&mut self) -> ::windows::core::Result<u8>;
    fn SubclassCode(&mut self) -> ::windows::core::Result<u8>;
    fn ProtocolCode(&mut self) -> ::windows::core::Result<u8>;
    fn AlternateSettingNumber(&mut self) -> ::windows::core::Result<u8>;
    fn InterfaceNumber(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbInterfaceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterfaceDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterfaceDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterfaceDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterfaceDescriptor_Vtbl {
        unsafe extern "system" fn ClassCode<Impl: IUsbInterfaceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SubclassCode<Impl: IUsbInterfaceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtocolCode<Impl: IUsbInterfaceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AlternateSettingNumber<Impl: IUsbInterfaceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterfaceNumber<Impl: IUsbInterfaceDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterfaceDescriptor, BASE_OFFSET>(),
            ClassCode: ClassCode::<Impl, IMPL_OFFSET>,
            SubclassCode: SubclassCode::<Impl, IMPL_OFFSET>,
            ProtocolCode: ProtocolCode::<Impl, IMPL_OFFSET>,
            AlternateSettingNumber: AlternateSettingNumber::<Impl, IMPL_OFFSET>,
            InterfaceNumber: InterfaceNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterfaceDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbInterfaceDescriptorStatics_Impl: Sized {
    fn TryParse(&mut self, descriptor: &::core::option::Option<UsbDescriptor>, parsed: &mut ::core::option::Option<UsbInterfaceDescriptor>) -> ::windows::core::Result<bool>;
    fn Parse(&mut self, descriptor: &::core::option::Option<UsbDescriptor>) -> ::windows::core::Result<UsbInterfaceDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbInterfaceDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterfaceDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbInterfaceDescriptorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterfaceDescriptorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterfaceDescriptorStatics_Vtbl {
        unsafe extern "system" fn TryParse<Impl: IUsbInterfaceDescriptorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Parse<Impl: IUsbInterfaceDescriptorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterfaceDescriptorStatics, BASE_OFFSET>(),
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
            Parse: Parse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterfaceDescriptorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUsbInterfaceSetting_Impl: Sized {
    fn BulkInEndpoints(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInEndpointDescriptor>>;
    fn InterruptInEndpoints(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInEndpointDescriptor>>;
    fn BulkOutEndpoints(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutEndpointDescriptor>>;
    fn InterruptOutEndpoints(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutEndpointDescriptor>>;
    fn Selected(&mut self) -> ::windows::core::Result<bool>;
    fn SelectSettingAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InterfaceDescriptor(&mut self) -> ::windows::core::Result<UsbInterfaceDescriptor>;
    fn Descriptors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbInterfaceSetting {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterfaceSetting";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUsbInterfaceSetting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterfaceSetting_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterfaceSetting_Vtbl {
        unsafe extern "system" fn BulkInEndpoints<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterruptInEndpoints<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BulkOutEndpoints<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterruptOutEndpoints<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Selected<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectSettingAsync<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InterfaceDescriptor<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Descriptors<Impl: IUsbInterfaceSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterfaceSetting, BASE_OFFSET>(),
            BulkInEndpoints: BulkInEndpoints::<Impl, IMPL_OFFSET>,
            InterruptInEndpoints: InterruptInEndpoints::<Impl, IMPL_OFFSET>,
            BulkOutEndpoints: BulkOutEndpoints::<Impl, IMPL_OFFSET>,
            InterruptOutEndpoints: InterruptOutEndpoints::<Impl, IMPL_OFFSET>,
            Selected: Selected::<Impl, IMPL_OFFSET>,
            SelectSettingAsync: SelectSettingAsync::<Impl, IMPL_OFFSET>,
            InterfaceDescriptor: InterfaceDescriptor::<Impl, IMPL_OFFSET>,
            Descriptors: Descriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterfaceSetting as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUsbInterruptInEndpointDescriptor_Impl: Sized {
    fn MaxPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&mut self) -> ::windows::core::Result<u8>;
    fn Interval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Pipe(&mut self) -> ::windows::core::Result<UsbInterruptInPipe>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbInterruptInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptInEndpointDescriptor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUsbInterruptInEndpointDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptInEndpointDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterruptInEndpointDescriptor_Vtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbInterruptInEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointNumber<Impl: IUsbInterruptInEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Interval<Impl: IUsbInterruptInEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pipe<Impl: IUsbInterruptInEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterruptInEndpointDescriptor, BASE_OFFSET>(),
            MaxPacketSize: MaxPacketSize::<Impl, IMPL_OFFSET>,
            EndpointNumber: EndpointNumber::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            Pipe: Pipe::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterruptInEndpointDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUsbInterruptInEventArgs_Impl: Sized {
    fn InterruptData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbInterruptInEventArgs {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptInEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUsbInterruptInEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptInEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterruptInEventArgs_Vtbl {
        unsafe extern "system" fn InterruptData<Impl: IUsbInterruptInEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterruptInEventArgs, BASE_OFFSET>(),
            InterruptData: InterruptData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterruptInEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUsbInterruptInPipe_Impl: Sized {
    fn EndpointDescriptor(&mut self) -> ::windows::core::Result<UsbInterruptInEndpointDescriptor>;
    fn ClearStallAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DataReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UsbInterruptInPipe, UsbInterruptInEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbInterruptInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptInPipe";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUsbInterruptInPipe_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptInPipe_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterruptInPipe_Vtbl {
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbInterruptInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbInterruptInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DataReceived<Impl: IUsbInterruptInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDataReceived<Impl: IUsbInterruptInPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterruptInPipe, BASE_OFFSET>(),
            EndpointDescriptor: EndpointDescriptor::<Impl, IMPL_OFFSET>,
            ClearStallAsync: ClearStallAsync::<Impl, IMPL_OFFSET>,
            DataReceived: DataReceived::<Impl, IMPL_OFFSET>,
            RemoveDataReceived: RemoveDataReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterruptInPipe as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUsbInterruptOutEndpointDescriptor_Impl: Sized {
    fn MaxPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn EndpointNumber(&mut self) -> ::windows::core::Result<u8>;
    fn Interval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Pipe(&mut self) -> ::windows::core::Result<UsbInterruptOutPipe>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbInterruptOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptOutEndpointDescriptor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUsbInterruptOutEndpointDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptOutEndpointDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterruptOutEndpointDescriptor_Vtbl {
        unsafe extern "system" fn MaxPacketSize<Impl: IUsbInterruptOutEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointNumber<Impl: IUsbInterruptOutEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Interval<Impl: IUsbInterruptOutEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pipe<Impl: IUsbInterruptOutEndpointDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterruptOutEndpointDescriptor, BASE_OFFSET>(),
            MaxPacketSize: MaxPacketSize::<Impl, IMPL_OFFSET>,
            EndpointNumber: EndpointNumber::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            Pipe: Pipe::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterruptOutEndpointDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUsbInterruptOutPipe_Impl: Sized {
    fn EndpointDescriptor(&mut self) -> ::windows::core::Result<UsbInterruptOutEndpointDescriptor>;
    fn ClearStallAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetWriteOptions(&mut self, value: UsbWriteOptions) -> ::windows::core::Result<()>;
    fn WriteOptions(&mut self) -> ::windows::core::Result<UsbWriteOptions>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbInterruptOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbInterruptOutPipe";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUsbInterruptOutPipe_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbInterruptOutPipe_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbInterruptOutPipe_Vtbl {
        unsafe extern "system" fn EndpointDescriptor<Impl: IUsbInterruptOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearStallAsync<Impl: IUsbInterruptOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWriteOptions<Impl: IUsbInterruptOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UsbWriteOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteOptions(value).into()
        }
        unsafe extern "system" fn WriteOptions<Impl: IUsbInterruptOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UsbWriteOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputStream<Impl: IUsbInterruptOutPipe_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbInterruptOutPipe, BASE_OFFSET>(),
            EndpointDescriptor: EndpointDescriptor::<Impl, IMPL_OFFSET>,
            ClearStallAsync: ClearStallAsync::<Impl, IMPL_OFFSET>,
            SetWriteOptions: SetWriteOptions::<Impl, IMPL_OFFSET>,
            WriteOptions: WriteOptions::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbInterruptOutPipe as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUsbSetupPacket_Impl: Sized {
    fn RequestType(&mut self) -> ::windows::core::Result<UsbControlRequestType>;
    fn SetRequestType(&mut self, value: &::core::option::Option<UsbControlRequestType>) -> ::windows::core::Result<()>;
    fn Request(&mut self) -> ::windows::core::Result<u8>;
    fn SetRequest(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<u32>;
    fn SetValue(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Index(&mut self) -> ::windows::core::Result<u32>;
    fn SetIndex(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<u32>;
    fn SetLength(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUsbSetupPacket {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbSetupPacket";
}
#[cfg(feature = "implement_exclusive")]
impl IUsbSetupPacket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbSetupPacket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbSetupPacket_Vtbl {
        unsafe extern "system" fn RequestType<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequestType<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestType(&*(&value as *const <UsbControlRequestType as ::windows::core::Abi>::Abi as *const <UsbControlRequestType as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Request<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequest<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequest(value).into()
        }
        unsafe extern "system" fn Value<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn Index<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIndex<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndex(value).into()
        }
        unsafe extern "system" fn Length<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLength<Impl: IUsbSetupPacket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbSetupPacket, BASE_OFFSET>(),
            RequestType: RequestType::<Impl, IMPL_OFFSET>,
            SetRequestType: SetRequestType::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
            SetRequest: SetRequest::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Index: Index::<Impl, IMPL_OFFSET>,
            SetIndex: SetIndex::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbSetupPacket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IUsbSetupPacketFactory_Impl: Sized {
    fn CreateWithEightByteBuffer(&mut self, eightbytebuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<UsbSetupPacket>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUsbSetupPacketFactory {
    const NAME: &'static str = "Windows.Devices.Usb.IUsbSetupPacketFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IUsbSetupPacketFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUsbSetupPacketFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUsbSetupPacketFactory_Vtbl {
        unsafe extern "system" fn CreateWithEightByteBuffer<Impl: IUsbSetupPacketFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eightbytebuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUsbSetupPacketFactory, BASE_OFFSET>(),
            CreateWithEightByteBuffer: CreateWithEightByteBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUsbSetupPacketFactory as ::windows::core::Interface>::IID
    }
}
