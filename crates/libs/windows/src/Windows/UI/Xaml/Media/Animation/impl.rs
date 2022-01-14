#[cfg(feature = "implement_exclusive")]
pub trait IAddDeleteThemeTransition_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAddDeleteThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IAddDeleteThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IAddDeleteThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddDeleteThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAddDeleteThemeTransition_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAddDeleteThemeTransition, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddDeleteThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEase_Impl: Sized {
    fn Amplitude(&mut self) -> ::windows::core::Result<f64>;
    fn SetAmplitude(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBackEase";
}
#[cfg(feature = "implement_exclusive")]
impl IBackEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackEase_Vtbl {
        unsafe extern "system" fn Amplitude<Impl: IBackEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAmplitude<Impl: IBackEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAmplitude(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackEase, BASE_OFFSET>(),
            Amplitude: Amplitude::<Impl, IMPL_OFFSET>,
            SetAmplitude: SetAmplitude::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEaseStatics_Impl: Sized {
    fn AmplitudeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBackEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackEaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackEaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackEaseStatics_Vtbl {
        unsafe extern "system" fn AmplitudeProperty<Impl: IBackEaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackEaseStatics, BASE_OFFSET>(),
            AmplitudeProperty: AmplitudeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBasicConnectedAnimationConfiguration_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBasicConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBasicConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IBasicConnectedAnimationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicConnectedAnimationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicConnectedAnimationConfiguration_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBasicConnectedAnimationConfiguration, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBasicConnectedAnimationConfigurationFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BasicConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBasicConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBasicConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBasicConnectedAnimationConfigurationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicConnectedAnimationConfigurationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicConnectedAnimationConfigurationFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBasicConnectedAnimationConfigurationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBasicConnectedAnimationConfigurationFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBeginStoryboard_Impl: Sized {
    fn Storyboard(&mut self) -> ::windows::core::Result<Storyboard>;
    fn SetStoryboard(&mut self, value: &::core::option::Option<Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBeginStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBeginStoryboard";
}
#[cfg(feature = "implement_exclusive")]
impl IBeginStoryboard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBeginStoryboard_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBeginStoryboard_Vtbl {
        unsafe extern "system" fn Storyboard<Impl: IBeginStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStoryboard<Impl: IBeginStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryboard(&*(&value as *const <Storyboard as ::windows::core::Abi>::Abi as *const <Storyboard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBeginStoryboard, BASE_OFFSET>(),
            Storyboard: Storyboard::<Impl, IMPL_OFFSET>,
            SetStoryboard: SetStoryboard::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBeginStoryboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBeginStoryboardStatics_Impl: Sized {
    fn StoryboardProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBeginStoryboardStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBeginStoryboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBeginStoryboardStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBeginStoryboardStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBeginStoryboardStatics_Vtbl {
        unsafe extern "system" fn StoryboardProperty<Impl: IBeginStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBeginStoryboardStatics, BASE_OFFSET>(),
            StoryboardProperty: StoryboardProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBeginStoryboardStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEase_Impl: Sized {
    fn Bounces(&mut self) -> ::windows::core::Result<i32>;
    fn SetBounces(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Bounciness(&mut self) -> ::windows::core::Result<f64>;
    fn SetBounciness(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBounceEase";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceEase_Vtbl {
        unsafe extern "system" fn Bounces<Impl: IBounceEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBounces<Impl: IBounceEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounces(value).into()
        }
        unsafe extern "system" fn Bounciness<Impl: IBounceEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBounciness<Impl: IBounceEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounciness(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBounceEase, BASE_OFFSET>(),
            Bounces: Bounces::<Impl, IMPL_OFFSET>,
            SetBounces: SetBounces::<Impl, IMPL_OFFSET>,
            Bounciness: Bounciness::<Impl, IMPL_OFFSET>,
            SetBounciness: SetBounciness::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEaseStatics_Impl: Sized {
    fn BouncesProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BouncinessProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IBounceEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceEaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceEaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceEaseStatics_Vtbl {
        unsafe extern "system" fn BouncesProperty<Impl: IBounceEaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BouncinessProperty<Impl: IBounceEaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBounceEaseStatics, BASE_OFFSET>(),
            BouncesProperty: BouncesProperty::<Impl, IMPL_OFFSET>,
            BouncinessProperty: BouncinessProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICircleEase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICircleEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICircleEase";
}
#[cfg(feature = "implement_exclusive")]
impl ICircleEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICircleEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICircleEase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICircleEase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICircleEase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IColorAnimation_Impl: Sized {
    fn From(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetFrom(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn To(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetTo(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn By(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::Color>>;
    fn SetBy(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::Color>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&mut self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&mut self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IColorAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimation_Vtbl {
        unsafe extern "system" fn From<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrom<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTo<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBy<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEasingFunction<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IColorAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorAnimation, BASE_OFFSET>(),
            From: From::<Impl, IMPL_OFFSET>,
            SetFrom: SetFrom::<Impl, IMPL_OFFSET>,
            To: To::<Impl, IMPL_OFFSET>,
            SetTo: SetTo::<Impl, IMPL_OFFSET>,
            By: By::<Impl, IMPL_OFFSET>,
            SetBy: SetBy::<Impl, IMPL_OFFSET>,
            EasingFunction: EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction: SetEasingFunction::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation: EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation: SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationStatics_Impl: Sized {
    fn FromProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimationStatics_Vtbl {
        unsafe extern "system" fn FromProperty<Impl: IColorAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToProperty<Impl: IColorAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ByProperty<Impl: IColorAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EasingFunctionProperty<Impl: IColorAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IColorAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorAnimationStatics, BASE_OFFSET>(),
            FromProperty: FromProperty::<Impl, IMPL_OFFSET>,
            ToProperty: ToProperty::<Impl, IMPL_OFFSET>,
            ByProperty: ByProperty::<Impl, IMPL_OFFSET>,
            EasingFunctionProperty: EasingFunctionProperty::<Impl, IMPL_OFFSET>,
            EnableDependentAnimationProperty: EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IColorAnimationUsingKeyFrames_Impl: Sized {
    fn KeyFrames(&mut self) -> ::windows::core::Result<ColorKeyFrameCollection>;
    fn EnableDependentAnimation(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IColorAnimationUsingKeyFrames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimationUsingKeyFrames_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimationUsingKeyFrames_Vtbl {
        unsafe extern "system" fn KeyFrames<Impl: IColorAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableDependentAnimation<Impl: IColorAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IColorAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorAnimationUsingKeyFrames, BASE_OFFSET>(),
            KeyFrames: KeyFrames::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation: EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation: SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorAnimationUsingKeyFramesStatics_Impl: Sized {
    fn EnableDependentAnimationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorAnimationUsingKeyFramesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorAnimationUsingKeyFramesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorAnimationUsingKeyFramesStatics_Vtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IColorAnimationUsingKeyFramesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorAnimationUsingKeyFramesStatics, BASE_OFFSET>(),
            EnableDependentAnimationProperty: EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IColorKeyFrame_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetValue(&mut self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn KeyTime(&mut self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&mut self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IColorKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrame_Vtbl {
        unsafe extern "system" fn Value<Impl: IColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeyTime<Impl: IColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorKeyFrame, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            KeyTime: KeyTime::<Impl, IMPL_OFFSET>,
            SetKeyTime: SetKeyTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorKeyFrameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrameFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorKeyFrameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorKeyFrameFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameStatics_Impl: Sized {
    fn ValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IColorKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrameStatics_Vtbl {
        unsafe extern "system" fn ValueProperty<Impl: IColorKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyTimeProperty<Impl: IColorKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorKeyFrameStatics, BASE_OFFSET>(),
            ValueProperty: ValueProperty::<Impl, IMPL_OFFSET>,
            KeyTimeProperty: KeyTimeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommonNavigationTransitionInfo_Impl: Sized {
    fn IsStaggeringEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommonNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICommonNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICommonNavigationTransitionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonNavigationTransitionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonNavigationTransitionInfo_Vtbl {
        unsafe extern "system" fn IsStaggeringEnabled<Impl: ICommonNavigationTransitionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: ICommonNavigationTransitionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonNavigationTransitionInfo, BASE_OFFSET>(),
            IsStaggeringEnabled: IsStaggeringEnabled::<Impl, IMPL_OFFSET>,
            SetIsStaggeringEnabled: SetIsStaggeringEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommonNavigationTransitionInfoStatics_Impl: Sized {
    fn IsStaggeringEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStaggerElementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsStaggerElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsStaggerElement(&mut self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommonNavigationTransitionInfoStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICommonNavigationTransitionInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICommonNavigationTransitionInfoStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonNavigationTransitionInfoStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonNavigationTransitionInfoStatics_Vtbl {
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: ICommonNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStaggerElementProperty<Impl: ICommonNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsStaggerElement<Impl: ICommonNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsStaggerElement<Impl: ICommonNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggerElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommonNavigationTransitionInfoStatics, BASE_OFFSET>(),
            IsStaggeringEnabledProperty: IsStaggeringEnabledProperty::<Impl, IMPL_OFFSET>,
            IsStaggerElementProperty: IsStaggerElementProperty::<Impl, IMPL_OFFSET>,
            GetIsStaggerElement: GetIsStaggerElement::<Impl, IMPL_OFFSET>,
            SetIsStaggerElement: SetIsStaggerElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonNavigationTransitionInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectedAnimation_Impl: Sized {
    fn Completed(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<ConnectedAnimation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryStart(&mut self, destination: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectedAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectedAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimation_Vtbl {
        unsafe extern "system" fn Completed<Impl: IConnectedAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCompleted<Impl: IConnectedAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryStart<Impl: IConnectedAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Impl: IConnectedAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectedAnimation, BASE_OFFSET>(),
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
            TryStart: TryStart::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IConnectedAnimation2_Impl: Sized {
    fn IsScaleAnimationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsScaleAnimationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TryStartWithCoordinatedElements(&mut self, destination: &::core::option::Option<super::super::UIElement>, coordinatedelements: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::UIElement>>) -> ::windows::core::Result<bool>;
    fn SetAnimationComponent(&mut self, component: ConnectedAnimationComponent, animation: &::core::option::Option<super::super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectedAnimation2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IConnectedAnimation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimation2_Vtbl {
        unsafe extern "system" fn IsScaleAnimationEnabled<Impl: IConnectedAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsScaleAnimationEnabled<Impl: IConnectedAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsScaleAnimationEnabled(value).into()
        }
        unsafe extern "system" fn TryStartWithCoordinatedElements<Impl: IConnectedAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, coordinatedelements: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAnimationComponent<Impl: IConnectedAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, component: ConnectedAnimationComponent, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimationComponent(component, &*(&animation as *const <super::super::super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectedAnimation2, BASE_OFFSET>(),
            IsScaleAnimationEnabled: IsScaleAnimationEnabled::<Impl, IMPL_OFFSET>,
            SetIsScaleAnimationEnabled: SetIsScaleAnimationEnabled::<Impl, IMPL_OFFSET>,
            TryStartWithCoordinatedElements: TryStartWithCoordinatedElements::<Impl, IMPL_OFFSET>,
            SetAnimationComponent: SetAnimationComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimation3_Impl: Sized {
    fn Configuration(&mut self) -> ::windows::core::Result<ConnectedAnimationConfiguration>;
    fn SetConfiguration(&mut self, value: &::core::option::Option<ConnectedAnimationConfiguration>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimation3 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimation3";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimation3_Vtbl {
        unsafe extern "system" fn Configuration<Impl: IConnectedAnimation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetConfiguration<Impl: IConnectedAnimation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfiguration(&*(&value as *const <ConnectedAnimationConfiguration as ::windows::core::Abi>::Abi as *const <ConnectedAnimationConfiguration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectedAnimation3, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            SetConfiguration: SetConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationConfiguration_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationConfiguration_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectedAnimationConfiguration, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationConfigurationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationConfigurationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationConfigurationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationConfigurationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectedAnimationConfigurationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IConnectedAnimationService_Impl: Sized {
    fn DefaultDuration(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SetDefaultDuration(&mut self, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DefaultEasingFunction(&mut self) -> ::windows::core::Result<super::super::super::Composition::CompositionEasingFunction>;
    fn SetDefaultEasingFunction(&mut self, value: &::core::option::Option<super::super::super::Composition::CompositionEasingFunction>) -> ::windows::core::Result<()>;
    fn PrepareToAnimate(&mut self, key: &::windows::core::HSTRING, source: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<ConnectedAnimation>;
    fn GetAnimation(&mut self, key: &::windows::core::HSTRING) -> ::windows::core::Result<ConnectedAnimation>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectedAnimationService {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationService";
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IConnectedAnimationService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationService_Vtbl {
        unsafe extern "system" fn DefaultDuration<Impl: IConnectedAnimationService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDefaultDuration<Impl: IConnectedAnimationService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultDuration(&*(&value as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultEasingFunction<Impl: IConnectedAnimationService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDefaultEasingFunction<Impl: IConnectedAnimationService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultEasingFunction(&*(&value as *const <super::super::super::Composition::CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <super::super::super::Composition::CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrepareToAnimate<Impl: IConnectedAnimationService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAnimation<Impl: IConnectedAnimationService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectedAnimationService, BASE_OFFSET>(),
            DefaultDuration: DefaultDuration::<Impl, IMPL_OFFSET>,
            SetDefaultDuration: SetDefaultDuration::<Impl, IMPL_OFFSET>,
            DefaultEasingFunction: DefaultEasingFunction::<Impl, IMPL_OFFSET>,
            SetDefaultEasingFunction: SetDefaultEasingFunction::<Impl, IMPL_OFFSET>,
            PrepareToAnimate: PrepareToAnimate::<Impl, IMPL_OFFSET>,
            GetAnimation: GetAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectedAnimationServiceStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<ConnectedAnimationService>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectedAnimationServiceStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IConnectedAnimationServiceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectedAnimationServiceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectedAnimationServiceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectedAnimationServiceStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IConnectedAnimationServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectedAnimationServiceStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectedAnimationServiceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentThemeTransition_Impl: Sized {
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContentThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IContentThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentThemeTransition_Vtbl {
        unsafe extern "system" fn HorizontalOffset<Impl: IContentThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHorizontalOffset<Impl: IContentThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IContentThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVerticalOffset<Impl: IContentThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentThemeTransition, BASE_OFFSET>(),
            HorizontalOffset: HorizontalOffset::<Impl, IMPL_OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Impl, IMPL_OFFSET>,
            VerticalOffset: VerticalOffset::<Impl, IMPL_OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentThemeTransitionStatics_Impl: Sized {
    fn HorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContentThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IContentThemeTransitionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentThemeTransitionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentThemeTransitionStatics_Vtbl {
        unsafe extern "system" fn HorizontalOffsetProperty<Impl: IContentThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalOffsetProperty<Impl: IContentThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentThemeTransitionStatics, BASE_OFFSET>(),
            HorizontalOffsetProperty: HorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            VerticalOffsetProperty: VerticalOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContinuumNavigationTransitionInfo_Impl: Sized {
    fn ExitElement(&mut self) -> ::windows::core::Result<super::super::UIElement>;
    fn SetExitElement(&mut self, value: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContinuumNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContinuumNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IContinuumNavigationTransitionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinuumNavigationTransitionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinuumNavigationTransitionInfo_Vtbl {
        unsafe extern "system" fn ExitElement<Impl: IContinuumNavigationTransitionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExitElement<Impl: IContinuumNavigationTransitionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitElement(&*(&value as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContinuumNavigationTransitionInfo, BASE_OFFSET>(),
            ExitElement: ExitElement::<Impl, IMPL_OFFSET>,
            SetExitElement: SetExitElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinuumNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IContinuumNavigationTransitionInfoStatics_Impl: Sized {
    fn ExitElementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsEntranceElementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsEntranceElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsEntranceElement(&mut self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn IsExitElementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsExitElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsExitElement(&mut self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn ExitElementContainerProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetExitElementContainer(&mut self, element: &::core::option::Option<super::super::Controls::ListViewBase>) -> ::windows::core::Result<bool>;
    fn SetExitElementContainer(&mut self, element: &::core::option::Option<super::super::Controls::ListViewBase>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContinuumNavigationTransitionInfoStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IContinuumNavigationTransitionInfoStatics";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IContinuumNavigationTransitionInfoStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContinuumNavigationTransitionInfoStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContinuumNavigationTransitionInfoStatics_Vtbl {
        unsafe extern "system" fn ExitElementProperty<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEntranceElementProperty<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsEntranceElement<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEntranceElement<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEntranceElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn IsExitElementProperty<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsExitElement<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsExitElement<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsExitElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn ExitElementContainerProperty<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetExitElementContainer<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExitElementContainer<Impl: IContinuumNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitElementContainer(&*(&element as *const <super::super::Controls::ListViewBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBase as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContinuumNavigationTransitionInfoStatics, BASE_OFFSET>(),
            ExitElementProperty: ExitElementProperty::<Impl, IMPL_OFFSET>,
            IsEntranceElementProperty: IsEntranceElementProperty::<Impl, IMPL_OFFSET>,
            GetIsEntranceElement: GetIsEntranceElement::<Impl, IMPL_OFFSET>,
            SetIsEntranceElement: SetIsEntranceElement::<Impl, IMPL_OFFSET>,
            IsExitElementProperty: IsExitElementProperty::<Impl, IMPL_OFFSET>,
            GetIsExitElement: GetIsExitElement::<Impl, IMPL_OFFSET>,
            SetIsExitElement: SetIsExitElement::<Impl, IMPL_OFFSET>,
            ExitElementContainerProperty: ExitElementContainerProperty::<Impl, IMPL_OFFSET>,
            GetExitElementContainer: GetExitElementContainer::<Impl, IMPL_OFFSET>,
            SetExitElementContainer: SetExitElementContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContinuumNavigationTransitionInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICubicEase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICubicEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ICubicEase";
}
#[cfg(feature = "implement_exclusive")]
impl ICubicEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICubicEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICubicEase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICubicEase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICubicEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirectConnectedAnimationConfiguration_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirectConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDirectConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IDirectConnectedAnimationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectConnectedAnimationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectConnectedAnimationConfiguration_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDirectConnectedAnimationConfiguration, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirectConnectedAnimationConfigurationFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DirectConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirectConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDirectConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDirectConnectedAnimationConfigurationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectConnectedAnimationConfigurationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectConnectedAnimationConfigurationFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDirectConnectedAnimationConfigurationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDirectConnectedAnimationConfigurationFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteColorKeyFrame_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscreteColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscreteColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscreteColorKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscreteColorKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscreteColorKeyFrame_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDiscreteColorKeyFrame, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscreteColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteDoubleKeyFrame_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscreteDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscreteDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscreteDoubleKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscreteDoubleKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscreteDoubleKeyFrame_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDiscreteDoubleKeyFrame, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscreteDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscreteObjectKeyFrame_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscreteObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscreteObjectKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscreteObjectKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscreteObjectKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscreteObjectKeyFrame_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDiscreteObjectKeyFrame, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscreteObjectKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiscretePointKeyFrame_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDiscretePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDiscretePointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDiscretePointKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscretePointKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscretePointKeyFrame_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDiscretePointKeyFrame, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscretePointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDoubleAnimation_Impl: Sized {
    fn From(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetFrom(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn To(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetTo(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn By(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<f64>>;
    fn SetBy(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&mut self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&mut self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDoubleAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDoubleAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimation_Vtbl {
        unsafe extern "system" fn From<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrom<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTo<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBy<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEasingFunction<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IDoubleAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleAnimation, BASE_OFFSET>(),
            From: From::<Impl, IMPL_OFFSET>,
            SetFrom: SetFrom::<Impl, IMPL_OFFSET>,
            To: To::<Impl, IMPL_OFFSET>,
            SetTo: SetTo::<Impl, IMPL_OFFSET>,
            By: By::<Impl, IMPL_OFFSET>,
            SetBy: SetBy::<Impl, IMPL_OFFSET>,
            EasingFunction: EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction: SetEasingFunction::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation: EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation: SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationStatics_Impl: Sized {
    fn FromProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimationStatics_Vtbl {
        unsafe extern "system" fn FromProperty<Impl: IDoubleAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToProperty<Impl: IDoubleAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ByProperty<Impl: IDoubleAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EasingFunctionProperty<Impl: IDoubleAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IDoubleAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleAnimationStatics, BASE_OFFSET>(),
            FromProperty: FromProperty::<Impl, IMPL_OFFSET>,
            ToProperty: ToProperty::<Impl, IMPL_OFFSET>,
            ByProperty: ByProperty::<Impl, IMPL_OFFSET>,
            EasingFunctionProperty: EasingFunctionProperty::<Impl, IMPL_OFFSET>,
            EnableDependentAnimationProperty: EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDoubleAnimationUsingKeyFrames_Impl: Sized {
    fn KeyFrames(&mut self) -> ::windows::core::Result<DoubleKeyFrameCollection>;
    fn EnableDependentAnimation(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDoubleAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDoubleAnimationUsingKeyFrames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimationUsingKeyFrames_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimationUsingKeyFrames_Vtbl {
        unsafe extern "system" fn KeyFrames<Impl: IDoubleAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableDependentAnimation<Impl: IDoubleAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IDoubleAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleAnimationUsingKeyFrames, BASE_OFFSET>(),
            KeyFrames: KeyFrames::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation: EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation: SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleAnimationUsingKeyFramesStatics_Impl: Sized {
    fn EnableDependentAnimationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleAnimationUsingKeyFramesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleAnimationUsingKeyFramesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleAnimationUsingKeyFramesStatics_Vtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IDoubleAnimationUsingKeyFramesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleAnimationUsingKeyFramesStatics, BASE_OFFSET>(),
            EnableDependentAnimationProperty: EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDoubleKeyFrame_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<f64>;
    fn SetValue(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTime(&mut self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&mut self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDoubleKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleKeyFrame_Vtbl {
        unsafe extern "system" fn Value<Impl: IDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeyTime<Impl: IDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleKeyFrame, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            KeyTime: KeyTime::<Impl, IMPL_OFFSET>,
            SetKeyTime: SetKeyTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DoubleKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleKeyFrameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleKeyFrameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleKeyFrameFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDoubleKeyFrameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleKeyFrameFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleKeyFrameStatics_Impl: Sized {
    fn ValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDoubleKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleKeyFrameStatics_Vtbl {
        unsafe extern "system" fn ValueProperty<Impl: IDoubleKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyTimeProperty<Impl: IDoubleKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleKeyFrameStatics, BASE_OFFSET>(),
            ValueProperty: ValueProperty::<Impl, IMPL_OFFSET>,
            KeyTimeProperty: KeyTimeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragItemThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDragItemThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragItemThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragItemThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IDragItemThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IDragItemThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragItemThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragItemThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragItemThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragItemThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragItemThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDragItemThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragItemThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragItemThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDragItemThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragItemThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragItemThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IDragOverThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ToOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetToOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Direction(&mut self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetDirection(&mut self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragOverThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragOverThemeAnimation";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IDragOverThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragOverThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragOverThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IDragOverThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IDragOverThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ToOffset<Impl: IDragOverThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetToOffset<Impl: IDragOverThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToOffset(value).into()
        }
        unsafe extern "system" fn Direction<Impl: IDragOverThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: IDragOverThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragOverThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            ToOffset: ToOffset::<Impl, IMPL_OFFSET>,
            SetToOffset: SetToOffset::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            SetDirection: SetDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragOverThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOverThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DirectionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragOverThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDragOverThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDragOverThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragOverThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragOverThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDragOverThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToOffsetProperty<Impl: IDragOverThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DirectionProperty<Impl: IDragOverThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragOverThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
            ToOffsetProperty: ToOffsetProperty::<Impl, IMPL_OFFSET>,
            DirectionProperty: DirectionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragOverThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInNavigationTransitionInfo_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillInNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillInNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillInNavigationTransitionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillInNavigationTransitionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillInNavigationTransitionInfo_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDrillInNavigationTransitionInfo, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillInNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInThemeAnimation_Impl: Sized {
    fn EntranceTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEntranceTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EntranceTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetEntranceTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExitTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetExitTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillInThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillInThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillInThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillInThemeAnimation_Vtbl {
        unsafe extern "system" fn EntranceTargetName<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEntranceTargetName<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EntranceTarget<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEntranceTarget<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTargetName<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExitTargetName<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTarget<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExitTarget<Impl: IDrillInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDrillInThemeAnimation, BASE_OFFSET>(),
            EntranceTargetName: EntranceTargetName::<Impl, IMPL_OFFSET>,
            SetEntranceTargetName: SetEntranceTargetName::<Impl, IMPL_OFFSET>,
            EntranceTarget: EntranceTarget::<Impl, IMPL_OFFSET>,
            SetEntranceTarget: SetEntranceTarget::<Impl, IMPL_OFFSET>,
            ExitTargetName: ExitTargetName::<Impl, IMPL_OFFSET>,
            SetExitTargetName: SetExitTargetName::<Impl, IMPL_OFFSET>,
            ExitTarget: ExitTarget::<Impl, IMPL_OFFSET>,
            SetExitTarget: SetExitTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillInThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillInThemeAnimationStatics_Impl: Sized {
    fn EntranceTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EntranceTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillInThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillInThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillInThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillInThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillInThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn EntranceTargetNameProperty<Impl: IDrillInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EntranceTargetProperty<Impl: IDrillInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExitTargetNameProperty<Impl: IDrillInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExitTargetProperty<Impl: IDrillInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDrillInThemeAnimationStatics, BASE_OFFSET>(),
            EntranceTargetNameProperty: EntranceTargetNameProperty::<Impl, IMPL_OFFSET>,
            EntranceTargetProperty: EntranceTargetProperty::<Impl, IMPL_OFFSET>,
            ExitTargetNameProperty: ExitTargetNameProperty::<Impl, IMPL_OFFSET>,
            ExitTargetProperty: ExitTargetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillInThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillOutThemeAnimation_Impl: Sized {
    fn EntranceTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEntranceTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EntranceTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetEntranceTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExitTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExitTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExitTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetExitTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillOutThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillOutThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillOutThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillOutThemeAnimation_Vtbl {
        unsafe extern "system" fn EntranceTargetName<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEntranceTargetName<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EntranceTarget<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEntranceTarget<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEntranceTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTargetName<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExitTargetName<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitTarget<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExitTarget<Impl: IDrillOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDrillOutThemeAnimation, BASE_OFFSET>(),
            EntranceTargetName: EntranceTargetName::<Impl, IMPL_OFFSET>,
            SetEntranceTargetName: SetEntranceTargetName::<Impl, IMPL_OFFSET>,
            EntranceTarget: EntranceTarget::<Impl, IMPL_OFFSET>,
            SetEntranceTarget: SetEntranceTarget::<Impl, IMPL_OFFSET>,
            ExitTargetName: ExitTargetName::<Impl, IMPL_OFFSET>,
            SetExitTargetName: SetExitTargetName::<Impl, IMPL_OFFSET>,
            ExitTarget: ExitTarget::<Impl, IMPL_OFFSET>,
            SetExitTarget: SetExitTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillOutThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDrillOutThemeAnimationStatics_Impl: Sized {
    fn EntranceTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EntranceTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ExitTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDrillOutThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDrillOutThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDrillOutThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDrillOutThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDrillOutThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn EntranceTargetNameProperty<Impl: IDrillOutThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EntranceTargetProperty<Impl: IDrillOutThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExitTargetNameProperty<Impl: IDrillOutThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExitTargetProperty<Impl: IDrillOutThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDrillOutThemeAnimationStatics, BASE_OFFSET>(),
            EntranceTargetNameProperty: EntranceTargetNameProperty::<Impl, IMPL_OFFSET>,
            EntranceTargetProperty: EntranceTargetProperty::<Impl, IMPL_OFFSET>,
            ExitTargetNameProperty: ExitTargetNameProperty::<Impl, IMPL_OFFSET>,
            ExitTargetProperty: ExitTargetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDrillOutThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetItemThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDropTargetItemThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetItemThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetItemThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetItemThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IDropTargetItemThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IDropTargetItemThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDropTargetItemThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetItemThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetItemThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropTargetItemThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IDropTargetItemThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDropTargetItemThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetItemThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetItemThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IDropTargetItemThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDropTargetItemThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropTargetItemThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingColorKeyFrame_Impl: Sized {
    fn EasingFunction(&mut self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&mut self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingColorKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingColorKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingColorKeyFrame_Vtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingColorKeyFrame, BASE_OFFSET>(),
            EasingFunction: EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction: SetEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingColorKeyFrameStatics_Impl: Sized {
    fn EasingFunctionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingColorKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingColorKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingColorKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingColorKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingColorKeyFrameStatics_Vtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingColorKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingColorKeyFrameStatics, BASE_OFFSET>(),
            EasingFunctionProperty: EasingFunctionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingColorKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingDoubleKeyFrame_Impl: Sized {
    fn EasingFunction(&mut self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&mut self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingDoubleKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingDoubleKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingDoubleKeyFrame_Vtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingDoubleKeyFrame, BASE_OFFSET>(),
            EasingFunction: EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction: SetEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingDoubleKeyFrameStatics_Impl: Sized {
    fn EasingFunctionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingDoubleKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingDoubleKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingDoubleKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingDoubleKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingDoubleKeyFrameStatics_Vtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingDoubleKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingDoubleKeyFrameStatics, BASE_OFFSET>(),
            EasingFunctionProperty: EasingFunctionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingDoubleKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBase_Impl: Sized {
    fn EasingMode(&mut self) -> ::windows::core::Result<EasingMode>;
    fn SetEasingMode(&mut self, value: EasingMode) -> ::windows::core::Result<()>;
    fn Ease(&mut self, normalizedtime: f64) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingFunctionBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingFunctionBase";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingFunctionBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingFunctionBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingFunctionBase_Vtbl {
        unsafe extern "system" fn EasingMode<Impl: IEasingFunctionBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EasingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEasingMode<Impl: IEasingFunctionBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EasingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingMode(value).into()
        }
        unsafe extern "system" fn Ease<Impl: IEasingFunctionBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedtime: f64, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingFunctionBase, BASE_OFFSET>(),
            EasingMode: EasingMode::<Impl, IMPL_OFFSET>,
            SetEasingMode: SetEasingMode::<Impl, IMPL_OFFSET>,
            Ease: Ease::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingFunctionBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingFunctionBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingFunctionBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingFunctionBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingFunctionBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingFunctionBaseFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingFunctionBaseFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingFunctionBaseFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingFunctionBaseStatics_Impl: Sized {
    fn EasingModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingFunctionBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingFunctionBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingFunctionBaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingFunctionBaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingFunctionBaseStatics_Vtbl {
        unsafe extern "system" fn EasingModeProperty<Impl: IEasingFunctionBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingFunctionBaseStatics, BASE_OFFSET>(),
            EasingModeProperty: EasingModeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingFunctionBaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingPointKeyFrame_Impl: Sized {
    fn EasingFunction(&mut self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&mut self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingPointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingPointKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingPointKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingPointKeyFrame_Vtbl {
        unsafe extern "system" fn EasingFunction<Impl: IEasingPointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEasingFunction<Impl: IEasingPointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingPointKeyFrame, BASE_OFFSET>(),
            EasingFunction: EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction: SetEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingPointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasingPointKeyFrameStatics_Impl: Sized {
    fn EasingFunctionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEasingPointKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEasingPointKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEasingPointKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEasingPointKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEasingPointKeyFrameStatics_Vtbl {
        unsafe extern "system" fn EasingFunctionProperty<Impl: IEasingPointKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEasingPointKeyFrameStatics, BASE_OFFSET>(),
            EasingFunctionProperty: EasingFunctionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEasingPointKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IEdgeUIThemeTransition_Impl: Sized {
    fn Edge(&mut self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&mut self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEdgeUIThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEdgeUIThemeTransition";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IEdgeUIThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEdgeUIThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEdgeUIThemeTransition_Vtbl {
        unsafe extern "system" fn Edge<Impl: IEdgeUIThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEdge<Impl: IEdgeUIThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEdge(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEdgeUIThemeTransition, BASE_OFFSET>(),
            Edge: Edge::<Impl, IMPL_OFFSET>,
            SetEdge: SetEdge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEdgeUIThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeUIThemeTransitionStatics_Impl: Sized {
    fn EdgeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEdgeUIThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEdgeUIThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEdgeUIThemeTransitionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEdgeUIThemeTransitionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEdgeUIThemeTransitionStatics_Vtbl {
        unsafe extern "system" fn EdgeProperty<Impl: IEdgeUIThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEdgeUIThemeTransitionStatics, BASE_OFFSET>(),
            EdgeProperty: EdgeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEdgeUIThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEase_Impl: Sized {
    fn Oscillations(&mut self) -> ::windows::core::Result<i32>;
    fn SetOscillations(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Springiness(&mut self) -> ::windows::core::Result<f64>;
    fn SetSpringiness(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElasticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IElasticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IElasticEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElasticEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElasticEase_Vtbl {
        unsafe extern "system" fn Oscillations<Impl: IElasticEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOscillations<Impl: IElasticEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOscillations(value).into()
        }
        unsafe extern "system" fn Springiness<Impl: IElasticEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSpringiness<Impl: IElasticEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpringiness(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElasticEase, BASE_OFFSET>(),
            Oscillations: Oscillations::<Impl, IMPL_OFFSET>,
            SetOscillations: SetOscillations::<Impl, IMPL_OFFSET>,
            Springiness: Springiness::<Impl, IMPL_OFFSET>,
            SetSpringiness: SetSpringiness::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElasticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEaseStatics_Impl: Sized {
    fn OscillationsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SpringinessProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElasticEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IElasticEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IElasticEaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElasticEaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElasticEaseStatics_Vtbl {
        unsafe extern "system" fn OscillationsProperty<Impl: IElasticEaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SpringinessProperty<Impl: IElasticEaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElasticEaseStatics, BASE_OFFSET>(),
            OscillationsProperty: OscillationsProperty::<Impl, IMPL_OFFSET>,
            SpringinessProperty: SpringinessProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElasticEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceNavigationTransitionInfo_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceNavigationTransitionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceNavigationTransitionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceNavigationTransitionInfo_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IEntranceNavigationTransitionInfo, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceNavigationTransitionInfoStatics_Impl: Sized {
    fn IsTargetElementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsTargetElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<bool>;
    fn SetIsTargetElement(&mut self, element: &::core::option::Option<super::super::UIElement>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceNavigationTransitionInfoStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceNavigationTransitionInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceNavigationTransitionInfoStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceNavigationTransitionInfoStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceNavigationTransitionInfoStatics_Vtbl {
        unsafe extern "system" fn IsTargetElementProperty<Impl: IEntranceNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsTargetElement<Impl: IEntranceNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsTargetElement<Impl: IEntranceNavigationTransitionInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTargetElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEntranceNavigationTransitionInfoStatics, BASE_OFFSET>(),
            IsTargetElementProperty: IsTargetElementProperty::<Impl, IMPL_OFFSET>,
            GetIsTargetElement: GetIsTargetElement::<Impl, IMPL_OFFSET>,
            SetIsTargetElement: SetIsTargetElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceNavigationTransitionInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceThemeTransition_Impl: Sized {
    fn FromHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn IsStaggeringEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceThemeTransition_Vtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IEntranceThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IEntranceThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IEntranceThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IEntranceThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        unsafe extern "system" fn IsStaggeringEnabled<Impl: IEntranceThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: IEntranceThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEntranceThemeTransition, BASE_OFFSET>(),
            FromHorizontalOffset: FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset: SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset: FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset: SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
            IsStaggeringEnabled: IsStaggeringEnabled::<Impl, IMPL_OFFSET>,
            SetIsStaggeringEnabled: SetIsStaggeringEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEntranceThemeTransitionStatics_Impl: Sized {
    fn FromHorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStaggeringEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEntranceThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IEntranceThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IEntranceThemeTransitionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEntranceThemeTransitionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEntranceThemeTransitionStatics_Vtbl {
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IEntranceThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IEntranceThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: IEntranceThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEntranceThemeTransitionStatics, BASE_OFFSET>(),
            FromHorizontalOffsetProperty: FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            FromVerticalOffsetProperty: FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>,
            IsStaggeringEnabledProperty: IsStaggeringEnabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEntranceThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEase_Impl: Sized {
    fn Exponent(&mut self) -> ::windows::core::Result<f64>;
    fn SetExponent(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExponentialEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IExponentialEase";
}
#[cfg(feature = "implement_exclusive")]
impl IExponentialEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExponentialEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExponentialEase_Vtbl {
        unsafe extern "system" fn Exponent<Impl: IExponentialEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExponent<Impl: IExponentialEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExponent(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExponentialEase, BASE_OFFSET>(),
            Exponent: Exponent::<Impl, IMPL_OFFSET>,
            SetExponent: SetExponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExponentialEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEaseStatics_Impl: Sized {
    fn ExponentProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExponentialEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IExponentialEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IExponentialEaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExponentialEaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExponentialEaseStatics_Vtbl {
        unsafe extern "system" fn ExponentProperty<Impl: IExponentialEaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExponentialEaseStatics, BASE_OFFSET>(),
            ExponentProperty: ExponentProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExponentialEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeInThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeInThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeInThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeInThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeInThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IFadeInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IFadeInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFadeInThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeInThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeInThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeInThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeInThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeInThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeInThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeInThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IFadeInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFadeInThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeInThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeOutThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeOutThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeOutThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeOutThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeOutThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IFadeOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IFadeOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFadeOutThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeOutThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFadeOutThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFadeOutThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IFadeOutThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFadeOutThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFadeOutThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFadeOutThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IFadeOutThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFadeOutThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFadeOutThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfiguration_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGravityConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IGravityConnectedAnimationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IGravityConnectedAnimationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGravityConnectedAnimationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGravityConnectedAnimationConfiguration_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGravityConnectedAnimationConfiguration, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGravityConnectedAnimationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfiguration2_Impl: Sized {
    fn IsShadowEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsShadowEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGravityConnectedAnimationConfiguration2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IGravityConnectedAnimationConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IGravityConnectedAnimationConfiguration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGravityConnectedAnimationConfiguration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGravityConnectedAnimationConfiguration2_Vtbl {
        unsafe extern "system" fn IsShadowEnabled<Impl: IGravityConnectedAnimationConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsShadowEnabled<Impl: IGravityConnectedAnimationConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsShadowEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGravityConnectedAnimationConfiguration2, BASE_OFFSET>(),
            IsShadowEnabled: IsShadowEnabled::<Impl, IMPL_OFFSET>,
            SetIsShadowEnabled: SetIsShadowEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGravityConnectedAnimationConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGravityConnectedAnimationConfigurationFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GravityConnectedAnimationConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGravityConnectedAnimationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IGravityConnectedAnimationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGravityConnectedAnimationConfigurationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGravityConnectedAnimationConfigurationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGravityConnectedAnimationConfigurationFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGravityConnectedAnimationConfigurationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGravityConnectedAnimationConfigurationFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGravityConnectedAnimationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeySpline_Impl: Sized {
    fn ControlPoint1(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint1(&mut self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn ControlPoint2(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetControlPoint2(&mut self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeySpline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeySpline";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeySpline_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeySpline_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeySpline_Vtbl {
        unsafe extern "system" fn ControlPoint1<Impl: IKeySpline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetControlPoint1<Impl: IKeySpline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlPoint1(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ControlPoint2<Impl: IKeySpline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetControlPoint2<Impl: IKeySpline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlPoint2(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeySpline, BASE_OFFSET>(),
            ControlPoint1: ControlPoint1::<Impl, IMPL_OFFSET>,
            SetControlPoint1: SetControlPoint1::<Impl, IMPL_OFFSET>,
            ControlPoint2: ControlPoint2::<Impl, IMPL_OFFSET>,
            SetControlPoint2: SetControlPoint2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeySpline as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyTimeHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyTimeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeyTimeHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyTimeHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyTimeHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyTimeHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyTimeHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyTimeHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeyTimeHelperStatics_Impl: Sized {
    fn FromTimeSpan(&mut self, timespan: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<KeyTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyTimeHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IKeyTimeHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeyTimeHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyTimeHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyTimeHelperStatics_Vtbl {
        unsafe extern "system" fn FromTimeSpan<Impl: IKeyTimeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timespan: super::super::super::super::Foundation::TimeSpan, result__: *mut KeyTime) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyTimeHelperStatics, BASE_OFFSET>(), FromTimeSpan: FromTimeSpan::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyTimeHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearColorKeyFrame_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ILinearColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearColorKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearColorKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearColorKeyFrame_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearColorKeyFrame, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearDoubleKeyFrame_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ILinearDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearDoubleKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearDoubleKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearDoubleKeyFrame_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearDoubleKeyFrame, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearPointKeyFrame_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ILinearPointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearPointKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearPointKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearPointKeyFrame_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearPointKeyFrame, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearPointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationThemeTransition_Impl: Sized {
    fn DefaultNavigationTransitionInfo(&mut self) -> ::windows::core::Result<NavigationTransitionInfo>;
    fn SetDefaultNavigationTransitionInfo(&mut self, value: &::core::option::Option<NavigationTransitionInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationThemeTransition_Vtbl {
        unsafe extern "system" fn DefaultNavigationTransitionInfo<Impl: INavigationThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDefaultNavigationTransitionInfo<Impl: INavigationThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultNavigationTransitionInfo(&*(&value as *const <NavigationTransitionInfo as ::windows::core::Abi>::Abi as *const <NavigationTransitionInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationThemeTransition, BASE_OFFSET>(),
            DefaultNavigationTransitionInfo: DefaultNavigationTransitionInfo::<Impl, IMPL_OFFSET>,
            SetDefaultNavigationTransitionInfo: SetDefaultNavigationTransitionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationThemeTransitionStatics_Impl: Sized {
    fn DefaultNavigationTransitionInfoProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationThemeTransitionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationThemeTransitionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationThemeTransitionStatics_Vtbl {
        unsafe extern "system" fn DefaultNavigationTransitionInfoProperty<Impl: INavigationThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationThemeTransitionStatics, BASE_OFFSET>(),
            DefaultNavigationTransitionInfoProperty: DefaultNavigationTransitionInfoProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfo_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationTransitionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationTransitionInfo_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationTransitionInfo, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationTransitionInfoFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfoFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationTransitionInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationTransitionInfoFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: INavigationTransitionInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationTransitionInfoFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationTransitionInfoOverrides_Impl: Sized {
    fn GetNavigationStateCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNavigationStateCore(&mut self, navigationstate: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationTransitionInfoOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfoOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationTransitionInfoOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationTransitionInfoOverrides_Vtbl {
        unsafe extern "system" fn GetNavigationStateCore<Impl: INavigationTransitionInfoOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNavigationStateCore<Impl: INavigationTransitionInfoOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navigationstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNavigationStateCore(&*(&navigationstate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationTransitionInfoOverrides, BASE_OFFSET>(),
            GetNavigationStateCore: GetNavigationStateCore::<Impl, IMPL_OFFSET>,
            SetNavigationStateCore: SetNavigationStateCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfoOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IObjectAnimationUsingKeyFrames_Impl: Sized {
    fn KeyFrames(&mut self) -> ::windows::core::Result<ObjectKeyFrameCollection>;
    fn EnableDependentAnimation(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IObjectAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IObjectAnimationUsingKeyFrames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAnimationUsingKeyFrames_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectAnimationUsingKeyFrames_Vtbl {
        unsafe extern "system" fn KeyFrames<Impl: IObjectAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableDependentAnimation<Impl: IObjectAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IObjectAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObjectAnimationUsingKeyFrames, BASE_OFFSET>(),
            KeyFrames: KeyFrames::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation: EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation: SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectAnimationUsingKeyFramesStatics_Impl: Sized {
    fn EnableDependentAnimationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectAnimationUsingKeyFramesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectAnimationUsingKeyFramesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectAnimationUsingKeyFramesStatics_Vtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IObjectAnimationUsingKeyFramesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObjectAnimationUsingKeyFramesStatics, BASE_OFFSET>(),
            EnableDependentAnimationProperty: EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IObjectKeyFrame_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn KeyTime(&mut self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&mut self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IObjectKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectKeyFrame_Vtbl {
        unsafe extern "system" fn Value<Impl: IObjectKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IObjectKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IObjectKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeyTime<Impl: IObjectKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObjectKeyFrame, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            KeyTime: KeyTime::<Impl, IMPL_OFFSET>,
            SetKeyTime: SetKeyTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ObjectKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectKeyFrameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectKeyFrameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectKeyFrameFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IObjectKeyFrameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObjectKeyFrameFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IObjectKeyFrameStatics_Impl: Sized {
    fn ValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IObjectKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IObjectKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IObjectKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectKeyFrameStatics_Vtbl {
        unsafe extern "system" fn ValueProperty<Impl: IObjectKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyTimeProperty<Impl: IObjectKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObjectKeyFrameStatics, BASE_OFFSET>(),
            ValueProperty: ValueProperty::<Impl, IMPL_OFFSET>,
            KeyTimeProperty: KeyTimeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IPaneThemeTransition_Impl: Sized {
    fn Edge(&mut self) -> ::windows::core::Result<super::super::Controls::Primitives::EdgeTransitionLocation>;
    fn SetEdge(&mut self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaneThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPaneThemeTransition";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IPaneThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaneThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaneThemeTransition_Vtbl {
        unsafe extern "system" fn Edge<Impl: IPaneThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEdge<Impl: IPaneThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEdge(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaneThemeTransition, BASE_OFFSET>(),
            Edge: Edge::<Impl, IMPL_OFFSET>,
            SetEdge: SetEdge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaneThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaneThemeTransitionStatics_Impl: Sized {
    fn EdgeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaneThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPaneThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPaneThemeTransitionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaneThemeTransitionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaneThemeTransitionStatics_Vtbl {
        unsafe extern "system" fn EdgeProperty<Impl: IPaneThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaneThemeTransitionStatics, BASE_OFFSET>(),
            EdgeProperty: EdgeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaneThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointAnimation_Impl: Sized {
    fn From(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetFrom(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn To(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetTo(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn By(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetBy(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn EasingFunction(&mut self) -> ::windows::core::Result<EasingFunctionBase>;
    fn SetEasingFunction(&mut self, value: &::core::option::Option<EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn EnableDependentAnimation(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPointAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimation_Vtbl {
        unsafe extern "system" fn From<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrom<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTo<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn By<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBy<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBy(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EasingFunction<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEasingFunction<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEasingFunction(&*(&value as *const <EasingFunctionBase as ::windows::core::Abi>::Abi as *const <EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableDependentAnimation<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IPointAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointAnimation, BASE_OFFSET>(),
            From: From::<Impl, IMPL_OFFSET>,
            SetFrom: SetFrom::<Impl, IMPL_OFFSET>,
            To: To::<Impl, IMPL_OFFSET>,
            SetTo: SetTo::<Impl, IMPL_OFFSET>,
            By: By::<Impl, IMPL_OFFSET>,
            SetBy: SetBy::<Impl, IMPL_OFFSET>,
            EasingFunction: EasingFunction::<Impl, IMPL_OFFSET>,
            SetEasingFunction: SetEasingFunction::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation: EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation: SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationStatics_Impl: Sized {
    fn FromProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ByProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EasingFunctionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn EnableDependentAnimationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimationStatics_Vtbl {
        unsafe extern "system" fn FromProperty<Impl: IPointAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToProperty<Impl: IPointAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ByProperty<Impl: IPointAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EasingFunctionProperty<Impl: IPointAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IPointAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointAnimationStatics, BASE_OFFSET>(),
            FromProperty: FromProperty::<Impl, IMPL_OFFSET>,
            ToProperty: ToProperty::<Impl, IMPL_OFFSET>,
            ByProperty: ByProperty::<Impl, IMPL_OFFSET>,
            EasingFunctionProperty: EasingFunctionProperty::<Impl, IMPL_OFFSET>,
            EnableDependentAnimationProperty: EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPointAnimationUsingKeyFrames_Impl: Sized {
    fn KeyFrames(&mut self) -> ::windows::core::Result<PointKeyFrameCollection>;
    fn EnableDependentAnimation(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableDependentAnimation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimationUsingKeyFrames";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPointAnimationUsingKeyFrames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimationUsingKeyFrames_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimationUsingKeyFrames_Vtbl {
        unsafe extern "system" fn KeyFrames<Impl: IPointAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableDependentAnimation<Impl: IPointAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnableDependentAnimation<Impl: IPointAnimationUsingKeyFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDependentAnimation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointAnimationUsingKeyFrames, BASE_OFFSET>(),
            KeyFrames: KeyFrames::<Impl, IMPL_OFFSET>,
            EnableDependentAnimation: EnableDependentAnimation::<Impl, IMPL_OFFSET>,
            SetEnableDependentAnimation: SetEnableDependentAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimationUsingKeyFrames as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointAnimationUsingKeyFramesStatics_Impl: Sized {
    fn EnableDependentAnimationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointAnimationUsingKeyFramesStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointAnimationUsingKeyFramesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointAnimationUsingKeyFramesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointAnimationUsingKeyFramesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointAnimationUsingKeyFramesStatics_Vtbl {
        unsafe extern "system" fn EnableDependentAnimationProperty<Impl: IPointAnimationUsingKeyFramesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointAnimationUsingKeyFramesStatics, BASE_OFFSET>(),
            EnableDependentAnimationProperty: EnableDependentAnimationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointAnimationUsingKeyFramesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointKeyFrame_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetValue(&mut self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn KeyTime(&mut self) -> ::windows::core::Result<KeyTime>;
    fn SetKeyTime(&mut self, value: &KeyTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointKeyFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPointKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointKeyFrame_Vtbl {
        unsafe extern "system" fn Value<Impl: IPointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IPointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyTime<Impl: IPointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeyTime<Impl: IPointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTime(&*(&value as *const <KeyTime as ::windows::core::Abi>::Abi as *const <KeyTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointKeyFrame, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            KeyTime: KeyTime::<Impl, IMPL_OFFSET>,
            SetKeyTime: SetKeyTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PointKeyFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointKeyFrameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointKeyFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPointKeyFrameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointKeyFrameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointKeyFrameFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPointKeyFrameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointKeyFrameFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointKeyFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointKeyFrameStatics_Impl: Sized {
    fn ValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn KeyTimeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointKeyFrameStatics_Vtbl {
        unsafe extern "system" fn ValueProperty<Impl: IPointKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyTimeProperty<Impl: IPointKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointKeyFrameStatics, BASE_OFFSET>(),
            ValueProperty: ValueProperty::<Impl, IMPL_OFFSET>,
            KeyTimeProperty: KeyTimeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDownThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDownThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerDownThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDownThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerDownThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerDownThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IPointerDownThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IPointerDownThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerDownThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerDownThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerDownThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerDownThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerDownThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerDownThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerDownThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerDownThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPointerDownThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerDownThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerDownThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerUpThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerUpThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerUpThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerUpThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerUpThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerUpThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IPointerUpThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IPointerUpThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerUpThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerUpThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerUpThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerUpThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPointerUpThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerUpThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerUpThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerUpThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPointerUpThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerUpThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerUpThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopInThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopInThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPopInThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopInThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopInThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IPopInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IPopInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: IPopInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IPopInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IPopInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IPopInThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopInThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            FromHorizontalOffset: FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset: SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset: FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset: SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopInThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopInThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopInThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopInThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopInThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopInThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopInThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPopInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IPopInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IPopInThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopInThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
            FromHorizontalOffsetProperty: FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            FromVerticalOffsetProperty: FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopInThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopOutThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopOutThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPopOutThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopOutThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopOutThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IPopOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IPopOutThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopOutThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopOutThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopOutThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopOutThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopOutThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopOutThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopOutThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopOutThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IPopOutThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopOutThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopOutThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupThemeTransition_Impl: Sized {
    fn FromHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopupThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupThemeTransition_Vtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IPopupThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IPopupThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IPopupThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IPopupThemeTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopupThemeTransition, BASE_OFFSET>(),
            FromHorizontalOffset: FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset: SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset: FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset: SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupThemeTransitionStatics_Impl: Sized {
    fn FromHorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupThemeTransitionStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPopupThemeTransitionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupThemeTransitionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupThemeTransitionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupThemeTransitionStatics_Vtbl {
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IPopupThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IPopupThemeTransitionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopupThemeTransitionStatics, BASE_OFFSET>(),
            FromHorizontalOffsetProperty: FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            FromVerticalOffsetProperty: FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupThemeTransitionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEase_Impl: Sized {
    fn Power(&mut self) -> ::windows::core::Result<f64>;
    fn SetPower(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPowerEase";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPowerEase_Vtbl {
        unsafe extern "system" fn Power<Impl: IPowerEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPower<Impl: IPowerEase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPower(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPowerEase, BASE_OFFSET>(),
            Power: Power::<Impl, IMPL_OFFSET>,
            SetPower: SetPower::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPowerEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEaseStatics_Impl: Sized {
    fn PowerProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerEaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IPowerEaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerEaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerEaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPowerEaseStatics_Vtbl {
        unsafe extern "system" fn PowerProperty<Impl: IPowerEaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPowerEaseStatics, BASE_OFFSET>(), PowerProperty: PowerProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPowerEaseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticEase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuadraticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IQuadraticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IQuadraticEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuadraticEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuadraticEase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IQuadraticEase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuadraticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuarticEase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuarticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IQuarticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IQuarticEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuarticEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuarticEase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IQuarticEase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuarticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuinticEase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IQuinticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IQuinticEase";
}
#[cfg(feature = "implement_exclusive")]
impl IQuinticEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuinticEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuinticEase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IQuinticEase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuinticEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReorderThemeTransition_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReorderThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IReorderThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IReorderThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReorderThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReorderThemeTransition_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IReorderThemeTransition, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReorderThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatBehaviorHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatBehaviorHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepeatBehaviorHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatBehaviorHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatBehaviorHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatBehaviorHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRepeatBehaviorHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatBehaviorHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRepeatBehaviorHelperStatics_Impl: Sized {
    fn Forever(&mut self) -> ::windows::core::Result<RepeatBehavior>;
    fn FromCount(&mut self, count: f64) -> ::windows::core::Result<RepeatBehavior>;
    fn FromDuration(&mut self, duration: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<RepeatBehavior>;
    fn GetHasCount(&mut self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn GetHasDuration(&mut self, target: &RepeatBehavior) -> ::windows::core::Result<bool>;
    fn Equals(&mut self, target: &RepeatBehavior, value: &RepeatBehavior) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRepeatBehaviorHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepeatBehaviorHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRepeatBehaviorHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatBehaviorHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatBehaviorHelperStatics_Vtbl {
        unsafe extern "system" fn Forever<Impl: IRepeatBehaviorHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromCount<Impl: IRepeatBehaviorHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: f64, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromDuration<Impl: IRepeatBehaviorHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: super::super::super::super::Foundation::TimeSpan, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetHasCount<Impl: IRepeatBehaviorHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetHasDuration<Impl: IRepeatBehaviorHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Equals<Impl: IRepeatBehaviorHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: RepeatBehavior, value: RepeatBehavior, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepeatBehaviorHelperStatics, BASE_OFFSET>(),
            Forever: Forever::<Impl, IMPL_OFFSET>,
            FromCount: FromCount::<Impl, IMPL_OFFSET>,
            FromDuration: FromDuration::<Impl, IMPL_OFFSET>,
            GetHasCount: GetHasCount::<Impl, IMPL_OFFSET>,
            GetHasDuration: GetHasDuration::<Impl, IMPL_OFFSET>,
            Equals: Equals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatBehaviorHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: IRepositionThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IRepositionThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: IRepositionThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: IRepositionThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IRepositionThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromVerticalOffset<Impl: IRepositionThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepositionThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            FromHorizontalOffset: FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset: SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset: FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset: SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: IRepositionThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: IRepositionThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: IRepositionThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepositionThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
            FromHorizontalOffsetProperty: FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            FromVerticalOffsetProperty: FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransition_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeTransition_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRepositionThemeTransition, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransition2_Impl: Sized {
    fn IsStaggeringEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsStaggeringEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeTransition2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeTransition2";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeTransition2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeTransition2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeTransition2_Vtbl {
        unsafe extern "system" fn IsStaggeringEnabled<Impl: IRepositionThemeTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsStaggeringEnabled<Impl: IRepositionThemeTransition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStaggeringEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepositionThemeTransition2, BASE_OFFSET>(),
            IsStaggeringEnabled: IsStaggeringEnabled::<Impl, IMPL_OFFSET>,
            SetIsStaggeringEnabled: SetIsStaggeringEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeTransition2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepositionThemeTransitionStatics2_Impl: Sized {
    fn IsStaggeringEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepositionThemeTransitionStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IRepositionThemeTransitionStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRepositionThemeTransitionStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepositionThemeTransitionStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepositionThemeTransitionStatics2_Vtbl {
        unsafe extern "system" fn IsStaggeringEnabledProperty<Impl: IRepositionThemeTransitionStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepositionThemeTransitionStatics2, BASE_OFFSET>(),
            IsStaggeringEnabledProperty: IsStaggeringEnabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepositionThemeTransitionStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISineEase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISineEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISineEase";
}
#[cfg(feature = "implement_exclusive")]
impl ISineEase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISineEase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISineEase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISineEase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISineEase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfo_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlideNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISlideNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISlideNavigationTransitionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlideNavigationTransitionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlideNavigationTransitionInfo_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISlideNavigationTransitionInfo, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlideNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfo2_Impl: Sized {
    fn Effect(&mut self) -> ::windows::core::Result<SlideNavigationTransitionEffect>;
    fn SetEffect(&mut self, value: SlideNavigationTransitionEffect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlideNavigationTransitionInfo2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISlideNavigationTransitionInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl ISlideNavigationTransitionInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlideNavigationTransitionInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlideNavigationTransitionInfo2_Vtbl {
        unsafe extern "system" fn Effect<Impl: ISlideNavigationTransitionInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SlideNavigationTransitionEffect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEffect<Impl: ISlideNavigationTransitionInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SlideNavigationTransitionEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffect(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISlideNavigationTransitionInfo2, BASE_OFFSET>(),
            Effect: Effect::<Impl, IMPL_OFFSET>,
            SetEffect: SetEffect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlideNavigationTransitionInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlideNavigationTransitionInfoStatics2_Impl: Sized {
    fn EffectProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlideNavigationTransitionInfoStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISlideNavigationTransitionInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISlideNavigationTransitionInfoStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlideNavigationTransitionInfoStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlideNavigationTransitionInfoStatics2_Vtbl {
        unsafe extern "system" fn EffectProperty<Impl: ISlideNavigationTransitionInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISlideNavigationTransitionInfoStatics2, BASE_OFFSET>(),
            EffectProperty: EffectProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlideNavigationTransitionInfoStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineColorKeyFrame_Impl: Sized {
    fn KeySpline(&mut self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&mut self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineColorKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineColorKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineColorKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineColorKeyFrame_Vtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplineColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeySpline<Impl: ISplineColorKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplineColorKeyFrame, BASE_OFFSET>(),
            KeySpline: KeySpline::<Impl, IMPL_OFFSET>,
            SetKeySpline: SetKeySpline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineColorKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineColorKeyFrameStatics_Impl: Sized {
    fn KeySplineProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineColorKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineColorKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineColorKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineColorKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineColorKeyFrameStatics_Vtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplineColorKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplineColorKeyFrameStatics, BASE_OFFSET>(),
            KeySplineProperty: KeySplineProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineColorKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineDoubleKeyFrame_Impl: Sized {
    fn KeySpline(&mut self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&mut self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineDoubleKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineDoubleKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineDoubleKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineDoubleKeyFrame_Vtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplineDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeySpline<Impl: ISplineDoubleKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplineDoubleKeyFrame, BASE_OFFSET>(),
            KeySpline: KeySpline::<Impl, IMPL_OFFSET>,
            SetKeySpline: SetKeySpline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineDoubleKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplineDoubleKeyFrameStatics_Impl: Sized {
    fn KeySplineProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplineDoubleKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplineDoubleKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplineDoubleKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplineDoubleKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplineDoubleKeyFrameStatics_Vtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplineDoubleKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplineDoubleKeyFrameStatics, BASE_OFFSET>(),
            KeySplineProperty: KeySplineProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplineDoubleKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplinePointKeyFrame_Impl: Sized {
    fn KeySpline(&mut self) -> ::windows::core::Result<KeySpline>;
    fn SetKeySpline(&mut self, value: &::core::option::Option<KeySpline>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplinePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplinePointKeyFrame";
}
#[cfg(feature = "implement_exclusive")]
impl ISplinePointKeyFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplinePointKeyFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplinePointKeyFrame_Vtbl {
        unsafe extern "system" fn KeySpline<Impl: ISplinePointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeySpline<Impl: ISplinePointKeyFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpline(&*(&value as *const <KeySpline as ::windows::core::Abi>::Abi as *const <KeySpline as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplinePointKeyFrame, BASE_OFFSET>(),
            KeySpline: KeySpline::<Impl, IMPL_OFFSET>,
            SetKeySpline: SetKeySpline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplinePointKeyFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplinePointKeyFrameStatics_Impl: Sized {
    fn KeySplineProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplinePointKeyFrameStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplinePointKeyFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplinePointKeyFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplinePointKeyFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplinePointKeyFrameStatics_Vtbl {
        unsafe extern "system" fn KeySplineProperty<Impl: ISplinePointKeyFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplinePointKeyFrameStatics, BASE_OFFSET>(),
            KeySplineProperty: KeySplineProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplinePointKeyFrameStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait ISplitCloseThemeAnimation_Impl: Sized {
    fn OpenedTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOpenedTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenedTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOpenedTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ClosedTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClosedTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClosedTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetClosedTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ContentTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetContentTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn OpenedLength(&mut self) -> ::windows::core::Result<f64>;
    fn SetOpenedLength(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ClosedLength(&mut self) -> ::windows::core::Result<f64>;
    fn SetClosedLength(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetFromCenter(&mut self) -> ::windows::core::Result<f64>;
    fn SetOffsetFromCenter(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ContentTranslationDirection(&mut self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetContentTranslationDirection(&mut self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
    fn ContentTranslationOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetContentTranslationOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISplitCloseThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitCloseThemeAnimation";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ISplitCloseThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitCloseThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitCloseThemeAnimation_Vtbl {
        unsafe extern "system" fn OpenedTargetName<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpenedTargetName<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedTarget<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpenedTarget<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTargetName<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClosedTargetName<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTarget<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClosedTarget<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTargetName<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTargetName<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTarget<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTarget<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedLength<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpenedLength<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedLength(value).into()
        }
        unsafe extern "system" fn ClosedLength<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClosedLength<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedLength(value).into()
        }
        unsafe extern "system" fn OffsetFromCenter<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffsetFromCenter<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetFromCenter(value).into()
        }
        unsafe extern "system" fn ContentTranslationDirection<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTranslationDirection<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationDirection(value).into()
        }
        unsafe extern "system" fn ContentTranslationOffset<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTranslationOffset<Impl: ISplitCloseThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplitCloseThemeAnimation, BASE_OFFSET>(),
            OpenedTargetName: OpenedTargetName::<Impl, IMPL_OFFSET>,
            SetOpenedTargetName: SetOpenedTargetName::<Impl, IMPL_OFFSET>,
            OpenedTarget: OpenedTarget::<Impl, IMPL_OFFSET>,
            SetOpenedTarget: SetOpenedTarget::<Impl, IMPL_OFFSET>,
            ClosedTargetName: ClosedTargetName::<Impl, IMPL_OFFSET>,
            SetClosedTargetName: SetClosedTargetName::<Impl, IMPL_OFFSET>,
            ClosedTarget: ClosedTarget::<Impl, IMPL_OFFSET>,
            SetClosedTarget: SetClosedTarget::<Impl, IMPL_OFFSET>,
            ContentTargetName: ContentTargetName::<Impl, IMPL_OFFSET>,
            SetContentTargetName: SetContentTargetName::<Impl, IMPL_OFFSET>,
            ContentTarget: ContentTarget::<Impl, IMPL_OFFSET>,
            SetContentTarget: SetContentTarget::<Impl, IMPL_OFFSET>,
            OpenedLength: OpenedLength::<Impl, IMPL_OFFSET>,
            SetOpenedLength: SetOpenedLength::<Impl, IMPL_OFFSET>,
            ClosedLength: ClosedLength::<Impl, IMPL_OFFSET>,
            SetClosedLength: SetClosedLength::<Impl, IMPL_OFFSET>,
            OffsetFromCenter: OffsetFromCenter::<Impl, IMPL_OFFSET>,
            SetOffsetFromCenter: SetOffsetFromCenter::<Impl, IMPL_OFFSET>,
            ContentTranslationDirection: ContentTranslationDirection::<Impl, IMPL_OFFSET>,
            SetContentTranslationDirection: SetContentTranslationDirection::<Impl, IMPL_OFFSET>,
            ContentTranslationOffset: ContentTranslationOffset::<Impl, IMPL_OFFSET>,
            SetContentTranslationOffset: SetContentTranslationOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitCloseThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitCloseThemeAnimationStatics_Impl: Sized {
    fn OpenedTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedLengthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedLengthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetFromCenterProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationDirectionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitCloseThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitCloseThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitCloseThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitCloseThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitCloseThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn OpenedTargetNameProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenedTargetProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClosedTargetNameProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClosedTargetProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTargetNameProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTargetProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenedLengthProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClosedLengthProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OffsetFromCenterProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTranslationDirectionProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTranslationOffsetProperty<Impl: ISplitCloseThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplitCloseThemeAnimationStatics, BASE_OFFSET>(),
            OpenedTargetNameProperty: OpenedTargetNameProperty::<Impl, IMPL_OFFSET>,
            OpenedTargetProperty: OpenedTargetProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetNameProperty: ClosedTargetNameProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetProperty: ClosedTargetProperty::<Impl, IMPL_OFFSET>,
            ContentTargetNameProperty: ContentTargetNameProperty::<Impl, IMPL_OFFSET>,
            ContentTargetProperty: ContentTargetProperty::<Impl, IMPL_OFFSET>,
            OpenedLengthProperty: OpenedLengthProperty::<Impl, IMPL_OFFSET>,
            ClosedLengthProperty: ClosedLengthProperty::<Impl, IMPL_OFFSET>,
            OffsetFromCenterProperty: OffsetFromCenterProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationDirectionProperty: ContentTranslationDirectionProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationOffsetProperty: ContentTranslationOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitCloseThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait ISplitOpenThemeAnimation_Impl: Sized {
    fn OpenedTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOpenedTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenedTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOpenedTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ClosedTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetClosedTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ClosedTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetClosedTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ContentTargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTarget(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetContentTarget(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
    fn OpenedLength(&mut self) -> ::windows::core::Result<f64>;
    fn SetOpenedLength(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ClosedLength(&mut self) -> ::windows::core::Result<f64>;
    fn SetClosedLength(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetFromCenter(&mut self) -> ::windows::core::Result<f64>;
    fn SetOffsetFromCenter(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ContentTranslationDirection(&mut self) -> ::windows::core::Result<super::super::Controls::Primitives::AnimationDirection>;
    fn SetContentTranslationDirection(&mut self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::Result<()>;
    fn ContentTranslationOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetContentTranslationOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISplitOpenThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitOpenThemeAnimation";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ISplitOpenThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitOpenThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitOpenThemeAnimation_Vtbl {
        unsafe extern "system" fn OpenedTargetName<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpenedTargetName<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedTarget<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpenedTarget<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTargetName<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClosedTargetName<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClosedTarget<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClosedTarget<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTargetName<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTargetName<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTarget<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTarget<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTarget(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenedLength<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpenedLength<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpenedLength(value).into()
        }
        unsafe extern "system" fn ClosedLength<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClosedLength<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosedLength(value).into()
        }
        unsafe extern "system" fn OffsetFromCenter<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffsetFromCenter<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffsetFromCenter(value).into()
        }
        unsafe extern "system" fn ContentTranslationDirection<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTranslationDirection<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationDirection(value).into()
        }
        unsafe extern "system" fn ContentTranslationOffset<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTranslationOffset<Impl: ISplitOpenThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTranslationOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplitOpenThemeAnimation, BASE_OFFSET>(),
            OpenedTargetName: OpenedTargetName::<Impl, IMPL_OFFSET>,
            SetOpenedTargetName: SetOpenedTargetName::<Impl, IMPL_OFFSET>,
            OpenedTarget: OpenedTarget::<Impl, IMPL_OFFSET>,
            SetOpenedTarget: SetOpenedTarget::<Impl, IMPL_OFFSET>,
            ClosedTargetName: ClosedTargetName::<Impl, IMPL_OFFSET>,
            SetClosedTargetName: SetClosedTargetName::<Impl, IMPL_OFFSET>,
            ClosedTarget: ClosedTarget::<Impl, IMPL_OFFSET>,
            SetClosedTarget: SetClosedTarget::<Impl, IMPL_OFFSET>,
            ContentTargetName: ContentTargetName::<Impl, IMPL_OFFSET>,
            SetContentTargetName: SetContentTargetName::<Impl, IMPL_OFFSET>,
            ContentTarget: ContentTarget::<Impl, IMPL_OFFSET>,
            SetContentTarget: SetContentTarget::<Impl, IMPL_OFFSET>,
            OpenedLength: OpenedLength::<Impl, IMPL_OFFSET>,
            SetOpenedLength: SetOpenedLength::<Impl, IMPL_OFFSET>,
            ClosedLength: ClosedLength::<Impl, IMPL_OFFSET>,
            SetClosedLength: SetClosedLength::<Impl, IMPL_OFFSET>,
            OffsetFromCenter: OffsetFromCenter::<Impl, IMPL_OFFSET>,
            SetOffsetFromCenter: SetOffsetFromCenter::<Impl, IMPL_OFFSET>,
            ContentTranslationDirection: ContentTranslationDirection::<Impl, IMPL_OFFSET>,
            SetContentTranslationDirection: SetContentTranslationDirection::<Impl, IMPL_OFFSET>,
            ContentTranslationOffset: ContentTranslationOffset::<Impl, IMPL_OFFSET>,
            SetContentTranslationOffset: SetContentTranslationOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitOpenThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitOpenThemeAnimationStatics_Impl: Sized {
    fn OpenedTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OpenedLengthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ClosedLengthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetFromCenterProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationDirectionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentTranslationOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitOpenThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISplitOpenThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitOpenThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitOpenThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitOpenThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn OpenedTargetNameProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenedTargetProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClosedTargetNameProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClosedTargetProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTargetNameProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTargetProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenedLengthProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClosedLengthProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OffsetFromCenterProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTranslationDirectionProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTranslationOffsetProperty<Impl: ISplitOpenThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplitOpenThemeAnimationStatics, BASE_OFFSET>(),
            OpenedTargetNameProperty: OpenedTargetNameProperty::<Impl, IMPL_OFFSET>,
            OpenedTargetProperty: OpenedTargetProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetNameProperty: ClosedTargetNameProperty::<Impl, IMPL_OFFSET>,
            ClosedTargetProperty: ClosedTargetProperty::<Impl, IMPL_OFFSET>,
            ContentTargetNameProperty: ContentTargetNameProperty::<Impl, IMPL_OFFSET>,
            ContentTargetProperty: ContentTargetProperty::<Impl, IMPL_OFFSET>,
            OpenedLengthProperty: OpenedLengthProperty::<Impl, IMPL_OFFSET>,
            ClosedLengthProperty: ClosedLengthProperty::<Impl, IMPL_OFFSET>,
            OffsetFromCenterProperty: OffsetFromCenterProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationDirectionProperty: ContentTranslationDirectionProperty::<Impl, IMPL_OFFSET>,
            ContentTranslationOffsetProperty: ContentTranslationOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitOpenThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoryboard_Impl: Sized {
    fn Children(&mut self) -> ::windows::core::Result<TimelineCollection>;
    fn Seek(&mut self, offset: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Begin(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn GetCurrentState(&mut self) -> ::windows::core::Result<ClockState>;
    fn GetCurrentTime(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SeekAlignedToLastTick(&mut self, offset: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SkipToFill(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IStoryboard";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoryboard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoryboard_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoryboard_Vtbl {
        unsafe extern "system" fn Children<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Seek<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(&*(&offset as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stop<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Begin<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin().into()
        }
        unsafe extern "system" fn Pause<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn GetCurrentState<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClockState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentTime<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SeekAlignedToLastTick<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SeekAlignedToLastTick(&*(&offset as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SkipToFill<Impl: IStoryboard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SkipToFill().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoryboard, BASE_OFFSET>(),
            Children: Children::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Begin: Begin::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            GetCurrentState: GetCurrentState::<Impl, IMPL_OFFSET>,
            GetCurrentTime: GetCurrentTime::<Impl, IMPL_OFFSET>,
            SeekAlignedToLastTick: SeekAlignedToLastTick::<Impl, IMPL_OFFSET>,
            SkipToFill: SkipToFill::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoryboard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoryboardStatics_Impl: Sized {
    fn TargetPropertyProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTargetProperty(&mut self, element: &::core::option::Option<Timeline>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetProperty(&mut self, element: &::core::option::Option<Timeline>, path: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTargetName(&mut self, element: &::core::option::Option<Timeline>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, element: &::core::option::Option<Timeline>, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetTarget(&mut self, timeline: &::core::option::Option<Timeline>, target: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoryboardStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.IStoryboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStoryboardStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoryboardStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoryboardStatics_Vtbl {
        unsafe extern "system" fn TargetPropertyProperty<Impl: IStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTargetProperty<Impl: IStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetProperty<Impl: IStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetProperty(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetNameProperty<Impl: IStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTargetName<Impl: IStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: IStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&element as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetTarget<Impl: IStoryboardStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeline: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&timeline as *const <Timeline as ::windows::core::Abi>::Abi as *const <Timeline as ::windows::core::DefaultType>::DefaultType), &*(&target as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoryboardStatics, BASE_OFFSET>(),
            TargetPropertyProperty: TargetPropertyProperty::<Impl, IMPL_OFFSET>,
            GetTargetProperty: GetTargetProperty::<Impl, IMPL_OFFSET>,
            SetTargetProperty: SetTargetProperty::<Impl, IMPL_OFFSET>,
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
            GetTargetName: GetTargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            SetTarget: SetTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoryboardStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISuppressNavigationTransitionInfo_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISuppressNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISuppressNavigationTransitionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISuppressNavigationTransitionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuppressNavigationTransitionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISuppressNavigationTransitionInfo_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISuppressNavigationTransitionInfo, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuppressNavigationTransitionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeBackThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FromHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn FromVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeBackThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeBackThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeBackThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeBackThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeBackThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: ISwipeBackThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: ISwipeBackThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FromHorizontalOffset<Impl: ISwipeBackThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromHorizontalOffset<Impl: ISwipeBackThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromHorizontalOffset(value).into()
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: ISwipeBackThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFromVerticalOffset<Impl: ISwipeBackThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromVerticalOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISwipeBackThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            FromHorizontalOffset: FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetFromHorizontalOffset: SetFromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset: FromVerticalOffset::<Impl, IMPL_OFFSET>,
            SetFromVerticalOffset: SetFromVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeBackThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeBackThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromHorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FromVerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeBackThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeBackThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeBackThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeBackThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeBackThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: ISwipeBackThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromHorizontalOffsetProperty<Impl: ISwipeBackThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromVerticalOffsetProperty<Impl: ISwipeBackThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISwipeBackThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
            FromHorizontalOffsetProperty: FromHorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            FromVerticalOffsetProperty: FromVerticalOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeBackThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeHintThemeAnimation_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ToHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetToHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ToVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetToVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeHintThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeHintThemeAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeHintThemeAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeHintThemeAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeHintThemeAnimation_Vtbl {
        unsafe extern "system" fn TargetName<Impl: ISwipeHintThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetName<Impl: ISwipeHintThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ToHorizontalOffset<Impl: ISwipeHintThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetToHorizontalOffset<Impl: ISwipeHintThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToHorizontalOffset(value).into()
        }
        unsafe extern "system" fn ToVerticalOffset<Impl: ISwipeHintThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetToVerticalOffset<Impl: ISwipeHintThemeAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToVerticalOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISwipeHintThemeAnimation, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            ToHorizontalOffset: ToHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetToHorizontalOffset: SetToHorizontalOffset::<Impl, IMPL_OFFSET>,
            ToVerticalOffset: ToVerticalOffset::<Impl, IMPL_OFFSET>,
            SetToVerticalOffset: SetToVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeHintThemeAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISwipeHintThemeAnimationStatics_Impl: Sized {
    fn TargetNameProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToHorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ToVerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISwipeHintThemeAnimationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ISwipeHintThemeAnimationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISwipeHintThemeAnimationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwipeHintThemeAnimationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwipeHintThemeAnimationStatics_Vtbl {
        unsafe extern "system" fn TargetNameProperty<Impl: ISwipeHintThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToHorizontalOffsetProperty<Impl: ISwipeHintThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToVerticalOffsetProperty<Impl: ISwipeHintThemeAnimationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISwipeHintThemeAnimationStatics, BASE_OFFSET>(),
            TargetNameProperty: TargetNameProperty::<Impl, IMPL_OFFSET>,
            ToHorizontalOffsetProperty: ToHorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            ToVerticalOffsetProperty: ToVerticalOffsetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwipeHintThemeAnimationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITimeline_Impl: Sized {
    fn AutoReverse(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoReverse(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BeginTime(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>;
    fn SetBeginTime(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Duration>;
    fn SetDuration(&mut self, value: &super::super::Duration) -> ::windows::core::Result<()>;
    fn SpeedRatio(&mut self) -> ::windows::core::Result<f64>;
    fn SetSpeedRatio(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn FillBehavior(&mut self) -> ::windows::core::Result<FillBehavior>;
    fn SetFillBehavior(&mut self, value: FillBehavior) -> ::windows::core::Result<()>;
    fn RepeatBehavior(&mut self) -> ::windows::core::Result<RepeatBehavior>;
    fn SetRepeatBehavior(&mut self, value: &RepeatBehavior) -> ::windows::core::Result<()>;
    fn Completed(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimeline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITimeline";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITimeline_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeline_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeline_Vtbl {
        unsafe extern "system" fn AutoReverse<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoReverse<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoReverse(value).into()
        }
        unsafe extern "system" fn BeginTime<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBeginTime<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBeginTime(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Duration) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Duration as ::windows::core::Abi>::Abi as *const <super::super::Duration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpeedRatio<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSpeedRatio<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeedRatio(value).into()
        }
        unsafe extern "system" fn FillBehavior<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FillBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFillBehavior<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FillBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBehavior(value).into()
        }
        unsafe extern "system" fn RepeatBehavior<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRepeatBehavior<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RepeatBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepeatBehavior(&*(&value as *const <RepeatBehavior as ::windows::core::Abi>::Abi as *const <RepeatBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCompleted<Impl: ITimeline_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimeline, BASE_OFFSET>(),
            AutoReverse: AutoReverse::<Impl, IMPL_OFFSET>,
            SetAutoReverse: SetAutoReverse::<Impl, IMPL_OFFSET>,
            BeginTime: BeginTime::<Impl, IMPL_OFFSET>,
            SetBeginTime: SetBeginTime::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            SpeedRatio: SpeedRatio::<Impl, IMPL_OFFSET>,
            SetSpeedRatio: SetSpeedRatio::<Impl, IMPL_OFFSET>,
            FillBehavior: FillBehavior::<Impl, IMPL_OFFSET>,
            SetFillBehavior: SetFillBehavior::<Impl, IMPL_OFFSET>,
            RepeatBehavior: RepeatBehavior::<Impl, IMPL_OFFSET>,
            SetRepeatBehavior: SetRepeatBehavior::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeline as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Timeline>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITimelineFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITimelineFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimelineFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimelineFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineStatics_Impl: Sized {
    fn AllowDependentAnimations(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowDependentAnimations(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutoReverseProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BeginTimeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DurationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SpeedRatioProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FillBehaviorProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RepeatBehaviorProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimelineStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITimelineStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimelineStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimelineStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimelineStatics_Vtbl {
        unsafe extern "system" fn AllowDependentAnimations<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowDependentAnimations<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowDependentAnimations(value).into()
        }
        unsafe extern "system" fn AutoReverseProperty<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BeginTimeProperty<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DurationProperty<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SpeedRatioProperty<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FillBehaviorProperty<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RepeatBehaviorProperty<Impl: ITimelineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimelineStatics, BASE_OFFSET>(),
            AllowDependentAnimations: AllowDependentAnimations::<Impl, IMPL_OFFSET>,
            SetAllowDependentAnimations: SetAllowDependentAnimations::<Impl, IMPL_OFFSET>,
            AutoReverseProperty: AutoReverseProperty::<Impl, IMPL_OFFSET>,
            BeginTimeProperty: BeginTimeProperty::<Impl, IMPL_OFFSET>,
            DurationProperty: DurationProperty::<Impl, IMPL_OFFSET>,
            SpeedRatioProperty: SpeedRatioProperty::<Impl, IMPL_OFFSET>,
            FillBehaviorProperty: FillBehaviorProperty::<Impl, IMPL_OFFSET>,
            RepeatBehaviorProperty: RepeatBehaviorProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimelineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransition_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITransition";
}
#[cfg(feature = "implement_exclusive")]
impl ITransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransition_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransition, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransitionFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ITransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITransitionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransitionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransitionFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITransitionFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransitionFactory as ::windows::core::Interface>::IID
    }
}
