#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdcChannel_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn Controller(&mut self) -> ::windows::core::Result<AdcController>;
    fn ReadValue(&mut self) -> ::windows::core::Result<i32>;
    fn ReadRatio(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdcChannel {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcChannel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdcChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcChannel_Vtbl {
        unsafe extern "system" fn Controller<Impl: IAdcChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadValue<Impl: IAdcChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadRatio<Impl: IAdcChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdcChannel, BASE_OFFSET>(),
            Controller: Controller::<Impl, IMPL_OFFSET>,
            ReadValue: ReadValue::<Impl, IMPL_OFFSET>,
            ReadRatio: ReadRatio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdcController_Impl: Sized {
    fn ChannelCount(&mut self) -> ::windows::core::Result<i32>;
    fn ResolutionInBits(&mut self) -> ::windows::core::Result<i32>;
    fn MinValue(&mut self) -> ::windows::core::Result<i32>;
    fn MaxValue(&mut self) -> ::windows::core::Result<i32>;
    fn ChannelMode(&mut self) -> ::windows::core::Result<AdcChannelMode>;
    fn SetChannelMode(&mut self, value: AdcChannelMode) -> ::windows::core::Result<()>;
    fn IsChannelModeSupported(&mut self, channelmode: AdcChannelMode) -> ::windows::core::Result<bool>;
    fn OpenChannel(&mut self, channelnumber: i32) -> ::windows::core::Result<AdcChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdcController {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcController";
}
#[cfg(feature = "implement_exclusive")]
impl IAdcController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcController_Vtbl {
        unsafe extern "system" fn ChannelCount<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResolutionInBits<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinValue<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxValue<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelMode<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdcChannelMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetChannelMode<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AdcChannelMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelMode(value).into()
        }
        unsafe extern "system" fn IsChannelModeSupported<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelmode: AdcChannelMode, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenChannel<Impl: IAdcController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenChannel(channelnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdcController, BASE_OFFSET>(),
            ChannelCount: ChannelCount::<Impl, IMPL_OFFSET>,
            ResolutionInBits: ResolutionInBits::<Impl, IMPL_OFFSET>,
            MinValue: MinValue::<Impl, IMPL_OFFSET>,
            MaxValue: MaxValue::<Impl, IMPL_OFFSET>,
            ChannelMode: ChannelMode::<Impl, IMPL_OFFSET>,
            SetChannelMode: SetChannelMode::<Impl, IMPL_OFFSET>,
            IsChannelModeSupported: IsChannelModeSupported::<Impl, IMPL_OFFSET>,
            OpenChannel: OpenChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAdcControllerStatics_Impl: Sized {
    fn GetControllersAsync(&mut self, provider: &::core::option::Option<Provider::IAdcProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AdcController>>>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdcControllerStatics {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcControllerStatics";
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAdcControllerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcControllerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcControllerStatics_Vtbl {
        unsafe extern "system" fn GetControllersAsync<Impl: IAdcControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllersAsync(&*(&provider as *const <Provider::IAdcProvider as ::windows::core::Abi>::Abi as *const <Provider::IAdcProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdcControllerStatics, BASE_OFFSET>(),
            GetControllersAsync: GetControllersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdcControllerStatics2_Impl: Sized {
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdcController>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdcControllerStatics2 {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcControllerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdcControllerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcControllerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcControllerStatics2_Vtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IAdcControllerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdcControllerStatics2, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcControllerStatics2 as ::windows::core::Interface>::IID
    }
}
