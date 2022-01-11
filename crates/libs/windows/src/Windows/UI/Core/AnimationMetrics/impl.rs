#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAnimationDescriptionImpl: Sized {
    fn Animations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>>;
    fn StaggerDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn StaggerDelayFactor(&self) -> ::windows::core::Result<f32>;
    fn DelayLimit(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn ZOrder(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAnimationDescription {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IAnimationDescription";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAnimationDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationDescriptionVtbl {
        unsafe extern "system" fn Animations<Impl: IAnimationDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Animations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StaggerDelay<Impl: IAnimationDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaggerDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StaggerDelayFactor<Impl: IAnimationDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaggerDelayFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelayLimit<Impl: IAnimationDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelayLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZOrder<Impl: IAnimationDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnimationDescription>, ::windows::core::GetTrustLevel, Animations::<Impl, IMPL_OFFSET>, StaggerDelay::<Impl, IMPL_OFFSET>, StaggerDelayFactor::<Impl, IMPL_OFFSET>, DelayLimit::<Impl, IMPL_OFFSET>, ZOrder::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationDescription as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationDescriptionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationDescriptionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAnimationDescriptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: AnimationEffect, target: AnimationEffectTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(effect, target) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnimationDescriptionFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationDescriptionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOpacityAnimationImpl: Sized + IPropertyAnimationImpl {
    fn InitialOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn FinalOpacity(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOpacityAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IOpacityAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOpacityAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpacityAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpacityAnimationVtbl {
        unsafe extern "system" fn InitialOpacity<Impl: IOpacityAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalOpacity<Impl: IOpacityAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOpacityAnimation>, ::windows::core::GetTrustLevel, InitialOpacity::<Impl, IMPL_OFFSET>, FinalOpacity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpacityAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPropertyAnimationImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PropertyAnimationType>;
    fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IPropertyAnimation";
}
#[cfg(feature = "Foundation")]
impl IPropertyAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyAnimationVtbl {
        unsafe extern "system" fn Type<Impl: IPropertyAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PropertyAnimationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delay<Impl: IPropertyAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IPropertyAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Control1<Impl: IPropertyAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Control2<Impl: IPropertyAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropertyAnimation>, ::windows::core::GetTrustLevel, Type::<Impl, IMPL_OFFSET>, Delay::<Impl, IMPL_OFFSET>, Duration::<Impl, IMPL_OFFSET>, Control1::<Impl, IMPL_OFFSET>, Control2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScaleAnimationImpl: Sized + IPropertyAnimationImpl {
    fn InitialScaleX(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn InitialScaleY(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn FinalScaleX(&self) -> ::windows::core::Result<f32>;
    fn FinalScaleY(&self) -> ::windows::core::Result<f32>;
    fn NormalizedOrigin(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScaleAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IScaleAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScaleAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScaleAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScaleAnimationVtbl {
        unsafe extern "system" fn InitialScaleX<Impl: IScaleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialScaleY<Impl: IScaleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalScaleX<Impl: IScaleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalScaleX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalScaleY<Impl: IScaleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalScaleY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedOrigin<Impl: IScaleAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScaleAnimation>, ::windows::core::GetTrustLevel, InitialScaleX::<Impl, IMPL_OFFSET>, InitialScaleY::<Impl, IMPL_OFFSET>, FinalScaleX::<Impl, IMPL_OFFSET>, FinalScaleY::<Impl, IMPL_OFFSET>, NormalizedOrigin::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScaleAnimation as ::windows::core::Interface>::IID
    }
}
