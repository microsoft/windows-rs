#[cfg(feature = "implement_exclusive")]
pub trait IAmbientLightImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAmbientLight {
    const NAME: &'static str = "Windows.UI.Composition.IAmbientLight";
}
#[cfg(feature = "implement_exclusive")]
impl IAmbientLightVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAmbientLightImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAmbientLightVtbl {
        unsafe extern "system" fn Color<Impl: IAmbientLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IAmbientLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAmbientLight, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAmbientLight as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAmbientLight2Impl: Sized {
    fn Intensity(&self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAmbientLight2 {
    const NAME: &'static str = "Windows.UI.Composition.IAmbientLight2";
}
#[cfg(feature = "implement_exclusive")]
impl IAmbientLight2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAmbientLight2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAmbientLight2Vtbl {
        unsafe extern "system" fn Intensity<Impl: IAmbientLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Intensity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntensity<Impl: IAmbientLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntensity(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAmbientLight2, BASE_OFFSET>(),
            Intensity: Intensity::<Impl, IMPL_OFFSET>,
            SetIntensity: SetIntensity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAmbientLight2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationControllerImpl: Sized {
    fn PlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn SetPlaybackRate(&self, value: f32) -> ::windows::core::Result<()>;
    fn Progress(&self) -> ::windows::core::Result<f32>;
    fn SetProgress(&self, value: f32) -> ::windows::core::Result<()>;
    fn ProgressBehavior(&self) -> ::windows::core::Result<AnimationControllerProgressBehavior>;
    fn SetProgressBehavior(&self, value: AnimationControllerProgressBehavior) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationController {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationController";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationControllerVtbl {
        unsafe extern "system" fn PlaybackRate<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackRate<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn Progress<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(value).into()
        }
        unsafe extern "system" fn ProgressBehavior<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationControllerProgressBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgressBehavior<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationControllerProgressBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgressBehavior(value).into()
        }
        unsafe extern "system" fn Pause<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IAnimationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnimationController, BASE_OFFSET>(),
            PlaybackRate: PlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate: SetPlaybackRate::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            SetProgress: SetProgress::<Impl, IMPL_OFFSET>,
            ProgressBehavior: ProgressBehavior::<Impl, IMPL_OFFSET>,
            SetProgressBehavior: SetProgressBehavior::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationControllerStaticsImpl: Sized {
    fn MaxPlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn MinPlaybackRate(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationControllerStatics {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationControllerStaticsVtbl {
        unsafe extern "system" fn MaxPlaybackRate<Impl: IAnimationControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPlaybackRate<Impl: IAnimationControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnimationControllerStatics, BASE_OFFSET>(),
            MaxPlaybackRate: MaxPlaybackRate::<Impl, IMPL_OFFSET>,
            MinPlaybackRate: MinPlaybackRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationControllerStatics as ::windows::core::Interface>::IID
    }
}
pub trait IAnimationObjectImpl: Sized {
    fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &::core::option::Option<AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
impl IAnimationObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationObjectVtbl {
        unsafe extern "system" fn PopulatePropertyInfo<Impl: IAnimationObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopulatePropertyInfo(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyinfo as *const <AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnimationObject, BASE_OFFSET>(),
            PopulatePropertyInfo: PopulatePropertyInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationPropertyInfoImpl: Sized {
    fn AccessMode(&self) -> ::windows::core::Result<AnimationPropertyAccessMode>;
    fn SetAccessMode(&self, value: AnimationPropertyAccessMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationPropertyInfo {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationPropertyInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationPropertyInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationPropertyInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationPropertyInfoVtbl {
        unsafe extern "system" fn AccessMode<Impl: IAnimationPropertyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationPropertyAccessMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessMode<Impl: IAnimationPropertyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationPropertyAccessMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnimationPropertyInfo, BASE_OFFSET>(),
            AccessMode: AccessMode::<Impl, IMPL_OFFSET>,
            SetAccessMode: SetAccessMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationPropertyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationPropertyInfo2Impl: Sized {
    fn GetResolvedCompositionObject(&self) -> ::windows::core::Result<CompositionObject>;
    fn GetResolvedCompositionObjectProperty(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationPropertyInfo2 {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationPropertyInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationPropertyInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationPropertyInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationPropertyInfo2Vtbl {
        unsafe extern "system" fn GetResolvedCompositionObject<Impl: IAnimationPropertyInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResolvedCompositionObject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolvedCompositionObjectProperty<Impl: IAnimationPropertyInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResolvedCompositionObjectProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnimationPropertyInfo2, BASE_OFFSET>(),
            GetResolvedCompositionObject: GetResolvedCompositionObject::<Impl, IMPL_OFFSET>,
            GetResolvedCompositionObjectProperty: GetResolvedCompositionObjectProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationPropertyInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Amplitude(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IBackEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IBackEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackEasingFunctionVtbl {
        unsafe extern "system" fn Mode<Impl: IBackEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Amplitude<Impl: IBackEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackEasingFunction, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Amplitude: Amplitude::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBooleanKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBooleanKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBooleanKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBooleanKeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBooleanKeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBooleanKeyFrameAnimationVtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IBooleanKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBooleanKeyFrameAnimation, BASE_OFFSET>(),
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBooleanKeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Bounces(&self) -> ::windows::core::Result<i32>;
    fn Bounciness(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IBounceEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceEasingFunctionVtbl {
        unsafe extern "system" fn Mode<Impl: IBounceEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bounces<Impl: IBounceEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Bounciness<Impl: IBounceEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBounceEasingFunction, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Bounces: Bounces::<Impl, IMPL_OFFSET>,
            Bounciness: Bounciness::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceScalarNaturalMotionAnimationImpl: Sized {
    fn Acceleration(&self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceScalarNaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBounceScalarNaturalMotionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceScalarNaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceScalarNaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceScalarNaturalMotionAnimationVtbl {
        unsafe extern "system" fn Acceleration<Impl: IBounceScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleration<Impl: IBounceScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleration(value).into()
        }
        unsafe extern "system" fn Restitution<Impl: IBounceScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restitution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestitution<Impl: IBounceScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestitution(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBounceScalarNaturalMotionAnimation, BASE_OFFSET>(),
            Acceleration: Acceleration::<Impl, IMPL_OFFSET>,
            SetAcceleration: SetAcceleration::<Impl, IMPL_OFFSET>,
            Restitution: Restitution::<Impl, IMPL_OFFSET>,
            SetRestitution: SetRestitution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceScalarNaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceVector2NaturalMotionAnimationImpl: Sized {
    fn Acceleration(&self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceVector2NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBounceVector2NaturalMotionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceVector2NaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceVector2NaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceVector2NaturalMotionAnimationVtbl {
        unsafe extern "system" fn Acceleration<Impl: IBounceVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleration<Impl: IBounceVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleration(value).into()
        }
        unsafe extern "system" fn Restitution<Impl: IBounceVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restitution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestitution<Impl: IBounceVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestitution(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBounceVector2NaturalMotionAnimation, BASE_OFFSET>(),
            Acceleration: Acceleration::<Impl, IMPL_OFFSET>,
            SetAcceleration: SetAcceleration::<Impl, IMPL_OFFSET>,
            Restitution: Restitution::<Impl, IMPL_OFFSET>,
            SetRestitution: SetRestitution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceVector2NaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceVector3NaturalMotionAnimationImpl: Sized {
    fn Acceleration(&self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceVector3NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBounceVector3NaturalMotionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceVector3NaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceVector3NaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceVector3NaturalMotionAnimationVtbl {
        unsafe extern "system" fn Acceleration<Impl: IBounceVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleration<Impl: IBounceVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleration(value).into()
        }
        unsafe extern "system" fn Restitution<Impl: IBounceVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restitution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestitution<Impl: IBounceVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestitution(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBounceVector3NaturalMotionAnimation, BASE_OFFSET>(),
            Acceleration: Acceleration::<Impl, IMPL_OFFSET>,
            SetAcceleration: SetAcceleration::<Impl, IMPL_OFFSET>,
            Restitution: Restitution::<Impl, IMPL_OFFSET>,
            SetRestitution: SetRestitution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBounceVector3NaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICircleEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICircleEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ICircleEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ICircleEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICircleEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICircleEasingFunctionVtbl {
        unsafe extern "system" fn Mode<Impl: ICircleEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICircleEasingFunction, BASE_OFFSET>(), Mode: Mode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICircleEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameAnimationImpl: Sized {
    fn InterpolationColorSpace(&self) -> ::windows::core::Result<CompositionColorSpace>;
    fn SetInterpolationColorSpace(&self, value: CompositionColorSpace) -> ::windows::core::Result<()>;
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::Color) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::Color, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IColorKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IColorKeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrameAnimationVtbl {
        unsafe extern "system" fn InterpolationColorSpace<Impl: IColorKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterpolationColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterpolationColorSpace<Impl: IColorKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterpolationColorSpace(value).into()
        }
        unsafe extern "system" fn InsertKeyFrame<Impl: IColorKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IColorKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::Color, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrameWithEasingFunction(normalizedprogresskey, &*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType), &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorKeyFrameAnimation, BASE_OFFSET>(),
            InterpolationColorSpace: InterpolationColorSpace::<Impl, IMPL_OFFSET>,
            SetInterpolationColorSpace: SetInterpolationColorSpace::<Impl, IMPL_OFFSET>,
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
            InsertKeyFrameWithEasingFunction: InsertKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorKeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionAnimationImpl: Sized {
    fn ClearAllParameters(&self) -> ::windows::core::Result<()>;
    fn ClearParameter(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetColorParameter(&self, key: &::windows::core::HSTRING, value: &super::Color) -> ::windows::core::Result<()>;
    fn SetMatrix3x2Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetMatrix4x4Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn SetQuaternionParameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn SetReferenceParameter(&self, key: &::windows::core::HSTRING, compositionobject: &::core::option::Option<CompositionObject>) -> ::windows::core::Result<()>;
    fn SetScalarParameter(&self, key: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn SetVector2Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn SetVector3Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetVector4Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationVtbl {
        unsafe extern "system" fn ClearAllParameters<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllParameters().into()
        }
        unsafe extern "system" fn ClearParameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetColorParameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMatrix3x2Parameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix3x2Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMatrix4x4Parameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix4x4Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetQuaternionParameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuaternionParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetReferenceParameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, compositionobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferenceParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&compositionobject as *const <CompositionObject as ::windows::core::Abi>::Abi as *const <CompositionObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetScalarParameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScalarParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetVector2Parameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVector2Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetVector3Parameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVector3Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetVector4Parameter<Impl: ICompositionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVector4Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimation, BASE_OFFSET>(),
            ClearAllParameters: ClearAllParameters::<Impl, IMPL_OFFSET>,
            ClearParameter: ClearParameter::<Impl, IMPL_OFFSET>,
            SetColorParameter: SetColorParameter::<Impl, IMPL_OFFSET>,
            SetMatrix3x2Parameter: SetMatrix3x2Parameter::<Impl, IMPL_OFFSET>,
            SetMatrix4x4Parameter: SetMatrix4x4Parameter::<Impl, IMPL_OFFSET>,
            SetQuaternionParameter: SetQuaternionParameter::<Impl, IMPL_OFFSET>,
            SetReferenceParameter: SetReferenceParameter::<Impl, IMPL_OFFSET>,
            SetScalarParameter: SetScalarParameter::<Impl, IMPL_OFFSET>,
            SetVector2Parameter: SetVector2Parameter::<Impl, IMPL_OFFSET>,
            SetVector3Parameter: SetVector3Parameter::<Impl, IMPL_OFFSET>,
            SetVector4Parameter: SetVector4Parameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimation2Impl: Sized {
    fn SetBooleanParameter(&self, key: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTarget(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimation2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimation2Vtbl {
        unsafe extern "system" fn SetBooleanParameter<Impl: ICompositionAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBooleanParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn Target<Impl: ICompositionAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTarget<Impl: ICompositionAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimation2, BASE_OFFSET>(),
            SetBooleanParameter: SetBooleanParameter::<Impl, IMPL_OFFSET>,
            Target: Target::<Impl, IMPL_OFFSET>,
            SetTarget: SetTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICompositionAnimation3Impl: Sized {
    fn InitialValueExpressions(&self) -> ::windows::core::Result<InitialValueExpressionCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionAnimation3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompositionAnimation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimation3Vtbl {
        unsafe extern "system" fn InitialValueExpressions<Impl: ICompositionAnimation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialValueExpressions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimation3, BASE_OFFSET>(),
            InitialValueExpressions: InitialValueExpressions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimation4Impl: Sized {
    fn SetExpressionReferenceParameter(&self, parametername: &::windows::core::HSTRING, source: &::core::option::Option<IAnimationObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimation4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimation4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimation4Vtbl {
        unsafe extern "system" fn SetExpressionReferenceParameter<Impl: ICompositionAnimation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parametername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpressionReferenceParameter(&*(&parametername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&source as *const <IAnimationObject as ::windows::core::Abi>::Abi as *const <IAnimationObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimation4, BASE_OFFSET>(),
            SetExpressionReferenceParameter: SetExpressionReferenceParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimation4 as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionAnimationBaseImpl: Sized {}
impl ::windows::core::RuntimeName for ICompositionAnimationBase {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationBase";
}
impl ICompositionAnimationBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationBaseVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimationBase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimationBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimationGroupImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, value: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn Remove(&self, value: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimationGroup {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationGroup";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimationGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationGroupVtbl {
        unsafe extern "system" fn Count<Impl: ICompositionAnimationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICompositionAnimationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&value as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICompositionAnimationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&value as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionAnimationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimationGroup, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimationGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBackdropBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBackdropBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBackdropBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBackdropBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBackdropBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBackdropBrushVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBackdropBrush, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBackdropBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBatchCompletedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBatchCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBatchCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBatchCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBatchCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBatchCompletedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBatchCompletedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBatchCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBrushVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBrush, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBrushFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBrushFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBrushFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBrushFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBrushFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBrushFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositionCapabilitiesImpl: Sized {
    fn AreEffectsSupported(&self) -> ::windows::core::Result<bool>;
    fn AreEffectsFast(&self) -> ::windows::core::Result<bool>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CompositionCapabilities, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionCapabilities {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionCapabilities";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionCapabilitiesVtbl {
        unsafe extern "system" fn AreEffectsSupported<Impl: ICompositionCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreEffectsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreEffectsFast<Impl: ICompositionCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreEffectsFast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: ICompositionCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CompositionCapabilities, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CompositionCapabilities, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: ICompositionCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionCapabilities, BASE_OFFSET>(),
            AreEffectsSupported: AreEffectsSupported::<Impl, IMPL_OFFSET>,
            AreEffectsFast: AreEffectsFast::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionCapabilitiesStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CompositionCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionCapabilitiesStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionCapabilitiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionCapabilitiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionCapabilitiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionCapabilitiesStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICompositionCapabilitiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionCapabilitiesStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionCapabilitiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionClipImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionClip {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionClip";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionClipVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionClip, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionClip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionClip2Impl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionClip2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionClip2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionClip2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionClip2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionClip2Vtbl {
        unsafe extern "system" fn AnchorPoint<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnchorPoint<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionClip2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionClip2, BASE_OFFSET>(),
            AnchorPoint: AnchorPoint::<Impl, IMPL_OFFSET>,
            SetAnchorPoint: SetAnchorPoint::<Impl, IMPL_OFFSET>,
            CenterPoint: CenterPoint::<Impl, IMPL_OFFSET>,
            SetCenterPoint: SetCenterPoint::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            RotationAngle: RotationAngle::<Impl, IMPL_OFFSET>,
            SetRotationAngle: SetRotationAngle::<Impl, IMPL_OFFSET>,
            RotationAngleInDegrees: RotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetRotationAngleInDegrees: SetRotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
            TransformMatrix: TransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionClip2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionClipFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionClipFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionClipFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionClipFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionClipFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionClipFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionClipFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionClipFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionColorBrushImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionColorBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionColorBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionColorBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionColorBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionColorBrushVtbl {
        unsafe extern "system" fn Color<Impl: ICompositionColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ICompositionColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionColorBrush, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionColorBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionColorGradientStopImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<f32>;
    fn SetOffset(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionColorGradientStop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionColorGradientStop";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionColorGradientStopVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionColorGradientStopImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionColorGradientStopVtbl {
        unsafe extern "system" fn Color<Impl: ICompositionColorGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ICompositionColorGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionColorGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionColorGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionColorGradientStop, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionColorGradientStop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionColorGradientStopCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionColorGradientStopCollection {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionColorGradientStopCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionColorGradientStopCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionColorGradientStopCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionColorGradientStopCollectionVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionColorGradientStopCollection, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionColorGradientStopCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositionCommitBatchImpl: Sized {
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn IsEnded(&self) -> ::windows::core::Result<bool>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionCommitBatch {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionCommitBatch";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionCommitBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionCommitBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionCommitBatchVtbl {
        unsafe extern "system" fn IsActive<Impl: ICompositionCommitBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnded<Impl: ICompositionCommitBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Completed<Impl: ICompositionCommitBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: ICompositionCommitBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionCommitBatch, BASE_OFFSET>(),
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            IsEnded: IsEnded::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionCommitBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICompositionContainerShapeImpl: Sized {
    fn Shapes(&self) -> ::windows::core::Result<CompositionShapeCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionContainerShape {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionContainerShape";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompositionContainerShapeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionContainerShapeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionContainerShapeVtbl {
        unsafe extern "system" fn Shapes<Impl: ICompositionContainerShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shapes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionContainerShape, BASE_OFFSET>(), Shapes: Shapes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionContainerShape as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ICompositionDrawingSurfaceImpl: Sized {
    fn AlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXAlphaMode>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionDrawingSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionDrawingSurface";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionDrawingSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurfaceVtbl {
        unsafe extern "system" fn AlphaMode<Impl: ICompositionDrawingSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelFormat<Impl: ICompositionDrawingSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: ICompositionDrawingSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionDrawingSurface, BASE_OFFSET>(),
            AlphaMode: AlphaMode::<Impl, IMPL_OFFSET>,
            PixelFormat: PixelFormat::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDrawingSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait ICompositionDrawingSurface2Impl: Sized {
    fn SizeInt32(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn Resize(&self, sizepixels: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn Scroll(&self, offset: &super::super::Graphics::PointInt32) -> ::windows::core::Result<()>;
    fn ScrollRect(&self, offset: &super::super::Graphics::PointInt32, scrollrect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
    fn ScrollWithClip(&self, offset: &super::super::Graphics::PointInt32, cliprect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
    fn ScrollRectWithClip(&self, offset: &super::super::Graphics::PointInt32, cliprect: &super::super::Graphics::RectInt32, scrollrect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionDrawingSurface2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionDrawingSurface2";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ICompositionDrawingSurface2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurface2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurface2Vtbl {
        unsafe extern "system" fn SizeInt32<Impl: ICompositionDrawingSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Impl: ICompositionDrawingSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(&*(&sizepixels as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Scroll<Impl: ICompositionDrawingSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(&*(&offset as *const <super::super::Graphics::PointInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::PointInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollRect<Impl: ICompositionDrawingSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32, scrollrect: super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollRect(&*(&offset as *const <super::super::Graphics::PointInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::PointInt32 as ::windows::core::DefaultType>::DefaultType), &*(&scrollrect as *const <super::super::Graphics::RectInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollWithClip<Impl: ICompositionDrawingSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32, cliprect: super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollWithClip(&*(&offset as *const <super::super::Graphics::PointInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::PointInt32 as ::windows::core::DefaultType>::DefaultType), &*(&cliprect as *const <super::super::Graphics::RectInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollRectWithClip<Impl: ICompositionDrawingSurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32, cliprect: super::super::Graphics::RectInt32, scrollrect: super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ScrollRectWithClip(
                    &*(&offset as *const <super::super::Graphics::PointInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::PointInt32 as ::windows::core::DefaultType>::DefaultType),
                    &*(&cliprect as *const <super::super::Graphics::RectInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType),
                    &*(&scrollrect as *const <super::super::Graphics::RectInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionDrawingSurface2, BASE_OFFSET>(),
            SizeInt32: SizeInt32::<Impl, IMPL_OFFSET>,
            Resize: Resize::<Impl, IMPL_OFFSET>,
            Scroll: Scroll::<Impl, IMPL_OFFSET>,
            ScrollRect: ScrollRect::<Impl, IMPL_OFFSET>,
            ScrollWithClip: ScrollWithClip::<Impl, IMPL_OFFSET>,
            ScrollRectWithClip: ScrollRectWithClip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDrawingSurface2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDrawingSurfaceFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDrawingSurfaceFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionDrawingSurfaceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDrawingSurfaceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurfaceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurfaceFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionDrawingSurfaceFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEasingFunctionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEasingFunctionVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEasingFunction, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEasingFunctionFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEasingFunctionFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEasingFunctionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEasingFunctionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEasingFunctionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEasingFunctionFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEasingFunctionFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEasingFunctionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionEasingFunctionStaticsImpl: Sized {
    fn CreateCubicBezierEasingFunction(&self, owner: &::core::option::Option<Compositor>, controlpoint1: &super::super::Foundation::Numerics::Vector2, controlpoint2: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CubicBezierEasingFunction>;
    fn CreateLinearEasingFunction(&self, owner: &::core::option::Option<Compositor>) -> ::windows::core::Result<LinearEasingFunction>;
    fn CreateStepEasingFunction(&self, owner: &::core::option::Option<Compositor>) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateStepEasingFunctionWithStepCount(&self, owner: &::core::option::Option<Compositor>, stepcount: i32) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateBackEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, amplitude: f32) -> ::windows::core::Result<BackEasingFunction>;
    fn CreateBounceEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, bounces: i32, bounciness: f32) -> ::windows::core::Result<BounceEasingFunction>;
    fn CreateCircleEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode) -> ::windows::core::Result<CircleEasingFunction>;
    fn CreateElasticEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, oscillations: i32, springiness: f32) -> ::windows::core::Result<ElasticEasingFunction>;
    fn CreateExponentialEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, exponent: f32) -> ::windows::core::Result<ExponentialEasingFunction>;
    fn CreatePowerEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, power: f32) -> ::windows::core::Result<PowerEasingFunction>;
    fn CreateSineEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode) -> ::windows::core::Result<SineEasingFunction>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionEasingFunctionStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEasingFunctionStatics";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionEasingFunctionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEasingFunctionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEasingFunctionStaticsVtbl {
        unsafe extern "system" fn CreateCubicBezierEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, controlpoint1: super::super::Foundation::Numerics::Vector2, controlpoint2: super::super::Foundation::Numerics::Vector2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCubicBezierEasingFunction(
                &*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType),
                &*(&controlpoint1 as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                &*(&controlpoint2 as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStepEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStepEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStepEasingFunctionWithStepCount<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, stepcount: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStepEasingFunctionWithStepCount(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), stepcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBackEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, amplitude: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBackEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), mode, amplitude) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBounceEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, bounces: i32, bounciness: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBounceEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), mode, bounces, bounciness) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCircleEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCircleEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateElasticEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, oscillations: i32, springiness: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateElasticEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), mode, oscillations, springiness) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateExponentialEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, exponent: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateExponentialEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), mode, exponent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePowerEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, power: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePowerEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), mode, power) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSineEasingFunction<Impl: ICompositionEasingFunctionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSineEasingFunction(&*(&owner as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEasingFunctionStatics, BASE_OFFSET>(),
            CreateCubicBezierEasingFunction: CreateCubicBezierEasingFunction::<Impl, IMPL_OFFSET>,
            CreateLinearEasingFunction: CreateLinearEasingFunction::<Impl, IMPL_OFFSET>,
            CreateStepEasingFunction: CreateStepEasingFunction::<Impl, IMPL_OFFSET>,
            CreateStepEasingFunctionWithStepCount: CreateStepEasingFunctionWithStepCount::<Impl, IMPL_OFFSET>,
            CreateBackEasingFunction: CreateBackEasingFunction::<Impl, IMPL_OFFSET>,
            CreateBounceEasingFunction: CreateBounceEasingFunction::<Impl, IMPL_OFFSET>,
            CreateCircleEasingFunction: CreateCircleEasingFunction::<Impl, IMPL_OFFSET>,
            CreateElasticEasingFunction: CreateElasticEasingFunction::<Impl, IMPL_OFFSET>,
            CreateExponentialEasingFunction: CreateExponentialEasingFunction::<Impl, IMPL_OFFSET>,
            CreatePowerEasingFunction: CreatePowerEasingFunction::<Impl, IMPL_OFFSET>,
            CreateSineEasingFunction: CreateSineEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEasingFunctionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectBrushImpl: Sized {
    fn GetSourceParameter(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<CompositionBrush>;
    fn SetSourceParameter(&self, name: &::windows::core::HSTRING, source: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectBrushVtbl {
        unsafe extern "system" fn GetSourceParameter<Impl: ICompositionEffectBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceParameter(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceParameter<Impl: ICompositionEffectBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceParameter(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&source as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEffectBrush, BASE_OFFSET>(),
            GetSourceParameter: GetSourceParameter::<Impl, IMPL_OFFSET>,
            SetSourceParameter: SetSourceParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEffectBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectFactoryImpl: Sized {
    fn CreateBrush(&self) -> ::windows::core::Result<CompositionEffectBrush>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn LoadStatus(&self) -> ::windows::core::Result<CompositionEffectFactoryLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectFactoryVtbl {
        unsafe extern "system" fn CreateBrush<Impl: ICompositionEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: ICompositionEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStatus<Impl: ICompositionEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEffectFactoryLoadStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEffectFactory, BASE_OFFSET>(),
            CreateBrush: CreateBrush::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            LoadStatus: LoadStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEffectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectSourceParameterImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectSourceParameter {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectSourceParameter";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectSourceParameterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectSourceParameterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectSourceParameterVtbl {
        unsafe extern "system" fn Name<Impl: ICompositionEffectSourceParameterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEffectSourceParameter, BASE_OFFSET>(), Name: Name::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEffectSourceParameter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectSourceParameterFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<CompositionEffectSourceParameter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectSourceParameterFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectSourceParameterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectSourceParameterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectSourceParameterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectSourceParameterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICompositionEffectSourceParameterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEffectSourceParameterFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEffectSourceParameterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionEllipseGeometryImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenter(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Radius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionEllipseGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEllipseGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionEllipseGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEllipseGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEllipseGeometryVtbl {
        unsafe extern "system" fn Center<Impl: ICompositionEllipseGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: ICompositionEllipseGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Radius<Impl: ICompositionEllipseGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Radius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadius<Impl: ICompositionEllipseGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEllipseGeometry, BASE_OFFSET>(),
            Center: Center::<Impl, IMPL_OFFSET>,
            SetCenter: SetCenter::<Impl, IMPL_OFFSET>,
            Radius: Radius::<Impl, IMPL_OFFSET>,
            SetRadius: SetRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEllipseGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGeometricClipImpl: Sized {
    fn Geometry(&self) -> ::windows::core::Result<CompositionGeometry>;
    fn SetGeometry(&self, value: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<()>;
    fn ViewBox(&self) -> ::windows::core::Result<CompositionViewBox>;
    fn SetViewBox(&self, value: &::core::option::Option<CompositionViewBox>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGeometricClip {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGeometricClip";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGeometricClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGeometricClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGeometricClipVtbl {
        unsafe extern "system" fn Geometry<Impl: ICompositionGeometricClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Geometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometry<Impl: ICompositionGeometricClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometry(&*(&value as *const <CompositionGeometry as ::windows::core::Abi>::Abi as *const <CompositionGeometry as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ViewBox<Impl: ICompositionGeometricClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewBox<Impl: ICompositionGeometricClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewBox(&*(&value as *const <CompositionViewBox as ::windows::core::Abi>::Abi as *const <CompositionViewBox as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGeometricClip, BASE_OFFSET>(),
            Geometry: Geometry::<Impl, IMPL_OFFSET>,
            SetGeometry: SetGeometry::<Impl, IMPL_OFFSET>,
            ViewBox: ViewBox::<Impl, IMPL_OFFSET>,
            SetViewBox: SetViewBox::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGeometricClip as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGeometryImpl: Sized {
    fn TrimEnd(&self) -> ::windows::core::Result<f32>;
    fn SetTrimEnd(&self, value: f32) -> ::windows::core::Result<()>;
    fn TrimOffset(&self) -> ::windows::core::Result<f32>;
    fn SetTrimOffset(&self, value: f32) -> ::windows::core::Result<()>;
    fn TrimStart(&self) -> ::windows::core::Result<f32>;
    fn SetTrimStart(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGeometryVtbl {
        unsafe extern "system" fn TrimEnd<Impl: ICompositionGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimEnd<Impl: ICompositionGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimEnd(value).into()
        }
        unsafe extern "system" fn TrimOffset<Impl: ICompositionGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimOffset<Impl: ICompositionGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimOffset(value).into()
        }
        unsafe extern "system" fn TrimStart<Impl: ICompositionGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimStart<Impl: ICompositionGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimStart(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGeometry, BASE_OFFSET>(),
            TrimEnd: TrimEnd::<Impl, IMPL_OFFSET>,
            SetTrimEnd: SetTrimEnd::<Impl, IMPL_OFFSET>,
            TrimOffset: TrimOffset::<Impl, IMPL_OFFSET>,
            SetTrimOffset: SetTrimOffset::<Impl, IMPL_OFFSET>,
            TrimStart: TrimStart::<Impl, IMPL_OFFSET>,
            SetTrimStart: SetTrimStart::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGeometryFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGeometryFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGeometryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGeometryFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGeometryFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGeometryFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGeometryFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGeometryFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionGradientBrushImpl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ColorStops(&self) -> ::windows::core::Result<CompositionColorGradientStopCollection>;
    fn ExtendMode(&self) -> ::windows::core::Result<CompositionGradientExtendMode>;
    fn SetExtendMode(&self, value: CompositionGradientExtendMode) -> ::windows::core::Result<()>;
    fn InterpolationSpace(&self) -> ::windows::core::Result<CompositionColorSpace>;
    fn SetInterpolationSpace(&self, value: CompositionColorSpace) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGradientBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGradientBrush";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGradientBrushVtbl {
        unsafe extern "system" fn AnchorPoint<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnchorPoint<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ColorStops<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorStops() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendMode<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionGradientExtendMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendMode<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionGradientExtendMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendMode(value).into()
        }
        unsafe extern "system" fn InterpolationSpace<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterpolationSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterpolationSpace<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterpolationSpace(value).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGradientBrush, BASE_OFFSET>(),
            AnchorPoint: AnchorPoint::<Impl, IMPL_OFFSET>,
            SetAnchorPoint: SetAnchorPoint::<Impl, IMPL_OFFSET>,
            CenterPoint: CenterPoint::<Impl, IMPL_OFFSET>,
            SetCenterPoint: SetCenterPoint::<Impl, IMPL_OFFSET>,
            ColorStops: ColorStops::<Impl, IMPL_OFFSET>,
            ExtendMode: ExtendMode::<Impl, IMPL_OFFSET>,
            SetExtendMode: SetExtendMode::<Impl, IMPL_OFFSET>,
            InterpolationSpace: InterpolationSpace::<Impl, IMPL_OFFSET>,
            SetInterpolationSpace: SetInterpolationSpace::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            RotationAngle: RotationAngle::<Impl, IMPL_OFFSET>,
            SetRotationAngle: SetRotationAngle::<Impl, IMPL_OFFSET>,
            RotationAngleInDegrees: RotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetRotationAngleInDegrees: SetRotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
            TransformMatrix: TransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGradientBrush2Impl: Sized {
    fn MappingMode(&self) -> ::windows::core::Result<CompositionMappingMode>;
    fn SetMappingMode(&self, value: CompositionMappingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGradientBrush2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGradientBrush2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGradientBrush2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGradientBrush2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGradientBrush2Vtbl {
        unsafe extern "system" fn MappingMode<Impl: ICompositionGradientBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionMappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MappingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMappingMode<Impl: ICompositionGradientBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionMappingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMappingMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGradientBrush2, BASE_OFFSET>(),
            MappingMode: MappingMode::<Impl, IMPL_OFFSET>,
            SetMappingMode: SetMappingMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGradientBrush2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGradientBrushFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGradientBrushFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGradientBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGradientBrushFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGradientBrushFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGradientBrushFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGradientBrushFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGradientBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ICompositionGraphicsDeviceImpl: Sized {
    fn CreateDrawingSurface(&self, sizepixels: &super::super::Foundation::Size, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionDrawingSurface>;
    fn RenderingDeviceReplaced(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CompositionGraphicsDevice, RenderingDeviceReplacedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRenderingDeviceReplaced(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDeviceVtbl {
        unsafe extern "system" fn CreateDrawingSurface<Impl: ICompositionGraphicsDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Foundation::Size, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDrawingSurface(&*(&sizepixels as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType), pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderingDeviceReplaced<Impl: ICompositionGraphicsDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderingDeviceReplaced(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CompositionGraphicsDevice, RenderingDeviceReplacedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CompositionGraphicsDevice, RenderingDeviceReplacedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRenderingDeviceReplaced<Impl: ICompositionGraphicsDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRenderingDeviceReplaced(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGraphicsDevice, BASE_OFFSET>(),
            CreateDrawingSurface: CreateDrawingSurface::<Impl, IMPL_OFFSET>,
            RenderingDeviceReplaced: RenderingDeviceReplaced::<Impl, IMPL_OFFSET>,
            RemoveRenderingDeviceReplaced: RemoveRenderingDeviceReplaced::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGraphicsDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ICompositionGraphicsDevice2Impl: Sized {
    fn CreateDrawingSurface2(&self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionDrawingSurface>;
    fn CreateVirtualDrawingSurface(&self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionVirtualDrawingSurface>;
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice2";
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDevice2Vtbl {
        unsafe extern "system" fn CreateDrawingSurface2<Impl: ICompositionGraphicsDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDrawingSurface2(&*(&sizepixels as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType), pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualDrawingSurface<Impl: ICompositionGraphicsDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualDrawingSurface(&*(&sizepixels as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType), pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGraphicsDevice2, BASE_OFFSET>(),
            CreateDrawingSurface2: CreateDrawingSurface2::<Impl, IMPL_OFFSET>,
            CreateVirtualDrawingSurface: CreateVirtualDrawingSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGraphicsDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ICompositionGraphicsDevice3Impl: Sized {
    fn CreateMipmapSurface(&self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionMipmapSurface>;
    fn Trim(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice3";
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDevice3Vtbl {
        unsafe extern "system" fn CreateMipmapSurface<Impl: ICompositionGraphicsDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMipmapSurface(&*(&sizepixels as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType), pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trim<Impl: ICompositionGraphicsDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Trim().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGraphicsDevice3, BASE_OFFSET>(),
            CreateMipmapSurface: CreateMipmapSurface::<Impl, IMPL_OFFSET>,
            Trim: Trim::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGraphicsDevice3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ICompositionGraphicsDevice4Impl: Sized {
    fn CaptureAsync(&self, capturevisual: &::core::option::Option<Visual>, size: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, sdrboost: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ICompositionSurface>>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice4";
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDevice4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDevice4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDevice4Vtbl {
        unsafe extern "system" fn CaptureAsync<Impl: ICompositionGraphicsDevice4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capturevisual: ::windows::core::RawPtr, size: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, sdrboost: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureAsync(&*(&capturevisual as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType), &*(&size as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType), pixelformat, alphamode, sdrboost) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGraphicsDevice4, BASE_OFFSET>(),
            CaptureAsync: CaptureAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGraphicsDevice4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLightImpl: Sized {
    fn Targets(&self) -> ::windows::core::Result<VisualUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLight {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLight";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLightVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLightImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLightVtbl {
        unsafe extern "system" fn Targets<Impl: ICompositionLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Targets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionLight, BASE_OFFSET>(), Targets: Targets::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionLight as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLight2Impl: Sized {
    fn ExclusionsFromTargets(&self) -> ::windows::core::Result<VisualUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLight2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLight2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLight2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLight2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLight2Vtbl {
        unsafe extern "system" fn ExclusionsFromTargets<Impl: ICompositionLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExclusionsFromTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionLight2, BASE_OFFSET>(),
            ExclusionsFromTargets: ExclusionsFromTargets::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionLight2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLight3Impl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLight3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLight3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLight3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLight3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLight3Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: ICompositionLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: ICompositionLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionLight3, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionLight3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLightFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLightFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLightFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLightFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLightFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLightFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionLightFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionLightFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionLineGeometryImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetStart(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn End(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEnd(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionLineGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLineGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionLineGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLineGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLineGeometryVtbl {
        unsafe extern "system" fn Start<Impl: ICompositionLineGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStart<Impl: ICompositionLineGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStart(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn End<Impl: ICompositionLineGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnd<Impl: ICompositionLineGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnd(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionLineGeometry, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            SetStart: SetStart::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            SetEnd: SetEnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionLineGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionLinearGradientBrushImpl: Sized {
    fn EndPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEndPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn StartPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetStartPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionLinearGradientBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLinearGradientBrush";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionLinearGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLinearGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLinearGradientBrushVtbl {
        unsafe extern "system" fn EndPoint<Impl: ICompositionLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: ICompositionLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPoint<Impl: ICompositionLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: ICompositionLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionLinearGradientBrush, BASE_OFFSET>(),
            EndPoint: EndPoint::<Impl, IMPL_OFFSET>,
            SetEndPoint: SetEndPoint::<Impl, IMPL_OFFSET>,
            StartPoint: StartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint: SetStartPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionLinearGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionMaskBrushImpl: Sized {
    fn Mask(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetMask(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSource(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionMaskBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionMaskBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionMaskBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionMaskBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionMaskBrushVtbl {
        unsafe extern "system" fn Mask<Impl: ICompositionMaskBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMask<Impl: ICompositionMaskBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMask(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: ICompositionMaskBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: ICompositionMaskBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionMaskBrush, BASE_OFFSET>(),
            Mask: Mask::<Impl, IMPL_OFFSET>,
            SetMask: SetMask::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionMaskBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ICompositionMipmapSurfaceImpl: Sized {
    fn LevelCount(&self) -> ::windows::core::Result<u32>;
    fn AlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXAlphaMode>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SizeInt32(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn GetDrawingSurfaceForLevel(&self, level: u32) -> ::windows::core::Result<CompositionDrawingSurface>;
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionMipmapSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionMipmapSurface";
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionMipmapSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionMipmapSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionMipmapSurfaceVtbl {
        unsafe extern "system" fn LevelCount<Impl: ICompositionMipmapSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlphaMode<Impl: ICompositionMipmapSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelFormat<Impl: ICompositionMipmapSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInt32<Impl: ICompositionMipmapSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrawingSurfaceForLevel<Impl: ICompositionMipmapSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDrawingSurfaceForLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionMipmapSurface, BASE_OFFSET>(),
            LevelCount: LevelCount::<Impl, IMPL_OFFSET>,
            AlphaMode: AlphaMode::<Impl, IMPL_OFFSET>,
            PixelFormat: PixelFormat::<Impl, IMPL_OFFSET>,
            SizeInt32: SizeInt32::<Impl, IMPL_OFFSET>,
            GetDrawingSurfaceForLevel: GetDrawingSurfaceForLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionMipmapSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionNineGridBrushImpl: Sized {
    fn BottomInset(&self) -> ::windows::core::Result<f32>;
    fn SetBottomInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn BottomInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetBottomInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn IsCenterHollow(&self) -> ::windows::core::Result<bool>;
    fn SetIsCenterHollow(&self, value: bool) -> ::windows::core::Result<()>;
    fn LeftInset(&self) -> ::windows::core::Result<f32>;
    fn SetLeftInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn LeftInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetLeftInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightInset(&self) -> ::windows::core::Result<f32>;
    fn SetRightInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetRightInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSource(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn TopInset(&self) -> ::windows::core::Result<f32>;
    fn SetTopInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetTopInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn SetInsets(&self, inset: f32) -> ::windows::core::Result<()>;
    fn SetInsetsWithValues(&self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<()>;
    fn SetInsetScales(&self, scale: f32) -> ::windows::core::Result<()>;
    fn SetInsetScalesWithValues(&self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionNineGridBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionNineGridBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionNineGridBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionNineGridBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionNineGridBrushVtbl {
        unsafe extern "system" fn BottomInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BottomInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomInset(value).into()
        }
        unsafe extern "system" fn BottomInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BottomInsetScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomInsetScale(value).into()
        }
        unsafe extern "system" fn IsCenterHollow<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCenterHollow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCenterHollow<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCenterHollow(value).into()
        }
        unsafe extern "system" fn LeftInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftInset(value).into()
        }
        unsafe extern "system" fn LeftInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftInsetScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftInsetScale(value).into()
        }
        unsafe extern "system" fn RightInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightInset(value).into()
        }
        unsafe extern "system" fn RightInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightInsetScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightInsetScale(value).into()
        }
        unsafe extern "system" fn Source<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TopInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopInset<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopInset(value).into()
        }
        unsafe extern "system" fn TopInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopInsetScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopInsetScale<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopInsetScale(value).into()
        }
        unsafe extern "system" fn SetInsets<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInsets(inset).into()
        }
        unsafe extern "system" fn SetInsetsWithValues<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInsetsWithValues(left, top, right, bottom).into()
        }
        unsafe extern "system" fn SetInsetScales<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInsetScales(scale).into()
        }
        unsafe extern "system" fn SetInsetScalesWithValues<Impl: ICompositionNineGridBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInsetScalesWithValues(left, top, right, bottom).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionNineGridBrush, BASE_OFFSET>(),
            BottomInset: BottomInset::<Impl, IMPL_OFFSET>,
            SetBottomInset: SetBottomInset::<Impl, IMPL_OFFSET>,
            BottomInsetScale: BottomInsetScale::<Impl, IMPL_OFFSET>,
            SetBottomInsetScale: SetBottomInsetScale::<Impl, IMPL_OFFSET>,
            IsCenterHollow: IsCenterHollow::<Impl, IMPL_OFFSET>,
            SetIsCenterHollow: SetIsCenterHollow::<Impl, IMPL_OFFSET>,
            LeftInset: LeftInset::<Impl, IMPL_OFFSET>,
            SetLeftInset: SetLeftInset::<Impl, IMPL_OFFSET>,
            LeftInsetScale: LeftInsetScale::<Impl, IMPL_OFFSET>,
            SetLeftInsetScale: SetLeftInsetScale::<Impl, IMPL_OFFSET>,
            RightInset: RightInset::<Impl, IMPL_OFFSET>,
            SetRightInset: SetRightInset::<Impl, IMPL_OFFSET>,
            RightInsetScale: RightInsetScale::<Impl, IMPL_OFFSET>,
            SetRightInsetScale: SetRightInsetScale::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            TopInset: TopInset::<Impl, IMPL_OFFSET>,
            SetTopInset: SetTopInset::<Impl, IMPL_OFFSET>,
            TopInsetScale: TopInsetScale::<Impl, IMPL_OFFSET>,
            SetTopInsetScale: SetTopInsetScale::<Impl, IMPL_OFFSET>,
            SetInsets: SetInsets::<Impl, IMPL_OFFSET>,
            SetInsetsWithValues: SetInsetsWithValues::<Impl, IMPL_OFFSET>,
            SetInsetScales: SetInsetScales::<Impl, IMPL_OFFSET>,
            SetInsetScalesWithValues: SetInsetScalesWithValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionNineGridBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICompositionObjectImpl: Sized {
    fn Compositor(&self) -> ::windows::core::Result<Compositor>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
    fn Properties(&self) -> ::windows::core::Result<CompositionPropertySet>;
    fn StartAnimation(&self, propertyname: &::windows::core::HSTRING, animation: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionObject {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ICompositionObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObjectVtbl {
        unsafe extern "system" fn Compositor<Impl: ICompositionObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compositor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: ICompositionObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ICompositionObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAnimation<Impl: ICompositionObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAnimation(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animation as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAnimation<Impl: ICompositionObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAnimation(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionObject, BASE_OFFSET>(),
            Compositor: Compositor::<Impl, IMPL_OFFSET>,
            Dispatcher: Dispatcher::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            StartAnimation: StartAnimation::<Impl, IMPL_OFFSET>,
            StopAnimation: StopAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObject2Impl: Sized {
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ImplicitAnimations(&self) -> ::windows::core::Result<ImplicitAnimationCollection>;
    fn SetImplicitAnimations(&self, value: &::core::option::Option<ImplicitAnimationCollection>) -> ::windows::core::Result<()>;
    fn StartAnimationGroup(&self, value: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn StopAnimationGroup(&self, value: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObject2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObject2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObject2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObject2Vtbl {
        unsafe extern "system" fn Comment<Impl: ICompositionObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComment<Impl: ICompositionObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComment(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImplicitAnimations<Impl: ICompositionObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImplicitAnimations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImplicitAnimations<Impl: ICompositionObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImplicitAnimations(&*(&value as *const <ImplicitAnimationCollection as ::windows::core::Abi>::Abi as *const <ImplicitAnimationCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAnimationGroup<Impl: ICompositionObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAnimationGroup(&*(&value as *const <ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAnimationGroup<Impl: ICompositionObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAnimationGroup(&*(&value as *const <ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionObject2, BASE_OFFSET>(),
            Comment: Comment::<Impl, IMPL_OFFSET>,
            SetComment: SetComment::<Impl, IMPL_OFFSET>,
            ImplicitAnimations: ImplicitAnimations::<Impl, IMPL_OFFSET>,
            SetImplicitAnimations: SetImplicitAnimations::<Impl, IMPL_OFFSET>,
            StartAnimationGroup: StartAnimationGroup::<Impl, IMPL_OFFSET>,
            StopAnimationGroup: StopAnimationGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionObject2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait ICompositionObject3Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionObject3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ICompositionObject3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObject3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObject3Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: ICompositionObject3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionObject3, BASE_OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionObject3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObject4Impl: Sized {
    fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<AnimationController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObject4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObject4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObject4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObject4Vtbl {
        unsafe extern "system" fn TryGetAnimationController<Impl: ICompositionObject4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetAnimationController(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionObject4, BASE_OFFSET>(),
            TryGetAnimationController: TryGetAnimationController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionObject4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObjectFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObjectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObjectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObjectFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionObjectFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionObjectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObjectStaticsImpl: Sized {
    fn StartAnimationWithIAnimationObject(&self, target: &::core::option::Option<IAnimationObject>, propertyname: &::windows::core::HSTRING, animation: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn StartAnimationGroupWithIAnimationObject(&self, target: &::core::option::Option<IAnimationObject>, animation: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObjectStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObjectStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObjectStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObjectStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObjectStaticsVtbl {
        unsafe extern "system" fn StartAnimationWithIAnimationObject<Impl: ICompositionObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartAnimationWithIAnimationObject(&*(&target as *const <IAnimationObject as ::windows::core::Abi>::Abi as *const <IAnimationObject as ::windows::core::DefaultType>::DefaultType), &*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animation as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn StartAnimationGroupWithIAnimationObject<Impl: ICompositionObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAnimationGroupWithIAnimationObject(&*(&target as *const <IAnimationObject as ::windows::core::Abi>::Abi as *const <IAnimationObject as ::windows::core::DefaultType>::DefaultType), &*(&animation as *const <ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionObjectStatics, BASE_OFFSET>(),
            StartAnimationWithIAnimationObject: StartAnimationWithIAnimationObject::<Impl, IMPL_OFFSET>,
            StartAnimationGroupWithIAnimationObject: StartAnimationGroupWithIAnimationObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionObjectStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPathImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionPath {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPath";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPathVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionPath, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait ICompositionPathFactoryImpl: Sized {
    fn Create(&self, source: &::core::option::Option<super::super::Graphics::IGeometrySource2D>) -> ::windows::core::Result<CompositionPath>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionPathFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPathFactory";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ICompositionPathFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPathFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPathFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICompositionPathFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&source as *const <super::super::Graphics::IGeometrySource2D as ::windows::core::Abi>::Abi as *const <super::super::Graphics::IGeometrySource2D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionPathFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionPathFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPathGeometryImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<CompositionPath>;
    fn SetPath(&self, value: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionPathGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPathGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionPathGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPathGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPathGeometryVtbl {
        unsafe extern "system" fn Path<Impl: ICompositionPathGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: ICompositionPathGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <CompositionPath as ::windows::core::Abi>::Abi as *const <CompositionPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionPathGeometry, BASE_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionPathGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowImpl: Sized {
    fn BlurRadiusMultiplier(&self) -> ::windows::core::Result<f32>;
    fn SetBlurRadiusMultiplier(&self, value: f32) -> ::windows::core::Result<()>;
    fn Casters(&self) -> ::windows::core::Result<CompositionProjectedShadowCasterCollection>;
    fn LightSource(&self) -> ::windows::core::Result<CompositionLight>;
    fn SetLightSource(&self, value: &::core::option::Option<CompositionLight>) -> ::windows::core::Result<()>;
    fn MaxBlurRadius(&self) -> ::windows::core::Result<f32>;
    fn SetMaxBlurRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn MinBlurRadius(&self) -> ::windows::core::Result<f32>;
    fn SetMinBlurRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn Receivers(&self) -> ::windows::core::Result<CompositionProjectedShadowReceiverUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadow {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadow";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowVtbl {
        unsafe extern "system" fn BlurRadiusMultiplier<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlurRadiusMultiplier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlurRadiusMultiplier<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlurRadiusMultiplier(value).into()
        }
        unsafe extern "system" fn Casters<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Casters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSource<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLightSource<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLightSource(&*(&value as *const <CompositionLight as ::windows::core::Abi>::Abi as *const <CompositionLight as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxBlurRadius<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBlurRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBlurRadius<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBlurRadius(value).into()
        }
        unsafe extern "system" fn MinBlurRadius<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinBlurRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinBlurRadius<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinBlurRadius(value).into()
        }
        unsafe extern "system" fn Receivers<Impl: ICompositionProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receivers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionProjectedShadow, BASE_OFFSET>(),
            BlurRadiusMultiplier: BlurRadiusMultiplier::<Impl, IMPL_OFFSET>,
            SetBlurRadiusMultiplier: SetBlurRadiusMultiplier::<Impl, IMPL_OFFSET>,
            Casters: Casters::<Impl, IMPL_OFFSET>,
            LightSource: LightSource::<Impl, IMPL_OFFSET>,
            SetLightSource: SetLightSource::<Impl, IMPL_OFFSET>,
            MaxBlurRadius: MaxBlurRadius::<Impl, IMPL_OFFSET>,
            SetMaxBlurRadius: SetMaxBlurRadius::<Impl, IMPL_OFFSET>,
            MinBlurRadius: MinBlurRadius::<Impl, IMPL_OFFSET>,
            SetMinBlurRadius: SetMinBlurRadius::<Impl, IMPL_OFFSET>,
            Receivers: Receivers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionProjectedShadow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowCasterImpl: Sized {
    fn Brush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn CastingVisual(&self) -> ::windows::core::Result<Visual>;
    fn SetCastingVisual(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowCaster {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowCaster";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowCasterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowCasterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowCasterVtbl {
        unsafe extern "system" fn Brush<Impl: ICompositionProjectedShadowCasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Brush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrush<Impl: ICompositionProjectedShadowCasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrush(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CastingVisual<Impl: ICompositionProjectedShadowCasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CastingVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCastingVisual<Impl: ICompositionProjectedShadowCasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCastingVisual(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionProjectedShadowCaster, BASE_OFFSET>(),
            Brush: Brush::<Impl, IMPL_OFFSET>,
            SetBrush: SetBrush::<Impl, IMPL_OFFSET>,
            CastingVisual: CastingVisual::<Impl, IMPL_OFFSET>,
            SetCastingVisual: SetCastingVisual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionProjectedShadowCaster as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowCasterCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn InsertAbove(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>, reference: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertAtBottom(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertAtTop(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertBelow(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>, reference: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn Remove(&self, caster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowCasterCollection {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowCasterCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowCasterCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowCasterCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowCasterCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ICompositionProjectedShadowCasterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAbove<Impl: ICompositionProjectedShadowCasterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAbove(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType), &*(&reference as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtBottom<Impl: ICompositionProjectedShadowCasterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtBottom(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtTop<Impl: ICompositionProjectedShadowCasterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtTop(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertBelow<Impl: ICompositionProjectedShadowCasterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertBelow(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType), &*(&reference as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICompositionProjectedShadowCasterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, caster: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&caster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionProjectedShadowCasterCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionProjectedShadowCasterCollection, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            InsertAbove: InsertAbove::<Impl, IMPL_OFFSET>,
            InsertAtBottom: InsertAtBottom::<Impl, IMPL_OFFSET>,
            InsertAtTop: InsertAtTop::<Impl, IMPL_OFFSET>,
            InsertBelow: InsertBelow::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionProjectedShadowCasterCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowCasterCollectionStaticsImpl: Sized {
    fn MaxRespectedCasters(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowCasterCollectionStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowCasterCollectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowCasterCollectionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowCasterCollectionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowCasterCollectionStaticsVtbl {
        unsafe extern "system" fn MaxRespectedCasters<Impl: ICompositionProjectedShadowCasterCollectionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxRespectedCasters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionProjectedShadowCasterCollectionStatics, BASE_OFFSET>(),
            MaxRespectedCasters: MaxRespectedCasters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionProjectedShadowCasterCollectionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowReceiverImpl: Sized {
    fn ReceivingVisual(&self) -> ::windows::core::Result<Visual>;
    fn SetReceivingVisual(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowReceiver {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowReceiver";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowReceiverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowReceiverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowReceiverVtbl {
        unsafe extern "system" fn ReceivingVisual<Impl: ICompositionProjectedShadowReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivingVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceivingVisual<Impl: ICompositionProjectedShadowReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceivingVisual(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionProjectedShadowReceiver, BASE_OFFSET>(),
            ReceivingVisual: ReceivingVisual::<Impl, IMPL_OFFSET>,
            SetReceivingVisual: SetReceivingVisual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionProjectedShadowReceiver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowReceiverUnorderedCollectionImpl: Sized {
    fn Add(&self, value: &::core::option::Option<CompositionProjectedShadowReceiver>) -> ::windows::core::Result<()>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Remove(&self, value: &::core::option::Option<CompositionProjectedShadowReceiver>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowReceiverUnorderedCollection {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowReceiverUnorderedCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowReceiverUnorderedCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowReceiverUnorderedCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowReceiverUnorderedCollectionVtbl {
        unsafe extern "system" fn Add<Impl: ICompositionProjectedShadowReceiverUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&value as *const <CompositionProjectedShadowReceiver as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowReceiver as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Count<Impl: ICompositionProjectedShadowReceiverUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ICompositionProjectedShadowReceiverUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&value as *const <CompositionProjectedShadowReceiver as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowReceiver as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionProjectedShadowReceiverUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionProjectedShadowReceiverUnorderedCollection, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionProjectedShadowReceiverUnorderedCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionPropertySetImpl: Sized {
    fn InsertColor(&self, propertyname: &::windows::core::HSTRING, value: &super::Color) -> ::windows::core::Result<()>;
    fn InsertMatrix3x2(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn InsertMatrix4x4(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn InsertQuaternion(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn InsertScalar(&self, propertyname: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn InsertVector2(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn InsertVector3(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InsertVector4(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn TryGetColor(&self, propertyname: &::windows::core::HSTRING, value: &mut super::Color) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetMatrix3x2(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetMatrix4x4(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetQuaternion(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetScalar(&self, propertyname: &::windows::core::HSTRING, value: &mut f32) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector2(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector3(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector4(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<CompositionGetValueStatus>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionPropertySet {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPropertySet";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPropertySetVtbl {
        unsafe extern "system" fn InsertColor<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertColor(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertMatrix3x2<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertMatrix3x2(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertMatrix4x4<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertMatrix4x4(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertQuaternion<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertQuaternion(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertScalar<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertScalar(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn InsertVector2<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertVector2(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertVector3<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertVector3(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertVector4<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertVector4(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetColor<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::Color, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetColor(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetMatrix3x2<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Matrix3x2, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetMatrix3x2(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetMatrix4x4<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetMatrix4x4(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetQuaternion<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Quaternion, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetQuaternion(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetScalar<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut f32, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetScalar(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetVector2<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Vector2, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetVector2(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetVector3<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Vector3, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetVector3(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetVector4<Impl: ICompositionPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Vector4, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetVector4(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionPropertySet, BASE_OFFSET>(),
            InsertColor: InsertColor::<Impl, IMPL_OFFSET>,
            InsertMatrix3x2: InsertMatrix3x2::<Impl, IMPL_OFFSET>,
            InsertMatrix4x4: InsertMatrix4x4::<Impl, IMPL_OFFSET>,
            InsertQuaternion: InsertQuaternion::<Impl, IMPL_OFFSET>,
            InsertScalar: InsertScalar::<Impl, IMPL_OFFSET>,
            InsertVector2: InsertVector2::<Impl, IMPL_OFFSET>,
            InsertVector3: InsertVector3::<Impl, IMPL_OFFSET>,
            InsertVector4: InsertVector4::<Impl, IMPL_OFFSET>,
            TryGetColor: TryGetColor::<Impl, IMPL_OFFSET>,
            TryGetMatrix3x2: TryGetMatrix3x2::<Impl, IMPL_OFFSET>,
            TryGetMatrix4x4: TryGetMatrix4x4::<Impl, IMPL_OFFSET>,
            TryGetQuaternion: TryGetQuaternion::<Impl, IMPL_OFFSET>,
            TryGetScalar: TryGetScalar::<Impl, IMPL_OFFSET>,
            TryGetVector2: TryGetVector2::<Impl, IMPL_OFFSET>,
            TryGetVector3: TryGetVector3::<Impl, IMPL_OFFSET>,
            TryGetVector4: TryGetVector4::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPropertySet2Impl: Sized {
    fn InsertBoolean(&self, propertyname: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn TryGetBoolean(&self, propertyname: &::windows::core::HSTRING, value: &mut bool) -> ::windows::core::Result<CompositionGetValueStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionPropertySet2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPropertySet2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionPropertySet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPropertySet2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPropertySet2Vtbl {
        unsafe extern "system" fn InsertBoolean<Impl: ICompositionPropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertBoolean(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn TryGetBoolean<Impl: ICompositionPropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut bool, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetBoolean(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionPropertySet2, BASE_OFFSET>(),
            InsertBoolean: InsertBoolean::<Impl, IMPL_OFFSET>,
            TryGetBoolean: TryGetBoolean::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionPropertySet2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionRadialGradientBrushImpl: Sized {
    fn EllipseCenter(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEllipseCenter(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn EllipseRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEllipseRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn GradientOriginOffset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetGradientOriginOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionRadialGradientBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionRadialGradientBrush";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionRadialGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionRadialGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionRadialGradientBrushVtbl {
        unsafe extern "system" fn EllipseCenter<Impl: ICompositionRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEllipseCenter<Impl: ICompositionRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEllipseCenter(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EllipseRadius<Impl: ICompositionRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEllipseRadius<Impl: ICompositionRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEllipseRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GradientOriginOffset<Impl: ICompositionRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GradientOriginOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientOriginOffset<Impl: ICompositionRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGradientOriginOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionRadialGradientBrush, BASE_OFFSET>(),
            EllipseCenter: EllipseCenter::<Impl, IMPL_OFFSET>,
            SetEllipseCenter: SetEllipseCenter::<Impl, IMPL_OFFSET>,
            EllipseRadius: EllipseRadius::<Impl, IMPL_OFFSET>,
            SetEllipseRadius: SetEllipseRadius::<Impl, IMPL_OFFSET>,
            GradientOriginOffset: GradientOriginOffset::<Impl, IMPL_OFFSET>,
            SetGradientOriginOffset: SetGradientOriginOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionRadialGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionRectangleGeometryImpl: Sized {
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionRectangleGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionRectangleGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionRectangleGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionRectangleGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionRectangleGeometryVtbl {
        unsafe extern "system" fn Offset<Impl: ICompositionRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: ICompositionRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: ICompositionRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionRectangleGeometry, BASE_OFFSET>(),
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionRectangleGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionRoundedRectangleGeometryImpl: Sized {
    fn CornerRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCornerRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionRoundedRectangleGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionRoundedRectangleGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionRoundedRectangleGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionRoundedRectangleGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionRoundedRectangleGeometryVtbl {
        unsafe extern "system" fn CornerRadius<Impl: ICompositionRoundedRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CornerRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCornerRadius<Impl: ICompositionRoundedRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCornerRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionRoundedRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionRoundedRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: ICompositionRoundedRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: ICompositionRoundedRectangleGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionRoundedRectangleGeometry, BASE_OFFSET>(),
            CornerRadius: CornerRadius::<Impl, IMPL_OFFSET>,
            SetCornerRadius: SetCornerRadius::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionRoundedRectangleGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositionScopedBatchImpl: Sized {
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn IsEnded(&self) -> ::windows::core::Result<bool>;
    fn End(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Suspend(&self) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionScopedBatch {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionScopedBatch";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionScopedBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionScopedBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionScopedBatchVtbl {
        unsafe extern "system" fn IsActive<Impl: ICompositionScopedBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnded<Impl: ICompositionScopedBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: ICompositionScopedBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End().into()
        }
        unsafe extern "system" fn Resume<Impl: ICompositionScopedBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Suspend<Impl: ICompositionScopedBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Suspend().into()
        }
        unsafe extern "system" fn Completed<Impl: ICompositionScopedBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: ICompositionScopedBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionScopedBatch, BASE_OFFSET>(),
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            IsEnded: IsEnded::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Suspend: Suspend::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionScopedBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShadowImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionShadow {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShadow";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionShadowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShadowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShadowVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionShadow, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionShadow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShadowFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionShadowFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShadowFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionShadowFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShadowFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShadowFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionShadowFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionShadowFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionShapeImpl: Sized {
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionShape {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShape";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionShapeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShapeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShapeVtbl {
        unsafe extern "system" fn CenterPoint<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionShape, BASE_OFFSET>(),
            CenterPoint: CenterPoint::<Impl, IMPL_OFFSET>,
            SetCenterPoint: SetCenterPoint::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            RotationAngle: RotationAngle::<Impl, IMPL_OFFSET>,
            SetRotationAngle: SetRotationAngle::<Impl, IMPL_OFFSET>,
            RotationAngleInDegrees: RotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetRotationAngleInDegrees: SetRotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
            TransformMatrix: TransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionShape as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShapeFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionShapeFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShapeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionShapeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShapeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShapeFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionShapeFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionShapeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICompositionSpriteShapeImpl: Sized {
    fn FillBrush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetFillBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Geometry(&self) -> ::windows::core::Result<CompositionGeometry>;
    fn SetGeometry(&self, value: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<()>;
    fn IsStrokeNonScaling(&self) -> ::windows::core::Result<bool>;
    fn SetIsStrokeNonScaling(&self, value: bool) -> ::windows::core::Result<()>;
    fn StrokeBrush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetStrokeBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn StrokeDashArray(&self) -> ::windows::core::Result<CompositionStrokeDashArray>;
    fn StrokeDashCap(&self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeDashCap(&self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeDashOffset(&self) -> ::windows::core::Result<f32>;
    fn SetStrokeDashOffset(&self, value: f32) -> ::windows::core::Result<()>;
    fn StrokeEndCap(&self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeEndCap(&self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeLineJoin(&self) -> ::windows::core::Result<CompositionStrokeLineJoin>;
    fn SetStrokeLineJoin(&self, value: CompositionStrokeLineJoin) -> ::windows::core::Result<()>;
    fn StrokeMiterLimit(&self) -> ::windows::core::Result<f32>;
    fn SetStrokeMiterLimit(&self, value: f32) -> ::windows::core::Result<()>;
    fn StrokeStartCap(&self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeStartCap(&self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f32>;
    fn SetStrokeThickness(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionSpriteShape {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSpriteShape";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompositionSpriteShapeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSpriteShapeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSpriteShapeVtbl {
        unsafe extern "system" fn FillBrush<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrush<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrush(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Geometry<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Geometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometry<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometry(&*(&value as *const <CompositionGeometry as ::windows::core::Abi>::Abi as *const <CompositionGeometry as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsStrokeNonScaling<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStrokeNonScaling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStrokeNonScaling<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStrokeNonScaling(value).into()
        }
        unsafe extern "system" fn StrokeBrush<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrush<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeBrush(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeDashArray<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeDashCap<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashCap(value).into()
        }
        unsafe extern "system" fn StrokeDashOffset<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeDashOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashOffset(value).into()
        }
        unsafe extern "system" fn StrokeEndCap<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeEndCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndCap<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeEndCap(value).into()
        }
        unsafe extern "system" fn StrokeLineJoin<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeLineJoin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeLineJoin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeLineJoin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeLineJoin(value).into()
        }
        unsafe extern "system" fn StrokeMiterLimit<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeMiterLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeMiterLimit(value).into()
        }
        unsafe extern "system" fn StrokeStartCap<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeStartCap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartCap<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeStartCap(value).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: ICompositionSpriteShapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSpriteShape, BASE_OFFSET>(),
            FillBrush: FillBrush::<Impl, IMPL_OFFSET>,
            SetFillBrush: SetFillBrush::<Impl, IMPL_OFFSET>,
            Geometry: Geometry::<Impl, IMPL_OFFSET>,
            SetGeometry: SetGeometry::<Impl, IMPL_OFFSET>,
            IsStrokeNonScaling: IsStrokeNonScaling::<Impl, IMPL_OFFSET>,
            SetIsStrokeNonScaling: SetIsStrokeNonScaling::<Impl, IMPL_OFFSET>,
            StrokeBrush: StrokeBrush::<Impl, IMPL_OFFSET>,
            SetStrokeBrush: SetStrokeBrush::<Impl, IMPL_OFFSET>,
            StrokeDashArray: StrokeDashArray::<Impl, IMPL_OFFSET>,
            StrokeDashCap: StrokeDashCap::<Impl, IMPL_OFFSET>,
            SetStrokeDashCap: SetStrokeDashCap::<Impl, IMPL_OFFSET>,
            StrokeDashOffset: StrokeDashOffset::<Impl, IMPL_OFFSET>,
            SetStrokeDashOffset: SetStrokeDashOffset::<Impl, IMPL_OFFSET>,
            StrokeEndCap: StrokeEndCap::<Impl, IMPL_OFFSET>,
            SetStrokeEndCap: SetStrokeEndCap::<Impl, IMPL_OFFSET>,
            StrokeLineJoin: StrokeLineJoin::<Impl, IMPL_OFFSET>,
            SetStrokeLineJoin: SetStrokeLineJoin::<Impl, IMPL_OFFSET>,
            StrokeMiterLimit: StrokeMiterLimit::<Impl, IMPL_OFFSET>,
            SetStrokeMiterLimit: SetStrokeMiterLimit::<Impl, IMPL_OFFSET>,
            StrokeStartCap: StrokeStartCap::<Impl, IMPL_OFFSET>,
            SetStrokeStartCap: SetStrokeStartCap::<Impl, IMPL_OFFSET>,
            StrokeThickness: StrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSpriteShape as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSupportsSystemBackdropImpl: Sized {
    fn SystemBackdrop(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSystemBackdrop(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICompositionSupportsSystemBackdrop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSupportsSystemBackdrop";
}
impl ICompositionSupportsSystemBackdropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSupportsSystemBackdropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSupportsSystemBackdropVtbl {
        unsafe extern "system" fn SystemBackdrop<Impl: ICompositionSupportsSystemBackdropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemBackdrop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemBackdrop<Impl: ICompositionSupportsSystemBackdropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemBackdrop(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSupportsSystemBackdrop, BASE_OFFSET>(),
            SystemBackdrop: SystemBackdrop::<Impl, IMPL_OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSupportsSystemBackdrop as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSurfaceImpl: Sized {}
impl ::windows::core::RuntimeName for ICompositionSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurface";
}
impl ICompositionSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurface, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionSurfaceBrushImpl: Sized {
    fn BitmapInterpolationMode(&self) -> ::windows::core::Result<CompositionBitmapInterpolationMode>;
    fn SetBitmapInterpolationMode(&self, value: CompositionBitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetHorizontalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<CompositionStretch>;
    fn SetStretch(&self, value: CompositionStretch) -> ::windows::core::Result<()>;
    fn Surface(&self) -> ::windows::core::Result<ICompositionSurface>;
    fn SetSurface(&self, value: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetVerticalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionSurfaceBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionSurfaceBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceBrushVtbl {
        unsafe extern "system" fn BitmapInterpolationMode<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapInterpolationMode(value).into()
        }
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalAlignmentRatio<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn Stretch<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStretch<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStretch(value).into()
        }
        unsafe extern "system" fn Surface<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Surface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSurface<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurface(&*(&value as *const <ICompositionSurface as ::windows::core::Abi>::Abi as *const <ICompositionSurface as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalAlignmentRatio<Impl: ICompositionSurfaceBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalAlignmentRatio(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurfaceBrush, BASE_OFFSET>(),
            BitmapInterpolationMode: BitmapInterpolationMode::<Impl, IMPL_OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Impl, IMPL_OFFSET>,
            HorizontalAlignmentRatio: HorizontalAlignmentRatio::<Impl, IMPL_OFFSET>,
            SetHorizontalAlignmentRatio: SetHorizontalAlignmentRatio::<Impl, IMPL_OFFSET>,
            Stretch: Stretch::<Impl, IMPL_OFFSET>,
            SetStretch: SetStretch::<Impl, IMPL_OFFSET>,
            Surface: Surface::<Impl, IMPL_OFFSET>,
            SetSurface: SetSurface::<Impl, IMPL_OFFSET>,
            VerticalAlignmentRatio: VerticalAlignmentRatio::<Impl, IMPL_OFFSET>,
            SetVerticalAlignmentRatio: SetVerticalAlignmentRatio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurfaceBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionSurfaceBrush2Impl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionSurfaceBrush2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceBrush2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionSurfaceBrush2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceBrush2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceBrush2Vtbl {
        unsafe extern "system" fn AnchorPoint<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnchorPoint<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionSurfaceBrush2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurfaceBrush2, BASE_OFFSET>(),
            AnchorPoint: AnchorPoint::<Impl, IMPL_OFFSET>,
            SetAnchorPoint: SetAnchorPoint::<Impl, IMPL_OFFSET>,
            CenterPoint: CenterPoint::<Impl, IMPL_OFFSET>,
            SetCenterPoint: SetCenterPoint::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            RotationAngle: RotationAngle::<Impl, IMPL_OFFSET>,
            SetRotationAngle: SetRotationAngle::<Impl, IMPL_OFFSET>,
            RotationAngleInDegrees: RotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetRotationAngleInDegrees: SetRotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
            TransformMatrix: TransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurfaceBrush2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionSurfaceBrush3Impl: Sized {
    fn SnapToPixels(&self) -> ::windows::core::Result<bool>;
    fn SetSnapToPixels(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionSurfaceBrush3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceBrush3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionSurfaceBrush3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceBrush3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceBrush3Vtbl {
        unsafe extern "system" fn SnapToPixels<Impl: ICompositionSurfaceBrush3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapToPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapToPixels<Impl: ICompositionSurfaceBrush3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapToPixels(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurfaceBrush3, BASE_OFFSET>(),
            SnapToPixels: SnapToPixels::<Impl, IMPL_OFFSET>,
            SetSnapToPixels: SetSnapToPixels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurfaceBrush3 as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSurfaceFacadeImpl: Sized {
    fn GetRealSurface(&self) -> ::windows::core::Result<ICompositionSurface>;
}
impl ::windows::core::RuntimeName for ICompositionSurfaceFacade {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceFacade";
}
impl ICompositionSurfaceFacadeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceFacadeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceFacadeVtbl {
        unsafe extern "system" fn GetRealSurface<Impl: ICompositionSurfaceFacadeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRealSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurfaceFacade, BASE_OFFSET>(),
            GetRealSurface: GetRealSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurfaceFacade as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetImpl: Sized {
    fn Root(&self) -> ::windows::core::Result<Visual>;
    fn SetRoot(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTarget {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTarget";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTargetVtbl {
        unsafe extern "system" fn Root<Impl: ICompositionTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Root() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoot<Impl: ICompositionTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoot(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTarget, BASE_OFFSET>(),
            Root: Root::<Impl, IMPL_OFFSET>,
            SetRoot: SetRoot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTargetFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTargetFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTargetFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTargetFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTargetFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTargetFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTargetFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTransformImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTransform {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTransformVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTransform, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTransformFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTransformFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTransformFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTransformFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTransformFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTransformFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTransformFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTransformFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionViewBoxImpl: Sized {
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetHorizontalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<CompositionStretch>;
    fn SetStretch(&self, value: CompositionStretch) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetVerticalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionViewBox {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionViewBox";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionViewBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionViewBoxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionViewBoxVtbl {
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalAlignmentRatio<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stretch<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStretch<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStretch(value).into()
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalAlignmentRatio<Impl: ICompositionViewBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalAlignmentRatio(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionViewBox, BASE_OFFSET>(),
            HorizontalAlignmentRatio: HorizontalAlignmentRatio::<Impl, IMPL_OFFSET>,
            SetHorizontalAlignmentRatio: SetHorizontalAlignmentRatio::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            Stretch: Stretch::<Impl, IMPL_OFFSET>,
            SetStretch: SetStretch::<Impl, IMPL_OFFSET>,
            VerticalAlignmentRatio: VerticalAlignmentRatio::<Impl, IMPL_OFFSET>,
            SetVerticalAlignmentRatio: SetVerticalAlignmentRatio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionViewBox as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait ICompositionVirtualDrawingSurfaceImpl: Sized {
    fn Trim(&self, rects: &[<super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionVirtualDrawingSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionVirtualDrawingSurface";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ICompositionVirtualDrawingSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionVirtualDrawingSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionVirtualDrawingSurfaceVtbl {
        unsafe extern "system" fn Trim<Impl: ICompositionVirtualDrawingSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rects_array_size: u32, rects: *const super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Trim(::core::slice::from_raw_parts(::core::mem::transmute_copy(&rects), rects_array_size as _)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionVirtualDrawingSurface, BASE_OFFSET>(), Trim: Trim::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionVirtualDrawingSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionVirtualDrawingSurfaceFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionVirtualDrawingSurfaceFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionVirtualDrawingSurfaceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionVirtualDrawingSurfaceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionVirtualDrawingSurfaceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionVirtualDrawingSurfaceFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionVirtualDrawingSurfaceFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionVirtualDrawingSurfaceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionVisualSurfaceImpl: Sized {
    fn SourceVisual(&self) -> ::windows::core::Result<Visual>;
    fn SetSourceVisual(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn SourceOffset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSourceOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn SourceSize(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSourceSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionVisualSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionVisualSurface";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionVisualSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionVisualSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionVisualSurfaceVtbl {
        unsafe extern "system" fn SourceVisual<Impl: ICompositionVisualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceVisual<Impl: ICompositionVisualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceVisual(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceOffset<Impl: ICompositionVisualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceOffset<Impl: ICompositionVisualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceSize<Impl: ICompositionVisualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceSize<Impl: ICompositionVisualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceSize(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionVisualSurface, BASE_OFFSET>(),
            SourceVisual: SourceVisual::<Impl, IMPL_OFFSET>,
            SetSourceVisual: SetSourceVisual::<Impl, IMPL_OFFSET>,
            SourceOffset: SourceOffset::<Impl, IMPL_OFFSET>,
            SetSourceOffset: SetSourceOffset::<Impl, IMPL_OFFSET>,
            SourceSize: SourceSize::<Impl, IMPL_OFFSET>,
            SetSourceSize: SetSourceSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionVisualSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics_Effects", feature = "implement_exclusive"))]
pub trait ICompositorImpl: Sized {
    fn CreateColorKeyFrameAnimation(&self) -> ::windows::core::Result<ColorKeyFrameAnimation>;
    fn CreateColorBrush(&self) -> ::windows::core::Result<CompositionColorBrush>;
    fn CreateColorBrushWithColor(&self, color: &super::Color) -> ::windows::core::Result<CompositionColorBrush>;
    fn CreateContainerVisual(&self) -> ::windows::core::Result<ContainerVisual>;
    fn CreateCubicBezierEasingFunction(&self, controlpoint1: &super::super::Foundation::Numerics::Vector2, controlpoint2: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CubicBezierEasingFunction>;
    fn CreateEffectFactory(&self, graphicseffect: &::core::option::Option<super::super::Graphics::Effects::IGraphicsEffect>) -> ::windows::core::Result<CompositionEffectFactory>;
    fn CreateEffectFactoryWithProperties(&self, graphicseffect: &::core::option::Option<super::super::Graphics::Effects::IGraphicsEffect>, animatableproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<CompositionEffectFactory>;
    fn CreateExpressionAnimation(&self) -> ::windows::core::Result<ExpressionAnimation>;
    fn CreateExpressionAnimationWithExpression(&self, expression: &::windows::core::HSTRING) -> ::windows::core::Result<ExpressionAnimation>;
    fn CreateInsetClip(&self) -> ::windows::core::Result<InsetClip>;
    fn CreateInsetClipWithInsets(&self, leftinset: f32, topinset: f32, rightinset: f32, bottominset: f32) -> ::windows::core::Result<InsetClip>;
    fn CreateLinearEasingFunction(&self) -> ::windows::core::Result<LinearEasingFunction>;
    fn CreatePropertySet(&self) -> ::windows::core::Result<CompositionPropertySet>;
    fn CreateQuaternionKeyFrameAnimation(&self) -> ::windows::core::Result<QuaternionKeyFrameAnimation>;
    fn CreateScalarKeyFrameAnimation(&self) -> ::windows::core::Result<ScalarKeyFrameAnimation>;
    fn CreateScopedBatch(&self, batchtype: CompositionBatchTypes) -> ::windows::core::Result<CompositionScopedBatch>;
    fn CreateSpriteVisual(&self) -> ::windows::core::Result<SpriteVisual>;
    fn CreateSurfaceBrush(&self) -> ::windows::core::Result<CompositionSurfaceBrush>;
    fn CreateSurfaceBrushWithSurface(&self, surface: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<CompositionSurfaceBrush>;
    fn CreateTargetForCurrentView(&self) -> ::windows::core::Result<CompositionTarget>;
    fn CreateVector2KeyFrameAnimation(&self) -> ::windows::core::Result<Vector2KeyFrameAnimation>;
    fn CreateVector3KeyFrameAnimation(&self) -> ::windows::core::Result<Vector3KeyFrameAnimation>;
    fn CreateVector4KeyFrameAnimation(&self) -> ::windows::core::Result<Vector4KeyFrameAnimation>;
    fn GetCommitBatch(&self, batchtype: CompositionBatchTypes) -> ::windows::core::Result<CompositionCommitBatch>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositor {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics_Effects", feature = "implement_exclusive"))]
impl ICompositorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorVtbl {
        unsafe extern "system" fn CreateColorKeyFrameAnimation<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorKeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorBrush<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorBrushWithColor<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: super::Color, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorBrushWithColor(&*(&color as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContainerVisual<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContainerVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierEasingFunction<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controlpoint1: super::super::Foundation::Numerics::Vector2, controlpoint2: super::super::Foundation::Numerics::Vector2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCubicBezierEasingFunction(&*(&controlpoint1 as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType), &*(&controlpoint2 as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectFactory<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graphicseffect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectFactory(&*(&graphicseffect as *const <super::super::Graphics::Effects::IGraphicsEffect as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Effects::IGraphicsEffect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectFactoryWithProperties<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graphicseffect: ::windows::core::RawPtr, animatableproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectFactoryWithProperties(
                &*(&graphicseffect as *const <super::super::Graphics::Effects::IGraphicsEffect as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Effects::IGraphicsEffect as ::windows::core::DefaultType>::DefaultType),
                &*(&animatableproperties as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateExpressionAnimation<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateExpressionAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateExpressionAnimationWithExpression<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateExpressionAnimationWithExpression(&*(&expression as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInsetClip<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInsetClip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInsetClipWithInsets<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leftinset: f32, topinset: f32, rightinset: f32, bottominset: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInsetClipWithInsets(leftinset, topinset, rightinset, bottominset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearEasingFunction<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearEasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertySet<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertySet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuaternionKeyFrameAnimation<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuaternionKeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScalarKeyFrameAnimation<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScalarKeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScopedBatch<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, batchtype: CompositionBatchTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScopedBatch(batchtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpriteVisual<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpriteVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceBrush<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceBrushWithSurface<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurfaceBrushWithSurface(&*(&surface as *const <ICompositionSurface as ::windows::core::Abi>::Abi as *const <ICompositionSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetForCurrentView<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVector2KeyFrameAnimation<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVector2KeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVector3KeyFrameAnimation<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVector3KeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVector4KeyFrameAnimation<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVector4KeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommitBatch<Impl: ICompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, batchtype: CompositionBatchTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCommitBatch(batchtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositor, BASE_OFFSET>(),
            CreateColorKeyFrameAnimation: CreateColorKeyFrameAnimation::<Impl, IMPL_OFFSET>,
            CreateColorBrush: CreateColorBrush::<Impl, IMPL_OFFSET>,
            CreateColorBrushWithColor: CreateColorBrushWithColor::<Impl, IMPL_OFFSET>,
            CreateContainerVisual: CreateContainerVisual::<Impl, IMPL_OFFSET>,
            CreateCubicBezierEasingFunction: CreateCubicBezierEasingFunction::<Impl, IMPL_OFFSET>,
            CreateEffectFactory: CreateEffectFactory::<Impl, IMPL_OFFSET>,
            CreateEffectFactoryWithProperties: CreateEffectFactoryWithProperties::<Impl, IMPL_OFFSET>,
            CreateExpressionAnimation: CreateExpressionAnimation::<Impl, IMPL_OFFSET>,
            CreateExpressionAnimationWithExpression: CreateExpressionAnimationWithExpression::<Impl, IMPL_OFFSET>,
            CreateInsetClip: CreateInsetClip::<Impl, IMPL_OFFSET>,
            CreateInsetClipWithInsets: CreateInsetClipWithInsets::<Impl, IMPL_OFFSET>,
            CreateLinearEasingFunction: CreateLinearEasingFunction::<Impl, IMPL_OFFSET>,
            CreatePropertySet: CreatePropertySet::<Impl, IMPL_OFFSET>,
            CreateQuaternionKeyFrameAnimation: CreateQuaternionKeyFrameAnimation::<Impl, IMPL_OFFSET>,
            CreateScalarKeyFrameAnimation: CreateScalarKeyFrameAnimation::<Impl, IMPL_OFFSET>,
            CreateScopedBatch: CreateScopedBatch::<Impl, IMPL_OFFSET>,
            CreateSpriteVisual: CreateSpriteVisual::<Impl, IMPL_OFFSET>,
            CreateSurfaceBrush: CreateSurfaceBrush::<Impl, IMPL_OFFSET>,
            CreateSurfaceBrushWithSurface: CreateSurfaceBrushWithSurface::<Impl, IMPL_OFFSET>,
            CreateTargetForCurrentView: CreateTargetForCurrentView::<Impl, IMPL_OFFSET>,
            CreateVector2KeyFrameAnimation: CreateVector2KeyFrameAnimation::<Impl, IMPL_OFFSET>,
            CreateVector3KeyFrameAnimation: CreateVector3KeyFrameAnimation::<Impl, IMPL_OFFSET>,
            CreateVector4KeyFrameAnimation: CreateVector4KeyFrameAnimation::<Impl, IMPL_OFFSET>,
            GetCommitBatch: GetCommitBatch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor2Impl: Sized {
    fn CreateAmbientLight(&self) -> ::windows::core::Result<AmbientLight>;
    fn CreateAnimationGroup(&self) -> ::windows::core::Result<CompositionAnimationGroup>;
    fn CreateBackdropBrush(&self) -> ::windows::core::Result<CompositionBackdropBrush>;
    fn CreateDistantLight(&self) -> ::windows::core::Result<DistantLight>;
    fn CreateDropShadow(&self) -> ::windows::core::Result<DropShadow>;
    fn CreateImplicitAnimationCollection(&self) -> ::windows::core::Result<ImplicitAnimationCollection>;
    fn CreateLayerVisual(&self) -> ::windows::core::Result<LayerVisual>;
    fn CreateMaskBrush(&self) -> ::windows::core::Result<CompositionMaskBrush>;
    fn CreateNineGridBrush(&self) -> ::windows::core::Result<CompositionNineGridBrush>;
    fn CreatePointLight(&self) -> ::windows::core::Result<PointLight>;
    fn CreateSpotLight(&self) -> ::windows::core::Result<SpotLight>;
    fn CreateStepEasingFunction(&self) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateStepEasingFunctionWithStepCount(&self, stepcount: i32) -> ::windows::core::Result<StepEasingFunction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor2Vtbl {
        unsafe extern "system" fn CreateAmbientLight<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAmbientLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationGroup<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimationGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBackdropBrush<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBackdropBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDistantLight<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDistantLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDropShadow<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDropShadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImplicitAnimationCollection<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImplicitAnimationCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLayerVisual<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLayerVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMaskBrush<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMaskBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNineGridBrush<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNineGridBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePointLight<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePointLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpotLight<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpotLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStepEasingFunction<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStepEasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStepEasingFunctionWithStepCount<Impl: ICompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepcount: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStepEasingFunctionWithStepCount(stepcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositor2, BASE_OFFSET>(),
            CreateAmbientLight: CreateAmbientLight::<Impl, IMPL_OFFSET>,
            CreateAnimationGroup: CreateAnimationGroup::<Impl, IMPL_OFFSET>,
            CreateBackdropBrush: CreateBackdropBrush::<Impl, IMPL_OFFSET>,
            CreateDistantLight: CreateDistantLight::<Impl, IMPL_OFFSET>,
            CreateDropShadow: CreateDropShadow::<Impl, IMPL_OFFSET>,
            CreateImplicitAnimationCollection: CreateImplicitAnimationCollection::<Impl, IMPL_OFFSET>,
            CreateLayerVisual: CreateLayerVisual::<Impl, IMPL_OFFSET>,
            CreateMaskBrush: CreateMaskBrush::<Impl, IMPL_OFFSET>,
            CreateNineGridBrush: CreateNineGridBrush::<Impl, IMPL_OFFSET>,
            CreatePointLight: CreatePointLight::<Impl, IMPL_OFFSET>,
            CreateSpotLight: CreateSpotLight::<Impl, IMPL_OFFSET>,
            CreateStepEasingFunction: CreateStepEasingFunction::<Impl, IMPL_OFFSET>,
            CreateStepEasingFunctionWithStepCount: CreateStepEasingFunctionWithStepCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor3Impl: Sized {
    fn CreateHostBackdropBrush(&self) -> ::windows::core::Result<CompositionBackdropBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor3Vtbl {
        unsafe extern "system" fn CreateHostBackdropBrush<Impl: ICompositor3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHostBackdropBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositor3, BASE_OFFSET>(),
            CreateHostBackdropBrush: CreateHostBackdropBrush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositor3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor4Impl: Sized {
    fn CreateColorGradientStop(&self) -> ::windows::core::Result<CompositionColorGradientStop>;
    fn CreateColorGradientStopWithOffsetAndColor(&self, offset: f32, color: &super::Color) -> ::windows::core::Result<CompositionColorGradientStop>;
    fn CreateLinearGradientBrush(&self) -> ::windows::core::Result<CompositionLinearGradientBrush>;
    fn CreateSpringScalarAnimation(&self) -> ::windows::core::Result<SpringScalarNaturalMotionAnimation>;
    fn CreateSpringVector2Animation(&self) -> ::windows::core::Result<SpringVector2NaturalMotionAnimation>;
    fn CreateSpringVector3Animation(&self) -> ::windows::core::Result<SpringVector3NaturalMotionAnimation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor4Vtbl {
        unsafe extern "system" fn CreateColorGradientStop<Impl: ICompositor4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorGradientStop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorGradientStopWithOffsetAndColor<Impl: ICompositor4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f32, color: super::Color, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorGradientStopWithOffsetAndColor(offset, &*(&color as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: ICompositor4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearGradientBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpringScalarAnimation<Impl: ICompositor4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpringScalarAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpringVector2Animation<Impl: ICompositor4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpringVector2Animation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpringVector3Animation<Impl: ICompositor4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpringVector3Animation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositor4, BASE_OFFSET>(),
            CreateColorGradientStop: CreateColorGradientStop::<Impl, IMPL_OFFSET>,
            CreateColorGradientStopWithOffsetAndColor: CreateColorGradientStopWithOffsetAndColor::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateSpringScalarAnimation: CreateSpringScalarAnimation::<Impl, IMPL_OFFSET>,
            CreateSpringVector2Animation: CreateSpringVector2Animation::<Impl, IMPL_OFFSET>,
            CreateSpringVector3Animation: CreateSpringVector3Animation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositor4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositor5Impl: Sized {
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GlobalPlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn SetGlobalPlaybackRate(&self, value: f32) -> ::windows::core::Result<()>;
    fn CreateBounceScalarAnimation(&self) -> ::windows::core::Result<BounceScalarNaturalMotionAnimation>;
    fn CreateBounceVector2Animation(&self) -> ::windows::core::Result<BounceVector2NaturalMotionAnimation>;
    fn CreateBounceVector3Animation(&self) -> ::windows::core::Result<BounceVector3NaturalMotionAnimation>;
    fn CreateContainerShape(&self) -> ::windows::core::Result<CompositionContainerShape>;
    fn CreateEllipseGeometry(&self) -> ::windows::core::Result<CompositionEllipseGeometry>;
    fn CreateLineGeometry(&self) -> ::windows::core::Result<CompositionLineGeometry>;
    fn CreatePathGeometry(&self) -> ::windows::core::Result<CompositionPathGeometry>;
    fn CreatePathGeometryWithPath(&self, path: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<CompositionPathGeometry>;
    fn CreatePathKeyFrameAnimation(&self) -> ::windows::core::Result<PathKeyFrameAnimation>;
    fn CreateRectangleGeometry(&self) -> ::windows::core::Result<CompositionRectangleGeometry>;
    fn CreateRoundedRectangleGeometry(&self) -> ::windows::core::Result<CompositionRoundedRectangleGeometry>;
    fn CreateShapeVisual(&self) -> ::windows::core::Result<ShapeVisual>;
    fn CreateSpriteShape(&self) -> ::windows::core::Result<CompositionSpriteShape>;
    fn CreateSpriteShapeWithGeometry(&self, geometry: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<CompositionSpriteShape>;
    fn CreateViewBox(&self) -> ::windows::core::Result<CompositionViewBox>;
    fn RequestCommitAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositor5 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositor5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor5Vtbl {
        unsafe extern "system" fn Comment<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComment<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComment(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GlobalPlaybackRate<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalPlaybackRate<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalPlaybackRate(value).into()
        }
        unsafe extern "system" fn CreateBounceScalarAnimation<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBounceScalarAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBounceVector2Animation<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBounceVector2Animation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBounceVector3Animation<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBounceVector3Animation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContainerShape<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContainerShape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEllipseGeometry<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEllipseGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLineGeometry<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLineGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePathGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathGeometryWithPath<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePathGeometryWithPath(&*(&path as *const <CompositionPath as ::windows::core::Abi>::Abi as *const <CompositionPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathKeyFrameAnimation<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePathKeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleGeometry<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoundedRectangleGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShapeVisual<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShapeVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpriteShape<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpriteShape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpriteShapeWithGeometry<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpriteShapeWithGeometry(&*(&geometry as *const <CompositionGeometry as ::windows::core::Abi>::Abi as *const <CompositionGeometry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateViewBox<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateViewBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCommitAsync<Impl: ICompositor5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCommitAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositor5, BASE_OFFSET>(),
            Comment: Comment::<Impl, IMPL_OFFSET>,
            SetComment: SetComment::<Impl, IMPL_OFFSET>,
            GlobalPlaybackRate: GlobalPlaybackRate::<Impl, IMPL_OFFSET>,
            SetGlobalPlaybackRate: SetGlobalPlaybackRate::<Impl, IMPL_OFFSET>,
            CreateBounceScalarAnimation: CreateBounceScalarAnimation::<Impl, IMPL_OFFSET>,
            CreateBounceVector2Animation: CreateBounceVector2Animation::<Impl, IMPL_OFFSET>,
            CreateBounceVector3Animation: CreateBounceVector3Animation::<Impl, IMPL_OFFSET>,
            CreateContainerShape: CreateContainerShape::<Impl, IMPL_OFFSET>,
            CreateEllipseGeometry: CreateEllipseGeometry::<Impl, IMPL_OFFSET>,
            CreateLineGeometry: CreateLineGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Impl, IMPL_OFFSET>,
            CreatePathGeometryWithPath: CreatePathGeometryWithPath::<Impl, IMPL_OFFSET>,
            CreatePathKeyFrameAnimation: CreatePathKeyFrameAnimation::<Impl, IMPL_OFFSET>,
            CreateRectangleGeometry: CreateRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateRoundedRectangleGeometry: CreateRoundedRectangleGeometry::<Impl, IMPL_OFFSET>,
            CreateShapeVisual: CreateShapeVisual::<Impl, IMPL_OFFSET>,
            CreateSpriteShape: CreateSpriteShape::<Impl, IMPL_OFFSET>,
            CreateSpriteShapeWithGeometry: CreateSpriteShapeWithGeometry::<Impl, IMPL_OFFSET>,
            CreateViewBox: CreateViewBox::<Impl, IMPL_OFFSET>,
            RequestCommitAsync: RequestCommitAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositor5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor6Impl: Sized {
    fn CreateGeometricClip(&self) -> ::windows::core::Result<CompositionGeometricClip>;
    fn CreateGeometricClipWithGeometry(&self, geometry: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<CompositionGeometricClip>;
    fn CreateRedirectVisual(&self) -> ::windows::core::Result<RedirectVisual>;
    fn CreateRedirectVisualWithSourceVisual(&self, source: &::core::option::Option<Visual>) -> ::windows::core::Result<RedirectVisual>;
    fn CreateBooleanKeyFrameAnimation(&self) -> ::windows::core::Result<BooleanKeyFrameAnimation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor6 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor6";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor6Vtbl {
        unsafe extern "system" fn CreateGeometricClip<Impl: ICompositor6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometricClip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometricClipWithGeometry<Impl: ICompositor6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometricClipWithGeometry(&*(&geometry as *const <CompositionGeometry as ::windows::core::Abi>::Abi as *const <CompositionGeometry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRedirectVisual<Impl: ICompositor6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRedirectVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRedirectVisualWithSourceVisual<Impl: ICompositor6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRedirectVisualWithSourceVisual(&*(&source as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBooleanKeyFrameAnimation<Impl: ICompositor6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBooleanKeyFrameAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositor6, BASE_OFFSET>(),
            CreateGeometricClip: CreateGeometricClip::<Impl, IMPL_OFFSET>,
            CreateGeometricClipWithGeometry: CreateGeometricClipWithGeometry::<Impl, IMPL_OFFSET>,
            CreateRedirectVisual: CreateRedirectVisual::<Impl, IMPL_OFFSET>,
            CreateRedirectVisualWithSourceVisual: CreateRedirectVisualWithSourceVisual::<Impl, IMPL_OFFSET>,
            CreateBooleanKeyFrameAnimation: CreateBooleanKeyFrameAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositor6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "System", feature = "implement_exclusive"))]
pub trait ICompositor7Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn CreateAnimationPropertyInfo(&self) -> ::windows::core::Result<AnimationPropertyInfo>;
    fn CreateRectangleClip(&self) -> ::windows::core::Result<RectangleClip>;
    fn CreateRectangleClipWithSides(&self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<RectangleClip>;
    fn CreateRectangleClipWithSidesAndRadius(&self, left: f32, top: f32, right: f32, bottom: f32, topleftradius: &super::super::Foundation::Numerics::Vector2, toprightradius: &super::super::Foundation::Numerics::Vector2, bottomrightradius: &super::super::Foundation::Numerics::Vector2, bottomleftradius: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<RectangleClip>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositor7 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor7";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "System", feature = "implement_exclusive"))]
impl ICompositor7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor7Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: ICompositor7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationPropertyInfo<Impl: ICompositor7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAnimationPropertyInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Impl: ICompositor7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClipWithSides<Impl: ICompositor7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClipWithSides(left, top, right, bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClipWithSidesAndRadius<Impl: ICompositor7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, topleftradius: super::super::Foundation::Numerics::Vector2, toprightradius: super::super::Foundation::Numerics::Vector2, bottomrightradius: super::super::Foundation::Numerics::Vector2, bottomleftradius: super::super::Foundation::Numerics::Vector2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectangleClipWithSidesAndRadius(
                left,
                top,
                right,
                bottom,
                &*(&topleftradius as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                &*(&toprightradius as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                &*(&bottomrightradius as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                &*(&bottomleftradius as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositor7, BASE_OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
            CreateAnimationPropertyInfo: CreateAnimationPropertyInfo::<Impl, IMPL_OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Impl, IMPL_OFFSET>,
            CreateRectangleClipWithSides: CreateRectangleClipWithSides::<Impl, IMPL_OFFSET>,
            CreateRectangleClipWithSidesAndRadius: CreateRectangleClipWithSidesAndRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositor7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorStaticsImpl: Sized {
    fn MaxGlobalPlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn MinGlobalPlaybackRate(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorStaticsVtbl {
        unsafe extern "system" fn MaxGlobalPlaybackRate<Impl: ICompositorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxGlobalPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinGlobalPlaybackRate<Impl: ICompositorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinGlobalPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositorStatics, BASE_OFFSET>(),
            MaxGlobalPlaybackRate: MaxGlobalPlaybackRate::<Impl, IMPL_OFFSET>,
            MinGlobalPlaybackRate: MinGlobalPlaybackRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithBlurredWallpaperBackdropBrushImpl: Sized {
    fn TryCreateBlurredWallpaperBackdropBrush(&self) -> ::windows::core::Result<CompositionBackdropBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithBlurredWallpaperBackdropBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithBlurredWallpaperBackdropBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithBlurredWallpaperBackdropBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithBlurredWallpaperBackdropBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithBlurredWallpaperBackdropBrushVtbl {
        unsafe extern "system" fn TryCreateBlurredWallpaperBackdropBrush<Impl: ICompositorWithBlurredWallpaperBackdropBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateBlurredWallpaperBackdropBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositorWithBlurredWallpaperBackdropBrush, BASE_OFFSET>(),
            TryCreateBlurredWallpaperBackdropBrush: TryCreateBlurredWallpaperBackdropBrush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositorWithBlurredWallpaperBackdropBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithProjectedShadowImpl: Sized {
    fn CreateProjectedShadowCaster(&self) -> ::windows::core::Result<CompositionProjectedShadowCaster>;
    fn CreateProjectedShadow(&self) -> ::windows::core::Result<CompositionProjectedShadow>;
    fn CreateProjectedShadowReceiver(&self) -> ::windows::core::Result<CompositionProjectedShadowReceiver>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithProjectedShadow {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithProjectedShadow";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithProjectedShadowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithProjectedShadowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithProjectedShadowVtbl {
        unsafe extern "system" fn CreateProjectedShadowCaster<Impl: ICompositorWithProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProjectedShadowCaster() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProjectedShadow<Impl: ICompositorWithProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProjectedShadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProjectedShadowReceiver<Impl: ICompositorWithProjectedShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProjectedShadowReceiver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositorWithProjectedShadow, BASE_OFFSET>(),
            CreateProjectedShadowCaster: CreateProjectedShadowCaster::<Impl, IMPL_OFFSET>,
            CreateProjectedShadow: CreateProjectedShadow::<Impl, IMPL_OFFSET>,
            CreateProjectedShadowReceiver: CreateProjectedShadowReceiver::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositorWithProjectedShadow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithRadialGradientImpl: Sized {
    fn CreateRadialGradientBrush(&self) -> ::windows::core::Result<CompositionRadialGradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithRadialGradient {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithRadialGradient";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithRadialGradientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithRadialGradientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithRadialGradientVtbl {
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: ICompositorWithRadialGradientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRadialGradientBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositorWithRadialGradient, BASE_OFFSET>(),
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositorWithRadialGradient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithVisualSurfaceImpl: Sized {
    fn CreateVisualSurface(&self) -> ::windows::core::Result<CompositionVisualSurface>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithVisualSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithVisualSurface";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithVisualSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithVisualSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithVisualSurfaceVtbl {
        unsafe extern "system" fn CreateVisualSurface<Impl: ICompositorWithVisualSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisualSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositorWithVisualSurface, BASE_OFFSET>(),
            CreateVisualSurface: CreateVisualSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositorWithVisualSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerVisualImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<VisualCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContainerVisual {
    const NAME: &'static str = "Windows.UI.Composition.IContainerVisual";
}
#[cfg(feature = "implement_exclusive")]
impl IContainerVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContainerVisualVtbl {
        unsafe extern "system" fn Children<Impl: IContainerVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContainerVisual, BASE_OFFSET>(), Children: Children::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContainerVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerVisualFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContainerVisualFactory {
    const NAME: &'static str = "Windows.UI.Composition.IContainerVisualFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IContainerVisualFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerVisualFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContainerVisualFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContainerVisualFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContainerVisualFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICubicBezierEasingFunctionImpl: Sized {
    fn ControlPoint1(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn ControlPoint2(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICubicBezierEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ICubicBezierEasingFunction";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICubicBezierEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICubicBezierEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICubicBezierEasingFunctionVtbl {
        unsafe extern "system" fn ControlPoint1<Impl: ICubicBezierEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ControlPoint2<Impl: ICubicBezierEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICubicBezierEasingFunction, BASE_OFFSET>(),
            ControlPoint1: ControlPoint1::<Impl, IMPL_OFFSET>,
            ControlPoint2: ControlPoint2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICubicBezierEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDelegatedInkTrailVisualImpl: Sized {
    fn AddTrailPoints(&self, inkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn AddTrailPointsWithPrediction(&self, inkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType], predictedinkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn RemoveTrailPoints(&self, generationid: u32) -> ::windows::core::Result<()>;
    fn StartNewTrail(&self, color: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDelegatedInkTrailVisual {
    const NAME: &'static str = "Windows.UI.Composition.IDelegatedInkTrailVisual";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDelegatedInkTrailVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDelegatedInkTrailVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDelegatedInkTrailVisualVtbl {
        unsafe extern "system" fn AddTrailPoints<Impl: IDelegatedInkTrailVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkPoints_array_size: u32, inkpoints: *const InkTrailPoint, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPoints(::core::slice::from_raw_parts(::core::mem::transmute_copy(&inkpoints), inkPoints_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Impl: IDelegatedInkTrailVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkPoints_array_size: u32, inkpoints: *const InkTrailPoint, predictedInkPoints_array_size: u32, predictedinkpoints: *const InkTrailPoint, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrailPointsWithPrediction(::core::slice::from_raw_parts(::core::mem::transmute_copy(&inkpoints), inkPoints_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&predictedinkpoints), predictedInkPoints_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrailPoints<Impl: IDelegatedInkTrailVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrailPoints(generationid).into()
        }
        unsafe extern "system" fn StartNewTrail<Impl: IDelegatedInkTrailVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartNewTrail(&*(&color as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDelegatedInkTrailVisual, BASE_OFFSET>(),
            AddTrailPoints: AddTrailPoints::<Impl, IMPL_OFFSET>,
            AddTrailPointsWithPrediction: AddTrailPointsWithPrediction::<Impl, IMPL_OFFSET>,
            RemoveTrailPoints: RemoveTrailPoints::<Impl, IMPL_OFFSET>,
            StartNewTrail: StartNewTrail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDelegatedInkTrailVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDelegatedInkTrailVisualStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<Compositor>) -> ::windows::core::Result<DelegatedInkTrailVisual>;
    fn CreateForSwapChain(&self, compositor: &::core::option::Option<Compositor>, swapchain: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<DelegatedInkTrailVisual>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDelegatedInkTrailVisualStatics {
    const NAME: &'static str = "Windows.UI.Composition.IDelegatedInkTrailVisualStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDelegatedInkTrailVisualStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDelegatedInkTrailVisualStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDelegatedInkTrailVisualStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IDelegatedInkTrailVisualStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForSwapChain<Impl: IDelegatedInkTrailVisualStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, swapchain: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForSwapChain(&*(&compositor as *const <Compositor as ::windows::core::Abi>::Abi as *const <Compositor as ::windows::core::DefaultType>::DefaultType), &*(&swapchain as *const <ICompositionSurface as ::windows::core::Abi>::Abi as *const <ICompositionSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDelegatedInkTrailVisualStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateForSwapChain: CreateForSwapChain::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDelegatedInkTrailVisualStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IDistantLightImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDistantLight {
    const NAME: &'static str = "Windows.UI.Composition.IDistantLight";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDistantLightVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDistantLightImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDistantLightVtbl {
        unsafe extern "system" fn Color<Impl: IDistantLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IDistantLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CoordinateSpace<Impl: IDistantLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoordinateSpace<Impl: IDistantLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoordinateSpace(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Direction<Impl: IDistantLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: IDistantLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDistantLight, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            CoordinateSpace: CoordinateSpace::<Impl, IMPL_OFFSET>,
            SetCoordinateSpace: SetCoordinateSpace::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            SetDirection: SetDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDistantLight as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDistantLight2Impl: Sized {
    fn Intensity(&self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDistantLight2 {
    const NAME: &'static str = "Windows.UI.Composition.IDistantLight2";
}
#[cfg(feature = "implement_exclusive")]
impl IDistantLight2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDistantLight2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDistantLight2Vtbl {
        unsafe extern "system" fn Intensity<Impl: IDistantLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Intensity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntensity<Impl: IDistantLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntensity(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDistantLight2, BASE_OFFSET>(),
            Intensity: Intensity::<Impl, IMPL_OFFSET>,
            SetIntensity: SetIntensity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDistantLight2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IDropShadowImpl: Sized {
    fn BlurRadius(&self) -> ::windows::core::Result<f32>;
    fn SetBlurRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Mask(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetMask(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDropShadow {
    const NAME: &'static str = "Windows.UI.Composition.IDropShadow";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDropShadowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropShadowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropShadowVtbl {
        unsafe extern "system" fn BlurRadius<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlurRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlurRadius<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlurRadius(value).into()
        }
        unsafe extern "system" fn Color<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Mask<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMask<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMask(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Opacity<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IDropShadowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDropShadow, BASE_OFFSET>(),
            BlurRadius: BlurRadius::<Impl, IMPL_OFFSET>,
            SetBlurRadius: SetBlurRadius::<Impl, IMPL_OFFSET>,
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Mask: Mask::<Impl, IMPL_OFFSET>,
            SetMask: SetMask::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            Opacity: Opacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropShadow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropShadow2Impl: Sized {
    fn SourcePolicy(&self) -> ::windows::core::Result<CompositionDropShadowSourcePolicy>;
    fn SetSourcePolicy(&self, value: CompositionDropShadowSourcePolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropShadow2 {
    const NAME: &'static str = "Windows.UI.Composition.IDropShadow2";
}
#[cfg(feature = "implement_exclusive")]
impl IDropShadow2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropShadow2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropShadow2Vtbl {
        unsafe extern "system" fn SourcePolicy<Impl: IDropShadow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionDropShadowSourcePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePolicy<Impl: IDropShadow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionDropShadowSourcePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourcePolicy(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDropShadow2, BASE_OFFSET>(),
            SourcePolicy: SourcePolicy::<Impl, IMPL_OFFSET>,
            SetSourcePolicy: SetSourcePolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropShadow2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Oscillations(&self) -> ::windows::core::Result<i32>;
    fn Springiness(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElasticEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IElasticEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IElasticEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElasticEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElasticEasingFunctionVtbl {
        unsafe extern "system" fn Mode<Impl: IElasticEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Oscillations<Impl: IElasticEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Springiness<Impl: IElasticEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElasticEasingFunction, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Oscillations: Oscillations::<Impl, IMPL_OFFSET>,
            Springiness: Springiness::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElasticEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Exponent(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExponentialEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IExponentialEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IExponentialEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExponentialEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExponentialEasingFunctionVtbl {
        unsafe extern "system" fn Mode<Impl: IExponentialEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exponent<Impl: IExponentialEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExponentialEasingFunction, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Exponent: Exponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExponentialEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExpressionAnimationImpl: Sized {
    fn Expression(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExpression(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExpressionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IExpressionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IExpressionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpressionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpressionAnimationVtbl {
        unsafe extern "system" fn Expression<Impl: IExpressionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expression() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpression<Impl: IExpressionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpression(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExpressionAnimation, BASE_OFFSET>(),
            Expression: Expression::<Impl, IMPL_OFFSET>,
            SetExpression: SetExpression::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpressionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImplicitAnimationCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImplicitAnimationCollection {
    const NAME: &'static str = "Windows.UI.Composition.IImplicitAnimationCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IImplicitAnimationCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImplicitAnimationCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImplicitAnimationCollectionVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImplicitAnimationCollection, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImplicitAnimationCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInsetClipImpl: Sized {
    fn BottomInset(&self) -> ::windows::core::Result<f32>;
    fn SetBottomInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn LeftInset(&self) -> ::windows::core::Result<f32>;
    fn SetLeftInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightInset(&self) -> ::windows::core::Result<f32>;
    fn SetRightInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopInset(&self) -> ::windows::core::Result<f32>;
    fn SetTopInset(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInsetClip {
    const NAME: &'static str = "Windows.UI.Composition.IInsetClip";
}
#[cfg(feature = "implement_exclusive")]
impl IInsetClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInsetClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInsetClipVtbl {
        unsafe extern "system" fn BottomInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BottomInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomInset(value).into()
        }
        unsafe extern "system" fn LeftInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftInset(value).into()
        }
        unsafe extern "system" fn RightInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightInset(value).into()
        }
        unsafe extern "system" fn TopInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopInset<Impl: IInsetClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopInset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInsetClip, BASE_OFFSET>(),
            BottomInset: BottomInset::<Impl, IMPL_OFFSET>,
            SetBottomInset: SetBottomInset::<Impl, IMPL_OFFSET>,
            LeftInset: LeftInset::<Impl, IMPL_OFFSET>,
            SetLeftInset: SetLeftInset::<Impl, IMPL_OFFSET>,
            RightInset: RightInset::<Impl, IMPL_OFFSET>,
            SetRightInset: SetRightInset::<Impl, IMPL_OFFSET>,
            TopInset: TopInset::<Impl, IMPL_OFFSET>,
            SetTopInset: SetTopInset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInsetClip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeyFrameAnimationImpl: Sized {
    fn DelayTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDelayTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IterationBehavior(&self) -> ::windows::core::Result<AnimationIterationBehavior>;
    fn SetIterationBehavior(&self, value: AnimationIterationBehavior) -> ::windows::core::Result<()>;
    fn IterationCount(&self) -> ::windows::core::Result<i32>;
    fn SetIterationCount(&self, value: i32) -> ::windows::core::Result<()>;
    fn KeyFrameCount(&self) -> ::windows::core::Result<i32>;
    fn StopBehavior(&self) -> ::windows::core::Result<AnimationStopBehavior>;
    fn SetStopBehavior(&self, value: AnimationStopBehavior) -> ::windows::core::Result<()>;
    fn InsertExpressionKeyFrame(&self, normalizedprogresskey: f32, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InsertExpressionKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &::windows::core::HSTRING, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimationVtbl {
        unsafe extern "system" fn DelayTime<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelayTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelayTime<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IterationBehavior<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationIterationBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IterationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIterationBehavior<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationIterationBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIterationBehavior(value).into()
        }
        unsafe extern "system" fn IterationCount<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IterationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIterationCount<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIterationCount(value).into()
        }
        unsafe extern "system" fn KeyFrameCount<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopBehavior<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationStopBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopBehavior<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationStopBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopBehavior(value).into()
        }
        unsafe extern "system" fn InsertExpressionKeyFrame<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertExpressionKeyFrame(normalizedprogresskey, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertExpressionKeyFrameWithEasingFunction<Impl: IKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertExpressionKeyFrameWithEasingFunction(normalizedprogresskey, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyFrameAnimation, BASE_OFFSET>(),
            DelayTime: DelayTime::<Impl, IMPL_OFFSET>,
            SetDelayTime: SetDelayTime::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            IterationBehavior: IterationBehavior::<Impl, IMPL_OFFSET>,
            SetIterationBehavior: SetIterationBehavior::<Impl, IMPL_OFFSET>,
            IterationCount: IterationCount::<Impl, IMPL_OFFSET>,
            SetIterationCount: SetIterationCount::<Impl, IMPL_OFFSET>,
            KeyFrameCount: KeyFrameCount::<Impl, IMPL_OFFSET>,
            StopBehavior: StopBehavior::<Impl, IMPL_OFFSET>,
            SetStopBehavior: SetStopBehavior::<Impl, IMPL_OFFSET>,
            InsertExpressionKeyFrame: InsertExpressionKeyFrame::<Impl, IMPL_OFFSET>,
            InsertExpressionKeyFrameWithEasingFunction: InsertExpressionKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyFrameAnimation2Impl: Sized {
    fn Direction(&self) -> ::windows::core::Result<AnimationDirection>;
    fn SetDirection(&self, value: AnimationDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyFrameAnimation2 {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimation2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyFrameAnimation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimation2Vtbl {
        unsafe extern "system" fn Direction<Impl: IKeyFrameAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: IKeyFrameAnimation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyFrameAnimation2, BASE_OFFSET>(),
            Direction: Direction::<Impl, IMPL_OFFSET>,
            SetDirection: SetDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyFrameAnimation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyFrameAnimation3Impl: Sized {
    fn DelayBehavior(&self) -> ::windows::core::Result<AnimationDelayBehavior>;
    fn SetDelayBehavior(&self, value: AnimationDelayBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyFrameAnimation3 {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimation3";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyFrameAnimation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimation3Vtbl {
        unsafe extern "system" fn DelayBehavior<Impl: IKeyFrameAnimation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDelayBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelayBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelayBehavior<Impl: IKeyFrameAnimation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationDelayBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayBehavior(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyFrameAnimation3, BASE_OFFSET>(),
            DelayBehavior: DelayBehavior::<Impl, IMPL_OFFSET>,
            SetDelayBehavior: SetDelayBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyFrameAnimation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyFrameAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyFrameAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyFrameAnimationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimationFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyFrameAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyFrameAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayerVisualImpl: Sized {
    fn Effect(&self) -> ::windows::core::Result<CompositionEffectBrush>;
    fn SetEffect(&self, value: &::core::option::Option<CompositionEffectBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayerVisual {
    const NAME: &'static str = "Windows.UI.Composition.ILayerVisual";
}
#[cfg(feature = "implement_exclusive")]
impl ILayerVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayerVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayerVisualVtbl {
        unsafe extern "system" fn Effect<Impl: ILayerVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEffect<Impl: ILayerVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffect(&*(&value as *const <CompositionEffectBrush as ::windows::core::Abi>::Abi as *const <CompositionEffectBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILayerVisual, BASE_OFFSET>(),
            Effect: Effect::<Impl, IMPL_OFFSET>,
            SetEffect: SetEffect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayerVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayerVisual2Impl: Sized {
    fn Shadow(&self) -> ::windows::core::Result<CompositionShadow>;
    fn SetShadow(&self, value: &::core::option::Option<CompositionShadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayerVisual2 {
    const NAME: &'static str = "Windows.UI.Composition.ILayerVisual2";
}
#[cfg(feature = "implement_exclusive")]
impl ILayerVisual2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayerVisual2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayerVisual2Vtbl {
        unsafe extern "system" fn Shadow<Impl: ILayerVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Impl: ILayerVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShadow(&*(&value as *const <CompositionShadow as ::windows::core::Abi>::Abi as *const <CompositionShadow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILayerVisual2, BASE_OFFSET>(),
            Shadow: Shadow::<Impl, IMPL_OFFSET>,
            SetShadow: SetShadow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayerVisual2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearEasingFunctionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ILinearEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearEasingFunctionVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearEasingFunction, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INaturalMotionAnimationImpl: Sized {
    fn DelayBehavior(&self) -> ::windows::core::Result<AnimationDelayBehavior>;
    fn SetDelayBehavior(&self, value: AnimationDelayBehavior) -> ::windows::core::Result<()>;
    fn DelayTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDelayTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StopBehavior(&self) -> ::windows::core::Result<AnimationStopBehavior>;
    fn SetStopBehavior(&self, value: AnimationStopBehavior) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.INaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INaturalMotionAnimationVtbl {
        unsafe extern "system" fn DelayBehavior<Impl: INaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDelayBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelayBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelayBehavior<Impl: INaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationDelayBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayBehavior(value).into()
        }
        unsafe extern "system" fn DelayTime<Impl: INaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelayTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelayTime<Impl: INaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopBehavior<Impl: INaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationStopBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopBehavior<Impl: INaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationStopBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopBehavior(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INaturalMotionAnimation, BASE_OFFSET>(),
            DelayBehavior: DelayBehavior::<Impl, IMPL_OFFSET>,
            SetDelayBehavior: SetDelayBehavior::<Impl, IMPL_OFFSET>,
            DelayTime: DelayTime::<Impl, IMPL_OFFSET>,
            SetDelayTime: SetDelayTime::<Impl, IMPL_OFFSET>,
            StopBehavior: StopBehavior::<Impl, IMPL_OFFSET>,
            SetStopBehavior: SetStopBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.INaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INaturalMotionAnimationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INaturalMotionAnimationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INaturalMotionAnimationFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, path: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, path: &::core::option::Option<CompositionPath>, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IPathKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPathKeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathKeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathKeyFrameAnimationVtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IPathKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, path: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&path as *const <CompositionPath as ::windows::core::Abi>::Abi as *const <CompositionPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IPathKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, path: ::windows::core::RawPtr, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrameWithEasingFunction(normalizedprogresskey, &*(&path as *const <CompositionPath as ::windows::core::Abi>::Abi as *const <CompositionPath as ::windows::core::DefaultType>::DefaultType), &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPathKeyFrameAnimation, BASE_OFFSET>(),
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
            InsertKeyFrameWithEasingFunction: InsertKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathKeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IPointLightImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn ConstantAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetConstantAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn LinearAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetLinearAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn QuadraticAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetQuadraticAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointLight {
    const NAME: &'static str = "Windows.UI.Composition.IPointLight";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IPointLightVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointLightImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointLightVtbl {
        unsafe extern "system" fn Color<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConstantAttenuation<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConstantAttenuation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConstantAttenuation<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstantAttenuation(value).into()
        }
        unsafe extern "system" fn CoordinateSpace<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoordinateSpace<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoordinateSpace(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LinearAttenuation<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinearAttenuation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinearAttenuation<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinearAttenuation(value).into()
        }
        unsafe extern "system" fn Offset<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuadraticAttenuation<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuadraticAttenuation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuadraticAttenuation<Impl: IPointLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuadraticAttenuation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointLight, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            ConstantAttenuation: ConstantAttenuation::<Impl, IMPL_OFFSET>,
            SetConstantAttenuation: SetConstantAttenuation::<Impl, IMPL_OFFSET>,
            CoordinateSpace: CoordinateSpace::<Impl, IMPL_OFFSET>,
            SetCoordinateSpace: SetCoordinateSpace::<Impl, IMPL_OFFSET>,
            LinearAttenuation: LinearAttenuation::<Impl, IMPL_OFFSET>,
            SetLinearAttenuation: SetLinearAttenuation::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            QuadraticAttenuation: QuadraticAttenuation::<Impl, IMPL_OFFSET>,
            SetQuadraticAttenuation: SetQuadraticAttenuation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointLight as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointLight2Impl: Sized {
    fn Intensity(&self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointLight2 {
    const NAME: &'static str = "Windows.UI.Composition.IPointLight2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointLight2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointLight2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointLight2Vtbl {
        unsafe extern "system" fn Intensity<Impl: IPointLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Intensity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntensity<Impl: IPointLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntensity(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointLight2, BASE_OFFSET>(),
            Intensity: Intensity::<Impl, IMPL_OFFSET>,
            SetIntensity: SetIntensity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointLight2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointLight3Impl: Sized {
    fn MinAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMinAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
    fn MaxAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMaxAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointLight3 {
    const NAME: &'static str = "Windows.UI.Composition.IPointLight3";
}
#[cfg(feature = "implement_exclusive")]
impl IPointLight3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointLight3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointLight3Vtbl {
        unsafe extern "system" fn MinAttenuationCutoff<Impl: IPointLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinAttenuationCutoff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinAttenuationCutoff<Impl: IPointLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinAttenuationCutoff(value).into()
        }
        unsafe extern "system" fn MaxAttenuationCutoff<Impl: IPointLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAttenuationCutoff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxAttenuationCutoff<Impl: IPointLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxAttenuationCutoff(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointLight3, BASE_OFFSET>(),
            MinAttenuationCutoff: MinAttenuationCutoff::<Impl, IMPL_OFFSET>,
            SetMinAttenuationCutoff: SetMinAttenuationCutoff::<Impl, IMPL_OFFSET>,
            MaxAttenuationCutoff: MaxAttenuationCutoff::<Impl, IMPL_OFFSET>,
            SetMaxAttenuationCutoff: SetMaxAttenuationCutoff::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointLight3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Power(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IPowerEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPowerEasingFunctionVtbl {
        unsafe extern "system" fn Mode<Impl: IPowerEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Power<Impl: IPowerEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPowerEasingFunction, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Power: Power::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPowerEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IQuaternionKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Quaternion, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IQuaternionKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IQuaternionKeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IQuaternionKeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuaternionKeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuaternionKeyFrameAnimationVtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IQuaternionKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IQuaternionKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Quaternion, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrameWithEasingFunction(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType), &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IQuaternionKeyFrameAnimation, BASE_OFFSET>(),
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
            InsertKeyFrameWithEasingFunction: InsertKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQuaternionKeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IRectangleClipImpl: Sized {
    fn Bottom(&self) -> ::windows::core::Result<f32>;
    fn SetBottom(&self, value: f32) -> ::windows::core::Result<()>;
    fn BottomLeftRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetBottomLeftRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn BottomRightRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetBottomRightRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Left(&self) -> ::windows::core::Result<f32>;
    fn SetLeft(&self, value: f32) -> ::windows::core::Result<()>;
    fn Right(&self) -> ::windows::core::Result<f32>;
    fn SetRight(&self, value: f32) -> ::windows::core::Result<()>;
    fn Top(&self) -> ::windows::core::Result<f32>;
    fn SetTop(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopLeftRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetTopLeftRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TopRightRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetTopRightRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRectangleClip {
    const NAME: &'static str = "Windows.UI.Composition.IRectangleClip";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IRectangleClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectangleClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectangleClipVtbl {
        unsafe extern "system" fn Bottom<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bottom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottom(value).into()
        }
        unsafe extern "system" fn BottomLeftRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BottomLeftRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomLeftRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BottomRightRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BottomRightRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomRightRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Left<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Left() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeft(value).into()
        }
        unsafe extern "system" fn Right<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Right() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRight(value).into()
        }
        unsafe extern "system" fn Top<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Top() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTop(value).into()
        }
        unsafe extern "system" fn TopLeftRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopLeftRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopLeftRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TopRightRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopRightRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopRightRadius<Impl: IRectangleClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopRightRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRectangleClip, BASE_OFFSET>(),
            Bottom: Bottom::<Impl, IMPL_OFFSET>,
            SetBottom: SetBottom::<Impl, IMPL_OFFSET>,
            BottomLeftRadius: BottomLeftRadius::<Impl, IMPL_OFFSET>,
            SetBottomLeftRadius: SetBottomLeftRadius::<Impl, IMPL_OFFSET>,
            BottomRightRadius: BottomRightRadius::<Impl, IMPL_OFFSET>,
            SetBottomRightRadius: SetBottomRightRadius::<Impl, IMPL_OFFSET>,
            Left: Left::<Impl, IMPL_OFFSET>,
            SetLeft: SetLeft::<Impl, IMPL_OFFSET>,
            Right: Right::<Impl, IMPL_OFFSET>,
            SetRight: SetRight::<Impl, IMPL_OFFSET>,
            Top: Top::<Impl, IMPL_OFFSET>,
            SetTop: SetTop::<Impl, IMPL_OFFSET>,
            TopLeftRadius: TopLeftRadius::<Impl, IMPL_OFFSET>,
            SetTopLeftRadius: SetTopLeftRadius::<Impl, IMPL_OFFSET>,
            TopRightRadius: TopRightRadius::<Impl, IMPL_OFFSET>,
            SetTopRightRadius: SetTopRightRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRectangleClip as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRedirectVisualImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<Visual>;
    fn SetSource(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRedirectVisual {
    const NAME: &'static str = "Windows.UI.Composition.IRedirectVisual";
}
#[cfg(feature = "implement_exclusive")]
impl IRedirectVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRedirectVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRedirectVisualVtbl {
        unsafe extern "system" fn Source<Impl: IRedirectVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IRedirectVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRedirectVisual, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRedirectVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderingDeviceReplacedEventArgsImpl: Sized {
    fn GraphicsDevice(&self) -> ::windows::core::Result<CompositionGraphicsDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderingDeviceReplacedEventArgs {
    const NAME: &'static str = "Windows.UI.Composition.IRenderingDeviceReplacedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderingDeviceReplacedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderingDeviceReplacedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderingDeviceReplacedEventArgsVtbl {
        unsafe extern "system" fn GraphicsDevice<Impl: IRenderingDeviceReplacedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraphicsDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRenderingDeviceReplacedEventArgs, BASE_OFFSET>(),
            GraphicsDevice: GraphicsDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRenderingDeviceReplacedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: f32) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: f32, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScalarKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IScalarKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IScalarKeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarKeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarKeyFrameAnimationVtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IScalarKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, value).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IScalarKeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: f32, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrameWithEasingFunction(normalizedprogresskey, value, &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScalarKeyFrameAnimation, BASE_OFFSET>(),
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
            InsertKeyFrameWithEasingFunction: InsertKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScalarKeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScalarNaturalMotionAnimationImpl: Sized {
    fn FinalValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn SetFinalValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn SetInitialValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&self) -> ::windows::core::Result<f32>;
    fn SetInitialVelocity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScalarNaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IScalarNaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScalarNaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarNaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarNaturalMotionAnimationVtbl {
        unsafe extern "system" fn FinalValue<Impl: IScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFinalValue<Impl: IScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalValue(&*(&value as *const <super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Impl: IScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(&*(&value as *const <super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialVelocity<Impl: IScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVelocity(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScalarNaturalMotionAnimation, BASE_OFFSET>(),
            FinalValue: FinalValue::<Impl, IMPL_OFFSET>,
            SetFinalValue: SetFinalValue::<Impl, IMPL_OFFSET>,
            InitialValue: InitialValue::<Impl, IMPL_OFFSET>,
            SetInitialValue: SetInitialValue::<Impl, IMPL_OFFSET>,
            InitialVelocity: InitialVelocity::<Impl, IMPL_OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScalarNaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarNaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScalarNaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IScalarNaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScalarNaturalMotionAnimationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarNaturalMotionAnimationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarNaturalMotionAnimationFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScalarNaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScalarNaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IShapeVisualImpl: Sized {
    fn Shapes(&self) -> ::windows::core::Result<CompositionShapeCollection>;
    fn ViewBox(&self) -> ::windows::core::Result<CompositionViewBox>;
    fn SetViewBox(&self, value: &::core::option::Option<CompositionViewBox>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShapeVisual {
    const NAME: &'static str = "Windows.UI.Composition.IShapeVisual";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IShapeVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShapeVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShapeVisualVtbl {
        unsafe extern "system" fn Shapes<Impl: IShapeVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shapes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewBox<Impl: IShapeVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewBox<Impl: IShapeVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewBox(&*(&value as *const <CompositionViewBox as ::windows::core::Abi>::Abi as *const <CompositionViewBox as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShapeVisual, BASE_OFFSET>(),
            Shapes: Shapes::<Impl, IMPL_OFFSET>,
            ViewBox: ViewBox::<Impl, IMPL_OFFSET>,
            SetViewBox: SetViewBox::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShapeVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISineEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISineEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ISineEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ISineEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISineEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISineEasingFunctionVtbl {
        unsafe extern "system" fn Mode<Impl: ISineEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISineEasingFunction, BASE_OFFSET>(), Mode: Mode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISineEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpotLightImpl: Sized {
    fn ConstantAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetConstantAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InnerConeAngle(&self) -> ::windows::core::Result<f32>;
    fn SetInnerConeAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn InnerConeAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetInnerConeAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn InnerConeColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetInnerConeColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn LinearAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetLinearAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn OuterConeAngle(&self) -> ::windows::core::Result<f32>;
    fn SetOuterConeAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetOuterConeAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetOuterConeColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn QuadraticAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetQuadraticAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpotLight {
    const NAME: &'static str = "Windows.UI.Composition.ISpotLight";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpotLightVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpotLightImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpotLightVtbl {
        unsafe extern "system" fn ConstantAttenuation<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConstantAttenuation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConstantAttenuation<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstantAttenuation(value).into()
        }
        unsafe extern "system" fn CoordinateSpace<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoordinateSpace<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoordinateSpace(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Direction<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InnerConeAngle<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InnerConeAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInnerConeAngle<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeAngle(value).into()
        }
        unsafe extern "system" fn InnerConeAngleInDegrees<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InnerConeAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInnerConeAngleInDegrees<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeAngleInDegrees(value).into()
        }
        unsafe extern "system" fn InnerConeColor<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InnerConeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInnerConeColor<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LinearAttenuation<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinearAttenuation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinearAttenuation<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinearAttenuation(value).into()
        }
        unsafe extern "system" fn Offset<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OuterConeAngle<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OuterConeAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOuterConeAngle<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuterConeAngle(value).into()
        }
        unsafe extern "system" fn OuterConeAngleInDegrees<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OuterConeAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOuterConeAngleInDegrees<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuterConeAngleInDegrees(value).into()
        }
        unsafe extern "system" fn OuterConeColor<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OuterConeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOuterConeColor<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuterConeColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuadraticAttenuation<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuadraticAttenuation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuadraticAttenuation<Impl: ISpotLightImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuadraticAttenuation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpotLight, BASE_OFFSET>(),
            ConstantAttenuation: ConstantAttenuation::<Impl, IMPL_OFFSET>,
            SetConstantAttenuation: SetConstantAttenuation::<Impl, IMPL_OFFSET>,
            CoordinateSpace: CoordinateSpace::<Impl, IMPL_OFFSET>,
            SetCoordinateSpace: SetCoordinateSpace::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            SetDirection: SetDirection::<Impl, IMPL_OFFSET>,
            InnerConeAngle: InnerConeAngle::<Impl, IMPL_OFFSET>,
            SetInnerConeAngle: SetInnerConeAngle::<Impl, IMPL_OFFSET>,
            InnerConeAngleInDegrees: InnerConeAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetInnerConeAngleInDegrees: SetInnerConeAngleInDegrees::<Impl, IMPL_OFFSET>,
            InnerConeColor: InnerConeColor::<Impl, IMPL_OFFSET>,
            SetInnerConeColor: SetInnerConeColor::<Impl, IMPL_OFFSET>,
            LinearAttenuation: LinearAttenuation::<Impl, IMPL_OFFSET>,
            SetLinearAttenuation: SetLinearAttenuation::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            OuterConeAngle: OuterConeAngle::<Impl, IMPL_OFFSET>,
            SetOuterConeAngle: SetOuterConeAngle::<Impl, IMPL_OFFSET>,
            OuterConeAngleInDegrees: OuterConeAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetOuterConeAngleInDegrees: SetOuterConeAngleInDegrees::<Impl, IMPL_OFFSET>,
            OuterConeColor: OuterConeColor::<Impl, IMPL_OFFSET>,
            SetOuterConeColor: SetOuterConeColor::<Impl, IMPL_OFFSET>,
            QuadraticAttenuation: QuadraticAttenuation::<Impl, IMPL_OFFSET>,
            SetQuadraticAttenuation: SetQuadraticAttenuation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpotLight as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpotLight2Impl: Sized {
    fn InnerConeIntensity(&self) -> ::windows::core::Result<f32>;
    fn SetInnerConeIntensity(&self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeIntensity(&self) -> ::windows::core::Result<f32>;
    fn SetOuterConeIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpotLight2 {
    const NAME: &'static str = "Windows.UI.Composition.ISpotLight2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpotLight2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpotLight2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpotLight2Vtbl {
        unsafe extern "system" fn InnerConeIntensity<Impl: ISpotLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InnerConeIntensity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInnerConeIntensity<Impl: ISpotLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeIntensity(value).into()
        }
        unsafe extern "system" fn OuterConeIntensity<Impl: ISpotLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OuterConeIntensity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOuterConeIntensity<Impl: ISpotLight2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuterConeIntensity(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpotLight2, BASE_OFFSET>(),
            InnerConeIntensity: InnerConeIntensity::<Impl, IMPL_OFFSET>,
            SetInnerConeIntensity: SetInnerConeIntensity::<Impl, IMPL_OFFSET>,
            OuterConeIntensity: OuterConeIntensity::<Impl, IMPL_OFFSET>,
            SetOuterConeIntensity: SetOuterConeIntensity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpotLight2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpotLight3Impl: Sized {
    fn MinAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMinAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
    fn MaxAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMaxAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpotLight3 {
    const NAME: &'static str = "Windows.UI.Composition.ISpotLight3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpotLight3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpotLight3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpotLight3Vtbl {
        unsafe extern "system" fn MinAttenuationCutoff<Impl: ISpotLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinAttenuationCutoff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinAttenuationCutoff<Impl: ISpotLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinAttenuationCutoff(value).into()
        }
        unsafe extern "system" fn MaxAttenuationCutoff<Impl: ISpotLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAttenuationCutoff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxAttenuationCutoff<Impl: ISpotLight3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxAttenuationCutoff(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpotLight3, BASE_OFFSET>(),
            MinAttenuationCutoff: MinAttenuationCutoff::<Impl, IMPL_OFFSET>,
            SetMinAttenuationCutoff: SetMinAttenuationCutoff::<Impl, IMPL_OFFSET>,
            MaxAttenuationCutoff: MaxAttenuationCutoff::<Impl, IMPL_OFFSET>,
            SetMaxAttenuationCutoff: SetMaxAttenuationCutoff::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpotLight3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpringScalarNaturalMotionAnimationImpl: Sized {
    fn DampingRatio(&self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpringScalarNaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ISpringScalarNaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpringScalarNaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpringScalarNaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpringScalarNaturalMotionAnimationVtbl {
        unsafe extern "system" fn DampingRatio<Impl: ISpringScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DampingRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDampingRatio<Impl: ISpringScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDampingRatio(value).into()
        }
        unsafe extern "system" fn Period<Impl: ISpringScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Period() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPeriod<Impl: ISpringScalarNaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPeriod(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpringScalarNaturalMotionAnimation, BASE_OFFSET>(),
            DampingRatio: DampingRatio::<Impl, IMPL_OFFSET>,
            SetDampingRatio: SetDampingRatio::<Impl, IMPL_OFFSET>,
            Period: Period::<Impl, IMPL_OFFSET>,
            SetPeriod: SetPeriod::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpringScalarNaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpringVector2NaturalMotionAnimationImpl: Sized {
    fn DampingRatio(&self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpringVector2NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ISpringVector2NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpringVector2NaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpringVector2NaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpringVector2NaturalMotionAnimationVtbl {
        unsafe extern "system" fn DampingRatio<Impl: ISpringVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DampingRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDampingRatio<Impl: ISpringVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDampingRatio(value).into()
        }
        unsafe extern "system" fn Period<Impl: ISpringVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Period() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPeriod<Impl: ISpringVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPeriod(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpringVector2NaturalMotionAnimation, BASE_OFFSET>(),
            DampingRatio: DampingRatio::<Impl, IMPL_OFFSET>,
            SetDampingRatio: SetDampingRatio::<Impl, IMPL_OFFSET>,
            Period: Period::<Impl, IMPL_OFFSET>,
            SetPeriod: SetPeriod::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpringVector2NaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpringVector3NaturalMotionAnimationImpl: Sized {
    fn DampingRatio(&self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpringVector3NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ISpringVector3NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpringVector3NaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpringVector3NaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpringVector3NaturalMotionAnimationVtbl {
        unsafe extern "system" fn DampingRatio<Impl: ISpringVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DampingRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDampingRatio<Impl: ISpringVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDampingRatio(value).into()
        }
        unsafe extern "system" fn Period<Impl: ISpringVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Period() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPeriod<Impl: ISpringVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPeriod(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpringVector3NaturalMotionAnimation, BASE_OFFSET>(),
            DampingRatio: DampingRatio::<Impl, IMPL_OFFSET>,
            SetDampingRatio: SetDampingRatio::<Impl, IMPL_OFFSET>,
            Period: Period::<Impl, IMPL_OFFSET>,
            SetPeriod: SetPeriod::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpringVector3NaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpriteVisualImpl: Sized {
    fn Brush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpriteVisual {
    const NAME: &'static str = "Windows.UI.Composition.ISpriteVisual";
}
#[cfg(feature = "implement_exclusive")]
impl ISpriteVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpriteVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpriteVisualVtbl {
        unsafe extern "system" fn Brush<Impl: ISpriteVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Brush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrush<Impl: ISpriteVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrush(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpriteVisual, BASE_OFFSET>(),
            Brush: Brush::<Impl, IMPL_OFFSET>,
            SetBrush: SetBrush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpriteVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpriteVisual2Impl: Sized {
    fn Shadow(&self) -> ::windows::core::Result<CompositionShadow>;
    fn SetShadow(&self, value: &::core::option::Option<CompositionShadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpriteVisual2 {
    const NAME: &'static str = "Windows.UI.Composition.ISpriteVisual2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpriteVisual2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpriteVisual2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpriteVisual2Vtbl {
        unsafe extern "system" fn Shadow<Impl: ISpriteVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Impl: ISpriteVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShadow(&*(&value as *const <CompositionShadow as ::windows::core::Abi>::Abi as *const <CompositionShadow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpriteVisual2, BASE_OFFSET>(),
            Shadow: Shadow::<Impl, IMPL_OFFSET>,
            SetShadow: SetShadow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpriteVisual2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStepEasingFunctionImpl: Sized {
    fn FinalStep(&self) -> ::windows::core::Result<i32>;
    fn SetFinalStep(&self, value: i32) -> ::windows::core::Result<()>;
    fn InitialStep(&self) -> ::windows::core::Result<i32>;
    fn SetInitialStep(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsFinalStepSingleFrame(&self) -> ::windows::core::Result<bool>;
    fn SetIsFinalStepSingleFrame(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsInitialStepSingleFrame(&self) -> ::windows::core::Result<bool>;
    fn SetIsInitialStepSingleFrame(&self, value: bool) -> ::windows::core::Result<()>;
    fn StepCount(&self) -> ::windows::core::Result<i32>;
    fn SetStepCount(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStepEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IStepEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IStepEasingFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStepEasingFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStepEasingFunctionVtbl {
        unsafe extern "system" fn FinalStep<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFinalStep<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalStep(value).into()
        }
        unsafe extern "system" fn InitialStep<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialStep<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialStep(value).into()
        }
        unsafe extern "system" fn IsFinalStepSingleFrame<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFinalStepSingleFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFinalStepSingleFrame<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFinalStepSingleFrame(value).into()
        }
        unsafe extern "system" fn IsInitialStepSingleFrame<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInitialStepSingleFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInitialStepSingleFrame<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInitialStepSingleFrame(value).into()
        }
        unsafe extern "system" fn StepCount<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StepCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStepCount<Impl: IStepEasingFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStepCount(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStepEasingFunction, BASE_OFFSET>(),
            FinalStep: FinalStep::<Impl, IMPL_OFFSET>,
            SetFinalStep: SetFinalStep::<Impl, IMPL_OFFSET>,
            InitialStep: InitialStep::<Impl, IMPL_OFFSET>,
            SetInitialStep: SetInitialStep::<Impl, IMPL_OFFSET>,
            IsFinalStepSingleFrame: IsFinalStepSingleFrame::<Impl, IMPL_OFFSET>,
            SetIsFinalStepSingleFrame: SetIsFinalStepSingleFrame::<Impl, IMPL_OFFSET>,
            IsInitialStepSingleFrame: IsInitialStepSingleFrame::<Impl, IMPL_OFFSET>,
            SetIsInitialStepSingleFrame: SetIsInitialStepSingleFrame::<Impl, IMPL_OFFSET>,
            StepCount: StepCount::<Impl, IMPL_OFFSET>,
            SetStepCount: SetStepCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStepEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVector2KeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector2, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector2KeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector2KeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector2KeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector2KeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector2KeyFrameAnimationVtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IVector2KeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IVector2KeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector2, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrameWithEasingFunction(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType), &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector2KeyFrameAnimation, BASE_OFFSET>(),
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
            InsertKeyFrameWithEasingFunction: InsertKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector2KeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVector2NaturalMotionAnimationImpl: Sized {
    fn FinalValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>;
    fn SetFinalValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>;
    fn SetInitialValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetInitialVelocity(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector2NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector2NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector2NaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector2NaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector2NaturalMotionAnimationVtbl {
        unsafe extern "system" fn FinalValue<Impl: IVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFinalValue<Impl: IVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Impl: IVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialVelocity<Impl: IVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IVector2NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVelocity(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector2NaturalMotionAnimation, BASE_OFFSET>(),
            FinalValue: FinalValue::<Impl, IMPL_OFFSET>,
            SetFinalValue: SetFinalValue::<Impl, IMPL_OFFSET>,
            InitialValue: InitialValue::<Impl, IMPL_OFFSET>,
            SetInitialValue: SetInitialValue::<Impl, IMPL_OFFSET>,
            InitialVelocity: InitialVelocity::<Impl, IMPL_OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector2NaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector2NaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVector2NaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IVector2NaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVector2NaturalMotionAnimationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector2NaturalMotionAnimationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector2NaturalMotionAnimationFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVector2NaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector2NaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVector3KeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector3, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector3KeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector3KeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector3KeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3KeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3KeyFrameAnimationVtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IVector3KeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IVector3KeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector3, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrameWithEasingFunction(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector3KeyFrameAnimation, BASE_OFFSET>(),
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
            InsertKeyFrameWithEasingFunction: InsertKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector3KeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVector3NaturalMotionAnimationImpl: Sized {
    fn FinalValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>;
    fn SetFinalValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>;
    fn SetInitialValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetInitialVelocity(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector3NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector3NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector3NaturalMotionAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3NaturalMotionAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3NaturalMotionAnimationVtbl {
        unsafe extern "system" fn FinalValue<Impl: IVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFinalValue<Impl: IVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Impl: IVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialVelocity<Impl: IVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IVector3NaturalMotionAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialVelocity(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector3NaturalMotionAnimation, BASE_OFFSET>(),
            FinalValue: FinalValue::<Impl, IMPL_OFFSET>,
            SetFinalValue: SetFinalValue::<Impl, IMPL_OFFSET>,
            InitialValue: InitialValue::<Impl, IMPL_OFFSET>,
            SetInitialValue: SetInitialValue::<Impl, IMPL_OFFSET>,
            InitialVelocity: InitialVelocity::<Impl, IMPL_OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector3NaturalMotionAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3NaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVector3NaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IVector3NaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVector3NaturalMotionAnimationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3NaturalMotionAnimationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3NaturalMotionAnimationFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVector3NaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector3NaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVector4KeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector4, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector4KeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector4KeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector4KeyFrameAnimationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector4KeyFrameAnimationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector4KeyFrameAnimationVtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IVector4KeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IVector4KeyFrameAnimationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector4, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrameWithEasingFunction(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType), &*(&easingfunction as *const <CompositionEasingFunction as ::windows::core::Abi>::Abi as *const <CompositionEasingFunction as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector4KeyFrameAnimation, BASE_OFFSET>(),
            InsertKeyFrame: InsertKeyFrame::<Impl, IMPL_OFFSET>,
            InsertKeyFrameWithEasingFunction: InsertKeyFrameWithEasingFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector4KeyFrameAnimation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVisualImpl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn BackfaceVisibility(&self) -> ::windows::core::Result<CompositionBackfaceVisibility>;
    fn SetBackfaceVisibility(&self, value: CompositionBackfaceVisibility) -> ::windows::core::Result<()>;
    fn BorderMode(&self) -> ::windows::core::Result<CompositionBorderMode>;
    fn SetBorderMode(&self, value: CompositionBorderMode) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Clip(&self) -> ::windows::core::Result<CompositionClip>;
    fn SetClip(&self, value: &::core::option::Option<CompositionClip>) -> ::windows::core::Result<()>;
    fn CompositeMode(&self) -> ::windows::core::Result<CompositionCompositeMode>;
    fn SetCompositeMode(&self, value: CompositionCompositeMode) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<ContainerVisual>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAxis(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisual {
    const NAME: &'static str = "Windows.UI.Composition.IVisual";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualVtbl {
        unsafe extern "system" fn AnchorPoint<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnchorPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnchorPoint<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackfaceVisibility<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionBackfaceVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackfaceVisibility() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackfaceVisibility<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionBackfaceVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackfaceVisibility(value).into()
        }
        unsafe extern "system" fn BorderMode<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionBorderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BorderMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderMode<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionBorderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(value).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterPoint<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clip<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClip<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClip(&*(&value as *const <CompositionClip as ::windows::core::Abi>::Abi as *const <CompositionClip as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompositeMode<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionCompositeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositeMode<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionCompositeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompositeMode(value).into()
        }
        unsafe extern "system" fn IsVisible<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsVisible<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVisible(value).into()
        }
        unsafe extern "system" fn Offset<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Opacity<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn Orientation<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationAngle<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngle<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn RotationAxis<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAxis() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAxis<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAxis(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Scale<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: IVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisual, BASE_OFFSET>(),
            AnchorPoint: AnchorPoint::<Impl, IMPL_OFFSET>,
            SetAnchorPoint: SetAnchorPoint::<Impl, IMPL_OFFSET>,
            BackfaceVisibility: BackfaceVisibility::<Impl, IMPL_OFFSET>,
            SetBackfaceVisibility: SetBackfaceVisibility::<Impl, IMPL_OFFSET>,
            BorderMode: BorderMode::<Impl, IMPL_OFFSET>,
            SetBorderMode: SetBorderMode::<Impl, IMPL_OFFSET>,
            CenterPoint: CenterPoint::<Impl, IMPL_OFFSET>,
            SetCenterPoint: SetCenterPoint::<Impl, IMPL_OFFSET>,
            Clip: Clip::<Impl, IMPL_OFFSET>,
            SetClip: SetClip::<Impl, IMPL_OFFSET>,
            CompositeMode: CompositeMode::<Impl, IMPL_OFFSET>,
            SetCompositeMode: SetCompositeMode::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
            SetIsVisible: SetIsVisible::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            Opacity: Opacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            SetOrientation: SetOrientation::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            RotationAngle: RotationAngle::<Impl, IMPL_OFFSET>,
            SetRotationAngle: SetRotationAngle::<Impl, IMPL_OFFSET>,
            RotationAngleInDegrees: RotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetRotationAngleInDegrees: SetRotationAngleInDegrees::<Impl, IMPL_OFFSET>,
            RotationAxis: RotationAxis::<Impl, IMPL_OFFSET>,
            SetRotationAxis: SetRotationAxis::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            TransformMatrix: TransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVisual2Impl: Sized {
    fn ParentForTransform(&self) -> ::windows::core::Result<Visual>;
    fn SetParentForTransform(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RelativeOffsetAdjustment(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRelativeOffsetAdjustment(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RelativeSizeAdjustment(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetRelativeSizeAdjustment(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisual2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisual2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVisual2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisual2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisual2Vtbl {
        unsafe extern "system" fn ParentForTransform<Impl: IVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentForTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentForTransform<Impl: IVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentForTransform(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeOffsetAdjustment<Impl: IVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeOffsetAdjustment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeOffsetAdjustment<Impl: IVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeOffsetAdjustment(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeSizeAdjustment<Impl: IVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeSizeAdjustment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeSizeAdjustment<Impl: IVisual2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeSizeAdjustment(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisual2, BASE_OFFSET>(),
            ParentForTransform: ParentForTransform::<Impl, IMPL_OFFSET>,
            SetParentForTransform: SetParentForTransform::<Impl, IMPL_OFFSET>,
            RelativeOffsetAdjustment: RelativeOffsetAdjustment::<Impl, IMPL_OFFSET>,
            SetRelativeOffsetAdjustment: SetRelativeOffsetAdjustment::<Impl, IMPL_OFFSET>,
            RelativeSizeAdjustment: RelativeSizeAdjustment::<Impl, IMPL_OFFSET>,
            SetRelativeSizeAdjustment: SetRelativeSizeAdjustment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisual2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisual3Impl: Sized {
    fn IsHitTestVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisual3 {
    const NAME: &'static str = "Windows.UI.Composition.IVisual3";
}
#[cfg(feature = "implement_exclusive")]
impl IVisual3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisual3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisual3Vtbl {
        unsafe extern "system" fn IsHitTestVisible<Impl: IVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHitTestVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHitTestVisible<Impl: IVisual3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHitTestVisible(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisual3, BASE_OFFSET>(),
            IsHitTestVisible: IsHitTestVisible::<Impl, IMPL_OFFSET>,
            SetIsHitTestVisible: SetIsHitTestVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisual3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisual4Impl: Sized {
    fn IsPixelSnappingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPixelSnappingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisual4 {
    const NAME: &'static str = "Windows.UI.Composition.IVisual4";
}
#[cfg(feature = "implement_exclusive")]
impl IVisual4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisual4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisual4Vtbl {
        unsafe extern "system" fn IsPixelSnappingEnabled<Impl: IVisual4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPixelSnappingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPixelSnappingEnabled<Impl: IVisual4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPixelSnappingEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisual4, BASE_OFFSET>(),
            IsPixelSnappingEnabled: IsPixelSnappingEnabled::<Impl, IMPL_OFFSET>,
            SetIsPixelSnappingEnabled: SetIsPixelSnappingEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisual4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn InsertAbove(&self, newchild: &::core::option::Option<Visual>, sibling: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertAtBottom(&self, newchild: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertAtTop(&self, newchild: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertBelow(&self, newchild: &::core::option::Option<Visual>, sibling: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Remove(&self, child: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualCollection {
    const NAME: &'static str = "Windows.UI.Composition.IVisualCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAbove<Impl: IVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, sibling: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAbove(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType), &*(&sibling as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtBottom<Impl: IVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtBottom(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtTop<Impl: IVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtTop(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertBelow<Impl: IVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, sibling: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertBelow(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType), &*(&sibling as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&child as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualCollection, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            InsertAbove: InsertAbove::<Impl, IMPL_OFFSET>,
            InsertAtBottom: InsertAtBottom::<Impl, IMPL_OFFSET>,
            InsertAtTop: InsertAtTop::<Impl, IMPL_OFFSET>,
            InsertBelow: InsertBelow::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualCollection as ::windows::core::Interface>::IID
    }
}
pub trait IVisualElementImpl: Sized {}
impl ::windows::core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement";
}
impl IVisualElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElementVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElement, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElement as ::windows::core::Interface>::IID
    }
}
pub trait IVisualElement2Impl: Sized {
    fn GetVisualInternal(&self) -> ::windows::core::Result<Visual>;
}
impl ::windows::core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement2";
}
impl IVisualElement2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElement2Vtbl {
        unsafe extern "system" fn GetVisualInternal<Impl: IVisualElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisualInternal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElement2, BASE_OFFSET>(),
            GetVisualInternal: GetVisualInternal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualFactory {
    const NAME: &'static str = "Windows.UI.Composition.IVisualFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualUnorderedCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, newvisual: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Remove(&self, visual: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualUnorderedCollection {
    const NAME: &'static str = "Windows.UI.Composition.IVisualUnorderedCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualUnorderedCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualUnorderedCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualUnorderedCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IVisualUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IVisualUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newvisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&newvisual as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IVisualUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&visual as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IVisualUnorderedCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualUnorderedCollection, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualUnorderedCollection as ::windows::core::Interface>::IID
    }
}
