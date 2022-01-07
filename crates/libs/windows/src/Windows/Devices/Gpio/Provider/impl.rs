pub trait IGpioControllerProviderImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn OpenPinProvider(&self, pin: i32, sharingmode: ProviderGpioSharingMode) -> ::windows::core::Result<IGpioPinProvider>;
}
impl ::windows::core::RuntimeName for IGpioControllerProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioControllerProvider";
}
impl IGpioControllerProviderVtbl {
    pub const fn new<Impl: IGpioControllerProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGpioControllerProviderVtbl {
        unsafe extern "system" fn PinCount<Impl: IGpioControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPinProvider<Impl: IGpioControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenPinProvider(pin, sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGpioControllerProvider>, base.5, PinCount::<Impl, OFFSET>, OpenPinProvider::<Impl, OFFSET>)
    }
}
pub trait IGpioPinProviderImpl: Sized {
    fn ValueChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PinNumber(&self) -> ::windows::core::Result<i32>;
    fn SharingMode(&self) -> ::windows::core::Result<ProviderGpioSharingMode>;
    fn IsDriveModeSupported(&self, drivemode: ProviderGpioPinDriveMode) -> ::windows::core::Result<bool>;
    fn GetDriveMode(&self) -> ::windows::core::Result<ProviderGpioPinDriveMode>;
    fn SetDriveMode(&self, value: ProviderGpioPinDriveMode) -> ::windows::core::Result<()>;
    fn Write(&self, value: ProviderGpioPinValue) -> ::windows::core::Result<()>;
    fn Read(&self) -> ::windows::core::Result<ProviderGpioPinValue>;
}
impl ::windows::core::RuntimeName for IGpioPinProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProvider";
}
impl IGpioPinProviderVtbl {
    pub const fn new<Impl: IGpioPinProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGpioPinProviderVtbl {
        unsafe extern "system" fn ValueChanged<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveValueChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DebounceTimeout<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DebounceTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDebounceTimeout<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDebounceTimeout(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PinNumber<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingMode<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioSharingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDriveModeSupported<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDriveModeSupported(drivemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDriveMode<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinDriveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDriveMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDriveMode<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinDriveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDriveMode(value).into()
        }
        unsafe extern "system" fn Write<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Write(value).into()
        }
        unsafe extern "system" fn Read<Impl: IGpioPinProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Read() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGpioPinProvider>, base.5, ValueChanged::<Impl, OFFSET>, RemoveValueChanged::<Impl, OFFSET>, DebounceTimeout::<Impl, OFFSET>, SetDebounceTimeout::<Impl, OFFSET>, PinNumber::<Impl, OFFSET>, SharingMode::<Impl, OFFSET>, IsDriveModeSupported::<Impl, OFFSET>, GetDriveMode::<Impl, OFFSET>, SetDriveMode::<Impl, OFFSET>, Write::<Impl, OFFSET>, Read::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinProviderValueChangedEventArgsImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<ProviderGpioPinEdge>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioPinProviderValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioPinProviderValueChangedEventArgsVtbl {
    pub const fn new<Impl: IGpioPinProviderValueChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGpioPinProviderValueChangedEventArgsVtbl {
        unsafe extern "system" fn Edge<Impl: IGpioPinProviderValueChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinEdge) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Edge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGpioPinProviderValueChangedEventArgs>, base.5, Edge::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinProviderValueChangedEventArgsFactoryImpl: Sized {
    fn Create(&self, edge: ProviderGpioPinEdge) -> ::windows::core::Result<GpioPinProviderValueChangedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGpioPinProviderValueChangedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProviderValueChangedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGpioPinProviderValueChangedEventArgsFactoryVtbl {
    pub const fn new<Impl: IGpioPinProviderValueChangedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGpioPinProviderValueChangedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGpioPinProviderValueChangedEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, edge: ProviderGpioPinEdge, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(edge) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGpioPinProviderValueChangedEventArgsFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait IGpioProviderImpl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>>;
}
impl ::windows::core::RuntimeName for IGpioProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioProvider";
}
impl IGpioProviderVtbl {
    pub const fn new<Impl: IGpioProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGpioProviderVtbl {
        unsafe extern "system" fn GetControllers<Impl: IGpioProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGpioProvider>, base.5, GetControllers::<Impl, OFFSET>)
    }
}
