#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGpioChangeCounterImpl: Sized + IClosableImpl {
    fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()>;
    fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity>;
    fn IsStarted(&self) -> ::windows::core::Result<bool>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Read(&self) -> ::windows::core::Result<GpioChangeCount>;
    fn Reset(&self) -> ::windows::core::Result<GpioChangeCount>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGpioChangeCounter {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioChangeCounter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGpioChangeCounterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioChangeCounterImpl, const OFFSET: isize>() -> IGpioChangeCounterVtbl {
        unsafe extern "system" fn SetPolarity<Impl: IGpioChangeCounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GpioChangePolarity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPolarity(value).into()
        }
        unsafe extern "system" fn Polarity<Impl: IGpioChangeCounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Polarity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStarted<Impl: IGpioChangeCounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IGpioChangeCounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IGpioChangeCounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Read<Impl: IGpioChangeCounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IGpioChangeCounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGpioChangeCounter>, ::windows::core::GetTrustLevel, SetPolarity::<Impl, OFFSET>, Polarity::<Impl, OFFSET>, IsStarted::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Read::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioChangeCounterFactoryImpl: Sized {
    fn Create(&self, pin: &::core::option::Option<GpioPin>) -> ::windows::core::Result<GpioChangeCounter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioChangeCounterFactory {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioChangeCounterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioChangeCounterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioChangeCounterFactoryImpl, const OFFSET: isize>() -> IGpioChangeCounterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGpioChangeCounterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&pin as *const <GpioPin as ::windows::core::Abi>::Abi as *const <GpioPin as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGpioChangeCounterFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGpioChangeReaderImpl: Sized + IClosableImpl {
    fn Capacity(&self) -> ::windows::core::Result<i32>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn IsEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsOverflowed(&self) -> ::windows::core::Result<bool>;
    fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()>;
    fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity>;
    fn IsStarted(&self) -> ::windows::core::Result<bool>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn GetNextItem(&self) -> ::windows::core::Result<GpioChangeRecord>;
    fn PeekNextItem(&self) -> ::windows::core::Result<GpioChangeRecord>;
    fn GetAllItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<GpioChangeRecord>>;
    fn WaitForItemsAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGpioChangeReader {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioChangeReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGpioChangeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioChangeReaderImpl, const OFFSET: isize>() -> IGpioChangeReaderVtbl {
        unsafe extern "system" fn Capacity<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEmpty<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOverflowed<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverflowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolarity<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GpioChangePolarity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPolarity(value).into()
        }
        unsafe extern "system" fn Polarity<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Polarity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStarted<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Clear<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn GetNextItem<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextItem<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNextItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllItems<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForItemsAsync<Impl: IGpioChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForItemsAsync(count) {
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
            ::windows::core::GetRuntimeClassName::<IGpioChangeReader>,
            ::windows::core::GetTrustLevel,
            Capacity::<Impl, OFFSET>,
            Length::<Impl, OFFSET>,
            IsEmpty::<Impl, OFFSET>,
            IsOverflowed::<Impl, OFFSET>,
            SetPolarity::<Impl, OFFSET>,
            Polarity::<Impl, OFFSET>,
            IsStarted::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            GetNextItem::<Impl, OFFSET>,
            PeekNextItem::<Impl, OFFSET>,
            GetAllItems::<Impl, OFFSET>,
            WaitForItemsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioChangeReaderFactoryImpl: Sized {
    fn Create(&self, pin: &::core::option::Option<GpioPin>) -> ::windows::core::Result<GpioChangeReader>;
    fn CreateWithCapacity(&self, pin: &::core::option::Option<GpioPin>, mincapacity: i32) -> ::windows::core::Result<GpioChangeReader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioChangeReaderFactory {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioChangeReaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioChangeReaderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioChangeReaderFactoryImpl, const OFFSET: isize>() -> IGpioChangeReaderFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGpioChangeReaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&pin as *const <GpioPin as ::windows::core::Abi>::Abi as *const <GpioPin as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithCapacity<Impl: IGpioChangeReaderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, mincapacity: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithCapacity(&*(&pin as *const <GpioPin as ::windows::core::Abi>::Abi as *const <GpioPin as ::windows::core::DefaultType>::DefaultType), mincapacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGpioChangeReaderFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>, CreateWithCapacity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioControllerImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<GpioPin>;
    fn OpenPinWithSharingMode(&self, pinnumber: i32, sharingmode: GpioSharingMode) -> ::windows::core::Result<GpioPin>;
    fn TryOpenPin(&self, pinnumber: i32, sharingmode: GpioSharingMode, pin: &mut ::core::option::Option<GpioPin>, openstatus: &mut GpioOpenStatus) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioController {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioController";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioControllerImpl, const OFFSET: isize>() -> IGpioControllerVtbl {
        unsafe extern "system" fn PinCount<Impl: IGpioControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPin<Impl: IGpioControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnumber: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPin(pinnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPinWithSharingMode<Impl: IGpioControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnumber: i32, sharingmode: GpioSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPinWithSharingMode(pinnumber, sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryOpenPin<Impl: IGpioControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnumber: i32, sharingmode: GpioSharingMode, pin: *mut ::windows::core::RawPtr, openstatus: *mut GpioOpenStatus, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryOpenPin(pinnumber, sharingmode, ::core::mem::transmute_copy(&pin), ::core::mem::transmute_copy(&openstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGpioController>, ::windows::core::GetTrustLevel, PinCount::<Impl, OFFSET>, OpenPin::<Impl, OFFSET>, OpenPinWithSharingMode::<Impl, OFFSET>, TryOpenPin::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioControllerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GpioController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioControllerStatics {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioControllerStaticsImpl, const OFFSET: isize>() -> IGpioControllerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IGpioControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGpioControllerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioControllerStatics2Impl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::IGpioProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GpioController>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioControllerStatics2 {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioControllerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioControllerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioControllerStatics2Impl, const OFFSET: isize>() -> IGpioControllerStatics2Vtbl {
        unsafe extern "system" fn GetControllersAsync<Impl: IGpioControllerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllersAsync(&*(&provider as *const <Provider::IGpioProvider as ::windows::core::Abi>::Abi as *const <Provider::IGpioProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAsync<Impl: IGpioControllerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGpioControllerStatics2>, ::windows::core::GetTrustLevel, GetControllersAsync::<Impl, OFFSET>, GetDefaultAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGpioPinImpl: Sized + IClosableImpl {
    fn ValueChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PinNumber(&self) -> ::windows::core::Result<i32>;
    fn SharingMode(&self) -> ::windows::core::Result<GpioSharingMode>;
    fn IsDriveModeSupported(&self, drivemode: GpioPinDriveMode) -> ::windows::core::Result<bool>;
    fn GetDriveMode(&self) -> ::windows::core::Result<GpioPinDriveMode>;
    fn SetDriveMode(&self, value: GpioPinDriveMode) -> ::windows::core::Result<()>;
    fn Write(&self, value: GpioPinValue) -> ::windows::core::Result<()>;
    fn Read(&self) -> ::windows::core::Result<GpioPinValue>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGpioPin {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioPin";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGpioPinVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioPinImpl, const OFFSET: isize>() -> IGpioPinVtbl {
        unsafe extern "system" fn ValueChanged<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValueChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DebounceTimeout<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DebounceTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDebounceTimeout<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDebounceTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PinNumber<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingMode<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioSharingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDriveModeSupported<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivemode: GpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDriveModeSupported(drivemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDriveMode<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioPinDriveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDriveMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDriveMode<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GpioPinDriveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDriveMode(value).into()
        }
        unsafe extern "system" fn Write<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GpioPinValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(value).into()
        }
        unsafe extern "system" fn Read<Impl: IGpioPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioPinValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read() {
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
            ::windows::core::GetRuntimeClassName::<IGpioPin>,
            ::windows::core::GetTrustLevel,
            ValueChanged::<Impl, OFFSET>,
            RemoveValueChanged::<Impl, OFFSET>,
            DebounceTimeout::<Impl, OFFSET>,
            SetDebounceTimeout::<Impl, OFFSET>,
            PinNumber::<Impl, OFFSET>,
            SharingMode::<Impl, OFFSET>,
            IsDriveModeSupported::<Impl, OFFSET>,
            GetDriveMode::<Impl, OFFSET>,
            SetDriveMode::<Impl, OFFSET>,
            Write::<Impl, OFFSET>,
            Read::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinValueChangedEventArgsImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<GpioPinEdge>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioPinValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.IGpioPinValueChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioPinValueChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioPinValueChangedEventArgsImpl, const OFFSET: isize>() -> IGpioPinValueChangedEventArgsVtbl {
        unsafe extern "system" fn Edge<Impl: IGpioPinValueChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GpioPinEdge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Edge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGpioPinValueChangedEventArgs>, ::windows::core::GetTrustLevel, Edge::<Impl, OFFSET>)
    }
}
