pub trait IGpioControllerProvider_Impl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn OpenPinProvider(&self, pin: i32, sharingmode: ProviderGpioSharingMode) -> ::windows::core::Result<IGpioPinProvider>;
}
impl ::windows::core::RuntimeName for IGpioControllerProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioControllerProvider";
}
impl IGpioControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: isize>() -> IGpioControllerProvider_Vtbl {
        unsafe extern "system" fn PinCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PinCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPinProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenPinProvider(pin, sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IGpioControllerProvider, OFFSET>(),
            PinCount: PinCount::<Identity, Impl, OFFSET>,
            OpenPinProvider: OpenPinProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IGpioPinProvider_Impl: Sized {
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
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IGpioPinProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProvider";
}
#[cfg(feature = "Foundation")]
impl IGpioPinProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>() -> IGpioPinProvider_Vtbl {
        unsafe extern "system" fn ValueChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValueChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveValueChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn DebounceTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DebounceTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDebounceTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebounceTimeout(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn PinNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PinNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SharingMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDriveModeSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDriveModeSupported(drivemode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDriveMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinDriveMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDriveMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDriveMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinDriveMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDriveMode(value).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinValue) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write(value).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinValue) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Read() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IGpioPinProvider, OFFSET>(),
            ValueChanged: ValueChanged::<Identity, Impl, OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Identity, Impl, OFFSET>,
            DebounceTimeout: DebounceTimeout::<Identity, Impl, OFFSET>,
            SetDebounceTimeout: SetDebounceTimeout::<Identity, Impl, OFFSET>,
            PinNumber: PinNumber::<Identity, Impl, OFFSET>,
            SharingMode: SharingMode::<Identity, Impl, OFFSET>,
            IsDriveModeSupported: IsDriveModeSupported::<Identity, Impl, OFFSET>,
            GetDriveMode: GetDriveMode::<Identity, Impl, OFFSET>,
            SetDriveMode: SetDriveMode::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioPinProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IGpioProvider_Impl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IGpioProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IGpioProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioProvider_Impl, const OFFSET: isize>() -> IGpioProvider_Vtbl {
        unsafe extern "system" fn GetControllers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGpioProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetControllers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IGpioProvider, OFFSET>(), GetControllers: GetControllers::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGpioProvider as ::windows::core::Interface>::IID
    }
}
