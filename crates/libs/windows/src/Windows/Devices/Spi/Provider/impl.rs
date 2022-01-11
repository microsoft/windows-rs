#[cfg(feature = "implement_exclusive")]
pub trait IProviderSpiConnectionSettingsImpl: Sized {
    fn ChipSelectLine(&self) -> ::windows::core::Result<i32>;
    fn SetChipSelectLine(&self, value: i32) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<ProviderSpiMode>;
    fn SetMode(&self, value: ProviderSpiMode) -> ::windows::core::Result<()>;
    fn DataBitLength(&self) -> ::windows::core::Result<i32>;
    fn SetDataBitLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn ClockFrequency(&self) -> ::windows::core::Result<i32>;
    fn SetClockFrequency(&self, value: i32) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<ProviderSpiSharingMode>;
    fn SetSharingMode(&self, value: ProviderSpiSharingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProviderSpiConnectionSettings {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.IProviderSpiConnectionSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IProviderSpiConnectionSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderSpiConnectionSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderSpiConnectionSettingsVtbl {
        unsafe extern "system" fn ChipSelectLine<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChipSelectLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChipSelectLine<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChipSelectLine(value).into()
        }
        unsafe extern "system" fn Mode<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderSpiMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderSpiMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn DataBitLength<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataBitLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataBitLength<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataBitLength(value).into()
        }
        unsafe extern "system" fn ClockFrequency<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClockFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClockFrequency<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClockFrequency(value).into()
        }
        unsafe extern "system" fn SharingMode<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderSpiSharingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSharingMode<Impl: IProviderSpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderSpiSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharingMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IProviderSpiConnectionSettings>,
            ::windows::core::GetTrustLevel,
            ChipSelectLine::<Impl, IMPL_OFFSET>,
            SetChipSelectLine::<Impl, IMPL_OFFSET>,
            Mode::<Impl, IMPL_OFFSET>,
            SetMode::<Impl, IMPL_OFFSET>,
            DataBitLength::<Impl, IMPL_OFFSET>,
            SetDataBitLength::<Impl, IMPL_OFFSET>,
            ClockFrequency::<Impl, IMPL_OFFSET>,
            SetClockFrequency::<Impl, IMPL_OFFSET>,
            SharingMode::<Impl, IMPL_OFFSET>,
            SetSharingMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderSpiConnectionSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProviderSpiConnectionSettingsFactoryImpl: Sized {
    fn Create(&self, chipselectline: i32) -> ::windows::core::Result<ProviderSpiConnectionSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProviderSpiConnectionSettingsFactory {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.IProviderSpiConnectionSettingsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProviderSpiConnectionSettingsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderSpiConnectionSettingsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderSpiConnectionSettingsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IProviderSpiConnectionSettingsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chipselectline: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(chipselectline) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProviderSpiConnectionSettingsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderSpiConnectionSettingsFactory as ::windows::core::Interface>::IID
    }
}
pub trait ISpiControllerProviderImpl: Sized {
    fn GetDeviceProvider(&self, settings: &::core::option::Option<ProviderSpiConnectionSettings>) -> ::windows::core::Result<ISpiDeviceProvider>;
}
impl ::windows::core::RuntimeName for ISpiControllerProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiControllerProvider";
}
impl ISpiControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiControllerProviderVtbl {
        unsafe extern "system" fn GetDeviceProvider<Impl: ISpiControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceProvider(&*(&settings as *const <ProviderSpiConnectionSettings as ::windows::core::Abi>::Abi as *const <ProviderSpiConnectionSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpiControllerProvider>, ::windows::core::GetTrustLevel, GetDeviceProvider::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISpiDeviceProviderImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionSettings(&self) -> ::windows::core::Result<ProviderSpiConnectionSettings>;
    fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferSequential(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferFullDuplex(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISpiDeviceProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiDeviceProvider";
}
#[cfg(feature = "Foundation")]
impl ISpiDeviceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiDeviceProviderVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISpiDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectionSettings<Impl: ISpiDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Write<Impl: ISpiDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn Read<Impl: ISpiDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn TransferSequential<Impl: ISpiDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferSequential(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn TransferFullDuplex<Impl: ISpiDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferFullDuplex(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISpiDeviceProvider>,
            ::windows::core::GetTrustLevel,
            DeviceId::<Impl, IMPL_OFFSET>,
            ConnectionSettings::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            Read::<Impl, IMPL_OFFSET>,
            TransferSequential::<Impl, IMPL_OFFSET>,
            TransferFullDuplex::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiDeviceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait ISpiProviderImpl: Sized {
    fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<ISpiControllerProvider>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for ISpiProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiProvider";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ISpiProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiProviderVtbl {
        unsafe extern "system" fn GetControllersAsync<Impl: ISpiProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllersAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpiProvider>, ::windows::core::GetTrustLevel, GetControllersAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiProvider as ::windows::core::Interface>::IID
    }
}
