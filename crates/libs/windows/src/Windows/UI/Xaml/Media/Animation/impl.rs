#[cfg(feature = "implement_exclusive")]
pub trait IAddDeleteThemeTransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAddDeleteThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IAddDeleteThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IAddDeleteThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddDeleteThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAddDeleteThemeTransitionVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAddDeleteThemeTransition>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddDeleteThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEaseImpl: Sized {
    fn Amplitude(&self) -> ::windows::core::Result<f64>;
    fn SetAmplitude(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBackEase";
}
#[cfg(feature = "implement_exclusive")]
impl IBackEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackEaseVtbl {
        unsafe extern "system" fn Amplitude<Impl: IBackEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Amplitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmplitude<Impl: IBackEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAmplitude(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackEase>, ::windows::core::GetTrustLevel, Amplitude::<Impl, IMPL_OFFSET>, SetAmplitude::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEaseStaticsImpl: Sized {
    fn AmplitudeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBackEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackEaseStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackEaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackEaseStaticsVtbl {
        unsafe extern "system" fn AmplitudeProperty<Impl: IBackEaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AmplitudeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackEaseStatics>, ::windows::core::GetTrustLevel, AmplitudeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBasicConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBasicConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBasicConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IBasicConnectedAnimationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicConnectedAnimationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicConnectedAnimationConfigurationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBasicConnectedAnimationConfiguration>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBasicConnectedAnimationConfigurationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BasicConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBasicConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBasicConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBasicConnectedAnimationConfigurationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicConnectedAnimationConfigurationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicConnectedAnimationConfigurationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBasicConnectedAnimationConfigurationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBasicConnectedAnimationConfigurationFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBeginStoryboardImpl: Sized {
    fn Storyboard(&self) -> ::windows::core::Result<Storyboard>;
    fn SetStoryboard(&self, value: &::core::option::Option<Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBeginStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBeginStoryboard";
}
#[cfg(feature = "implement_exclusive")]
impl IBeginStoryboardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBeginStoryboardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBeginStoryboardVtbl {
        unsafe extern "system" fn Storyboard<Impl: IBeginStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Storyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboard<Impl: IBeginStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryboard(&*(&value as *const <Storyboard as ::windows::core::Abi>::Abi as *const <Storyboard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBeginStoryboard>, ::windows::core::GetTrustLevel, Storyboard::<Impl, IMPL_OFFSET>, SetStoryboard::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBeginStoryboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBeginStoryboardStaticsImpl: Sized {
    fn StoryboardProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBeginStoryboardStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBeginStoryboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBeginStoryboardStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBeginStoryboardStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBeginStoryboardStaticsVtbl {
        unsafe extern "system" fn StoryboardProperty<Impl: IBeginStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoryboardProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBeginStoryboardStatics>, ::windows::core::GetTrustLevel, StoryboardProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBeginStoryboardStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEaseImpl: Sized {
    fn Bounces(&self) -> ::windows::core::Result<i32>;
    fn SetBounces(&self, value: i32) -> ::windows::core::Result<()>;
    fn Bounciness(&self) -> ::windows::core::Result<f64>;
    fn SetBounciness(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBounceEase";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceEaseVtbl {
        unsafe extern "system" fn Bounces<Impl: IBounceEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounces<Impl: IBounceEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounces(value).into()
        }
        unsafe extern "system" fn Bounciness<Impl: IBounceEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounciness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounciness<Impl: IBounceEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounciness(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBounceEase>, ::windows::core::GetTrustLevel, Bounces::<Impl, IMPL_OFFSET>, SetBounces::<Impl, IMPL_OFFSET>, Bounciness::<Impl, IMPL_OFFSET>, SetBounciness::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEaseStaticsImpl: Sized {
    fn BouncesProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BouncinessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBounceEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceEaseStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceEaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceEaseStaticsVtbl {
        unsafe extern "system" fn BouncesProperty<Impl: IBounceEaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BouncesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BouncinessProperty<Impl: IBounceEaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BouncinessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBounceEaseStatics>, ::windows::core::GetTrustLevel, BouncesProperty::<Impl, IMPL_OFFSET>, BouncinessProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICircleEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICircleEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICircleEase";
}
#[cfg(feature = "implement_exclusive")]
impl ICircleEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICircleEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICircleEaseVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICircleEase>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICircleEase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IColorAnimationImpl: Sized {
    fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetFrom(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetTo(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetBy(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IColorAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimationVtbl {
        unsafe extern "system" fn From<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).By() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBy<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IColorAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IColorAnimation>,
            ::windows::core::GetTrustLevel,
            From::<Impl, IMPL_OFFSET>,
            SetFrom::<Impl, IMPL_OFFSET>,
            To::<Impl, IMPL_OFFSET>,
            SetTo::<Impl, IMPL_OFFSET>,
            By::<Impl, IMPL_OFFSET>,
            SetBy::<Impl, IMPL_OFFSET>,
            EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationStaticsImpl: Sized {
    fn FromProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimationStaticsVtbl {
        unsafe extern "system" fn FromProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ByProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EasingFunctionProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorAnimationStatics>, ::windows::core::GetTrustLevel, FromProperty::<Impl, IMPL_OFFSET>, ToProperty::<Impl, IMPL_OFFSET>, ByProperty::<Impl, IMPL_OFFSET>, EasingFunctionProperty::<Impl, IMPL_OFFSET>, EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IColorAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<ColorKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IColorAnimationUsingKeyFramesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimationUsingKeyFramesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IColorAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IColorAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IColorAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorAnimationUsingKeyFrames>, ::windows::core::GetTrustLevel, KeyFrames::<Impl, IMPL_OFFSET>, EnableDependentAnimation::<Impl, IMPL_OFFSET>, SetEnableDependentAnimation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorAnimationUsingKeyFramesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimationUsingKeyFramesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IColorAnimationUsingKeyFramesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorAnimationUsingKeyFramesStatics>, ::windows::core::GetTrustLevel, EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IColorKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetValue(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IColorKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorKeyFrame>, ::windows::core::GetTrustLevel, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, KeyTime::<Impl, IMPL_OFFSET>, SetKeyTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorKeyFrameFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorKeyFrameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorKeyFrameFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IColorKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IColorKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorKeyFrameStatics>, ::windows::core::GetTrustLevel, ValueProperty::<Impl, IMPL_OFFSET>, KeyTimeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommonNavigationTransitionInfoImpl: Sized {
    fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommonNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICommonNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICommonNavigationTransitionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonNavigationTransitionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonNavigationTransitionInfoVtbl {
        unsafe extern "system" fn IsStaggeringEnabled<Impl: ICommonNavigationTransitionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: ICommonNavigationTransitionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICommonNavigationTransitionInfo>, ::windows::core::GetTrustLevel, IsStaggeringEnabled::<Impl, IMPL_OFFSET>, SetIsStaggeringEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommonNavigationTransitionInfoStaticsImpl: Sized {
    fn IsStaggeringEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStaggerElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsStaggerElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsStaggerElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommonNavigationTransitionInfoStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICommonNavigationTransitionInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICommonNavigationTransitionInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonNavigationTransitionInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonNavigationTransitionInfoStaticsVtbl {
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStaggerElementProperty<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaggerElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsStaggerElement<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsStaggerElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggerElement<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggerElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICommonNavigationTransitionInfoStatics>, ::windows::core::GetTrustLevel, IsStaggeringEnabledProperty::<Impl, IMPL_OFFSET>, IsStaggerElementProperty::<Impl, IMPL_OFFSET>, GetIsStaggerElement::<Impl, IMPL_OFFSET>, SetIsStaggerElement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonNavigationTransitionInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectedAnimationImpl: Sized {
    fn Completed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryStart(&self, destination: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectedAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectedAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationVtbl {
        unsafe extern "system" fn Completed<Impl: IConnectedAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IConnectedAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryStart<Impl: IConnectedAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStart(&*(&destination as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IConnectedAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectedAnimation>, ::windows::core::GetTrustLevel, Completed::<Impl, IMPL_OFFSET>, RemoveCompleted::<Impl, IMPL_OFFSET>, TryStart::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IConnectedAnimation2Impl: Sized {
    fn IsScaleAnimationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsScaleAnimationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryStartWithCoordinatedElements(&self, destination: &::core::option::Option<super::super::UIElement>, coordinatedelements: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement>>) -> ::windows::core::Result<bool>;
    fn SetAnimationComponent(&self, component: ConnectedAnimationComponent, animation: &::core::option::Option<super::super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectedAnimation2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IConnectedAnimation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimation2Vtbl {
        unsafe extern "system" fn IsScaleAnimationEnabled<Impl: IConnectedAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScaleAnimationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsScaleAnimationEnabled<Impl: IConnectedAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsScaleAnimationEnabled(value).into()
        }
        unsafe extern "system" fn TryStartWithCoordinatedElements<Impl: IConnectedAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, coordinatedelements: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStartWithCoordinatedElements(&*(&destination as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&coordinatedelements as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationComponent<Impl: IConnectedAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, component: ConnectedAnimationComponent, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimationComponent(component, &*(&animation as *const <super::super::super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectedAnimation2>, ::windows::core::GetTrustLevel, IsScaleAnimationEnabled::<Impl, IMPL_OFFSET>, SetIsScaleAnimationEnabled::<Impl, IMPL_OFFSET>, TryStartWithCoordinatedElements::<Impl, IMPL_OFFSET>, SetAnimationComponent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimation3Impl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<ConnectedAnimationConfiguration>;
    fn SetConfiguration(&self, value: &::core::option::Option<ConnectedAnimationConfiguration>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimation3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation3";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimation3Vtbl {
        unsafe extern "system" fn Configuration<Impl: IConnectedAnimation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Impl: IConnectedAnimation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfiguration(&*(&value as *const <ConnectedAnimationConfiguration as ::windows::core::Abi>::Abi as *const <ConnectedAnimationConfiguration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectedAnimation3>, ::windows::core::GetTrustLevel, Configuration::<Impl, IMPL_OFFSET>, SetConfiguration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationConfigurationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectedAnimationConfiguration>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationConfigurationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationConfigurationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationConfigurationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationConfigurationFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectedAnimationConfigurationFactory>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IConnectedAnimationServiceImpl: Sized {
    fn DefaultDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SetDefaultDuration(&self, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DefaultEasingFunction(&self) -> ::windows::core::Result<super::super::super::Composition::CompositionEasingFunction>;
    fn SetDefaultEasingFunction(&self, value: &::core::option::Option<super::super::super::Composition::CompositionEasingFunction>) -> ::windows::core::Result<()>;
    fn PrepareToAnimate(&self, key: &::windows::core::HSTRING, source: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<ConnectedAnimation>;
    fn GetAnimation(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<ConnectedAnimation>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectedAnimationService {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationService";
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IConnectedAnimationServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationServiceVtbl {
        unsafe extern "system" fn DefaultDuration<Impl: IConnectedAnimationServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDuration<Impl: IConnectedAnimationServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultDuration(&*(&value as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultEasingFunction<Impl: IConnectedAnimationServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultEasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultEasingFunction<Impl: IConnectedAnimationServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultEasingFunction(&*(&value as *const <super::super::super::Composition::CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrepareToAnimate<Impl: IConnectedAnimationServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareToAnimate(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&source as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnimation<Impl: IConnectedAnimationServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnimation(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IConnectedAnimationService>,
            ::windows::core::GetTrustLevel,
            DefaultDuration::<Impl, IMPL_OFFSET>,
            SetDefaultDuration::<Impl, IMPL_OFFSET>,
            DefaultEasingFunction::<Impl, IMPL_OFFSET>,
            SetDefaultEasingFunction::<Impl, IMPL_OFFSET>,
            PrepareToAnimate::<Impl, IMPL_OFFSET>,
            GetAnimation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationServiceStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ConnectedAnimationService>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimationServiceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationServiceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationServiceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationServiceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationServiceStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IConnectedAnimationServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectedAnimationServiceStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationServiceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentThemeTransitionImpl: Sized {
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContentThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IContentThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentThemeTransitionVtbl {
        unsafe extern "system" fn HorizontalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentThemeTransition>, ::windows::core::GetTrustLevel, HorizontalOffset::<Impl, IMPL_OFFSET>, SetHorizontalOffset::<Impl, IMPL_OFFSET>, VerticalOffset::<Impl, IMPL_OFFSET>, SetVerticalOffset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentThemeTransitionStaticsImpl: Sized {
    fn HorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContentThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContentThemeTransitionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentThemeTransitionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentThemeTransitionStaticsVtbl {
        unsafe extern "system" fn HorizontalOffsetProperty<Impl: IContentThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffsetProperty<Impl: IContentThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentThemeTransitionStatics>, ::windows::core::GetTrustLevel, HorizontalOffsetProperty::<Impl, IMPL_OFFSET>, VerticalOffsetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContinuumNavigationTransitionInfoImpl: Sized {
    fn ExitElement(&self) -> ::windows::core::Result<super::super::UIElement>;
    fn SetExitElement(&self, value: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContinuumNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContinuumNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IContinuumNavigationTransitionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinuumNavigationTransitionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinuumNavigationTransitionInfoVtbl {
        unsafe extern "system" fn ExitElement<Impl: IContinuumNavigationTransitionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitElement<Impl: IContinuumNavigationTransitionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitElement(&*(&value as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContinuumNavigationTransitionInfo>, ::windows::core::GetTrustLevel, ExitElement::<Impl, IMPL_OFFSET>, SetExitElement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinuumNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IContinuumNavigationTransitionInfoStaticsImpl: Sized {
    fn ExitElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsEntranceElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsEntranceElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsEntranceElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn IsExitElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsExitElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsExitElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn ExitElementContainerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetExitElementContainer(&self, element: &::core::option::Option<super::super::Controls::ListViewBase>) -> ::windows::core::Result<bool>;
    fn SetExitElementContainer(&self, element: &::core::option::Option<super::super::Controls::ListViewBase>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContinuumNavigationTransitionInfoStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContinuumNavigationTransitionInfoStatics";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IContinuumNavigationTransitionInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinuumNavigationTransitionInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinuumNavigationTransitionInfoStaticsVtbl {
        unsafe extern "system" fn ExitElementProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEntranceElementProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEntranceElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsEntranceElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsEntranceElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEntranceElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEntranceElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn IsExitElementProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsExitElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsExitElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsExitElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsExitElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsExitElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ExitElementContainerProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitElementContainerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExitElementContainer<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExitElementContainer(&*(&element as *const <super::super::Controls::ListViewBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBase as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitElementContainer<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitElementContainer(&*(&element as *const <super::super::Controls::ListViewBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBase as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContinuumNavigationTransitionInfoStatics>,
            ::windows::core::GetTrustLevel,
            ExitElementProperty::<Impl, IMPL_OFFSET>,
            IsEntranceElementProperty::<Impl, IMPL_OFFSET>,
            GetIsEntranceElement::<Impl, IMPL_OFFSET>,
            SetIsEntranceElement::<Impl, IMPL_OFFSET>,
            IsExitElementProperty::<Impl, IMPL_OFFSET>,
            GetIsExitElement::<Impl, IMPL_OFFSET>,
            SetIsExitElement::<Impl, IMPL_OFFSET>,
            ExitElementContainerProperty::<Impl, IMPL_OFFSET>,
            GetExitElementContainer::<Impl, IMPL_OFFSET>,
            SetExitElementContainer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinuumNavigationTransitionInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICubicEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICubicEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICubicEase";
}
#[cfg(feature = "implement_exclusive")]
impl ICubicEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICubicEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICubicEaseVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICubicEase>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICubicEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirectConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirectConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDirectConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IDirectConnectedAnimationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectConnectedAnimationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectConnectedAnimationConfigurationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectConnectedAnimationConfiguration>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirectConnectedAnimationConfigurationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DirectConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirectConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDirectConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDirectConnectedAnimationConfigurationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectConnectedAnimationConfigurationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectConnectedAnimationConfigurationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDirectConnectedAnimationConfigurationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectConnectedAnimationConfigurationFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteColorKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscreteColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscreteColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscreteColorKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscreteColorKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscreteColorKeyFrameVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscreteColorKeyFrame>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscreteColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteDoubleKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscreteDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscreteDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscreteDoubleKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscreteDoubleKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscreteDoubleKeyFrameVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscreteDoubleKeyFrame>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscreteDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteObjectKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscreteObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscreteObjectKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscreteObjectKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscreteObjectKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscreteObjectKeyFrameVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscreteObjectKeyFrame>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscreteObjectKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscretePointKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscretePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscretePointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscretePointKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscretePointKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscretePointKeyFrameVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscretePointKeyFrame>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscretePointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDoubleAnimationImpl: Sized {
    fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetFrom(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetTo(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetBy(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDoubleAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDoubleAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimationVtbl {
        unsafe extern "system" fn From<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).By() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBy<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IDoubleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDoubleAnimation>,
            ::windows::core::GetTrustLevel,
            From::<Impl, IMPL_OFFSET>,
            SetFrom::<Impl, IMPL_OFFSET>,
            To::<Impl, IMPL_OFFSET>,
            SetTo::<Impl, IMPL_OFFSET>,
            By::<Impl, IMPL_OFFSET>,
            SetBy::<Impl, IMPL_OFFSET>,
            EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationStaticsImpl: Sized {
    fn FromProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimationStaticsVtbl {
        unsafe extern "system" fn FromProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ByProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EasingFunctionProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
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
            ::windows::core::GetRuntimeClassName::<IDoubleAnimationStatics>,
            ::windows::core::GetTrustLevel,
            FromProperty::<Impl, IMPL_OFFSET>,
            ToProperty::<Impl, IMPL_OFFSET>,
            ByProperty::<Impl, IMPL_OFFSET>,
            EasingFunctionProperty::<Impl, IMPL_OFFSET>,
            EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDoubleAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<DoubleKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDoubleAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDoubleAnimationUsingKeyFramesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimationUsingKeyFramesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IDoubleAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IDoubleAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IDoubleAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDoubleAnimationUsingKeyFrames>, ::windows::core::GetTrustLevel, KeyFrames::<Impl, IMPL_OFFSET>, EnableDependentAnimation::<Impl, IMPL_OFFSET>, SetEnableDependentAnimation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleAnimationUsingKeyFramesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimationUsingKeyFramesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IDoubleAnimationUsingKeyFramesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDoubleAnimationUsingKeyFramesStatics>, ::windows::core::GetTrustLevel, EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDoubleKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDoubleKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDoubleKeyFrame>, ::windows::core::GetTrustLevel, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, KeyTime::<Impl, IMPL_OFFSET>, SetKeyTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DoubleKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleKeyFrameFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleKeyFrameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDoubleKeyFrameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDoubleKeyFrameFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IDoubleKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IDoubleKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDoubleKeyFrameStatics>, ::windows::core::GetTrustLevel, ValueProperty::<Impl, IMPL_OFFSET>, KeyTimeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragItemThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDragItemThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragItemThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragItemThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IDragItemThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IDragItemThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDragItemThemeAnimation>, ::windows::core::GetTrustLevel, TargetName::<Impl, IMPL_OFFSET>, SetTargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragItemThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragItemThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragItemThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDragItemThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragItemThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragItemThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDragItemThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDragItemThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragItemThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IDragOverThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ToOffset(&self) -> ::windows::core::Result<f64>;
    fn SetToOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragOverThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragOverThemeAnimation";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IDragOverThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragOverThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragOverThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IDragOverThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IDragOverThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ToOffset<Impl: IDragOverThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToOffset<Impl: IDragOverThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToOffset(value).into()
        }
        unsafe extern "system" fn Direction<Impl: IDragOverThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Impl: IDragOverThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDragOverThemeAnimation>,
            ::windows::core::GetTrustLevel,
            TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            ToOffset::<Impl, IMPL_OFFSET>,
            SetToOffset::<Impl, IMPL_OFFSET>,
            Direction::<Impl, IMPL_OFFSET>,
            SetDirection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragOverThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOverThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DirectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragOverThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragOverThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDragOverThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragOverThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragOverThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDragOverThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToOffsetProperty<Impl: IDragOverThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionProperty<Impl: IDragOverThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDragOverThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>, ToOffsetProperty::<Impl, IMPL_OFFSET>, DirectionProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragOverThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillInNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillInNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillInNavigationTransitionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillInNavigationTransitionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillInNavigationTransitionInfoVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDrillInNavigationTransitionInfo>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillInNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInThemeAnimationImpl: Sized {
    fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEntranceTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetEntranceTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetExitTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillInThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillInThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillInThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillInThemeAnimationVtbl {
        unsafe extern "system" fn EntranceTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EntranceTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDrillInThemeAnimation>,
            ::windows::core::GetTrustLevel,
            EntranceTargetName::<Impl, IMPL_OFFSET>,
            SetEntranceTargetName::<Impl, IMPL_OFFSET>,
            EntranceTarget::<Impl, IMPL_OFFSET>,
            SetEntranceTarget::<Impl, IMPL_OFFSET>,
            ExitTargetName::<Impl, IMPL_OFFSET>,
            SetExitTargetName::<Impl, IMPL_OFFSET>,
            ExitTarget::<Impl, IMPL_OFFSET>,
            SetExitTarget::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillInThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInThemeAnimationStaticsImpl: Sized {
    fn EntranceTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EntranceTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillInThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillInThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillInThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillInThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillInThemeAnimationStaticsVtbl {
        unsafe extern "system" fn EntranceTargetNameProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntranceTargetProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetNameProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDrillInThemeAnimationStatics>, ::windows::core::GetTrustLevel, EntranceTargetNameProperty::<Impl, IMPL_OFFSET>, EntranceTargetProperty::<Impl, IMPL_OFFSET>, ExitTargetNameProperty::<Impl, IMPL_OFFSET>, ExitTargetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillInThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillOutThemeAnimationImpl: Sized {
    fn EntranceTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEntranceTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EntranceTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetEntranceTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExitTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetExitTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillOutThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillOutThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillOutThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillOutThemeAnimationVtbl {
        unsafe extern "system" fn EntranceTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EntranceTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDrillOutThemeAnimation>,
            ::windows::core::GetTrustLevel,
            EntranceTargetName::<Impl, IMPL_OFFSET>,
            SetEntranceTargetName::<Impl, IMPL_OFFSET>,
            EntranceTarget::<Impl, IMPL_OFFSET>,
            SetEntranceTarget::<Impl, IMPL_OFFSET>,
            ExitTargetName::<Impl, IMPL_OFFSET>,
            SetExitTargetName::<Impl, IMPL_OFFSET>,
            ExitTarget::<Impl, IMPL_OFFSET>,
            SetExitTarget::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillOutThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillOutThemeAnimationStaticsImpl: Sized {
    fn EntranceTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EntranceTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillOutThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillOutThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillOutThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillOutThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillOutThemeAnimationStaticsVtbl {
        unsafe extern "system" fn EntranceTargetNameProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntranceTargetProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntranceTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetNameProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDrillOutThemeAnimationStatics>, ::windows::core::GetTrustLevel, EntranceTargetNameProperty::<Impl, IMPL_OFFSET>, EntranceTargetProperty::<Impl, IMPL_OFFSET>, ExitTargetNameProperty::<Impl, IMPL_OFFSET>, ExitTargetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillOutThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetItemThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDropTargetItemThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetItemThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetItemThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetItemThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IDropTargetItemThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IDropTargetItemThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropTargetItemThemeAnimation>, ::windows::core::GetTrustLevel, TargetName::<Impl, IMPL_OFFSET>, SetTargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetItemThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetItemThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetItemThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDropTargetItemThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetItemThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetItemThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetItemThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDropTargetItemThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropTargetItemThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetItemThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingColorKeyFrameImpl: Sized {
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingColorKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingColorKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingColorKeyFrameVtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingColorKeyFrame>, ::windows::core::GetTrustLevel, EasingFunction::<Impl, IMPL_OFFSET>, SetEasingFunction::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingColorKeyFrameStaticsImpl: Sized {
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingColorKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingColorKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingColorKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingColorKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingColorKeyFrameStaticsVtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingColorKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingColorKeyFrameStatics>, ::windows::core::GetTrustLevel, EasingFunctionProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingColorKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingDoubleKeyFrameImpl: Sized {
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingDoubleKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingDoubleKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingDoubleKeyFrameVtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingDoubleKeyFrame>, ::windows::core::GetTrustLevel, EasingFunction::<Impl, IMPL_OFFSET>, SetEasingFunction::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingDoubleKeyFrameStaticsImpl: Sized {
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingDoubleKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingDoubleKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingDoubleKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingDoubleKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingDoubleKeyFrameStaticsVtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingDoubleKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingDoubleKeyFrameStatics>, ::windows::core::GetTrustLevel, EasingFunctionProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingDoubleKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseImpl: Sized {
    fn EasingMode(&self) -> ::windows::core::Result<EasingMode>;
    fn SetEasingMode(&self, value: EasingMode) -> ::windows::core::Result<()>;
    fn Ease(&self, normalizedtime: f64) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingFunctionBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingFunctionBase";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingFunctionBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingFunctionBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingFunctionBaseVtbl {
        unsafe extern "system" fn EasingMode<Impl: IEasingFunctionBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingMode<Impl: IEasingFunctionBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EasingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingMode(value).into()
        }
        unsafe extern "system" fn Ease<Impl: IEasingFunctionBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedtime: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ease(normalizedtime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingFunctionBase>, ::windows::core::GetTrustLevel, EasingMode::<Impl, IMPL_OFFSET>, SetEasingMode::<Impl, IMPL_OFFSET>, Ease::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingFunctionBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingFunctionBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingFunctionBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingFunctionBaseFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingFunctionBaseFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingFunctionBaseFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingFunctionBaseFactory>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingFunctionBaseFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseStaticsImpl: Sized {
    fn EasingModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingFunctionBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingFunctionBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingFunctionBaseStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingFunctionBaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingFunctionBaseStaticsVtbl {
        unsafe extern "system" fn EasingModeProperty<Impl: IEasingFunctionBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingFunctionBaseStatics>, ::windows::core::GetTrustLevel, EasingModeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingFunctionBaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingPointKeyFrameImpl: Sized {
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingPointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingPointKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingPointKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingPointKeyFrameVtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingPointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingPointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingPointKeyFrame>, ::windows::core::GetTrustLevel, EasingFunction::<Impl, IMPL_OFFSET>, SetEasingFunction::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingPointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingPointKeyFrameStaticsImpl: Sized {
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingPointKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingPointKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingPointKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingPointKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingPointKeyFrameStaticsVtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingPointKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEasingPointKeyFrameStatics>, ::windows::core::GetTrustLevel, EasingFunctionProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingPointKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IEdgeUIThemeTransitionImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEdgeUIThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEdgeUIThemeTransition";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IEdgeUIThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEdgeUIThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEdgeUIThemeTransitionVtbl {
        unsafe extern "system" fn Edge<Impl: IEdgeUIThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEdge<Impl: IEdgeUIThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEdge(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEdgeUIThemeTransition>, ::windows::core::GetTrustLevel, Edge::<Impl, IMPL_OFFSET>, SetEdge::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEdgeUIThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeUIThemeTransitionStaticsImpl: Sized {
    fn EdgeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeUIThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEdgeUIThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeUIThemeTransitionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEdgeUIThemeTransitionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEdgeUIThemeTransitionStaticsVtbl {
        unsafe extern "system" fn EdgeProperty<Impl: IEdgeUIThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EdgeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEdgeUIThemeTransitionStatics>, ::windows::core::GetTrustLevel, EdgeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEdgeUIThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEaseImpl: Sized {
    fn Oscillations(&self) -> ::windows::core::Result<i32>;
    fn SetOscillations(&self, value: i32) -> ::windows::core::Result<()>;
    fn Springiness(&self) -> ::windows::core::Result<f64>;
    fn SetSpringiness(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElasticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IElasticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IElasticEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElasticEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElasticEaseVtbl {
        unsafe extern "system" fn Oscillations<Impl: IElasticEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Oscillations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOscillations<Impl: IElasticEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOscillations(value).into()
        }
        unsafe extern "system" fn Springiness<Impl: IElasticEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Springiness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpringiness<Impl: IElasticEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpringiness(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IElasticEase>, ::windows::core::GetTrustLevel, Oscillations::<Impl, IMPL_OFFSET>, SetOscillations::<Impl, IMPL_OFFSET>, Springiness::<Impl, IMPL_OFFSET>, SetSpringiness::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElasticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEaseStaticsImpl: Sized {
    fn OscillationsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SpringinessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElasticEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IElasticEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IElasticEaseStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElasticEaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElasticEaseStaticsVtbl {
        unsafe extern "system" fn OscillationsProperty<Impl: IElasticEaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OscillationsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpringinessProperty<Impl: IElasticEaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpringinessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IElasticEaseStatics>, ::windows::core::GetTrustLevel, OscillationsProperty::<Impl, IMPL_OFFSET>, SpringinessProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElasticEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceNavigationTransitionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceNavigationTransitionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceNavigationTransitionInfoVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEntranceNavigationTransitionInfo>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceNavigationTransitionInfoStaticsImpl: Sized {
    fn IsTargetElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsTargetElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsTargetElement(&self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceNavigationTransitionInfoStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceNavigationTransitionInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceNavigationTransitionInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceNavigationTransitionInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceNavigationTransitionInfoStaticsVtbl {
        unsafe extern "system" fn IsTargetElementProperty<Impl: IEntranceNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTargetElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsTargetElement<Impl: IEntranceNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsTargetElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTargetElement<Impl: IEntranceNavigationTransitionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTargetElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEntranceNavigationTransitionInfoStatics>, ::windows::core::GetTrustLevel, IsTargetElementProperty::<Impl, IMPL_OFFSET>, GetIsTargetElement::<Impl, IMPL_OFFSET>, SetIsTargetElement::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceNavigationTransitionInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceThemeTransitionImpl: Sized {
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceThemeTransitionVtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        unsafe extern "system" fn IsStaggeringEnabled<Impl: IEntranceThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: IEntranceThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEntranceThemeTransition>,
            ::windows::core::GetTrustLevel,
            FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
            IsStaggeringEnabled::<Impl, IMPL_OFFSET>,
            SetIsStaggeringEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceThemeTransitionStaticsImpl: Sized {
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStaggeringEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceThemeTransitionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceThemeTransitionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceThemeTransitionStaticsVtbl {
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IEntranceThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IEntranceThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: IEntranceThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEntranceThemeTransitionStatics>, ::windows::core::GetTrustLevel, FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>, FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>, IsStaggeringEnabledProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEaseImpl: Sized {
    fn Exponent(&self) -> ::windows::core::Result<f64>;
    fn SetExponent(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExponentialEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IExponentialEase";
}
#[cfg(feature = "implement_exclusive")]
impl IExponentialEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExponentialEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExponentialEaseVtbl {
        unsafe extern "system" fn Exponent<Impl: IExponentialEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exponent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExponent<Impl: IExponentialEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExponent(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExponentialEase>, ::windows::core::GetTrustLevel, Exponent::<Impl, IMPL_OFFSET>, SetExponent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExponentialEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEaseStaticsImpl: Sized {
    fn ExponentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExponentialEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IExponentialEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IExponentialEaseStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExponentialEaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExponentialEaseStaticsVtbl {
        unsafe extern "system" fn ExponentProperty<Impl: IExponentialEaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExponentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExponentialEaseStatics>, ::windows::core::GetTrustLevel, ExponentProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExponentialEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeInThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeInThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeInThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeInThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeInThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IFadeInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IFadeInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFadeInThemeAnimation>, ::windows::core::GetTrustLevel, TargetName::<Impl, IMPL_OFFSET>, SetTargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeInThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeInThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeInThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeInThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeInThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeInThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeInThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IFadeInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFadeInThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeInThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeOutThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeOutThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeOutThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeOutThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeOutThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IFadeOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IFadeOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFadeOutThemeAnimation>, ::windows::core::GetTrustLevel, TargetName::<Impl, IMPL_OFFSET>, SetTargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeOutThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeOutThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeOutThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeOutThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeOutThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeOutThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeOutThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IFadeOutThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFadeOutThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeOutThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGravityConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IGravityConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IGravityConnectedAnimationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGravityConnectedAnimationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGravityConnectedAnimationConfigurationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGravityConnectedAnimationConfiguration>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGravityConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfiguration2Impl: Sized {
    fn IsShadowEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsShadowEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGravityConnectedAnimationConfiguration2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IGravityConnectedAnimationConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IGravityConnectedAnimationConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGravityConnectedAnimationConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGravityConnectedAnimationConfiguration2Vtbl {
        unsafe extern "system" fn IsShadowEnabled<Impl: IGravityConnectedAnimationConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShadowEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsShadowEnabled<Impl: IGravityConnectedAnimationConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsShadowEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGravityConnectedAnimationConfiguration2>, ::windows::core::GetTrustLevel, IsShadowEnabled::<Impl, IMPL_OFFSET>, SetIsShadowEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGravityConnectedAnimationConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfigurationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GravityConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGravityConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IGravityConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGravityConnectedAnimationConfigurationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGravityConnectedAnimationConfigurationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGravityConnectedAnimationConfigurationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGravityConnectedAnimationConfigurationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGravityConnectedAnimationConfigurationFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGravityConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeySplineImpl: Sized {
    fn ControlPoint1(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint1(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn ControlPoint2(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint2(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeySpline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeySpline";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeySplineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeySplineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeySplineVtbl {
        unsafe extern "system" fn ControlPoint1<Impl: IKeySplineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlPoint1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlPoint1<Impl: IKeySplineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlPoint1(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlPoint2<Impl: IKeySplineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlPoint2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlPoint2<Impl: IKeySplineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlPoint2(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKeySpline>, ::windows::core::GetTrustLevel, ControlPoint1::<Impl, IMPL_OFFSET>, SetControlPoint1::<Impl, IMPL_OFFSET>, ControlPoint2::<Impl, IMPL_OFFSET>, SetControlPoint2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeySpline as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyTimeHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyTimeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeyTimeHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyTimeHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyTimeHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyTimeHelperVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKeyTimeHelper>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyTimeHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeyTimeHelperStaticsImpl: Sized {
    fn FromTimeSpan(&self, timespan: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<KeyTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyTimeHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeyTimeHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeyTimeHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyTimeHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyTimeHelperStaticsVtbl {
        unsafe extern "system" fn FromTimeSpan<Impl: IKeyTimeHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timespan: super::super::super::super::Foundation::TimeSpan, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromTimeSpan(&*(&timespan as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKeyTimeHelperStatics>, ::windows::core::GetTrustLevel, FromTimeSpan::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyTimeHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearColorKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ILinearColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearColorKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearColorKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearColorKeyFrameVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILinearColorKeyFrame>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearDoubleKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ILinearDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearDoubleKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearDoubleKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearDoubleKeyFrameVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILinearDoubleKeyFrame>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearPointKeyFrameImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ILinearPointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearPointKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearPointKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearPointKeyFrameVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILinearPointKeyFrame>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearPointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationThemeTransitionImpl: Sized {
    fn DefaultNavigationTransitionInfo(&self) -> ::windows::core::Result<NavigationTransitionInfo>;
    fn SetDefaultNavigationTransitionInfo(&self, value: &::core::option::Option<NavigationTransitionInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationThemeTransitionVtbl {
        unsafe extern "system" fn DefaultNavigationTransitionInfo<Impl: INavigationThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultNavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultNavigationTransitionInfo<Impl: INavigationThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultNavigationTransitionInfo(&*(&value as *const <NavigationTransitionInfo as ::windows::core::Abi>::Abi as *const <NavigationTransitionInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INavigationThemeTransition>, ::windows::core::GetTrustLevel, DefaultNavigationTransitionInfo::<Impl, IMPL_OFFSET>, SetDefaultNavigationTransitionInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationThemeTransitionStaticsImpl: Sized {
    fn DefaultNavigationTransitionInfoProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationThemeTransitionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationThemeTransitionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationThemeTransitionStaticsVtbl {
        unsafe extern "system" fn DefaultNavigationTransitionInfoProperty<Impl: INavigationThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultNavigationTransitionInfoProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INavigationThemeTransitionStatics>, ::windows::core::GetTrustLevel, DefaultNavigationTransitionInfoProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationTransitionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationTransitionInfoVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INavigationTransitionInfo>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationTransitionInfoFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfoFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationTransitionInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationTransitionInfoFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INavigationTransitionInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INavigationTransitionInfoFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoOverridesImpl: Sized {
    fn GetNavigationStateCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNavigationStateCore(&self, navigationstate: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationTransitionInfoOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfoOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationTransitionInfoOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationTransitionInfoOverridesVtbl {
        unsafe extern "system" fn GetNavigationStateCore<Impl: INavigationTransitionInfoOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNavigationStateCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNavigationStateCore<Impl: INavigationTransitionInfoOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navigationstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNavigationStateCore(&*(&navigationstate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INavigationTransitionInfoOverrides>, ::windows::core::GetTrustLevel, GetNavigationStateCore::<Impl, IMPL_OFFSET>, SetNavigationStateCore::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfoOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IObjectAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<ObjectKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IObjectAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IObjectAnimationUsingKeyFramesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAnimationUsingKeyFramesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IObjectAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IObjectAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IObjectAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectAnimationUsingKeyFrames>, ::windows::core::GetTrustLevel, KeyFrames::<Impl, IMPL_OFFSET>, EnableDependentAnimation::<Impl, IMPL_OFFSET>, SetEnableDependentAnimation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectAnimationUsingKeyFramesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAnimationUsingKeyFramesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IObjectAnimationUsingKeyFramesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectAnimationUsingKeyFramesStatics>, ::windows::core::GetTrustLevel, EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IObjectKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IObjectKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IObjectKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IObjectKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IObjectKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IObjectKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectKeyFrame>, ::windows::core::GetTrustLevel, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, KeyTime::<Impl, IMPL_OFFSET>, SetKeyTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ObjectKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectKeyFrameFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectKeyFrameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IObjectKeyFrameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectKeyFrameFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IObjectKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IObjectKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectKeyFrameStatics>, ::windows::core::GetTrustLevel, ValueProperty::<Impl, IMPL_OFFSET>, KeyTimeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IPaneThemeTransitionImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaneThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPaneThemeTransition";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IPaneThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaneThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaneThemeTransitionVtbl {
        unsafe extern "system" fn Edge<Impl: IPaneThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEdge<Impl: IPaneThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEdge(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaneThemeTransition>, ::windows::core::GetTrustLevel, Edge::<Impl, IMPL_OFFSET>, SetEdge::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaneThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaneThemeTransitionStaticsImpl: Sized {
    fn EdgeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaneThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPaneThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPaneThemeTransitionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaneThemeTransitionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaneThemeTransitionStaticsVtbl {
        unsafe extern "system" fn EdgeProperty<Impl: IPaneThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EdgeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPaneThemeTransitionStatics>, ::windows::core::GetTrustLevel, EdgeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaneThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointAnimationImpl: Sized {
    fn From(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetFrom(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetTo(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn By(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetBy(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPointAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimationVtbl {
        unsafe extern "system" fn From<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).By() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBy<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IPointAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPointAnimation>,
            ::windows::core::GetTrustLevel,
            From::<Impl, IMPL_OFFSET>,
            SetFrom::<Impl, IMPL_OFFSET>,
            To::<Impl, IMPL_OFFSET>,
            SetTo::<Impl, IMPL_OFFSET>,
            By::<Impl, IMPL_OFFSET>,
            SetBy::<Impl, IMPL_OFFSET>,
            EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationStaticsImpl: Sized {
    fn FromProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimationStaticsVtbl {
        unsafe extern "system" fn FromProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ByProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EasingFunctionProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointAnimationStatics>, ::windows::core::GetTrustLevel, FromProperty::<Impl, IMPL_OFFSET>, ToProperty::<Impl, IMPL_OFFSET>, ByProperty::<Impl, IMPL_OFFSET>, EasingFunctionProperty::<Impl, IMPL_OFFSET>, EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPointAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<PointKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPointAnimationUsingKeyFramesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimationUsingKeyFramesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IPointAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IPointAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IPointAnimationUsingKeyFramesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointAnimationUsingKeyFrames>, ::windows::core::GetTrustLevel, KeyFrames::<Impl, IMPL_OFFSET>, EnableDependentAnimation::<Impl, IMPL_OFFSET>, SetEnableDependentAnimation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationUsingKeyFramesStaticsImpl: Sized {
    fn EnableDependentAnimationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointAnimationUsingKeyFramesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimationUsingKeyFramesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IPointAnimationUsingKeyFramesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointAnimationUsingKeyFramesStatics>, ::windows::core::GetTrustLevel, EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetValue(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPointKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IPointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IPointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IPointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointKeyFrame>, ::windows::core::GetTrustLevel, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, KeyTime::<Impl, IMPL_OFFSET>, SetKeyTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PointKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPointKeyFrameFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointKeyFrameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPointKeyFrameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointKeyFrameFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameStaticsImpl: Sized {
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IPointKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IPointKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointKeyFrameStatics>, ::windows::core::GetTrustLevel, ValueProperty::<Impl, IMPL_OFFSET>, KeyTimeProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDownThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDownThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerDownThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDownThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerDownThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerDownThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPointerDownThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPointerDownThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointerDownThemeAnimation>, ::windows::core::GetTrustLevel, TargetName::<Impl, IMPL_OFFSET>, SetTargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerDownThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDownThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDownThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerDownThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDownThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerDownThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerDownThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPointerDownThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointerDownThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerDownThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerUpThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerUpThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerUpThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerUpThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerUpThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerUpThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPointerUpThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPointerUpThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointerUpThemeAnimation>, ::windows::core::GetTrustLevel, TargetName::<Impl, IMPL_OFFSET>, SetTargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerUpThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerUpThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerUpThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerUpThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerUpThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerUpThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerUpThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPointerUpThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPointerUpThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerUpThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopInThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopInThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPopInThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopInThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopInThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPopInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPopInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPopInThemeAnimation>,
            ::windows::core::GetTrustLevel,
            TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopInThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopInThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopInThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopInThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopInThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopInThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopInThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPopInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IPopInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IPopInThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPopInThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>, FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>, FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopInThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopOutThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopOutThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPopOutThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopOutThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopOutThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPopOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPopOutThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPopOutThemeAnimation>, ::windows::core::GetTrustLevel, TargetName::<Impl, IMPL_OFFSET>, SetTargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopOutThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopOutThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopOutThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopOutThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopOutThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopOutThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopOutThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPopOutThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPopOutThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopOutThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupThemeTransitionImpl: Sized {
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopupThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupThemeTransitionVtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPopupThemeTransition>, ::windows::core::GetTrustLevel, FromHorizontalOffset::<Impl, IMPL_OFFSET>, SetFromHorizontalOffset::<Impl, IMPL_OFFSET>, FromVerticalOffset::<Impl, IMPL_OFFSET>, SetFromVerticalOffset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupThemeTransitionStaticsImpl: Sized {
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopupThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupThemeTransitionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupThemeTransitionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupThemeTransitionStaticsVtbl {
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IPopupThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IPopupThemeTransitionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPopupThemeTransitionStatics>, ::windows::core::GetTrustLevel, FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>, FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEaseImpl: Sized {
    fn Power(&self) -> ::windows::core::Result<f64>;
    fn SetPower(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPowerEase";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPowerEaseVtbl {
        unsafe extern "system" fn Power<Impl: IPowerEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Power() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPower<Impl: IPowerEaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPower(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPowerEase>, ::windows::core::GetTrustLevel, Power::<Impl, IMPL_OFFSET>, SetPower::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPowerEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEaseStaticsImpl: Sized {
    fn PowerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPowerEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerEaseStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerEaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPowerEaseStaticsVtbl {
        unsafe extern "system" fn PowerProperty<Impl: IPowerEaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPowerEaseStatics>, ::windows::core::GetTrustLevel, PowerProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPowerEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuadraticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IQuadraticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IQuadraticEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuadraticEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuadraticEaseVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQuadraticEase>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuadraticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuarticEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuarticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IQuarticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IQuarticEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuarticEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuarticEaseVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQuarticEase>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuarticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuinticEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuinticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IQuinticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IQuinticEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuinticEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuinticEaseVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQuinticEase>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuinticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReorderThemeTransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReorderThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IReorderThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IReorderThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReorderThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReorderThemeTransitionVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IReorderThemeTransition>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReorderThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatBehaviorHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatBehaviorHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepeatBehaviorHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatBehaviorHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatBehaviorHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatBehaviorHelperVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRepeatBehaviorHelper>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatBehaviorHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRepeatBehaviorHelperStaticsImpl: Sized {
    fn Forever(&self) -> ::windows::core::Result<RepeatBehavior>;
    fn FromCount(&self, count: f64) -> ::windows::core::Result<RepeatBehavior>;
    fn FromDuration(&self, duration: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<RepeatBehavior>;
    fn GetHasCount(&self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn GetHasDuration(&self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &RepeatBehavior, value: &RepeatBehavior) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRepeatBehaviorHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepeatBehaviorHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRepeatBehaviorHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatBehaviorHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatBehaviorHelperStaticsVtbl {
        unsafe extern "system" fn Forever<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Forever() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromCount<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: f64, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromCount(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromDuration<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: super::super::super::super::Foundation::TimeSpan, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromDuration(&*(&duration as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCount<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasCount(&*(&target as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasDuration<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasDuration(&*(&target as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, value: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IRepeatBehaviorHelperStatics>,
            ::windows::core::GetTrustLevel,
            Forever::<Impl, IMPL_OFFSET>,
            FromCount::<Impl, IMPL_OFFSET>,
            FromDuration::<Impl, IMPL_OFFSET>,
            GetHasCount::<Impl, IMPL_OFFSET>,
            GetHasDuration::<Impl, IMPL_OFFSET>,
            Equals::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatBehaviorHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IRepositionThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IRepositionThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRepositionThemeAnimation>,
            ::windows::core::GetTrustLevel,
            TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IRepositionThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IRepositionThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IRepositionThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRepositionThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>, FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>, FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeTransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeTransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeTransitionVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRepositionThemeTransition>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransition2Impl: Sized {
    fn IsStaggeringEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeTransition2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeTransition2";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeTransition2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeTransition2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeTransition2Vtbl {
        unsafe extern "system" fn IsStaggeringEnabled<Impl: IRepositionThemeTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: IRepositionThemeTransition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRepositionThemeTransition2>, ::windows::core::GetTrustLevel, IsStaggeringEnabled::<Impl, IMPL_OFFSET>, SetIsStaggeringEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeTransition2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransitionStatics2Impl: Sized {
    fn IsStaggeringEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeTransitionStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeTransitionStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeTransitionStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeTransitionStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeTransitionStatics2Vtbl {
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: IRepositionThemeTransitionStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRepositionThemeTransitionStatics2>, ::windows::core::GetTrustLevel, IsStaggeringEnabledProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeTransitionStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISineEaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISineEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISineEase";
}
#[cfg(feature = "implement_exclusive")]
impl ISineEaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISineEaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISineEaseVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISineEase>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISineEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlideNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISlideNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISlideNavigationTransitionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlideNavigationTransitionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlideNavigationTransitionInfoVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISlideNavigationTransitionInfo>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlideNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfo2Impl: Sized {
    fn Effect(&self) -> ::windows::core::Result<SlideNavigationTransitionEffect>;
    fn SetEffect(&self, value: SlideNavigationTransitionEffect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlideNavigationTransitionInfo2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISlideNavigationTransitionInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl ISlideNavigationTransitionInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlideNavigationTransitionInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlideNavigationTransitionInfo2Vtbl {
        unsafe extern "system" fn Effect<Impl: ISlideNavigationTransitionInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SlideNavigationTransitionEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Effect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffect<Impl: ISlideNavigationTransitionInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SlideNavigationTransitionEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffect(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISlideNavigationTransitionInfo2>, ::windows::core::GetTrustLevel, Effect::<Impl, IMPL_OFFSET>, SetEffect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlideNavigationTransitionInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfoStatics2Impl: Sized {
    fn EffectProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlideNavigationTransitionInfoStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISlideNavigationTransitionInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISlideNavigationTransitionInfoStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlideNavigationTransitionInfoStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlideNavigationTransitionInfoStatics2Vtbl {
        unsafe extern "system" fn EffectProperty<Impl: ISlideNavigationTransitionInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISlideNavigationTransitionInfoStatics2>, ::windows::core::GetTrustLevel, EffectProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlideNavigationTransitionInfoStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineColorKeyFrameImpl: Sized {
    fn KeySpline(&self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineColorKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineColorKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineColorKeyFrameVtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplineColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpline<Impl: ISplineColorKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISplineColorKeyFrame>, ::windows::core::GetTrustLevel, KeySpline::<Impl, IMPL_OFFSET>, SetKeySpline::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineColorKeyFrameStaticsImpl: Sized {
    fn KeySplineProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineColorKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineColorKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineColorKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineColorKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineColorKeyFrameStaticsVtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplineColorKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySplineProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISplineColorKeyFrameStatics>, ::windows::core::GetTrustLevel, KeySplineProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineColorKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineDoubleKeyFrameImpl: Sized {
    fn KeySpline(&self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineDoubleKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineDoubleKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineDoubleKeyFrameVtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplineDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpline<Impl: ISplineDoubleKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISplineDoubleKeyFrame>, ::windows::core::GetTrustLevel, KeySpline::<Impl, IMPL_OFFSET>, SetKeySpline::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineDoubleKeyFrameStaticsImpl: Sized {
    fn KeySplineProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineDoubleKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineDoubleKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineDoubleKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineDoubleKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineDoubleKeyFrameStaticsVtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplineDoubleKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySplineProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISplineDoubleKeyFrameStatics>, ::windows::core::GetTrustLevel, KeySplineProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineDoubleKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplinePointKeyFrameImpl: Sized {
    fn KeySpline(&self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplinePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplinePointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ISplinePointKeyFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplinePointKeyFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplinePointKeyFrameVtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplinePointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpline<Impl: ISplinePointKeyFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISplinePointKeyFrame>, ::windows::core::GetTrustLevel, KeySpline::<Impl, IMPL_OFFSET>, SetKeySpline::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplinePointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplinePointKeyFrameStaticsImpl: Sized {
    fn KeySplineProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplinePointKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplinePointKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplinePointKeyFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplinePointKeyFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplinePointKeyFrameStaticsVtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplinePointKeyFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySplineProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISplinePointKeyFrameStatics>, ::windows::core::GetTrustLevel, KeySplineProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplinePointKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait ISplitCloseThemeAnimationImpl: Sized {
    fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOpenedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOpenedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClosedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetClosedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetContentTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn OpenedLength(&self) -> ::windows::core::Result<f64>;
    fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn ClosedLength(&self) -> ::windows::core::Result<f64>;
    fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetFromCenter(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()>;
    fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
    fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64>;
    fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISplitCloseThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitCloseThemeAnimation";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ISplitCloseThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitCloseThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitCloseThemeAnimationVtbl {
        unsafe extern "system" fn OpenedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedLength(value).into()
        }
        unsafe extern "system" fn ClosedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedLength(value).into()
        }
        unsafe extern "system" fn OffsetFromCenter<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetFromCenter<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetFromCenter(value).into()
        }
        unsafe extern "system" fn ContentTranslationDirection<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationDirection<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationDirection(value).into()
        }
        unsafe extern "system" fn ContentTranslationOffset<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationOffset<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationOffset(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISplitCloseThemeAnimation>,
            ::windows::core::GetTrustLevel,
            OpenedTargetName::<Impl, IMPL_OFFSET>,
            SetOpenedTargetName::<Impl, IMPL_OFFSET>,
            OpenedTarget::<Impl, IMPL_OFFSET>,
            SetOpenedTarget::<Impl, IMPL_OFFSET>,
            ClosedTargetName::<Impl, IMPL_OFFSET>,
            SetClosedTargetName::<Impl, IMPL_OFFSET>,
            ClosedTarget::<Impl, IMPL_OFFSET>,
            SetClosedTarget::<Impl, IMPL_OFFSET>,
            ContentTargetName::<Impl, IMPL_OFFSET>,
            SetContentTargetName::<Impl, IMPL_OFFSET>,
            ContentTarget::<Impl, IMPL_OFFSET>,
            SetContentTarget::<Impl, IMPL_OFFSET>,
            OpenedLength::<Impl, IMPL_OFFSET>,
            SetOpenedLength::<Impl, IMPL_OFFSET>,
            ClosedLength::<Impl, IMPL_OFFSET>,
            SetClosedLength::<Impl, IMPL_OFFSET>,
            OffsetFromCenter::<Impl, IMPL_OFFSET>,
            SetOffsetFromCenter::<Impl, IMPL_OFFSET>,
            ContentTranslationDirection::<Impl, IMPL_OFFSET>,
            SetContentTranslationDirection::<Impl, IMPL_OFFSET>,
            ContentTranslationOffset::<Impl, IMPL_OFFSET>,
            SetContentTranslationOffset::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitCloseThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitCloseThemeAnimationStaticsImpl: Sized {
    fn OpenedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetFromCenterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationDirectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitCloseThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitCloseThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitCloseThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitCloseThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitCloseThemeAnimationStaticsVtbl {
        unsafe extern "system" fn OpenedTargetNameProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedTargetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetNameProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetNameProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedLengthProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedLengthProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromCenterProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationDirectionProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationOffsetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationOffsetProperty() {
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
            ::windows::core::GetRuntimeClassName::<ISplitCloseThemeAnimationStatics>,
            ::windows::core::GetTrustLevel,
            OpenedTargetNameProperty::<Impl, IMPL_OFFSET>,
            OpenedTargetProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetNameProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetProperty::<Impl, IMPL_OFFSET>,
            ContentTargetNameProperty::<Impl, IMPL_OFFSET>,
            ContentTargetProperty::<Impl, IMPL_OFFSET>,
            OpenedLengthProperty::<Impl, IMPL_OFFSET>,
            ClosedLengthProperty::<Impl, IMPL_OFFSET>,
            OffsetFromCenterProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationDirectionProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationOffsetProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitCloseThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait ISplitOpenThemeAnimationImpl: Sized {
    fn OpenedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOpenedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOpenedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ClosedTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClosedTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClosedTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetClosedTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ContentTargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTarget(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetContentTarget(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn OpenedLength(&self) -> ::windows::core::Result<f64>;
    fn SetOpenedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn ClosedLength(&self) -> ::windows::core::Result<f64>;
    fn SetClosedLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetFromCenter(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetFromCenter(&self, value: f64) -> ::windows::core::Result<()>;
    fn ContentTranslationDirection(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
    fn ContentTranslationOffset(&self) -> ::windows::core::Result<f64>;
    fn SetContentTranslationOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISplitOpenThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitOpenThemeAnimation";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ISplitOpenThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitOpenThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitOpenThemeAnimationVtbl {
        unsafe extern "system" fn OpenedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedLength(value).into()
        }
        unsafe extern "system" fn ClosedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedLength(value).into()
        }
        unsafe extern "system" fn OffsetFromCenter<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetFromCenter<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetFromCenter(value).into()
        }
        unsafe extern "system" fn ContentTranslationDirection<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationDirection<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationDirection(value).into()
        }
        unsafe extern "system" fn ContentTranslationOffset<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationOffset<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationOffset(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISplitOpenThemeAnimation>,
            ::windows::core::GetTrustLevel,
            OpenedTargetName::<Impl, IMPL_OFFSET>,
            SetOpenedTargetName::<Impl, IMPL_OFFSET>,
            OpenedTarget::<Impl, IMPL_OFFSET>,
            SetOpenedTarget::<Impl, IMPL_OFFSET>,
            ClosedTargetName::<Impl, IMPL_OFFSET>,
            SetClosedTargetName::<Impl, IMPL_OFFSET>,
            ClosedTarget::<Impl, IMPL_OFFSET>,
            SetClosedTarget::<Impl, IMPL_OFFSET>,
            ContentTargetName::<Impl, IMPL_OFFSET>,
            SetContentTargetName::<Impl, IMPL_OFFSET>,
            ContentTarget::<Impl, IMPL_OFFSET>,
            SetContentTarget::<Impl, IMPL_OFFSET>,
            OpenedLength::<Impl, IMPL_OFFSET>,
            SetOpenedLength::<Impl, IMPL_OFFSET>,
            ClosedLength::<Impl, IMPL_OFFSET>,
            SetClosedLength::<Impl, IMPL_OFFSET>,
            OffsetFromCenter::<Impl, IMPL_OFFSET>,
            SetOffsetFromCenter::<Impl, IMPL_OFFSET>,
            ContentTranslationDirection::<Impl, IMPL_OFFSET>,
            SetContentTranslationDirection::<Impl, IMPL_OFFSET>,
            ContentTranslationOffset::<Impl, IMPL_OFFSET>,
            SetContentTranslationOffset::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitOpenThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitOpenThemeAnimationStaticsImpl: Sized {
    fn OpenedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedLengthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetFromCenterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationDirectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitOpenThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitOpenThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitOpenThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitOpenThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitOpenThemeAnimationStaticsVtbl {
        unsafe extern "system" fn OpenedTargetNameProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedTargetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetNameProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetNameProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedLengthProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedLengthProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromCenterProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationDirectionProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationOffsetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTranslationOffsetProperty() {
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
            ::windows::core::GetRuntimeClassName::<ISplitOpenThemeAnimationStatics>,
            ::windows::core::GetTrustLevel,
            OpenedTargetNameProperty::<Impl, IMPL_OFFSET>,
            OpenedTargetProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetNameProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetProperty::<Impl, IMPL_OFFSET>,
            ContentTargetNameProperty::<Impl, IMPL_OFFSET>,
            ContentTargetProperty::<Impl, IMPL_OFFSET>,
            OpenedLengthProperty::<Impl, IMPL_OFFSET>,
            ClosedLengthProperty::<Impl, IMPL_OFFSET>,
            OffsetFromCenterProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationDirectionProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationOffsetProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitOpenThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoryboardImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<TimelineCollection>;
    fn Seek(&self, offset: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Begin(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn GetCurrentState(&self) -> ::windows::core::Result<ClockState>;
    fn GetCurrentTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SeekAlignedToLastTick(&self, offset: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SkipToFill(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IStoryboard";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoryboardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoryboardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoryboardVtbl {
        unsafe extern "system" fn Children<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(&*(&offset as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stop<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Begin<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin().into()
        }
        unsafe extern "system" fn Pause<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn GetCurrentState<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClockState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTime<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeekAlignedToLastTick<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SeekAlignedToLastTick(&*(&offset as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SkipToFill<Impl: IStoryboardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SkipToFill().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoryboard>,
            ::windows::core::GetTrustLevel,
            Children::<Impl, IMPL_OFFSET>,
            Seek::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Begin::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            GetCurrentState::<Impl, IMPL_OFFSET>,
            GetCurrentTime::<Impl, IMPL_OFFSET>,
            SeekAlignedToLastTick::<Impl, IMPL_OFFSET>,
            SkipToFill::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoryboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoryboardStaticsImpl: Sized {
    fn TargetPropertyProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTargetProperty(&self, element: &::core::option::Option<Timeline>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetProperty(&self, element: &::core::option::Option<Timeline>, path: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTargetName(&self, element: &::core::option::Option<Timeline>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, element: &::core::option::Option<Timeline>, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetTarget(&self, timeline: &::core::option::Option<Timeline>, target: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoryboardStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IStoryboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStoryboardStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoryboardStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoryboardStaticsVtbl {
        unsafe extern "system" fn TargetPropertyProperty<Impl: IStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetPropertyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetProperty<Impl: IStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetProperty(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetProperty<Impl: IStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetProperty(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetNameProperty<Impl: IStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetName<Impl: IStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetName(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetTarget<Impl: IStoryboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeline: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&timeline as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&target as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoryboardStatics>,
            ::windows::core::GetTrustLevel,
            TargetPropertyProperty::<Impl, IMPL_OFFSET>,
            GetTargetProperty::<Impl, IMPL_OFFSET>,
            SetTargetProperty::<Impl, IMPL_OFFSET>,
            TargetNameProperty::<Impl, IMPL_OFFSET>,
            GetTargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            SetTarget::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoryboardStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISuppressNavigationTransitionInfoImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISuppressNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISuppressNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISuppressNavigationTransitionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuppressNavigationTransitionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISuppressNavigationTransitionInfoVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISuppressNavigationTransitionInfo>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuppressNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeBackThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeBackThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeBackThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeBackThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeBackThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeBackThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISwipeBackThemeAnimation>,
            ::windows::core::GetTrustLevel,
            TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeBackThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeBackThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeBackThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeBackThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeBackThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeBackThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeBackThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: ISwipeBackThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: ISwipeBackThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: ISwipeBackThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISwipeBackThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>, FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>, FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeBackThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeHintThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ToHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetToHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn ToVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetToVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeHintThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeHintThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeHintThemeAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeHintThemeAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeHintThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ToHorizontalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToHorizontalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToHorizontalOffset(value).into()
        }
        unsafe extern "system" fn ToVerticalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToVerticalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToVerticalOffset(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISwipeHintThemeAnimation>,
            ::windows::core::GetTrustLevel,
            TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            ToHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetToHorizontalOffset::<Impl, IMPL_OFFSET>,
            ToVerticalOffset::<Impl, IMPL_OFFSET>,
            SetToVerticalOffset::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeHintThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeHintThemeAnimationStaticsImpl: Sized {
    fn TargetNameProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToHorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToVerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeHintThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeHintThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeHintThemeAnimationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeHintThemeAnimationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeHintThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: ISwipeHintThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToHorizontalOffsetProperty<Impl: ISwipeHintThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVerticalOffsetProperty<Impl: ISwipeHintThemeAnimationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISwipeHintThemeAnimationStatics>, ::windows::core::GetTrustLevel, TargetNameProperty::<Impl, IMPL_OFFSET>, ToHorizontalOffsetProperty::<Impl, IMPL_OFFSET>, ToVerticalOffsetProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeHintThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITimelineImpl: Sized {
    fn AutoReverse(&self) -> ::windows::core::Result<bool>;
    fn SetAutoReverse(&self, value: bool) -> ::windows::core::Result<()>;
    fn BeginTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>;
    fn SetBeginTime(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Duration>;
    fn SetDuration(&self, value: &super::super::Duration) -> ::windows::core::Result<()>;
    fn SpeedRatio(&self) -> ::windows::core::Result<f64>;
    fn SetSpeedRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn FillBehavior(&self) -> ::windows::core::Result<FillBehavior>;
    fn SetFillBehavior(&self, value: FillBehavior) -> ::windows::core::Result<()>;
    fn RepeatBehavior(&self) -> ::windows::core::Result<RepeatBehavior>;
    fn SetRepeatBehavior(&self, value: &RepeatBehavior) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimeline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITimeline";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITimelineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineVtbl {
        unsafe extern "system" fn AutoReverse<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoReverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReverse<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoReverse(value).into()
        }
        unsafe extern "system" fn BeginTime<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeginTime<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBeginTime(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Duration as ::windows::core::Abi>::Abi as *const <super::super::Duration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpeedRatio<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeedRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeedRatio<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeedRatio(value).into()
        }
        unsafe extern "system" fn FillBehavior<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FillBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBehavior<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FillBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBehavior(value).into()
        }
        unsafe extern "system" fn RepeatBehavior<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepeatBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepeatBehavior<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepeatBehavior(&*(&value as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: ITimelineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITimeline>,
            ::windows::core::GetTrustLevel,
            AutoReverse::<Impl, IMPL_OFFSET>,
            SetAutoReverse::<Impl, IMPL_OFFSET>,
            BeginTime::<Impl, IMPL_OFFSET>,
            SetBeginTime::<Impl, IMPL_OFFSET>,
            Duration::<Impl, IMPL_OFFSET>,
            SetDuration::<Impl, IMPL_OFFSET>,
            SpeedRatio::<Impl, IMPL_OFFSET>,
            SetSpeedRatio::<Impl, IMPL_OFFSET>,
            FillBehavior::<Impl, IMPL_OFFSET>,
            SetFillBehavior::<Impl, IMPL_OFFSET>,
            RepeatBehavior::<Impl, IMPL_OFFSET>,
            SetRepeatBehavior::<Impl, IMPL_OFFSET>,
            Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeline as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Timeline>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITimelineFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITimelineFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimelineFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimelineFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineStaticsImpl: Sized {
    fn AllowDependentAnimations(&self) -> ::windows::core::Result<bool>;
    fn SetAllowDependentAnimations(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoReverseProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BeginTimeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DurationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SpeedRatioProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FillBehaviorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RepeatBehaviorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITimelineStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineStaticsVtbl {
        unsafe extern "system" fn AllowDependentAnimations<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowDependentAnimations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowDependentAnimations<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowDependentAnimations(value).into()
        }
        unsafe extern "system" fn AutoReverseProperty<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoReverseProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginTimeProperty<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DurationProperty<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DurationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeedRatioProperty<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeedRatioProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillBehaviorProperty<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillBehaviorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RepeatBehaviorProperty<Impl: ITimelineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepeatBehaviorProperty() {
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
            ::windows::core::GetRuntimeClassName::<ITimelineStatics>,
            ::windows::core::GetTrustLevel,
            AllowDependentAnimations::<Impl, IMPL_OFFSET>,
            SetAllowDependentAnimations::<Impl, IMPL_OFFSET>,
            AutoReverseProperty::<Impl, IMPL_OFFSET>,
            BeginTimeProperty::<Impl, IMPL_OFFSET>,
            DurationProperty::<Impl, IMPL_OFFSET>,
            SpeedRatioProperty::<Impl, IMPL_OFFSET>,
            FillBehaviorProperty::<Impl, IMPL_OFFSET>,
            RepeatBehaviorProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimelineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITransition";
}
#[cfg(feature = "implement_exclusive")]
impl ITransitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransitionVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransition>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransitionFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITransitionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransitionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransitionFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransitionFactory>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransitionFactory as ::windows::core::Interface>::IID
    }
}
