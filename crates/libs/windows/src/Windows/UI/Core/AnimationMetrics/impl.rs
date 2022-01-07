#[cfg(feature = "implement_exclusive")]
pub trait IAnimationDescriptionImpl: Sized {
    fn Animations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>>;
    fn StaggerDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn StaggerDelayFactor(&self) -> ::windows::core::Result<f32>;
    fn DelayLimit(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn ZOrder(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationDescription {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IAnimationDescription";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationDescriptionVtbl {
    pub const fn new<Impl: IAnimationDescriptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAnimationDescriptionVtbl {
        unsafe extern "system" fn Animations<Impl: IAnimationDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Animations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StaggerDelay<Impl: IAnimationDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StaggerDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StaggerDelayFactor<Impl: IAnimationDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StaggerDelayFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelayLimit<Impl: IAnimationDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DelayLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZOrder<Impl: IAnimationDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAnimationDescription>, base.5, Animations::<Impl, OFFSET>, StaggerDelay::<Impl, OFFSET>, StaggerDelayFactor::<Impl, OFFSET>, DelayLimit::<Impl, OFFSET>, ZOrder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationDescriptionFactoryImpl: Sized {
    fn CreateInstance(&self, effect: AnimationEffect, target: AnimationEffectTarget) -> ::windows::core::Result<AnimationDescription>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationDescriptionFactory {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IAnimationDescriptionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationDescriptionFactoryVtbl {
    pub const fn new<Impl: IAnimationDescriptionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAnimationDescriptionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAnimationDescriptionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: AnimationEffect, target: AnimationEffectTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(effect, target) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAnimationDescriptionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOpacityAnimationImpl: Sized + IPropertyAnimationImpl {
    fn InitialOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn FinalOpacity(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOpacityAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IOpacityAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IOpacityAnimationVtbl {
    pub const fn new<Impl: IOpacityAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpacityAnimationVtbl {
        unsafe extern "system" fn InitialOpacity<Impl: IOpacityAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitialOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalOpacity<Impl: IOpacityAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinalOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpacityAnimation>, base.5, InitialOpacity::<Impl, OFFSET>, FinalOpacity::<Impl, OFFSET>)
    }
}
pub trait IPropertyAnimationImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PropertyAnimationType>;
    fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
impl ::windows::core::RuntimeName for IPropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IPropertyAnimation";
}
impl IPropertyAnimationVtbl {
    pub const fn new<Impl: IPropertyAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyAnimationVtbl {
        unsafe extern "system" fn Type<Impl: IPropertyAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PropertyAnimationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delay<Impl: IPropertyAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IPropertyAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Control1<Impl: IPropertyAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Control1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Control2<Impl: IPropertyAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Control2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyAnimation>, base.5, Type::<Impl, OFFSET>, Delay::<Impl, OFFSET>, Duration::<Impl, OFFSET>, Control1::<Impl, OFFSET>, Control2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleAnimationImpl: Sized + IPropertyAnimationImpl {
    fn InitialScaleX(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn InitialScaleY(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn FinalScaleX(&self) -> ::windows::core::Result<f32>;
    fn FinalScaleY(&self) -> ::windows::core::Result<f32>;
    fn NormalizedOrigin(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScaleAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IScaleAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IScaleAnimationVtbl {
    pub const fn new<Impl: IScaleAnimationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScaleAnimationVtbl {
        unsafe extern "system" fn InitialScaleX<Impl: IScaleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitialScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialScaleY<Impl: IScaleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitialScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalScaleX<Impl: IScaleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinalScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalScaleY<Impl: IScaleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinalScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedOrigin<Impl: IScaleAnimationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalizedOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScaleAnimation>, base.5, InitialScaleX::<Impl, OFFSET>, InitialScaleY::<Impl, OFFSET>, FinalScaleX::<Impl, OFFSET>, FinalScaleY::<Impl, OFFSET>, NormalizedOrigin::<Impl, OFFSET>)
    }
}
