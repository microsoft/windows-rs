#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdcChannelImpl: Sized + IClosableImpl {
    fn Controller(&self) -> ::windows::core::Result<AdcController>;
    fn ReadValue(&self) -> ::windows::core::Result<i32>;
    fn ReadRatio(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdcChannel {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcChannel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdcChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcChannelVtbl {
        unsafe extern "system" fn Controller<Impl: IAdcChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadValue<Impl: IAdcChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadRatio<Impl: IAdcChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdcChannel>, ::windows::core::GetTrustLevel, Controller::<Impl, IMPL_OFFSET>, ReadValue::<Impl, IMPL_OFFSET>, ReadRatio::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdcControllerImpl: Sized {
    fn ChannelCount(&self) -> ::windows::core::Result<i32>;
    fn ResolutionInBits(&self) -> ::windows::core::Result<i32>;
    fn MinValue(&self) -> ::windows::core::Result<i32>;
    fn MaxValue(&self) -> ::windows::core::Result<i32>;
    fn ChannelMode(&self) -> ::windows::core::Result<AdcChannelMode>;
    fn SetChannelMode(&self, value: AdcChannelMode) -> ::windows::core::Result<()>;
    fn IsChannelModeSupported(&self, channelmode: AdcChannelMode) -> ::windows::core::Result<bool>;
    fn OpenChannel(&self, channelnumber: i32) -> ::windows::core::Result<AdcChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdcController {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcController";
}
#[cfg(feature = "implement_exclusive")]
impl IAdcControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcControllerVtbl {
        unsafe extern "system" fn ChannelCount<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResolutionInBits<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinValue<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxValue<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelMode<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdcChannelMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetChannelMode<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AdcChannelMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelMode(value).into()
        }
        unsafe extern "system" fn IsChannelModeSupported<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelmode: AdcChannelMode, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenChannel<Impl: IAdcControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdcController>,
            ::windows::core::GetTrustLevel,
            ChannelCount::<Impl, IMPL_OFFSET>,
            ResolutionInBits::<Impl, IMPL_OFFSET>,
            MinValue::<Impl, IMPL_OFFSET>,
            MaxValue::<Impl, IMPL_OFFSET>,
            ChannelMode::<Impl, IMPL_OFFSET>,
            SetChannelMode::<Impl, IMPL_OFFSET>,
            IsChannelModeSupported::<Impl, IMPL_OFFSET>,
            OpenChannel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAdcControllerStaticsImpl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::IAdcProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AdcController>>>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdcControllerStatics {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcControllerStatics";
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAdcControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcControllerStaticsVtbl {
        unsafe extern "system" fn GetControllersAsync<Impl: IAdcControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdcControllerStatics>, ::windows::core::GetTrustLevel, GetControllersAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdcControllerStatics2Impl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdcController>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdcControllerStatics2 {
    const NAME: &'static str = "Windows.Devices.Adc.IAdcControllerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdcControllerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdcControllerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdcControllerStatics2Vtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IAdcControllerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdcControllerStatics2>, ::windows::core::GetTrustLevel, GetDefaultAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdcControllerStatics2 as ::windows::core::Interface>::IID
    }
}
