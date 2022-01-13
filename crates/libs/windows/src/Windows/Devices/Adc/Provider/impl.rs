pub trait IAdcControllerProviderImpl: Sized {
    fn ChannelCount(&mut self) -> ::windows::core::Result<i32>;
    fn ResolutionInBits(&mut self) -> ::windows::core::Result<i32>;
    fn MinValue(&mut self) -> ::windows::core::Result<i32>;
    fn MaxValue(&mut self) -> ::windows::core::Result<i32>;
    fn ChannelMode(&mut self) -> ::windows::core::Result<ProviderAdcChannelMode>;
    fn SetChannelMode(&mut self, value: ProviderAdcChannelMode) -> ::windows::core::Result<()>;
    fn IsChannelModeSupported(&mut self, channelmode: ProviderAdcChannelMode) -> ::windows::core::Result<bool>;
    fn AcquireChannel(&mut self, channel: i32) -> ::windows::core::Result<()>;
    fn ReleaseChannel(&mut self, channel: i32) -> ::windows::core::Result<()>;
    fn ReadValue(&mut self, channelnumber: i32) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IAdcControllerProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcControllerProvider";
}
impl IAdcControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcControllerProviderVtbl {
        unsafe extern "system" fn ChannelCount<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolutionInBits<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolutionInBits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinValue<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValue<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelMode<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderAdcChannelMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelMode<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProviderAdcChannelMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelMode(value).into()
        }
        unsafe extern "system" fn IsChannelModeSupported<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsChannelModeSupported(channelmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireChannel<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireChannel(channel).into()
        }
        unsafe extern "system" fn ReleaseChannel<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseChannel(channel).into()
        }
        unsafe extern "system" fn ReadValue<Impl: IAdcControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadValue(channelnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdcControllerProvider, BASE_OFFSET>(),
            ChannelCount: ChannelCount::<Impl, IMPL_OFFSET>,
            ResolutionInBits: ResolutionInBits::<Impl, IMPL_OFFSET>,
            MinValue: MinValue::<Impl, IMPL_OFFSET>,
            MaxValue: MaxValue::<Impl, IMPL_OFFSET>,
            ChannelMode: ChannelMode::<Impl, IMPL_OFFSET>,
            SetChannelMode: SetChannelMode::<Impl, IMPL_OFFSET>,
            IsChannelModeSupported: IsChannelModeSupported::<Impl, IMPL_OFFSET>,
            AcquireChannel: AcquireChannel::<Impl, IMPL_OFFSET>,
            ReleaseChannel: ReleaseChannel::<Impl, IMPL_OFFSET>,
            ReadValue: ReadValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IAdcProviderImpl: Sized {
    fn GetControllers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAdcProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IAdcProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcProviderVtbl {
        unsafe extern "system" fn GetControllers<Impl: IAdcProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAdcProvider, BASE_OFFSET>(), GetControllers: GetControllers::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcProvider as ::windows::core::Interface>::IID
    }
}
