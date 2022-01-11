#[cfg(feature = "implement_exclusive")]
pub trait II2cConnectionSettingsImpl: Sized {
    fn SlaveAddress(&self) -> ::windows::core::Result<i32>;
    fn SetSlaveAddress(&self, value: i32) -> ::windows::core::Result<()>;
    fn BusSpeed(&self) -> ::windows::core::Result<I2cBusSpeed>;
    fn SetBusSpeed(&self, value: I2cBusSpeed) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<I2cSharingMode>;
    fn SetSharingMode(&self, value: I2cSharingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for II2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.II2cConnectionSettings";
}
#[cfg(feature = "implement_exclusive")]
impl II2cConnectionSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: II2cConnectionSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> II2cConnectionSettingsVtbl {
        unsafe extern "system" fn SlaveAddress<Impl: II2cConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlaveAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlaveAddress<Impl: II2cConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSlaveAddress(value).into()
        }
        unsafe extern "system" fn BusSpeed<Impl: II2cConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut I2cBusSpeed) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusSpeed<Impl: II2cConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: I2cBusSpeed) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusSpeed(value).into()
        }
        unsafe extern "system" fn SharingMode<Impl: II2cConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut I2cSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharingMode<Impl: II2cConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: I2cSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharingMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<II2cConnectionSettings>,
            ::windows::core::GetTrustLevel,
            SlaveAddress::<Impl, IMPL_OFFSET>,
            SetSlaveAddress::<Impl, IMPL_OFFSET>,
            BusSpeed::<Impl, IMPL_OFFSET>,
            SetBusSpeed::<Impl, IMPL_OFFSET>,
            SharingMode::<Impl, IMPL_OFFSET>,
            SetSharingMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<II2cConnectionSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait II2cConnectionSettingsFactoryImpl: Sized {
    fn Create(&self, slaveaddress: i32) -> ::windows::core::Result<I2cConnectionSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for II2cConnectionSettingsFactory {
    const NAME: &'static str = "Windows.Devices.I2c.II2cConnectionSettingsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl II2cConnectionSettingsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: II2cConnectionSettingsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> II2cConnectionSettingsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: II2cConnectionSettingsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slaveaddress: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(slaveaddress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<II2cConnectionSettingsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<II2cConnectionSettingsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait II2cControllerImpl: Sized {
    fn GetDevice(&self, settings: &::core::option::Option<I2cConnectionSettings>) -> ::windows::core::Result<I2cDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for II2cController {
    const NAME: &'static str = "Windows.Devices.I2c.II2cController";
}
#[cfg(feature = "implement_exclusive")]
impl II2cControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: II2cControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> II2cControllerVtbl {
        unsafe extern "system" fn GetDevice<Impl: II2cControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&settings as *const <I2cConnectionSettings as ::windows::core::Abi>::Abi as *const <I2cConnectionSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<II2cController>, ::windows::core::GetTrustLevel, GetDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<II2cController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait II2cControllerStaticsImpl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::II2cProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<I2cController>>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cController>>;
}
#[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for II2cControllerStatics {
    const NAME: &'static str = "Windows.Devices.I2c.II2cControllerStatics";
}
#[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl II2cControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: II2cControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> II2cControllerStaticsVtbl {
        unsafe extern "system" fn GetControllersAsync<Impl: II2cControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllersAsync(&*(&provider as *const <Provider::II2cProvider as ::windows::core::Abi>::Abi as *const <Provider::II2cProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAsync<Impl: II2cControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<II2cControllerStatics>, ::windows::core::GetTrustLevel, GetControllersAsync::<Impl, IMPL_OFFSET>, GetDefaultAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<II2cControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait II2cDeviceImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionSettings(&self) -> ::windows::core::Result<I2cConnectionSettings>;
    fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WritePartial(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult>;
    fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ReadPartial(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult>;
    fn WriteRead(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WriteReadPartial(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for II2cDevice {
    const NAME: &'static str = "Windows.Devices.I2c.II2cDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl II2cDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: II2cDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> II2cDeviceVtbl {
        unsafe extern "system" fn DeviceId<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionSettings<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn WritePartial<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WritePartial(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn ReadPartial<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadPartial(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteRead<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteRead(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn WriteReadPartial<Impl: II2cDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteReadPartial(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<II2cDevice>,
            ::windows::core::GetTrustLevel,
            DeviceId::<Impl, IMPL_OFFSET>,
            ConnectionSettings::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            WritePartial::<Impl, IMPL_OFFSET>,
            Read::<Impl, IMPL_OFFSET>,
            ReadPartial::<Impl, IMPL_OFFSET>,
            WriteRead::<Impl, IMPL_OFFSET>,
            WriteReadPartial::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<II2cDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait II2cDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, settings: &::core::option::Option<I2cConnectionSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for II2cDeviceStatics {
    const NAME: &'static str = "Windows.Devices.I2c.II2cDeviceStatics";
}
#[cfg(feature = "Foundation")]
impl II2cDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: II2cDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> II2cDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: II2cDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Impl: II2cDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromFriendlyName(&*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: II2cDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&settings as *const <I2cConnectionSettings as ::windows::core::Abi>::Abi as *const <I2cConnectionSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<II2cDeviceStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, GetDeviceSelectorFromFriendlyName::<Impl, IMPL_OFFSET>, FromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<II2cDeviceStatics as ::windows::core::Interface>::IID
    }
}
