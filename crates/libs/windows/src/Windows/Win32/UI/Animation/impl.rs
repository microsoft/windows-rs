pub trait IUIAnimationInterpolatorImpl: Sized {
    fn SetInitialValueAndVelocity();
    fn SetDuration();
    fn GetDuration();
    fn GetFinalValue();
    fn InterpolateValue();
    fn InterpolateVelocity();
    fn GetDependencies();
}
impl ::windows::core::RuntimeName for IUIAnimationInterpolator {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationInterpolator";
}
impl IUIAnimationInterpolatorVtbl {
    pub const fn new<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationInterpolatorVtbl {
        unsafe extern "system" fn SetInitialValueAndVelocity<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialValueAndVelocity(initialvalue, initialvelocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDuration(duration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDuration(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalValue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateValue<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterpolateValue(offset, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateVelocity<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterpolateVelocity(offset, ::core::mem::transmute_copy(&velocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Impl: IUIAnimationInterpolatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDependencies(::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationInterpolator>, base.5, SetInitialValueAndVelocity::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, GetDuration::<Impl, OFFSET>, GetFinalValue::<Impl, OFFSET>, InterpolateValue::<Impl, OFFSET>, InterpolateVelocity::<Impl, OFFSET>, GetDependencies::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationInterpolator2Impl: Sized {
    fn GetDimension();
    fn SetInitialValueAndVelocity();
    fn SetDuration();
    fn GetDuration();
    fn GetFinalValue();
    fn InterpolateValue();
    fn InterpolateVelocity();
    fn GetPrimitiveInterpolation();
    fn GetDependencies();
}
impl ::windows::core::RuntimeName for IUIAnimationInterpolator2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationInterpolator2";
}
impl IUIAnimationInterpolator2Vtbl {
    pub const fn new<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationInterpolator2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDimension(::core::mem::transmute_copy(&dimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialValueAndVelocity(initialvalue, initialvelocity, cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDuration(duration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDuration(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalValue(::core::mem::transmute_copy(&value), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateValue<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterpolateValue(offset, ::core::mem::transmute_copy(&value), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateVelocity<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterpolateVelocity(offset, ::core::mem::transmute_copy(&velocity), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolation: ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrimitiveInterpolation(&*(&interpolation as *const <IUIAnimationPrimitiveInterpolation as ::windows::core::Abi>::Abi as *const <IUIAnimationPrimitiveInterpolation as ::windows::core::DefaultType>::DefaultType), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Impl: IUIAnimationInterpolator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDependencies(::core::mem::transmute_copy(&initialvaluedependencies), ::core::mem::transmute_copy(&initialvelocitydependencies), ::core::mem::transmute_copy(&durationdependencies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationInterpolator2>, base.5, GetDimension::<Impl, OFFSET>, SetInitialValueAndVelocity::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, GetDuration::<Impl, OFFSET>, GetFinalValue::<Impl, OFFSET>, InterpolateValue::<Impl, OFFSET>, InterpolateVelocity::<Impl, OFFSET>, GetPrimitiveInterpolation::<Impl, OFFSET>, GetDependencies::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationLoopIterationChangeHandler2Impl: Sized {
    fn OnLoopIterationChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationLoopIterationChangeHandler2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationLoopIterationChangeHandler2";
}
impl IUIAnimationLoopIterationChangeHandler2Vtbl {
    pub const fn new<Impl: IUIAnimationLoopIterationChangeHandler2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationLoopIterationChangeHandler2Vtbl {
        unsafe extern "system" fn OnLoopIterationChanged<Impl: IUIAnimationLoopIterationChangeHandler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnLoopIterationChanged(&*(&storyboard as *const <IUIAnimationStoryboard2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard2 as ::windows::core::DefaultType>::DefaultType), id, newiterationcount, olditerationcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationLoopIterationChangeHandler2>, base.5, OnLoopIterationChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationManagerImpl: Sized {
    fn CreateAnimationVariable();
    fn ScheduleTransition();
    fn CreateStoryboard();
    fn FinishAllStoryboards();
    fn AbandonAllStoryboards();
    fn Update();
    fn GetVariableFromTag();
    fn GetStoryboardFromTag();
    fn GetStatus();
    fn SetAnimationMode();
    fn Pause();
    fn Resume();
    fn SetManagerEventHandler();
    fn SetCancelPriorityComparison();
    fn SetTrimPriorityComparison();
    fn SetCompressPriorityComparison();
    fn SetConcludePriorityComparison();
    fn SetDefaultLongestAcceptableDelay();
    fn Shutdown();
}
impl ::windows::core::RuntimeName for IUIAnimationManager {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationManager";
}
impl IUIAnimationManagerVtbl {
    pub const fn new<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationManagerVtbl {
        unsafe extern "system" fn CreateAnimationVariable<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAnimationVariable(initialvalue, ::core::mem::transmute_copy(&variable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScheduleTransition(&*(&variable as *const <IUIAnimationVariable as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable as ::windows::core::DefaultType>::DefaultType), &*(&transition as *const <IUIAnimationTransition as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition as ::windows::core::DefaultType>::DefaultType), timenow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryboard<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStoryboard(::core::mem::transmute_copy(&storyboard)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAllStoryboards(completiondeadline) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbandonAllStoryboards<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbandonAllStoryboards() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Update(timenow, ::core::mem::transmute_copy(&updateresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableFromTag<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVariableFromTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id, ::core::mem::transmute_copy(&variable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStoryboardFromTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id, ::core::mem::transmute_copy(&storyboard)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAnimationMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManagerEventHandler<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetManagerEventHandler(&*(&handler as *const <IUIAnimationManagerEventHandler as ::windows::core::Abi>::Abi as *const <IUIAnimationManagerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCancelPriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTrimPriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCompressPriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConcludePriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultLongestAcceptableDelay(delay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IUIAnimationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationManager>,
            base.5,
            CreateAnimationVariable::<Impl, OFFSET>,
            ScheduleTransition::<Impl, OFFSET>,
            CreateStoryboard::<Impl, OFFSET>,
            FinishAllStoryboards::<Impl, OFFSET>,
            AbandonAllStoryboards::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            GetVariableFromTag::<Impl, OFFSET>,
            GetStoryboardFromTag::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            SetAnimationMode::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            SetManagerEventHandler::<Impl, OFFSET>,
            SetCancelPriorityComparison::<Impl, OFFSET>,
            SetTrimPriorityComparison::<Impl, OFFSET>,
            SetCompressPriorityComparison::<Impl, OFFSET>,
            SetConcludePriorityComparison::<Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay::<Impl, OFFSET>,
            Shutdown::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationManager2Impl: Sized {
    fn CreateAnimationVectorVariable();
    fn CreateAnimationVariable();
    fn ScheduleTransition();
    fn CreateStoryboard();
    fn FinishAllStoryboards();
    fn AbandonAllStoryboards();
    fn Update();
    fn GetVariableFromTag();
    fn GetStoryboardFromTag();
    fn EstimateNextEventTime();
    fn GetStatus();
    fn SetAnimationMode();
    fn Pause();
    fn Resume();
    fn SetManagerEventHandler();
    fn SetCancelPriorityComparison();
    fn SetTrimPriorityComparison();
    fn SetCompressPriorityComparison();
    fn SetConcludePriorityComparison();
    fn SetDefaultLongestAcceptableDelay();
    fn Shutdown();
}
impl ::windows::core::RuntimeName for IUIAnimationManager2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationManager2";
}
impl IUIAnimationManager2Vtbl {
    pub const fn new<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationManager2Vtbl {
        unsafe extern "system" fn CreateAnimationVectorVariable<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAnimationVectorVariable(initialvalue, cdimension, ::core::mem::transmute_copy(&variable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationVariable<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAnimationVariable(initialvalue, ::core::mem::transmute_copy(&variable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScheduleTransition(&*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType), &*(&transition as *const <IUIAnimationTransition2 as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition2 as ::windows::core::DefaultType>::DefaultType), timenow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryboard<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStoryboard(::core::mem::transmute_copy(&storyboard)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAllStoryboards(completiondeadline) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbandonAllStoryboards<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbandonAllStoryboards() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Update(timenow, ::core::mem::transmute_copy(&updateresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableFromTag<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVariableFromTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id, ::core::mem::transmute_copy(&variable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStoryboardFromTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id, ::core::mem::transmute_copy(&storyboard)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimateNextEventTime<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EstimateNextEventTime(::core::mem::transmute_copy(&seconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAnimationMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManagerEventHandler<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetManagerEventHandler(&*(&handler as *const <IUIAnimationManagerEventHandler2 as ::windows::core::Abi>::Abi as *const <IUIAnimationManagerEventHandler2 as ::windows::core::DefaultType>::DefaultType), &*(&fregisterfornextanimationevent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCancelPriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison2 as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTrimPriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison2 as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCompressPriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison2 as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConcludePriorityComparison(&*(&comparison as *const <IUIAnimationPriorityComparison2 as ::windows::core::Abi>::Abi as *const <IUIAnimationPriorityComparison2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultLongestAcceptableDelay(delay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IUIAnimationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationManager2>,
            base.5,
            CreateAnimationVectorVariable::<Impl, OFFSET>,
            CreateAnimationVariable::<Impl, OFFSET>,
            ScheduleTransition::<Impl, OFFSET>,
            CreateStoryboard::<Impl, OFFSET>,
            FinishAllStoryboards::<Impl, OFFSET>,
            AbandonAllStoryboards::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            GetVariableFromTag::<Impl, OFFSET>,
            GetStoryboardFromTag::<Impl, OFFSET>,
            EstimateNextEventTime::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            SetAnimationMode::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            SetManagerEventHandler::<Impl, OFFSET>,
            SetCancelPriorityComparison::<Impl, OFFSET>,
            SetTrimPriorityComparison::<Impl, OFFSET>,
            SetCompressPriorityComparison::<Impl, OFFSET>,
            SetConcludePriorityComparison::<Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay::<Impl, OFFSET>,
            Shutdown::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationManagerEventHandlerImpl: Sized {
    fn OnManagerStatusChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationManagerEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationManagerEventHandler";
}
impl IUIAnimationManagerEventHandlerVtbl {
    pub const fn new<Impl: IUIAnimationManagerEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationManagerEventHandlerVtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Impl: IUIAnimationManagerEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnManagerStatusChanged(newstatus, previousstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationManagerEventHandler>, base.5, OnManagerStatusChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationManagerEventHandler2Impl: Sized {
    fn OnManagerStatusChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationManagerEventHandler2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationManagerEventHandler2";
}
impl IUIAnimationManagerEventHandler2Vtbl {
    pub const fn new<Impl: IUIAnimationManagerEventHandler2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationManagerEventHandler2Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Impl: IUIAnimationManagerEventHandler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnManagerStatusChanged(newstatus, previousstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationManagerEventHandler2>, base.5, OnManagerStatusChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationPrimitiveInterpolationImpl: Sized {
    fn AddCubic();
    fn AddSinusoidal();
}
impl ::windows::core::RuntimeName for IUIAnimationPrimitiveInterpolation {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationPrimitiveInterpolation";
}
impl IUIAnimationPrimitiveInterpolationVtbl {
    pub const fn new<Impl: IUIAnimationPrimitiveInterpolationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationPrimitiveInterpolationVtbl {
        unsafe extern "system" fn AddCubic<Impl: IUIAnimationPrimitiveInterpolationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddCubic(dimension, beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSinusoidal<Impl: IUIAnimationPrimitiveInterpolationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddSinusoidal(dimension, beginoffset, bias, amplitude, frequency, phase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationPrimitiveInterpolation>, base.5, AddCubic::<Impl, OFFSET>, AddSinusoidal::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationPriorityComparisonImpl: Sized {
    fn HasPriority();
}
impl ::windows::core::RuntimeName for IUIAnimationPriorityComparison {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationPriorityComparison";
}
impl IUIAnimationPriorityComparisonVtbl {
    pub const fn new<Impl: IUIAnimationPriorityComparisonImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationPriorityComparisonVtbl {
        unsafe extern "system" fn HasPriority<Impl: IUIAnimationPriorityComparisonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasPriority(&*(&scheduledstoryboard as *const <IUIAnimationStoryboard as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard as ::windows::core::DefaultType>::DefaultType), &*(&newstoryboard as *const <IUIAnimationStoryboard as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard as ::windows::core::DefaultType>::DefaultType), priorityeffect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationPriorityComparison>, base.5, HasPriority::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationPriorityComparison2Impl: Sized {
    fn HasPriority();
}
impl ::windows::core::RuntimeName for IUIAnimationPriorityComparison2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationPriorityComparison2";
}
impl IUIAnimationPriorityComparison2Vtbl {
    pub const fn new<Impl: IUIAnimationPriorityComparison2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationPriorityComparison2Vtbl {
        unsafe extern "system" fn HasPriority<Impl: IUIAnimationPriorityComparison2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasPriority(&*(&scheduledstoryboard as *const <IUIAnimationStoryboard2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard2 as ::windows::core::DefaultType>::DefaultType), &*(&newstoryboard as *const <IUIAnimationStoryboard2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard2 as ::windows::core::DefaultType>::DefaultType), priorityeffect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationPriorityComparison2>, base.5, HasPriority::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationStoryboardImpl: Sized {
    fn AddTransition();
    fn AddKeyframeAtOffset();
    fn AddKeyframeAfterTransition();
    fn AddTransitionAtKeyframe();
    fn AddTransitionBetweenKeyframes();
    fn RepeatBetweenKeyframes();
    fn HoldVariable();
    fn SetLongestAcceptableDelay();
    fn Schedule();
    fn Conclude();
    fn Finish();
    fn Abandon();
    fn SetTag();
    fn GetTag();
    fn GetStatus();
    fn GetElapsedTime();
    fn SetStoryboardEventHandler();
}
impl ::windows::core::RuntimeName for IUIAnimationStoryboard {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationStoryboard";
}
impl IUIAnimationStoryboardVtbl {
    pub const fn new<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationStoryboardVtbl {
        unsafe extern "system" fn AddTransition<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTransition(&*(&variable as *const <IUIAnimationVariable as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable as ::windows::core::DefaultType>::DefaultType), &*(&transition as *const <IUIAnimationTransition as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAtOffset(&*(&existingkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType), offset, ::core::mem::transmute_copy(&keyframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAfterTransition(&*(&transition as *const <IUIAnimationTransition as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&keyframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTransitionAtKeyframe(&*(&variable as *const <IUIAnimationVariable as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable as ::windows::core::DefaultType>::DefaultType), &*(&transition as *const <IUIAnimationTransition as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition as ::windows::core::DefaultType>::DefaultType), &*(&startkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTransitionBetweenKeyframes(
                &*(&variable as *const <IUIAnimationVariable as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable as ::windows::core::DefaultType>::DefaultType),
                &*(&transition as *const <IUIAnimationTransition as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition as ::windows::core::DefaultType>::DefaultType),
                &*(&startkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType),
                &*(&endkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RepeatBetweenKeyframes(&*(&startkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType), &*(&endkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType), repetitioncount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldVariable<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldVariable(&*(&variable as *const <IUIAnimationVariable as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLongestAcceptableDelay(delay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Schedule<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Schedule(timenow, ::core::mem::transmute_copy(&schedulingresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conclude<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Conclude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish(completiondeadline) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abandon<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abandon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElapsedTime(::core::mem::transmute_copy(&elapsedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Impl: IUIAnimationStoryboardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStoryboardEventHandler(&*(&handler as *const <IUIAnimationStoryboardEventHandler as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboardEventHandler as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationStoryboard>,
            base.5,
            AddTransition::<Impl, OFFSET>,
            AddKeyframeAtOffset::<Impl, OFFSET>,
            AddKeyframeAfterTransition::<Impl, OFFSET>,
            AddTransitionAtKeyframe::<Impl, OFFSET>,
            AddTransitionBetweenKeyframes::<Impl, OFFSET>,
            RepeatBetweenKeyframes::<Impl, OFFSET>,
            HoldVariable::<Impl, OFFSET>,
            SetLongestAcceptableDelay::<Impl, OFFSET>,
            Schedule::<Impl, OFFSET>,
            Conclude::<Impl, OFFSET>,
            Finish::<Impl, OFFSET>,
            Abandon::<Impl, OFFSET>,
            SetTag::<Impl, OFFSET>,
            GetTag::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetElapsedTime::<Impl, OFFSET>,
            SetStoryboardEventHandler::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationStoryboard2Impl: Sized {
    fn AddTransition();
    fn AddKeyframeAtOffset();
    fn AddKeyframeAfterTransition();
    fn AddTransitionAtKeyframe();
    fn AddTransitionBetweenKeyframes();
    fn RepeatBetweenKeyframes();
    fn HoldVariable();
    fn SetLongestAcceptableDelay();
    fn SetSkipDuration();
    fn Schedule();
    fn Conclude();
    fn Finish();
    fn Abandon();
    fn SetTag();
    fn GetTag();
    fn GetStatus();
    fn GetElapsedTime();
    fn SetStoryboardEventHandler();
}
impl ::windows::core::RuntimeName for IUIAnimationStoryboard2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationStoryboard2";
}
impl IUIAnimationStoryboard2Vtbl {
    pub const fn new<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationStoryboard2Vtbl {
        unsafe extern "system" fn AddTransition<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTransition(&*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType), &*(&transition as *const <IUIAnimationTransition2 as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAtOffset(&*(&existingkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType), offset, ::core::mem::transmute_copy(&keyframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddKeyframeAfterTransition(&*(&transition as *const <IUIAnimationTransition2 as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&keyframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTransitionAtKeyframe(&*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType), &*(&transition as *const <IUIAnimationTransition2 as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition2 as ::windows::core::DefaultType>::DefaultType), &*(&startkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTransitionBetweenKeyframes(
                &*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType),
                &*(&transition as *const <IUIAnimationTransition2 as ::windows::core::Abi>::Abi as *const <IUIAnimationTransition2 as ::windows::core::DefaultType>::DefaultType),
                &*(&startkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType),
                &*(&endkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::windows::core::RawPtr, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RepeatBetweenKeyframes(
                &*(&startkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType),
                &*(&endkeyframe as *const <UI_ANIMATION_KEYFRAME as ::windows::core::Abi>::Abi as *const <UI_ANIMATION_KEYFRAME as ::windows::core::DefaultType>::DefaultType),
                crepetition,
                repeatmode,
                &*(&piterationchangehandler as *const <IUIAnimationLoopIterationChangeHandler2 as ::windows::core::Abi>::Abi as *const <IUIAnimationLoopIterationChangeHandler2 as ::windows::core::DefaultType>::DefaultType),
                id,
                &*(&fregisterfornextanimationevent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldVariable<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldVariable(&*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLongestAcceptableDelay(delay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSkipDuration<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSkipDuration(secondsduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Schedule<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Schedule(timenow, ::core::mem::transmute_copy(&schedulingresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conclude<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Conclude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish(completiondeadline) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abandon<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abandon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElapsedTime(::core::mem::transmute_copy(&elapsedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Impl: IUIAnimationStoryboard2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStoryboardEventHandler(
                &*(&handler as *const <IUIAnimationStoryboardEventHandler2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboardEventHandler2 as ::windows::core::DefaultType>::DefaultType),
                &*(&fregisterstatuschangefornextanimationevent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fregisterupdatefornextanimationevent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationStoryboard2>,
            base.5,
            AddTransition::<Impl, OFFSET>,
            AddKeyframeAtOffset::<Impl, OFFSET>,
            AddKeyframeAfterTransition::<Impl, OFFSET>,
            AddTransitionAtKeyframe::<Impl, OFFSET>,
            AddTransitionBetweenKeyframes::<Impl, OFFSET>,
            RepeatBetweenKeyframes::<Impl, OFFSET>,
            HoldVariable::<Impl, OFFSET>,
            SetLongestAcceptableDelay::<Impl, OFFSET>,
            SetSkipDuration::<Impl, OFFSET>,
            Schedule::<Impl, OFFSET>,
            Conclude::<Impl, OFFSET>,
            Finish::<Impl, OFFSET>,
            Abandon::<Impl, OFFSET>,
            SetTag::<Impl, OFFSET>,
            GetTag::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetElapsedTime::<Impl, OFFSET>,
            SetStoryboardEventHandler::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationStoryboardEventHandlerImpl: Sized {
    fn OnStoryboardStatusChanged();
    fn OnStoryboardUpdated();
}
impl ::windows::core::RuntimeName for IUIAnimationStoryboardEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationStoryboardEventHandler";
}
impl IUIAnimationStoryboardEventHandlerVtbl {
    pub const fn new<Impl: IUIAnimationStoryboardEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationStoryboardEventHandlerVtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Impl: IUIAnimationStoryboardEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStoryboardStatusChanged(&*(&storyboard as *const <IUIAnimationStoryboard as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard as ::windows::core::DefaultType>::DefaultType), newstatus, previousstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStoryboardUpdated<Impl: IUIAnimationStoryboardEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStoryboardUpdated(&*(&storyboard as *const <IUIAnimationStoryboard as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationStoryboardEventHandler>, base.5, OnStoryboardStatusChanged::<Impl, OFFSET>, OnStoryboardUpdated::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationStoryboardEventHandler2Impl: Sized {
    fn OnStoryboardStatusChanged();
    fn OnStoryboardUpdated();
}
impl ::windows::core::RuntimeName for IUIAnimationStoryboardEventHandler2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationStoryboardEventHandler2";
}
impl IUIAnimationStoryboardEventHandler2Vtbl {
    pub const fn new<Impl: IUIAnimationStoryboardEventHandler2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationStoryboardEventHandler2Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Impl: IUIAnimationStoryboardEventHandler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStoryboardStatusChanged(&*(&storyboard as *const <IUIAnimationStoryboard2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard2 as ::windows::core::DefaultType>::DefaultType), newstatus, previousstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStoryboardUpdated<Impl: IUIAnimationStoryboardEventHandler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStoryboardUpdated(&*(&storyboard as *const <IUIAnimationStoryboard2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationStoryboardEventHandler2>, base.5, OnStoryboardStatusChanged::<Impl, OFFSET>, OnStoryboardUpdated::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTimerImpl: Sized {
    fn SetTimerUpdateHandler();
    fn SetTimerEventHandler();
    fn Enable();
    fn Disable();
    fn IsEnabled();
    fn GetTime();
    fn SetFrameRateThreshold();
}
impl ::windows::core::RuntimeName for IUIAnimationTimer {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTimer";
}
impl IUIAnimationTimerVtbl {
    pub const fn new<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTimerVtbl {
        unsafe extern "system" fn SetTimerUpdateHandler<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatehandler: ::windows::core::RawPtr, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimerUpdateHandler(&*(&updatehandler as *const <IUIAnimationTimerUpdateHandler as ::windows::core::Abi>::Abi as *const <IUIAnimationTimerUpdateHandler as ::windows::core::DefaultType>::DefaultType), idlebehavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimerEventHandler<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimerEventHandler(&*(&handler as *const <IUIAnimationTimerEventHandler as ::windows::core::Abi>::Abi as *const <IUIAnimationTimerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTime<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTime(::core::mem::transmute_copy(&seconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameRateThreshold<Impl: IUIAnimationTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFrameRateThreshold(framespersecond) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTimer>, base.5, SetTimerUpdateHandler::<Impl, OFFSET>, SetTimerEventHandler::<Impl, OFFSET>, Enable::<Impl, OFFSET>, Disable::<Impl, OFFSET>, IsEnabled::<Impl, OFFSET>, GetTime::<Impl, OFFSET>, SetFrameRateThreshold::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTimerClientEventHandlerImpl: Sized {
    fn OnTimerClientStatusChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationTimerClientEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTimerClientEventHandler";
}
impl IUIAnimationTimerClientEventHandlerVtbl {
    pub const fn new<Impl: IUIAnimationTimerClientEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTimerClientEventHandlerVtbl {
        unsafe extern "system" fn OnTimerClientStatusChanged<Impl: IUIAnimationTimerClientEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnTimerClientStatusChanged(newstatus, previousstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTimerClientEventHandler>, base.5, OnTimerClientStatusChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTimerEventHandlerImpl: Sized {
    fn OnPreUpdate();
    fn OnPostUpdate();
    fn OnRenderingTooSlow();
}
impl ::windows::core::RuntimeName for IUIAnimationTimerEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTimerEventHandler";
}
impl IUIAnimationTimerEventHandlerVtbl {
    pub const fn new<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTimerEventHandlerVtbl {
        unsafe extern "system" fn OnPreUpdate<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPreUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPostUpdate<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPostUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnRenderingTooSlow<Impl: IUIAnimationTimerEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRenderingTooSlow(framespersecond) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTimerEventHandler>, base.5, OnPreUpdate::<Impl, OFFSET>, OnPostUpdate::<Impl, OFFSET>, OnRenderingTooSlow::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTimerUpdateHandlerImpl: Sized {
    fn OnUpdate();
    fn SetTimerClientEventHandler();
    fn ClearTimerClientEventHandler();
}
impl ::windows::core::RuntimeName for IUIAnimationTimerUpdateHandler {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTimerUpdateHandler";
}
impl IUIAnimationTimerUpdateHandlerVtbl {
    pub const fn new<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTimerUpdateHandlerVtbl {
        unsafe extern "system" fn OnUpdate<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnUpdate(timenow, ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimerClientEventHandler(&*(&handler as *const <IUIAnimationTimerClientEventHandler as ::windows::core::Abi>::Abi as *const <IUIAnimationTimerClientEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Impl: IUIAnimationTimerUpdateHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearTimerClientEventHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTimerUpdateHandler>, base.5, OnUpdate::<Impl, OFFSET>, SetTimerClientEventHandler::<Impl, OFFSET>, ClearTimerClientEventHandler::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTransitionImpl: Sized {
    fn SetInitialValue();
    fn SetInitialVelocity();
    fn IsDurationKnown();
    fn GetDuration();
}
impl ::windows::core::RuntimeName for IUIAnimationTransition {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTransition";
}
impl IUIAnimationTransitionVtbl {
    pub const fn new<Impl: IUIAnimationTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTransitionVtbl {
        unsafe extern "system" fn SetInitialValue<Impl: IUIAnimationTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialValue(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IUIAnimationTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialVelocity(velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDurationKnown<Impl: IUIAnimationTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDurationKnown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDuration(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTransition>, base.5, SetInitialValue::<Impl, OFFSET>, SetInitialVelocity::<Impl, OFFSET>, IsDurationKnown::<Impl, OFFSET>, GetDuration::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTransition2Impl: Sized {
    fn GetDimension();
    fn SetInitialValue();
    fn SetInitialVectorValue();
    fn SetInitialVelocity();
    fn SetInitialVectorVelocity();
    fn IsDurationKnown();
    fn GetDuration();
}
impl ::windows::core::RuntimeName for IUIAnimationTransition2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTransition2";
}
impl IUIAnimationTransition2Vtbl {
    pub const fn new<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTransition2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDimension(::core::mem::transmute_copy(&dimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialValue(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVectorValue<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialVectorValue(value, cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocity<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialVelocity(velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialVectorVelocity(velocity, cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDurationKnown<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDurationKnown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Impl: IUIAnimationTransition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDuration(::core::mem::transmute_copy(&duration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTransition2>, base.5, GetDimension::<Impl, OFFSET>, SetInitialValue::<Impl, OFFSET>, SetInitialVectorValue::<Impl, OFFSET>, SetInitialVelocity::<Impl, OFFSET>, SetInitialVectorVelocity::<Impl, OFFSET>, IsDurationKnown::<Impl, OFFSET>, GetDuration::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTransitionFactoryImpl: Sized {
    fn CreateTransition();
}
impl ::windows::core::RuntimeName for IUIAnimationTransitionFactory {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTransitionFactory";
}
impl IUIAnimationTransitionFactoryVtbl {
    pub const fn new<Impl: IUIAnimationTransitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTransitionFactoryVtbl {
        unsafe extern "system" fn CreateTransition<Impl: IUIAnimationTransitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTransition(&*(&interpolator as *const <IUIAnimationInterpolator as ::windows::core::Abi>::Abi as *const <IUIAnimationInterpolator as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTransitionFactory>, base.5, CreateTransition::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTransitionFactory2Impl: Sized {
    fn CreateTransition();
}
impl ::windows::core::RuntimeName for IUIAnimationTransitionFactory2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTransitionFactory2";
}
impl IUIAnimationTransitionFactory2Vtbl {
    pub const fn new<Impl: IUIAnimationTransitionFactory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTransitionFactory2Vtbl {
        unsafe extern "system" fn CreateTransition<Impl: IUIAnimationTransitionFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTransition(&*(&interpolator as *const <IUIAnimationInterpolator2 as ::windows::core::Abi>::Abi as *const <IUIAnimationInterpolator2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationTransitionFactory2>, base.5, CreateTransition::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationTransitionLibraryImpl: Sized {
    fn CreateInstantaneousTransition();
    fn CreateConstantTransition();
    fn CreateDiscreteTransition();
    fn CreateLinearTransition();
    fn CreateLinearTransitionFromSpeed();
    fn CreateSinusoidalTransitionFromVelocity();
    fn CreateSinusoidalTransitionFromRange();
    fn CreateAccelerateDecelerateTransition();
    fn CreateReversalTransition();
    fn CreateCubicTransition();
    fn CreateSmoothStopTransition();
    fn CreateParabolicTransitionFromAcceleration();
}
impl ::windows::core::RuntimeName for IUIAnimationTransitionLibrary {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTransitionLibrary";
}
impl IUIAnimationTransitionLibraryVtbl {
    pub const fn new<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTransitionLibraryVtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstantaneousTransition(finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateConstantTransition(duration, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDiscreteTransition(delay, finalvalue, hold, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransition(duration, finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransitionFromSpeed(speed, finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromVelocity(duration, period, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromRange(duration, minimumvalue, maximumvalue, period, slope, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAccelerateDecelerateTransition(duration, finalvalue, accelerationratio, decelerationratio, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateReversalTransition(duration, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCubicTransition(duration, finalvalue, finalvelocity, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSmoothStopTransition(maximumduration, finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Impl: IUIAnimationTransitionLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateParabolicTransitionFromAcceleration(finalvalue, finalvelocity, acceleration, ::core::mem::transmute_copy(&transition)) {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationTransitionLibrary>,
            base.5,
            CreateInstantaneousTransition::<Impl, OFFSET>,
            CreateConstantTransition::<Impl, OFFSET>,
            CreateDiscreteTransition::<Impl, OFFSET>,
            CreateLinearTransition::<Impl, OFFSET>,
            CreateLinearTransitionFromSpeed::<Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity::<Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange::<Impl, OFFSET>,
            CreateAccelerateDecelerateTransition::<Impl, OFFSET>,
            CreateReversalTransition::<Impl, OFFSET>,
            CreateCubicTransition::<Impl, OFFSET>,
            CreateSmoothStopTransition::<Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationTransitionLibrary2Impl: Sized {
    fn CreateInstantaneousTransition();
    fn CreateInstantaneousVectorTransition();
    fn CreateConstantTransition();
    fn CreateDiscreteTransition();
    fn CreateDiscreteVectorTransition();
    fn CreateLinearTransition();
    fn CreateLinearVectorTransition();
    fn CreateLinearTransitionFromSpeed();
    fn CreateLinearVectorTransitionFromSpeed();
    fn CreateSinusoidalTransitionFromVelocity();
    fn CreateSinusoidalTransitionFromRange();
    fn CreateAccelerateDecelerateTransition();
    fn CreateReversalTransition();
    fn CreateCubicTransition();
    fn CreateCubicVectorTransition();
    fn CreateSmoothStopTransition();
    fn CreateParabolicTransitionFromAcceleration();
    fn CreateCubicBezierLinearTransition();
    fn CreateCubicBezierLinearVectorTransition();
}
impl ::windows::core::RuntimeName for IUIAnimationTransitionLibrary2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationTransitionLibrary2";
}
impl IUIAnimationTransitionLibrary2Vtbl {
    pub const fn new<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationTransitionLibrary2Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstantaneousTransition(finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstantaneousVectorTransition(finalvalue, cdimension, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateConstantTransition(duration, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDiscreteTransition(delay, finalvalue, hold, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDiscreteVectorTransition(delay, finalvalue, cdimension, hold, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransition(duration, finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearVectorTransition(duration, finalvalue, cdimension, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearTransitionFromSpeed(speed, finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearVectorTransitionFromSpeed(speed, finalvalue, cdimension, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromVelocity(duration, period, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSinusoidalTransitionFromRange(duration, minimumvalue, maximumvalue, period, slope, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAccelerateDecelerateTransition(duration, finalvalue, accelerationratio, decelerationratio, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateReversalTransition(duration, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCubicTransition(duration, finalvalue, finalvelocity, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCubicVectorTransition(duration, finalvalue, finalvelocity, cdimension, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSmoothStopTransition(maximumduration, finalvalue, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateParabolicTransitionFromAcceleration(finalvalue, finalvelocity, acceleration, ::core::mem::transmute_copy(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCubicBezierLinearTransition(duration, finalvalue, x1, y1, x2, y2, ::core::mem::transmute_copy(&pptransition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Impl: IUIAnimationTransitionLibrary2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCubicBezierLinearVectorTransition(duration, finalvalue, cdimension, x1, y1, x2, y2, ::core::mem::transmute_copy(&pptransition)) {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationTransitionLibrary2>,
            base.5,
            CreateInstantaneousTransition::<Impl, OFFSET>,
            CreateInstantaneousVectorTransition::<Impl, OFFSET>,
            CreateConstantTransition::<Impl, OFFSET>,
            CreateDiscreteTransition::<Impl, OFFSET>,
            CreateDiscreteVectorTransition::<Impl, OFFSET>,
            CreateLinearTransition::<Impl, OFFSET>,
            CreateLinearVectorTransition::<Impl, OFFSET>,
            CreateLinearTransitionFromSpeed::<Impl, OFFSET>,
            CreateLinearVectorTransitionFromSpeed::<Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity::<Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange::<Impl, OFFSET>,
            CreateAccelerateDecelerateTransition::<Impl, OFFSET>,
            CreateReversalTransition::<Impl, OFFSET>,
            CreateCubicTransition::<Impl, OFFSET>,
            CreateCubicVectorTransition::<Impl, OFFSET>,
            CreateSmoothStopTransition::<Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration::<Impl, OFFSET>,
            CreateCubicBezierLinearTransition::<Impl, OFFSET>,
            CreateCubicBezierLinearVectorTransition::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationVariableImpl: Sized {
    fn GetValue();
    fn GetFinalValue();
    fn GetPreviousValue();
    fn GetIntegerValue();
    fn GetFinalIntegerValue();
    fn GetPreviousIntegerValue();
    fn GetCurrentStoryboard();
    fn SetLowerBound();
    fn SetUpperBound();
    fn SetRoundingMode();
    fn SetTag();
    fn GetTag();
    fn SetVariableChangeHandler();
    fn SetVariableIntegerChangeHandler();
}
impl ::windows::core::RuntimeName for IUIAnimationVariable {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationVariable";
}
impl IUIAnimationVariableVtbl {
    pub const fn new<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationVariableVtbl {
        unsafe extern "system" fn GetValue<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalValue(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousValue<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousValue(::core::mem::transmute_copy(&previousvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntegerValue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalIntegerValue(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousIntegerValue(::core::mem::transmute_copy(&previousvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStoryboard<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentStoryboard(::core::mem::transmute_copy(&storyboard)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLowerBound(bound) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpperBound<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUpperBound(bound) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoundingMode<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRoundingMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVariableChangeHandler<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVariableChangeHandler(&*(&handler as *const <IUIAnimationVariableChangeHandler as ::windows::core::Abi>::Abi as *const <IUIAnimationVariableChangeHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Impl: IUIAnimationVariableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVariableIntegerChangeHandler(&*(&handler as *const <IUIAnimationVariableIntegerChangeHandler as ::windows::core::Abi>::Abi as *const <IUIAnimationVariableIntegerChangeHandler as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationVariable>,
            base.5,
            GetValue::<Impl, OFFSET>,
            GetFinalValue::<Impl, OFFSET>,
            GetPreviousValue::<Impl, OFFSET>,
            GetIntegerValue::<Impl, OFFSET>,
            GetFinalIntegerValue::<Impl, OFFSET>,
            GetPreviousIntegerValue::<Impl, OFFSET>,
            GetCurrentStoryboard::<Impl, OFFSET>,
            SetLowerBound::<Impl, OFFSET>,
            SetUpperBound::<Impl, OFFSET>,
            SetRoundingMode::<Impl, OFFSET>,
            SetTag::<Impl, OFFSET>,
            GetTag::<Impl, OFFSET>,
            SetVariableChangeHandler::<Impl, OFFSET>,
            SetVariableIntegerChangeHandler::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationVariable2Impl: Sized {
    fn GetDimension();
    fn GetValue();
    fn GetVectorValue();
    fn GetCurve();
    fn GetVectorCurve();
    fn GetFinalValue();
    fn GetFinalVectorValue();
    fn GetPreviousValue();
    fn GetPreviousVectorValue();
    fn GetIntegerValue();
    fn GetIntegerVectorValue();
    fn GetFinalIntegerValue();
    fn GetFinalIntegerVectorValue();
    fn GetPreviousIntegerValue();
    fn GetPreviousIntegerVectorValue();
    fn GetCurrentStoryboard();
    fn SetLowerBound();
    fn SetLowerBoundVector();
    fn SetUpperBound();
    fn SetUpperBoundVector();
    fn SetRoundingMode();
    fn SetTag();
    fn GetTag();
    fn SetVariableChangeHandler();
    fn SetVariableIntegerChangeHandler();
    fn SetVariableCurveChangeHandler();
}
impl ::windows::core::RuntimeName for IUIAnimationVariable2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationVariable2";
}
impl IUIAnimationVariable2Vtbl {
    pub const fn new<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationVariable2Vtbl {
        unsafe extern "system" fn GetDimension<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDimension(::core::mem::transmute_copy(&dimension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVectorValue(::core::mem::transmute_copy(&value), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurve<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurve(&*(&animation as *const <super::super::Graphics::DirectComposition::IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectComposition::IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVectorCurve<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *const ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVectorCurve(&*(&animation as *const <super::super::Graphics::DirectComposition::IDCompositionAnimation as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectComposition::IDCompositionAnimation as ::windows::core::DefaultType>::DefaultType), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalValue(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalVectorValue(::core::mem::transmute_copy(&finalvalue), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousValue(::core::mem::transmute_copy(&previousvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousVectorValue(::core::mem::transmute_copy(&previousvalue), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntegerValue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntegerVectorValue(::core::mem::transmute_copy(&value), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalIntegerValue(::core::mem::transmute_copy(&finalvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalIntegerVectorValue(::core::mem::transmute_copy(&finalvalue), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousIntegerValue(::core::mem::transmute_copy(&previousvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousIntegerVectorValue(::core::mem::transmute_copy(&previousvalue), cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStoryboard<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentStoryboard(::core::mem::transmute_copy(&storyboard)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLowerBound(bound) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBoundVector<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLowerBoundVector(bound, cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpperBound<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUpperBound(bound) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpperBoundVector<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUpperBoundVector(bound, cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoundingMode<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRoundingMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTag(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVariableChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVariableChangeHandler(&*(&handler as *const <IUIAnimationVariableChangeHandler2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariableChangeHandler2 as ::windows::core::DefaultType>::DefaultType), &*(&fregisterfornextanimationevent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVariableIntegerChangeHandler(&*(&handler as *const <IUIAnimationVariableIntegerChangeHandler2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariableIntegerChangeHandler2 as ::windows::core::DefaultType>::DefaultType), &*(&fregisterfornextanimationevent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Impl: IUIAnimationVariable2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVariableCurveChangeHandler(&*(&handler as *const <IUIAnimationVariableCurveChangeHandler2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariableCurveChangeHandler2 as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IUIAnimationVariable2>,
            base.5,
            GetDimension::<Impl, OFFSET>,
            GetValue::<Impl, OFFSET>,
            GetVectorValue::<Impl, OFFSET>,
            GetCurve::<Impl, OFFSET>,
            GetVectorCurve::<Impl, OFFSET>,
            GetFinalValue::<Impl, OFFSET>,
            GetFinalVectorValue::<Impl, OFFSET>,
            GetPreviousValue::<Impl, OFFSET>,
            GetPreviousVectorValue::<Impl, OFFSET>,
            GetIntegerValue::<Impl, OFFSET>,
            GetIntegerVectorValue::<Impl, OFFSET>,
            GetFinalIntegerValue::<Impl, OFFSET>,
            GetFinalIntegerVectorValue::<Impl, OFFSET>,
            GetPreviousIntegerValue::<Impl, OFFSET>,
            GetPreviousIntegerVectorValue::<Impl, OFFSET>,
            GetCurrentStoryboard::<Impl, OFFSET>,
            SetLowerBound::<Impl, OFFSET>,
            SetLowerBoundVector::<Impl, OFFSET>,
            SetUpperBound::<Impl, OFFSET>,
            SetUpperBoundVector::<Impl, OFFSET>,
            SetRoundingMode::<Impl, OFFSET>,
            SetTag::<Impl, OFFSET>,
            GetTag::<Impl, OFFSET>,
            SetVariableChangeHandler::<Impl, OFFSET>,
            SetVariableIntegerChangeHandler::<Impl, OFFSET>,
            SetVariableCurveChangeHandler::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAnimationVariableChangeHandlerImpl: Sized {
    fn OnValueChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationVariableChangeHandler {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationVariableChangeHandler";
}
impl IUIAnimationVariableChangeHandlerVtbl {
    pub const fn new<Impl: IUIAnimationVariableChangeHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationVariableChangeHandlerVtbl {
        unsafe extern "system" fn OnValueChanged<Impl: IUIAnimationVariableChangeHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: f64, previousvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnValueChanged(&*(&storyboard as *const <IUIAnimationStoryboard as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard as ::windows::core::DefaultType>::DefaultType), &*(&variable as *const <IUIAnimationVariable as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable as ::windows::core::DefaultType>::DefaultType), newvalue, previousvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationVariableChangeHandler>, base.5, OnValueChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationVariableChangeHandler2Impl: Sized {
    fn OnValueChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationVariableChangeHandler2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationVariableChangeHandler2";
}
impl IUIAnimationVariableChangeHandler2Vtbl {
    pub const fn new<Impl: IUIAnimationVariableChangeHandler2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationVariableChangeHandler2Vtbl {
        unsafe extern "system" fn OnValueChanged<Impl: IUIAnimationVariableChangeHandler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnValueChanged(&*(&storyboard as *const <IUIAnimationStoryboard2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard2 as ::windows::core::DefaultType>::DefaultType), &*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType), newvalue, previousvalue, cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationVariableChangeHandler2>, base.5, OnValueChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationVariableCurveChangeHandler2Impl: Sized {
    fn OnCurveChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationVariableCurveChangeHandler2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationVariableCurveChangeHandler2";
}
impl IUIAnimationVariableCurveChangeHandler2Vtbl {
    pub const fn new<Impl: IUIAnimationVariableCurveChangeHandler2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationVariableCurveChangeHandler2Vtbl {
        unsafe extern "system" fn OnCurveChanged<Impl: IUIAnimationVariableCurveChangeHandler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCurveChanged(&*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationVariableCurveChangeHandler2>, base.5, OnCurveChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationVariableIntegerChangeHandlerImpl: Sized {
    fn OnIntegerValueChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationVariableIntegerChangeHandler {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationVariableIntegerChangeHandler";
}
impl IUIAnimationVariableIntegerChangeHandlerVtbl {
    pub const fn new<Impl: IUIAnimationVariableIntegerChangeHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationVariableIntegerChangeHandlerVtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Impl: IUIAnimationVariableIntegerChangeHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: i32, previousvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnIntegerValueChanged(&*(&storyboard as *const <IUIAnimationStoryboard as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard as ::windows::core::DefaultType>::DefaultType), &*(&variable as *const <IUIAnimationVariable as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable as ::windows::core::DefaultType>::DefaultType), newvalue, previousvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationVariableIntegerChangeHandler>, base.5, OnIntegerValueChanged::<Impl, OFFSET>)
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler2Impl: Sized {
    fn OnIntegerValueChanged();
}
impl ::windows::core::RuntimeName for IUIAnimationVariableIntegerChangeHandler2 {
    const NAME: &'static str = "Windows.Win32.UI.Animation.IUIAnimationVariableIntegerChangeHandler2";
}
impl IUIAnimationVariableIntegerChangeHandler2Vtbl {
    pub const fn new<Impl: IUIAnimationVariableIntegerChangeHandler2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIAnimationVariableIntegerChangeHandler2Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Impl: IUIAnimationVariableIntegerChangeHandler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnIntegerValueChanged(&*(&storyboard as *const <IUIAnimationStoryboard2 as ::windows::core::Abi>::Abi as *const <IUIAnimationStoryboard2 as ::windows::core::DefaultType>::DefaultType), &*(&variable as *const <IUIAnimationVariable2 as ::windows::core::Abi>::Abi as *const <IUIAnimationVariable2 as ::windows::core::DefaultType>::DefaultType), newvalue, previousvalue, cdimension) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIAnimationVariableIntegerChangeHandler2>, base.5, OnIntegerValueChanged::<Impl, OFFSET>)
    }
}
