pub trait ISpiControllerProvider_Impl: Sized {
    fn GetDeviceProvider(&self, settings: &::core::option::Option<ProviderSpiConnectionSettings>) -> ::windows::core::Result<ISpiDeviceProvider>;
}
impl ::windows::core::RuntimeName for ISpiControllerProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiControllerProvider";
}
impl ISpiControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiControllerProvider_Impl, const OFFSET: isize>() -> ISpiControllerProvider_Vtbl {
        unsafe extern "system" fn GetDeviceProvider<Identity: ::windows::core::IUnknownImpl, Impl: ISpiControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceProvider(::core::mem::transmute(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiControllerProvider, OFFSET>(),
            GetDeviceProvider: GetDeviceProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISpiDeviceProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionSettings(&self) -> ::windows::core::Result<ProviderSpiConnectionSettings>;
    fn Write(&self, buffer: &[u8]) -> ::windows::core::Result<()>;
    fn Read(&self, buffer: &mut [u8]) -> ::windows::core::Result<()>;
    fn TransferSequential(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()>;
    fn TransferFullDuplex(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISpiDeviceProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiDeviceProvider";
}
#[cfg(feature = "Foundation")]
impl ISpiDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>() -> ISpiDeviceProvider_Vtbl {
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionSettings<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectionSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Write(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn TransferSequential<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TransferSequential(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn TransferFullDuplex<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TransferFullDuplex(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiDeviceProvider, OFFSET>(),
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            ConnectionSettings: ConnectionSettings::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            TransferSequential: TransferSequential::<Identity, Impl, OFFSET>,
            TransferFullDuplex: TransferFullDuplex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiDeviceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ISpiProvider_Impl: Sized {
    fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<ISpiControllerProvider>>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ISpiProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl ISpiProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiProvider_Impl, const OFFSET: isize>() -> ISpiProvider_Vtbl {
        unsafe extern "system" fn GetControllersAsync<Identity: ::windows::core::IUnknownImpl, Impl: ISpiProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetControllersAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiProvider, OFFSET>(),
            GetControllersAsync: GetControllersAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiProvider as ::windows::core::Interface>::IID
    }
}
