#[cfg(feature = "implement_exclusive")]
pub trait IAddDeleteThemeTransitionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAddDeleteThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IAddDeleteThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IAddDeleteThemeTransitionVtbl {
    pub const fn new<Impl: IAddDeleteThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAddDeleteThemeTransitionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAddDeleteThemeTransition>, base.5)
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
    pub const fn new<Impl: IBackEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackEaseVtbl {
        unsafe extern "system" fn Amplitude<Impl: IBackEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Amplitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmplitude<Impl: IBackEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAmplitude(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackEase>, base.5, Amplitude::<Impl, OFFSET>, SetAmplitude::<Impl, OFFSET>)
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
    pub const fn new<Impl: IBackEaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackEaseStaticsVtbl {
        unsafe extern "system" fn AmplitudeProperty<Impl: IBackEaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AmplitudeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackEaseStatics>, base.5, AmplitudeProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IBasicConnectedAnimationConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBasicConnectedAnimationConfigurationVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBasicConnectedAnimationConfiguration>, base.5)
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
    pub const fn new<Impl: IBasicConnectedAnimationConfigurationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBasicConnectedAnimationConfigurationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBasicConnectedAnimationConfigurationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBasicConnectedAnimationConfigurationFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IBeginStoryboardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBeginStoryboardVtbl {
        unsafe extern "system" fn Storyboard<Impl: IBeginStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Storyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboard<Impl: IBeginStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStoryboard(&*(&value as *const <Storyboard as ::windows::core::Abi>::Abi as *const <Storyboard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBeginStoryboard>, base.5, Storyboard::<Impl, OFFSET>, SetStoryboard::<Impl, OFFSET>)
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
    pub const fn new<Impl: IBeginStoryboardStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBeginStoryboardStaticsVtbl {
        unsafe extern "system" fn StoryboardProperty<Impl: IBeginStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StoryboardProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBeginStoryboardStatics>, base.5, StoryboardProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IBounceEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBounceEaseVtbl {
        unsafe extern "system" fn Bounces<Impl: IBounceEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounces<Impl: IBounceEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBounces(value).into()
        }
        unsafe extern "system" fn Bounciness<Impl: IBounceEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounciness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounciness<Impl: IBounceEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBounciness(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBounceEase>, base.5, Bounces::<Impl, OFFSET>, SetBounces::<Impl, OFFSET>, Bounciness::<Impl, OFFSET>, SetBounciness::<Impl, OFFSET>)
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
    pub const fn new<Impl: IBounceEaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBounceEaseStaticsVtbl {
        unsafe extern "system" fn BouncesProperty<Impl: IBounceEaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BouncesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BouncinessProperty<Impl: IBounceEaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BouncinessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBounceEaseStatics>, base.5, BouncesProperty::<Impl, OFFSET>, BouncinessProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICircleEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICircleEaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICircleEase>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IColorAnimationVtbl {
    pub const fn new<Impl: IColorAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorAnimationVtbl {
        unsafe extern "system" fn From<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).By() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBy<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IColorAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorAnimation>, base.5, From::<Impl, OFFSET>, SetFrom::<Impl, OFFSET>, To::<Impl, OFFSET>, SetTo::<Impl, OFFSET>, By::<Impl, OFFSET>, SetBy::<Impl, OFFSET>, EasingFunction::<Impl, OFFSET>, SetEasingFunction::<Impl, OFFSET>, EnableDependentAnimation::<Impl, OFFSET>, SetEnableDependentAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IColorAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorAnimationStaticsVtbl {
        unsafe extern "system" fn FromProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ByProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EasingFunctionProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IColorAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorAnimationStatics>, base.5, FromProperty::<Impl, OFFSET>, ToProperty::<Impl, OFFSET>, ByProperty::<Impl, OFFSET>, EasingFunctionProperty::<Impl, OFFSET>, EnableDependentAnimationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<ColorKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimationUsingKeyFrames";
}
#[cfg(feature = "implement_exclusive")]
impl IColorAnimationUsingKeyFramesVtbl {
    pub const fn new<Impl: IColorAnimationUsingKeyFramesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IColorAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IColorAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IColorAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorAnimationUsingKeyFrames>, base.5, KeyFrames::<Impl, OFFSET>, EnableDependentAnimation::<Impl, OFFSET>, SetEnableDependentAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IColorAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IColorAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorAnimationUsingKeyFramesStatics>, base.5, EnableDependentAnimationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetValue(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IColorKeyFrameVtbl {
    pub const fn new<Impl: IColorKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorKeyFrame>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, KeyTime::<Impl, OFFSET>, SetKeyTime::<Impl, OFFSET>)
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
    pub const fn new<Impl: IColorKeyFrameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorKeyFrameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorKeyFrameFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IColorKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IColorKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IColorKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorKeyFrameStatics>, base.5, ValueProperty::<Impl, OFFSET>, KeyTimeProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICommonNavigationTransitionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommonNavigationTransitionInfoVtbl {
        unsafe extern "system" fn IsStaggeringEnabled<Impl: ICommonNavigationTransitionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: ICommonNavigationTransitionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommonNavigationTransitionInfo>, base.5, IsStaggeringEnabled::<Impl, OFFSET>, SetIsStaggeringEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommonNavigationTransitionInfoStaticsVtbl {
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStaggerElementProperty<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStaggerElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsStaggerElement<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsStaggerElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggerElement<Impl: ICommonNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsStaggerElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommonNavigationTransitionInfoStatics>, base.5, IsStaggeringEnabledProperty::<Impl, OFFSET>, IsStaggerElementProperty::<Impl, OFFSET>, GetIsStaggerElement::<Impl, OFFSET>, SetIsStaggerElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationImpl: Sized {
    fn Completed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryStart(&self, destination: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationVtbl {
    pub const fn new<Impl: IConnectedAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConnectedAnimationVtbl {
        unsafe extern "system" fn Completed<Impl: IConnectedAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IConnectedAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryStart<Impl: IConnectedAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStart(&*(&destination as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IConnectedAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConnectedAnimation>, base.5, Completed::<Impl, OFFSET>, RemoveCompleted::<Impl, OFFSET>, TryStart::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimation2Impl: Sized {
    fn IsScaleAnimationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsScaleAnimationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryStartWithCoordinatedElements(&self, destination: &::core::option::Option<super::super::UIElement>, coordinatedelements: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement>>) -> ::windows::core::Result<bool>;
    fn SetAnimationComponent(&self, component: ConnectedAnimationComponent, animation: &::core::option::Option<super::super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimation2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation2";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimation2Vtbl {
    pub const fn new<Impl: IConnectedAnimation2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConnectedAnimation2Vtbl {
        unsafe extern "system" fn IsScaleAnimationEnabled<Impl: IConnectedAnimation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScaleAnimationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsScaleAnimationEnabled<Impl: IConnectedAnimation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsScaleAnimationEnabled(value).into()
        }
        unsafe extern "system" fn TryStartWithCoordinatedElements<Impl: IConnectedAnimation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, coordinatedelements: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStartWithCoordinatedElements(&*(&destination as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&coordinatedelements as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationComponent<Impl: IConnectedAnimation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, component: ConnectedAnimationComponent, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAnimationComponent(component, &*(&animation as *const <super::super::super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConnectedAnimation2>, base.5, IsScaleAnimationEnabled::<Impl, OFFSET>, SetIsScaleAnimationEnabled::<Impl, OFFSET>, TryStartWithCoordinatedElements::<Impl, OFFSET>, SetAnimationComponent::<Impl, OFFSET>)
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
    pub const fn new<Impl: IConnectedAnimation3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConnectedAnimation3Vtbl {
        unsafe extern "system" fn Configuration<Impl: IConnectedAnimation3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Impl: IConnectedAnimation3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetConfiguration(&*(&value as *const <ConnectedAnimationConfiguration as ::windows::core::Abi>::Abi as *const <ConnectedAnimationConfiguration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConnectedAnimation3>, base.5, Configuration::<Impl, OFFSET>, SetConfiguration::<Impl, OFFSET>)
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
    pub const fn new<Impl: IConnectedAnimationConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConnectedAnimationConfigurationVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConnectedAnimationConfiguration>, base.5)
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
    pub const fn new<Impl: IConnectedAnimationConfigurationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConnectedAnimationConfigurationFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConnectedAnimationConfigurationFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationServiceImpl: Sized {
    fn DefaultDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SetDefaultDuration(&self, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DefaultEasingFunction(&self) -> ::windows::core::Result<super::super::super::Composition::CompositionEasingFunction>;
    fn SetDefaultEasingFunction(&self, value: &::core::option::Option<super::super::super::Composition::CompositionEasingFunction>) -> ::windows::core::Result<()>;
    fn PrepareToAnimate(&self, key: &::windows::core::HSTRING, source: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<ConnectedAnimation>;
    fn GetAnimation(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<ConnectedAnimation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimationService {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationService";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationServiceVtbl {
    pub const fn new<Impl: IConnectedAnimationServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConnectedAnimationServiceVtbl {
        unsafe extern "system" fn DefaultDuration<Impl: IConnectedAnimationServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDuration<Impl: IConnectedAnimationServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultDuration(&*(&value as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultEasingFunction<Impl: IConnectedAnimationServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultEasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultEasingFunction<Impl: IConnectedAnimationServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultEasingFunction(&*(&value as *const <super::super::super::Composition::CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrepareToAnimate<Impl: IConnectedAnimationServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareToAnimate(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&source as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnimation<Impl: IConnectedAnimationServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAnimation(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConnectedAnimationService>, base.5, DefaultDuration::<Impl, OFFSET>, SetDefaultDuration::<Impl, OFFSET>, DefaultEasingFunction::<Impl, OFFSET>, SetDefaultEasingFunction::<Impl, OFFSET>, PrepareToAnimate::<Impl, OFFSET>, GetAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IConnectedAnimationServiceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConnectedAnimationServiceStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IConnectedAnimationServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConnectedAnimationServiceStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
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
    pub const fn new<Impl: IContentThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContentThemeTransitionVtbl {
        unsafe extern "system" fn HorizontalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IContentThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContentThemeTransition>, base.5, HorizontalOffset::<Impl, OFFSET>, SetHorizontalOffset::<Impl, OFFSET>, VerticalOffset::<Impl, OFFSET>, SetVerticalOffset::<Impl, OFFSET>)
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
    pub const fn new<Impl: IContentThemeTransitionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContentThemeTransitionStaticsVtbl {
        unsafe extern "system" fn HorizontalOffsetProperty<Impl: IContentThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffsetProperty<Impl: IContentThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContentThemeTransitionStatics>, base.5, HorizontalOffsetProperty::<Impl, OFFSET>, VerticalOffsetProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IContinuumNavigationTransitionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContinuumNavigationTransitionInfoVtbl {
        unsafe extern "system" fn ExitElement<Impl: IContinuumNavigationTransitionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitElement<Impl: IContinuumNavigationTransitionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitElement(&*(&value as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContinuumNavigationTransitionInfo>, base.5, ExitElement::<Impl, OFFSET>, SetExitElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContinuumNavigationTransitionInfoStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContinuumNavigationTransitionInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContinuumNavigationTransitionInfoStaticsVtbl {
    pub const fn new<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContinuumNavigationTransitionInfoStaticsVtbl {
        unsafe extern "system" fn ExitElementProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEntranceElementProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEntranceElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsEntranceElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsEntranceElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEntranceElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEntranceElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn IsExitElementProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsExitElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsExitElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsExitElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsExitElement<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsExitElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ExitElementContainerProperty<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitElementContainerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExitElementContainer<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExitElementContainer(&*(&element as *const <super::super::Controls::ListViewBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBase as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitElementContainer<Impl: IContinuumNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitElementContainer(&*(&element as *const <super::super::Controls::ListViewBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBase as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IContinuumNavigationTransitionInfoStatics>,
            base.5,
            ExitElementProperty::<Impl, OFFSET>,
            IsEntranceElementProperty::<Impl, OFFSET>,
            GetIsEntranceElement::<Impl, OFFSET>,
            SetIsEntranceElement::<Impl, OFFSET>,
            IsExitElementProperty::<Impl, OFFSET>,
            GetIsExitElement::<Impl, OFFSET>,
            SetIsExitElement::<Impl, OFFSET>,
            ExitElementContainerProperty::<Impl, OFFSET>,
            GetExitElementContainer::<Impl, OFFSET>,
            SetExitElementContainer::<Impl, OFFSET>,
        )
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
    pub const fn new<Impl: ICubicEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICubicEaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICubicEase>, base.5)
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
    pub const fn new<Impl: IDirectConnectedAnimationConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectConnectedAnimationConfigurationVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectConnectedAnimationConfiguration>, base.5)
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
    pub const fn new<Impl: IDirectConnectedAnimationConfigurationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectConnectedAnimationConfigurationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDirectConnectedAnimationConfigurationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectConnectedAnimationConfigurationFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDiscreteColorKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiscreteColorKeyFrameVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiscreteColorKeyFrame>, base.5)
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
    pub const fn new<Impl: IDiscreteDoubleKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiscreteDoubleKeyFrameVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiscreteDoubleKeyFrame>, base.5)
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
    pub const fn new<Impl: IDiscreteObjectKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiscreteObjectKeyFrameVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiscreteObjectKeyFrame>, base.5)
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
    pub const fn new<Impl: IDiscretePointKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiscretePointKeyFrameVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiscretePointKeyFrame>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleAnimationVtbl {
    pub const fn new<Impl: IDoubleAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleAnimationVtbl {
        unsafe extern "system" fn From<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).By() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBy<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IDoubleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleAnimation>, base.5, From::<Impl, OFFSET>, SetFrom::<Impl, OFFSET>, To::<Impl, OFFSET>, SetTo::<Impl, OFFSET>, By::<Impl, OFFSET>, SetBy::<Impl, OFFSET>, EasingFunction::<Impl, OFFSET>, SetEasingFunction::<Impl, OFFSET>, EnableDependentAnimation::<Impl, OFFSET>, SetEnableDependentAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDoubleAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleAnimationStaticsVtbl {
        unsafe extern "system" fn FromProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ByProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EasingFunctionProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IDoubleAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleAnimationStatics>, base.5, FromProperty::<Impl, OFFSET>, ToProperty::<Impl, OFFSET>, ByProperty::<Impl, OFFSET>, EasingFunctionProperty::<Impl, OFFSET>, EnableDependentAnimationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<DoubleKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimationUsingKeyFrames";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleAnimationUsingKeyFramesVtbl {
    pub const fn new<Impl: IDoubleAnimationUsingKeyFramesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IDoubleAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IDoubleAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IDoubleAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleAnimationUsingKeyFrames>, base.5, KeyFrames::<Impl, OFFSET>, EnableDependentAnimation::<Impl, OFFSET>, SetEnableDependentAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDoubleAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IDoubleAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleAnimationUsingKeyFramesStatics>, base.5, EnableDependentAnimationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleKeyFrameVtbl {
    pub const fn new<Impl: IDoubleKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleKeyFrame>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, KeyTime::<Impl, OFFSET>, SetKeyTime::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDoubleKeyFrameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDoubleKeyFrameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleKeyFrameFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDoubleKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IDoubleKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IDoubleKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleKeyFrameStatics>, base.5, ValueProperty::<Impl, OFFSET>, KeyTimeProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDragItemThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragItemThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IDragItemThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IDragItemThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragItemThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDragItemThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragItemThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDragItemThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragItemThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOverThemeAnimationImpl: Sized {
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ToOffset(&self) -> ::windows::core::Result<f64>;
    fn SetToOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragOverThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragOverThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDragOverThemeAnimationVtbl {
    pub const fn new<Impl: IDragOverThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragOverThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IDragOverThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IDragOverThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ToOffset<Impl: IDragOverThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToOffset<Impl: IDragOverThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToOffset(value).into()
        }
        unsafe extern "system" fn Direction<Impl: IDragOverThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Impl: IDragOverThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDirection(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragOverThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>, ToOffset::<Impl, OFFSET>, SetToOffset::<Impl, OFFSET>, Direction::<Impl, OFFSET>, SetDirection::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDragOverThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragOverThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDragOverThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToOffsetProperty<Impl: IDragOverThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionProperty<Impl: IDragOverThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragOverThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>, ToOffsetProperty::<Impl, OFFSET>, DirectionProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDrillInNavigationTransitionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDrillInNavigationTransitionInfoVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDrillInNavigationTransitionInfo>, base.5)
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
    pub const fn new<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDrillInThemeAnimationVtbl {
        unsafe extern "system" fn EntranceTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEntranceTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EntranceTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEntranceTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTargetName<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTarget<Impl: IDrillInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDrillInThemeAnimation>, base.5, EntranceTargetName::<Impl, OFFSET>, SetEntranceTargetName::<Impl, OFFSET>, EntranceTarget::<Impl, OFFSET>, SetEntranceTarget::<Impl, OFFSET>, ExitTargetName::<Impl, OFFSET>, SetExitTargetName::<Impl, OFFSET>, ExitTarget::<Impl, OFFSET>, SetExitTarget::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDrillInThemeAnimationStaticsVtbl {
        unsafe extern "system" fn EntranceTargetNameProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntranceTargetProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetNameProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetProperty<Impl: IDrillInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDrillInThemeAnimationStatics>, base.5, EntranceTargetNameProperty::<Impl, OFFSET>, EntranceTargetProperty::<Impl, OFFSET>, ExitTargetNameProperty::<Impl, OFFSET>, ExitTargetProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDrillOutThemeAnimationVtbl {
        unsafe extern "system" fn EntranceTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEntranceTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EntranceTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEntranceTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEntranceTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTargetName<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitTarget<Impl: IDrillOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDrillOutThemeAnimation>, base.5, EntranceTargetName::<Impl, OFFSET>, SetEntranceTargetName::<Impl, OFFSET>, EntranceTarget::<Impl, OFFSET>, SetEntranceTarget::<Impl, OFFSET>, ExitTargetName::<Impl, OFFSET>, SetExitTargetName::<Impl, OFFSET>, ExitTarget::<Impl, OFFSET>, SetExitTarget::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDrillOutThemeAnimationStaticsVtbl {
        unsafe extern "system" fn EntranceTargetNameProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntranceTargetProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntranceTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetNameProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitTargetProperty<Impl: IDrillOutThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDrillOutThemeAnimationStatics>, base.5, EntranceTargetNameProperty::<Impl, OFFSET>, EntranceTargetProperty::<Impl, OFFSET>, ExitTargetNameProperty::<Impl, OFFSET>, ExitTargetProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDropTargetItemThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDropTargetItemThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IDropTargetItemThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IDropTargetItemThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDropTargetItemThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDropTargetItemThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDropTargetItemThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDropTargetItemThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDropTargetItemThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingColorKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingColorKeyFrameVtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingColorKeyFrame>, base.5, EasingFunction::<Impl, OFFSET>, SetEasingFunction::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingColorKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingColorKeyFrameStaticsVtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingColorKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingColorKeyFrameStatics>, base.5, EasingFunctionProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingDoubleKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingDoubleKeyFrameVtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingDoubleKeyFrame>, base.5, EasingFunction::<Impl, OFFSET>, SetEasingFunction::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingDoubleKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingDoubleKeyFrameStaticsVtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingDoubleKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingDoubleKeyFrameStatics>, base.5, EasingFunctionProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingFunctionBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingFunctionBaseVtbl {
        unsafe extern "system" fn EasingMode<Impl: IEasingFunctionBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EasingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingMode<Impl: IEasingFunctionBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: EasingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEasingMode(value).into()
        }
        unsafe extern "system" fn Ease<Impl: IEasingFunctionBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, normalizedtime: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ease(normalizedtime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingFunctionBase>, base.5, EasingMode::<Impl, OFFSET>, SetEasingMode::<Impl, OFFSET>, Ease::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingFunctionBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingFunctionBaseFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingFunctionBaseFactory>, base.5)
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
    pub const fn new<Impl: IEasingFunctionBaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingFunctionBaseStaticsVtbl {
        unsafe extern "system" fn EasingModeProperty<Impl: IEasingFunctionBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingFunctionBaseStatics>, base.5, EasingModeProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingPointKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingPointKeyFrameVtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingPointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingPointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingPointKeyFrame>, base.5, EasingFunction::<Impl, OFFSET>, SetEasingFunction::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEasingPointKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEasingPointKeyFrameStaticsVtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingPointKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEasingPointKeyFrameStatics>, base.5, EasingFunctionProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeUIThemeTransitionImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeUIThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEdgeUIThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeUIThemeTransitionVtbl {
    pub const fn new<Impl: IEdgeUIThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEdgeUIThemeTransitionVtbl {
        unsafe extern "system" fn Edge<Impl: IEdgeUIThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEdge<Impl: IEdgeUIThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEdge(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEdgeUIThemeTransition>, base.5, Edge::<Impl, OFFSET>, SetEdge::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEdgeUIThemeTransitionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEdgeUIThemeTransitionStaticsVtbl {
        unsafe extern "system" fn EdgeProperty<Impl: IEdgeUIThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EdgeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEdgeUIThemeTransitionStatics>, base.5, EdgeProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IElasticEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElasticEaseVtbl {
        unsafe extern "system" fn Oscillations<Impl: IElasticEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Oscillations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOscillations<Impl: IElasticEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOscillations(value).into()
        }
        unsafe extern "system" fn Springiness<Impl: IElasticEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Springiness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpringiness<Impl: IElasticEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpringiness(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElasticEase>, base.5, Oscillations::<Impl, OFFSET>, SetOscillations::<Impl, OFFSET>, Springiness::<Impl, OFFSET>, SetSpringiness::<Impl, OFFSET>)
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
    pub const fn new<Impl: IElasticEaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElasticEaseStaticsVtbl {
        unsafe extern "system" fn OscillationsProperty<Impl: IElasticEaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OscillationsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpringinessProperty<Impl: IElasticEaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpringinessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElasticEaseStatics>, base.5, OscillationsProperty::<Impl, OFFSET>, SpringinessProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEntranceNavigationTransitionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEntranceNavigationTransitionInfoVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEntranceNavigationTransitionInfo>, base.5)
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
    pub const fn new<Impl: IEntranceNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEntranceNavigationTransitionInfoStaticsVtbl {
        unsafe extern "system" fn IsTargetElementProperty<Impl: IEntranceNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTargetElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsTargetElement<Impl: IEntranceNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsTargetElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTargetElement<Impl: IEntranceNavigationTransitionInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsTargetElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEntranceNavigationTransitionInfoStatics>, base.5, IsTargetElementProperty::<Impl, OFFSET>, GetIsTargetElement::<Impl, OFFSET>, SetIsTargetElement::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEntranceThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEntranceThemeTransitionVtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IEntranceThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        unsafe extern "system" fn IsStaggeringEnabled<Impl: IEntranceThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: IEntranceThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEntranceThemeTransition>, base.5, FromHorizontalOffset::<Impl, OFFSET>, SetFromHorizontalOffset::<Impl, OFFSET>, FromVerticalOffset::<Impl, OFFSET>, SetFromVerticalOffset::<Impl, OFFSET>, IsStaggeringEnabled::<Impl, OFFSET>, SetIsStaggeringEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: IEntranceThemeTransitionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEntranceThemeTransitionStaticsVtbl {
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IEntranceThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IEntranceThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: IEntranceThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEntranceThemeTransitionStatics>, base.5, FromHorizontalOffsetProperty::<Impl, OFFSET>, FromVerticalOffsetProperty::<Impl, OFFSET>, IsStaggeringEnabledProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IExponentialEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExponentialEaseVtbl {
        unsafe extern "system" fn Exponent<Impl: IExponentialEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exponent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExponent<Impl: IExponentialEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExponent(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExponentialEase>, base.5, Exponent::<Impl, OFFSET>, SetExponent::<Impl, OFFSET>)
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
    pub const fn new<Impl: IExponentialEaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExponentialEaseStaticsVtbl {
        unsafe extern "system" fn ExponentProperty<Impl: IExponentialEaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExponentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExponentialEaseStatics>, base.5, ExponentProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFadeInThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFadeInThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IFadeInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IFadeInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFadeInThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFadeInThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFadeInThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IFadeInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFadeInThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFadeOutThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFadeOutThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IFadeOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IFadeOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFadeOutThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFadeOutThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFadeOutThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IFadeOutThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFadeOutThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGravityConnectedAnimationConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGravityConnectedAnimationConfigurationVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGravityConnectedAnimationConfiguration>, base.5)
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
    pub const fn new<Impl: IGravityConnectedAnimationConfiguration2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGravityConnectedAnimationConfiguration2Vtbl {
        unsafe extern "system" fn IsShadowEnabled<Impl: IGravityConnectedAnimationConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsShadowEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsShadowEnabled<Impl: IGravityConnectedAnimationConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsShadowEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGravityConnectedAnimationConfiguration2>, base.5, IsShadowEnabled::<Impl, OFFSET>, SetIsShadowEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGravityConnectedAnimationConfigurationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGravityConnectedAnimationConfigurationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGravityConnectedAnimationConfigurationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGravityConnectedAnimationConfigurationFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeySplineImpl: Sized {
    fn ControlPoint1(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint1(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn ControlPoint2(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint2(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeySpline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeySpline";
}
#[cfg(feature = "implement_exclusive")]
impl IKeySplineVtbl {
    pub const fn new<Impl: IKeySplineImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeySplineVtbl {
        unsafe extern "system" fn ControlPoint1<Impl: IKeySplineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlPoint1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlPoint1<Impl: IKeySplineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetControlPoint1(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlPoint2<Impl: IKeySplineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlPoint2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlPoint2<Impl: IKeySplineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetControlPoint2(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeySpline>, base.5, ControlPoint1::<Impl, OFFSET>, SetControlPoint1::<Impl, OFFSET>, ControlPoint2::<Impl, OFFSET>, SetControlPoint2::<Impl, OFFSET>)
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
    pub const fn new<Impl: IKeyTimeHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyTimeHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyTimeHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyTimeHelperStaticsImpl: Sized {
    fn FromTimeSpan(&self, timespan: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<KeyTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyTimeHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeyTimeHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyTimeHelperStaticsVtbl {
    pub const fn new<Impl: IKeyTimeHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyTimeHelperStaticsVtbl {
        unsafe extern "system" fn FromTimeSpan<Impl: IKeyTimeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timespan: super::super::super::super::Foundation::TimeSpan, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromTimeSpan(&*(&timespan as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyTimeHelperStatics>, base.5, FromTimeSpan::<Impl, OFFSET>)
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
    pub const fn new<Impl: ILinearColorKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILinearColorKeyFrameVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILinearColorKeyFrame>, base.5)
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
    pub const fn new<Impl: ILinearDoubleKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILinearDoubleKeyFrameVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILinearDoubleKeyFrame>, base.5)
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
    pub const fn new<Impl: ILinearPointKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILinearPointKeyFrameVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILinearPointKeyFrame>, base.5)
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
    pub const fn new<Impl: INavigationThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationThemeTransitionVtbl {
        unsafe extern "system" fn DefaultNavigationTransitionInfo<Impl: INavigationThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultNavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultNavigationTransitionInfo<Impl: INavigationThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultNavigationTransitionInfo(&*(&value as *const <NavigationTransitionInfo as ::windows::core::Abi>::Abi as *const <NavigationTransitionInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationThemeTransition>, base.5, DefaultNavigationTransitionInfo::<Impl, OFFSET>, SetDefaultNavigationTransitionInfo::<Impl, OFFSET>)
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
    pub const fn new<Impl: INavigationThemeTransitionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationThemeTransitionStaticsVtbl {
        unsafe extern "system" fn DefaultNavigationTransitionInfoProperty<Impl: INavigationThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultNavigationTransitionInfoProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationThemeTransitionStatics>, base.5, DefaultNavigationTransitionInfoProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: INavigationTransitionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationTransitionInfoVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationTransitionInfo>, base.5)
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
    pub const fn new<Impl: INavigationTransitionInfoFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationTransitionInfoFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INavigationTransitionInfoFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationTransitionInfoFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: INavigationTransitionInfoOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationTransitionInfoOverridesVtbl {
        unsafe extern "system" fn GetNavigationStateCore<Impl: INavigationTransitionInfoOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNavigationStateCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNavigationStateCore<Impl: INavigationTransitionInfoOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, navigationstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNavigationStateCore(&*(&navigationstate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationTransitionInfoOverrides>, base.5, GetNavigationStateCore::<Impl, OFFSET>, SetNavigationStateCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<ObjectKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectAnimationUsingKeyFrames";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectAnimationUsingKeyFramesVtbl {
    pub const fn new<Impl: IObjectAnimationUsingKeyFramesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IObjectAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IObjectAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IObjectAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectAnimationUsingKeyFrames>, base.5, KeyFrames::<Impl, OFFSET>, EnableDependentAnimation::<Impl, OFFSET>, SetEnableDependentAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IObjectAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IObjectAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectAnimationUsingKeyFramesStatics>, base.5, EnableDependentAnimationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectKeyFrameVtbl {
    pub const fn new<Impl: IObjectKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IObjectKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IObjectKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IObjectKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IObjectKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectKeyFrame>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, KeyTime::<Impl, OFFSET>, SetKeyTime::<Impl, OFFSET>)
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
    pub const fn new<Impl: IObjectKeyFrameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IObjectKeyFrameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectKeyFrameFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IObjectKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IObjectKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IObjectKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectKeyFrameStatics>, base.5, ValueProperty::<Impl, OFFSET>, KeyTimeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaneThemeTransitionImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaneThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPaneThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IPaneThemeTransitionVtbl {
    pub const fn new<Impl: IPaneThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaneThemeTransitionVtbl {
        unsafe extern "system" fn Edge<Impl: IPaneThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEdge<Impl: IPaneThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEdge(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaneThemeTransition>, base.5, Edge::<Impl, OFFSET>, SetEdge::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPaneThemeTransitionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaneThemeTransitionStaticsVtbl {
        unsafe extern "system" fn EdgeProperty<Impl: IPaneThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EdgeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaneThemeTransitionStatics>, base.5, EdgeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPointAnimationVtbl {
    pub const fn new<Impl: IPointAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointAnimationVtbl {
        unsafe extern "system" fn From<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).By() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBy<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEasingFunction<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IPointAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointAnimation>, base.5, From::<Impl, OFFSET>, SetFrom::<Impl, OFFSET>, To::<Impl, OFFSET>, SetTo::<Impl, OFFSET>, By::<Impl, OFFSET>, SetBy::<Impl, OFFSET>, EasingFunction::<Impl, OFFSET>, SetEasingFunction::<Impl, OFFSET>, EnableDependentAnimation::<Impl, OFFSET>, SetEnableDependentAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointAnimationStaticsVtbl {
        unsafe extern "system" fn FromProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ByProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ByProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EasingFunctionProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EasingFunctionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IPointAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointAnimationStatics>, base.5, FromProperty::<Impl, OFFSET>, ToProperty::<Impl, OFFSET>, ByProperty::<Impl, OFFSET>, EasingFunctionProperty::<Impl, OFFSET>, EnableDependentAnimationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationUsingKeyFramesImpl: Sized {
    fn KeyFrames(&self) -> ::windows::core::Result<PointKeyFrameCollection>;
    fn EnableDependentAnimation(&self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimationUsingKeyFrames";
}
#[cfg(feature = "implement_exclusive")]
impl IPointAnimationUsingKeyFramesVtbl {
    pub const fn new<Impl: IPointAnimationUsingKeyFramesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointAnimationUsingKeyFramesVtbl {
        unsafe extern "system" fn KeyFrames<Impl: IPointAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyFrames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IPointAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IPointAnimationUsingKeyFramesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointAnimationUsingKeyFrames>, base.5, KeyFrames::<Impl, OFFSET>, EnableDependentAnimation::<Impl, OFFSET>, SetEnableDependentAnimation::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointAnimationUsingKeyFramesStaticsVtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IPointAnimationUsingKeyFramesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableDependentAnimationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointAnimationUsingKeyFramesStatics>, base.5, EnableDependentAnimationProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetValue(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn KeyTime(&self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IPointKeyFrameVtbl {
    pub const fn new<Impl: IPointKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointKeyFrameVtbl {
        unsafe extern "system" fn Value<Impl: IPointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IPointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTime<Impl: IPointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointKeyFrame>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, KeyTime::<Impl, OFFSET>, SetKeyTime::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointKeyFrameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointKeyFrameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPointKeyFrameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointKeyFrameFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointKeyFrameStaticsVtbl {
        unsafe extern "system" fn ValueProperty<Impl: IPointKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTimeProperty<Impl: IPointKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointKeyFrameStatics>, base.5, ValueProperty::<Impl, OFFSET>, KeyTimeProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointerDownThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerDownThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPointerDownThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPointerDownThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerDownThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointerDownThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerDownThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPointerDownThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerDownThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointerUpThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerUpThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPointerUpThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPointerUpThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerUpThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPointerUpThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerUpThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPointerUpThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerUpThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPopInThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopInThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPopInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPopInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IPopInThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopInThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>, FromHorizontalOffset::<Impl, OFFSET>, SetFromHorizontalOffset::<Impl, OFFSET>, FromVerticalOffset::<Impl, OFFSET>, SetFromVerticalOffset::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPopInThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopInThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPopInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IPopInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IPopInThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopInThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>, FromHorizontalOffsetProperty::<Impl, OFFSET>, FromVerticalOffsetProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPopOutThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopOutThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IPopOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IPopOutThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopOutThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPopOutThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopOutThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPopOutThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopOutThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPopupThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopupThemeTransitionVtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IPopupThemeTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopupThemeTransition>, base.5, FromHorizontalOffset::<Impl, OFFSET>, SetFromHorizontalOffset::<Impl, OFFSET>, FromVerticalOffset::<Impl, OFFSET>, SetFromVerticalOffset::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPopupThemeTransitionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopupThemeTransitionStaticsVtbl {
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IPopupThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IPopupThemeTransitionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopupThemeTransitionStatics>, base.5, FromHorizontalOffsetProperty::<Impl, OFFSET>, FromVerticalOffsetProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPowerEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPowerEaseVtbl {
        unsafe extern "system" fn Power<Impl: IPowerEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Power() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPower<Impl: IPowerEaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPower(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPowerEase>, base.5, Power::<Impl, OFFSET>, SetPower::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPowerEaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPowerEaseStaticsVtbl {
        unsafe extern "system" fn PowerProperty<Impl: IPowerEaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPowerEaseStatics>, base.5, PowerProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IQuadraticEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQuadraticEaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQuadraticEase>, base.5)
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
    pub const fn new<Impl: IQuarticEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQuarticEaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQuarticEase>, base.5)
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
    pub const fn new<Impl: IQuinticEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IQuinticEaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IQuinticEase>, base.5)
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
    pub const fn new<Impl: IReorderThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReorderThemeTransitionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReorderThemeTransition>, base.5)
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
    pub const fn new<Impl: IRepeatBehaviorHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepeatBehaviorHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepeatBehaviorHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatBehaviorHelperStaticsImpl: Sized {
    fn Forever(&self) -> ::windows::core::Result<RepeatBehavior>;
    fn FromCount(&self, count: f64) -> ::windows::core::Result<RepeatBehavior>;
    fn FromDuration(&self, duration: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<RepeatBehavior>;
    fn GetHasCount(&self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn GetHasDuration(&self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &RepeatBehavior, value: &RepeatBehavior) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatBehaviorHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepeatBehaviorHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatBehaviorHelperStaticsVtbl {
    pub const fn new<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepeatBehaviorHelperStaticsVtbl {
        unsafe extern "system" fn Forever<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forever() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromCount<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: f64, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromCount(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromDuration<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: super::super::super::super::Foundation::TimeSpan, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromDuration(&*(&duration as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCount<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHasCount(&*(&target as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasDuration<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHasDuration(&*(&target as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IRepeatBehaviorHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, value: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepeatBehaviorHelperStatics>, base.5, Forever::<Impl, OFFSET>, FromCount::<Impl, OFFSET>, FromDuration::<Impl, OFFSET>, GetHasCount::<Impl, OFFSET>, GetHasDuration::<Impl, OFFSET>, Equals::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRepositionThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepositionThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: IRepositionThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IRepositionThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IRepositionThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepositionThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>, FromHorizontalOffset::<Impl, OFFSET>, SetFromHorizontalOffset::<Impl, OFFSET>, FromVerticalOffset::<Impl, OFFSET>, SetFromVerticalOffset::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRepositionThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepositionThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IRepositionThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IRepositionThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IRepositionThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepositionThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>, FromHorizontalOffsetProperty::<Impl, OFFSET>, FromVerticalOffsetProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRepositionThemeTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepositionThemeTransitionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepositionThemeTransition>, base.5)
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
    pub const fn new<Impl: IRepositionThemeTransition2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepositionThemeTransition2Vtbl {
        unsafe extern "system" fn IsStaggeringEnabled<Impl: IRepositionThemeTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: IRepositionThemeTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepositionThemeTransition2>, base.5, IsStaggeringEnabled::<Impl, OFFSET>, SetIsStaggeringEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRepositionThemeTransitionStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepositionThemeTransitionStatics2Vtbl {
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: IRepositionThemeTransitionStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStaggeringEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepositionThemeTransitionStatics2>, base.5, IsStaggeringEnabledProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISineEaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISineEaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISineEase>, base.5)
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
    pub const fn new<Impl: ISlideNavigationTransitionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISlideNavigationTransitionInfoVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISlideNavigationTransitionInfo>, base.5)
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
    pub const fn new<Impl: ISlideNavigationTransitionInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISlideNavigationTransitionInfo2Vtbl {
        unsafe extern "system" fn Effect<Impl: ISlideNavigationTransitionInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SlideNavigationTransitionEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Effect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffect<Impl: ISlideNavigationTransitionInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SlideNavigationTransitionEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEffect(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISlideNavigationTransitionInfo2>, base.5, Effect::<Impl, OFFSET>, SetEffect::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISlideNavigationTransitionInfoStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISlideNavigationTransitionInfoStatics2Vtbl {
        unsafe extern "system" fn EffectProperty<Impl: ISlideNavigationTransitionInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EffectProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISlideNavigationTransitionInfoStatics2>, base.5, EffectProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISplineColorKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplineColorKeyFrameVtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplineColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeySpline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpline<Impl: ISplineColorKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplineColorKeyFrame>, base.5, KeySpline::<Impl, OFFSET>, SetKeySpline::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISplineColorKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplineColorKeyFrameStaticsVtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplineColorKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeySplineProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplineColorKeyFrameStatics>, base.5, KeySplineProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISplineDoubleKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplineDoubleKeyFrameVtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplineDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeySpline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpline<Impl: ISplineDoubleKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplineDoubleKeyFrame>, base.5, KeySpline::<Impl, OFFSET>, SetKeySpline::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISplineDoubleKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplineDoubleKeyFrameStaticsVtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplineDoubleKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeySplineProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplineDoubleKeyFrameStatics>, base.5, KeySplineProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISplinePointKeyFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplinePointKeyFrameVtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplinePointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeySpline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpline<Impl: ISplinePointKeyFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplinePointKeyFrame>, base.5, KeySpline::<Impl, OFFSET>, SetKeySpline::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISplinePointKeyFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplinePointKeyFrameStaticsVtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplinePointKeyFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeySplineProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplinePointKeyFrameStatics>, base.5, KeySplineProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitCloseThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitCloseThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitCloseThemeAnimationVtbl {
    pub const fn new<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplitCloseThemeAnimationVtbl {
        unsafe extern "system" fn OpenedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpenedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpenedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClosedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClosedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTargetName<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTarget<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpenedLength(value).into()
        }
        unsafe extern "system" fn ClosedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedLength<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClosedLength(value).into()
        }
        unsafe extern "system" fn OffsetFromCenter<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetFromCenter<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOffsetFromCenter(value).into()
        }
        unsafe extern "system" fn ContentTranslationDirection<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationDirection<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTranslationDirection(value).into()
        }
        unsafe extern "system" fn ContentTranslationOffset<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTranslationOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationOffset<Impl: ISplitCloseThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTranslationOffset(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISplitCloseThemeAnimation>,
            base.5,
            OpenedTargetName::<Impl, OFFSET>,
            SetOpenedTargetName::<Impl, OFFSET>,
            OpenedTarget::<Impl, OFFSET>,
            SetOpenedTarget::<Impl, OFFSET>,
            ClosedTargetName::<Impl, OFFSET>,
            SetClosedTargetName::<Impl, OFFSET>,
            ClosedTarget::<Impl, OFFSET>,
            SetClosedTarget::<Impl, OFFSET>,
            ContentTargetName::<Impl, OFFSET>,
            SetContentTargetName::<Impl, OFFSET>,
            ContentTarget::<Impl, OFFSET>,
            SetContentTarget::<Impl, OFFSET>,
            OpenedLength::<Impl, OFFSET>,
            SetOpenedLength::<Impl, OFFSET>,
            ClosedLength::<Impl, OFFSET>,
            SetClosedLength::<Impl, OFFSET>,
            OffsetFromCenter::<Impl, OFFSET>,
            SetOffsetFromCenter::<Impl, OFFSET>,
            ContentTranslationDirection::<Impl, OFFSET>,
            SetContentTranslationDirection::<Impl, OFFSET>,
            ContentTranslationOffset::<Impl, OFFSET>,
            SetContentTranslationOffset::<Impl, OFFSET>,
        )
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
    pub const fn new<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplitCloseThemeAnimationStaticsVtbl {
        unsafe extern "system" fn OpenedTargetNameProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedTargetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetNameProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetNameProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedLengthProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedLengthProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromCenterProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationDirectionProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationOffsetProperty<Impl: ISplitCloseThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISplitCloseThemeAnimationStatics>,
            base.5,
            OpenedTargetNameProperty::<Impl, OFFSET>,
            OpenedTargetProperty::<Impl, OFFSET>,
            ClosedTargetNameProperty::<Impl, OFFSET>,
            ClosedTargetProperty::<Impl, OFFSET>,
            ContentTargetNameProperty::<Impl, OFFSET>,
            ContentTargetProperty::<Impl, OFFSET>,
            OpenedLengthProperty::<Impl, OFFSET>,
            ClosedLengthProperty::<Impl, OFFSET>,
            OffsetFromCenterProperty::<Impl, OFFSET>,
            ContentTranslationDirectionProperty::<Impl, OFFSET>,
            ContentTranslationOffsetProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitOpenThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitOpenThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitOpenThemeAnimationVtbl {
    pub const fn new<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplitOpenThemeAnimationVtbl {
        unsafe extern "system" fn OpenedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpenedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpenedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClosedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClosedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTargetName<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTarget<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpenedLength(value).into()
        }
        unsafe extern "system" fn ClosedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosedLength<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClosedLength(value).into()
        }
        unsafe extern "system" fn OffsetFromCenter<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffsetFromCenter<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOffsetFromCenter(value).into()
        }
        unsafe extern "system" fn ContentTranslationDirection<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationDirection<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTranslationDirection(value).into()
        }
        unsafe extern "system" fn ContentTranslationOffset<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTranslationOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTranslationOffset<Impl: ISplitOpenThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTranslationOffset(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISplitOpenThemeAnimation>,
            base.5,
            OpenedTargetName::<Impl, OFFSET>,
            SetOpenedTargetName::<Impl, OFFSET>,
            OpenedTarget::<Impl, OFFSET>,
            SetOpenedTarget::<Impl, OFFSET>,
            ClosedTargetName::<Impl, OFFSET>,
            SetClosedTargetName::<Impl, OFFSET>,
            ClosedTarget::<Impl, OFFSET>,
            SetClosedTarget::<Impl, OFFSET>,
            ContentTargetName::<Impl, OFFSET>,
            SetContentTargetName::<Impl, OFFSET>,
            ContentTarget::<Impl, OFFSET>,
            SetContentTarget::<Impl, OFFSET>,
            OpenedLength::<Impl, OFFSET>,
            SetOpenedLength::<Impl, OFFSET>,
            ClosedLength::<Impl, OFFSET>,
            SetClosedLength::<Impl, OFFSET>,
            OffsetFromCenter::<Impl, OFFSET>,
            SetOffsetFromCenter::<Impl, OFFSET>,
            ContentTranslationDirection::<Impl, OFFSET>,
            SetContentTranslationDirection::<Impl, OFFSET>,
            ContentTranslationOffset::<Impl, OFFSET>,
            SetContentTranslationOffset::<Impl, OFFSET>,
        )
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
    pub const fn new<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplitOpenThemeAnimationStaticsVtbl {
        unsafe extern "system" fn OpenedTargetNameProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedTargetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetNameProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedTargetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetNameProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTargetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenedLengthProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedLengthProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClosedLengthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromCenterProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OffsetFromCenterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationDirectionProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTranslationDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTranslationOffsetProperty<Impl: ISplitOpenThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISplitOpenThemeAnimationStatics>,
            base.5,
            OpenedTargetNameProperty::<Impl, OFFSET>,
            OpenedTargetProperty::<Impl, OFFSET>,
            ClosedTargetNameProperty::<Impl, OFFSET>,
            ClosedTargetProperty::<Impl, OFFSET>,
            ContentTargetNameProperty::<Impl, OFFSET>,
            ContentTargetProperty::<Impl, OFFSET>,
            OpenedLengthProperty::<Impl, OFFSET>,
            ClosedLengthProperty::<Impl, OFFSET>,
            OffsetFromCenterProperty::<Impl, OFFSET>,
            ContentTranslationDirectionProperty::<Impl, OFFSET>,
            ContentTranslationOffsetProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IStoryboard";
}
#[cfg(feature = "implement_exclusive")]
impl IStoryboardVtbl {
    pub const fn new<Impl: IStoryboardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStoryboardVtbl {
        unsafe extern "system" fn Children<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Seek(&*(&offset as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stop<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Begin<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Begin().into()
        }
        unsafe extern "system" fn Pause<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn GetCurrentState<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ClockState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTime<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeekAlignedToLastTick<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SeekAlignedToLastTick(&*(&offset as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SkipToFill<Impl: IStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SkipToFill().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStoryboard>, base.5, Children::<Impl, OFFSET>, Seek::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Begin::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Resume::<Impl, OFFSET>, GetCurrentState::<Impl, OFFSET>, GetCurrentTime::<Impl, OFFSET>, SeekAlignedToLastTick::<Impl, OFFSET>, SkipToFill::<Impl, OFFSET>)
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
    pub const fn new<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStoryboardStaticsVtbl {
        unsafe extern "system" fn TargetPropertyProperty<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetPropertyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetProperty<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTargetProperty(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetProperty<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetProperty(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetNameProperty<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetName<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTargetName(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetTarget<Impl: IStoryboardStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeline: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&timeline as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&target as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStoryboardStatics>, base.5, TargetPropertyProperty::<Impl, OFFSET>, GetTargetProperty::<Impl, OFFSET>, SetTargetProperty::<Impl, OFFSET>, TargetNameProperty::<Impl, OFFSET>, GetTargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>, SetTarget::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISuppressNavigationTransitionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISuppressNavigationTransitionInfoVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISuppressNavigationTransitionInfo>, base.5)
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
    pub const fn new<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwipeBackThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromVerticalOffset<Impl: ISwipeBackThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwipeBackThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>, FromHorizontalOffset::<Impl, OFFSET>, SetFromHorizontalOffset::<Impl, OFFSET>, FromVerticalOffset::<Impl, OFFSET>, SetFromVerticalOffset::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISwipeBackThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwipeBackThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: ISwipeBackThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: ISwipeBackThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: ISwipeBackThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwipeBackThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>, FromHorizontalOffsetProperty::<Impl, OFFSET>, FromVerticalOffsetProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwipeHintThemeAnimationVtbl {
        unsafe extern "system" fn TargetName<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ToHorizontalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToHorizontalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToHorizontalOffset(value).into()
        }
        unsafe extern "system" fn ToVerticalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToVerticalOffset<Impl: ISwipeHintThemeAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToVerticalOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwipeHintThemeAnimation>, base.5, TargetName::<Impl, OFFSET>, SetTargetName::<Impl, OFFSET>, ToHorizontalOffset::<Impl, OFFSET>, SetToHorizontalOffset::<Impl, OFFSET>, ToVerticalOffset::<Impl, OFFSET>, SetToVerticalOffset::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISwipeHintThemeAnimationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwipeHintThemeAnimationStaticsVtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: ISwipeHintThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetNameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToHorizontalOffsetProperty<Impl: ISwipeHintThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVerticalOffsetProperty<Impl: ISwipeHintThemeAnimationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwipeHintThemeAnimationStatics>, base.5, TargetNameProperty::<Impl, OFFSET>, ToHorizontalOffsetProperty::<Impl, OFFSET>, ToVerticalOffsetProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimeline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITimeline";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineVtbl {
    pub const fn new<Impl: ITimelineImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimelineVtbl {
        unsafe extern "system" fn AutoReverse<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoReverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReverse<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoReverse(value).into()
        }
        unsafe extern "system" fn BeginTime<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBeginTime<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBeginTime(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Duration as ::windows::core::Abi>::Abi as *const <super::super::Duration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpeedRatio<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpeedRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeedRatio<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpeedRatio(value).into()
        }
        unsafe extern "system" fn FillBehavior<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FillBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBehavior<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FillBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFillBehavior(value).into()
        }
        unsafe extern "system" fn RepeatBehavior<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RepeatBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepeatBehavior<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRepeatBehavior(&*(&value as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: ITimelineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITimeline>,
            base.5,
            AutoReverse::<Impl, OFFSET>,
            SetAutoReverse::<Impl, OFFSET>,
            BeginTime::<Impl, OFFSET>,
            SetBeginTime::<Impl, OFFSET>,
            Duration::<Impl, OFFSET>,
            SetDuration::<Impl, OFFSET>,
            SpeedRatio::<Impl, OFFSET>,
            SetSpeedRatio::<Impl, OFFSET>,
            FillBehavior::<Impl, OFFSET>,
            SetFillBehavior::<Impl, OFFSET>,
            RepeatBehavior::<Impl, OFFSET>,
            SetRepeatBehavior::<Impl, OFFSET>,
            Completed::<Impl, OFFSET>,
            RemoveCompleted::<Impl, OFFSET>,
        )
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
    pub const fn new<Impl: ITimelineFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimelineFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITimelineFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimelineFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITimelineStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimelineStaticsVtbl {
        unsafe extern "system" fn AllowDependentAnimations<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowDependentAnimations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowDependentAnimations<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowDependentAnimations(value).into()
        }
        unsafe extern "system" fn AutoReverseProperty<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoReverseProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginTimeProperty<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginTimeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DurationProperty<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DurationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeedRatioProperty<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpeedRatioProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillBehaviorProperty<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillBehaviorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RepeatBehaviorProperty<Impl: ITimelineStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RepeatBehaviorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimelineStatics>, base.5, AllowDependentAnimations::<Impl, OFFSET>, SetAllowDependentAnimations::<Impl, OFFSET>, AutoReverseProperty::<Impl, OFFSET>, BeginTimeProperty::<Impl, OFFSET>, DurationProperty::<Impl, OFFSET>, SpeedRatioProperty::<Impl, OFFSET>, FillBehaviorProperty::<Impl, OFFSET>, RepeatBehaviorProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransitionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransition>, base.5)
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
    pub const fn new<Impl: ITransitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITransitionFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITransitionFactory>, base.5)
    }
}
