#[doc = "*Required features: `\"Devices_Spi_Provider\"`, `\"implement\"`*"]
pub trait ISpiControllerProvider_Impl: Sized {
    fn GetDeviceProvider(&self, settings: ::core::option::Option<&ProviderSpiConnectionSettings>) -> ::windows_core::Result<ISpiDeviceProvider>;
}
impl ::windows_core::RuntimeName for ISpiControllerProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiControllerProvider";
}
impl ISpiControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiControllerProvider_Impl, const OFFSET: isize>() -> ISpiControllerProvider_Vtbl {
        unsafe extern "system" fn GetDeviceProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceProvider(::windows_core::from_raw_borrowed(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISpiControllerProvider, OFFSET>(),
            GetDeviceProvider: GetDeviceProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpiControllerProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait ISpiDeviceProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ConnectionSettings(&self) -> ::windows_core::Result<ProviderSpiConnectionSettings>;
    fn Write(&self, buffer: &[u8]) -> ::windows_core::Result<()>;
    fn Read(&self, buffer: &mut [u8]) -> ::windows_core::Result<()>;
    fn TransferSequential(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<()>;
    fn TransferFullDuplex(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ISpiDeviceProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiDeviceProvider";
}
#[cfg(feature = "Foundation")]
impl ISpiDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>() -> ISpiDeviceProvider_Vtbl {
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionSettings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectionSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
        }
        unsafe extern "system" fn TransferSequential<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferSequential(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)).into()
        }
        unsafe extern "system" fn TransferFullDuplex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferFullDuplex(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISpiDeviceProvider, OFFSET>(),
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            ConnectionSettings: ConnectionSettings::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            TransferSequential: TransferSequential::<Identity, Impl, OFFSET>,
            TransferFullDuplex: TransferFullDuplex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpiDeviceProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_Spi_Provider\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait ISpiProvider_Impl: Sized {
    fn GetControllersAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<ISpiControllerProvider>>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ISpiProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl ISpiProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiProvider_Impl, const OFFSET: isize>() -> ISpiProvider_Vtbl {
        unsafe extern "system" fn GetControllersAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpiProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetControllersAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISpiProvider, OFFSET>(),
            GetControllersAsync: GetControllersAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpiProvider as ::windows_core::ComInterface>::IID
    }
}
