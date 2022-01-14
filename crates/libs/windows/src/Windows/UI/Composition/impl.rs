#[cfg(feature = "implement_exclusive")]
pub trait IAmbientLight_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAmbientLight {
    const NAME: &'static str = "Windows.UI.Composition.IAmbientLight";
}
#[cfg(feature = "implement_exclusive")]
impl IAmbientLight_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAmbientLight_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAmbientLight_Vtbl {
        unsafe extern "system" fn Color<Impl: IAmbientLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IAmbientLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
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
pub trait IAmbientLight2_Impl: Sized {
    fn Intensity(&mut self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAmbientLight2 {
    const NAME: &'static str = "Windows.UI.Composition.IAmbientLight2";
}
#[cfg(feature = "implement_exclusive")]
impl IAmbientLight2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAmbientLight2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAmbientLight2_Vtbl {
        unsafe extern "system" fn Intensity<Impl: IAmbientLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIntensity<Impl: IAmbientLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IAnimationController_Impl: Sized {
    fn PlaybackRate(&mut self) -> ::windows::core::Result<f32>;
    fn SetPlaybackRate(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Progress(&mut self) -> ::windows::core::Result<f32>;
    fn SetProgress(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn ProgressBehavior(&mut self) -> ::windows::core::Result<AnimationControllerProgressBehavior>;
    fn SetProgressBehavior(&mut self, value: AnimationControllerProgressBehavior) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationController {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationController";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationController_Vtbl {
        unsafe extern "system" fn PlaybackRate<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackRate<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn Progress<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProgress<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(value).into()
        }
        unsafe extern "system" fn ProgressBehavior<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationControllerProgressBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProgressBehavior<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationControllerProgressBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgressBehavior(value).into()
        }
        unsafe extern "system" fn Pause<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IAnimationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAnimationControllerStatics_Impl: Sized {
    fn MaxPlaybackRate(&mut self) -> ::windows::core::Result<f32>;
    fn MinPlaybackRate(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationControllerStatics {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationControllerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationControllerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationControllerStatics_Vtbl {
        unsafe extern "system" fn MaxPlaybackRate<Impl: IAnimationControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinPlaybackRate<Impl: IAnimationControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IAnimationObject_Impl: Sized {
    fn PopulatePropertyInfo(&mut self, propertyname: &::windows::core::HSTRING, propertyinfo: &::core::option::Option<AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
impl IAnimationObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationObject_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfo<Impl: IAnimationObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAnimationPropertyInfo_Impl: Sized {
    fn AccessMode(&mut self) -> ::windows::core::Result<AnimationPropertyAccessMode>;
    fn SetAccessMode(&mut self, value: AnimationPropertyAccessMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationPropertyInfo {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationPropertyInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationPropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationPropertyInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationPropertyInfo_Vtbl {
        unsafe extern "system" fn AccessMode<Impl: IAnimationPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationPropertyAccessMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccessMode<Impl: IAnimationPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationPropertyAccessMode) -> ::windows::core::HRESULT {
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
pub trait IAnimationPropertyInfo2_Impl: Sized {
    fn GetResolvedCompositionObject(&mut self) -> ::windows::core::Result<CompositionObject>;
    fn GetResolvedCompositionObjectProperty(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAnimationPropertyInfo2 {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationPropertyInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAnimationPropertyInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationPropertyInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationPropertyInfo2_Vtbl {
        unsafe extern "system" fn GetResolvedCompositionObject<Impl: IAnimationPropertyInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetResolvedCompositionObjectProperty<Impl: IAnimationPropertyInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IBackEasingFunction_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Amplitude(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IBackEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IBackEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackEasingFunction_Vtbl {
        unsafe extern "system" fn Mode<Impl: IBackEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Amplitude<Impl: IBackEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IBooleanKeyFrameAnimation_Impl: Sized {
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBooleanKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBooleanKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBooleanKeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBooleanKeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBooleanKeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IBooleanKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: bool) -> ::windows::core::HRESULT {
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
pub trait IBounceEasingFunction_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Bounces(&mut self) -> ::windows::core::Result<i32>;
    fn Bounciness(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IBounceEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceEasingFunction_Vtbl {
        unsafe extern "system" fn Mode<Impl: IBounceEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Bounces<Impl: IBounceEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Bounciness<Impl: IBounceEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IBounceScalarNaturalMotionAnimation_Impl: Sized {
    fn Acceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&mut self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceScalarNaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBounceScalarNaturalMotionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceScalarNaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceScalarNaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceScalarNaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn Acceleration<Impl: IBounceScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAcceleration<Impl: IBounceScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleration(value).into()
        }
        unsafe extern "system" fn Restitution<Impl: IBounceScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRestitution<Impl: IBounceScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IBounceVector2NaturalMotionAnimation_Impl: Sized {
    fn Acceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&mut self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceVector2NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBounceVector2NaturalMotionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceVector2NaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceVector2NaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceVector2NaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn Acceleration<Impl: IBounceVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAcceleration<Impl: IBounceVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleration(value).into()
        }
        unsafe extern "system" fn Restitution<Impl: IBounceVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRestitution<Impl: IBounceVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IBounceVector3NaturalMotionAnimation_Impl: Sized {
    fn Acceleration(&mut self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&mut self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBounceVector3NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IBounceVector3NaturalMotionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IBounceVector3NaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBounceVector3NaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBounceVector3NaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn Acceleration<Impl: IBounceVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAcceleration<Impl: IBounceVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleration(value).into()
        }
        unsafe extern "system" fn Restitution<Impl: IBounceVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRestitution<Impl: IBounceVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ICircleEasingFunction_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICircleEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ICircleEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ICircleEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICircleEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICircleEasingFunction_Vtbl {
        unsafe extern "system" fn Mode<Impl: ICircleEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
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
pub trait IColorKeyFrameAnimation_Impl: Sized {
    fn InterpolationColorSpace(&mut self) -> ::windows::core::Result<CompositionColorSpace>;
    fn SetInterpolationColorSpace(&mut self, value: CompositionColorSpace) -> ::windows::core::Result<()>;
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, value: &super::Color) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, value: &super::Color, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IColorKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IColorKeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorKeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorKeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InterpolationColorSpace<Impl: IColorKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionColorSpace) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInterpolationColorSpace<Impl: IColorKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterpolationColorSpace(value).into()
        }
        unsafe extern "system" fn InsertKeyFrame<Impl: IColorKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IColorKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::Color, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionAnimation_Impl: Sized {
    fn ClearAllParameters(&mut self) -> ::windows::core::Result<()>;
    fn ClearParameter(&mut self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetColorParameter(&mut self, key: &::windows::core::HSTRING, value: &super::Color) -> ::windows::core::Result<()>;
    fn SetMatrix3x2Parameter(&mut self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetMatrix4x4Parameter(&mut self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn SetQuaternionParameter(&mut self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn SetReferenceParameter(&mut self, key: &::windows::core::HSTRING, compositionobject: &::core::option::Option<CompositionObject>) -> ::windows::core::Result<()>;
    fn SetScalarParameter(&mut self, key: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn SetVector2Parameter(&mut self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn SetVector3Parameter(&mut self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetVector4Parameter(&mut self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimation_Vtbl {
        unsafe extern "system" fn ClearAllParameters<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllParameters().into()
        }
        unsafe extern "system" fn ClearParameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetColorParameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMatrix3x2Parameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix3x2Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMatrix4x4Parameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix4x4Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetQuaternionParameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuaternionParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetReferenceParameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, compositionobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferenceParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&compositionobject as *const <CompositionObject as ::windows::core::Abi>::Abi as *const <CompositionObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetScalarParameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScalarParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetVector2Parameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVector2Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetVector3Parameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVector3Parameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetVector4Parameter<Impl: ICompositionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
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
pub trait ICompositionAnimation2_Impl: Sized {
    fn SetBooleanParameter(&mut self, key: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn Target(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTarget(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimation2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimation2_Vtbl {
        unsafe extern "system" fn SetBooleanParameter<Impl: ICompositionAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBooleanParameter(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn Target<Impl: ICompositionAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTarget<Impl: ICompositionAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ICompositionAnimation3_Impl: Sized {
    fn InitialValueExpressions(&mut self) -> ::windows::core::Result<InitialValueExpressionCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionAnimation3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompositionAnimation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimation3_Vtbl {
        unsafe extern "system" fn InitialValueExpressions<Impl: ICompositionAnimation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionAnimation4_Impl: Sized {
    fn SetExpressionReferenceParameter(&mut self, parametername: &::windows::core::HSTRING, source: &::core::option::Option<IAnimationObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimation4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimation4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimation4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimation4_Vtbl {
        unsafe extern "system" fn SetExpressionReferenceParameter<Impl: ICompositionAnimation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parametername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionAnimationBase_Impl: Sized {}
impl ::windows::core::RuntimeName for ICompositionAnimationBase {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationBase";
}
impl ICompositionAnimationBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationBase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimationBase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimationBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimationGroup_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, value: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, value: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionAnimationGroup {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationGroup";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionAnimationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationGroup_Vtbl {
        unsafe extern "system" fn Count<Impl: ICompositionAnimationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Add<Impl: ICompositionAnimationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&value as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICompositionAnimationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&value as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionAnimationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ICompositionBackdropBrush_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBackdropBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBackdropBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBackdropBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBackdropBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBackdropBrush_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBackdropBrush, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBackdropBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBatchCompletedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBatchCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBatchCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBatchCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBatchCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBatchCompletedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBatchCompletedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBatchCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBrush_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBrush_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBrush, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBrushFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionBrushFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionBrushFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionBrushFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositionCapabilities_Impl: Sized {
    fn AreEffectsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn AreEffectsFast(&mut self) -> ::windows::core::Result<bool>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CompositionCapabilities, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionCapabilities {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionCapabilities";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionCapabilities_Vtbl {
        unsafe extern "system" fn AreEffectsSupported<Impl: ICompositionCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AreEffectsFast<Impl: ICompositionCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Changed<Impl: ICompositionCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChanged<Impl: ICompositionCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait ICompositionCapabilitiesStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<CompositionCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionCapabilitiesStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionCapabilitiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionCapabilitiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionCapabilitiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionCapabilitiesStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICompositionCapabilitiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionClip_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionClip {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionClip";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionClip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionClip_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionClip, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionClip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionClip2_Impl: Sized {
    fn AnchorPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&mut self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionClip2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionClip2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionClip2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionClip2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionClip2_Vtbl {
        unsafe extern "system" fn AnchorPoint<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAnchorPoint<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScale<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionClip2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
pub trait ICompositionClipFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionClipFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionClipFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionClipFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionClipFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionClipFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionClipFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionClipFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionColorBrush_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionColorBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionColorBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionColorBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionColorBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionColorBrush_Vtbl {
        unsafe extern "system" fn Color<Impl: ICompositionColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: ICompositionColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
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
pub trait ICompositionColorGradientStop_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<f32>;
    fn SetOffset(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionColorGradientStop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionColorGradientStop";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionColorGradientStop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionColorGradientStop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionColorGradientStop_Vtbl {
        unsafe extern "system" fn Color<Impl: ICompositionColorGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: ICompositionColorGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionColorGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionColorGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ICompositionColorGradientStopCollection_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionColorGradientStopCollection {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionColorGradientStopCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionColorGradientStopCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionColorGradientStopCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionColorGradientStopCollection_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionColorGradientStopCollection, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionColorGradientStopCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICompositionCommitBatch_Impl: Sized {
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnded(&mut self) -> ::windows::core::Result<bool>;
    fn Completed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionCommitBatch {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionCommitBatch";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionCommitBatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionCommitBatch_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionCommitBatch_Vtbl {
        unsafe extern "system" fn IsActive<Impl: ICompositionCommitBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnded<Impl: ICompositionCommitBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Completed<Impl: ICompositionCommitBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCompleted<Impl: ICompositionCommitBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait ICompositionContainerShape_Impl: Sized {
    fn Shapes(&mut self) -> ::windows::core::Result<CompositionShapeCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionContainerShape {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionContainerShape";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompositionContainerShape_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionContainerShape_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionContainerShape_Vtbl {
        unsafe extern "system" fn Shapes<Impl: ICompositionContainerShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionDrawingSurface_Impl: Sized {
    fn AlphaMode(&mut self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXAlphaMode>;
    fn PixelFormat(&mut self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionDrawingSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionDrawingSurface";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionDrawingSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurface_Vtbl {
        unsafe extern "system" fn AlphaMode<Impl: ICompositionDrawingSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelFormat<Impl: ICompositionDrawingSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<Impl: ICompositionDrawingSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
pub trait ICompositionDrawingSurface2_Impl: Sized {
    fn SizeInt32(&mut self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn Resize(&mut self, sizepixels: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn Scroll(&mut self, offset: &super::super::Graphics::PointInt32) -> ::windows::core::Result<()>;
    fn ScrollRect(&mut self, offset: &super::super::Graphics::PointInt32, scrollrect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
    fn ScrollWithClip(&mut self, offset: &super::super::Graphics::PointInt32, cliprect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
    fn ScrollRectWithClip(&mut self, offset: &super::super::Graphics::PointInt32, cliprect: &super::super::Graphics::RectInt32, scrollrect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionDrawingSurface2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionDrawingSurface2";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ICompositionDrawingSurface2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurface2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurface2_Vtbl {
        unsafe extern "system" fn SizeInt32<Impl: ICompositionDrawingSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Resize<Impl: ICompositionDrawingSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(&*(&sizepixels as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Scroll<Impl: ICompositionDrawingSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scroll(&*(&offset as *const <super::super::Graphics::PointInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::PointInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollRect<Impl: ICompositionDrawingSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32, scrollrect: super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollRect(&*(&offset as *const <super::super::Graphics::PointInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::PointInt32 as ::windows::core::DefaultType>::DefaultType), &*(&scrollrect as *const <super::super::Graphics::RectInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollWithClip<Impl: ICompositionDrawingSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32, cliprect: super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollWithClip(&*(&offset as *const <super::super::Graphics::PointInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::PointInt32 as ::windows::core::DefaultType>::DefaultType), &*(&cliprect as *const <super::super::Graphics::RectInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScrollRectWithClip<Impl: ICompositionDrawingSurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: super::super::Graphics::PointInt32, cliprect: super::super::Graphics::RectInt32, scrollrect: super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
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
pub trait ICompositionDrawingSurfaceFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDrawingSurfaceFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionDrawingSurfaceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDrawingSurfaceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDrawingSurfaceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDrawingSurfaceFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionDrawingSurfaceFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDrawingSurfaceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEasingFunction_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEasingFunction_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEasingFunction, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEasingFunctionFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEasingFunctionFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEasingFunctionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEasingFunctionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEasingFunctionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEasingFunctionFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionEasingFunctionFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionEasingFunctionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionEasingFunctionStatics_Impl: Sized {
    fn CreateCubicBezierEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, controlpoint1: &super::super::Foundation::Numerics::Vector2, controlpoint2: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CubicBezierEasingFunction>;
    fn CreateLinearEasingFunction(&mut self, owner: &::core::option::Option<Compositor>) -> ::windows::core::Result<LinearEasingFunction>;
    fn CreateStepEasingFunction(&mut self, owner: &::core::option::Option<Compositor>) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateStepEasingFunctionWithStepCount(&mut self, owner: &::core::option::Option<Compositor>, stepcount: i32) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateBackEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, amplitude: f32) -> ::windows::core::Result<BackEasingFunction>;
    fn CreateBounceEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, bounces: i32, bounciness: f32) -> ::windows::core::Result<BounceEasingFunction>;
    fn CreateCircleEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode) -> ::windows::core::Result<CircleEasingFunction>;
    fn CreateElasticEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, oscillations: i32, springiness: f32) -> ::windows::core::Result<ElasticEasingFunction>;
    fn CreateExponentialEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, exponent: f32) -> ::windows::core::Result<ExponentialEasingFunction>;
    fn CreatePowerEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, power: f32) -> ::windows::core::Result<PowerEasingFunction>;
    fn CreateSineEasingFunction(&mut self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode) -> ::windows::core::Result<SineEasingFunction>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionEasingFunctionStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEasingFunctionStatics";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionEasingFunctionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEasingFunctionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEasingFunctionStatics_Vtbl {
        unsafe extern "system" fn CreateCubicBezierEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, controlpoint1: super::super::Foundation::Numerics::Vector2, controlpoint2: super::super::Foundation::Numerics::Vector2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateLinearEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateStepEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateStepEasingFunctionWithStepCount<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, stepcount: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateBackEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, amplitude: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateBounceEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, bounces: i32, bounciness: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateCircleEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateElasticEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, oscillations: i32, springiness: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateExponentialEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, exponent: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePowerEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, power: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSineEasingFunction<Impl: ICompositionEasingFunctionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, mode: CompositionEasingFunctionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionEffectBrush_Impl: Sized {
    fn GetSourceParameter(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<CompositionBrush>;
    fn SetSourceParameter(&mut self, name: &::windows::core::HSTRING, source: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectBrush_Vtbl {
        unsafe extern "system" fn GetSourceParameter<Impl: ICompositionEffectBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSourceParameter<Impl: ICompositionEffectBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionEffectFactory_Impl: Sized {
    fn CreateBrush(&mut self) -> ::windows::core::Result<CompositionEffectBrush>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn LoadStatus(&mut self) -> ::windows::core::Result<CompositionEffectFactoryLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectFactory_Vtbl {
        unsafe extern "system" fn CreateBrush<Impl: ICompositionEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: ICompositionEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LoadStatus<Impl: ICompositionEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEffectFactoryLoadStatus) -> ::windows::core::HRESULT {
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
pub trait ICompositionEffectSourceParameter_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectSourceParameter {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectSourceParameter";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectSourceParameter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectSourceParameter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectSourceParameter_Vtbl {
        unsafe extern "system" fn Name<Impl: ICompositionEffectSourceParameter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ICompositionEffectSourceParameterFactory_Impl: Sized {
    fn Create(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<CompositionEffectSourceParameter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionEffectSourceParameterFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEffectSourceParameterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionEffectSourceParameterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEffectSourceParameterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEffectSourceParameterFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ICompositionEffectSourceParameterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionEllipseGeometry_Impl: Sized {
    fn Center(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenter(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Radius(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetRadius(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionEllipseGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionEllipseGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionEllipseGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionEllipseGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionEllipseGeometry_Vtbl {
        unsafe extern "system" fn Center<Impl: ICompositionEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenter<Impl: ICompositionEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Radius<Impl: ICompositionEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRadius<Impl: ICompositionEllipseGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait ICompositionGeometricClip_Impl: Sized {
    fn Geometry(&mut self) -> ::windows::core::Result<CompositionGeometry>;
    fn SetGeometry(&mut self, value: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<()>;
    fn ViewBox(&mut self) -> ::windows::core::Result<CompositionViewBox>;
    fn SetViewBox(&mut self, value: &::core::option::Option<CompositionViewBox>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGeometricClip {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGeometricClip";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGeometricClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGeometricClip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGeometricClip_Vtbl {
        unsafe extern "system" fn Geometry<Impl: ICompositionGeometricClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGeometry<Impl: ICompositionGeometricClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometry(&*(&value as *const <CompositionGeometry as ::windows::core::Abi>::Abi as *const <CompositionGeometry as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ViewBox<Impl: ICompositionGeometricClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewBox<Impl: ICompositionGeometricClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionGeometry_Impl: Sized {
    fn TrimEnd(&mut self) -> ::windows::core::Result<f32>;
    fn SetTrimEnd(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn TrimOffset(&mut self) -> ::windows::core::Result<f32>;
    fn SetTrimOffset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn TrimStart(&mut self) -> ::windows::core::Result<f32>;
    fn SetTrimStart(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGeometry_Vtbl {
        unsafe extern "system" fn TrimEnd<Impl: ICompositionGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTrimEnd<Impl: ICompositionGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimEnd(value).into()
        }
        unsafe extern "system" fn TrimOffset<Impl: ICompositionGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTrimOffset<Impl: ICompositionGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimOffset(value).into()
        }
        unsafe extern "system" fn TrimStart<Impl: ICompositionGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTrimStart<Impl: ICompositionGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ICompositionGeometryFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGeometryFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGeometryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGeometryFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGeometryFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGeometryFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGeometryFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGeometryFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionGradientBrush_Impl: Sized {
    fn AnchorPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ColorStops(&mut self) -> ::windows::core::Result<CompositionColorGradientStopCollection>;
    fn ExtendMode(&mut self) -> ::windows::core::Result<CompositionGradientExtendMode>;
    fn SetExtendMode(&mut self, value: CompositionGradientExtendMode) -> ::windows::core::Result<()>;
    fn InterpolationSpace(&mut self) -> ::windows::core::Result<CompositionColorSpace>;
    fn SetInterpolationSpace(&mut self, value: CompositionColorSpace) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&mut self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGradientBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGradientBrush";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGradientBrush_Vtbl {
        unsafe extern "system" fn AnchorPoint<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAnchorPoint<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ColorStops<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendMode<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionGradientExtendMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExtendMode<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionGradientExtendMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendMode(value).into()
        }
        unsafe extern "system" fn InterpolationSpace<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionColorSpace) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInterpolationSpace<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionColorSpace) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterpolationSpace(value).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScale<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
pub trait ICompositionGradientBrush2_Impl: Sized {
    fn MappingMode(&mut self) -> ::windows::core::Result<CompositionMappingMode>;
    fn SetMappingMode(&mut self, value: CompositionMappingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGradientBrush2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGradientBrush2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGradientBrush2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGradientBrush2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGradientBrush2_Vtbl {
        unsafe extern "system" fn MappingMode<Impl: ICompositionGradientBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionMappingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMappingMode<Impl: ICompositionGradientBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionMappingMode) -> ::windows::core::HRESULT {
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
pub trait ICompositionGradientBrushFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionGradientBrushFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGradientBrushFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionGradientBrushFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGradientBrushFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGradientBrushFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionGradientBrushFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionGradientBrushFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait ICompositionGraphicsDevice_Impl: Sized {
    fn CreateDrawingSurface(&mut self, sizepixels: &super::super::Foundation::Size, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionDrawingSurface>;
    fn RenderingDeviceReplaced(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CompositionGraphicsDevice, RenderingDeviceReplacedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRenderingDeviceReplaced(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDevice_Vtbl {
        unsafe extern "system" fn CreateDrawingSurface<Impl: ICompositionGraphicsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Foundation::Size, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenderingDeviceReplaced<Impl: ICompositionGraphicsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRenderingDeviceReplaced<Impl: ICompositionGraphicsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait ICompositionGraphicsDevice2_Impl: Sized {
    fn CreateDrawingSurface2(&mut self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionDrawingSurface>;
    fn CreateVirtualDrawingSurface(&mut self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionVirtualDrawingSurface>;
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice2";
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDevice2_Vtbl {
        unsafe extern "system" fn CreateDrawingSurface2<Impl: ICompositionGraphicsDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateVirtualDrawingSurface<Impl: ICompositionGraphicsDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionGraphicsDevice3_Impl: Sized {
    fn CreateMipmapSurface(&mut self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionMipmapSurface>;
    fn Trim(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice3";
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDevice3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDevice3_Vtbl {
        unsafe extern "system" fn CreateMipmapSurface<Impl: ICompositionGraphicsDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Trim<Impl: ICompositionGraphicsDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ICompositionGraphicsDevice4_Impl: Sized {
    fn CaptureAsync(&mut self, capturevisual: &::core::option::Option<Visual>, size: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, sdrboost: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ICompositionSurface>>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionGraphicsDevice4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionGraphicsDevice4";
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionGraphicsDevice4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionGraphicsDevice4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionGraphicsDevice4_Vtbl {
        unsafe extern "system" fn CaptureAsync<Impl: ICompositionGraphicsDevice4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capturevisual: ::windows::core::RawPtr, size: super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, sdrboost: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionLight_Impl: Sized {
    fn Targets(&mut self) -> ::windows::core::Result<VisualUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLight {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLight";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLight_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLight_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLight_Vtbl {
        unsafe extern "system" fn Targets<Impl: ICompositionLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionLight2_Impl: Sized {
    fn ExclusionsFromTargets(&mut self) -> ::windows::core::Result<VisualUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLight2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLight2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLight2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLight2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLight2_Vtbl {
        unsafe extern "system" fn ExclusionsFromTargets<Impl: ICompositionLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionLight3_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLight3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLight3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLight3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLight3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLight3_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: ICompositionLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: ICompositionLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ICompositionLightFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionLightFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLightFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionLightFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLightFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLightFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionLightFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionLightFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionLineGeometry_Impl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetStart(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn End(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEnd(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionLineGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLineGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionLineGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLineGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLineGeometry_Vtbl {
        unsafe extern "system" fn Start<Impl: ICompositionLineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStart<Impl: ICompositionLineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStart(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn End<Impl: ICompositionLineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnd<Impl: ICompositionLineGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait ICompositionLinearGradientBrush_Impl: Sized {
    fn EndPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEndPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn StartPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetStartPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionLinearGradientBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionLinearGradientBrush";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionLinearGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionLinearGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionLinearGradientBrush_Vtbl {
        unsafe extern "system" fn EndPoint<Impl: ICompositionLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEndPoint<Impl: ICompositionLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPoint<Impl: ICompositionLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStartPoint<Impl: ICompositionLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait ICompositionMaskBrush_Impl: Sized {
    fn Mask(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetMask(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSource(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionMaskBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionMaskBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionMaskBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionMaskBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionMaskBrush_Vtbl {
        unsafe extern "system" fn Mask<Impl: ICompositionMaskBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMask<Impl: ICompositionMaskBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMask(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: ICompositionMaskBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: ICompositionMaskBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionMipmapSurface_Impl: Sized {
    fn LevelCount(&mut self) -> ::windows::core::Result<u32>;
    fn AlphaMode(&mut self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXAlphaMode>;
    fn PixelFormat(&mut self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SizeInt32(&mut self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn GetDrawingSurfaceForLevel(&mut self, level: u32) -> ::windows::core::Result<CompositionDrawingSurface>;
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionMipmapSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionMipmapSurface";
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ICompositionMipmapSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionMipmapSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionMipmapSurface_Vtbl {
        unsafe extern "system" fn LevelCount<Impl: ICompositionMipmapSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AlphaMode<Impl: ICompositionMipmapSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelFormat<Impl: ICompositionMipmapSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SizeInt32<Impl: ICompositionMipmapSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDrawingSurfaceForLevel<Impl: ICompositionMipmapSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionNineGridBrush_Impl: Sized {
    fn BottomInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetBottomInset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn BottomInsetScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetBottomInsetScale(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn IsCenterHollow(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCenterHollow(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn LeftInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetLeftInset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn LeftInsetScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetLeftInsetScale(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RightInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetRightInset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RightInsetScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetRightInsetScale(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSource(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn TopInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetTopInset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn TopInsetScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetTopInsetScale(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn SetInsets(&mut self, inset: f32) -> ::windows::core::Result<()>;
    fn SetInsetsWithValues(&mut self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<()>;
    fn SetInsetScales(&mut self, scale: f32) -> ::windows::core::Result<()>;
    fn SetInsetScalesWithValues(&mut self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionNineGridBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionNineGridBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionNineGridBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionNineGridBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionNineGridBrush_Vtbl {
        unsafe extern "system" fn BottomInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBottomInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomInset(value).into()
        }
        unsafe extern "system" fn BottomInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBottomInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomInsetScale(value).into()
        }
        unsafe extern "system" fn IsCenterHollow<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsCenterHollow<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCenterHollow(value).into()
        }
        unsafe extern "system" fn LeftInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLeftInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftInset(value).into()
        }
        unsafe extern "system" fn LeftInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLeftInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftInsetScale(value).into()
        }
        unsafe extern "system" fn RightInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRightInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightInset(value).into()
        }
        unsafe extern "system" fn RightInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRightInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightInsetScale(value).into()
        }
        unsafe extern "system" fn Source<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TopInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTopInset<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopInset(value).into()
        }
        unsafe extern "system" fn TopInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTopInsetScale<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopInsetScale(value).into()
        }
        unsafe extern "system" fn SetInsets<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInsets(inset).into()
        }
        unsafe extern "system" fn SetInsetsWithValues<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInsetsWithValues(left, top, right, bottom).into()
        }
        unsafe extern "system" fn SetInsetScales<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInsetScales(scale).into()
        }
        unsafe extern "system" fn SetInsetScalesWithValues<Impl: ICompositionNineGridBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::HRESULT {
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
pub trait ICompositionObject_Impl: Sized {
    fn Compositor(&mut self) -> ::windows::core::Result<Compositor>;
    fn Dispatcher(&mut self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
    fn Properties(&mut self) -> ::windows::core::Result<CompositionPropertySet>;
    fn StartAnimation(&mut self, propertyname: &::windows::core::HSTRING, animation: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn StopAnimation(&mut self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionObject {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ICompositionObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObject_Vtbl {
        unsafe extern "system" fn Compositor<Impl: ICompositionObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Dispatcher<Impl: ICompositionObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: ICompositionObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAnimation<Impl: ICompositionObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAnimation(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animation as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAnimation<Impl: ICompositionObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ICompositionObject2_Impl: Sized {
    fn Comment(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetComment(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ImplicitAnimations(&mut self) -> ::windows::core::Result<ImplicitAnimationCollection>;
    fn SetImplicitAnimations(&mut self, value: &::core::option::Option<ImplicitAnimationCollection>) -> ::windows::core::Result<()>;
    fn StartAnimationGroup(&mut self, value: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn StopAnimationGroup(&mut self, value: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObject2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObject2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObject2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObject2_Vtbl {
        unsafe extern "system" fn Comment<Impl: ICompositionObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetComment<Impl: ICompositionObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComment(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImplicitAnimations<Impl: ICompositionObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetImplicitAnimations<Impl: ICompositionObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImplicitAnimations(&*(&value as *const <ImplicitAnimationCollection as ::windows::core::Abi>::Abi as *const <ImplicitAnimationCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAnimationGroup<Impl: ICompositionObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAnimationGroup(&*(&value as *const <ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAnimationGroup<Impl: ICompositionObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionObject3_Impl: Sized {
    fn DispatcherQueue(&mut self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionObject3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ICompositionObject3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObject3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObject3_Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: ICompositionObject3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionObject4_Impl: Sized {
    fn TryGetAnimationController(&mut self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<AnimationController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObject4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObject4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObject4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObject4_Vtbl {
        unsafe extern "system" fn TryGetAnimationController<Impl: ICompositionObject4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionObjectFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObjectFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObjectFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObjectFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObjectFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionObjectFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionObjectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObjectStatics_Impl: Sized {
    fn StartAnimationWithIAnimationObject(&mut self, target: &::core::option::Option<IAnimationObject>, propertyname: &::windows::core::HSTRING, animation: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn StartAnimationGroupWithIAnimationObject(&mut self, target: &::core::option::Option<IAnimationObject>, animation: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionObjectStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObjectStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionObjectStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionObjectStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionObjectStatics_Vtbl {
        unsafe extern "system" fn StartAnimationWithIAnimationObject<Impl: ICompositionObjectStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartAnimationWithIAnimationObject(&*(&target as *const <IAnimationObject as ::windows::core::Abi>::Abi as *const <IAnimationObject as ::windows::core::DefaultType>::DefaultType), &*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animation as *const <CompositionAnimation as ::windows::core::Abi>::Abi as *const <CompositionAnimation as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn StartAnimationGroupWithIAnimationObject<Impl: ICompositionObjectStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionPath_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionPath {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPath";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPath_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPath_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionPath, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait ICompositionPathFactory_Impl: Sized {
    fn Create(&mut self, source: &::core::option::Option<super::super::Graphics::IGeometrySource2D>) -> ::windows::core::Result<CompositionPath>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionPathFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPathFactory";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ICompositionPathFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPathFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPathFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ICompositionPathFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionPathGeometry_Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<CompositionPath>;
    fn SetPath(&mut self, value: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionPathGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPathGeometry";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionPathGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPathGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPathGeometry_Vtbl {
        unsafe extern "system" fn Path<Impl: ICompositionPathGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPath<Impl: ICompositionPathGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionProjectedShadow_Impl: Sized {
    fn BlurRadiusMultiplier(&mut self) -> ::windows::core::Result<f32>;
    fn SetBlurRadiusMultiplier(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Casters(&mut self) -> ::windows::core::Result<CompositionProjectedShadowCasterCollection>;
    fn LightSource(&mut self) -> ::windows::core::Result<CompositionLight>;
    fn SetLightSource(&mut self, value: &::core::option::Option<CompositionLight>) -> ::windows::core::Result<()>;
    fn MaxBlurRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetMaxBlurRadius(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn MinBlurRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetMinBlurRadius(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Receivers(&mut self) -> ::windows::core::Result<CompositionProjectedShadowReceiverUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadow {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadow";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadow_Vtbl {
        unsafe extern "system" fn BlurRadiusMultiplier<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBlurRadiusMultiplier<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlurRadiusMultiplier(value).into()
        }
        unsafe extern "system" fn Casters<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LightSource<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLightSource<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLightSource(&*(&value as *const <CompositionLight as ::windows::core::Abi>::Abi as *const <CompositionLight as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxBlurRadius<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxBlurRadius<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBlurRadius(value).into()
        }
        unsafe extern "system" fn MinBlurRadius<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinBlurRadius<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinBlurRadius(value).into()
        }
        unsafe extern "system" fn Receivers<Impl: ICompositionProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionProjectedShadowCaster_Impl: Sized {
    fn Brush(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetBrush(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn CastingVisual(&mut self) -> ::windows::core::Result<Visual>;
    fn SetCastingVisual(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowCaster {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowCaster";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowCaster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowCaster_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowCaster_Vtbl {
        unsafe extern "system" fn Brush<Impl: ICompositionProjectedShadowCaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBrush<Impl: ICompositionProjectedShadowCaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrush(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CastingVisual<Impl: ICompositionProjectedShadowCaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCastingVisual<Impl: ICompositionProjectedShadowCaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionProjectedShadowCasterCollection_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn InsertAbove(&mut self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>, reference: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertAtBottom(&mut self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertAtTop(&mut self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertBelow(&mut self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>, reference: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, caster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowCasterCollection {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowCasterCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowCasterCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowCasterCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowCasterCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: ICompositionProjectedShadowCasterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InsertAbove<Impl: ICompositionProjectedShadowCasterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAbove(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType), &*(&reference as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtBottom<Impl: ICompositionProjectedShadowCasterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtBottom(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtTop<Impl: ICompositionProjectedShadowCasterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtTop(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertBelow<Impl: ICompositionProjectedShadowCasterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcaster: ::windows::core::RawPtr, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertBelow(&*(&newcaster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType), &*(&reference as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICompositionProjectedShadowCasterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, caster: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&caster as *const <CompositionProjectedShadowCaster as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowCaster as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionProjectedShadowCasterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ICompositionProjectedShadowCasterCollectionStatics_Impl: Sized {
    fn MaxRespectedCasters(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowCasterCollectionStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowCasterCollectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowCasterCollectionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowCasterCollectionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowCasterCollectionStatics_Vtbl {
        unsafe extern "system" fn MaxRespectedCasters<Impl: ICompositionProjectedShadowCasterCollectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait ICompositionProjectedShadowReceiver_Impl: Sized {
    fn ReceivingVisual(&mut self) -> ::windows::core::Result<Visual>;
    fn SetReceivingVisual(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowReceiver {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowReceiver";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowReceiver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowReceiver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowReceiver_Vtbl {
        unsafe extern "system" fn ReceivingVisual<Impl: ICompositionProjectedShadowReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReceivingVisual<Impl: ICompositionProjectedShadowReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionProjectedShadowReceiverUnorderedCollection_Impl: Sized {
    fn Add(&mut self, value: &::core::option::Option<CompositionProjectedShadowReceiver>) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Remove(&mut self, value: &::core::option::Option<CompositionProjectedShadowReceiver>) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionProjectedShadowReceiverUnorderedCollection {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionProjectedShadowReceiverUnorderedCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionProjectedShadowReceiverUnorderedCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionProjectedShadowReceiverUnorderedCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionProjectedShadowReceiverUnorderedCollection_Vtbl {
        unsafe extern "system" fn Add<Impl: ICompositionProjectedShadowReceiverUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&value as *const <CompositionProjectedShadowReceiver as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowReceiver as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Count<Impl: ICompositionProjectedShadowReceiverUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Remove<Impl: ICompositionProjectedShadowReceiverUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&value as *const <CompositionProjectedShadowReceiver as ::windows::core::Abi>::Abi as *const <CompositionProjectedShadowReceiver as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionProjectedShadowReceiverUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ICompositionPropertySet_Impl: Sized {
    fn InsertColor(&mut self, propertyname: &::windows::core::HSTRING, value: &super::Color) -> ::windows::core::Result<()>;
    fn InsertMatrix3x2(&mut self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn InsertMatrix4x4(&mut self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn InsertQuaternion(&mut self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn InsertScalar(&mut self, propertyname: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn InsertVector2(&mut self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn InsertVector3(&mut self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InsertVector4(&mut self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn TryGetColor(&mut self, propertyname: &::windows::core::HSTRING, value: &mut super::Color) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetMatrix3x2(&mut self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetMatrix4x4(&mut self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetQuaternion(&mut self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetScalar(&mut self, propertyname: &::windows::core::HSTRING, value: &mut f32) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector2(&mut self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector3(&mut self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector4(&mut self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<CompositionGetValueStatus>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionPropertySet {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPropertySet";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPropertySet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPropertySet_Vtbl {
        unsafe extern "system" fn InsertColor<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertColor(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertMatrix3x2<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertMatrix3x2(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix3x2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertMatrix4x4<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertMatrix4x4(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertQuaternion<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertQuaternion(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertScalar<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertScalar(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn InsertVector2<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertVector2(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertVector3<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertVector3(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertVector4<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertVector4(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetColor<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::Color, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetMatrix3x2<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Matrix3x2, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetMatrix4x4<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetQuaternion<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Quaternion, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetScalar<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut f32, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetVector2<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Vector2, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetVector3<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Vector3, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetVector4<Impl: ICompositionPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut super::super::Foundation::Numerics::Vector4, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
pub trait ICompositionPropertySet2_Impl: Sized {
    fn InsertBoolean(&mut self, propertyname: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn TryGetBoolean(&mut self, propertyname: &::windows::core::HSTRING, value: &mut bool) -> ::windows::core::Result<CompositionGetValueStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionPropertySet2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionPropertySet2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionPropertySet2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionPropertySet2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionPropertySet2_Vtbl {
        unsafe extern "system" fn InsertBoolean<Impl: ICompositionPropertySet2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertBoolean(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn TryGetBoolean<Impl: ICompositionPropertySet2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut bool, result__: *mut CompositionGetValueStatus) -> ::windows::core::HRESULT {
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
pub trait ICompositionRadialGradientBrush_Impl: Sized {
    fn EllipseCenter(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEllipseCenter(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn EllipseRadius(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEllipseRadius(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn GradientOriginOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetGradientOriginOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionRadialGradientBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionRadialGradientBrush";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionRadialGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionRadialGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionRadialGradientBrush_Vtbl {
        unsafe extern "system" fn EllipseCenter<Impl: ICompositionRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEllipseCenter<Impl: ICompositionRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEllipseCenter(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EllipseRadius<Impl: ICompositionRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEllipseRadius<Impl: ICompositionRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEllipseRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GradientOriginOffset<Impl: ICompositionRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGradientOriginOffset<Impl: ICompositionRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait ICompositionRectangleGeometry_Impl: Sized {
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionRectangleGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionRectangleGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionRectangleGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionRectangleGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionRectangleGeometry_Vtbl {
        unsafe extern "system" fn Offset<Impl: ICompositionRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: ICompositionRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSize<Impl: ICompositionRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait ICompositionRoundedRectangleGeometry_Impl: Sized {
    fn CornerRadius(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCornerRadius(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionRoundedRectangleGeometry {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionRoundedRectangleGeometry";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionRoundedRectangleGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionRoundedRectangleGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionRoundedRectangleGeometry_Vtbl {
        unsafe extern "system" fn CornerRadius<Impl: ICompositionRoundedRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCornerRadius<Impl: ICompositionRoundedRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCornerRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionRoundedRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionRoundedRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: ICompositionRoundedRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSize<Impl: ICompositionRoundedRectangleGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait ICompositionScopedBatch_Impl: Sized {
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnded(&mut self) -> ::windows::core::Result<bool>;
    fn End(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Suspend(&mut self) -> ::windows::core::Result<()>;
    fn Completed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionScopedBatch {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionScopedBatch";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositionScopedBatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionScopedBatch_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionScopedBatch_Vtbl {
        unsafe extern "system" fn IsActive<Impl: ICompositionScopedBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnded<Impl: ICompositionScopedBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn End<Impl: ICompositionScopedBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End().into()
        }
        unsafe extern "system" fn Resume<Impl: ICompositionScopedBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Suspend<Impl: ICompositionScopedBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Suspend().into()
        }
        unsafe extern "system" fn Completed<Impl: ICompositionScopedBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCompleted<Impl: ICompositionScopedBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait ICompositionShadow_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionShadow {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShadow";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionShadow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShadow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShadow_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionShadow, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionShadow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShadowFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionShadowFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShadowFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionShadowFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShadowFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShadowFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionShadowFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionShadowFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionShape_Impl: Sized {
    fn CenterPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&mut self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionShape {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShape";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionShape_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShape_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShape_Vtbl {
        unsafe extern "system" fn CenterPoint<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScale<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
pub trait ICompositionShapeFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionShapeFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionShapeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionShapeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionShapeFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionShapeFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionShapeFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionShapeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICompositionSpriteShape_Impl: Sized {
    fn FillBrush(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetFillBrush(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Geometry(&mut self) -> ::windows::core::Result<CompositionGeometry>;
    fn SetGeometry(&mut self, value: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<()>;
    fn IsStrokeNonScaling(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsStrokeNonScaling(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StrokeBrush(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetStrokeBrush(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn StrokeDashArray(&mut self) -> ::windows::core::Result<CompositionStrokeDashArray>;
    fn StrokeDashCap(&mut self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeDashCap(&mut self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeDashOffset(&mut self) -> ::windows::core::Result<f32>;
    fn SetStrokeDashOffset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn StrokeEndCap(&mut self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeEndCap(&mut self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeLineJoin(&mut self) -> ::windows::core::Result<CompositionStrokeLineJoin>;
    fn SetStrokeLineJoin(&mut self, value: CompositionStrokeLineJoin) -> ::windows::core::Result<()>;
    fn StrokeMiterLimit(&mut self) -> ::windows::core::Result<f32>;
    fn SetStrokeMiterLimit(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn StrokeStartCap(&mut self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeStartCap(&mut self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeThickness(&mut self) -> ::windows::core::Result<f32>;
    fn SetStrokeThickness(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionSpriteShape {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSpriteShape";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICompositionSpriteShape_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSpriteShape_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSpriteShape_Vtbl {
        unsafe extern "system" fn FillBrush<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFillBrush<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrush(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Geometry<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGeometry<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometry(&*(&value as *const <CompositionGeometry as ::windows::core::Abi>::Abi as *const <CompositionGeometry as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsStrokeNonScaling<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsStrokeNonScaling<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStrokeNonScaling(value).into()
        }
        unsafe extern "system" fn StrokeBrush<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeBrush<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeBrush(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StrokeDashArray<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StrokeDashCap<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeCap) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeDashCap<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashCap(value).into()
        }
        unsafe extern "system" fn StrokeDashOffset<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeDashOffset<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashOffset(value).into()
        }
        unsafe extern "system" fn StrokeEndCap<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeCap) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeEndCap<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeEndCap(value).into()
        }
        unsafe extern "system" fn StrokeLineJoin<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeLineJoin) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeLineJoin<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeLineJoin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeLineJoin(value).into()
        }
        unsafe extern "system" fn StrokeMiterLimit<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeMiterLimit<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeMiterLimit(value).into()
        }
        unsafe extern "system" fn StrokeStartCap<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStrokeCap) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeStartCap<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStrokeCap) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeStartCap(value).into()
        }
        unsafe extern "system" fn StrokeThickness<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStrokeThickness<Impl: ICompositionSpriteShape_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ICompositionSupportsSystemBackdrop_Impl: Sized {
    fn SystemBackdrop(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSystemBackdrop(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICompositionSupportsSystemBackdrop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSupportsSystemBackdrop";
}
impl ICompositionSupportsSystemBackdrop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSupportsSystemBackdrop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSupportsSystemBackdrop_Vtbl {
        unsafe extern "system" fn SystemBackdrop<Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSystemBackdrop<Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionSurface_Impl: Sized {}
impl ::windows::core::RuntimeName for ICompositionSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurface";
}
impl ICompositionSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurface_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurface, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionSurfaceBrush_Impl: Sized {
    fn BitmapInterpolationMode(&mut self) -> ::windows::core::Result<CompositionBitmapInterpolationMode>;
    fn SetBitmapInterpolationMode(&mut self, value: CompositionBitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn HorizontalAlignmentRatio(&mut self) -> ::windows::core::Result<f32>;
    fn SetHorizontalAlignmentRatio(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Stretch(&mut self) -> ::windows::core::Result<CompositionStretch>;
    fn SetStretch(&mut self, value: CompositionStretch) -> ::windows::core::Result<()>;
    fn Surface(&mut self) -> ::windows::core::Result<ICompositionSurface>;
    fn SetSurface(&mut self, value: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&mut self) -> ::windows::core::Result<f32>;
    fn SetVerticalAlignmentRatio(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionSurfaceBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionSurfaceBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceBrush_Vtbl {
        unsafe extern "system" fn BitmapInterpolationMode<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionBitmapInterpolationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBitmapInterpolationMode<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionBitmapInterpolationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapInterpolationMode(value).into()
        }
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHorizontalAlignmentRatio<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn Stretch<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStretch) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStretch<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStretch(value).into()
        }
        unsafe extern "system" fn Surface<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSurface<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurface(&*(&value as *const <ICompositionSurface as ::windows::core::Abi>::Abi as *const <ICompositionSurface as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVerticalAlignmentRatio<Impl: ICompositionSurfaceBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ICompositionSurfaceBrush2_Impl: Sized {
    fn AnchorPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&mut self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionSurfaceBrush2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceBrush2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionSurfaceBrush2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceBrush2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceBrush2_Vtbl {
        unsafe extern "system" fn AnchorPoint<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAnchorPoint<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenterPoint<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAngle<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngle<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn Scale<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScale<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransformMatrix<Impl: ICompositionSurfaceBrush2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT {
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
pub trait ICompositionSurfaceBrush3_Impl: Sized {
    fn SnapToPixels(&mut self) -> ::windows::core::Result<bool>;
    fn SetSnapToPixels(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionSurfaceBrush3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceBrush3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionSurfaceBrush3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceBrush3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceBrush3_Vtbl {
        unsafe extern "system" fn SnapToPixels<Impl: ICompositionSurfaceBrush3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSnapToPixels<Impl: ICompositionSurfaceBrush3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ICompositionSurfaceFacade_Impl: Sized {
    fn GetRealSurface(&mut self) -> ::windows::core::Result<ICompositionSurface>;
}
impl ::windows::core::RuntimeName for ICompositionSurfaceFacade {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceFacade";
}
impl ICompositionSurfaceFacade_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceFacade_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceFacade_Vtbl {
        unsafe extern "system" fn GetRealSurface<Impl: ICompositionSurfaceFacade_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionTarget_Impl: Sized {
    fn Root(&mut self) -> ::windows::core::Result<Visual>;
    fn SetRoot(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTarget {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTarget";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTarget_Vtbl {
        unsafe extern "system" fn Root<Impl: ICompositionTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRoot<Impl: ICompositionTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionTargetFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTargetFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTargetFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTargetFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTargetFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTargetFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTargetFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTargetFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTransform_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTransform {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTransform";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTransform_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTransform, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTransformFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionTransformFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTransformFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionTransformFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionTransformFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionTransformFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionTransformFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionTransformFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionViewBox_Impl: Sized {
    fn HorizontalAlignmentRatio(&mut self) -> ::windows::core::Result<f32>;
    fn SetHorizontalAlignmentRatio(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Stretch(&mut self) -> ::windows::core::Result<CompositionStretch>;
    fn SetStretch(&mut self, value: CompositionStretch) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&mut self) -> ::windows::core::Result<f32>;
    fn SetVerticalAlignmentRatio(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionViewBox {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionViewBox";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionViewBox_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionViewBox_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionViewBox_Vtbl {
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHorizontalAlignmentRatio<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn Offset<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSize<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stretch<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionStretch) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStretch<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStretch(value).into()
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVerticalAlignmentRatio<Impl: ICompositionViewBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ICompositionVirtualDrawingSurface_Impl: Sized {
    fn Trim(&mut self, rects: &[<super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionVirtualDrawingSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionVirtualDrawingSurface";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ICompositionVirtualDrawingSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionVirtualDrawingSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionVirtualDrawingSurface_Vtbl {
        unsafe extern "system" fn Trim<Impl: ICompositionVirtualDrawingSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rects_array_size: u32, rects: *const super::super::Graphics::RectInt32) -> ::windows::core::HRESULT {
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
pub trait ICompositionVirtualDrawingSurfaceFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionVirtualDrawingSurfaceFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionVirtualDrawingSurfaceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionVirtualDrawingSurfaceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionVirtualDrawingSurfaceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionVirtualDrawingSurfaceFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionVirtualDrawingSurfaceFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionVirtualDrawingSurfaceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICompositionVisualSurface_Impl: Sized {
    fn SourceVisual(&mut self) -> ::windows::core::Result<Visual>;
    fn SetSourceVisual(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn SourceOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSourceOffset(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn SourceSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSourceSize(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositionVisualSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionVisualSurface";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICompositionVisualSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionVisualSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionVisualSurface_Vtbl {
        unsafe extern "system" fn SourceVisual<Impl: ICompositionVisualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSourceVisual<Impl: ICompositionVisualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceVisual(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceOffset<Impl: ICompositionVisualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSourceOffset<Impl: ICompositionVisualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceSize<Impl: ICompositionVisualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSourceSize<Impl: ICompositionVisualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait ICompositor_Impl: Sized {
    fn CreateColorKeyFrameAnimation(&mut self) -> ::windows::core::Result<ColorKeyFrameAnimation>;
    fn CreateColorBrush(&mut self) -> ::windows::core::Result<CompositionColorBrush>;
    fn CreateColorBrushWithColor(&mut self, color: &super::Color) -> ::windows::core::Result<CompositionColorBrush>;
    fn CreateContainerVisual(&mut self) -> ::windows::core::Result<ContainerVisual>;
    fn CreateCubicBezierEasingFunction(&mut self, controlpoint1: &super::super::Foundation::Numerics::Vector2, controlpoint2: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CubicBezierEasingFunction>;
    fn CreateEffectFactory(&mut self, graphicseffect: &::core::option::Option<super::super::Graphics::Effects::IGraphicsEffect>) -> ::windows::core::Result<CompositionEffectFactory>;
    fn CreateEffectFactoryWithProperties(&mut self, graphicseffect: &::core::option::Option<super::super::Graphics::Effects::IGraphicsEffect>, animatableproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<CompositionEffectFactory>;
    fn CreateExpressionAnimation(&mut self) -> ::windows::core::Result<ExpressionAnimation>;
    fn CreateExpressionAnimationWithExpression(&mut self, expression: &::windows::core::HSTRING) -> ::windows::core::Result<ExpressionAnimation>;
    fn CreateInsetClip(&mut self) -> ::windows::core::Result<InsetClip>;
    fn CreateInsetClipWithInsets(&mut self, leftinset: f32, topinset: f32, rightinset: f32, bottominset: f32) -> ::windows::core::Result<InsetClip>;
    fn CreateLinearEasingFunction(&mut self) -> ::windows::core::Result<LinearEasingFunction>;
    fn CreatePropertySet(&mut self) -> ::windows::core::Result<CompositionPropertySet>;
    fn CreateQuaternionKeyFrameAnimation(&mut self) -> ::windows::core::Result<QuaternionKeyFrameAnimation>;
    fn CreateScalarKeyFrameAnimation(&mut self) -> ::windows::core::Result<ScalarKeyFrameAnimation>;
    fn CreateScopedBatch(&mut self, batchtype: CompositionBatchTypes) -> ::windows::core::Result<CompositionScopedBatch>;
    fn CreateSpriteVisual(&mut self) -> ::windows::core::Result<SpriteVisual>;
    fn CreateSurfaceBrush(&mut self) -> ::windows::core::Result<CompositionSurfaceBrush>;
    fn CreateSurfaceBrushWithSurface(&mut self, surface: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<CompositionSurfaceBrush>;
    fn CreateTargetForCurrentView(&mut self) -> ::windows::core::Result<CompositionTarget>;
    fn CreateVector2KeyFrameAnimation(&mut self) -> ::windows::core::Result<Vector2KeyFrameAnimation>;
    fn CreateVector3KeyFrameAnimation(&mut self) -> ::windows::core::Result<Vector3KeyFrameAnimation>;
    fn CreateVector4KeyFrameAnimation(&mut self) -> ::windows::core::Result<Vector4KeyFrameAnimation>;
    fn GetCommitBatch(&mut self, batchtype: CompositionBatchTypes) -> ::windows::core::Result<CompositionCommitBatch>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositor {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "Graphics_Effects", feature = "implement_exclusive"))]
impl ICompositor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor_Vtbl {
        unsafe extern "system" fn CreateColorKeyFrameAnimation<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateColorBrush<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateColorBrushWithColor<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: super::Color, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateContainerVisual<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateCubicBezierEasingFunction<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controlpoint1: super::super::Foundation::Numerics::Vector2, controlpoint2: super::super::Foundation::Numerics::Vector2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateEffectFactory<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graphicseffect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateEffectFactoryWithProperties<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graphicseffect: ::windows::core::RawPtr, animatableproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateExpressionAnimation<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateExpressionAnimationWithExpression<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInsetClip<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInsetClipWithInsets<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leftinset: f32, topinset: f32, rightinset: f32, bottominset: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateLinearEasingFunction<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePropertySet<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateQuaternionKeyFrameAnimation<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateScalarKeyFrameAnimation<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateScopedBatch<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, batchtype: CompositionBatchTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSpriteVisual<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSurfaceBrush<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSurfaceBrushWithSurface<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateTargetForCurrentView<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateVector2KeyFrameAnimation<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateVector3KeyFrameAnimation<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateVector4KeyFrameAnimation<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCommitBatch<Impl: ICompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, batchtype: CompositionBatchTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositor2_Impl: Sized {
    fn CreateAmbientLight(&mut self) -> ::windows::core::Result<AmbientLight>;
    fn CreateAnimationGroup(&mut self) -> ::windows::core::Result<CompositionAnimationGroup>;
    fn CreateBackdropBrush(&mut self) -> ::windows::core::Result<CompositionBackdropBrush>;
    fn CreateDistantLight(&mut self) -> ::windows::core::Result<DistantLight>;
    fn CreateDropShadow(&mut self) -> ::windows::core::Result<DropShadow>;
    fn CreateImplicitAnimationCollection(&mut self) -> ::windows::core::Result<ImplicitAnimationCollection>;
    fn CreateLayerVisual(&mut self) -> ::windows::core::Result<LayerVisual>;
    fn CreateMaskBrush(&mut self) -> ::windows::core::Result<CompositionMaskBrush>;
    fn CreateNineGridBrush(&mut self) -> ::windows::core::Result<CompositionNineGridBrush>;
    fn CreatePointLight(&mut self) -> ::windows::core::Result<PointLight>;
    fn CreateSpotLight(&mut self) -> ::windows::core::Result<SpotLight>;
    fn CreateStepEasingFunction(&mut self) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateStepEasingFunctionWithStepCount(&mut self, stepcount: i32) -> ::windows::core::Result<StepEasingFunction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor2";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor2_Vtbl {
        unsafe extern "system" fn CreateAmbientLight<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateAnimationGroup<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateBackdropBrush<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateDistantLight<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateDropShadow<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateImplicitAnimationCollection<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateLayerVisual<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateMaskBrush<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateNineGridBrush<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePointLight<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSpotLight<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateStepEasingFunction<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateStepEasingFunctionWithStepCount<Impl: ICompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepcount: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositor3_Impl: Sized {
    fn CreateHostBackdropBrush(&mut self) -> ::windows::core::Result<CompositionBackdropBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor3";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor3_Vtbl {
        unsafe extern "system" fn CreateHostBackdropBrush<Impl: ICompositor3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositor4_Impl: Sized {
    fn CreateColorGradientStop(&mut self) -> ::windows::core::Result<CompositionColorGradientStop>;
    fn CreateColorGradientStopWithOffsetAndColor(&mut self, offset: f32, color: &super::Color) -> ::windows::core::Result<CompositionColorGradientStop>;
    fn CreateLinearGradientBrush(&mut self) -> ::windows::core::Result<CompositionLinearGradientBrush>;
    fn CreateSpringScalarAnimation(&mut self) -> ::windows::core::Result<SpringScalarNaturalMotionAnimation>;
    fn CreateSpringVector2Animation(&mut self) -> ::windows::core::Result<SpringVector2NaturalMotionAnimation>;
    fn CreateSpringVector3Animation(&mut self) -> ::windows::core::Result<SpringVector3NaturalMotionAnimation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor4";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor4_Vtbl {
        unsafe extern "system" fn CreateColorGradientStop<Impl: ICompositor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateColorGradientStopWithOffsetAndColor<Impl: ICompositor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f32, color: super::Color, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: ICompositor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSpringScalarAnimation<Impl: ICompositor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSpringVector2Animation<Impl: ICompositor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSpringVector3Animation<Impl: ICompositor4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositor5_Impl: Sized {
    fn Comment(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetComment(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GlobalPlaybackRate(&mut self) -> ::windows::core::Result<f32>;
    fn SetGlobalPlaybackRate(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn CreateBounceScalarAnimation(&mut self) -> ::windows::core::Result<BounceScalarNaturalMotionAnimation>;
    fn CreateBounceVector2Animation(&mut self) -> ::windows::core::Result<BounceVector2NaturalMotionAnimation>;
    fn CreateBounceVector3Animation(&mut self) -> ::windows::core::Result<BounceVector3NaturalMotionAnimation>;
    fn CreateContainerShape(&mut self) -> ::windows::core::Result<CompositionContainerShape>;
    fn CreateEllipseGeometry(&mut self) -> ::windows::core::Result<CompositionEllipseGeometry>;
    fn CreateLineGeometry(&mut self) -> ::windows::core::Result<CompositionLineGeometry>;
    fn CreatePathGeometry(&mut self) -> ::windows::core::Result<CompositionPathGeometry>;
    fn CreatePathGeometryWithPath(&mut self, path: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<CompositionPathGeometry>;
    fn CreatePathKeyFrameAnimation(&mut self) -> ::windows::core::Result<PathKeyFrameAnimation>;
    fn CreateRectangleGeometry(&mut self) -> ::windows::core::Result<CompositionRectangleGeometry>;
    fn CreateRoundedRectangleGeometry(&mut self) -> ::windows::core::Result<CompositionRoundedRectangleGeometry>;
    fn CreateShapeVisual(&mut self) -> ::windows::core::Result<ShapeVisual>;
    fn CreateSpriteShape(&mut self) -> ::windows::core::Result<CompositionSpriteShape>;
    fn CreateSpriteShapeWithGeometry(&mut self, geometry: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<CompositionSpriteShape>;
    fn CreateViewBox(&mut self) -> ::windows::core::Result<CompositionViewBox>;
    fn RequestCommitAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositor5 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICompositor5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor5_Vtbl {
        unsafe extern "system" fn Comment<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetComment<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComment(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GlobalPlaybackRate<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGlobalPlaybackRate<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalPlaybackRate(value).into()
        }
        unsafe extern "system" fn CreateBounceScalarAnimation<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateBounceVector2Animation<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateBounceVector3Animation<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateContainerShape<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateEllipseGeometry<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateLineGeometry<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePathGeometry<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePathGeometryWithPath<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePathKeyFrameAnimation<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRectangleGeometry<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRoundedRectangleGeometry<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateShapeVisual<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSpriteShape<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSpriteShapeWithGeometry<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateViewBox<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestCommitAsync<Impl: ICompositor5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositor6_Impl: Sized {
    fn CreateGeometricClip(&mut self) -> ::windows::core::Result<CompositionGeometricClip>;
    fn CreateGeometricClipWithGeometry(&mut self, geometry: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<CompositionGeometricClip>;
    fn CreateRedirectVisual(&mut self) -> ::windows::core::Result<RedirectVisual>;
    fn CreateRedirectVisualWithSourceVisual(&mut self, source: &::core::option::Option<Visual>) -> ::windows::core::Result<RedirectVisual>;
    fn CreateBooleanKeyFrameAnimation(&mut self) -> ::windows::core::Result<BooleanKeyFrameAnimation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositor6 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor6";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositor6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor6_Vtbl {
        unsafe extern "system" fn CreateGeometricClip<Impl: ICompositor6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateGeometricClipWithGeometry<Impl: ICompositor6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRedirectVisual<Impl: ICompositor6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRedirectVisualWithSourceVisual<Impl: ICompositor6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateBooleanKeyFrameAnimation<Impl: ICompositor6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositor7_Impl: Sized {
    fn DispatcherQueue(&mut self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn CreateAnimationPropertyInfo(&mut self) -> ::windows::core::Result<AnimationPropertyInfo>;
    fn CreateRectangleClip(&mut self) -> ::windows::core::Result<RectangleClip>;
    fn CreateRectangleClipWithSides(&mut self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<RectangleClip>;
    fn CreateRectangleClipWithSidesAndRadius(&mut self, left: f32, top: f32, right: f32, bottom: f32, topleftradius: &super::super::Foundation::Numerics::Vector2, toprightradius: &super::super::Foundation::Numerics::Vector2, bottomrightradius: &super::super::Foundation::Numerics::Vector2, bottomleftradius: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<RectangleClip>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositor7 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositor7";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "System", feature = "implement_exclusive"))]
impl ICompositor7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositor7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositor7_Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: ICompositor7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateAnimationPropertyInfo<Impl: ICompositor7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRectangleClip<Impl: ICompositor7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRectangleClipWithSides<Impl: ICompositor7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRectangleClipWithSidesAndRadius<Impl: ICompositor7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, topleftradius: super::super::Foundation::Numerics::Vector2, toprightradius: super::super::Foundation::Numerics::Vector2, bottomrightradius: super::super::Foundation::Numerics::Vector2, bottomleftradius: super::super::Foundation::Numerics::Vector2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositorStatics_Impl: Sized {
    fn MaxGlobalPlaybackRate(&mut self) -> ::windows::core::Result<f32>;
    fn MinGlobalPlaybackRate(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorStatics_Vtbl {
        unsafe extern "system" fn MaxGlobalPlaybackRate<Impl: ICompositorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinGlobalPlaybackRate<Impl: ICompositorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait ICompositorWithBlurredWallpaperBackdropBrush_Impl: Sized {
    fn TryCreateBlurredWallpaperBackdropBrush(&mut self) -> ::windows::core::Result<CompositionBackdropBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithBlurredWallpaperBackdropBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithBlurredWallpaperBackdropBrush";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithBlurredWallpaperBackdropBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithBlurredWallpaperBackdropBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithBlurredWallpaperBackdropBrush_Vtbl {
        unsafe extern "system" fn TryCreateBlurredWallpaperBackdropBrush<Impl: ICompositorWithBlurredWallpaperBackdropBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositorWithProjectedShadow_Impl: Sized {
    fn CreateProjectedShadowCaster(&mut self) -> ::windows::core::Result<CompositionProjectedShadowCaster>;
    fn CreateProjectedShadow(&mut self) -> ::windows::core::Result<CompositionProjectedShadow>;
    fn CreateProjectedShadowReceiver(&mut self) -> ::windows::core::Result<CompositionProjectedShadowReceiver>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithProjectedShadow {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithProjectedShadow";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithProjectedShadow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithProjectedShadow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithProjectedShadow_Vtbl {
        unsafe extern "system" fn CreateProjectedShadowCaster<Impl: ICompositorWithProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateProjectedShadow<Impl: ICompositorWithProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateProjectedShadowReceiver<Impl: ICompositorWithProjectedShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositorWithRadialGradient_Impl: Sized {
    fn CreateRadialGradientBrush(&mut self) -> ::windows::core::Result<CompositionRadialGradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithRadialGradient {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithRadialGradient";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithRadialGradient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithRadialGradient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithRadialGradient_Vtbl {
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: ICompositorWithRadialGradient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositorWithVisualSurface_Impl: Sized {
    fn CreateVisualSurface(&mut self) -> ::windows::core::Result<CompositionVisualSurface>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositorWithVisualSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositorWithVisualSurface";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositorWithVisualSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositorWithVisualSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositorWithVisualSurface_Vtbl {
        unsafe extern "system" fn CreateVisualSurface<Impl: ICompositorWithVisualSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IContainerVisual_Impl: Sized {
    fn Children(&mut self) -> ::windows::core::Result<VisualCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContainerVisual {
    const NAME: &'static str = "Windows.UI.Composition.IContainerVisual";
}
#[cfg(feature = "implement_exclusive")]
impl IContainerVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContainerVisual_Vtbl {
        unsafe extern "system" fn Children<Impl: IContainerVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IContainerVisualFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContainerVisualFactory {
    const NAME: &'static str = "Windows.UI.Composition.IContainerVisualFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IContainerVisualFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerVisualFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContainerVisualFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContainerVisualFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContainerVisualFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ICubicBezierEasingFunction_Impl: Sized {
    fn ControlPoint1(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn ControlPoint2(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICubicBezierEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ICubicBezierEasingFunction";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ICubicBezierEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICubicBezierEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICubicBezierEasingFunction_Vtbl {
        unsafe extern "system" fn ControlPoint1<Impl: ICubicBezierEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ControlPoint2<Impl: ICubicBezierEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait IDelegatedInkTrailVisual_Impl: Sized {
    fn AddTrailPoints(&mut self, inkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn AddTrailPointsWithPrediction(&mut self, inkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType], predictedinkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn RemoveTrailPoints(&mut self, generationid: u32) -> ::windows::core::Result<()>;
    fn StartNewTrail(&mut self, color: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDelegatedInkTrailVisual {
    const NAME: &'static str = "Windows.UI.Composition.IDelegatedInkTrailVisual";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDelegatedInkTrailVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDelegatedInkTrailVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDelegatedInkTrailVisual_Vtbl {
        unsafe extern "system" fn AddTrailPoints<Impl: IDelegatedInkTrailVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkPoints_array_size: u32, inkpoints: *const InkTrailPoint, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddTrailPointsWithPrediction<Impl: IDelegatedInkTrailVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkPoints_array_size: u32, inkpoints: *const InkTrailPoint, predictedInkPoints_array_size: u32, predictedinkpoints: *const InkTrailPoint, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTrailPoints<Impl: IDelegatedInkTrailVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrailPoints(generationid).into()
        }
        unsafe extern "system" fn StartNewTrail<Impl: IDelegatedInkTrailVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: super::Color) -> ::windows::core::HRESULT {
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
pub trait IDelegatedInkTrailVisualStatics_Impl: Sized {
    fn Create(&mut self, compositor: &::core::option::Option<Compositor>) -> ::windows::core::Result<DelegatedInkTrailVisual>;
    fn CreateForSwapChain(&mut self, compositor: &::core::option::Option<Compositor>, swapchain: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<DelegatedInkTrailVisual>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDelegatedInkTrailVisualStatics {
    const NAME: &'static str = "Windows.UI.Composition.IDelegatedInkTrailVisualStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDelegatedInkTrailVisualStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDelegatedInkTrailVisualStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDelegatedInkTrailVisualStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IDelegatedInkTrailVisualStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateForSwapChain<Impl: IDelegatedInkTrailVisualStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, swapchain: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDistantLight_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&mut self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Direction(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDistantLight {
    const NAME: &'static str = "Windows.UI.Composition.IDistantLight";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDistantLight_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDistantLight_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDistantLight_Vtbl {
        unsafe extern "system" fn Color<Impl: IDistantLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IDistantLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CoordinateSpace<Impl: IDistantLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCoordinateSpace<Impl: IDistantLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoordinateSpace(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Direction<Impl: IDistantLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: IDistantLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
pub trait IDistantLight2_Impl: Sized {
    fn Intensity(&mut self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDistantLight2 {
    const NAME: &'static str = "Windows.UI.Composition.IDistantLight2";
}
#[cfg(feature = "implement_exclusive")]
impl IDistantLight2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDistantLight2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDistantLight2_Vtbl {
        unsafe extern "system" fn Intensity<Impl: IDistantLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIntensity<Impl: IDistantLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IDropShadow_Impl: Sized {
    fn BlurRadius(&mut self) -> ::windows::core::Result<f32>;
    fn SetBlurRadius(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Color(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Mask(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetMask(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Opacity(&mut self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDropShadow {
    const NAME: &'static str = "Windows.UI.Composition.IDropShadow";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IDropShadow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropShadow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropShadow_Vtbl {
        unsafe extern "system" fn BlurRadius<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBlurRadius<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlurRadius(value).into()
        }
        unsafe extern "system" fn Color<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Mask<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMask<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMask(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Offset<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Opacity<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpacity<Impl: IDropShadow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IDropShadow2_Impl: Sized {
    fn SourcePolicy(&mut self) -> ::windows::core::Result<CompositionDropShadowSourcePolicy>;
    fn SetSourcePolicy(&mut self, value: CompositionDropShadowSourcePolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropShadow2 {
    const NAME: &'static str = "Windows.UI.Composition.IDropShadow2";
}
#[cfg(feature = "implement_exclusive")]
impl IDropShadow2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropShadow2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropShadow2_Vtbl {
        unsafe extern "system" fn SourcePolicy<Impl: IDropShadow2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionDropShadowSourcePolicy) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSourcePolicy<Impl: IDropShadow2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionDropShadowSourcePolicy) -> ::windows::core::HRESULT {
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
pub trait IElasticEasingFunction_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Oscillations(&mut self) -> ::windows::core::Result<i32>;
    fn Springiness(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElasticEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IElasticEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IElasticEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElasticEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElasticEasingFunction_Vtbl {
        unsafe extern "system" fn Mode<Impl: IElasticEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Oscillations<Impl: IElasticEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Springiness<Impl: IElasticEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IExponentialEasingFunction_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Exponent(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExponentialEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IExponentialEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IExponentialEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExponentialEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExponentialEasingFunction_Vtbl {
        unsafe extern "system" fn Mode<Impl: IExponentialEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Exponent<Impl: IExponentialEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IExpressionAnimation_Impl: Sized {
    fn Expression(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExpression(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExpressionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IExpressionAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IExpressionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpressionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpressionAnimation_Vtbl {
        unsafe extern "system" fn Expression<Impl: IExpressionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpression<Impl: IExpressionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IImplicitAnimationCollection_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImplicitAnimationCollection {
    const NAME: &'static str = "Windows.UI.Composition.IImplicitAnimationCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IImplicitAnimationCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImplicitAnimationCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImplicitAnimationCollection_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImplicitAnimationCollection, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImplicitAnimationCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInsetClip_Impl: Sized {
    fn BottomInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetBottomInset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn LeftInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetLeftInset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RightInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetRightInset(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn TopInset(&mut self) -> ::windows::core::Result<f32>;
    fn SetTopInset(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInsetClip {
    const NAME: &'static str = "Windows.UI.Composition.IInsetClip";
}
#[cfg(feature = "implement_exclusive")]
impl IInsetClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInsetClip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInsetClip_Vtbl {
        unsafe extern "system" fn BottomInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBottomInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomInset(value).into()
        }
        unsafe extern "system" fn LeftInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLeftInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftInset(value).into()
        }
        unsafe extern "system" fn RightInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRightInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightInset(value).into()
        }
        unsafe extern "system" fn TopInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTopInset<Impl: IInsetClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IKeyFrameAnimation_Impl: Sized {
    fn DelayTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDelayTime(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IterationBehavior(&mut self) -> ::windows::core::Result<AnimationIterationBehavior>;
    fn SetIterationBehavior(&mut self, value: AnimationIterationBehavior) -> ::windows::core::Result<()>;
    fn IterationCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetIterationCount(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn KeyFrameCount(&mut self) -> ::windows::core::Result<i32>;
    fn StopBehavior(&mut self) -> ::windows::core::Result<AnimationStopBehavior>;
    fn SetStopBehavior(&mut self, value: AnimationStopBehavior) -> ::windows::core::Result<()>;
    fn InsertExpressionKeyFrame(&mut self, normalizedprogresskey: f32, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InsertExpressionKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, value: &::windows::core::HSTRING, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimation_Vtbl {
        unsafe extern "system" fn DelayTime<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDelayTime<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IterationBehavior<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationIterationBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIterationBehavior<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationIterationBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIterationBehavior(value).into()
        }
        unsafe extern "system" fn IterationCount<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIterationCount<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIterationCount(value).into()
        }
        unsafe extern "system" fn KeyFrameCount<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StopBehavior<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationStopBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStopBehavior<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationStopBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopBehavior(value).into()
        }
        unsafe extern "system" fn InsertExpressionKeyFrame<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertExpressionKeyFrame(normalizedprogresskey, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertExpressionKeyFrameWithEasingFunction<Impl: IKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IKeyFrameAnimation2_Impl: Sized {
    fn Direction(&mut self) -> ::windows::core::Result<AnimationDirection>;
    fn SetDirection(&mut self, value: AnimationDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyFrameAnimation2 {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimation2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyFrameAnimation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimation2_Vtbl {
        unsafe extern "system" fn Direction<Impl: IKeyFrameAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: IKeyFrameAnimation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationDirection) -> ::windows::core::HRESULT {
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
pub trait IKeyFrameAnimation3_Impl: Sized {
    fn DelayBehavior(&mut self) -> ::windows::core::Result<AnimationDelayBehavior>;
    fn SetDelayBehavior(&mut self, value: AnimationDelayBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyFrameAnimation3 {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimation3";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyFrameAnimation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimation3_Vtbl {
        unsafe extern "system" fn DelayBehavior<Impl: IKeyFrameAnimation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDelayBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDelayBehavior<Impl: IKeyFrameAnimation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationDelayBehavior) -> ::windows::core::HRESULT {
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
pub trait IKeyFrameAnimationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyFrameAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IKeyFrameAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyFrameAnimationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyFrameAnimationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyFrameAnimationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyFrameAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyFrameAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayerVisual_Impl: Sized {
    fn Effect(&mut self) -> ::windows::core::Result<CompositionEffectBrush>;
    fn SetEffect(&mut self, value: &::core::option::Option<CompositionEffectBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayerVisual {
    const NAME: &'static str = "Windows.UI.Composition.ILayerVisual";
}
#[cfg(feature = "implement_exclusive")]
impl ILayerVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayerVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayerVisual_Vtbl {
        unsafe extern "system" fn Effect<Impl: ILayerVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEffect<Impl: ILayerVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ILayerVisual2_Impl: Sized {
    fn Shadow(&mut self) -> ::windows::core::Result<CompositionShadow>;
    fn SetShadow(&mut self, value: &::core::option::Option<CompositionShadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayerVisual2 {
    const NAME: &'static str = "Windows.UI.Composition.ILayerVisual2";
}
#[cfg(feature = "implement_exclusive")]
impl ILayerVisual2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayerVisual2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayerVisual2_Vtbl {
        unsafe extern "system" fn Shadow<Impl: ILayerVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShadow<Impl: ILayerVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ILinearEasingFunction_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILinearEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ILinearEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ILinearEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILinearEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILinearEasingFunction_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILinearEasingFunction, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILinearEasingFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INaturalMotionAnimation_Impl: Sized {
    fn DelayBehavior(&mut self) -> ::windows::core::Result<AnimationDelayBehavior>;
    fn SetDelayBehavior(&mut self, value: AnimationDelayBehavior) -> ::windows::core::Result<()>;
    fn DelayTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDelayTime(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StopBehavior(&mut self) -> ::windows::core::Result<AnimationStopBehavior>;
    fn SetStopBehavior(&mut self, value: AnimationStopBehavior) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.INaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn DelayBehavior<Impl: INaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDelayBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDelayBehavior<Impl: INaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationDelayBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayBehavior(value).into()
        }
        unsafe extern "system" fn DelayTime<Impl: INaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDelayTime<Impl: INaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopBehavior<Impl: INaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationStopBehavior) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStopBehavior<Impl: INaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AnimationStopBehavior) -> ::windows::core::HRESULT {
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
pub trait INaturalMotionAnimationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.INaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INaturalMotionAnimationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INaturalMotionAnimationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INaturalMotionAnimationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathKeyFrameAnimation_Impl: Sized {
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, path: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, path: &::core::option::Option<CompositionPath>, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IPathKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IPathKeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathKeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathKeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IPathKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, path: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&path as *const <CompositionPath as ::windows::core::Abi>::Abi as *const <CompositionPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IPathKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, path: ::windows::core::RawPtr, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPointLight_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn ConstantAttenuation(&mut self) -> ::windows::core::Result<f32>;
    fn SetConstantAttenuation(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&mut self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn LinearAttenuation(&mut self) -> ::windows::core::Result<f32>;
    fn SetLinearAttenuation(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn QuadraticAttenuation(&mut self) -> ::windows::core::Result<f32>;
    fn SetQuadraticAttenuation(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointLight {
    const NAME: &'static str = "Windows.UI.Composition.IPointLight";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IPointLight_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointLight_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointLight_Vtbl {
        unsafe extern "system" fn Color<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConstantAttenuation<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetConstantAttenuation<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstantAttenuation(value).into()
        }
        unsafe extern "system" fn CoordinateSpace<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCoordinateSpace<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoordinateSpace(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LinearAttenuation<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLinearAttenuation<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinearAttenuation(value).into()
        }
        unsafe extern "system" fn Offset<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuadraticAttenuation<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetQuadraticAttenuation<Impl: IPointLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IPointLight2_Impl: Sized {
    fn Intensity(&mut self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointLight2 {
    const NAME: &'static str = "Windows.UI.Composition.IPointLight2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointLight2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointLight2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointLight2_Vtbl {
        unsafe extern "system" fn Intensity<Impl: IPointLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIntensity<Impl: IPointLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IPointLight3_Impl: Sized {
    fn MinAttenuationCutoff(&mut self) -> ::windows::core::Result<f32>;
    fn SetMinAttenuationCutoff(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn MaxAttenuationCutoff(&mut self) -> ::windows::core::Result<f32>;
    fn SetMaxAttenuationCutoff(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointLight3 {
    const NAME: &'static str = "Windows.UI.Composition.IPointLight3";
}
#[cfg(feature = "implement_exclusive")]
impl IPointLight3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointLight3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointLight3_Vtbl {
        unsafe extern "system" fn MinAttenuationCutoff<Impl: IPointLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinAttenuationCutoff<Impl: IPointLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinAttenuationCutoff(value).into()
        }
        unsafe extern "system" fn MaxAttenuationCutoff<Impl: IPointLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxAttenuationCutoff<Impl: IPointLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IPowerEasingFunction_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Power(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPowerEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IPowerEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IPowerEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPowerEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPowerEasingFunction_Vtbl {
        unsafe extern "system" fn Mode<Impl: IPowerEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Power<Impl: IPowerEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IQuaternionKeyFrameAnimation_Impl: Sized {
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Quaternion, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IQuaternionKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IQuaternionKeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IQuaternionKeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQuaternionKeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQuaternionKeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IQuaternionKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IQuaternionKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Quaternion, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRectangleClip_Impl: Sized {
    fn Bottom(&mut self) -> ::windows::core::Result<f32>;
    fn SetBottom(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn BottomLeftRadius(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetBottomLeftRadius(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn BottomRightRadius(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetBottomRightRadius(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Left(&mut self) -> ::windows::core::Result<f32>;
    fn SetLeft(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Right(&mut self) -> ::windows::core::Result<f32>;
    fn SetRight(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Top(&mut self) -> ::windows::core::Result<f32>;
    fn SetTop(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn TopLeftRadius(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetTopLeftRadius(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TopRightRadius(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetTopRightRadius(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRectangleClip {
    const NAME: &'static str = "Windows.UI.Composition.IRectangleClip";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IRectangleClip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectangleClip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectangleClip_Vtbl {
        unsafe extern "system" fn Bottom<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBottom<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottom(value).into()
        }
        unsafe extern "system" fn BottomLeftRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBottomLeftRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomLeftRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BottomRightRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBottomRightRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomRightRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Left<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLeft<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeft(value).into()
        }
        unsafe extern "system" fn Right<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRight<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRight(value).into()
        }
        unsafe extern "system" fn Top<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTop<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTop(value).into()
        }
        unsafe extern "system" fn TopLeftRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTopLeftRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopLeftRadius(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TopRightRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTopRightRadius<Impl: IRectangleClip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait IRedirectVisual_Impl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<Visual>;
    fn SetSource(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRedirectVisual {
    const NAME: &'static str = "Windows.UI.Composition.IRedirectVisual";
}
#[cfg(feature = "implement_exclusive")]
impl IRedirectVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRedirectVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRedirectVisual_Vtbl {
        unsafe extern "system" fn Source<Impl: IRedirectVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: IRedirectVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRenderingDeviceReplacedEventArgs_Impl: Sized {
    fn GraphicsDevice(&mut self) -> ::windows::core::Result<CompositionGraphicsDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRenderingDeviceReplacedEventArgs {
    const NAME: &'static str = "Windows.UI.Composition.IRenderingDeviceReplacedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRenderingDeviceReplacedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRenderingDeviceReplacedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRenderingDeviceReplacedEventArgs_Vtbl {
        unsafe extern "system" fn GraphicsDevice<Impl: IRenderingDeviceReplacedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IScalarKeyFrameAnimation_Impl: Sized {
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, value: f32) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, value: f32, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScalarKeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IScalarKeyFrameAnimation";
}
#[cfg(feature = "implement_exclusive")]
impl IScalarKeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarKeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarKeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IScalarKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, value).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IScalarKeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: f32, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IScalarNaturalMotionAnimation_Impl: Sized {
    fn FinalValue(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn SetFinalValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn InitialValue(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn SetInitialValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&mut self) -> ::windows::core::Result<f32>;
    fn SetInitialVelocity(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScalarNaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IScalarNaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScalarNaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarNaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarNaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn FinalValue<Impl: IScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFinalValue<Impl: IScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalValue(&*(&value as *const <super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialValue<Impl: IScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(&*(&value as *const <super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialVelocity<Impl: IScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialVelocity<Impl: IScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait IScalarNaturalMotionAnimationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScalarNaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IScalarNaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScalarNaturalMotionAnimationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarNaturalMotionAnimationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarNaturalMotionAnimationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScalarNaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScalarNaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IShapeVisual_Impl: Sized {
    fn Shapes(&mut self) -> ::windows::core::Result<CompositionShapeCollection>;
    fn ViewBox(&mut self) -> ::windows::core::Result<CompositionViewBox>;
    fn SetViewBox(&mut self, value: &::core::option::Option<CompositionViewBox>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShapeVisual {
    const NAME: &'static str = "Windows.UI.Composition.IShapeVisual";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IShapeVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShapeVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShapeVisual_Vtbl {
        unsafe extern "system" fn Shapes<Impl: IShapeVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewBox<Impl: IShapeVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewBox<Impl: IShapeVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISineEasingFunction_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISineEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.ISineEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl ISineEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISineEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISineEasingFunction_Vtbl {
        unsafe extern "system" fn Mode<Impl: ISineEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionEasingFunctionMode) -> ::windows::core::HRESULT {
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
pub trait ISpotLight_Impl: Sized {
    fn ConstantAttenuation(&mut self) -> ::windows::core::Result<f32>;
    fn SetConstantAttenuation(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&mut self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Direction(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InnerConeAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetInnerConeAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InnerConeAngleInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetInnerConeAngleInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn InnerConeColor(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetInnerConeColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn LinearAttenuation(&mut self) -> ::windows::core::Result<f32>;
    fn SetLinearAttenuation(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn OuterConeAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetOuterConeAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeAngleInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetOuterConeAngleInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeColor(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetOuterConeColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn QuadraticAttenuation(&mut self) -> ::windows::core::Result<f32>;
    fn SetQuadraticAttenuation(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpotLight {
    const NAME: &'static str = "Windows.UI.Composition.ISpotLight";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpotLight_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpotLight_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpotLight_Vtbl {
        unsafe extern "system" fn ConstantAttenuation<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetConstantAttenuation<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstantAttenuation(value).into()
        }
        unsafe extern "system" fn CoordinateSpace<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCoordinateSpace<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoordinateSpace(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Direction<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDirection<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InnerConeAngle<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInnerConeAngle<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeAngle(value).into()
        }
        unsafe extern "system" fn InnerConeAngleInDegrees<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInnerConeAngleInDegrees<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeAngleInDegrees(value).into()
        }
        unsafe extern "system" fn InnerConeColor<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInnerConeColor<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LinearAttenuation<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLinearAttenuation<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinearAttenuation(value).into()
        }
        unsafe extern "system" fn Offset<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OuterConeAngle<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOuterConeAngle<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuterConeAngle(value).into()
        }
        unsafe extern "system" fn OuterConeAngleInDegrees<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOuterConeAngleInDegrees<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuterConeAngleInDegrees(value).into()
        }
        unsafe extern "system" fn OuterConeColor<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOuterConeColor<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuterConeColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuadraticAttenuation<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetQuadraticAttenuation<Impl: ISpotLight_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ISpotLight2_Impl: Sized {
    fn InnerConeIntensity(&mut self) -> ::windows::core::Result<f32>;
    fn SetInnerConeIntensity(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeIntensity(&mut self) -> ::windows::core::Result<f32>;
    fn SetOuterConeIntensity(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpotLight2 {
    const NAME: &'static str = "Windows.UI.Composition.ISpotLight2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpotLight2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpotLight2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpotLight2_Vtbl {
        unsafe extern "system" fn InnerConeIntensity<Impl: ISpotLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInnerConeIntensity<Impl: ISpotLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInnerConeIntensity(value).into()
        }
        unsafe extern "system" fn OuterConeIntensity<Impl: ISpotLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOuterConeIntensity<Impl: ISpotLight2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ISpotLight3_Impl: Sized {
    fn MinAttenuationCutoff(&mut self) -> ::windows::core::Result<f32>;
    fn SetMinAttenuationCutoff(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn MaxAttenuationCutoff(&mut self) -> ::windows::core::Result<f32>;
    fn SetMaxAttenuationCutoff(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpotLight3 {
    const NAME: &'static str = "Windows.UI.Composition.ISpotLight3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpotLight3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpotLight3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpotLight3_Vtbl {
        unsafe extern "system" fn MinAttenuationCutoff<Impl: ISpotLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinAttenuationCutoff<Impl: ISpotLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinAttenuationCutoff(value).into()
        }
        unsafe extern "system" fn MaxAttenuationCutoff<Impl: ISpotLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxAttenuationCutoff<Impl: ISpotLight3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
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
pub trait ISpringScalarNaturalMotionAnimation_Impl: Sized {
    fn DampingRatio(&mut self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpringScalarNaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ISpringScalarNaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpringScalarNaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpringScalarNaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpringScalarNaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn DampingRatio<Impl: ISpringScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDampingRatio<Impl: ISpringScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDampingRatio(value).into()
        }
        unsafe extern "system" fn Period<Impl: ISpringScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPeriod<Impl: ISpringScalarNaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
pub trait ISpringVector2NaturalMotionAnimation_Impl: Sized {
    fn DampingRatio(&mut self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpringVector2NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ISpringVector2NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpringVector2NaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpringVector2NaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpringVector2NaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn DampingRatio<Impl: ISpringVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDampingRatio<Impl: ISpringVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDampingRatio(value).into()
        }
        unsafe extern "system" fn Period<Impl: ISpringVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPeriod<Impl: ISpringVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
pub trait ISpringVector3NaturalMotionAnimation_Impl: Sized {
    fn DampingRatio(&mut self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpringVector3NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.ISpringVector3NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpringVector3NaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpringVector3NaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpringVector3NaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn DampingRatio<Impl: ISpringVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDampingRatio<Impl: ISpringVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDampingRatio(value).into()
        }
        unsafe extern "system" fn Period<Impl: ISpringVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPeriod<Impl: ISpringVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
pub trait ISpriteVisual_Impl: Sized {
    fn Brush(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetBrush(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpriteVisual {
    const NAME: &'static str = "Windows.UI.Composition.ISpriteVisual";
}
#[cfg(feature = "implement_exclusive")]
impl ISpriteVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpriteVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpriteVisual_Vtbl {
        unsafe extern "system" fn Brush<Impl: ISpriteVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBrush<Impl: ISpriteVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISpriteVisual2_Impl: Sized {
    fn Shadow(&mut self) -> ::windows::core::Result<CompositionShadow>;
    fn SetShadow(&mut self, value: &::core::option::Option<CompositionShadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpriteVisual2 {
    const NAME: &'static str = "Windows.UI.Composition.ISpriteVisual2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpriteVisual2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpriteVisual2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpriteVisual2_Vtbl {
        unsafe extern "system" fn Shadow<Impl: ISpriteVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShadow<Impl: ISpriteVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IStepEasingFunction_Impl: Sized {
    fn FinalStep(&mut self) -> ::windows::core::Result<i32>;
    fn SetFinalStep(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn InitialStep(&mut self) -> ::windows::core::Result<i32>;
    fn SetInitialStep(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn IsFinalStepSingleFrame(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsFinalStepSingleFrame(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsInitialStepSingleFrame(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsInitialStepSingleFrame(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StepCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetStepCount(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStepEasingFunction {
    const NAME: &'static str = "Windows.UI.Composition.IStepEasingFunction";
}
#[cfg(feature = "implement_exclusive")]
impl IStepEasingFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStepEasingFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStepEasingFunction_Vtbl {
        unsafe extern "system" fn FinalStep<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFinalStep<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalStep(value).into()
        }
        unsafe extern "system" fn InitialStep<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialStep<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialStep(value).into()
        }
        unsafe extern "system" fn IsFinalStepSingleFrame<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsFinalStepSingleFrame<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFinalStepSingleFrame(value).into()
        }
        unsafe extern "system" fn IsInitialStepSingleFrame<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsInitialStepSingleFrame<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInitialStepSingleFrame(value).into()
        }
        unsafe extern "system" fn StepCount<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStepCount<Impl: IStepEasingFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
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
pub trait IVector2KeyFrameAnimation_Impl: Sized {
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector2, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector2KeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector2KeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector2KeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector2KeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector2KeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IVector2KeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IVector2KeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector2, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVector2NaturalMotionAnimation_Impl: Sized {
    fn FinalValue(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>;
    fn SetFinalValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>) -> ::windows::core::Result<()>;
    fn InitialValue(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>;
    fn SetInitialValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetInitialVelocity(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector2NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector2NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector2NaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector2NaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector2NaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn FinalValue<Impl: IVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFinalValue<Impl: IVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialValue<Impl: IVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialVelocity<Impl: IVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialVelocity<Impl: IVector2NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait IVector2NaturalMotionAnimationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVector2NaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IVector2NaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVector2NaturalMotionAnimationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector2NaturalMotionAnimationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector2NaturalMotionAnimationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVector2NaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector2NaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVector3KeyFrameAnimation_Impl: Sized {
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector3, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector3KeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector3KeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector3KeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3KeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3KeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IVector3KeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IVector3KeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector3, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVector3NaturalMotionAnimation_Impl: Sized {
    fn FinalValue(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>;
    fn SetFinalValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn InitialValue(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>;
    fn SetInitialValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetInitialVelocity(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector3NaturalMotionAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector3NaturalMotionAnimation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector3NaturalMotionAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3NaturalMotionAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3NaturalMotionAnimation_Vtbl {
        unsafe extern "system" fn FinalValue<Impl: IVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFinalValue<Impl: IVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFinalValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialValue<Impl: IVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialVelocity<Impl: IVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialVelocity<Impl: IVector3NaturalMotionAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
pub trait IVector3NaturalMotionAnimationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVector3NaturalMotionAnimationFactory {
    const NAME: &'static str = "Windows.UI.Composition.IVector3NaturalMotionAnimationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVector3NaturalMotionAnimationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3NaturalMotionAnimationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3NaturalMotionAnimationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVector3NaturalMotionAnimationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector3NaturalMotionAnimationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IVector4KeyFrameAnimation_Impl: Sized {
    fn InsertKeyFrame(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&mut self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector4, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector4KeyFrameAnimation {
    const NAME: &'static str = "Windows.UI.Composition.IVector4KeyFrameAnimation";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVector4KeyFrameAnimation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector4KeyFrameAnimation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector4KeyFrameAnimation_Vtbl {
        unsafe extern "system" fn InsertKeyFrame<Impl: IVector4KeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertKeyFrame(normalizedprogresskey, &*(&value as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertKeyFrameWithEasingFunction<Impl: IVector4KeyFrameAnimation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, normalizedprogresskey: f32, value: super::super::Foundation::Numerics::Vector4, easingfunction: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVisual_Impl: Sized {
    fn AnchorPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn BackfaceVisibility(&mut self) -> ::windows::core::Result<CompositionBackfaceVisibility>;
    fn SetBackfaceVisibility(&mut self, value: CompositionBackfaceVisibility) -> ::windows::core::Result<()>;
    fn BorderMode(&mut self) -> ::windows::core::Result<CompositionBorderMode>;
    fn SetBorderMode(&mut self, value: CompositionBorderMode) -> ::windows::core::Result<()>;
    fn CenterPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetCenterPoint(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Clip(&mut self) -> ::windows::core::Result<CompositionClip>;
    fn SetClip(&mut self, value: &::core::option::Option<CompositionClip>) -> ::windows::core::Result<()>;
    fn CompositeMode(&mut self) -> ::windows::core::Result<CompositionCompositeMode>;
    fn SetCompositeMode(&mut self, value: CompositionCompositeMode) -> ::windows::core::Result<()>;
    fn IsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Opacity(&mut self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Orientation(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetOrientation(&mut self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn Parent(&mut self) -> ::windows::core::Result<ContainerVisual>;
    fn RotationAngle(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAxis(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetTransformMatrix(&mut self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisual {
    const NAME: &'static str = "Windows.UI.Composition.IVisual";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisual_Vtbl {
        unsafe extern "system" fn AnchorPoint<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAnchorPoint<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnchorPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackfaceVisibility<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionBackfaceVisibility) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackfaceVisibility<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionBackfaceVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackfaceVisibility(value).into()
        }
        unsafe extern "system" fn BorderMode<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionBorderMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBorderMode<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionBorderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderMode(value).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenterPoint<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clip<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClip<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClip(&*(&value as *const <CompositionClip as ::windows::core::Abi>::Abi as *const <CompositionClip as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompositeMode<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CompositionCompositeMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCompositeMode<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CompositionCompositeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompositeMode(value).into()
        }
        unsafe extern "system" fn IsVisible<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsVisible<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVisible(value).into()
        }
        unsafe extern "system" fn Offset<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Opacity<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOpacity<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn Orientation<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOrientation<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RotationAngle<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngle<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngle(value).into()
        }
        unsafe extern "system" fn RotationAngleInDegrees<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAngleInDegrees<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAngleInDegrees(value).into()
        }
        unsafe extern "system" fn RotationAxis<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationAxis<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAxis(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Scale<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScale<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Size<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSize<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransformMatrix<Impl: IVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
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
pub trait IVisual2_Impl: Sized {
    fn ParentForTransform(&mut self) -> ::windows::core::Result<Visual>;
    fn SetParentForTransform(&mut self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RelativeOffsetAdjustment(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRelativeOffsetAdjustment(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RelativeSizeAdjustment(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetRelativeSizeAdjustment(&mut self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisual2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisual2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVisual2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisual2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisual2_Vtbl {
        unsafe extern "system" fn ParentForTransform<Impl: IVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetParentForTransform<Impl: IVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentForTransform(&*(&value as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeOffsetAdjustment<Impl: IVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRelativeOffsetAdjustment<Impl: IVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeOffsetAdjustment(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeSizeAdjustment<Impl: IVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRelativeSizeAdjustment<Impl: IVisual2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
pub trait IVisual3_Impl: Sized {
    fn IsHitTestVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsHitTestVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisual3 {
    const NAME: &'static str = "Windows.UI.Composition.IVisual3";
}
#[cfg(feature = "implement_exclusive")]
impl IVisual3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisual3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisual3_Vtbl {
        unsafe extern "system" fn IsHitTestVisible<Impl: IVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsHitTestVisible<Impl: IVisual3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IVisual4_Impl: Sized {
    fn IsPixelSnappingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPixelSnappingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisual4 {
    const NAME: &'static str = "Windows.UI.Composition.IVisual4";
}
#[cfg(feature = "implement_exclusive")]
impl IVisual4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisual4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisual4_Vtbl {
        unsafe extern "system" fn IsPixelSnappingEnabled<Impl: IVisual4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPixelSnappingEnabled<Impl: IVisual4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IVisualCollection_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn InsertAbove(&mut self, newchild: &::core::option::Option<Visual>, sibling: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertAtBottom(&mut self, newchild: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertAtTop(&mut self, newchild: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertBelow(&mut self, newchild: &::core::option::Option<Visual>, sibling: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, child: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualCollection {
    const NAME: &'static str = "Windows.UI.Composition.IVisualCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InsertAbove<Impl: IVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, sibling: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAbove(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType), &*(&sibling as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtBottom<Impl: IVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtBottom(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAtTop<Impl: IVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAtTop(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertBelow<Impl: IVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, sibling: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertBelow(&*(&newchild as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType), &*(&sibling as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&child as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IVisualElement_Impl: Sized {}
impl ::windows::core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement";
}
impl IVisualElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElement_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElement, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElement as ::windows::core::Interface>::IID
    }
}
pub trait IVisualElement2_Impl: Sized {
    fn GetVisualInternal(&mut self) -> ::windows::core::Result<Visual>;
}
impl ::windows::core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement2";
}
impl IVisualElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElement2_Vtbl {
        unsafe extern "system" fn GetVisualInternal<Impl: IVisualElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVisualFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualFactory {
    const NAME: &'static str = "Windows.UI.Composition.IVisualFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualUnorderedCollection_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, newvisual: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, visual: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualUnorderedCollection {
    const NAME: &'static str = "Windows.UI.Composition.IVisualUnorderedCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualUnorderedCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualUnorderedCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualUnorderedCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IVisualUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Add<Impl: IVisualUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newvisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&newvisual as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IVisualUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&visual as *const <Visual as ::windows::core::Abi>::Abi as *const <Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IVisualUnorderedCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
