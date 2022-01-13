pub trait IGpioControllerProviderImpl: Sized {
    fn PinCount(&mut self) -> ::windows::core::Result<i32>;
    fn OpenPinProvider(&mut self, pin: i32, sharingmode: ProviderGpioSharingMode) -> ::windows::core::Result<IGpioPinProvider>;
}
impl ::windows::core::RuntimeName for IGpioControllerProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioControllerProvider";
}
impl IGpioControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGpioControllerProviderVtbl {
        unsafe extern "system" fn PinCount<Impl: IGpioControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenPinProvider<Impl: IGpioControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPinProvider(pin, sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGpioControllerProvider, BASE_OFFSET>(),
            PinCount: PinCount::<Impl, IMPL_OFFSET>,
            OpenPinProvider: OpenPinProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IGpioPinProviderImpl: Sized {
    fn ValueChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DebounceTimeout(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PinNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SharingMode(&mut self) -> ::windows::core::Result<ProviderGpioSharingMode>;
    fn IsDriveModeSupported(&mut self, drivemode: ProviderGpioPinDriveMode) -> ::windows::core::Result<bool>;
    fn GetDriveMode(&mut self) -> ::windows::core::Result<ProviderGpioPinDriveMode>;
    fn SetDriveMode(&mut self, value: ProviderGpioPinDriveMode) -> ::windows::core::Result<()>;
    fn Write(&mut self, value: ProviderGpioPinValue) -> ::windows::core::Result<()>;
    fn Read(&mut self) -> ::windows::core::Result<ProviderGpioPinValue>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IGpioPinProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProvider";
}
#[cfg(feature = "Foundation")]
impl IGpioPinProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioPinProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGpioPinProviderVtbl {
        unsafe extern "system" fn ValueChanged<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValueChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DebounceTimeout<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDebounceTimeout<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDebounceTimeout(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PinNumber<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SharingMode<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioSharingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDriveModeSupported<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDriveMode<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinDriveMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDriveMode<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinDriveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDriveMode(value).into()
        }
        unsafe extern "system" fn Write<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(value).into()
        }
        unsafe extern "system" fn Read<Impl: IGpioPinProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinValue) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGpioPinProvider, BASE_OFFSET>(),
            ValueChanged: ValueChanged::<Impl, IMPL_OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Impl, IMPL_OFFSET>,
            DebounceTimeout: DebounceTimeout::<Impl, IMPL_OFFSET>,
            SetDebounceTimeout: SetDebounceTimeout::<Impl, IMPL_OFFSET>,
            PinNumber: PinNumber::<Impl, IMPL_OFFSET>,
            SharingMode: SharingMode::<Impl, IMPL_OFFSET>,
            IsDriveModeSupported: IsDriveModeSupported::<Impl, IMPL_OFFSET>,
            GetDriveMode: GetDriveMode::<Impl, IMPL_OFFSET>,
            SetDriveMode: SetDriveMode::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioPinProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinProviderValueChangedEventArgsImpl: Sized {
    fn Edge(&mut self) -> ::windows::core::Result<ProviderGpioPinEdge>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioPinProviderValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioPinProviderValueChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioPinProviderValueChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGpioPinProviderValueChangedEventArgsVtbl {
        unsafe extern "system" fn Edge<Impl: IGpioPinProviderValueChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinEdge) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGpioPinProviderValueChangedEventArgs, BASE_OFFSET>(), Edge: Edge::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioPinProviderValueChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinProviderValueChangedEventArgsFactoryImpl: Sized {
    fn Create(&mut self, edge: ProviderGpioPinEdge) -> ::windows::core::Result<GpioPinProviderValueChangedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioPinProviderValueChangedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioPinProviderValueChangedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioPinProviderValueChangedEventArgsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGpioPinProviderValueChangedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGpioPinProviderValueChangedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edge: ProviderGpioPinEdge, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(edge) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGpioPinProviderValueChangedEventArgsFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioPinProviderValueChangedEventArgsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IGpioProviderImpl: Sized {
    fn GetControllers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IGpioProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IGpioProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGpioProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGpioProviderVtbl {
        unsafe extern "system" fn GetControllers<Impl: IGpioProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGpioProvider, BASE_OFFSET>(), GetControllers: GetControllers::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioProvider as ::windows::core::Interface>::IID
    }
}
