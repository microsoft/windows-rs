#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISpiBusInfoImpl: Sized {
    fn ChipSelectLineCount(&mut self) -> ::windows::core::Result<i32>;
    fn MinClockFrequency(&mut self) -> ::windows::core::Result<i32>;
    fn MaxClockFrequency(&mut self) -> ::windows::core::Result<i32>;
    fn SupportedDataBitLengths(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpiBusInfo {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiBusInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISpiBusInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiBusInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiBusInfoVtbl {
        unsafe extern "system" fn ChipSelectLineCount<Impl: ISpiBusInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChipSelectLineCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinClockFrequency<Impl: ISpiBusInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinClockFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxClockFrequency<Impl: ISpiBusInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxClockFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedDataBitLengths<Impl: ISpiBusInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedDataBitLengths() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiBusInfo, BASE_OFFSET>(),
            ChipSelectLineCount: ChipSelectLineCount::<Impl, IMPL_OFFSET>,
            MinClockFrequency: MinClockFrequency::<Impl, IMPL_OFFSET>,
            MaxClockFrequency: MaxClockFrequency::<Impl, IMPL_OFFSET>,
            SupportedDataBitLengths: SupportedDataBitLengths::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiBusInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpiConnectionSettingsImpl: Sized {
    fn ChipSelectLine(&mut self) -> ::windows::core::Result<i32>;
    fn SetChipSelectLine(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Mode(&mut self) -> ::windows::core::Result<SpiMode>;
    fn SetMode(&mut self, value: SpiMode) -> ::windows::core::Result<()>;
    fn DataBitLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetDataBitLength(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ClockFrequency(&mut self) -> ::windows::core::Result<i32>;
    fn SetClockFrequency(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SharingMode(&mut self) -> ::windows::core::Result<SpiSharingMode>;
    fn SetSharingMode(&mut self, value: SpiSharingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpiConnectionSettings {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiConnectionSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ISpiConnectionSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiConnectionSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiConnectionSettingsVtbl {
        unsafe extern "system" fn ChipSelectLine<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetChipSelectLine<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChipSelectLine(value).into()
        }
        unsafe extern "system" fn Mode<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpiMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMode<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpiMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn DataBitLength<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDataBitLength<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataBitLength(value).into()
        }
        unsafe extern "system" fn ClockFrequency<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClockFrequency<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClockFrequency(value).into()
        }
        unsafe extern "system" fn SharingMode<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpiSharingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSharingMode<Impl: ISpiConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpiSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharingMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiConnectionSettings, BASE_OFFSET>(),
            ChipSelectLine: ChipSelectLine::<Impl, IMPL_OFFSET>,
            SetChipSelectLine: SetChipSelectLine::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            DataBitLength: DataBitLength::<Impl, IMPL_OFFSET>,
            SetDataBitLength: SetDataBitLength::<Impl, IMPL_OFFSET>,
            ClockFrequency: ClockFrequency::<Impl, IMPL_OFFSET>,
            SetClockFrequency: SetClockFrequency::<Impl, IMPL_OFFSET>,
            SharingMode: SharingMode::<Impl, IMPL_OFFSET>,
            SetSharingMode: SetSharingMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiConnectionSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpiConnectionSettingsFactoryImpl: Sized {
    fn Create(&mut self, chipselectline: i32) -> ::windows::core::Result<SpiConnectionSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpiConnectionSettingsFactory {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiConnectionSettingsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpiConnectionSettingsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiConnectionSettingsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiConnectionSettingsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISpiConnectionSettingsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chipselectline: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiConnectionSettingsFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiConnectionSettingsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpiControllerImpl: Sized {
    fn GetDevice(&mut self, settings: &::core::option::Option<SpiConnectionSettings>) -> ::windows::core::Result<SpiDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpiController {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiController";
}
#[cfg(feature = "implement_exclusive")]
impl ISpiControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiControllerVtbl {
        unsafe extern "system" fn GetDevice<Impl: ISpiControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&settings as *const <SpiConnectionSettings as ::windows::core::Abi>::Abi as *const <SpiConnectionSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiController, BASE_OFFSET>(), GetDevice: GetDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Spi_Provider", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpiControllerStaticsImpl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiController>>;
    fn GetControllersAsync(&mut self, provider: &::core::option::Option<Provider::ISpiProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SpiController>>>;
}
#[cfg(all(feature = "Devices_Spi_Provider", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpiControllerStatics {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiControllerStatics";
}
#[cfg(all(feature = "Devices_Spi_Provider", feature = "Foundation", feature = "implement_exclusive"))]
impl ISpiControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiControllerStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: ISpiControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetControllersAsync<Impl: ISpiControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllersAsync(&*(&provider as *const <Provider::ISpiProvider as ::windows::core::Abi>::Abi as *const <Provider::ISpiProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiControllerStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            GetControllersAsync: GetControllersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpiDeviceImpl: Sized + IClosableImpl {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionSettings(&mut self) -> ::windows::core::Result<SpiConnectionSettings>;
    fn Write(&mut self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn Read(&mut self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferSequential(&mut self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferFullDuplex(&mut self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpiDevice {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpiDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiDeviceVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISpiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectionSettings<Impl: ISpiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Write<Impl: ISpiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn Read<Impl: ISpiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _)).into()
        }
        unsafe extern "system" fn TransferSequential<Impl: ISpiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferSequential(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn TransferFullDuplex<Impl: ISpiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferFullDuplex(::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiDevice, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            ConnectionSettings: ConnectionSettings::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            TransferSequential: TransferSequential::<Impl, IMPL_OFFSET>,
            TransferFullDuplex: TransferFullDuplex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISpiDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&mut self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBusInfo(&mut self, busid: &::windows::core::HSTRING) -> ::windows::core::Result<SpiBusInfo>;
    fn FromIdAsync(&mut self, busid: &::windows::core::HSTRING, settings: &::core::option::Option<SpiConnectionSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiDevice>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISpiDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiDeviceStatics";
}
#[cfg(feature = "Foundation")]
impl ISpiDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpiDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpiDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ISpiDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Impl: ISpiDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBusInfo<Impl: ISpiDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBusInfo(&*(&busid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ISpiDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&busid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&settings as *const <SpiConnectionSettings as ::windows::core::Abi>::Abi as *const <SpiConnectionSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpiDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromFriendlyName: GetDeviceSelectorFromFriendlyName::<Impl, IMPL_OFFSET>,
            GetBusInfo: GetBusInfo::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpiDeviceStatics as ::windows::core::Interface>::IID
    }
}
