pub trait II2cControllerProviderImpl: Sized {
    fn GetDeviceProvider(&self, settings: &::core::option::Option<ProviderI2cConnectionSettings>) -> ::windows::core::Result<II2cDeviceProvider>;
}
impl ::windows::core::RuntimeName for II2cControllerProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cControllerProvider";
}
impl II2cControllerProviderVtbl {
    pub const fn new<Impl: II2cControllerProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> II2cControllerProviderVtbl {
        unsafe extern "system" fn GetDeviceProvider<Impl: II2cControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceProvider(&*(&settings as *const <ProviderI2cConnectionSettings as ::windows::core::Abi>::Abi as *const <ProviderI2cConnectionSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<II2cControllerProvider>, base.5, GetDeviceProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait II2cDeviceProviderImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WritePartial(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<ProviderI2cTransferResult>;
    fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ReadPartial(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<ProviderI2cTransferResult>;
    fn WriteRead(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WriteReadPartial(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<ProviderI2cTransferResult>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for II2cDeviceProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cDeviceProvider";
}
#[cfg(feature = "Foundation")]
impl II2cDeviceProviderVtbl {
    pub const fn new<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> II2cDeviceProviderVtbl {
        unsafe extern "system" fn DeviceId<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Write(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn WritePartial<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WritePartial(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Read(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn ReadPartial<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadPartial(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteRead<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteRead(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn WriteReadPartial<Impl: II2cDeviceProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteReadPartial(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<II2cDeviceProvider>, base.5, DeviceId::<Impl, OFFSET>, Write::<Impl, OFFSET>, WritePartial::<Impl, OFFSET>, Read::<Impl, OFFSET>, ReadPartial::<Impl, OFFSET>, WriteRead::<Impl, OFFSET>, WriteReadPartial::<Impl, OFFSET>)
    }
}
pub trait II2cProviderImpl: Sized {
    fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>>>;
}
impl ::windows::core::RuntimeName for II2cProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cProvider";
}
impl II2cProviderVtbl {
    pub const fn new<Impl: II2cProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> II2cProviderVtbl {
        unsafe extern "system" fn GetControllersAsync<Impl: II2cProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetControllersAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<II2cProvider>, base.5, GetControllersAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProviderI2cConnectionSettingsImpl: Sized {
    fn SlaveAddress(&self) -> ::windows::core::Result<i32>;
    fn SetSlaveAddress(&self, value: i32) -> ::windows::core::Result<()>;
    fn BusSpeed(&self) -> ::windows::core::Result<ProviderI2cBusSpeed>;
    fn SetBusSpeed(&self, value: ProviderI2cBusSpeed) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<ProviderI2cSharingMode>;
    fn SetSharingMode(&self, value: ProviderI2cSharingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProviderI2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.IProviderI2cConnectionSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IProviderI2cConnectionSettingsVtbl {
    pub const fn new<Impl: IProviderI2cConnectionSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProviderI2cConnectionSettingsVtbl {
        unsafe extern "system" fn SlaveAddress<Impl: IProviderI2cConnectionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SlaveAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlaveAddress<Impl: IProviderI2cConnectionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSlaveAddress(value).into()
        }
        unsafe extern "system" fn BusSpeed<Impl: IProviderI2cConnectionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderI2cBusSpeed) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BusSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusSpeed<Impl: IProviderI2cConnectionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ProviderI2cBusSpeed) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBusSpeed(value).into()
        }
        unsafe extern "system" fn SharingMode<Impl: IProviderI2cConnectionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderI2cSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SharingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharingMode<Impl: IProviderI2cConnectionSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ProviderI2cSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSharingMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProviderI2cConnectionSettings>, base.5, SlaveAddress::<Impl, OFFSET>, SetSlaveAddress::<Impl, OFFSET>, BusSpeed::<Impl, OFFSET>, SetBusSpeed::<Impl, OFFSET>, SharingMode::<Impl, OFFSET>, SetSharingMode::<Impl, OFFSET>)
    }
}
