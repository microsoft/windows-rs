pub trait IAdcControllerProvider_Impl: Sized {
    fn ChannelCount(&self) -> ::windows::core::Result<i32>;
    fn ResolutionInBits(&self) -> ::windows::core::Result<i32>;
    fn MinValue(&self) -> ::windows::core::Result<i32>;
    fn MaxValue(&self) -> ::windows::core::Result<i32>;
    fn ChannelMode(&self) -> ::windows::core::Result<ProviderAdcChannelMode>;
    fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> ::windows::core::Result<()>;
    fn IsChannelModeSupported(&self, channelmode: ProviderAdcChannelMode) -> ::windows::core::Result<bool>;
    fn AcquireChannel(&self, channel: i32) -> ::windows::core::Result<()>;
    fn ReleaseChannel(&self, channel: i32) -> ::windows::core::Result<()>;
    fn ReadValue(&self, channelnumber: i32) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IAdcControllerProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcControllerProvider";
}
impl IAdcControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>() -> IAdcControllerProvider_Vtbl {
        unsafe extern "system" fn ChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolutionInBits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResolutionInBits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderAdcChannelMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChannelMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderAdcChannelMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannelMode(value).into()
        }
        unsafe extern "system" fn IsChannelModeSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsChannelModeSupported(channelmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireChannel(channel).into()
        }
        unsafe extern "system" fn ReleaseChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseChannel(channel).into()
        }
        unsafe extern "system" fn ReadValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadValue(channelnumber) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IAdcControllerProvider, OFFSET>(),
            ChannelCount: ChannelCount::<Identity, Impl, OFFSET>,
            ResolutionInBits: ResolutionInBits::<Identity, Impl, OFFSET>,
            MinValue: MinValue::<Identity, Impl, OFFSET>,
            MaxValue: MaxValue::<Identity, Impl, OFFSET>,
            ChannelMode: ChannelMode::<Identity, Impl, OFFSET>,
            SetChannelMode: SetChannelMode::<Identity, Impl, OFFSET>,
            IsChannelModeSupported: IsChannelModeSupported::<Identity, Impl, OFFSET>,
            AcquireChannel: AcquireChannel::<Identity, Impl, OFFSET>,
            ReleaseChannel: ReleaseChannel::<Identity, Impl, OFFSET>,
            ReadValue: ReadValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IAdcProvider_Impl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAdcProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IAdcProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcProvider_Impl, const OFFSET: isize>() -> IAdcProvider_Vtbl {
        unsafe extern "system" fn GetControllers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdcProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IAdcProvider, OFFSET>(), GetControllers: GetControllers::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcProvider as ::windows::core::Interface>::IID
    }
}
