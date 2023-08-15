#[doc = "*Required features: `\"Devices_I2c_Provider\"`, `\"implement\"`*"]
pub trait II2cControllerProvider_Impl: Sized {
    fn GetDeviceProvider(&self, settings: ::core::option::Option<&ProviderI2cConnectionSettings>) -> ::windows_core::Result<II2cDeviceProvider>;
}
impl ::windows_core::RuntimeName for II2cControllerProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cControllerProvider";
}
impl II2cControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cControllerProvider_Impl, const OFFSET: isize>() -> II2cControllerProvider_Vtbl {
        unsafe extern "system" fn GetDeviceProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, II2cControllerProvider, OFFSET>(),
            GetDeviceProvider: GetDeviceProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<II2cControllerProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait II2cDeviceProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Write(&self, buffer: &[u8]) -> ::windows_core::Result<()>;
    fn WritePartial(&self, buffer: &[u8]) -> ::windows_core::Result<ProviderI2cTransferResult>;
    fn Read(&self, buffer: &mut [u8]) -> ::windows_core::Result<()>;
    fn ReadPartial(&self, buffer: &mut [u8]) -> ::windows_core::Result<ProviderI2cTransferResult>;
    fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<()>;
    fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<ProviderI2cTransferResult>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for II2cDeviceProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cDeviceProvider";
}
#[cfg(feature = "Foundation")]
impl II2cDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>() -> II2cDeviceProvider_Vtbl {
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
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
        unsafe extern "system" fn Write<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
        }
        unsafe extern "system" fn WritePartial<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WritePartial(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
        }
        unsafe extern "system" fn ReadPartial<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadPartial(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteRead<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteRead(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)).into()
        }
        unsafe extern "system" fn WriteReadPartial<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteReadPartial(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, II2cDeviceProvider, OFFSET>(),
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            WritePartial: WritePartial::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            ReadPartial: ReadPartial::<Identity, Impl, OFFSET>,
            WriteRead: WriteRead::<Identity, Impl, OFFSET>,
            WriteReadPartial: WriteReadPartial::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<II2cDeviceProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_I2c_Provider\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait II2cProvider_Impl: Sized {
    fn GetControllersAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for II2cProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl II2cProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cProvider_Impl, const OFFSET: isize>() -> II2cProvider_Vtbl {
        unsafe extern "system" fn GetControllersAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: II2cProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, II2cProvider, OFFSET>(),
            GetControllersAsync: GetControllersAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<II2cProvider as ::windows_core::ComInterface>::IID
    }
}
